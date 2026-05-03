---
id: 31af8c82-2d05-409e-8cd3-b65388e149d6
title: ETL vs ELT
track: ml-engineer
topic: feature-stores-data-management
difficulty: 3
tags:
- data-pipeline
- etl
- elt
- data-warehouse
- transformation
- scalability
aliases:
- extract-load-transform
- extract-transform-load
sources:
- url: https://www.databricks.com/blog/what-feature-store-complete-guide-ml-feature-engineering
  label: Feature Store Guide - Databricks
- url: https://www.featurestore.org/
  label: Feature Store Org - ETL Patterns
cards:
- id: 22423d51-109d-4ccc-bcf4-827608fb0033
  type: flip
  front: What is the key difference in where transformation happens between ETL and ELT?
  back: 'ETL: Transform before loading (external system → warehouse). ELT: Load raw data first, transform
    inside the warehouse (warehouse is the transformation engine).'
- id: ac579e01-ea99-4a88-8698-41eab3771efd
  type: mcq
  front: Why is ELT often preferred in modern cloud data warehouses for feature stores?
  back: In ELT, the data warehouse (BigQuery, Snowflake, Databricks) does the heavy lifting. You load
    raw data cheaply, then write SQL transformations. This is simpler (no external cluster), versioned
    (SQL in repo), and scales with warehouse capacity.
  choices:
  - key: a
    text: It requires fewer cloud resources
    correct: false
  - key: b
    text: Warehouse compute is cheap, simpler architecture, and transformation logic is SQL (easier to
      version/test)
    correct: true
  - key: c
    text: ELT guarantees data consistency
    correct: false
  - key: d
    text: ELT is faster than ETL
    correct: false
- id: b8ce0d62-4122-4e60-80f8-5af51bd1026d
  type: flip
  front: What is a disadvantage of ELT compared to ETL?
  back: ELT stores raw data in the warehouse, which increases storage costs. However, cloud storage is
    cheap, so this trade-off is often worth it. The benefit is simpler architecture and easier recovery
    from transformation errors.
- id: 8184d606-1bfc-4fde-8b77-1413db250ba3
  type: mcq
  front: In an ELT pipeline for feature engineering, what role does the data warehouse play?
  back: In ELT, the warehouse is not passive. Feature definitions become SQL/transformations executed
    by the warehouse query engine. The warehouse transforms raw data into features, which are then read
    for training or synced to online stores.
  choices:
  - key: a
    text: Passive storage only
    correct: false
  - key: b
    text: Extraction point
    correct: false
  - key: c
    text: Active transformation engine—running SQL queries to compute features from raw data
    correct: true
  - key: d
    text: Load verification
    correct: false
---

## Intuition
ETL (Extract, Transform, Load) processes data before loading into the warehouse; ELT (Extract, Load, Transform) loads raw data first, then transforms it inside the warehouse. ELT is more modern and scalable with cloud data warehouses.

## Detail
**ETL (Traditional)**:
1. Extract raw data from sources (databases, APIs, logs)
2. Transform in an external system (Spark cluster, Python scripts)
3. Load cleaned/aggregated data into data warehouse

Advantages:
- Compute can be optimized for the transformation logic
- Smaller data loaded into warehouse (less storage cost)
- Can use specialized tools (Spark, DBT if on transformation side)

Disadvantages:
- Infrastructure overhead: need a separate cluster for transformation
- Data duplication: raw data extracted, stored locally, loaded to warehouse
- Harder to recover from transformation errors (rerun entire pipeline)

**ELT (Modern)**:
1. Extract raw data from sources
2. Load into data warehouse (as-is, often partitioned Parquet/Delta)
3. Transform using warehouse-native SQL (Snowflake, BigQuery, Databricks)

Advantages:
- Warehouse compute is cheaper per-operation (especially cloud)
- Simpler architecture: fewer moving pieces
- Data stays in warehouse; easier recovery and incremental updates
- Pushdown: transform logic becomes SQL, optimized by warehouse query engine

Disadvantages:
- Warehouse storage costs if you keep raw data (often mitigated by cheap cloud storage)
- Transformation logic in SQL can be harder to version/test vs. code

**Feature Store Implication**:
Modern feature stores (Feast, Tecton, Hopsworks) assume ELT: raw data in lake/warehouse, feature definitions as SQL/Python transformations applied at feature store level. Feature computation is the "T" in ELT.

## Common gotchas / interview framings
- **ELT doesn't mean no transformation before warehouse**: You still clean/parse during extract
- **Cost trade-off**: ELT cheaper for storage (cloud) but may cost more on compute if transformation is expensive
- **Data lineage**: ELT makes lineage easier (all in one place) vs. ETL (scattered across extract, transform, load)
- **Testing**: ELT transformations (SQL) are easier to unit test; ETL code (Spark, Python) requires more infrastructure
- **Interview angle**: "Design an ELT pipeline for real-time features" vs "Why would you choose ETL over ELT?"

## See also
- [[data-warehouse-architecture-star-schema-factdimension-tables]]
- [[incremental-and-full-refresh-logic]]
- [[feature-definitions-and-computation]]

## Sources
See frontmatter `sources:`.
