---
id: f047a763-fd31-462a-92bd-694c16a8ecfc
title: Guardrail metrics
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- metrics
- risk-mitigation
- monitoring
- side-effects
- business-impact
aliases:
- guard rails
- metric thresholds
- red lines
- harm detection
sources:
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: A/B Testing and Guardrails'
- url: https://research.netflix.com/research-area/experimentation-and-causal-inference
  label: 'Netflix: Experimentation Best Practices'
cards:
- id: d06d95ef-09c9-4e9c-a9d8-22ff35efc4bb
  type: flip
  front: ''
  back: ''
- id: 61381160-6883-41a0-995a-63e349679742
  type: flip
  front: ''
  back: ''
- id: 0e459f71-3a97-439c-b3c6-c8e0ba4ff855
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Yes, point estimate (0.8) is below 1
    correct: false
  - key: b
    text: No, confidence interval includes 1.4 pp
    correct: false
  - key: c
    text: Yes, we're 95% confident crash rate increase is < 1.4 pp
    correct: false
  - key: d
    text: No, the upper bound exceeds the guardrail
    correct: false
- id: b4869c0e-7aff-4d5c-a6ab-04c721a3e110
  type: flip
  front: ''
  back: ''
---

## Intuition

A guardrail metric is a metric you'll **block the launch on if it gets worse**, even if primary succeeds. Guardrails prevent launching features that win on one metric but break something critical.

## Detail

**Common guardrail metrics**:
- **Revenue**: Don't launch if revenue drops >2%
- **Latency**: Don't launch if p99 latency increases >10%
- **Crash rate**: Don't launch if crashes increase >0.1pp
- **Churn rate**: Don't launch if 30-day churn increases >1pp
- **Support tickets**: Don't launch if support volume increases >5%
- **User satisfaction**: Don't launch if NPS drops >3 points

**Types of guardrails**:
1. **Absolute guardrail**: Set a threshold (e.g., "crash rate < 0.5%")
2. **Relative guardrail**: Set a % change (e.g., "no more than -5% revenue")
3. **Trending guardrail**: Watch for trends (e.g., "latency increasing day-over-day")

**Key principle**: Guardrails are **binary blocks**, not context. If guardrail is violated, don't launch, period. If primary is +10% but revenue is -3%, don't launch.

**Setting guardrails**:
- Consult stakeholders: What are the absolute no-nos? (Revenue, latency, safety)
- Set realistic thresholds: Don't set -0.1% revenue if variance is ±1%
- Use historical data: "What's a material change?" e.g., -2% revenue is 1 week of company revenue loss

## Common gotchas / interview framings
- "Primary is +5%, but guardrail (revenue) is -1%. Launch?" → No, guardrail blocks. Re-examine feature: does it cannibalize revenue? Iterate design
- "Should we tighten guardrails to reduce risk?" → Yes, but too tight → tests fail to launch (false negatives). Balance: set at 2-3x minimum detectable change
- "If guardrail is triggered, do we run the test longer?" → No, longer test won't fix a true harm. Better: fix the feature or revert
- "Guardrail keeps bouncing around 0. Do we interpret as violated?" → If confidence interval includes the threshold, it's inconclusive. Wait for more samples or increase guardrail tolerance

## See also
- [[primary-vs-secondary-metrics]]
- [[metric-construction-ratios-counts-timing]]
- [[survivorship-bias]]

## Sources
See frontmatter `sources:`.
