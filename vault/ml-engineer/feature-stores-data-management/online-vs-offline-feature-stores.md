---
id: 22708b06-4e80-4359-828e-11591bbab8db
title: Online vs offline feature stores
track: ml-engineer
topic: feature-stores-data-management
difficulty: 5
tags:
- feature-store-architecture
- online-serving
- batch-processing
- latency
- cost
- training-serving-skew
aliases:
- dual-store-architecture
- real-time-vs-batch-features
- low-latency-serving
sources:
- url: https://system-design.space/en/chapter/feature-store-model-serving/
  label: System Design Space - Feature Store & Model Serving
- url: https://uplatz.com/blog/a-comparative-analysis-of-modern-feature-stores-feast-vs-tecton-vs-hopsworks/
  label: Comparative Analysis of Modern Feature Stores
cards:
- id: 235d5988-2b4d-4006-9fe2-78a9a0292ffd
  type: flip
  front: What is the primary purpose of an offline feature store in ML pipelines?
  back: The offline store stores historical feature values with timestamps, enabling training datasets
    to be created by joining labels with feature snapshots as of the label date. This supports point-in-time
    correctness and prevents data leakage.
- id: 6ad9146f-6387-4185-b209-0c2f0e3116ed
  type: mcq
  front: Why would you use an online feature store instead of an offline store for real-time predictions?
  back: Online stores use low-latency datastores (Redis, DynamoDB) optimized for single-key lookups in
    <10ms. Offline stores (data warehouses) are optimized for batch queries and historical data, not real-time
    serving.
  choices:
  - key: a
    text: Online stores are cheaper per operation
    correct: false
  - key: b
    text: Online stores provide sub-millisecond latency with key-value lookups, vs 100ms+ for warehouse
      queries
    correct: true
  - key: c
    text: Online stores are more accurate
    correct: false
  - key: d
    text: Online stores eliminate the need for training data
    correct: false
- id: 721e95ba-dd45-4f4f-a21f-0cf7d3ae4ab4
  type: flip
  front: What causes training-serving skew in an online/offline architecture?
  back: 'If the online store is not kept in sync with the offline store, features may have different values
    during training vs serving. Example: User purchase count is 10 in offline (batch), but 12 in online
    (real-time updates), causing the model to see inconsistent feature distributions.'
- id: d3d66d44-6415-454b-b54d-06f4db36903f
  type: mcq
  front: Which consistency guarantee is most practical for online stores in production?
  back: Strong consistency at sub-millisecond latency is expensive. In practice, you accept eventual consistency
    and define a staleness SLO (e.g., features must be < 5 minutes old) that you monitor and alert on.
  choices:
  - key: a
    text: Strong consistency—all reads see the latest write
    correct: false
  - key: b
    text: Eventual consistency with low staleness (seconds to minutes), managed by monitoring SLOs
    correct: true
  - key: c
    text: No consistency guarantees
    correct: false
  - key: d
    text: Causal consistency
    correct: false
---

## Intuition
Online stores serve features to models in <10ms (sub-millisecond for some systems); offline stores are data warehouses/lakes optimized for batch access during training and analytics. A feature store typically manages both, syncing features between them.

## Detail
**Offline Store (Training & Analytics)**:
- Stores historical feature values with timestamps
- Optimized for bulk reads, point-in-time joins, and complex aggregations
- Examples: Snowflake, BigQuery, Spark Delta Lake, Parquet files
- Latency: 100ms to seconds (acceptable for batch)
- Cost: Lower per-row, bulk access is efficient

**Online Store (Real-time Serving)**:
- Low-latency key-value store (Redis, DynamoDB, Cassandra, Aerospike)
- Stores only current/recent feature values
- Lookup by entity key (user_id, item_id) in <10ms
- Cost: Higher per-operation, but constrained volume

**Synchronization Challenge**:
- Offline features computed in batch; online features updated via streaming pipelines
- Keeping both in sync without data leakage is non-trivial
- Example: User purchase count—if batch job is 1 hour behind, online count may differ

## Common gotchas / interview framings
- **Training-serving skew**: If offline and online have different values for the same feature, model performance drops
- **Staleness**: Online features lag behind the latest data; must decide acceptable lag
- **Cost-latency trade-off**: Real-time online updates are expensive; batch updates are cheap but add latency
- **Data duplication**: Storing features in two places increases storage and sync complexity
- **Consistency model**: Eventual consistency vs strong consistency; what guarantees does your feature store provide?

## See also
- [[feature-definitions-and-computation]]
- [[feature-freshness-and-staleness-slos]]
- [[point-in-time-correctness-and-data-leakage-prevention]]
- [[data-warehouse-architecture-star-schema-factdimension-tables]]

## Sources
See frontmatter `sources:`.
