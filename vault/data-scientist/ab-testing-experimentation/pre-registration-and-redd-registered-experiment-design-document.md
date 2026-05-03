---
id: 75683f9d-9764-4985-8738-de49815217b3
title: Pre-registration and REDD (Registered Experiment Design Document)
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- p-hacking
- harking
- statistical-validity
- experiment-governance
- transparency
aliases:
- pre-registration
- registered design
- experiment plan
- protocol registration
sources:
- url: https://netflixtechblog.com/experimentation-is-a-major-focus-of-data-science-across-netflix-f67923f8e985
  label: 'Netflix: Experimentation as Core Data Science'
- url: https://research.netflix.com/research-area/experimentation-and-causal-inference
  label: 'Netflix Research: Experimentation Infrastructure'
cards:
- id: 423945bc-d6dc-45ec-9914-72f2d2397be0
  type: flip
  front: ''
  back: ''
- id: af54debc-bc2d-41f1-ad70-7824e4e8e5e6
  type: flip
  front: ''
  back: ''
- id: 5653f94f-546f-4f49-9904-16c9122e4642
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Yes, five comparisons is reasonable
    correct: false
  - key: b
    text: No, 5 metrics = multiple comparisons problem (inflate error rate)
    correct: false
  - key: c
    text: Only if you pre-registered all 5 as primary
    correct: false
  - key: d
    text: Only if you Bonferroni-correct (alpha = 0.05/5)
    correct: false
- id: c163cf59-8592-4bc4-a6a1-bc56a3aa7813
  type: flip
  front: ''
  back: ''
---

## Intuition

Pre-registration means you write down your analysis plan *before* running the experiment. This prevents **p-hacking** (testing many hypotheses until one is significant) and **HARKing** (Hypothesizing After Results are Known). It locks you into one primary metric and one stopping rule.

## Detail

**REDD** (Registered Experiment Design Document) includes:
1. **Primary metric**: The one outcome you care most about
2. **Hypothesis**: Expected direction and size of effect
3. **Sample size & duration**: How long until decision
4. **Stopping rule**: Fixed horizon (2 weeks), sequential (peek penalties), or Bayesian
5. **Subgroup analyses**: If pre-planned, not p-hacking
6. **Sanity checks**: Did randomization work? Did treatment reach users?

**Benefits**:
- Prevents p-hacking: lock in alpha (0.05) and primary metric upfront
- Increases trust: stakeholders see the plan was fair, not cherry-picked
- Speeds decisions: no debate over which metric to use

**Rigorous firms** (Netflix, Microsoft, Doordash) require REDDs before running tests. Looser shops skip it and face p-hacking risk.

## Common gotchas / interview framings
- "But what if we discover a new interesting result in the data?" → Document it as exploratory, not primary. Replicate in future test.
- "Can we add metrics mid-experiment?" → Yes, as secondary. But primary must be locked. Adding primary metrics = p-hacking
- "Do we need REDD for small internal tests?" → No, not worth bureaucracy. But for customer-facing launches, yes.
- "What if the REDD prediction is wrong?" → OK. Pre-registration is about preventing p-hacking, not perfect forecasting

## See also
- [[sequential-testing-and-peek-penalties]]
- [[confidence-intervals-vs-bayesian-posterior]]
- [[effect-size-estimation-and-practical-significance]]

## Sources
See frontmatter `sources:`.
