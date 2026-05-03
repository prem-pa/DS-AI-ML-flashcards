---
id: c0fd164d-f393-49db-b029-f6fb16458b55
title: Data validation and schema checks
track: data-scientist
topic: statistics-in-production
difficulty: 3
tags:
- data-quality
- validation
- schema
- data-pipelines
- upstream-errors
aliases:
- schema validation
- data contracts
- input validation
- data integrity checks
sources:
- url: https://www.thedataletter.com/p/how-netflix-does-data-reliability
  label: How Netflix Does Data Reliability
- url: https://www.twosigma.com/articles/treating-data-as-code-at-two-sigma/
  label: 'Two Sigma: Treating Data as Code'
- url: https://www.sigmacomputing.com/blog/automate-data-validation
  label: Automated Data Validation
- url: https://www.bigeye.com/blog/data-in-practice-anomaly-detection-for-data-quality-at-netflix
  label: 'Bigeye: Data Quality at Netflix'
cards:
- id: 0a0e4169-9012-4928-b4fd-3c24fcd5a7cd
  type: flip
  front: Your data pipeline ingests user events with a 'timestamp' field. One upstream service changes
    format from ISO-8601 to Unix seconds. Validation passes (type is still numeric). How do you prevent
    silent data corruption?
  back: 'Schema-only validation is insufficient. Add a semantic check: timestamp should be recent (e.g.,
    within last 24 hours). Unix seconds since 1970 would be ~1.7e9, placing events in 2024. ISO-8601 parsed
    as seconds would place events in year 1970–1980, failing the recency check. Alternatively, validate
    against a known range (e.g., timestamp > cutoff_date) or add a ''timestamp_format'' field to the schema.'
- id: 034478b4-b43d-45a8-81ac-5bf916f6b713
  type: mcq
  front: A downstream recommender model relies on 'item_embeddings' from a feature store. A bug causes
    null values in 50% of embeddings one morning. Your validation rule was 'embedding is not null.' It
    passed because the overall table is 50% null, within the 'tolerable' 10% null threshold. How do you
    improve this?
  back: Fixed thresholds (a) may be too strict or too loose. Anomaly detection on null rate (b) catches
    sudden increases. Combined, you detect both chronic (always 50% null) and acute (jumped from 5% to
    50%) issues. (d) is also important but doesn't address the null spike.
  choices:
  - key: a
    text: Lower null threshold to 0.1%.
    correct: false
  - key: b
    text: 'Add a ''null rate change'' metric: alert if null rate increases by > 5% from yesterday.'
    correct: false
  - key: c
    text: Use both (a) and (b); detect static violations and sudden changes.
    correct: false
  - key: d
    text: Check embedding values, not nulls; validate that embeddings are normalized.
    correct: false
- id: 6079a865-6897-4727-86e4-c1a615403489
  type: flip
  front: Your model uses a 'user_country' field. Yesterday, cardinality was ~200 countries. Today, you
    see 50,000 unique values. Your validation rule 'cardinality < 100,000' passes. Is this a problem,
    and how do you detect it?
  back: 'Yes, this is suspicious. 50,000 countries is impossible; likely a data quality bug (join error,
    encoding corruption, leaked internal IDs). Add a validation rule: ''cardinality should match historical
    values ± 10%.'' Flag if cardinality >> expected range. Alternatively, use a probabilistic cardinality
    check (HyperLogLog) or compare recent distinct values to a 30-day rolling median.'
- id: 38676558-ea39-4060-81db-80bf8d2094dc
  type: mcq
  front: You validate a user-events table and find null values in the 'session_id' field. Your rule says
    nulls are acceptable (platform allows anonymous events). But for your model, session_id is required.
    Where should this validation live?
  back: Producer-level (a) catches errors early. Model-level (c) handles legitimate nulls (other consumers
    may accept them). Warehouse-level (b) is too rigid if different consumers have different tolerances.
    Both layers protect against different failure modes.
  choices:
  - key: a
    text: At the data producer (source system)—reject null session_ids before ingestion.
    correct: false
  - key: b
    text: At the data warehouse schema level—enforce NOT NULL constraint.
    correct: false
  - key: c
    text: In your model's data loader—skip rows with null session_id before training.
    correct: false
  - key: d
    text: At the producer (a) and model level (c); separate concerns.
    correct: false
---

## Intuition

Data validation catches upstream errors before they propagate through ML pipelines. A misconfigured data producer (wrong timestamp format, missing column) could corrupt downstream models. Validation at the source (catching errors early) is cheaper than debugging at the sink (finding bad predictions).

## Detail

**Schema Validation**: Check that incoming data matches expected structure—column names, types, required fields. Use JSON Schema, Avro, Protobuf, or SQL constraints. Catch type errors (string in a numeric field) and structural changes (missing column).

**Statistical Validation**: Check that values fall within expected ranges—age ∈ [18, 120], price ∈ [0.01, 10,000]. Detect errors like negative counts, future timestamps, or outliers indicating data corruption.

**Cardinality Checks**: Categorical fields should have a bounded set of values. If a user_id field suddenly has 10M unique values (instead of typical 100K), that's an error—likely a join bug or data duplication.

**Freshness Checks**: Data should arrive on schedule. If yesterday's batch didn't arrive by 6 AM, alert—the ETL pipeline may have failed.

**Referential Integrity**: Foreign keys should exist. If an order references a missing user_id, that's a join error, not valid data.

**Anomaly Detection on Metadata**: Count of nulls, distinct values, min/max should be stable. A sudden spike in nulls indicates upstream failure.

**Sample-based Validation**: For large tables, sample rows and validate; trade off completeness for speed.

## Common gotchas / interview framings

- **Validation paralysis**: Over-strict validation rejects edge cases (legitimate outliers). Set thresholds based on business context, not just statistical extremes.
- **Downstream propagation**: If validation fails, what happens to downstream consumers? Queue messages? Skip batch? Wait for manual intervention? Define SLOs for validation failures.
- **Silent failures**: Validation passes but data is subtly wrong (e.g., timezone inconsistency, encoding mismatch). Add semantic validation, not just schema checks.
- **Retroactive detection**: Validation often happens post-ingestion. Prefer pre-ingestion (producers validate before sending) to reduce latency.

## See also
- [[missing-data-and-imputation]]
- [[outlier-detection]]
- [[data-lineage-and-versioning]]
- [[guardrail-metrics-and-alert-thresholds]]

## Sources
See frontmatter `sources:`.
