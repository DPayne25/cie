CREATE TABLE public.raw_cot_reports (
    report_date DATE NOT NULL UNIQUE,
    tff_code VARCHAR(10) NOT NULL UNIQUE,
    trader_type VARCHAR(30) NOT NULL UNIQUE,
    long_positions INT,
    short_positions INT,
    net_positions INT,
    change_from_prev INT
);

CREATE TABLE public.raw_fx_prices (
    fx_price_date DATE NOT NULL UNIQUE,
    symbol VARCHAR(10) NOT NULL UNIQUE,
    close_price DECIMAL(18,5) UNIQUE,
    is_tuesday BOOLEAN
);

CREATE TABLE public.weekly_sentiment (
    report_date DATE NOT NULL UNIQUE,
    symbol VARCHAR(6) NOT NULL UNIQUE,
    release_date DATE,
    am_net_pos INT,
    lf_net_pos INT,
    price_tuesday DECIMAL(18,5),
    price_friday DECIMAL(18,5),
    price_delta_percent DECIMAL(18,5),
    market_repsonse VARCHAR(50)
);