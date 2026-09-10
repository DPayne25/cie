CREATE TYPE currency_base_type AS ENUM ('Direct', 'Inverted');

CREATE TABLE IF NOT EXISTS dim_currency (
    cftc_contract_market_code VARCHAR(10) PRIMARY KEY,
    currency_pair VARCHAR(10) UNIQUE NOT NULL,
    base_type currency_base_type NOT NULL
);

CREATE TABLE IF NOT EXISTS cot_tff (
    market_exchange_names VARCHAR(255) NOT NULL,
    report_date DATE NOT NULL,
    cftc_contract_market_code VARCHAR(10) NOT NULL REFERENCES dim_currency(cftc_contract_code),
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
    PRIMARY KEY (report_date, cftc_contract_market_code)
);

CREATE TABLE IF NOT EXISTS fx_price (
    currency_pair VARCHAR(10) NOT NULL REFERENCES dim_currency(currency_pair),
    price_date DATE NOT NULL,
    complete BOOLEAN NOT NULL,
    open_price NUMERIC(10,5) NOT NULL,
    high_price NUMERIC(10,5) NOT NULL,
    low_price NUMERIC(10,5) NOT NULL,
    close_price NUMERIC(10,5) NOT NULL,
    tick_volume INTEGER NOT NULL,
    PRIMARY KEY (price_date, currency_pair)
);