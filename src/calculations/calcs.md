## Exact Calculations Needed

**From COT Data:**
1. Net Position = Asset_Mgr_Long - Asset_Mgr_Short (repeat for Leveraged_Funds)
2. Net Position % = Net Position / Open_Interest × 100
3. Z-Score = (Current Net % - Mean(5yr)) / StdDev(5yr)
4. Percentile Rank = Rank Current Net % within 5-year history (0-100)
5. 1-Week Delta = Current Net % - Previous Week Net %
6. 4-Week Delta = Current Net % - Net % from 4 weeks ago
7. Acceleration = Current 1w Delta - Previous 1w Delta
8. Open Interest Change = Current OI - Prior Week OI

**From FX Prices (join by date to COT report):**
9. Price Tuesday = Close on report date
10. Price Friday = Close on report date + 3 days
11. Price Delta % = (Price Friday - Price Tuesday) / Price Tuesday × 100

**Classification:**
12. Market Response = IF (Net bullish AND Price up) THEN "Bullish Conviction" ELSE... (logic varies by pair/trend)

---

**Output:** One row per (report_date, tff_code, trader_type) with all 12 calculated columns → CSV/JSON → PostgreSQL