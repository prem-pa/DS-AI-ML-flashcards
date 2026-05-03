---
id: a0c054ec-0e14-48c1-b7be-8376bb64738b
title: Index strategies
track: data-scientist
topic: sql-data-manipulation
difficulty: 3
tags:
- performance
- optimization
- indexing
- query-speed
aliases:
- index optimization
- database indexing
- query acceleration
sources:
- url: https://www.postgresql.org/docs/current/sql-createindex.html
  label: PostgreSQL CREATE INDEX Documentation
- url: https://mode.com/sql-tutorial/index-optimization/
  label: Mode Analytics Indexing Guide
- url: https://dataschool.com/sql-optimization/indexes/
  label: DataSchool Index Strategies
cards:
- id: 368e1ad7-7f62-4ca0-bfd1-79b498f57c4b
  type: mcq
  front: ''
  back: Low-cardinality columns don't benefit much from indexing. The optimizer may do a full table scan
    anyway. Index selectivity matters.
  choices:
  - key: a
    text: Yes, always index WHERE columns
    correct: false
  - key: b
    text: No, too low cardinality; full table scan is fine
    correct: true
  - key: c
    text: Yes, but mark it as low-priority
    correct: false
  - key: d
    text: Only if the table is very large (>1M rows)
    correct: false
- id: dbcc164b-8cf7-422c-b28a-3da32a641587
  type: flip
  front: ''
  back: ''
- id: 005a2407-65ae-4671-a6ad-52216d90f088
  type: flip
  front: ''
  back: ''
- id: d4768020-573c-452a-bc59-7d1d93268941
  type: flip
  front: ''
  back: ''
---

## Intuition

Indexes speed up WHERE and JOIN conditions by creating efficient lookup structures, but they slow inserts/updates (must maintain index). Strategic indexing balances read speed with write cost. Single-column indexes help equality/range filters; composite indexes help multiple conditions or cover entire queries.

## Detail

Basic index creation:
```sql
-- Single-column index (common)
CREATE INDEX idx_orders_customer_id ON orders(customer_id);

-- Composite index (orders on multiple columns)
CREATE INDEX idx_orders_customer_date ON orders(customer_id, order_date DESC);

-- Covering index (includes data, avoids table lookup)
CREATE INDEX idx_orders_covering ON orders(customer_id) INCLUDE (amount, status);

-- Partial index (only index subset of rows, saves space)
CREATE INDEX idx_active_orders ON orders(customer_id) WHERE status = 'active';
```

Index selection rules:
1. **WHERE columns**: Index columns in WHERE clause (high priority)
2. **JOIN keys**: Index foreign keys and join columns
3. **ORDER BY**: Index ORDER BY columns if you have them in WHERE too
4. **Selectivity**: Index high-cardinality columns (many distinct values) over low-cardinality
5. **Composite order**: Put equality conditions before range conditions (WHERE id = 5 AND date > '2025-01-01')

When NOT to index:
- Low cardinality (e.g., boolean, status with 3 values): minimal benefit
- Write-heavy tables: index maintenance slows inserts
- Tiny tables: full table scan is fast enough

Index tradeoffs:
```sql
-- Good index selection
CREATE INDEX idx_users_email ON users(email);  -- email is unique lookup key
CREATE INDEX idx_orders_customer_date ON orders(customer_id, order_date);  -- common WHERE pattern

-- Poor index selection
CREATE INDEX idx_users_active ON users(is_active);  -- only 2 values, low selectivity
CREATE INDEX idx_huge_table_col1_col2_col3_col4 ON huge_table(...);  -- too many columns, slow maintenance
```

## Common gotchas / interview framings
- **Gotcha**: Indexing low-cardinality columns (boolean, small enum)
- **Gotcha**: Too many indexes (slows inserts, wastes storage)
- **Gotcha**: Index on computed expressions (not actual columns)
- **Gotcha**: Missing index on foreign keys in JOINs
- **Interview**: "This query is slow. What indexes would help?"
- **Interview**: "Design index strategy for a write-heavy table"

## See also
- [[index-types]]
- [[single-column-index]]
- [[composite-index]]
- [[index-tradeoffs]]
- [[index-maintenance]]
- [[covering-index]]

## Sources
See frontmatter `sources:`.
