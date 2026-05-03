---
id: acb2accd-7262-4027-b659-4de7bceacfce
title: Missing value and anomaly monitoring
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 3
tags:
- data-quality
- anomaly-detection
- missing-data
- outlier-detection
- validation
- pipeline-monitoring
aliases:
- null rate monitoring
- outlier detection
- data quality metrics
sources:
- url: https://www.peerspot.com/categories/model-monitoring
  label: 'PeerSpot: Best Model Monitoring solutions 2026'
cards:
- id: 0e6f7d44-8cf6-4558-9b2d-5c502c5ca493
  type: flip
  front: Null rate on a critical feature jumps from 0.2% → 8% in 2 hours. Your SLA says feature must be
    present for prediction. What is your response?
  back: '**Immediate escalation**: (1) **ALERT**: Page on-call data engineer. 8% nulls likely signal upstream
    pipeline failure (database query broke, ETL timeout, schema change). (2) **Failover**: If SLA allows,
    serve stale cached predictions (last 1 hour) while investigating. (3) **Investigation**: Check data
    source (table exists?), pipeline logs (errors?), schema (column renamed?). (4) **Mitigation**: (a)
    Impute nulls with recent median/mode (temporary), (b) Use model that handles nulls (tree-based), (c)
    Predict with subset of features. (5) **RCA**: Update data quality checks upstream to catch this earlier.'
- id: 24119ce4-d35f-417b-999f-bbc499501f75
  type: mcq
  front: You detect 15 anomalies in a batch of 10,000 transactions (0.15% anomaly rate). Z-score > 3σ
    on multiple features. Should you quarantine these or investigate further?
  back: '**Correct: b** Z-score > 3σ naturally occurs ~0.3% in normal data—0.15% is actually low. But
    outliers can be real (high-value legitimate transactions) or corrupted (malformed data). Use domain
    knowledge: (1) Do outlier values make business sense? (Fraud detection: yes, high amount = legit fraud.
    Age=999 = data bug). (2) Are outliers multivariate (extreme in multiple features simultaneously) or
    univariate (extreme in one feature only)? Multivariate = likely corrupt. (3) Check if outlier features
    are used by model; if not, safe to ignore.'
  choices:
  - key: a
    text: Quarantine immediately; 0.15% is abnormal
    correct: false
  - key: b
    text: Investigate first; distinguish legitimate outliers (e.g., high-value fraud) from corrupted data
    correct: true
  - key: c
    text: Ignore; Z-score > 3σ is expected for normal distribution (~0.3%)
    correct: false
  - key: d
    text: Retrain model to be robust to outliers
    correct: false
- id: 6c398469-529b-43d0-9be6-1dd76f3da6a0
  type: flip
  front: 'Design anomaly detection for a feature with multi-modal distribution (e.g., transaction amounts:
    $5 for vending, $500 for online purchase). Z-score thresholds don''t work well. What''s your approach?'
  back: '**Multi-modal anomaly detection**:


    1. **Segment by mode**: Identify modes (e.g., amount < $50 vs $50-$1000). Set separate baselines/thresholds
    per segment. Z-score works within each mode.


    2. **Mixture models**: Fit Gaussian mixture model (GMM) to data; compute likelihood per point. Flag
    low-likelihood points as anomalies (not just tail extremes).


    3. **Isolation Forest**: Tree-based, works for multi-modal data without assuming distribution. Anomalies
    isolated in few splits.


    4. **Domain-based rules**: vending < $20, online $50-$2000. Flag anything outside expected range per
    transaction type.


    5. **Validate on labeled data**: If you have past frauds/errors, train supervised detector (classifier)
    on legitimate vs corrupted samples.


    **Recommendation**: Combine isolation forest (unsupervised, no distribution assumption) + domain rules
    (interpretable) for robustness.'
- id: df387509-623f-49fe-90e9-3eb660a043ce
  type: flip
  front: 'A feature shows cardinality explosion: normally 50 unique values, now 500. Most new values are
    one-hot encoded rare categories. Is this a data quality issue or label shift?'
  back: '**Likely data quality issue or encoding bug**, not label shift. Reasons:


    1. **One-hot encoding explosion**: If a new categorical value appears (e.g., new zip code 99999),
    one-hot encoder creates new column, increasing feature count. Sign: feature engineering pipeline changed
    or data source had corrupted entry (e.g., zip scraped from typo field).


    2. **Legitimate cardinality increase**: New product categories, new regions. Check if new values are
    valid (new business expansion) or garbage (encoding artifacts). Query data: what are the 450 new values?


    3. **Model impact**: If model can''t handle unseen categorical values at inference, it fails (NaN
    or default behavior). Retrain to include new values or use fallback encoding.


    **Action**: (1) Investigate new values: valid or corruption? (2) If valid, retrain with new categories.
    (3) If corruption, fix upstream. (4) If model can''t handle unseen values, add graceful handling (rare
    category bucket, default encoding).'
---

## Intuition
Data quality issues kill ML models silently. Missing features (NaN, null) break inference. Corrupted values (age=999, negative prices) confuse models. Anomalies (feature outliers, extreme combinations) are either real rare events (handle) or data bugs (alert). Monitor null %, outlier count, and value ranges per feature to catch upstream pipeline breaks before they degrade model.

## Detail
Missing value rate: Alert if null % > baseline (e.g., normally 0.1% age nulls, prod 5% = pipeline issue). Anomaly detection: (1) **Univariate**: Z-score (|x - μ| > 3σ), IQR (x > Q3 + 1.5*IQR), isolation forest. (2) **Multivariate**: Mahalanobis distance, LOF (local outlier factor), DBSCAN. Set thresholds during training (e.g., top 1% outliers in train = normal), then flag anomalies in prod. Value range checks: if age should be [0, 120], flag age=999. Cardinality explosion: if zip_code usually has 1000 values, alert if > 2000. Timestamp checks: ensure data is fresh (recent timestamps), not stale.

## Common gotchas / interview framings
- Over-alerting on transient nulls; set 1-hour buffer for pipeline catchup
- Confusing legitimate outliers (high-value transactions) with corrupted data
- Not accounting for seasonality in anomaly thresholds
- Z-score assumes normality; IQR more robust for skewed features

## See also
- [[feature-distribution-monitoring]]
- [[data-freshness]]
- [[schema-drift-and-pipeline-monitoring]]
- [[model-performance-degradation-accuracy-drop-calibration-shift]]

## Sources
See frontmatter `sources:`.
