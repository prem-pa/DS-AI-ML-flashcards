---
id: 8156eefe-a1b5-4596-8ea9-67d2c12a6a6c
title: Feature distribution monitoring
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 3
tags:
- drift-detection
- feature-stats
- distribution-monitoring
- statistical-testing
- divergence-metrics
- alerting
aliases:
- input distribution monitoring
- covariate monitoring
- statistical drift
sources:
- url: https://www.evidentlyai.com/blog/data-drift-detection-large-datasets
  label: 'Evidently AI: Which test is best for detecting data drift on large datasets'
- url: https://link.springer.com/article/10.1007/s42488-024-00119-y
  label: 'Springer: Detecting drifts using KL divergence for data engineering'
cards:
- id: 273abf20-025a-4d7d-9ca4-54f80aa0191a
  type: flip
  front: 'You monitor customer age feature: mean=35 years in training, mean=42 in production (std_train=12).
    Calculate PSI-like shift metric and decide if you should alert.'
  back: 'Shift magnitude: (42 - 35) / 12 = 0.58 std deviations ≈ **0.58σ shift**


    Interpretation: Moderate shift. If baseline threshold is 0.5σ, this triggers alert. Age distribution
    shifted older (7 year diff). Likely cause: product user base aging, acquisition targeting changed,
    or cohort composition shift.


    Decision: **Alert, but investigate first**. Older users may have different credit profiles, risk,
    or behavior—this could degrade model if not accounted for. Check:

    1. Is age correlated with target (Y) in your model? If strong, retrain.

    2. Does performance (AUC) actually drop on new age distribution?

    3. Segment model metrics by age; if younger users degrade, retrain with age-stratified data.'
- id: 21177b7d-d21c-4bad-8b03-05891a9b8b60
  type: mcq
  front: You compute KL divergence on 50 features. 45 have KL < 0.05, but 5 have KL > 0.5. What is your
    most likely next action?
  back: '**Correct: b** Feature drift ≠ automatic retrain. First assess impact: (1) Are the 5 high-KL
    features important for your model (high SHAP values)? If marginal, ignore. (2) Do they have data quality
    issues (nulls, encoding errors)? (3) Does performance degrade on new distribution? Root-cause investigation
    before action. If 5 features are low-importance and performance stable, drift is not actionable. If
    features are high-impact and performance ↓, retrain.'
  choices:
  - key: a
    text: Immediately retrain the model
    correct: false
  - key: b
    text: Investigate the 5 high-KL features for data quality/pipeline issues; assess impact on model
    correct: true
  - key: c
    text: Ignore; most features are stable
    correct: false
  - key: d
    text: Disable the 5 drifted features from the model
    correct: false
- id: d428467d-b2c3-4841-8c2e-3cbddb7d0616
  type: flip
  front: Design a feature distribution monitoring dashboard. What metrics and visualizations would you
    include for a numerical feature?
  back: '**Feature distribution monitoring dashboard**:


    **Metrics (KPIs)**:

    - Mean, std, quantiles (Q1, median, Q3) vs baseline

    - PSI or KL divergence (scalar drift score)

    - Null count %

    - Cardinality (# unique values, for categorical)


    **Visualizations**:

    1. **Histogram overlay**: Training distribution (blue) vs current week (red), aligned x-axis

    2. **Box plot timeline**: Box plots by week, showing trend in median/IQR

    3. **KL divergence time series**: Weekly KL trend; alert threshold line

    4. **Q-Q plot**: Compare quantiles of train vs prod (diagonal = stable)

    5. **Correlation heatmap**: Feature correlations in train vs prod (catch multivariate drift)


    **Alerting**:

    - KL or PSI > threshold (e.g., 0.15)

    - Mean drift > 1σ

    - Sudden spike in nulls

    - Cardinality explosion (e.g., new categorical values appearing)


    Tools: Evidently, Great Expectations, or custom Prometheus + Grafana.'
- id: 60cfbb94-4a8a-45ad-8327-974bbad047be
  type: flip
  front: Your model was trained on Q4 (holiday season, high spending). You deploy in January (post-holiday,
    low spending). Feature distribution shifts drastically. Should you retrain immediately?
  back: '**No—first seasonality-adjust**. Spending is highly seasonal; January vs Q4 shift is expected,
    not drift.


    Steps: (1) **Establish seasonal baseline**: Retrain feature monitoring using same season (Jan vs Jan,
    Q4 vs Q4). (2) **Adjust monitoring thresholds**: Higher PSI tolerance for seasonal shifts. (3) **Check
    model performance** on Jan data; if accuracy stays stable, model is robust to seasonality. (4) **Retrain
    only if**:

    - Performance actually degrades in Jan

    - Jan behavior differs from historical Jan patterns (true drift)

    - Feature relationships changed (e.g., spending no longer correlates with fraud risk)


    Instead: Implement seasonal models or train on balanced seasonal mix (multi-year history).'
---

## Intuition
Each feature has a distribution in training data (baseline). In production, if feature statistics change—mean income ↑ 20%, fraud rate ↓ 50%, feature skewness flips—this signals data drift. Monitor mean, std, quantiles, cardinality (for categorical). Use divergence metrics (KL, PSI, JS) to quantify shift. Features that drift are early warning signs of model degradation.

## Detail
For each numerical feature: compute baseline mean μ_train, std σ_train from training data. Monthly, compute μ_prod, σ_prod on production. Alert if |μ_prod - μ_train| / σ_train > 0.5 (0.5 std devs shift). For categorical: monitor distribution of each category. Use Population Stability Index (PSI) = Σ (observed% - expected%) * ln(observed% / expected%); PSI > 0.1 indicates moderate drift. Kolmogorov-Smirnov test compares CDFs (nonparametric, works for any distribution). Tools: Evidently (open-source), Arize, WhyLabs. NannyML excels at predicting drift before performance dips. Precision requires enough samples (central limit theorem; n > 30 per bin for normal approx).

## Common gotchas / interview framings
- Monitoring raw counts instead of proportions; feature can drift in composition without distributional shift
- Using univariate tests (KS, PSI) missing multivariate correlations; features drift together
- Seasonal effects: income naturally higher in Q4; should compare to Q4 baseline, not annual
- Threshold tuning: PSI=0.1 may be noisy for rare features, unrealistic for high-cardinality features

## See also
- [[prediction-drift-and-label-shift]]
- [[missing-value-and-anomaly-monitoring]]
- [[data-freshness]]
- [[model-performance-degradation-accuracy-drop-calibration-shift]]

## Sources
See frontmatter `sources:`.
