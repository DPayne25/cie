# Project Roadmap: The Development Log

**System:** FX COT Dashboard.

**Stack:** Debian 13, Rust, Postgres, Cron, Power BI.

## Purpose
This document serves as the "Black Box" of the development process. It is a targeted record of technical hurdles and their specific resolutions.

## Sturcture
Each entry is categorized by the project Phase.

- *Objective & Solutions*: The technical logic, dependicies, and resources used to accomplish each task in the session

---

# [P0] Mission Definition (MD)

Goal: Lock the objective and scope.

## Objectives & Solutions

- [x] Define project objective in one sentence 

>**Execution & Solution:**
>
>Build a Power BI dashboard that displays COT report data from CFTC for effective decision making in the Foriegn Exchange (FX) market. 

- [x] Define scope (Data)



>**Execution & Solution:**
    > - **COT Data Sources:** [CFTC](https://cftc.gov/) (Webscrape) & [Socrata](https://dev.socrata.com/foundry/publicreporting.cftc.gov/gpe5-46if) (API).
    >
    > - **FX Price Data Sources:** 
    >   - [Oanda](https://developer.oanda.com/rest-live-v20/introduction/) (EURUSD, USDJPY, GBPUSD, AUDUSD, USDCHF, USDCAD, NZDUSD)
    >   - Fallback: [Massive](https://massive.com/) 
    >
    > - **Target Contracts (TFF Codes):**
    >     - Euro (099741)
    >     - Japanese Yen (097741)
    >     - British Pound Sterling (096742)
    >     - Australian Dollar (232741)
    >     - Canadian Dollar (090741)
    >     - Swiss Franc (092741)
    >     - New Zealand Dollar (112741)
    >
    > - **Historical Depth:** Minimum of 5 years of history to ensure normalization metrics (Z-Score, Percentiles) are statistically reliable.




# [P1] Server Foundation (SF)

Goal: Designate the Debian 13 server (Kensho-Dev-v1) as the single source of truth.

## Objectives & Solutions

- [x] Designate server as single source of truth.

>Execution & Solution:
>
>SSH to Kensho-Dev-v1:

- [x] Create project root: ~/myProjects/FXCOTDashboard/ and subdirectories.

>Execution & Solution:

```
~/myProjects/FXCOTDashboard/
│
├── README.md
│
├── docs/
│   ├── architecture.md
│   ├── data_sources.md
│   ├── dev_log.md
│   ├── assumptions.md
│   ├── update_schedule.md
│   └── interpretation_rules.md
│
├── config/
│   ├── database.env
│   ├── ingestion.toml
│   ├── logging.toml
│   └── pairs.yaml
│
├── bin/
│   └── cot_ingestor
│
├── src/
│   ├── main.rs
│   │
│   ├── ingestion/
│   │   ├── mod.rs
│   │   ├── cot_fetch.rs
│   │   ├── csv_validate.rs
│   │   └── fx_price_fetch.rs
│   │
│   ├── transform/
│   │   ├── mod.rs
│   │   ├── positioning.rs
│   │   ├── rolling_stats.rs
│   │   └── volatility.rs
│   │
│   ├── db/
│   │   ├── mod.rs
│   │   ├── connection.rs
│   │   ├── insert_raw.rs
│   │   └── insert_processed.rs
│   │
│   ├── time/
│   │   ├── mod.rs
│   │   └── cot_calendar.rs
│   │
│   └── utils/
│       ├── logging.rs
│       ├── checksum.rs
│       └── error.rs
│
├── sql/
│   ├── schema/
│   │   ├── 001_create_tables.sql
│   │   ├── 002_indexes.sql
│   │   └── 003_views.sql
│   │
│   ├── transforms/
│   │   ├── compute_zscores.sql
│   │   ├── compute_percentiles.sql
│   │   └── weekly_deltas.sql
│   │
│   └── validation/
│       ├── row_counts.sql
│       └── sanity_checks.sql
│
├── data/
│   ├── raw/
│   │   ├── cot/
│   │   ├── fx_prices/
│   │   └── volatility/
│   │
│   └── processed/
│       ├── cot/
│       └── metrics/
│
├── logs/
│   ├── ingestion.log
│   ├── errors.log
│   └── cron.log
│
├── cron/
│   └── cot_weekly.cron
│
├── powerbi/
│   ├── model_notes.md
│   ├── measures.dax
│   └── screenshots/
│
└── scripts/
    ├── bootstrap.sh
    ├── backfill_cot.sh
    └── health_check.sh
```

- [x] Install base services: Postgres, Cron, Git.

>Execution & Solution:
>
>`sudo apt install postgresql git `


- [x] Harden access: SSH keys only, Firewall locked, Postgres local or restricted IP.


# [P2] Data Modeling (DM)

Goal: Map futures contracts and design the Postgres schema. No code until this is coherent.

## Objectives & Solutions

- [x] Identify COT report type (CFTC).

>Execution & Solution:
>
> - TFF Report
> - HTML ([CFTC](https://www.cftc.gov/MarketReports/CommitmentsofTraders/index.htm))
> - CSV ([CFTC](https://www.cftc.gov/MarketReports/CommitmentsofTraders/index.htm))
> - JSON ([Scrata](https://dev.socrata.com/foundry/publicreporting.cftc.gov/gpe5-46if))

- [x] Map futures contracts → FX pairs.

>Execution & Solution:

| Futures Contract (COT Report) | TFF Code | FX Pair |
| ----------------------------- | -------- | ------- |
| Euro                          | 099741   | EURUSD  |
| Japanese Yen                  | 097741   | USDJPY  |
| British Pound Sterling        | 096742   | GBPUSD  |
| Australian Dollar             | 232741   | AUDUSD  |
| Canadian Dollar               | 090741   | USDCAD  |
| Swiss Franc                   | 092741   | USDCHF  |
| New Zealand Dollar            | 112741   | NZDUSD  |



- [x] Define trader classes to track.

> Execution & Solution:
>
> The system will exclusively focus on the Asset Manager and Leveraged Fund categories. This means the Rust ingestion engine (P3) will only extract the Long, Short, Net, and Change in Net columns for these two groups, effectively filtering out the structural hedging (Dealer) and unclassified retail (Other) noise.


| Trader Class                    | Action     |
| ------------------------------- | ---------- |
| Dealer/Intermediary             | NOISE      |
| **Asset Manager/Institutional** | **SIGNAL** |
| **Leveraged Funds**             | **SIGNAL** |
| Other Reportables               | NOISE      | 

- Long Positions
- Short Positions
- Net Position
- Change in Net Position
- Open Interest



- [x] Define time alignment rules (Tuesday → Friday).

>Execution & Solution:
>
>The data from the COT report and the FX price data will be ingested each Friday at 17:30 EST. 
>The COT data is collected each Tuesday and audited before release on immediate Friday. That means the FX price data by Friday close is the reaction to institutional position that week.

- [x] Design Postgres schema for Raw COT data, Processed positioning metrics, FX prices, FX volatility, and Pair mappings.

Execution & Solution:

## Table 1: raw_cot_reports

| headers          | data_type | meaning                                |
| ---------------- | --------- | -------------------------------------- |
| report_date (PK) | DATE      | The Tuesday date (As-Of)               |
| tff_code (PK)    | VARCHAR   | e.g., '099741' (Euro)                  |
| trader_type (PK) | VARCHAR   | *Asset_Manager* or *Leveraged_Funds*   |
| long_positions   | INT       | Number of long positions               |
| short_positions  | INT       | Number of short positions              |
| net_positions    | INT       | `'Long Positions' - 'Short Positions'` |
| change_from_prev | INT       | $∆$ reported by CFTC                   |


## Table 2: raw_fx_prices

| headers     | data_type | meaning                       |
| ----------- | --------- | ----------------------------- |
| date (PK)   | DATE      | The daily candle date         |
| symbol (PK) | VARCHAR   | e.g., 'EURUSD'                |
| close_price | DECIMAL   | The closing price (Weekly)    |
| is_tuesday  | BOOLEAN   | Helper flag for fast indexing | 


## Table 3: weekly_sentiment

| headers             | data_type | meaning                                               |
| ------------------- | --------- | ----------------------------------------------------- |
| report_date (PK)    | DATE      | The Tuesday date                                      |
| symbol (PK)         | VARCHAR   | e.g., 'EURUSD'                                        |
| release_date        | DATE      | The Friday Date (report_date + 3)                     |
| am_net_pos          | INT       | Asset Manager Net                                     |
| lf_net_pos          | INT       | Leveraged Funds Net                                   |
| price_tuesday       | DECIMAL   | Price at the moment of the snapshot                   |
| price_friday        | DECIMAL   | Price at the moment of release                        |
| price_delta_percent | DECIMAL   | `(price_friday - price_tuesday) / price_tuesday`      |
| market_response     | VARCHAR   | Computed classification (e.g., 'Bullish Convergence') |


- [x] Decide where each computation lives (Rust vs SQL vs Power BI).

>Execution & Solution:
>
>Home Server -> Tool Environment
>Rust -> Calculation (Z-Scores)/Connection (API)/Transformation
>Postgres -> Storage 
>Power BI -> Presentation

| Task            | Component                   |
| --------------- | --------------------------- |
| Environment     | Kensho-Dev-v1 (Home Server) | 
| Z-Scores/Stats  | Rust                        |
| Market Verdicts | Rust                        |
| Joins/Deltas    | Postgres                    |

# [P3] Ingestion Engine (IE)

Goal: Build the Rust-based engine to automate data fetching. At this point, raw COT data is trustworthy and repeatable.

Objectives & Solutions

- [x] Initialize Rust project.

Execution & Solution:

`cargo init`

- [x] Build CSV downloader.
  Resources: 

    `reqwest`: https://crates.io/crates/reqwest, https://docs.rs/reqwest/latest/reqwest/

    `tokio`: https://crates.io/crates/tokio/1.48.0, https://docs.rs/tokio/latest/tokio/
    
    `std`: https://doc.rust-lang.org/std/
    
    `std::fs`: https://doc.rust-lang.org/std/fs/
    
    `std::io`: https://doc.rust-lang.org/std/io/
    
    `Packages, Crates, and Modules`: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
    
    `NEW`: Started using VSCode extension `rust-analyzer`

Execution & Solution:
 First successful compiling. ✅
 `csv_validate.rs`
 {included csv validator but this is a hard coded method. I will have to shift to using an API}





- [x] Validate file structure strictly.


Execution & Solution:

`Naming the File`: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-02-reading-a-file.html

- [x] Normalize dates & fields.

Execution & Solution:

Main solution was to keep the vector outside of the for loop and ensure correct index.

Also asynchronously downloaded the FX price data using `futures` crate.

I decided unapologetically use the AI along this process to teach me and expose me to new concepts and not write any code for so I have a particular prop that I'm going to include below so that can use it for future segments of the project and future projects:

> Pre Chat Prompt:
> 
    >  "I need you to stop writing code for me. Just think through the problems with me. And if I need help on code, I will explicitly ask you and initially give me the link to the documentation so I can do my own research and figure it out for myself. And if I keep struggling with it, and if I press you to actually give me the code or at least a suggestive code to improve I would ask you to give me an example instead of the explicit code. Do you understand?"

>Post Chat Prompt: 
>
    >"Now. What I want you to do is summarize this particular. conversation include all the documentations and connected to the segments that solve the problem like this documentation documentation helped us solve this particular problem. Okay, and really make it simple to read because I will be reviewing it and going to the documentation. For myself and recognize also that I will be pacing this into my obsidian note folder or no Vault. So put it in markdown and Link it to particular lines so that it's easily referenceable for this was the problem or topic that we talked about and this was the documentation that links to that and not like you to put it in kind of like a bullet point format the best that you can you can include headers as well to just categorize the issue and if you want to add a brief description that you section don't make it any more than four sentences long. Got it. Do you need any clarification?"

### [P3] Ingestion Engine (IE)

**Goal:** Automate data retrieval from CFTC and FX Providers.

- [x] **[IE-v1] The File-System MVP (Disk-Bound)**
    
    - Objective: Successfully pull raw bytes from CFTC/Oanda and write to `/data/raw/`.
        
    - [x] _Task:_ Implement `reqwest` for HTTP GET and `std::fs` for file persistence.

        
    - [x] _Validation:_ Manually open CSV in Excel/Text Editor to ensure no encoding issues.

- [ ] **[IE-v2] The Database Bridge (Hybrid)**
    
    - Objective: Read the validated CSVs from disk and batch-insert into Postgres.
        
    - [ ] _Task:_ Use `csv` crate for parsing and `sqlx` (or `postgres`) for `COPY` or `INSERT` commands.
        
    - [ ] _Validation:_ Run `SELECT COUNT(*)` in Postgres to match CSV row counts.
        
- [ ] **[IE-v3] The Direct Pipeline (Stream-to-DB)**
    
    - Objective: Bypass the physical disk entirely.
        
    - [ ] _Task:_ Refactor `cot_fetch.rs` to pipe the HTTP response body directly into a parser and then into a DB transaction.
        
    - [ ] _Validation:_ Successful ingestion without a trace of a `.csv` file in the `/data/` folder.
        

# [P4] Transformation & Intelligence (TI)

Goal: Now the data becomes intelligence, not numbers.

Objectives & Solutions

- [ ] Compute net positioning.

Execution & Solution:

- [ ] Normalize by open interest.

Execution & Solution:

- [ ] Segment by trader class.

Execution & Solution:

- [ ] Build rolling windows (3y/5y).

Execution & Solution:

- [ ] Calculate: Z-scores, Percentile ranks, 1w/4w deltas, Acceleration.

Execution & Solution:

- [ ] Store results in processed tables.

Execution & Solution:

- [ ] Validate outputs (no nulls, sane ranges).

Execution & Solution:

# [P5] Price & Volatility Context (PV)

Goal: This is your risk amplifier lens.

Objectives & Solutions

- [ ] Choose volatility source (Implied preferred, Realized fallback).

Execution & Solution:

- [ ] Ingest weekly FX prices.

Execution & Solution:

- [ ] Align prices to COT weeks.

Execution & Solution:

- [ ] Compute volatility regimes (Compressed, Normal, Expanded).

Execution & Solution:

- [ ] Store separately, no mixing concerns.

Execution & Solution:

# [P6] Pair-Level Synthesis (PS)

Goal: Construct relative positioning logic and classify the weekly verdict.

Objectives & Solutions

- [ ] Construct relative positioning logic.

Execution & Solution:

- [ ] Compare base vs quote currency.

Execution & Solution:

- [ ] Detect: Crowding, Unwinds, Pressure asymmetry.

Execution & Solution:

- [ ] Classify each pair weekly (Accumulation, Distribution, Crowded Trend, Unstable Extreme, Neutral).

Execution & Solution:

# [P7] Automation (AU)

Goal: Once this works, humans are removed from the loop (Non-Negotiable).

Objectives & Solutions

- [ ] Write cron job.

Execution & Solution:

- [ ] Set fixed weekly execution time.

Execution & Solution:

- [ ] Redirect logs to file.

Execution & Solution:

- [ ] Test failure cases.

Execution & Solution:

- [ ] Confirm: Idempotency, No duplicate inserts, Clean reruns.

Execution & Solution:

# [P8] Power BI (PB)

Goal: Presentation, not thinking. If Power BI calculates anything heavy, you failed earlier.

Objectives & Solutions

- [ ] Connect Power BI to Postgres (read-only).

Execution & Solution:

- [ ] Build star schema model.

Execution & Solution:

- [ ] Create measures only for display.

Execution & Solution:

- [ ] Create mandatory panels (Positioning heatmap, Extremes table, Momentum view, Volatility overlay, Pair verdict summary).

Execution & Solution:

- [ ] Enforce weekly snapshot logic.

Execution & Solution:

# [P9] Validation & Audit (VA)

Goal: This is where credibility is earned.

Objectives & Solutions

- [ ] Backtest positioning extremes vs outcomes (qualitative).

Execution & Solution:

- [ ] Verify no lookahead bias.

Execution & Solution:

- [ ] Validate time lags.

Execution & Solution:

- [ ] Confirm weekly cadence integrity.

Execution & Solution:

# [P10] Portfolio Narrative (PN)

Goal: Document the system for serious traders and quant teams.

Objectives & Solutions

- [ ] Write a 1-page system explanation.

Execution & Solution:

- [ ] Define what problem it solves.

Execution & Solution:

- [ ] Explain why COT matters in FX.

Execution & Solution:

- [ ] Explain volatility interaction.

Execution & Solution:

- [ ] Show example weeks (screenshots).

Execution & Solution:
