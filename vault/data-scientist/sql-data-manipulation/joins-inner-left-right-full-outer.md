---
id: 775e29e9-3f58-4de1-949a-1dba8c781b8f
title: Joins (INNER, LEFT, RIGHT, FULL OUTER)
track: data-scientist
topic: sql-data-manipulation
difficulty: 3
tags:
- joins
- cardinality
- relational-logic
- data-combination
aliases:
- join types
- table joining
- combining tables
- join semantics
sources:
- url: https://www.postgresql.org/docs/current/tutorial-join.html
  label: PostgreSQL JOIN Tutorial
- url: https://mode.com/sql-tutorial/sql-joins/
  label: Mode Analytics JOIN Guide
- url: https://dataschool.com/sql-optimization/joins/
  label: DataSchool Advanced JOIN Patterns
cards:
- id: 15c74746-635b-498a-85ba-b2cef069fcee
  type: flip
  front: ''
  back: ''
- id: 033c759a-e429-4c17-a5a4-14891e84870d
  type: mcq
  front: ''
  back: With INNER JOIN returning 4500 rows from 5000 order rows, either 500 orders lack customers OR
    the join is one-to-many. You need more context.
  choices:
  - key: a
    text: 500 customers have no orders
    correct: false
  - key: b
    text: 500 orders have no matching customer
    correct: false
  - key: c
    text: Both A and B are possible
    correct: true
  - key: d
    text: One table has duplicate keys
    correct: false
- id: c300ee03-4c32-46e9-8e51-af824f493ac7
  type: flip
  front: ''
  back: ''
- id: 5dd1654f-68dc-4ef0-9c65-487ece87c50d
  type: flip
  front: ''
  back: ''
- id: 276f8096-8c4e-408d-a070-c6b832de4337
  type: mcq
  front: ''
  back: In SQL, NULL is never equal to anything, including NULL. Left rows with NULL keys won't match
    right rows, even in LEFT JOIN.
  choices:
  - key: a
    text: The row is included with NULL right-side columns
    correct: false
  - key: b
    text: The row is excluded (same as INNER JOIN)
    correct: true
  - key: c
    text: The NULL is treated as a special value and matches
    correct: false
  - key: d
    text: PostgreSQL throws an error
    correct: false
---

## Intuition

JOINs combine rows from multiple tables based on a join condition. INNER JOIN keeps only matching rows, LEFT JOIN keeps all left rows plus matches, RIGHT JOIN keeps all right rows plus matches, and FULL OUTER JOIN keeps all rows from both tables. Understanding cardinality (one-to-one, one-to-many, many-to-many) is essential.

## Detail

```sql
-- INNER JOIN: only matching rows
SELECT o.order_id, c.customer_name, o.amount
FROM orders o
INNER JOIN customers c ON o.customer_id = c.customer_id;

-- LEFT JOIN: all left rows + matches
SELECT c.customer_id, c.name, COUNT(o.order_id) as order_count
FROM customers c
LEFT JOIN orders o ON c.customer_id = o.customer_id
GROUP BY c.customer_id, c.name;

-- RIGHT JOIN: all right rows + matches (less common)
SELECT o.order_id, c.customer_name
FROM orders o
RIGHT JOIN customers c ON o.customer_id = c.customer_id;

-- FULL OUTER JOIN: all rows from both tables
SELECT COALESCE(c.customer_id, o.customer_id) as customer_id,
       c.name, o.order_id
FROM customers c
FULL OUTER JOIN orders o ON c.customer_id = o.customer_id;
```

Cardinality matters:
- One-to-one: joins don't multiply rows
- One-to-many: left table rows multiply if multiple matches
- Many-to-many: both tables multiply, creating large result sets

Common pitfall: joining on non-unique keys without considering cardinality.

## Common gotchas / interview framings
- **Gotcha**: Silent row multiplication from one-to-many joins
- **Gotcha**: NULL join keys don't match (even NULL = NULL is false)
- **Gotcha**: Filtering in WHERE after LEFT JOIN eliminates the "left" protection
- **Interview**: "What's the difference between filtering in WHERE vs ON clause?"
- **Interview**: "Why did my row count explode after joining?" (cardinality issue)

## See also
- [[inner-join]]
- [[left-join]]
- [[right-join]]
- [[full-outer-join]]
- [[cross-join]]
- [[join-cardinality]]
- [[join-conditions]]

## Sources
See frontmatter `sources:`.
