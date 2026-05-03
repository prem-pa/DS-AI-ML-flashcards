---
id: e5392ad4-9130-4c32-9133-a703fa05c504
title: Subqueries and CTEs (WITH clauses)
track: data-scientist
topic: sql-data-manipulation
difficulty: 3
tags:
- modularity
- readability
- query-decomposition
- performance
aliases:
- nested queries
- common table expressions
- CTE patterns
- subquery optimization
sources:
- url: https://www.postgresql.org/docs/current/queries-with.html
  label: PostgreSQL CTE Documentation
- url: https://neon.com/postgresql/tutorial/cte
  label: Neon CTE Tutorial
- url: https://mode.com/sql-tutorial/advanced-sql-tutorial/
  label: Mode Analytics CTE Patterns
cards:
- id: 90b5f6f9-b750-4db3-8990-adfb6a9745ca
  type: flip
  front: ''
  back: ''
- id: 8dc7b00e-7c70-4bcb-9f00-27f620528886
  type: flip
  front: ''
  back: ''
- id: 980d4fca-7220-4db0-9089-39e9e05c5bd3
  type: mcq
  front: ''
  back: Correlated subqueries execute once per outer row, creating N+1 query risk. This is why they're
    usually slower than CTEs or JOINs.
  choices:
  - key: a
    text: Once (subqueries always execute once)
    correct: false
  - key: b
    text: Once per row in customers table
    correct: true
  - key: c
    text: Once per DISTINCT value in the join key
    correct: false
  - key: d
    text: Zero times (correlated subqueries are optimized away)
    correct: false
- id: 976ec5cf-e5a3-4ffb-9ba1-ef27c1eda9f0
  type: flip
  front: ''
  back: ''
---

## Intuition

Subqueries embed queries within queries; CTEs (Common Table Expressions) define named result sets before the main query. CTEs improve readability and allow referencing the same result set multiple times. Use CTEs to break complex logic into digestible steps.

## Detail

Subquery example (less readable):
```sql
SELECT customer_id, (SELECT COUNT(*) FROM orders WHERE customer_id = c.customer_id) as order_count
FROM customers c;
```

CTE example (more readable):
```sql
WITH customer_orders AS (
  SELECT customer_id, COUNT(*) as order_count
  FROM orders
  GROUP BY customer_id
)
SELECT c.customer_id, c.name, COALESCE(co.order_count, 0) as order_count
FROM customers c
LEFT JOIN customer_orders co ON c.customer_id = co.customer_id;
```

Multiple CTEs:
```sql
WITH high_value_orders AS (
  SELECT customer_id, SUM(amount) as total
  FROM orders
  GROUP BY customer_id
  HAVING SUM(amount) > 10000
),
top_customers AS (
  SELECT hvo.customer_id, c.name, hvo.total
  FROM high_value_orders hvo
  JOIN customers c ON hvo.customer_id = c.customer_id
)
SELECT * FROM top_customers
ORDER BY total DESC;
```

Subquery types:
- Scalar subqueries: return single value
- Correlated subqueries: reference outer query (slower)
- Subqueries in FROM: inline views
- Subqueries in WHERE: filtering with complex logic

## Common gotchas / interview framings
- **Gotcha**: Scalar subqueries that return multiple rows cause errors
- **Gotcha**: Correlated subqueries are slow (N+1 risk)
- **Gotcha**: CTEs don't auto-materialize; some databases evaluate inline
- **Interview**: "Optimize this correlated subquery"
- **Interview**: "Refactor this deeply nested query as CTEs"

## See also
- [[cte-basics]]
- [[subquery-types]]
- [[inline-views]]
- [[materialized-ctes]]
- [[performance-ctes]]
- [[recursive-preview]]

## Sources
See frontmatter `sources:`.
