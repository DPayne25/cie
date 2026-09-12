use chrono::{Duration, TimeZone, Utc, NaiveDate};
use fxoanda;
use reqwest;
use sqlx::PgPool;
use sqlx::types::BigDecimal;
use std::time::Duration as StdDuration;
use tokio::time::sleep;
use anyhow::{Context, Result};

pub async fn fetch_fx_price_data(
    client: &reqwest::Client,
    price_type: &str,
    instrument: &str,
    broker_key: &String,
    pool: &PgPool,
) -> Result<(), anyhow::Error> {

    let oanda_client = fxoanda::Client {
        host: "api-fxtrade.oanda.com".to_string(),
        reqwest: client.clone(),
        authentication: broker_key.to_string(),
    };

    let last: Option<NaiveDate> = sqlx::query_scalar!(
        "SELECT MAX(price_date) FROM fx_price_daily WHERE currency_pair = $1",
        instrument
    )
    .fetch_one(pool)
    .await?;

    let mut current_start = match last {
        Some(d) => Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap()) + Duration::days(1),
        None => Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap(),
    };

    // --- Daily ---
    let mut all_candles_d: Vec<fxoanda::Candlestick> = Vec::new();

    while current_start < Utc::now() {
        let d_response = fxoanda::GetInstrumentCandlesRequest::new()
            .with_instrument(instrument.to_string())
            .with_from(current_start)
            .with_granularity(fxoanda::CandlestickGranularity::D)
            .with_price(price_type.to_string())
            .with_count(5000)
            .remote(&oanda_client)
            .await
            .map_err(|e| anyhow::anyhow!("OANDA request failed {instrument}: {e}"))?;

        let candles = match d_response.candles {
            Some(c) if !c.is_empty() => c,
            _ => break,
        };

        match candles.last().and_then(|c| c.time) {
            Some(last_time) => current_start = last_time + Duration::days(1),
            None => break,
        }

        all_candles_d.extend(candles);
        sleep(StdDuration::from_millis(500)).await;
    }


    let rows: Vec<FxPriceDaily> = all_candles_d
        .iter()
        .filter(|c| c.complete.unwrap_or(false))
        .map(|c| FxPriceDaily::from_candle(c, instrument))
        .collect::<Result<Vec<_>>>()?;

    ingest_price_db(&rows, pool).await?;

    Ok(())
}



#[derive(Debug)]
pub struct FxPriceDaily {
    pub currency_pair: String,
    pub price_date: NaiveDate,
    pub complete: bool,
    pub open_price: BigDecimal,
    pub high_price: BigDecimal,
    pub low_price: BigDecimal,
    pub close_price: BigDecimal,
    pub tick_volume: i32,
}
impl FxPriceDaily {
    pub fn from_candle(c: &fxoanda::Candlestick, pair: &str) -> Result<Self> {
        let mid = c.mid.as_ref().context("candle missing mid prices")?;
        let time = c.time.context("candle missing time")?;

        Ok(Self {
            currency_pair: pair.replace("_", ""),
            price_date: time.date_naive(),
            complete: c.complete.unwrap_or(false),
            open_price: BigDecimal::try_from(mid.o.context("missing open")?)?,
            high_price: BigDecimal::try_from(mid.h.context("missing high")?)?,
            low_price: BigDecimal::try_from(mid.l.context("missing low")?)?,
            close_price: BigDecimal::try_from(mid.c.context("missing close")?)?,
            tick_volume: c.volume.context("missing volume")? as i32,
        })
    }
}

pub async fn ingest_price_db(rows: &Vec<FxPriceDaily>, pool: &PgPool) -> Result<(), anyhow::Error> {
    let mut tx = pool.begin().await?;

    for row in rows {
        sqlx::query!(
            r#"INSERT INTO fx_price_daily
                (currency_pair, price_date, complete, open_price, high_price, low_price, close_price, tick_volume)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (price_date, currency_pair) DO UPDATE SET
                    complete = EXCLUDED.complete,
                    open_price = EXCLUDED.open_price,
                    high_price = EXCLUDED.high_price,
                    low_price = EXCLUDED.low_price,
                    close_price = EXCLUDED.close_price,
                    tick_volume = EXCLUDED.tick_volume;
    "#,
            row.currency_pair,
            row.price_date,
            row.complete,
            row.open_price,
            row.high_price,
            row.low_price,
            row.close_price,
            row.tick_volume,
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    Ok(())
}