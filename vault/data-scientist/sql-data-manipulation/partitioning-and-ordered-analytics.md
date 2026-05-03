---
id: 91791fc5-8187-44a6-bd13-36044e10e554
title: Partitioning and ordered analytics
track: data-scientist
topic: sql-data-manipulation
difficulty: 3
tags:
- partitioning
- analytics
- cohort-analysis
- funnel-analysis
aliases:
- partition by
- analytical functions
- group analytics
- windowed aggregates
sources:
- url: https://www.postgresql.org/docs/current/sql-expressions.html#syntax-window-functions
  label: PostgreSQL Window Function Syntax
- url: https://mode.com/sql-tutorial/sql-window-functions/
  label: Mode Analytics Partitioning Guide
- url: https://dataschool.com/sql-optimization/cohort-retention-analysis/
  label: DataSchool Cohort Analysis
cards:
- id: e249a0d3-43d4-43b3-b3d9-05ec897a4344
  type: flip
  front: ''
  back: ''
- id: 61e4b603-411d-4b4d-aa42-e8f56b65ce59
  type: flip
  front: ''
  back: ''
- id: 80d5b66d-6ee9-4223-93e2-bea857255f80
  type: mcq
  front: ''
  back: Days 1, 2, 3 are only 3 rows (no previous 6 days). Frame expands as you approach day 7.
  choices:
  - key: a
    text: 7 rows (days 1-7)
    correct: false
  - key: b
    text: 3 rows (days 1-3)
    correct: true
  - key: c
    text: 4 rows (days 0-3, where day 0 is out of bounds)
    correct: false
  - key: d
    text: Depends on how many rows exist before day 3
    correct: false
- id: e84e16aa-d588-4936-adca-d87f4284051b
  type: flip
  front: ''
  back: ''
---

## Intuition

Partitioning divides result sets into groups and applies aggregations within each group without reducing row count. Essential for analytics: cohort analysis (comparing user groups), funnel analysis (tracking progression), and running totals. PARTITION BY groups rows; ORDER BY defines ordering within partitions.

## Detail

```sql
-- Running totals within partitions
SELECT 
  user_id,
  transaction_date,
  amount,
  SUM(amount) OVER (PARTITION BY user_id ORDER BY transaction_date) as cumulative_spend,
  AVG(amount) OVER (PARTITION BY user_id) as avg_user_transaction
FROM transactions
ORDER BY user_id, transaction_date;

-- Cohort analysis: compare user groups by signup month
WITH cohorts AS (
  SELECT 
    DATE_TRUNC('month', signup_date)::DATE as cohort_month,
    user_id,
    DATE_TRUNC('month', first_purchase_date)::DATE as first_purchase_month
  FROM users
)
SELECT 
  cohort_month,
  (DATE_PART('month', first_purchase_month) - DATE_PART('month', cohort_month))::INT as months_to_first_purchase,
  COUNT(DISTINCT user_id) as user_count
FROM cohorts
GROUP BY cohort_month, months_to_first_purchase
ORDER BY cohort_month, months_to_first_purchase;

-- Funnel analysis: track progression through steps
WITH user_journey AS (
  SELECT 
    user_id,
    CASE WHEN action = 'view' THEN 1 ELSE 0 END as viewed,
    CASE WHEN action = 'add_to_cart' THEN 1 ELSE 0 END as added_to_cart,
    CASE WHEN action = 'purchase' THEN 1 ELSE 0 END as purchased
  FROM events
)
SELECT 
  COUNT(DISTINCT CASE WHEN viewed = 1 THEN user_id END) as viewers,
  COUNT(DISTINCT CASE WHEN added_to_cart = 1 THEN user_id END) as added_to_cart_users,
  COUNT(DISTINCT CASE WHEN purchased = 1 THEN user_id END) as purchasers
FROM user_journey;

-- Frame specification for running aggregates
SELECT 
  date,
  revenue,
  SUM(revenue) OVER (ORDER BY date ROWS BETWEEN 6 PRECEDING AND CURRENT ROW) as running_7day_total,
  AVG(revenue) OVER (ORDER BY date ROWS BETWEEN 29 PRECEDING AND CURRENT ROW) as moving_30day_avg
FROM daily_revenue;
```

Frame types:
- ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW: cumulative
- ROWS BETWEEN 6 PRECEDING AND CURRENT ROW: rolling window
- ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING: entire partition

## Common gotchas / interview framings
- **Gotcha**: Forgetting PARTITION BY when you need group-level analytics
- **Gotcha**: Misunderstanding frame specifications (ROWS vs RANGE)
- **Gotcha**: Performance issues with deep partitions (millions of rows per partition)
- **Interview**: "Calculate 7-day rolling average of revenue"
- **Interview**: "Build a cohort retention table"

## See also
- [[partition-by-clause]]
- [[over-clause]]
- [[frame-specification]]
- [[running-aggregates]]
- [[cohort-analysis]]
- [[funnel-analysis]]

## Sources
See frontmatter `sources:`.
