---
author: DakotaJPayne
date: 2025-12-24
project: cie
---

# The Problem
Most COT analysis tools are broken: cluttered free services with terrible UX, or expensive subscription tiers ($1,399/year) that don't justify the cost for independent traders. The data exists, but it's buried.

# The Solution
A production-grade Power BI dashboard that transforms raw CFTC Commitments of Traders (COT) reports into **actionable positioning intelligence** for forex traders. Real data. Clean visuals. Zero markup.

**Current Status:**
- [x] **Data Ingestion** — Pulling COT reports and FX prices reliably
- [ ] **Calculations** — Building Z-scores, volatility analysis, volume metrics, and pair matching logic
- [ ] **Power BI Integration** — Coming next

# What You'll Get
Each Friday at 18:00 EST (when CFTC data is released and markets are closed), the system automatically computes and displays:

- **Z-Score Analysis** — Where are large traders positioned relative to historical norms?
- **Positioning Volume** — How much capital is actually moving?
- **Volatility Context** — Risk amplification lens for each pair
- **Pair Matching** — How do correlated contracts amplify signals?

Starting with major FX pairs (EUR/USD, GBP/USD, USD/JPY, USD/CHF, AUD/USD, USD/CAD, NZD/USD). Full forex coverage in future versions.

# Tech Stack
- **Rust** — High-performance data ingestion and transformation
- **Postgres** — Historical data store with 5+ years of normalized metrics
- **Cron** — Fully automated weekly execution (no manual updates)
- **Power BI** — Presentation layer (calculations done upstream, not in the dashboard)

# Data Sources
- [CFTC - COT Reports](https://www.cftc.gov/MarketReports/CommitmentsofTraders/index.htm)
- [Oanda FX Prices](https://www.oanda.com/)

# Project Phases
See `/docs/dev_log.md` for the full development roadmap (P0–P10).
