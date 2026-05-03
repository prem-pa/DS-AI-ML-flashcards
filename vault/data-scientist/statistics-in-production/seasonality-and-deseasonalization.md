---
id: 93a7d22d-cd68-4804-8c2b-5484d538d2b4
title: Seasonality and deseasonalization
track: data-scientist
topic: statistics-in-production
difficulty: 3
tags:
- time-series
- seasonality
- detrending
- forecasting
- signal-processing
aliases:
- seasonal adjustment
- trend removal
- STL decomposition
- seasonal patterns
sources:
- url: https://netflixtechblog.com/lessons-from-building-observability-tools-at-netflix-7cfafed6ab17
  label: 'Netflix: Observability Tools and Seasonal Patterns'
- url: https://www.thedataletter.com/p/how-netflix-does-data-reliability
  label: How Netflix Does Data Reliability
- url: https://twosigmaventures.com/blog/article/the-rise-of-ml-ops-why-model-performance-monitoring-could-be-the-next-billion-dollar-industry/
  label: 'Two Sigma: Model Performance Monitoring'
- url: https://www.datacouncil.ai/talks/anomaly-detection-for-data-quality-and-metric-shifts-at-netflix
  label: 'Data Council: Anomaly Detection at Netflix'
cards:
- id: fb7827e6-fb48-47ea-9e51-eb1c1b1a98dc
  type: flip
  front: Your model's accuracy is 92% on Mondays and 88% on Thursdays, every week. Your guardrail fires
    Monday alerts due to high variance. How do you fix this?
  back: 'Compute day-of-week seasonal factors: for each day, calculate the median accuracy over the past
    12 weeks. Subtract the day-of-week factor from the current accuracy to deseasonalize. Set guardrail
    thresholds on the deseasonalized metric (residual), not raw accuracy. This isolates true anomalies
    from expected variation.'
- id: 307ec1dc-5155-418a-8e10-88fff15351dc
  type: mcq
  front: You're monitoring a production metric with both hourly and daily seasonality (e.g., CTR). Which
    decomposition method is most robust to changing patterns?
  back: STL with rolling windows detects and adapts to changing seasonality patterns. Fixed factors miss
    evolution; simple subtraction ignores trends; ignoring seasonality floods guardrails with false positives.
  choices:
  - key: a
    text: Fixed seasonal factors; recompute once per month.
    correct: false
  - key: b
    text: STL decomposition with a rolling window (e.g., last 60 days) to adapt to evolving seasonality.
    correct: false
  - key: c
    text: Simple median subtraction for each hour of the day.
    correct: false
  - key: d
    text: Ignore seasonality; set very wide guardrails.
    correct: false
- id: 9099ea46-decd-47a5-bb65-f8f4889aecee
  type: flip
  front: During Black Friday, your model's performance deviates wildly from seasonal patterns. Your deseasonalized
    residual shows a large anomaly. Is this a model failure or a known event?
  back: This is likely a known event (Black Friday traffic surge), not a model failure. Deseasonalization
    removes calendar seasonality but not event-based spikes. Add an exogenous variable (e.g., 'is_black_friday'
    or 'marketing_spend') to the deseasonalization model, or maintain a calendar of planned events. Flag
    calendar events separately so on-call teams don't confuse expected anomalies with real failures.
- id: fef6c548-d850-4abd-aab4-4b2dd083056b
  type: mcq
  front: You've deseasonalized a time series and now monitor the residuals. The residuals show a slow
    drift from 0 to +2. Your guardrail (threshold = 3σ of residuals) doesn't fire. Why is this dangerous?
  back: As residuals drift, their variance increases, causing the threshold to widen (σ grows). CUSUM
    or online learning methods are better for detecting gradual shifts than fixed thresholds on growing
    variance.
  choices:
  - key: a
    text: The guardrail is too tight; should be 2σ.
    correct: false
  - key: b
    text: Slow drift is expected; no alert needed.
    correct: false
  - key: c
    text: The 3σ threshold assumes stationarity; drift inflates residual variance, widening the threshold.
      Use a smaller window or CUSUM to detect gradual drift.
    correct: false
  - key: d
    text: Residuals should always be zero; something is wrong with deseasonalization.
    correct: false
---

## Intuition

Seasonality is predictable, recurring variation in metrics at fixed intervals (daily, weekly, yearly). Deseasonalization removes these known patterns, exposing underlying anomalies. Without deseasonalization, your guardrails trigger on expected fluctuations (Monday traffic is always 30% lower), burying real issues.

## Detail

**Seasonal Patterns in Production**:
- **Intra-day**: Traffic peaks at lunch, evening; drops at 3 AM. Model accuracy may vary by traffic profile.
- **Weekly**: Weekends show different user behavior (lower B2B engagement, higher social usage).
- **Yearly**: Black Friday/Cyber Monday spike; holiday season drops; back-to-school surge.
- **Event-based**: Product launches, marketing campaigns, competitor moves inject non-seasonal anomalies.

**Deseasonalization Techniques**:

**Simple Subtraction**: Compute hourly/daily/weekly median for each season, subtract from current observation. Fast, interpretable, works for stable patterns.

**STL Decomposition** (Seasonal-Trend decomposition via LOESS): Splits time series into seasonal, trend, and residual components via robust regression. Handles slowly-changing seasonal patterns. Netflix uses variants for high-cardinality metrics.

**Regression-based**: Fit a model with seasonal dummies (one-hot encoded day-of-week, hour-of-day). Residuals are deseasonalized. Flexible, supports covariates (temperature, campaign spend).

**Anomaly Detection on Residuals**: After removing seasonality, anomalies in the residual component reveal true shifts (e.g., model drift, data quality issues).

## Common gotchas / interview framings

- **Changing seasonality**: User behavior evolves; last year's seasonal pattern no longer applies. Use adaptive methods (EWMA, online learning) to update seasonal factors.
- **Overfitting to noise**: If you deseasonalize too aggressively (capturing day-level noise), residuals hide true anomalies. Use rolling windows (last 30 days) and validate on holdout.
- **Missing external context**: Deseasonalization assumes the only pattern is time-of-period. But marketing campaigns, weather, or competitor actions add irregular seasonality. Include exogenous variables in regression.
- **Lag in deseasonalization**: If you compute seasonal factors hourly but lag by 1 hour, alerts trigger late. Pre-compute seasonal factors at inference time.

## See also
- [[guardrail-metrics-and-alert-thresholds]]
- [[dashboards-and-real-time-reporting]]
- [[anomaly-detection]]
- [[outlier-detection]]

## Sources
See frontmatter `sources:`.
