---
id: 6485f25a-0da1-4769-82f2-e51f2ea77050
title: Anomaly detection
track: data-scientist
topic: statistics-in-production
difficulty: 3
tags:
- anomaly-detection
- outlier-detection
- statistical-testing
- unsupervised-learning
- time-series
aliases:
- outlier detection
- deviation detection
- change-point detection
- statistical monitoring
sources:
- url: https://netflixtechblog.com/tracking-down-the-villains-outlier-detection-at-netflix-40360b31732
  label: 'Netflix: Tracking Down the Villains—Outlier Detection'
- url: https://www.bigeye.com/blog/data-in-practice-anomaly-detection-for-data-quality-at-netflix
  label: 'Bigeye: Anomaly Detection for Data Quality at Netflix'
- url: https://www.datacouncil.ai/talks/anomaly-detection-for-data-quality-and-metric-shifts-at-netflix
  label: 'Data Council: Anomaly Detection at Netflix'
- url: https://www.twosigma.com/articles/treating-data-as-code-at-two-sigma/
  label: 'Two Sigma: Treating Data as Code'
cards:
- id: 54de7740-2926-4119-b46b-3a3a5157e9a2
  type: flip
  front: You're monitoring a recommendation system's precision across 1,000 user segments in real-time.
    Precision is skewed (median 0.70, mean 0.65, some segments 0.95). Z-scores fire alerts on segments
    with 0.95 precision. Why, and how do you fix it?
  back: 'Z-scores assume normality and use mean/std; skewed data has high std (due to outliers), so the
    z-score threshold is too tight. Instead, use Median Absolute Deviation (MAD): |precision - median|
    > 2.5 * MAD. Or use Robust PCA to detect anomalies in the segment-level correlation structure, treating
    individual segments as low-rank (normal) + sparse (anomalous).'
- id: 6fd3b293-aa95-498a-be7a-f77c0fe21de4
  type: mcq
  front: Your DBSCAN-based anomaly detector requires tuning eps and min_samples. In production, data distribution
    shifts weekly. Which approach best handles this?
  back: Retraining with sliding windows adapts to evolving distributions. Percentile-based eps automatically
    scales with data density changes. Fixed parameters lead to concept drift; z-scores are simpler but
    less robust to multimodality.
  choices:
  - key: a
    text: Use fixed eps/min_samples; tune once, never change.
    correct: false
  - key: b
    text: Retrain DBSCAN daily with a sliding 30-day window; use percentile-based eps (e.g., 99th percentile
      distance).
    correct: false
  - key: c
    text: Switch to z-scores; no parameters.
    correct: false
  - key: d
    text: Use CUSUM only; DBSCAN is overkill for production.
    correct: false
- id: bd35bf47-a7be-4cb9-8af0-f905be36505d
  type: flip
  front: Your time-series anomaly detector uses Exponential Smoothing. It forecast yesterday's value correctly
    but missed today's sudden 20% drop. Why, and how do you detect this drop faster?
  back: 'Exponential smoothing is retrospective—it forecasts based on past values, so sudden shifts lag
    detection by ~1 period. To detect faster, combine Exponential Smoothing with CUSUM: accumulate the
    standardized residuals (actual - forecast) / std_forecast. CUSUM detects small sustained shifts faster
    than fixed thresholds on individual residuals.'
- id: 170b7be8-899f-4942-86bf-b77e5002b825
  type: mcq
  front: A contextual anomaly detector flags model accuracy as anomalous on Thursdays (85%) vs. other
    days (92%), even though both are within historical ranges for those days. Is this a bug or a feature?
  back: Contextual anomalies are real but should be detected after deseasonalization. If 85% is normal
    for Thursdays, it shouldn't trigger an alert. Deseasonalize first, then detect anomalies in residuals.
  choices:
  - key: a
    text: Bug; both are normal. Detector is too sensitive.
    correct: false
  - key: b
    text: Feature; 85% is anomalous on Thursdays because Thursdays are usually 92%. Indicates real degradation
      on that day.
    correct: false
  - key: c
    text: Neither; seasonality should be removed first. After deseasonalization, 85% would be flagged
      only if it's unexpectedly low for a Thursday.
    correct: false
  - key: d
    text: Indicates data quality issue; investigate Thursday data sources.
    correct: false
---

## Intuition

Anomalies are observations that deviate significantly from expected behavior. In production, anomalies signal data quality issues (corrupt records, upstream failures), model failures (distribution shift, feature engineering bugs), or real events (security breaches, system overload). Detecting them early prevents cascading failures.

## Detail

**Statistical Approaches**:

**Z-score / Standard Deviation**: Flag values > mean ± 3σ. Fast, interpretable, assumes normality. Fails on skewed data (outliers inflate σ) and multimodal distributions.

**Median Absolute Deviation (MAD)**: Robust z-score using median and MAD instead of mean/std. Better for skewed data. Threshold: |x - median| > 2.5 * MAD is approximately 99% confidence interval under normality.

**Robust PCA (RPCA)**: Netflix's RAD tool decomposes high-dimensional metrics into low-rank (normal behavior) and sparse (anomalies) components. Effective for correlated, high-cardinality metrics (per-server, per-country).

**DBSCAN / Isolation Forest**: Unsupervised clustering/tree-based methods. DBSCAN groups dense regions; points in sparse regions are anomalies. Isolation Forest isolates points using random decision trees. Both handle multivariate, non-Gaussian data but require tuning.

**Time-series Methods**:
- **ARIMA Residuals**: Fit an ARIMA model, flag residuals > threshold.
- **Exponential Smoothing**: Forecast with Holt-Winters, compare forecast to observation.
- **CUSUM (Cumulative Sum Control Chart)**: Detect small sustained shifts by accumulating deviations. More sensitive to drift than fixed thresholds.

**Contextual Anomalies**: An observation may be normal in isolation but anomalous given context (e.g., model accuracy 80% is normal, but 80% on a Thursday is anomalous if Thursdays are usually 85%).

## Common gotchas / interview framings

- **High false-positive rate**: Anomaly detection methods flag too many innocuous deviations. Root cause: no deseasonalization, no adaptation to natural variance. Solution: deseasonalize first, tune thresholds on historical data.
- **Delay in detection**: Methods that fit models (ARIMA, RPCA) have latency. For real-time alerts, use lightweight methods (MAD, z-score) with frequent updates.
- **Ignoring domain context**: Statistical anomalies (unusual but harmless) differ from business anomalies (costly). A 5% traffic spike is anomalous but not concerning; a 5% accuracy drop is both.
- **Concept drift**: Anomaly thresholds based on old data become miscalibrated as data distributions shift. Use online learning (update model incrementally) or sliding windows.

## See also
- [[guardrail-metrics-and-alert-thresholds]]
- [[outlier-detection]]
- [[seasonality-and-deseasonalization]]
- [[data-validation-and-schema-checks]]
- [[dashboards-and-real-time-reporting]]

## Sources
See frontmatter `sources:`.
