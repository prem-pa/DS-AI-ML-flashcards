---
id: 1ab90bc6-7958-4f4a-ba7e-42ef523dc8c1
title: Duration and seasonal bias
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- experiment-duration
- seasonal-effects
- day-of-week
- temporal-confounds
- time-to-decision
aliases:
- run duration
- seasonality
- temporal bias
- day-of-week effect
sources:
- url: https://blog.analytics-toolkit.com/2022/statistical-power-mde-and-designing-statistical-tests/
  label: 'Analytics Toolkit: Statistical Power & MDE'
- url: https://www.abtasty.com/blog/sample-size-calculation/
  label: 'AB Tasty: Sample Size Best Practices'
cards:
- id: 28f0190f-49be-45e9-b7d5-c84bf8640c50
  type: flip
  front: ''
  back: ''
- id: ba85ed22-c7b9-45cf-9471-f7080ba31f17
  type: flip
  front: ''
  back: ''
- id: 767c43b3-dbed-4a01-82ce-f20045ecd867
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: No effect doesn't mean absence of effect; test might be underpowered
    correct: false
  - key: b
    text: You didn't test weekends; B2B users may behave differently on weekends
    correct: false
  - key: c
    text: You only tested weekdays; weekend behavior might differ
    correct: false
  - key: d
    text: The test was too short; you need at least 2 weeks
    correct: false
- id: fc3ffae5-02a9-458e-8bd3-26a87478f4a5
  type: flip
  front: ''
  back: ''
---

## Intuition

User behavior varies by day-of-week (weekday vs weekend) and season (summer vs winter). Running an experiment over too short a time risks capturing anomalies, not true effects.

## Detail

**Day-of-week effects**:
- Weekends: different traffic, engagement, revenue (e.g., shopping surge, gaming)
- Weekdays: work users vs leisure users
- Monday: bounce-back effect; Tuesday-Thursday: steady; Friday: plans shift

**Seasonal effects**:
- Summer/winter: vacation patterns, shopping seasons
- Holidays: holidays change behavior
- Academic calendar: back-to-school, graduation
- Weather: rain vs sun

**Solution**: Run experiments for **at least 1-2 weeks** (one full cycle of Mon-Sun).
- 1 week: captures one cycle; better than 1-2 days
- 2 weeks: captures two cycles; more robust
- 4+ weeks: captures longer seasonality (payday cycles, habit formation)

**Minimum duration**: 1 week minimum (7 days); 2 weeks recommended; never 2-3 days.

**Why not just adjust for day-of-week?** You could stratify or block by day, but:
- Adds complexity
- Assumes day effect is the same in treatment and control (interaction effects)
- Best practice: run through the natural cycle

## Common gotchas / interview framings
- "We have enough power; can we stop at day 3?" → No. Day 3 might be a lucky day (e.g., viral news boosted traffic). Run the full week
- "Should we exclude weekends from the test?" → No, your users shop on weekends. Keep them in; just ensure test spans both
- "How do we test during holidays?" → Avoid running on holidays (Black Friday, Christmas) unless that's your target season. Otherwise, holiday effects confound the test
- "Can we use last year's data as control to speed up?" → Risky: user base, seasonality, product changes between years

## See also
- [[velocity-sample-size-per-day]]
- [[instrumentation-and-logging]]
- [[novelty-effects-and-long-term-impact]]

## Sources
See frontmatter `sources:`.
