---
author: DakotaJPayne
date: 2025-12-24
project: cie
---

<p align="center">
  <img src="docs/cie_v1.gif" alt="CIE System Flowchart" width="80%">
</p>

# Introduction
I built this to get the Commitments of Traders data into a dashboard I actually control. COT shows how dealers, asset managers, and leveraged funds are positioned in currency futures. I use it to check whether the trend I'm trading is backed by positioning or running against it.

## Setup

### Prerequisites

- [PostgreSQL 14+](https://www.postgresql.org/download/)
- [Rust](https://rustup.rs/) (stable)
- An OANDA account with a [personal access token](https://developer.oanda.com/rest-live-v20/introduction/)

### 1. Create the database

```bash
createdb cie
```

### 2. Run the schema

```bash
psql -d cie -f schema.sql
```

In pgAdmin: open `schema.sql` in the Query Tool with `cie` selected, then execute.

This creates the tables and seeds `dim_currency` with the eight currency contracts the CFTC reports on.

### 3. Configure credentials

Copy `.env.example` to `.env` and fill in your values:

```
OANDA_API_KEY=""
OANDA_HOST="api-fxtrade.oanda.com"
DATABASE_URL="postgres://user:password@localhost:5432/cie"
```

### 4. Run

```bash
cargo run
```

First run backfills COT from 2016 and daily FX candles from 2016, which takes a few minutes. Subsequent runs resume from the latest date already stored and only fetch what's new.

### Connecting Power BI

Get Data → PostgreSQL database. Server `localhost`, database `cie`. The tables are `cot_tff`, `fx_price_daily`, and `dim_currency`, joined on `cftc_contract_market_code` and `currency_pair`.

<p align="center">
  <img src="docs/cie_flowchart.png" alt="CIE System Flowchart" width="80%">
</p>
