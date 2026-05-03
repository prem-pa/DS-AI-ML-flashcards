---
id: 4ee92e15-564d-4386-bff4-4c633bb111d5
title: Data warehouse architecture (star schema, fact/dimension tables)
track: ml-engineer
topic: feature-stores-data-management
difficulty: 3
tags:
- data-warehouse
- schema-design
- analytics
- normalization
- performance
- dimensional-modeling
aliases:
- dimensional-modeling
- data-mart
- snowflake-schema
sources:
- url: https://www.databricks.com/blog/what-feature-store-complete-guide-ml-feature-engineering
  label: What is a Feature Store - Databricks
- url: https://www.featurestore.org/
  label: Feature Store Official Guide
cards:
- id: 54565900-68a4-4d1f-b11d-7e58ae145c80
  type: flip
  front: What is the primary advantage of a star schema for feature engineering?
  back: 'A star schema allows easy aggregation and pivoting: join the fact table to dimensions, then group-by
    or pivot dimensions into features. Example: Group purchases by user & product_category to create features
    like ''electronics_purchase_count''.'
- id: d93632f1-839d-4771-bbe6-b60377b61bf4
  type: mcq
  front: In a star schema, which table contains event-level metrics?
  back: The fact table is central and contains the business metrics (amount, duration) and references
    to dimensions via foreign keys. Each row represents an atomic event (transaction, click, login).
  choices:
  - key: a
    text: Dimension tables
    correct: false
  - key: b
    text: Fact table, containing one row per event with metrics and foreign keys to dimensions
    correct: true
  - key: c
    text: Conformed tables
    correct: false
  - key: d
    text: The time dimension
    correct: false
- id: 4922b3f7-7534-4e2d-bcc7-8593fad7b64d
  type: flip
  front: What is a slowly changing dimension (SCD), and why does it matter for point-in-time joins?
  back: A slowly changing dimension is an attribute (e.g., user's country) that changes over time. SCD
    Type 2 maintains history with effective_date and end_date. This allows point-in-time joins to use
    the correct attribute value as-of the label timestamp.
- id: f49860aa-45a8-4baa-aa07-0cd2fef15ff5
  type: mcq
  front: You are designing a fact table for user transactions. What is the most appropriate granularity?
  back: The finest granularity (per transaction) is most flexible—you can aggregate up to daily, monthly,
    or user-level. Coarser granularities (per-user-per-day) lose detail and prevent features that require
    finer aggregations.
  choices:
  - key: a
    text: One row per user per day
    correct: false
  - key: b
    text: One row per transaction (finest granularity, supports any aggregation)
    correct: true
  - key: c
    text: One row per user per month
    correct: false
  - key: d
    text: One row per product
    correct: false
---

## Intuition
A star schema organizes data into a central fact table (events) surrounded by dimension tables (attributes). Example: fact table = purchases, dimensions = users, products, time. This design optimizes both analytics queries and feature extraction.

## Detail
**Fact Table**:
- One row per event (purchase, click, login)
- Contains foreign keys to dimensions and metrics (amount, duration)
- Typically immutable or append-only
- Example: purchases(purchase_id, user_id, product_id, date_id, amount, quantity)

**Dimension Tables**:
- One row per entity or attribute value
- Describes the "who, what, where, when" of the fact
- Often slowly changing: user demographics, product categories
- Examples:
  - User dimension: user_id, age, country, signup_date
  - Product dimension: product_id, category, price, vendor_id
  - Time dimension: date_id, year, month, day_of_week

**Why Star Schema for ML**:
- **Easy aggregations**: Join fact to dimensions; aggregate metrics by any dimension
- **Feature engineering**: Pivot dimensions into features; aggregate facts into statistics
- **Denormalization sweet spot**: Normalized enough for storage efficiency; denormalized enough for fast queries
- **Slowly changing dimensions (SCD)**: Track dimension changes over time (SCD Type 2 with effective_date, end_date)

**Comparison: Snowflake Schema**:
- Further normalized: dimensions have sub-dimensions
- Saves storage but adds join complexity
- Less common in practice for feature stores

## Common gotchas / interview framings
- **Fact granularity**: Define clearly (per transaction? per item? per day?). Wrong granularity breaks aggregations
- **SCD handling**: If a user's country changes, do you keep the old record (SCD Type 2) or overwrite (Type 1)? This affects point-in-time joins
- **Surrogate keys**: Use auto-increment surrogate key (date_id) instead of natural key (2024-01-01) for performance
- **Conformed dimensions**: Shared dimensions (e.g., user_dim) must be consistent across fact tables
- **NULL handling**: NULLs in fact tables can distort aggregations; clarify semantics
- **Interview question**: "Design a star schema for an e-commerce recommendation system"

## See also
- [[etl-vs-elt]]
- [[incremental-and-full-refresh-logic]]
- [[feature-definitions-and-computation]]
- [[online-vs-offline-feature-stores]]

## Sources
See frontmatter `sources:`.
