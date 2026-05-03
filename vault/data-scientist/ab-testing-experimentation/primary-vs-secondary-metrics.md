---
id: ebce9a5f-cd2e-4a72-b101-8bf872f14ff8
title: Primary vs secondary metrics
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- metrics
- hypothesis-testing
- effect-measurement
- multiple-testing
- business-impact
aliases:
- primary metric
- secondary metric
- metric hierarchy
- outcome metric
sources:
- url: https://www.statsig.com/perspectives/ab-test-sample-size
  label: 'Statsig: A/B Test Design'
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: A/B Testing Best Practices'
cards:
- id: 455d3e26-7613-4e4b-890e-7233b77de951
  type: flip
  front: ''
  back: ''
- id: f761b9f0-6dea-4976-883a-987c7716ea8c
  type: flip
  front: ''
  back: ''
- id: 79480148-d12e-4f94-8fe6-4409e0cb68f6
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: One metric is always significant by chance
    correct: false
  - key: b
    text: Multiple primary metrics inflate Type I error
    correct: false
  - key: c
    text: 'You need Bonferroni correction: 0.05/3 = 0.017 per metric'
    correct: false
  - key: d
    text: All of the above
    correct: false
- id: cead84dc-718e-45fc-b509-2788667398c3
  type: flip
  front: ''
  back: ''
---

## Intuition

Primary metric is the one outcome you care most about and will use to decide launch/no-launch. Secondary metrics provide context but don't override primary decision.

## Detail

**Primary metric**:
- **One metric** (sometimes 2 if closely related, e.g., CTR and conversion rate)
- **Business-aligned**: directly reflects business goal (revenue, retention, engagement)
- **Locked before test**: Can't change after seeing results (prevents p-hacking)
- **Drives decision**: If primary is significant, launch; if not, don't

**Secondary metrics**:
- **2-5 metrics** providing context
- **Check for side effects**: E.g., primary is engagement; secondary is latency (make sure we didn't slow down)
- **Exploratory**: interesting findings but don't override primary
- **Can be added mid-test**: Document as secondary, not primary

**Examples**:
- Feature to increase engagement (primary: DAU). Secondaries: retention, revenue, latency
- Feature to reduce latency (primary: median load time). Secondaries: p99 latency, engagement
- New search algorithm (primary: engagement). Secondaries: revenue, user satisfaction

**Multiple testing correction**:
- If testing 5 primary metrics, apply Bonferroni correction: alpha = 0.05/5 = 0.01
- Or pre-register one primary, secondaries are exploratory (no correction needed)

**Trade-off**:
- One primary = faster decision, clear governance
- Multiple primaries = richer picture, but complicates decisions (what if metrics disagree?)

## Common gotchas / interview framings
- "Metric A is significant, metric B is not. Which drives launch?" → Metric A if it's primary. Metric B is secondary context. Launch if primary is positive and no guardrails violated
- "Can we demote metric A from primary to secondary because it's not significant?" → No, that's p-hacking. Chose A before test; must live with result
- "Should we use revenue as primary for every test?" → No, revenue is slow to move. Use engagement (faster signal), but monitor revenue as secondary
- "We have 5 secondary metrics, none significant, but primary is significant. Launch?" → Yes. Secondaries are context, not blocking. Launched feature didn't harm secondary goals (on average), even if no clear benefit

## See also
- [[guardrail-metrics]]
- [[metric-construction-ratios-counts-timing]]
- [[effect-size-estimation-and-practical-significance]]

## Sources
See frontmatter `sources:`.
