DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'currency_base_type') THEN
        CREATE TYPE currency_base_type AS ENUM ('Direct', 'Inverted');
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS dim_currency (
    cftc_contract_market_code VARCHAR(10) PRIMARY KEY,
    currency_pair VARCHAR(10) UNIQUE NOT NULL,
    base_type currency_base_type NOT NULL
);

CREATE TABLE IF NOT EXISTS cot_tff (
    market_exchange_names VARCHAR(255) NOT NULL,
    report_date DATE NOT NULL,
    cftc_contract_market_code VARCHAR(10) NOT NULL REFERENCES dim_currency(cftc_contract_market_code),
    open_interest INTEGER NOT NULL,
    dealer_positions_long INTEGER NOT NULL,
    dealer_positions_short INTEGER NOT NULL,
    dealer_positions_spread INTEGER NOT NULL,
    asset_manager_positions_long INTEGER NOT NULL,
    asset_manager_positions_short INTEGER NOT NULL,
    asset_manager_positions_spread INTEGER NOT NULL,
    leveraged_money_positions_long INTEGER NOT NULL,
    leveraged_money_positions_short INTEGER NOT NULL,
    leveraged_money_positions_spread INTEGER NOT NULL,
    other_rept_positions_long INTEGER NOT NULL,
    other_rept_positions_short INTEGER NOT NULL,
    other_rept_positions_spread INTEGER NOT NULL,
    as_of DATE NOT NULL,
    PRIMARY KEY (report_date, cftc_contract_market_code)
);

COMMENT ON COLUMN cot_tff.as_of IS
  'CFTC publication date. Derived as report_date + 3 days (Tue report, Fri 3:30pm EST release). Federal holidays can delay actual release.';

-- UPDATE cot_tff SET as_of = report_date + INTERVAL '3 days' WHERE as_of IS NULL;

CREATE TABLE IF NOT EXISTS fx_price_daily (
    currency_pair VARCHAR(10) NOT NULL,
    price_date DATE NOT NULL,
    complete BOOLEAN NOT NULL,
    open_price NUMERIC(10,5) NOT NULL,
    high_price NUMERIC(10,5) NOT NULL,
    low_price NUMERIC(10,5) NOT NULL,
    close_price NUMERIC(10,5) NOT NULL,
    tick_volume INTEGER NOT NULL,
    PRIMARY KEY (price_date, currency_pair)
);

CREATE TABLE IF NOT EXISTS fx_price_h1 (
    currency_pair VARCHAR(10) NOT NULL,
    price_date TIMESTAMPTZ NOT NULL,
    complete BOOLEAN NOT NULL,
    open_price NUMERIC(10,5) NOT NULL,
    high_price NUMERIC(10,5) NOT NULL,
    low_price NUMERIC(10,5) NOT NULL,
    close_price NUMERIC(10,5) NOT NULL,
    tick_volume INTEGER NOT NULL,
    PRIMARY KEY (price_date, currency_pair)
);

INSERT INTO dim_currency (cftc_contract_market_code, currency_pair, base_type) VALUES
  ('099741', 'EURUSD', 'Direct'),
  ('096742', 'GBPUSD', 'Direct'),
  ('232741', 'AUDUSD', 'Direct'),
  ('112741', 'NZDUSD', 'Direct'),
  ('097741', 'USDJPY', 'Inverted'),
  ('090741', 'USDCAD', 'Inverted'),
  ('092741', 'USDCHF', 'Inverted'),
  ('399741', 'EURJPY', 'Direct')
ON CONFLICT (cftc_contract_market_code) DO NOTHING;

SELECT * FROM fx_price_daily;