---
id: f2792562-6119-414c-8f53-7c0bd4fcec45
title: Incremental and full refresh logic
track: ml-engineer
topic: feature-stores-data-management
difficulty: 3
tags:
- incremental-updates
- batch-processing
- data-refresh
- scheduling
- cost-optimization
- idempotency
aliases:
- batch-refresh
- delta-updates
- snapshot-management
sources:
- url: https://docs.databricks.com/aws/en/machine-learning/feature-store/time-series
  label: Point-in-Time Feature Joins - Databricks
- url: https://www.gocodeo.com/post/top-5-feature-stores-in-2025-tecton-feast-and-beyond
  label: Feature Store Patterns 2025
cards:
- id: 1e0898ec-be8d-4fb7-b1e3-bc4fe51fb868
  type: flip
  front: What is the main trade-off between full refresh and incremental refresh?
  back: Full refresh is simple and idempotent but expensive (recomputes all features every time). Incremental
    is cheap and fast but complex (must track state, handle late-arriving data, and avoid double-counting).
- id: 9bef7b1f-6153-4413-ba46-4ad5ae491143
  type: mcq
  front: What is a watermark in incremental feature refresh?
  back: 'A watermark is a timestamp marker that tracks the boundary of processed data. Example: ''last_watermark
    = 2024-01-10 10:00 AM'' means all data up to that time has been processed. New data arriving after
    the watermark triggers an incremental job.'
  choices:
  - key: a
    text: A visual marker in dashboards
    correct: false
  - key: b
    text: The latest timestamp of source data that has been processed; new data beyond the watermark triggers
      recomputation
    correct: true
  - key: c
    text: A threshold for feature values
    correct: false
  - key: d
    text: A schema version number
    correct: false
- id: 2cfe7329-e201-4489-8453-bcf8d729581e
  type: flip
  front: Why is idempotency important in incremental refresh?
  back: 'If a job fails and reruns, it must produce the same result without double-counting. Example:
    If a purchase event is processed twice, the purchase_count must still be correct. Without idempotency,
    rerunning a failed job corrupts features.'
- id: c4e102f1-97ea-4816-8df5-44bd75a1dbab
  type: mcq
  front: How would you handle late-arriving data (data with timestamp before watermark but arriving after)
    in incremental refresh?
  back: If late-arriving data is important for correctness (especially for training), shift the watermark
    back and reprocess entities affected by the late data. For real-time serving, you may accept the trade-off
    of missing late data to maintain SLO. Document your policy clearly.
  choices:
  - key: a
    text: Always ignore it
    correct: false
  - key: b
    text: Always include it by shifting watermark back and reprocessing affected entities
    correct: true
  - key: c
    text: Include it without reprocessing (acceptable for some use cases like analytics)
    correct: false
  - key: d
    text: Store it separately and never merge
    correct: false
---

## Intuition
Full refresh recomputes all features from scratch; incremental refresh only updates changed data. Full is simple, expensive, and slow. Incremental is efficient but complex—you must track what changed and handle late-arriving data correctly.

## Detail
**Full Refresh**:
- Recompute all features from raw source data
- Example: `SELECT user_id, COUNT(*) as purchase_count FROM purchases GROUP BY user_id`
- Pros: Simple, no state management, idempotent (same result every time)
- Cons: Expensive (scan all data), slow (takes hours for large datasets), doesn't support real-time
- When: Use for small datasets, daily batch, or when you suspect data corruption

**Incremental Refresh**:
- Compute only features for entities where source data changed
- Track with watermarks: last_processed_timestamp (e.g., "last processed data up to 2024-01-10 10:00 AM")
- Example: `SELECT user_id, COUNT(*) as purchase_count FROM purchases WHERE event_timestamp > last_watermark GROUP BY user_id`
- Pros: Fast, cheap, supports near-real-time updates
- Cons: Complex state management, risks of missing data or double-counting

**State Management in Incremental**:
- **Watermark**: Track the latest timestamp of processed data
- **Idempotency**: Rerunning a computation must not double-count
  - Use deduplication (DISTINCT on source key)
  - Or re-aggregate from scratch for that entity (defeats incremental efficiency)
- **Late-arriving data**: Data with timestamp before watermark but arriving after
  - Option 1: Include it (reprocess affected entities, shift watermark back)
  - Option 2: Exclude it (miss data, but maintain schedule)
  - Option 3: SCD Type 2 (keep both versions)

**Scheduling**:
- Full refresh: daily or weekly (low frequency)
- Incremental: hourly, every 30 minutes, or streaming
- Hybrid: daily full refresh + hourly incremental (catches errors, stays fresh)

## Common gotchas / interview framings
- **Watermark backfill**: When you change a feature definition, must you recompute all history? (Usually yes for training data)
- **Concurrent updates**: If two incremental jobs run simultaneously, they may overwrite each other
- **Storage overhead**: Incremental requires storing old values to detect changes; full refresh doesn't
- **Feature dependency**: If Feature B depends on Feature A (incremental), and A is delayed, B doesn't see the update
- **Interview question**: "Design an incremental feature refresh for 1 billion users with 100 features. How do you handle late-arriving data?"

## See also
- [[etl-vs-elt]]
- [[data-warehouse-architecture-star-schema-factdimension-tables]]
- [[feature-definitions-and-computation]]
- [[feature-freshness-and-staleness-slos]]

## Sources
See frontmatter `sources:`.
