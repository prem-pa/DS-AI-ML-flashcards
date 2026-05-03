---
id: d457b196-b98f-4717-be2d-11ccc9b322ed
title: Metric construction (ratios, counts, timing)
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- metrics
- ratio-metrics
- variance
- statistical-properties
- metric-design
aliases:
- ratio metric
- count metric
- binomial metric
- metric calculation
sources:
- url: https://www.statsig.com/perspectives/ab-test-sample-size
  label: 'Statsig: Ratio Metrics in A/B Testing'
- url: https://medium.com/@QuarkAndCode/power-analysis-for-a-b-tests-metrics-mde-sample-size-8a107625c791
  label: 'QuarkAndCode: Power Analysis for A/B Tests'
cards:
- id: eb42cb45-9341-41bf-8ff0-7974c33dd2ef
  type: flip
  front: ''
  back: ''
- id: 229e54eb-f8f8-4380-93e9-9b960691b8b6
  type: flip
  front: ''
  back: ''
- id: c7574d81-0c13-4e81-aa80-18a2a7bb9c6c
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Use 'monthly revenue per user' instead
    correct: false
  - key: b
    text: Use CUPED (variance reduction)
    correct: false
  - key: c
    text: Switch to a count metric (e.g., transactions per user)
    correct: false
  - key: d
    text: All of the above
    correct: false
- id: 5109679c-d38c-4add-8fc8-f245978f01df
  type: flip
  front: ''
  back: ''
---

## Intuition

How you calculate a metric affects its variance, power, and interpretability. Ratio metrics (e.g., CTR) are noisier than count metrics (e.g., clicks).

## Detail

**Count metrics**:
- Simple: total clicks per user
- Variance scales with count
- Example: E[clicks] = 10, Var[clicks] = 10 (Poisson-like)
- Power: higher (fewer samples needed)

**Ratio metrics** (CTR, conversion rate, ARPU):
- Ratio = numerator / denominator (e.g., conversions / visitors)
- Variance is higher: depends on both numerator and denominator variance
- Example: CTR = 10% with std dev 5% (higher variance than count)
- Power: lower (more samples needed for same effect size)
- Sensitivity: small changes in denominator amplify noise

**Cumulative metrics**:
- Count across a time window (e.g., 7-day revenue)
- Higher variance: longer window = more noise
- Better power if measuring user-lifetime (7-day revenue vs daily revenue)

**Timing**:
- **Event-level**: measure at moment of event (e.g., clicks per session) → captures fine-grained effects
- **Batch-level**: aggregate to user-day or user-week → captures sustained effects, less noisy
- **Post-experiment**: measure after experiment ends (e.g., 7-day retention after test) → captures long-term effects but slow decision

**Best practice**:
- Use count metrics if possible (lower variance)
- If ratio metric, ensure denominator is stable (e.g., CTR is clicks/impressions; if impressions vary wildly, CTR is noisy)
- Use longer aggregation windows (user-week vs user-day) to reduce variance
- Combine counts: instead of CTR = conversions/visitors, use click rate (clicks/user) separately

## Common gotchas / interview framings
- "Our click metric has high variance. Should we use CTR instead?" → No, CTR might have *even higher* variance if impression count varies. Better: use CUPED or longer aggregation window
- "We measure daily revenue, but it's noisy. Should we measure monthly revenue?" → Yes, longer windows reduce variance. Trade-off: slower to detect short-term effects
- "CTR is up 2%, conversion rate is down 1%. Did the feature work?" → Investigate: did clicks increase but conversions decrease? That's a red flag (more engagement, less conversion). Check denominator stability
- "Can we truncate ratio metrics (e.g., cap CTR at 100% to reduce outliers)?" → Only if outliers are errors. If real, truncation introduces bias

## See also
- [[primary-vs-secondary-metrics]]
- [[variance-reduction-techniques-cuped-stratification]]
- [[instrumentation-and-logging]]

## Sources
See frontmatter `sources:`.
