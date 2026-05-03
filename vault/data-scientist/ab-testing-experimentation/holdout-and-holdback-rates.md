---
id: d1a7a156-d36b-434e-b1f8-0632e45718b9
title: Holdout and holdback rates
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- traffic-allocation
- experiment-design
- ramp-up
- risk-mitigation
- sample-size
aliases:
- holdout group
- control holdout
- traffic split
- ramp fraction
sources:
- url: https://www.statsig.com/perspectives/ab-test-sample-size
  label: 'Statsig: A/B Test Sample Size Planning'
- url: https://www.mida.so/blog/how-to-calculate-power-statistics-for-ab-testing
  label: 'Mida: Power Statistics for A/B Testing'
cards:
- id: 39bdb4a8-cb24-4a20-9fbe-92583da44252
  type: flip
  front: ''
  back: ''
- id: 2c19235d-09b3-4c3e-9ff1-bf1eb34f0a26
  type: flip
  front: ''
  back: ''
- id: fa9eb3af-a041-4b30-9c70-aab9b0396ecb
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Holdback inflates variance
    correct: false
  - key: b
    text: 250k per arm is smaller sample size than 500k
    correct: false
  - key: c
    text: Holdback users have different characteristics
    correct: false
  - key: d
    text: No, power is the same if split is 50-50
    correct: false
- id: 88dec004-9d94-46f1-9175-e4b223dc9bbf
  type: flip
  front: ''
  back: ''
---

## Intuition

A **holdout** is a group of users permanently kept out of treatment, used as an ongoing control. **Holdback** is the fraction of users not randomized (kept on old version). Both are tools for risk mitigation and understanding long-term effects.

## Detail

**Holdout** (1-10% of users):
- Permanent control group, never receives updates
- Used to measure **causal effect of all changes over time** ("would users be different if we reverted everything?")
- Detects slow-moving effects (habit formation, churn acceleration)
- Requires careful monitoring: holdout users may be an artifact over time (old app version, stale data)

**Holdback** (0-50% of users not randomized in current test):
- Users excluded from this specific A/B test
- Reasons: engineering constraints, risk mitigation, incremental ramp-up
- Example: test feature on 50% traffic (50% holdback, 25% control, 25% treatment)
- Helps if feature is risky; ramp up from 25% → 50% → 100% as you gain confidence

**Holdout vs holdback**:
- **Holdout**: Long-term, never updated; measures cumulative effect of all launches
- **Holdback**: Short-term, same users as usual; protects against this test's failures

**Best practice**:
- Keep 1% permanent holdout for causal accounting
- Use 50-50 control/treatment for individual tests (no holdback needed if infrastructure is stable)
- Only holdback if feature is very risky or infrastructure can't handle full rollout

## Common gotchas / interview framings
- "We have a 10% holdout. Does that reduce our power?" → No, holdout is separate. 50% control/treatment within the other 90% is still valid power-wise
- "Can we use the holdout as control for this test?" → No, holdout is stale (old code). Use concurrent randomization
- "Should we holdback 90% traffic because we're nervous?" → No, that's wasting data. If nervous, run on smaller % (10% treatment) but still 50-50 split
- "Holdout users are churning. Should we remove it?" → Investigate: are they churning because feature prevented something? If so, holdout is working. If they're just old, refresh cohort

## See also
- [[velocity-sample-size-per-day]]
- [[randomization-and-control-groups]]
- [[guardrail-metrics]]

## Sources
See frontmatter `sources:`.
