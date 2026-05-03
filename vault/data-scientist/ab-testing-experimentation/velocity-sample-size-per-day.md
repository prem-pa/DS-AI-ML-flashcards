---
id: 072d0e5e-aec7-4199-8684-ee7207000094
title: Velocity (sample size per day)
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- sample-size
- time-to-decision
- experiment-duration
- traffic
- power-analysis
aliases:
- daily velocity
- sample rate
- throughput
- units per day
sources:
- url: https://www.statsig.com/perspectives/power-analysis-ab-testing
  label: 'Statsig: Power Analysis for A/B Testing'
- url: https://guessthetest.com/calculating-sample-size-in-a-b-testing-everything-you-need-to-know/
  label: 'GuessTheTest: Sample Size Calculation'
cards:
- id: a43a66e3-3fb9-4da0-8530-6b7e63f6dbdd
  type: flip
  front: ''
  back: ''
- id: 04948704-6b53-4837-b5eb-a653a83a3df1
  type: flip
  front: ''
  back: ''
- id: 39e86cb5-9a02-4c0b-aa3c-f3c1acc336b9
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: 1 day (you have enough)
    correct: false
  - key: b
    text: 2 days (100k → 200k)
    correct: false
  - key: c
    text: 7 days (full week to avoid seasonality)
    correct: false
  - key: d
    text: 14 days (always 2 weeks minimum)
    correct: false
- id: 3d0d1dc4-6a25-413c-b2b0-0c9b6f5b7a80
  type: flip
  front: ''
  back: ''
---

## Intuition

Velocity is how many users (or events) you collect per day. High velocity = fast decisions. Low velocity = experiment runs forever.

## Detail

**Velocity = daily active users (DAU) × fraction randomized**

Example:
- 10M DAU, randomize 1% → 100k users/day
- 10M DAU, randomize 50% → 5M users/day

**Impact on duration**:
- Need 200k total samples (calculated from MDE, power, alpha)
- Velocity = 100k/day → 2 days to reach sample size
- Velocity = 10k/day → 20 days

**Trade-offs**:
- **High velocity**: Reach decision faster; but may overwhelm servers, annoy users
- **Low velocity**: Slow decision; but save resources, reduce user disruption

**Key decisions**:
1. How much traffic can we afford to randomize? (Engineering constraint)
2. How fast must we decide? (Business urgency)
3. Can we use CUPED or other variance reduction? (Increase effective velocity)

## Common gotchas / interview framings
- "Our DAU is 100k but we need 1M samples. Can we run the test for 10 days?" → Only if you randomize 100% traffic, which risks stability
- "Should we increase velocity by reducing holdback?" → No, holdbacks are separate from randomization fraction. Adjust randomization % instead
- "Can we increase velocity by using session-level randomization?" → Yes, sessions are more frequent than users, but introduces bias if same user sees both arms
- "Velocity is too low. Use CUPED?" → Yes, CUPED reduces variance, allowing smaller sample size, thus faster completion

## See also
- [[duration-and-seasonal-bias]]
- [[holdout-and-holdback-rates]]
- [[variance-reduction-techniques-cuped-stratification]]

## Sources
See frontmatter `sources:`.
