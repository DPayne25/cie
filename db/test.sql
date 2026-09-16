SELECT * FROM dim_currency;
SELECT * FROM cot_tff LIMIT 3;

SELECT column_name, data_type
FROM information_schema.columns
WHERE table_name IN ('cot_tff', 'fx_price_daily')
  AND column_name IN ('as_of', 'price_date');

SELECT MIN(as_of), MAX(as_of), COUNT(DISTINCT as_of) FROM cot_tff;
SELECT MIN(price_date), MAX(price_date), COUNT(DISTINCT price_date) FROM fx_price_daily;

SELECT 'cot' AS src, COUNT(*) FROM cot_tff WHERE as_of = '2016-01-08'
UNION ALL
SELECT 'fx', COUNT(*) FROM fx_price_daily WHERE price_date = '2016-01-08';


-- # Make view TODO 
SELECT (
    (ct.leveraged_money_positions_long - ct.leveraged_money_positions_short) * (CASE WHEN dc.base_type = 'Inverted'
        THEN -1 ELSE 1 END)
) AS leveraged_net_position
FROM dim_currency AS dc
JOIN cot_tff as ct ON dc.cftc_contract_market_code = ct.cftc_contract_market_code;