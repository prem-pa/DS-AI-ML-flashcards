---
id: fe3b0dcc-30b0-4100-859a-469b40f8891a
title: Outlier detection
track: data-scientist
topic: statistics-in-production
difficulty: 3
tags:
- outlier-detection
- statistical-testing
- anomaly-detection
- data-quality
- robustness
aliases:
- extreme-value detection
- anomalies
- statistical outliers
- domain outliers
sources:
- url: https://netflixtechblog.com/tracking-down-the-villains-outlier-detection-at-netflix-40360b31732
  label: 'Netflix: Tracking Down the Villains—Outlier Detection'
- url: https://www.bigeye.com/blog/data-in-practice-anomaly-detection-for-data-quality-at-netflix
  label: 'Bigeye: Anomaly Detection for Data Quality'
- url: https://www.twosigma.com/articles/treating-data-as-code-at-two-sigma/
  label: 'Two Sigma: Data as Code'
- url: https://www.datacouncil.ai/talks/anomaly-detection-for-data-quality-and-metric-shifts-at-netflix
  label: 'Data Council: Anomaly Detection at Scale'
cards:
- id: 11b4a88a-3a49-4bf6-9a3c-c633d5fae7ed
  type: flip
  front: 'Your dataset has user purchase amounts: median $50, IQR [$20–$100]. You flag outliers as > Q3
    + 1.5*IQR = $150. A purchase of $155 is removed as an outlier. But that customer has 100 purchases
    averaging $100. Is this outlier real?'
  back: 'This is a false positive. The purchase is statistically outlying (> 150) but normal for this
    user in context. Better approach: use contextual outliers—flag values that are anomalous relative
    to user history (e.g., > user_mean + 3*user_std). Or use a domain rule: high purchases are normal
    if the customer has high lifetime value. Domain context reduces false positives.'
- id: 1c0b3d6a-f61b-47f5-9950-3aace9308707
  type: mcq
  front: Your training data has 1% of values as statistical outliers (> mean + 3σ). You remove them before
    training. At inference, 1% of new data are also outliers. Your model performs poorly. Why?
  back: 'Train/test shift: model is trained on data without 1% outliers, but at inference, 1% are outliers
    (distribution mismatch). Solutions: keep outliers in training, use robust models (e.g., Huber loss,
    Isolation Forest), or flag outliers at inference with a separate handling pathway.'
  choices:
  - key: a
    text: Training data had insufficient outliers; model didn't learn to handle them.
    correct: false
  - key: b
    text: Removing outliers during training but not inference causes distribution mismatch (train/test
      shift).
    correct: false
  - key: c
    text: The outlier threshold should be wider (e.g., 4σ instead of 3σ).
    correct: false
  - key: d
    text: Outliers should never be removed; use robust loss functions instead.
    correct: false
- id: 15749cb0-34dd-482f-b5ba-40c37b0c7d42
  type: flip
  front: You monitor a metric with a 99th percentile threshold for outlier detection. For 12 months of
    data, the 99th percentile is $1000. One month, you see values > $1000 suddenly appear—is this outlier
    inflation or real change?
  back: 'Unclear without context. This could be: (1) real business change (e.g., holiday season, new market),
    (2) data quality issue (encoding error, currency conversion bug), or (3) concept drift (customer behavior
    evolving). Investigate: (a) check timestamps—if outliers are recent, use a rolling percentile (last
    30 days) to adapt. (b) Correlate with external events (campaign, policy change). (c) Validate data
    quality. Don''t use static percentiles; use sliding windows.'
- id: cec0a1b9-551a-41ab-b578-7928ca7e6f57
  type: mcq
  front: You want to detect multivariate outliers (e.g., unusual combinations of features). Between Mahalanobis
    distance, Isolation Forest, and DBSCAN, which is best for high-dimensional production data (100+ features)?
  back: Mahalanobis distance (a) requires estimating covariance (100×100 matrix), prone to numerical instability
    in high dims. DBSCAN (c) requires tuning and doesn't scale well. Isolation Forest (b) avoids distance
    computation, scales well, and is fast. It's a strong default for high-dimensional production outlier
    detection.
  choices:
  - key: a
    text: Mahalanobis distance; works in all dimensions.
    correct: false
  - key: b
    text: Isolation Forest; scales well, works on any distribution, less sensitive to curse of dimensionality.
    correct: false
  - key: c
    text: DBSCAN; requires tuning eps but very accurate.
    correct: false
  - key: d
    text: All three equally; choose by accuracy on holdout.
    correct: false
---

## Intuition

Outliers are extreme values that don't match the bulk of data. In production, outliers can be legitimate (billion-dollar transactions, superusers) or errors (sensor malfunction, data corruption). Detecting and handling outliers is critical for model training (outliers skew parameters) and inference (outliers may be adversarial).

## Detail

**Statistical Outlier Detection**:

**Z-score**: Flag values > mean ± 3σ. Assumes normality; breaks on skewed/heavy-tailed data.

**Modified Z-score (MAD-based)**: Flag values where |x - median| > 2.5 * MAD. Robust to skewness and outliers in parameter estimation.

**IQR (Interquartile Range)**: Flag values < Q1 - 1.5*IQR or > Q3 + 1.5*IQR. Common in exploratory analysis, robust, but arbitrary multiplier.

**Percentile-based**: Flag values in tails (e.g., < 1st percentile or > 99th percentile). Simple, adapts to data, but misses subtle anomalies.

**Domain-based Outlier Detection**: Business logic defines valid ranges. E.g., age must be in [18, 120], price must be > 0. Catches errors that statistical methods miss (a price of -$10 is unusual statistically, but a business rule catches it).

**Multivariate Outliers**: A single feature may be normal, but the combination is rare. E.g., age 85 + income $500K is unusual. Use Mahalanobis distance, Isolation Forest, or DBSCAN.

**Contextual Outliers**: An observation may be normal in isolation but anomalous in context. E.g., CPU usage 80% is normal at peak hours but anomalous at 3 AM.

## Common gotchas / interview framings

- **Removing outliers without investigation**: Outliers may be real (superusers, large transactions, security breaches). Removing them loses information. Better: flag for review, fit robust models (resistant to outliers), or use separate modeling paths for outliers.
- **Outlier leakage**: Identifying outliers on the full dataset, then training on the outlier-removed set, introduces bias. Identify outliers on training data only; apply same threshold to test data.
- **Conflating statistical and domain outliers**: A statistically extreme value (e.g., 6σ) may be legitimate; a statistically normal value may violate business rules. Use both approaches.
- **Temporal outliers**: Outlier thresholds should adapt over time (distributions change). Use sliding windows or online learning.

## See also
- [[anomaly-detection]]
- [[data-validation-and-schema-checks]]
- [[missing-data-and-imputation]]
- [[guardrail-metrics-and-alert-thresholds]]

## Sources
See frontmatter `sources:`.
