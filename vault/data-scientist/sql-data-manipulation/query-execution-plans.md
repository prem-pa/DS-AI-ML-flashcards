---
id: fbebe33b-825f-4d45-8966-9c28cf8e9022
title: Query execution plans
track: data-scientist
topic: sql-data-manipulation
difficulty: 5
tags:
- optimization
- execution
- performance-analysis
- bottleneck-identification
aliases:
- EXPLAIN PLAN
- query analysis
- performance debugging
- execution strategy
sources:
- url: https://www.postgresql.org/docs/current/sql-explain.html
  label: PostgreSQL EXPLAIN Documentation
- url: https://mode.com/sql-tutorial/query-performance/
  label: Mode Analytics Query Performance
- url: https://www.crunchydata.com/developers/playground
  label: Crunchy Data SQL Playground
cards:
- id: c5700e01-d362-4315-8c02-e70171cbd19a
  type: flip
  front: ''
  back: ''
- id: 869dfdac-e952-45b2-8e59-e498e2d7e92f
  type: mcq
  front: ''
  back: Huge gap between estimated and actual rows means optimizer stats are wrong. Run ANALYZE table
    to refresh statistics.
  choices:
  - key: a
    text: The query is working correctly; estimates match reality
    correct: false
  - key: b
    text: The optimizer overestimated; statistics may be stale or incorrect
    correct: true
  - key: c
    text: The index is broken
    correct: false
  - key: d
    text: The WHERE filter is too permissive
    correct: false
- id: 05c5399c-04f4-41cd-83b2-038b04da1e3e
  type: flip
  front: ''
  back: ''
- id: 184f6f0f-2be9-4035-b43d-538f622aac27
  type: flip
  front: ''
  back: ''
---

## Intuition

Query execution plans show HOW the database executes a query: which indexes it uses, how it joins tables, what order it processes data. Understanding plans helps identify bottlenecks (slow scans, expensive joins) and optimize queries. EXPLAIN shows estimated costs; EXPLAIN ANALYZE shows actual execution.

## Detail

```sql
-- Show execution plan (estimated)
EXPLAIN SELECT * FROM orders WHERE customer_id = 5;

-- Show actual execution stats
EXPLAIN ANALYZE SELECT * FROM orders WHERE customer_id = 5;

-- Full plan output example:
-- Seq Scan on orders (cost=0.00..35.50 rows=10 width=60)
--   Filter: (customer_id = 5)

-- With Index
-- Index Scan using idx_orders_customer_id on orders (cost=0.29..12.00 rows=10 width=60)
--   Index Cond: (customer_id = 5)
```

Key plan terms:
- **Seq Scan**: Full table scan (slow for large tables)
- **Index Scan**: Using an index (faster)
- **Nested Loop**: Join algorithm (good for small result sets)
- **Hash Join**: Join algorithm (good for large result sets)
- **Sort**: Sorting data (expensive)
- **Cost**: Estimated I/O and CPU units (lower is better)
- **Rows**: Estimated output rows
- **Actual Rows**: Real rows returned (EXPLAIN ANALYZE only)

Common plan patterns:
```sql
-- Expensive join: 1M * 500K rows = huge temporary hash table
EXPLAIN ANALYZE
SELECT * FROM large_table a
JOIN large_table b ON a.id = b.id
WHERE a.status = 'active';  -- Apply filter FIRST (in CTE or subquery)

-- Better: filter first, then join
WITH filtered_a AS (
  SELECT * FROM large_table WHERE status = 'active'
)
SELECT * FROM filtered_a a
JOIN large_table b ON a.id = b.id;
```

Optimization signals:
- **Seq Scan on large table**: Add index on WHERE column
- **Nested Loop with large inner table**: Use Hash Join (rewrite or add indexes)
- **Sort step**: Check if ORDER BY is necessary; add index on ORDER BY column
- **Rows >> Actual Rows**: Optimizer is overestimating; stats may be stale (ANALYZE table)

## Common gotchas / interview framings
- **Gotcha**: Plan cost is estimated; EXPLAIN ANALYZE gives truth
- **Gotcha**: Indexes don't auto-update stats; run ANALYZE to refresh
- **Gotcha**: Nested loop join on large tables is very slow
- **Interview**: "This query is slow. Walk me through the execution plan."
- **Interview**: "How would you optimize this plan?" (with plan shown)

## See also
- [[explain-command]]
- [[execution-nodes]]
- [[sequential-scan]]
- [[index-scan]]
- [[nested-loop-join]]
- [[hash-join]]
- [[cost-estimation]]

## Sources
See frontmatter `sources:`.
