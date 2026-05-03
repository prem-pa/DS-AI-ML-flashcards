---
id: fb023f9d-6606-4e42-b627-43a59124ca69
title: Aggregation functions (COUNT, SUM, AVG, MAX)
track: data-scientist
topic: sql-data-manipulation
difficulty: 1
tags:
- aggregation
- summarization
- analytics
- null-handling
aliases:
- aggregate functions
- summary statistics
- data summarization
sources:
- url: https://www.postgresql.org/docs/current/functions-aggregate.html
  label: PostgreSQL Aggregate Functions
- url: https://mode.com/sql-tutorial/sql-aggregate-functions/
  label: Mode Analytics Aggregate Functions
- url: https://dataschool.com/sql-optimization/aggregations/
  label: DataSchool Aggregation Patterns
cards:
- id: a6ad1d95-5bd0-405f-9b22-45d323711787
  type: mcq
  front: ''
  back: COUNT(*) counts all rows, COUNT(column) excludes NULL values in that column.
  choices:
  - key: a
    text: COUNT(*) = 100, COUNT(bonus) = 100
    correct: false
  - key: b
    text: COUNT(*) = 100, COUNT(bonus) = 90
    correct: true
  - key: c
    text: COUNT(*) = 90, COUNT(bonus) = 100
    correct: false
  - key: d
    text: Both return 90 because NULL rows are excluded
    correct: false
- id: e2f75dd3-0e86-4e99-8f9a-4fa914f2dd91
  type: flip
  front: ''
  back: ''
- id: 7e84a86a-5dba-428f-8642-87ce868b3757
  type: flip
  front: ''
  back: ''
- id: 6ea5c8f1-c5eb-429b-879c-76b2954e7183
  type: flip
  front: ''
  back: ''
---

## Intuition

Aggregation functions collapse multiple rows into a single result, computing summary statistics. COUNT tallies occurrences, SUM totals values, AVG computes mean, and MAX/MIN find extremes. These are essential for generating reports and analytics.

## Detail

Common aggregation functions:

```sql
SELECT 
  COUNT(*) as total_rows,              -- counts all rows including NULLs
  COUNT(column) as non_null_count,     -- counts non-NULL values only
  SUM(amount) as total,                -- sums numeric values (NULL-safe)
  AVG(price) as average_price,         -- average (excludes NULLs)
  MAX(salary) as highest_salary,       -- maximum value
  MIN(salary) as lowest_salary,        -- minimum value
  STRING_AGG(name, ', ') as all_names  -- concatenate strings (PostgreSQL)
FROM employees
WHERE department = 'Sales';
```

Null handling is crucial:
- COUNT(*) includes NULLs
- COUNT(column) excludes NULLs
- SUM, AVG, MAX, MIN all ignore NULLs
- If all values are NULL, SUM/AVG/MAX/MIN return NULL (not 0 or 0 rows)

Schema example:
```sql
CREATE TABLE employees (
  name VARCHAR(100),
  salary DECIMAL(10, 2),
  bonus DECIMAL(10, 2)
);
```

## Common gotchas / interview framings
- **Gotcha**: COUNT(*) vs COUNT(column) when NULLs exist
- **Gotcha**: AVG ignores NULLs, which can bias results
- **Gotcha**: Dividing aggregates without GROUP BY doesn't work as expected
- **Interview**: "Why might COUNT(bonus) differ from COUNT(*)?"
- **Interview**: "How would you calculate the average bonus excluding NULL values?"

## See also
- [[count-function]]
- [[sum-function]]
- [[avg-function]]
- [[max-function]]
- [[min-function]]
- [[null-semantics]]
- [[aggregation-pitfalls]]

## Sources
See frontmatter `sources:`.
