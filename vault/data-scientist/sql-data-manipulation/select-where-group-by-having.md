---
id: d59eaa09-6cb9-4277-97ff-dcf7cf48feed
title: SELECT, WHERE, GROUP BY, HAVING
track: data-scientist
topic: sql-data-manipulation
difficulty: 1
tags:
- fundamentals
- filtering
- aggregation
- grouping
aliases:
- basic query structure
- filtering data
- group and filter
sources:
- url: https://www.postgresql.org/docs/current/sql-select.html
  label: PostgreSQL SELECT Documentation
- url: https://mode.com/sql-tutorial/sql-where/
  label: Mode Analytics SQL WHERE Tutorial
- url: https://dataschool.com/sql-optimization/understanding-group-by/
  label: DataSchool GROUP BY Guide
cards:
- id: 6deddf76-fcfd-4481-a56e-9c997f6c7e26
  type: flip
  front: ''
  back: ''
- id: f6c37234-dfc0-4d5a-a9cf-16c9e864568a
  type: mcq
  front: ''
  back: WHERE operates on individual rows before GROUP BY, while HAVING operates on aggregated groups
    after GROUP BY.
  choices:
  - key: a
    text: WHERE filters rows before grouping; HAVING filters groups after aggregation
    correct: true
  - key: b
    text: WHERE and HAVING are identical and can be used interchangeably
    correct: false
  - key: c
    text: HAVING is used for non-grouped columns; WHERE is for grouped columns
    correct: false
  - key: d
    text: WHERE is obsolete and HAVING should always be used instead
    correct: false
- id: a6f87d29-4eec-4b54-be30-72a2ee95eb98
  type: flip
  front: ''
  back: ''
- id: 8fa7a3ed-6fcc-46d7-a8ea-77078ed40a5d
  type: flip
  front: ''
  back: ''
---

## Intuition

SELECT, WHERE, GROUP BY, and HAVING are the foundational clauses for querying data. They form the core pipeline: SELECT specifies which columns to retrieve, WHERE filters rows before grouping, GROUP BY aggregates rows into groups, and HAVING filters groups after aggregation.

## Detail

The execution order matters:
1. FROM/JOIN tables
2. WHERE filters individual rows
3. GROUP BY aggregates matching rows
4. HAVING filters aggregated groups
5. SELECT chooses final columns
6. ORDER BY sorts results

Example schema:
```sql
CREATE TABLE sales (
  id INT,
  product_name VARCHAR(50),
  amount DECIMAL(10, 2),
  date DATE
);
```

Query:
```sql
SELECT 
  product_name,
  COUNT(*) as order_count,
  SUM(amount) as total_revenue
FROM sales
WHERE date >= '2025-01-01'
GROUP BY product_name
HAVING SUM(amount) > 1000
ORDER BY total_revenue DESC;
```

Key distinctions:
- WHERE filters before grouping (row-level predicates)
- HAVING filters after grouping (group-level predicates)
- You can only SELECT columns that are grouped or aggregated

## Common gotchas / interview framings
- **Gotcha**: Putting non-grouped, non-aggregated columns in SELECT
- **Gotcha**: Using WHERE instead of HAVING for group filters
- **Interview**: "Write a query that shows products with total sales > 1000"
- **Interview**: "What's wrong with this GROUP BY query?" (testing WHERE vs HAVING knowledge)

## See also
- [[where-clause]]
- [[group-by-clause]]
- [[having-clause]]
- [[query-execution-order]]
- [[aggregation-fundamentals]]
- [[filtering-and-grouping]]

## Sources
See frontmatter `sources:`.
