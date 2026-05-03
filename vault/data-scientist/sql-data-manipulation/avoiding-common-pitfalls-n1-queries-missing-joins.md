---
id: 22959dee-242e-4a58-9d11-094974dc1517
title: Avoiding common pitfalls (N+1 queries, missing JOINs)
track: data-scientist
topic: sql-data-manipulation
difficulty: 3
tags:
- performance
- best-practices
- anti-patterns
- efficiency
aliases:
- N+1 problem
- query anti-patterns
- common mistakes
- data quality
sources:
- url: https://dataschool.com/sql-optimization/preventing-common-errors/
  label: DataSchool Common SQL Errors
- url: https://mode.com/sql-tutorial/sql-best-practices/
  label: Mode Analytics Best Practices
- url: https://www.postgresql.org/docs/current/sql-syntax.html
  label: PostgreSQL SQL Syntax Guide
cards:
- id: 4d5a17b6-a41c-490c-a6c7-914843a0ffc4
  type: flip
  front: ''
  back: ''
- id: 9c3cb6d4-7dd5-4223-8fce-b83bd7b5eac7
  type: mcq
  front: ''
  back: 'N+1 problem: 1 initial query + N queries in the loop = 1001 total. A single JOIN query would
    use 1 query.'
  choices:
  - key: a
    text: 1 query (customers with orders)
    correct: false
  - key: b
    text: 1001 queries (1 for all customers, 1000 for each customer's orders)
    correct: true
  - key: c
    text: 1000 queries (orders per customer)
    correct: false
  - key: d
    text: Depends on caching
    correct: false
- id: 1719d25e-f555-48e2-b3ed-af3b6708ca1d
  type: flip
  front: ''
  back: ''
- id: 027b06ac-860b-44c5-8ab2-9fd2f7361baf
  type: mcq
  front: ''
  back: All are useful. Check DISTINCT to see if rows duplicated. EXPLAIN shows join cardinality. Compare
    row count to expectations.
  choices:
  - key: a
    text: Count DISTINCT values in key columns
    correct: false
  - key: b
    text: Run EXPLAIN ANALYZE to see cardinality
    correct: false
  - key: c
    text: Check if the row count equals expected rows * number of joined tables
    correct: false
  - key: d
    text: All of the above
    correct: true
---

## Intuition

Common SQL mistakes reduce query efficiency and data integrity. N+1 queries run a query for each row (exponential explosion). Missing joins create Cartesian products or incorrect aggregations. Implicit logic in code instead of SQL violates separation of concerns. Learning these pitfalls prevents slow/incorrect queries in interviews and production.

## Detail

### N+1 Query Problem
```python
# ANTI-PATTERN: N+1 queries in code
customers = get_all_customers()  # 1 query
for customer in customers:  # N iterations
  orders = get_orders_for_customer(customer.id)  # N queries
  print(customer.name, orders)

# SOLUTION: Single SQL query
SELECT c.name, COUNT(o.order_id) as order_count
FROM customers c
LEFT JOIN orders o ON c.customer_id = o.customer_id
GROUP BY c.customer_id, c.name;
```

### Missing JOIN: Cartesian Product
```sql
-- WRONG: Joins on wrong key (or no key)
SELECT * FROM customers, orders;
-- Result: every customer matched to every order (M * N rows)

-- CORRECT: Join on relationship
SELECT * FROM customers c
JOIN orders o ON c.customer_id = o.customer_id;
```

### Missing JOIN in Aggregation
```sql
-- WRONG: Forgot to join order_items; counts products not orders
SELECT o.order_id, COUNT(*)
FROM orders o
GROUP BY o.order_id;  -- Counts 1 (only orders row)

-- CORRECT: Include items
SELECT o.order_id, COUNT(oi.item_id)
FROM orders o
JOIN order_items oi ON o.order_id = oi.order_id
GROUP BY o.order_id;  -- Counts items per order
```

### Duplicated Data from Many-to-Many Join
```sql
-- WRONG: Double-counts revenue when joining orders + order_items + promotions
SELECT o.order_id, SUM(oi.price) as revenue
FROM orders o
JOIN order_items oi ON o.order_id = oi.order_id
JOIN promotions p ON oi.item_id = p.item_id
GROUP BY o.order_id;  -- If item has 2 promotions, counted twice!

-- CORRECT: Use derived tables
WITH item_revenue AS (
  SELECT order_id, SUM(price) as revenue
  FROM order_items
  GROUP BY order_id
)
SELECT ir.order_id, ir.revenue
FROM item_revenue ir
JOIN promotions p ON ir.order_id = p.order_id;
```

### Logic in Code vs SQL
```python
# ANTI-PATTERN: Business logic in code
orders = select_all_orders()
high_value = [o for o in orders if o.amount > 1000 and o.status == 'completed']
revenue = sum(o.amount for o in high_value)

# CORRECT: Logic in SQL
SELECT SUM(amount) as revenue
FROM orders
WHERE amount > 1000 AND status = 'completed';
```

## Common gotchas / interview framings
- **Gotcha**: Forgetting to verify join keys are primary/foreign key relationships
- **Gotcha**: Counting rows when you meant to count distinct entities
- **Gotcha**: N+1 queries in loops (appears in code reviews)
- **Interview**: "This query returns wrong counts. Debug it."
- **Interview**: "Optimize this code loop that runs a query for each row"

## See also
- [[n-plus-1-queries]]
- [[missing-join-keys]]
- [[incorrect-aggregation]]
- [[duplicate-logic]]
- [[query-smell-tests]]

## Sources
See frontmatter `sources:`.
