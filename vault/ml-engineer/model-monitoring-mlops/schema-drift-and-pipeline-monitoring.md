---
id: 5be0fe44-b607-40ca-8b19-44b6bf686348
title: Schema drift and pipeline monitoring
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 3
tags:
- data-quality
- schema-validation
- pipeline-monitoring
- backwards-compatibility
- data-contracts
- versioning
aliases:
- data schema changes
- field-level monitoring
- upstream dependency tracking
sources:
- url: https://www.peerspot.com/categories/model-monitoring
  label: 'PeerSpot: Best Model Monitoring solutions 2026'
cards:
- id: 22cccc34-a8a2-4c9a-9837-8eb96f36add2
  type: flip
  front: A column 'customer_id' changed from INT to STRING (leading zeros added). Your model uses this
    as a categorical feature. What happens and how do you handle it?
  back: '**What happens**: If your feature preprocessing expects integer, it may fail (type casting error)
    or silently produce wrong values (e.g., 001 != 1 as strings). Model retrains on STRING type; old INT-trained
    weights no longer apply.


    **Handling**:

    1. **Backward-compatibility check**: Is the change additive (old values still valid)? Leading zeros
    = valid change, one-to-one mapping preserved.

    2. **Test impact**: Retrain model on new schema; compare performance. If identical, safe. If degraded,
    STRING representation lost predictive power (e.g., one-hot encoding now treats 001 and 1 as separate
    categories).

    3. **Adapt preprocessing**: (a) Normalize: strip leading zeros back to INT before encoding, or (b)
    Accept new encoding, retrain model.

    4. **Version schema**: Document "v2: customer_id now STRING with leading zeros" so old models don''t
    load on new data.

    5. **Deploy carefully**: Blue-green deploy old + new model; compare predictions on same data; switch
    once validated.'
- id: 1feda1e0-7be2-4d62-81e6-b66247c2fcbf
  type: mcq
  front: A new column 'user_segment' appeared in your feature table. Schema validator reports 'new field
    detected'. Your model currently uses 15 features and does NOT use this new column. Should you update
    your model?
  back: '**Correct: b** Schema change ≠ requires model change. Additive changes (new columns) are safe
    if not used by model. Your model''s input features are fixed; new columns won''t affect it. Action:
    (1) Update schema validator to accept new column. (2) Optionally (low priority): Test if user_segment
    improves AUC; if yes, retrain next cycle. This is planned improvement, not urgent fix.'
  choices:
  - key: a
    text: Yes, always incorporate new features; more data is better
    correct: false
  - key: b
    text: No, if model doesn't use it and current performance is stable, no action needed
    correct: true
  - key: c
    text: Yes, retrain to test if new feature improves AUC
    correct: false
  - key: d
    text: No, new columns always break models
    correct: false
- id: e879e5a9-4689-4a3c-a7fd-a09b36500ad6
  type: flip
  front: Design a data contract between upstream ETL and your model serving team. What schema violations
    should trigger an alert vs a halt?
  back: "**Data contract schema violations**:\n\n**HALT (Immediate, break prediction serving)**:\n- Missing\
    \ required column (feature used by model)\n- Type change that breaks preprocessing (INT → STRING for\
    \ numerical feature)\n- Column marked nullable changed to NOT NULL (or vice versa; data handling breaks)\n\
    - Major cardinality change in categorical (enum > 10x)\n\n**ALERT (Monitor, investigate, but don't\
    \ block)**:\n- New column added (additive)\n- Column marked nullable changed to nullable (more flexible,\
    \ usually safe)\n- Minor cardinality expansion (new enum values < 5x)\n- Column documentation updated\n\
    \n**Config example**:\n```\nschema_contract:\n  required_columns: [customer_id, amount, timestamp]\n\
    \  types: {customer_id: INT, amount: FLOAT, timestamp: TIMESTAMP}\n  max_cardinality: {region: 50,\
    \ product_id: 10000}\n  rules:\n    - if missing(required_columns) -> HALT\n    - if type_change ->\
    \ HALT\n    - if cardinality > max_cardinality * 2 -> ALERT\n    - if new_column -> ALERT\n```\n\n\
    Implementation: Great Expectations or dbt tests run on every ETL load."
- id: 48e60ec3-15ef-4c72-a455-ae8006354be4
  type: flip
  front: 'A database schema evolved: a nested JSON field was flattened into separate columns. Your feature
    extraction code breaks because it expects the nested structure. How do you prevent this in the future?'
  back: "**Prevention strategy**:\n\n1. **Version your schema contract**: Document expected structure\
    \ + versions. Schema v1 = nested JSON, v2 = flattened. \n\n2. **Test backward compatibility**: Before\
    \ upstream deploys v2, test your feature extraction on both v1 and v2 data. Ensure graceful handling.\n\
    \n3. **Use schema registry**: Tools like Kafka schema registry or data contracts (in dbt, Great Expectations)\
    \ enforce version compatibility. Upstream change must be marked \"compatible\" before deployment.\n\
    \n4. **Feature extraction versioning**: Version your feature code. Feature extraction v1 expects nested\
    \ JSON; v2 expects flattened columns. Deploy v2 to serve before upstream deploys v2.\n\n5. **Monitoring**:\
    \ Alert on schema version changes. If upstream deploys v2 but your code still expects v1, immediately\
    \ page team.\n\n6. **Integration testing**: Include upstream schema tests in your CI/CD. Each feature\
    \ extraction code change is tested against current upstream schema AND versioned schemas.\n\n**Best\
    \ practice**: Coordinate with upstream team; use data contracts (mutual agreement on interface) instead\
    \ of reactive monitoring."
---

## Intuition
Upstream data sources change: columns renamed, new fields added, types changed (int → string), nested structures flattened. These schema changes silently break downstream models. Monitor schema: alert when upstream data changes, so you can validate backward-compatibility and adapt your model before predictions fail.

## Detail
Schema contract: document expected columns, types, nullable status. On each ETL run, compare actual schema to contract. Violations: missing column (breaking), new column (additive, usually safe), type change (int → float safe, int → string breaking), cardinality explosion (new enum values, may need model update). Tools: data catalogs (Apache Atlas, Collibra), validation frameworks (Great Expectations with schema checks), Kafka schema registry (for streaming). Action on violation: (1) Additive (new safe column): ignore. (2) Breaking (column removed, type change): alert + halt model serving, investigate. (3) Cardinality change: assess model's handling of unseen values; retrain if needed.

## Common gotchas / interview framings
- Not versioning schemas; hard to track what broke when
- Confusing schema drift (structural change) with data drift (distribution change)
- Over-strict validation; blocking on non-impactful changes (timestamp format)
- Not testing downstream effects; schema change may not break feature extraction but breaks downstream business logic

## See also
- [[missing-value-and-anomaly-monitoring]]
- [[data-freshness]]
- [[feature-distribution-monitoring]]
- [[model-performance-degradation-accuracy-drop-calibration-shift]]

## Sources
See frontmatter `sources:`.
