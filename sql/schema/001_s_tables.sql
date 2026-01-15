CREATE TABLE public.raw_cot_reports (
    report_date date NOT NULL,
    tff_code character varying NOT NULL,
    trader_type character varying NOT NULL,
    long_positions integer,
    short_positions integer,
    net_positions integer,
    change_from_prev integer
);

CREATE TABLE public.raw_fx_prices (
    date date NOT NULL,
    symbol character varying(10) NOT NULL,
    close_price numeric(18,5),
    is_tuesday boolean
);

CREATE TABLE public.weekly_sentiment (
    report_date date NOT NULL,
    symbol character varying(6) NOT NULL,
    release_date date,
    am_net_pos integer,
    lf_net_pos integer,
    price_tuesday numeric(18,5),
    price_friday numeric(18,5),
    price_delta_percent numeric(18,5),
    market_repsonse character varying(50)
);