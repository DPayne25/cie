--
-- PostgreSQL database dump
--

\restrict DnS0mCac4zpKHd4ZzgmLp29t0aAEfuiHnNDweVGowqhxLsDcWu9zm9dRhbsMwiF

-- Dumped from database version 17.6 (Debian 17.6-0+deb13u1)
-- Dumped by pg_dump version 17.6 (Debian 17.6-0+deb13u1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: raw_cot_reports; Type: TABLE; Schema: public; Owner: dakotajpayne
--

CREATE TABLE public.raw_cot_reports (
    report_date date NOT NULL,
    tff_code character varying NOT NULL,
    trader_type character varying NOT NULL,
    long_positions integer,
    short_positions integer,
    net_positions integer,
    change_from_prev integer
);


ALTER TABLE public.raw_cot_reports OWNER TO dakotajpayne;

--
-- Name: raw_fx_prices; Type: TABLE; Schema: public; Owner: dakotajpayne
--

CREATE TABLE public.raw_fx_prices (
    date date NOT NULL,
    symbol character varying(10) NOT NULL,
    close_price numeric(18,5),
    is_tuesday boolean
);


ALTER TABLE public.raw_fx_prices OWNER TO dakotajpayne;

--
-- Name: weekly_sentiment; Type: TABLE; Schema: public; Owner: dakotajpayne
--

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


ALTER TABLE public.weekly_sentiment OWNER TO dakotajpayne;

--
-- Name: raw_cot_reports raw_cot_reports_pkey; Type: CONSTRAINT; Schema: public; Owner: dakotajpayne
--

ALTER TABLE ONLY public.raw_cot_reports
    ADD CONSTRAINT raw_cot_reports_pkey PRIMARY KEY (report_date, tff_code, trader_type);


--
-- Name: raw_fx_prices raw_fx_prices_pkey; Type: CONSTRAINT; Schema: public; Owner: dakotajpayne
--

ALTER TABLE ONLY public.raw_fx_prices
    ADD CONSTRAINT raw_fx_prices_pkey PRIMARY KEY (date, symbol);


--
-- Name: weekly_sentiment weekly_sentiment_pkey; Type: CONSTRAINT; Schema: public; Owner: dakotajpayne
--

ALTER TABLE ONLY public.weekly_sentiment
    ADD CONSTRAINT weekly_sentiment_pkey PRIMARY KEY (report_date, symbol);


--
-- PostgreSQL database dump complete
--

\unrestrict DnS0mCac4zpKHd4ZzgmLp29t0aAEfuiHnNDweVGowqhxLsDcWu9zm9dRhbsMwiF

