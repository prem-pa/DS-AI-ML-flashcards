---
id: a673a5fe-1b57-4ea7-bdb6-7023ed63b6ca
title: Variance reduction techniques (CUPED, stratification)
track: data-scientist
topic: ab-testing-experimentation
difficulty: 5
tags:
- variance-reduction
- cuped
- covariate-adjustment
- statistical-efficiency
- power-increase
aliases:
- CUPED
- covariate adjustment
- stratified randomization
- pre-experiment data
sources:
- url: https://docs.statsig.com/experiments/statistical-methods/methodologies/cuped
  label: 'Statsig: CUPED Documentation'
- url: https://medium.com/@garret.oconnell/cuped-for-switchback-tests-9e5b924ce1b0
  label: 'Medium: CUPED for Switchback Tests'
- url: https://www.geteppo.com/blog/cuped-bending-time-in-experimentation
  label: 'Eppo: CUPED and CUPED++ Bending Time'
cards:
- id: b773a247-3deb-41c8-aee6-bb8eff5dd671
  type: flip
  front: ''
  back: ''
- id: e4b335b5-0874-4fca-999f-ccf860ec79e2
  type: flip
  front: ''
  back: ''
- id: 2355e1e2-e01b-4283-8c12-f12f06929958
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Variance reduction = 1 - 0.3² = 91%
    correct: false
  - key: b
    text: Variance reduction = 0.3² = 9%
    correct: false
  - key: c
    text: No variance reduction, might add noise
    correct: false
  - key: d
    text: Need more info about metric distribution
    correct: false
- id: 1f66cfbe-74ad-479f-872f-c27ab6b99371
  type: flip
  front: ''
  back: ''
---

## Intuition

CUPED (Controlled-experiment Using Pre-Experiment Data) reduces variance by predicting each user's metric using historical data, then measuring treatment effect *relative to prediction*. This can speed up experiments by 25-65%.

## Detail

**CUPED mechanism**:
1. Collect pre-experiment metric for each user (e.g., last 7 days of revenue)
2. For each user in test, predict their metric under control using the pre-experiment data
3. Measure adjusted metric = observed - predicted
4. Run t-test on adjusted metric

**Why it works**: If past revenue correlates with current revenue (ρ > 0.5), then prediction reduces noise. Adjustment removes user-level baseline differences, focusing on *change* induced by treatment.

**Formula**: Adjusted_metric = observed_metric - β × (pre_experiment_metric - mean_pre)
- β = correlation between pre and post
- Variance reduction ~ 1 / (1 - ρ²)
- If ρ = 0.7 (high correlation), variance drops by 51%

**Stratification**:
- Divide users into strata (e.g., high/medium/low spenders)
- Randomize within each stratum (balanced design)
- Reduces variance if strata differ in metric

**When to use**:
- CUPED: When historical data is stable and correlated with outcome (e.g., revenue, engagement)
- Stratification: When you have discrete user segments and want balanced power

**Risks**:
- CUPED assumes pre-experiment and post-experiment relationship is stable (may break if user behavior shifts)
- Stratification can backfire if strata are misaligned with actual variance

## Common gotchas / interview framings
- "We use CUPED and got 50% variance reduction. Can we halve our sample size?" → No, variance reduction ≠ sample size reduction. You can run tests 50% faster (same sample size in less time). If you halve sample size, you get partial variance reduction, but combined effect is weaker power
- "Should we always use CUPED?" → No, only if pre-experiment data is correlated. For rare events (conversions), pre-data has ~0 correlation. Use CUPED for stable metrics (revenue, engagement)
- "CUPED assumes linear relationship. What if non-linear?" → CUPED assumes linearity. If relationship is non-linear (e.g., saturation), machine learning variants (e.g., CUPAC, MLCV) better
- "Can we use CUPED on all metrics?" → No, risky for guardrails (crash rate, latency) where relationship is unstable. Better for engagement/revenue

## See also
- [[velocity-sample-size-per-day]]
- [[variance-reduction-techniques-cuped-stratification]]
- [[sequential-testing-and-peek-penalties]]

## Sources
See frontmatter `sources:`.
