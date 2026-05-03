---
id: 9018cc2d-1b07-4e41-9273-8d8a7dd63e5f
title: Missing data and imputation
track: data-scientist
topic: statistics-in-production
difficulty: 3
tags:
- missing-data
- imputation
- MCAR
- MAR
- MNAR
- bias
aliases:
- data imputation
- missingness mechanisms
- handling nulls
- incomplete data
sources:
- url: https://www.thedataletter.com/p/how-netflix-does-data-reliability
  label: How Netflix Does Data Reliability
- url: https://www.twosigma.com/articles/treating-data-as-code-at-two-sigma/
  label: 'Two Sigma: Treating Data as Code'
- url: https://www.bigeye.com/blog/data-in-practice-anomaly-detection-for-data-quality-at-netflix
  label: 'Bigeye: Data Quality Monitoring'
- url: https://medium.com/geekculture/how-the-tech-giants-are-ensuring-data-quality-with-12d1f731681d
  label: 'Tech Giants: Data Quality with ML'
cards:
- id: 8e7a7dcf-b962-4ad1-8181-e6668c4b6a66
  type: flip
  front: In your training data, income is missing for 30% of users, primarily those reporting < 50K annual
    income (strong correlation). You use mean imputation. Will your model have bias?
  back: 'Yes, strong bias. This is MAR (missingness depends on observed income level). Mean imputation
    replaces missing low-income values with the overall mean (diluting low-income representation), biasing
    the model toward overestimating income. Solution: use multiple imputation (MICE) or maximum likelihood,
    which preserve the relationship between income and other features. Alternatively, use a separate indicator
    variable (is_income_missing) to let the model learn the pattern.'
- id: 147efff9-f41b-4a46-b6ce-8a4543cc610d
  type: mcq
  front: You have time-series data with 20% missing values in daily temperature. Missingness is random
    (broken sensor). At inference, you must impute, but you can't use future values. Which method is best?
  back: Missingness is MCAR (random), so any method is theoretically unbiased. Forward-fill is practical
    for inference because you only use past data (no future leakage). Mean imputation (a) works statistically
    but forward-fill is more realistic temporally. MICE (c) is powerful but overkill for MCAR. Listwise
    deletion (d) discards data unnecessarily.
  choices:
  - key: a
    text: Mean imputation using all historical data.
    correct: false
  - key: b
    text: Forward-fill (last known value).
    correct: false
  - key: c
    text: MICE (multiple imputation by chained equations).
    correct: false
  - key: d
    text: Listwise deletion (remove days with missing temps).
    correct: false
- id: 5d72c398-e45a-48f4-9bcf-f12bb5a6a3c2
  type: flip
  front: You deploy a model trained with mean imputation for missing values. At inference, missingness
    jumps from 5% to 40% due to a data source outage. Your model's performance drops. Explain why and
    propose a fix.
  back: 'The model was trained on data with 5% mean imputation (close to true values). At inference, 40%
    are means (synthetic), far from true distributions. This distribution shift causes poor calibration.
    Fix: (1) retrain with higher missingness to match the new regime, (2) use a missingness indicator
    feature to let the model learn the pattern, or (3) switch to MICE, which better handles OOD missingness
    by modeling relationships, not just using overall means.'
- id: 96e10da1-42ec-484c-b198-e38cae2b43d1
  type: mcq
  front: During model development, you impute missing values in training and test sets using the combined
    mean (all data). At deployment, you impute using only training data mean. Which scenario is this addressing,
    and is it correct?
  back: In development, using combined mean leaks test information into imputed values (test labels influence
    the mean). At deployment (no test data yet), you impute using training mean only. This is a standard
    precaution against overfitting / leakage.
  choices:
  - key: a
    text: Addressing data leakage; correct. Development mean includes test labels; production mean is
      unbiased.
    correct: false
  - key: b
    text: Addressing concept drift; incorrect. Should use the same mean for consistency.
    correct: false
  - key: c
    text: Both (a) and additional insurance against test contamination; correct.
    correct: false
  - key: d
    text: Unnecessary complication; using combined mean everywhere is fine.
    correct: false
---

## Intuition

Missing data is unavoidable in production (sensors fail, users skip optional fields, integrations lag). Imputation (filling missing values) is necessary, but naive approaches introduce bias. The choice between listwise deletion, mean imputation, or sophisticated methods depends on why data is missing.

## Detail

**Missingness Mechanisms** (Rubin's taxonomy):

**MCAR (Missing Completely At Random)**: Missingness is independent of observed and unobserved data. E.g., sensor randomly fails 5% of the time. Listwise deletion (remove rows with any missing value) is unbiased but wasteful.

**MAR (Missing At Random)**: Missingness depends on observed data, not the missing value itself. E.g., users with low income are less likely to report income (missing income depends on observed education). Solution: multiple imputation by chained equations (MICE), maximum likelihood, or inverse probability weighting.

**MNAR (Missing Not At Random)**: Missingness depends on the unobserved value. E.g., users with high debt don't report debt (missing debt depends on debt itself). No unbiased solution; requires assumptions or sensitivity analysis.

**Imputation Methods**:

**Listwise Deletion**: Remove rows with any missing value. Unbiased under MCAR but loses data; biased under MAR/MNAR.

**Mean/Median Imputation**: Replace missing values with column mean/median. Fast, reduces variance, but underestimates true variance and correlation. Use cautiously.

**Forward-fill / Backward-fill** (time-series): For temporal data, use last observed value. Reasonable for slowly-changing features (customer demographics) but biased if true values drift.

**Multiple Imputation**: Generate m (e.g., 5–20) imputed datasets, train models on each, combine predictions (pool via Rubin's rules). Preserves uncertainty, reduces bias under MAR.

**Model-based Imputation** (KNN, regression, trees): Use observed features to predict missing values. Reduces bias under MAR but risks overfitting if imputation model is misspecified.

## Common gotchas / interview framings

- **Imputation leakage**: Training data uses imputation trained on all data (including test labels). At inference, you can't impute the same way. Solution: fit imputation on training data only, apply to test data.
- **Misidentifying missingness mechanism**: Assuming MCAR when truly MAR/MNAR introduces bias. Audit missing patterns: does missingness correlate with observed variables?
- **Over-imputation**: Imputing many columns can induce complex correlation patterns, leading to overfitting. Consider feature selection—drop columns with > 50% missingness.
- **Monitoring post-imputation**: After imputation, synthetic values may not reflect true distributions. Monitor imputation rates; if they surge, investigate the cause (upstream failure, schema change).

## See also
- [[data-validation-and-schema-checks]]
- [[outlier-detection]]
- [[data-lineage-and-versioning]]

## Sources
See frontmatter `sources:`.
