---
id: 467fd41e-1815-4d4c-af5a-f0c46e9576e5
title: Randomization units (user, session, account)
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- experiment-design
- randomization-unit
- session-vs-user
- clustering
- statistical-power
aliases:
- unit of analysis
- randomization level
- clustering
sources:
- url: https://netflixtechblog.com/sequential-a-b-testing-keeps-the-world-streaming-netflix-part-1-continuous-data-cba6c7ed49df
  label: 'Netflix: Sequential A/B Testing'
- url: https://www.statsig.com/perspectives/ab-test-sample-size
  label: 'Statsig: A/B Test Sample Size'
cards:
- id: b1d298bb-4e6d-47f2-9701-77899f8e6965
  type: flip
  front: ''
  back: ''
- id: 3f3e6e00-92e4-4756-b212-e74339a41cf9
  type: flip
  front: ''
  back: ''
- id: 9e935d47-8470-47e6-9c84-eca63bd1d13e
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Selection bias
    correct: false
  - key: b
    text: Interference / spillover
    correct: false
  - key: c
    text: Survivorship bias
    correct: false
  - key: d
    text: Temporal confounding
    correct: false
- id: 8434eab6-c7ad-4f78-b2ec-9a9928bb1ecc
  type: flip
  front: ''
  back: ''
---

## Intuition

The randomization unit is the entity you assign to treatment or control. Choosing the wrong unit wastes power (sample size) or introduces bias (interference between treated and untreated).

## Detail

**User-level**: Randomize each user once, they stay in arm for duration. Pros: simple, matches most metrics (user revenue, retention). Cons: can't measure session/visit effects well; network effects cause interference.

**Session-level**: Randomize each visit independently. Pros: captures visit variance; useful for web (each session is fresh). Cons: same user may see both arms → confusing experience; need more samples (higher variance).

**Account-level**: Randomize at account/workspace level. Pros: clean for B2B; avoids user confusion. Cons: fewer units, less power.

**Matched pairs/cluster**: Group users by similar past behavior, randomize pairs. Pros: reduces variance (CUPED-like). Cons: complex implementation.

**Key decision**: 
- If metric is user-aggregate (lifetime value, retention) → user-level
- If metric is session-aggregate (CTR, conversion per visit) → session-level
- If interference exists (e.g., recommendation feed affects other users) → cluster-level

## Common gotchas / interview framings
- "If we randomize session-level, doesn't the same user see both?" → Yes, problem: they may compare and be confused. Good if test duration is short or user doesn't notice
- "Why does randomization unit matter for power?" → Intra-user correlation inflates variance. Randomizing sessions gives more samples but noisier (same user repeated → correlated)
- "Should we randomize at the platform level (e.g., iOS vs Android)?" → Only if the treatment applies to entire platform; risks confounding with device effects
- "What if we randomize users but measure sessions?" → Valid but noisy. Better to use user-level metrics or switch to session randomization

## See also
- [[interference-and-network-effects]]
- [[survivorship-bias]]
- [[randomization-and-control-groups]]

## Sources
See frontmatter `sources:`.
