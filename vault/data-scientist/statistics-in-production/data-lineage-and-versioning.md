---
id: 68079e39-ac29-4069-ada3-f2f39947591b
title: Data lineage and versioning
track: data-scientist
topic: statistics-in-production
difficulty: 3
tags:
- data-lineage
- versioning
- reproducibility
- data-provenance
- metadata
aliases:
- data provenance
- pipeline tracking
- feature versioning
- reproducibility
sources:
- url: https://www.twosigma.com/articles/treating-data-as-code-at-two-sigma/
  label: 'Two Sigma: Treating Data as Code'
- url: https://www.thedataletter.com/p/how-netflix-does-data-reliability
  label: How Netflix Does Data Reliability
- url: https://www.bigeye.com/blog/data-in-practice-anomaly-detection-for-data-quality-at-netflix
  label: 'Bigeye: Data Quality and Lineage'
- url: https://www.datacouncil.ai/talks/anomaly-detection-for-data-quality-and-metric-shifts-at-netflix
  label: 'Data Council: Data Reliability at Scale'
cards:
- id: a6ceaa44-96b2-4d20-8291-4100c0453d84
  type: flip
  front: Your model was trained on features from a source table that was updated yesterday (rows deduplicated).
    The model's accuracy dropped 5%. How do you diagnose if the source change caused it?
  back: 'Without data lineage, you''re blind. With lineage: (1) Identify which version of the source table
    was used to train (lineage metadata). (2) Check if that version is still available (snapshot or recomputable).
    (3) Retrain on the old version; if accuracy recovers, the source change caused the issue. (4) Investigate
    the deduplication logic—did it remove legitimate duplicates or collapse important user events? Use
    tools like dbt or Airflow to track lineage automatically.'
- id: 2a665d0d-ff52-47cb-ba8d-06dd66e5d751
  type: mcq
  front: You discover a bug in your feature engineering code (e.g., wrong aggregation window). The bug
    was deployed 2 months ago. Which versioning approach enables the fastest rollback?
  back: Snapshot versioning enables instant rollback—switch model to the old feature snapshot. Deterministic
    versioning (b) requires recomputation (time-consuming). Hybrid (c) works if snapshot is available;
    if not, falls back to recomputation. Manual (d) is unreliable at scale.
  choices:
  - key: a
    text: 'Snapshot versioning: revert to the last-known-good snapshot from 2 months ago.'
    correct: false
  - key: b
    text: 'Deterministic versioning: fix the code, recompute all historical features.'
    correct: false
  - key: c
    text: 'Hybrid: store recent snapshots (last month); recompute older features if needed.'
    correct: false
  - key: d
    text: 'Manual versioning: ask the team for the old dataset.'
    correct: false
- id: e1ffc4e2-1bd6-48c8-b4cf-7aef0c9ceb0c
  type: flip
  front: Your model's performance varies depending on the time-of-day when features are computed (morning
    features → 92% accuracy, evening features → 88%). Should lineage track time-of-computation?
  back: 'Yes, absolutely. This indicates time-dependent data quality or feature shifts (e.g., features
    computed from streaming data differ in latency/freshness depending on time-of-day). Lineage should
    include: (1) timestamp features were materialized, (2) data cutoff times for aggregations, (3) any
    time-dependent sources. This helps identify whether the issue is data freshness (evening features
    use stale data) or data quality (evening ETL produces different results).'
- id: 3e2c956a-23c5-41e5-8e3c-b1120db79d48
  type: mcq
  front: You're implementing feature versioning for 10,000 features across 50 models. Snapshot versioning
    requires storing full feature datasets (massive storage). How do you balance versioning and storage?
  back: Pure snapshots (a) is expensive. Pure recomputation (b) is slow at scale. Hybrid (c) prioritizes
    storage for critical paths (high-revenue models), recomputes others, and maintains full lineage for
    auditing. (d) is insufficient if data sources evolve.
  choices:
  - key: a
    text: Snapshot everything; storage is cheap.
    correct: false
  - key: b
    text: Track lineage of all features and code; recompute features from source code on-demand.
    correct: false
  - key: c
    text: Snapshot only features used by critical models; recompute others; track lineage everywhere.
    correct: false
  - key: d
    text: No snapshots; rely on git history of code.
    correct: false
---

## Intuition

Data lineage tracks how data flows through ETL pipelines—which source tables, transformations, and features feed into your model. Versioning enables reproducibility: "which training data produced this model?" and "did results change after we updated the feature definition?" Without lineage, debugging production issues is blind; without versioning, you can't reproduce or rollback.

## Detail

**Data Lineage**: A DAG (directed acyclic graph) showing:
- **Source**: Raw data tables (logs, databases, third-party APIs).
- **Transformations**: ETL/feature engineering jobs (SQL, Spark, Python).
- **Derived Tables**: Intermediate and final feature sets.
- **Consumers**: Models, dashboards, reports.

Useful for: impact analysis (if source schema changes, which models break?), debugging (trace bad predictions back to bad features), and compliance (audit trail).

**Versioning Strategies**:

**Snapshot Versioning**: Periodically freeze feature datasets with a version ID (v1, v2). Models point to specific versions. Enables rollback but requires storage.

**Deterministic Materialization**: Features are deterministic functions of raw data + parameters. Version parameters (feature SQL, thresholds), not data. Requires recomputation.

**Hybrid**: Store recent snapshots; older versions are recomputed. Trade off storage vs. computation.

**Metadata Tracking**:
- **Data Profile**: Row count, column types, null rates, min/max, cardinality at each version.
- **Transformation Logs**: Which code version, parameters, and execution time for each job.
- **Model Lineage**: Which training data (table versions, date range, features) trained which model version.

**Reproducibility**:
- Pin dependencies (Python version, library versions).
- Record random seeds for train/test splits.
- Store feature computation code and parameters.
- Commit all to version control.

## Common gotchas / interview framings

- **Silent feature changes**: Feature definition evolves (e.g., zip code bucketing changes from 5-digit to 3-digit), but versioning isn't updated. Model behavior shifts mysteriously. Use schema versioning with validation.
- **Implicit dependencies**: Feature A depends on Feature B, but not documented. When Feature B breaks, Feature A silently degrades. Use explicit lineage tooling (Great Expectations, dbt, Airflow).
- **Reproducibility gaps**: Training used local Python scripts and manual data exports; lineage is lost. Require all training to flow through versioned pipelines (no manual steps).
- **Scalability of lineage tracking**: Tracking millions of micro-features across thousands of jobs is expensive. Focus lineage on critical paths (features used in production models).

## See also
- [[data-validation-and-schema-checks]]
- [[missing-data-and-imputation]]
- [[outlier-detection]]

## Sources
See frontmatter `sources:`.
