---
id: b5049840-a29e-43d4-adad-9f72adbee762
title: Set operations (UNION, INTERSECT, EXCEPT)
track: data-scientist
topic: sql-data-manipulation
difficulty: 3
tags:
- set-theory
- combining-results
- deduplication
- comparison
aliases:
- union queries
- intersect except
- set combination
- result merging
sources:
- url: https://www.postgresql.org/docs/current/queries-union.html
  label: PostgreSQL Set Operations
- url: https://mode.com/sql-tutorial/sql-union/
  label: Mode Analytics UNION Tutorial
- url: https://dataschool.com/sql-optimization/set-operations/
  label: DataSchool Set Operations
cards:
- id: 4d3d8034-07cc-423f-aa7b-f409dc590b77
  type: flip
  front: ''
  back: ''
- id: f1f62423-e377-45ea-9b75-95aa43ab4c82
  type: mcq
  front: ''
  back: UNION merges and deduplicates. 1000 + 800 - 200 = 1600 unique rows. UNION ALL would return 1800.
  choices:
  - key: a
    text: 1000 (only A's rows)
    correct: false
  - key: b
    text: 1800 (all rows)
    correct: false
  - key: c
    text: 1600 (A + non-duplicate B rows)
    correct: true
  - key: d
    text: 2000 (A + B rows)
    correct: false
- id: 86f64e05-a3f2-4466-a284-a004b6202234
  type: flip
  front: ''
  back: ''
- id: dbee9966-fa40-4697-9c0f-7a498ef5d078
  type: flip
  front: ''
  back: ''
---

## Intuition

Set operations combine multiple queries: UNION merges result sets (removing duplicates), UNION ALL merges without dedup, INTERSECT finds common rows, and EXCEPT finds rows in first set but not second. Used for combining data from different sources or filtering based on conditions in other tables.

## Detail

```sql
-- UNION: combine and deduplicate
SELECT customer_id, 'online' as channel FROM online_customers
UNION
SELECT customer_id, 'offline' as channel FROM offline_customers;

-- UNION ALL: combine without deduplicating (faster)
SELECT order_id FROM 2024_orders
UNION ALL
SELECT order_id FROM 2025_orders;

-- INTERSECT: find common rows
SELECT customer_id FROM customers_bought_electronics
INTERSECT
SELECT customer_id FROM customers_bought_clothing;

-- EXCEPT: set difference
SELECT customer_id FROM all_customers
EXCEPT
SELECT customer_id FROM customers_with_active_orders;
```

Rules:
- All queries must have the same number of columns
- Column types must be compatible
- Column names come from first query
- UNION removes duplicates (slower); UNION ALL doesn't
- INTERSECT and EXCEPT are less common but powerful

Performance note: UNION is expensive due to deduplication. Use UNION ALL if you're confident there are no duplicates.

## Common gotchas / interview framings
- **Gotcha**: UNION auto-deduplicates; UNION ALL doesn't (know the difference)
- **Gotcha**: Column count mismatch or type mismatch causes errors
- **Gotcha**: ORDER BY applies to entire result, not individual queries
- **Interview**: "Find customers who bought from both online and offline channels"
- **Interview**: "List all transactions excluding refunds" (use EXCEPT)

## See also
- [[union-operation]]
- [[intersect-operation]]
- [[except-operation]]
- [[union-all]]
- [[duplicate-handling]]
- [[column-alignment]]

## Sources
See frontmatter `sources:`.
