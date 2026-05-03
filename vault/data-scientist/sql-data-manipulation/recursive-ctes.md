---
id: ccfa33a0-758a-4ab8-af13-bd5e5f7a41e5
title: Recursive CTEs
track: data-scientist
topic: sql-data-manipulation
difficulty: 5
tags:
- hierarchical-data
- tree-traversal
- graph-algorithms
- recursion
aliases:
- recursive queries
- hierarchy traversal
- org chart queries
- graph traversal
sources:
- url: https://www.postgresql.org/docs/current/queries-with.html
  label: PostgreSQL Recursive CTE Documentation
- url: https://neon.com/postgresql/tutorial/cte
  label: Neon Recursive CTE Patterns
- url: https://www.crunchydata.com/blog/postgres-subquery-powertools-subqueries-ctes-materialized-views-window-functions-and-lateral
  label: Crunchy Data Recursive CTE Deep Dive
cards:
- id: 9cf9ca5e-72dd-447d-aed2-0b99a025ef22
  type: flip
  front: ''
  back: ''
- id: 2a2397f9-9dfc-4468-99de-c30a4e8f397a
  type: mcq
  front: ''
  back: UNION deduplicates between recursive iterations, which can prematurely stop recursion. UNION ALL
    is standard for recursive CTEs.
  choices:
  - key: a
    text: UNION and UNION ALL produce the same results
    correct: false
  - key: b
    text: UNION removes duplicates (slower); UNION ALL keeps duplicates
    correct: true
  - key: c
    text: UNION ALL is invalid in recursive CTEs
    correct: false
  - key: d
    text: UNION allows multiple recursion paths; UNION ALL doesn't
    correct: false
- id: f59bfad1-27e9-42ba-9068-0c7e75158cbd
  type: flip
  front: ''
  back: ''
- id: b7f8e0e8-3467-46ef-afa7-bcdb7d266c09
  type: flip
  front: ''
  back: ''
---

## Intuition

Recursive CTEs traverse hierarchical or graph data (org charts, file systems, bill-of-materials). They work through two parts: an anchor (base case) that starts the recursion, and a recursive member that references the CTE itself. Each iteration adds more rows until no new rows are found.

## Detail

```sql
WITH RECURSIVE org_hierarchy AS (
  -- Anchor: start with CEO (manager_id IS NULL)
  SELECT employee_id, name, manager_id, 1 as level
  FROM employees
  WHERE manager_id IS NULL
  
  UNION ALL
  
  -- Recursive member: find reports of current level
  SELECT e.employee_id, e.name, e.manager_id, oh.level + 1
  FROM employees e
  INNER JOIN org_hierarchy oh ON e.manager_id = oh.employee_id
  WHERE oh.level < 10  -- prevent infinite loops
)
SELECT * FROM org_hierarchy
ORDER BY level, name;
```

Another example (path building):
```sql
WITH RECURSIVE path_finding AS (
  SELECT source, target, 1 as hops, ARRAY[source, target] as path
  FROM edges
  WHERE source = 'A'
  
  UNION ALL
  
  SELECT pf.source, e.target, pf.hops + 1, pf.path || e.target
  FROM path_finding pf
  JOIN edges e ON pf.target = e.source
  WHERE pf.hops < 5 AND NOT e.target = ANY(pf.path)  -- avoid cycles
)
SELECT * FROM path_finding WHERE target = 'Z';
```

Critical safeguards:
- Include a termination condition (depth limit, visited check)
- Use UNION ALL (not UNION) for performance
- Recursion depth can explode without careful limits

## Common gotchas / interview framings
- **Gotcha**: Infinite recursion without termination condition
- **Gotcha**: UNION instead of UNION ALL causes deduplication (slower)
- **Gotcha**: Forgetting to increment depth or check visited nodes
- **Interview**: "Write a query to find all employees under a specific manager"
- **Interview**: "Find the shortest path in a graph" (advanced)

## See also
- [[recursive-cte-basics]]
- [[anchor-member]]
- [[recursive-member]]
- [[base-case]]
- [[tree-structures]]
- [[cycle-detection]]

## Sources
See frontmatter `sources:`.
