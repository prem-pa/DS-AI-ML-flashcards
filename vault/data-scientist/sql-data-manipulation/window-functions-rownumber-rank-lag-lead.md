---
id: e2edf3b7-2177-420f-a6bf-4866855425d7
title: Window functions (ROW_NUMBER, RANK, LAG, LEAD)
track: data-scientist
topic: sql-data-manipulation
difficulty: 3
tags:
- ranking
- ordering
- analytical
- time-series
aliases:
- window analytics
- ranking functions
- lag lead functions
- running calculations
sources:
- url: https://www.postgresql.org/docs/current/tutorial-window.html
  label: PostgreSQL Window Functions
- url: https://neon.com/postgresql/postgresql-window-function
  label: Neon Window Functions Guide
- url: https://mode.com/sql-tutorial/sql-window-functions/
  label: Mode Analytics Window Functions
cards:
- id: 53b20d54-b730-4d47-b2aa-f89e41161e32
  type: flip
  front: ''
  back: ''
- id: 7892347d-7f5d-42d8-962e-28292d927308
  type: mcq
  front: ''
  back: ROW_NUMBER assigns unique numbers (1,2,3). RANK skips after ties (1,1,3). DENSE_RANK is consecutive
    (1,1,2).
  choices:
  - key: a
    text: 'ROW_NUMBER: 1,2; RANK: 1,1; DENSE_RANK: 1,1'
    correct: true
  - key: b
    text: All return 1,1,2
    correct: false
  - key: c
    text: All return 1,2,3
    correct: false
  - key: d
    text: 'ROW_NUMBER: 1,1; RANK: 1,2; DENSE_RANK: 1,1'
    correct: false
- id: 080ed425-de44-4ef6-942d-10652bb65cb6
  type: flip
  front: ''
  back: ''
- id: a5e6dc73-a0a9-48a6-883d-4d6fb11f678e
  type: flip
  front: ''
  back: ''
---

## Intuition

Window functions compute values across a set of rows (a "window") without collapsing rows like aggregates do. ROW_NUMBER ranks rows uniquely, RANK handles ties, LAG/LEAD access previous/next rows, and cumulative functions compute running totals. Perfect for analytics: ranks, percent-of-total, row comparisons.

## Detail

```sql
SELECT 
  employee_id,
  salary,
  -- Ranking functions
  ROW_NUMBER() OVER (ORDER BY salary DESC) as row_num,
  RANK() OVER (ORDER BY salary DESC) as rank,
  DENSE_RANK() OVER (ORDER BY salary DESC) as dense_rank,
  -- Comparison functions
  LAG(salary) OVER (ORDER BY salary DESC) as prev_salary,
  LEAD(salary) OVER (ORDER BY salary DESC) as next_salary,
  salary - LAG(salary) OVER (ORDER BY salary DESC) as salary_diff,
  -- Aggregates as window functions
  SUM(salary) OVER (ORDER BY salary DESC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) as running_total,
  AVG(salary) OVER (ORDER BY salary DESC) as avg_all,
  -- With partitioning
  ROW_NUMBER() OVER (PARTITION BY department ORDER BY salary DESC) as dept_rank
FROM employees
ORDER BY salary DESC;
```

Key concepts:
- PARTITION BY divides rows into groups (like GROUP BY but doesn't collapse rows)
- ORDER BY defines ranking/ordering within window
- Frame specification (ROWS BETWEEN) defines range for aggregate window functions
- Window functions don't reduce row count (unlike GROUP BY)

Common patterns:
```sql
-- Top N per group
WITH ranked_employees AS (
  SELECT *, ROW_NUMBER() OVER (PARTITION BY department ORDER BY salary DESC) as rank
  FROM employees
)
SELECT * FROM ranked_employees WHERE rank <= 3;

-- Month-over-month change
SELECT month, revenue, LAG(revenue) OVER (ORDER BY month) as prev_month_revenue,
       revenue - LAG(revenue) OVER (ORDER BY month) as revenue_change
FROM monthly_revenue;
```

## Common gotchas / interview framings
- **Gotcha**: ROW_NUMBER assigns unique ranks even for ties; RANK creates gaps; DENSE_RANK is consecutive
- **Gotcha**: LAG/LEAD with no PARTITION treats entire result set as one window (often not what you want)
- **Gotcha**: Misunderstanding frame specifications (ROWS BETWEEN) causes wrong running totals
- **Interview**: "Write a query to get the top 3 earners per department"
- **Interview**: "Calculate month-over-month revenue change using window functions"

## See also
- [[row-number]]
- [[rank]]
- [[dense-rank]]
- [[lag-function]]
- [[lead-function]]
- [[partition-by]]
- [[order-by-window]]

## Sources
See frontmatter `sources:`.
