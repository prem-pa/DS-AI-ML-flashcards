---
id: 408367ba-7673-4a1c-aa55-747b14da557f
title: Feature definitions and computation
track: ml-engineer
topic: feature-stores-data-management
difficulty: 5
tags:
- feature-engineering
- feature-store
- ml-infrastructure
- reproducibility
- reusability
- centralization
aliases:
- feature-library
- feature-registry
- centralized-feature-logic
sources:
- url: https://madewithml.com/courses/mlops/feature-store/
  label: Made With ML - Feature Store
- url: https://www.featurestore.org/
  label: Feature Store Official Documentation
cards:
- id: 3a6d7cfe-87b5-4876-912d-6d421fe3754c
  type: flip
  front: What is the core problem that feature definition centralization solves?
  back: It eliminates duplicate feature logic across batch training, streaming serving, and experimentation
    code. A single definition is computed once and reused, ensuring consistency and reducing maintenance
    burden.
- id: a9ba8fe2-df2e-41f2-92cd-d0932c074ecd
  type: mcq
  front: In a feature store, what does feature definition include?
  back: Feature definitions are specifications of how to compute features, including the transformation
    logic, input data sources, whether computation is batch or streaming, output schema/types, and upstream
    dependencies.
  choices:
  - key: a
    text: Only SQL queries for data extraction
    correct: false
  - key: b
    text: Transformation logic, input sources, computation mode, schema, and lineage
    correct: true
  - key: c
    text: Only the model training code
    correct: false
  - key: d
    text: Feature values stored in cache
    correct: false
- id: f51b0051-ba38-4888-9c29-c4dcd62893e2
  type: flip
  front: Why is feature lineage important in a feature store?
  back: Feature lineage tracks which raw data sources and upstream features feed into each feature. This
    enables impact analysis (if a source changes, which features break), debugging data quality issues,
    and understanding feature dependencies for orchestration.
- id: 09f270c6-5372-4f43-ac33-541156bda183
  type: mcq
  front: What is a key challenge when features depend on other features?
  back: When Feature B depends on Feature A, the feature store must compute Feature A first, then Feature
    B. If Feature A is stale or delayed, Feature B becomes stale. This creates orchestration dependencies
    that must be managed explicitly.
  choices:
  - key: a
    text: It makes features impossible to cache
    correct: false
  - key: b
    text: Dependency ordering and potential cascading staleness if upstream features are delayed
    correct: true
  - key: c
    text: It requires duplicating feature definitions
    correct: false
  - key: d
    text: It eliminates the need for a feature registry
    correct: false
---

## Intuition
Feature definitions are the source of truth for how to compute features. Instead of writing feature logic in notebooks, batch jobs, and serving code separately, a feature store maintains a single definition that can be reused everywhere.

## Detail
A feature definition specifies:
- **Transformation logic**: SQL, Python, or framework-specific transformations
- **Input sources**: Raw tables, other features, APIs
- **Computation mode**: Batch (offline), streaming (online), or both
- **Schema and types**: What columns/values the feature produces
- **Lineage**: Dependencies on upstream features or data sources

Examples:
- User lifetime value (LTV): aggregation over historical transactions with rolling windows
- Recent click rate: count clicks in last 7 days / 7 days, computed per user
- Device type embedding: categorical feature mapped to learned vector

Feature stores like Feast, Tecton, and Hopsworks provide frameworks to define these once and register them in a central registry, eliminating duplication.

## Common gotchas / interview framings
- **Compute once, use everywhere**: The core promise—same feature logic for training and serving
- **Latency vs accuracy trade-off**: Simple features are fast; complex aggregations may require caching
- **Dependency management**: Features that depend on other features create lineage that must be tracked
- **State management**: Stateful features (like running counters) require careful synchronization
- **Versioning**: Feature definitions evolve; need backward compatibility for old training datasets

## See also
- [[online-vs-offline-feature-stores]]
- [[feature-freshness-and-staleness-slos]]
- [[point-in-time-correctness-and-data-leakage-prevention]]
- [[data-warehouse-architecture-star-schema-factdimension-tables]]
- [[etl-vs-elt]]

## Sources
See frontmatter `sources:`.
