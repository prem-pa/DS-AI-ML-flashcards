---
id: 00e14231-6d1f-490d-958f-8c9e8e53d920
title: CASE statements for conditional logic
track: data-scientist
topic: sql-data-manipulation
difficulty: 3
tags:
- conditional
- logic
- categorization
- metrics
aliases:
- if-then logic
- conditional expressions
- categorization logic
sources:
- url: https://www.postgresql.org/docs/current/functions-conditional.html
  label: PostgreSQL CASE Documentation
- url: https://mode.com/sql-tutorial/sql-case/
  label: Mode Analytics CASE Tutorial
- url: https://dataschool.com/sql-optimization/case-statements/
  label: DataSchool CASE Patterns
cards:
- id: 63a345a3-fed7-4290-a381-d54cbee9f1d7
  type: flip
  front: ''
  back: ''
- id: 05fdba3c-c853-4426-9c19-0998f05cf965
  type: mcq
  front: ''
  back: CASE without matching WHEN and no ELSE returns NULL. Always include ELSE to handle unexpected
    values.
  choices:
  - key: a
    text: 0 (zero)
    correct: false
  - key: b
    text: 'NULL'
    correct: true
  - key: c
    text: An error
    correct: false
  - key: d
    text: An empty string
    correct: false
- id: d2c09327-0e8d-4936-9c36-65dd8609af33
  type: flip
  front: ''
  back: ''
- id: 896cd321-0db3-44bc-9e5d-04210ca37a4e
  type: flip
  front: ''
  back: ''
---

## Intuition

CASE statements enable if-then-else logic within queries, crucial for creating metrics and categorizing data. Two forms exist: simple CASE (comparing one column to values) and searched CASE (evaluating conditions). CASE transforms data on-the-fly without separate transformations.

## Detail

Simple CASE:
```sql
SELECT 
  order_id,
  status,
  CASE status
    WHEN 'completed' THEN 'Success'
    WHEN 'pending' THEN 'In Progress'
    WHEN 'cancelled' THEN 'Failed'
    ELSE 'Unknown'
  END as status_category
FROM orders;
```

Searched CASE (more flexible):
```sql
SELECT 
  employee_id,
  salary,
  CASE
    WHEN salary >= 100000 THEN 'Senior'
    WHEN salary >= 75000 THEN 'Mid-level'
    WHEN salary >= 50000 THEN 'Junior'
    ELSE 'Entry-level'
  END as salary_band,
  -- Nested CASE for complex logic
  CASE
    WHEN salary >= 100000 AND years_employed >= 5 THEN 'Promote'
    WHEN salary >= 100000 AND years_employed < 5 THEN 'Monitor'
    ELSE 'Standard'
  END as action
FROM employees;
```

CASE in aggregates (common pattern):
```sql
SELECT 
  SUM(CASE WHEN status = 'completed' THEN 1 ELSE 0 END) as completed_count,
  SUM(CASE WHEN status = 'pending' THEN 1 ELSE 0 END) as pending_count,
  SUM(CASE WHEN status = 'completed' THEN amount ELSE 0 END) as completed_revenue,
  COUNT(CASE WHEN referral_source IS NOT NULL THEN 1 END) as referred_customers
FROM orders
GROUP BY order_date;
```

Common pattern: conditional counts
```sql
SELECT COUNT(*) as total_orders,
       COUNT(CASE WHEN amount > 1000 THEN 1 END) as high_value_orders
FROM orders;
```

## Common gotchas / interview framings
- **Gotcha**: CASE without ELSE returns NULL for non-matching rows
- **Gotcha**: Order matters; first matching WHEN wins (like if-elseif chains)
- **Gotcha**: Type coercion in THEN clauses can cause errors
- **Interview**: "Write a query that counts orders by status using CASE"
- **Interview**: "Create a revenue metric that excludes cancelled orders"

## See also
- [[case-simple]]
- [[case-searched]]
- [[when-then]]
- [[else-clause]]
- [[null-handling-case]]
- [[aggregation-with-case]]

## Sources
See frontmatter `sources:`.
