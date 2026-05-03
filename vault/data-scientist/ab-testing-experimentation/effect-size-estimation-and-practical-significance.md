---
id: 1839f1c1-d3fd-4429-8b5d-a74ab3dffd30
title: Effect size estimation and practical significance
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- effect-size
- practical-significance
- business-impact
- statistical-significance
- mde
aliases:
- effect size
- practical vs statistical significance
- business impact
- economic significance
sources:
- url: https://swenotes.com/2025/10/04/minimum-detectable-effect-mde-in-a-b-testing/
  label: 'Software Engineer''s Notes: MDE in A/B Testing'
- url: https://www.abtasty.com/blog/sample-size-calculation/
  label: 'AB Tasty: Sample Size and Effect Size'
cards:
- id: a3c8d326-34d7-44c7-a046-81b4f7b010a2
  type: flip
  front: ''
  back: ''
- id: 23a3bb7e-820d-4df2-a73c-16950d4bbae7
  type: flip
  front: ''
  back: ''
- id: ae97f50a-ecc3-47a0-9924-46743474c122
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Yes, effect is significant and exceeds MDE
    correct: false
  - key: b
    text: No, confidence interval includes 0.5%, below MDE
    correct: false
  - key: c
    text: Yes, p=0.01 is strong evidence
    correct: false
  - key: d
    text: Need more info about guardrails
    correct: false
- id: 35fd4152-e8fa-416f-a74f-d0e9aa3d6192
  type: flip
  front: ''
  back: ''
---

## Intuition

**Statistical significance** = "result is unlikely due to chance (p < 0.05)." **Practical significance** = "result is big enough to matter for business." A statistically significant effect might be too small to care about.

## Detail

**Statistical significance**:
- Depends on sample size
- Large sample → small effects become significant
- Example: 100k users, +0.1% CTR increase, p=0.02 (significant but tiny)

**Practical significance**:
- Does effect size matter for business?
- Example: +0.1% CTR = 100k × 0.001 = 100 more clicks/day
  - If 100 clicks = $1000 revenue, meaningful
  - If 100 clicks = $1 revenue, probably not
- Depends on context: for retention, 0.1pp is huge; for CTR, 0.1% is tiny

**Effect size**:
- **Absolute effect**: difference in metric (e.g., +2% CTR)
- **Relative effect**: % change (e.g., +5% relative to baseline)
- **Cohen's d**: standardized effect (effect size / std dev) — used in power calculations
- **Practical rule**: interpret effect size against MDE (minimum detectable effect)

**Minimum Detectable Effect (MDE)**:
- Smallest effect you care about detecting
- Set based on business impact: "What's the smallest change worth launching?"
- Example: "Only launch if conversion rate increases >1%"
- MDE drives sample size: smaller MDE → larger sample size needed

**When significant ≠ practical**:
- Large sample size, tiny effect size
- Example: 1M users, +0.05% engagement increase, p=0.02 → statistically significant but doesn't justify engineering effort

**When practical > significant**:
- Small sample size, large effect size but not yet p<0.05
- Example: 1k users, +5% conversion, p=0.08 → not yet statistically significant but promising; run longer

## Common gotchas / interview framings
- "Effect is not significant. So no real effect?" → Inconclusive, not proven null. If underpowered, real effect might exist
- "We should only launch when effect is >2% lift and p<0.05." → Good, tie practical (2%) to statistical (p<0.05) thresholds. Prevents both false positives and false negatives
- "Why do we set MDE upfront? Shouldn't we wait and see?" → MDE determines sample size. If you wait and see, you're p-hacking (biased effect size). Set MDE based on business impact, lock it in
- "Small effect size but statistically significant. Launch?" → Depends on business ROI. If cost to launch < benefit of 0.1% lift, yes. If cost is high, maybe not

## See also
- [[primary-vs-secondary-metrics]]
- [[confidence-intervals-vs-bayesian-posterior]]

## Sources
See frontmatter `sources:`.
