---
id: 033a700f-2112-43ad-bbdb-cb86823e5851
title: Simpson's paradox
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- simpson-paradox
- confounding
- stratification
- aggregation-bias
- subgroup-analysis
aliases:
- reversal paradox
- confounding bias
- aggregation bias
sources:
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: Handling Simpson''s Paradox in Experiments'
- url: https://research.netflix.com/research-area/experimentation-and-causal-inference
  label: 'Netflix: Experimentation Research'
cards:
- id: cacc6078-fe4c-4e9f-ba5b-4cfb249ef5d3
  type: flip
  front: ''
  back: ''
- id: 3353ba06-2c5a-4dda-9b87-c075a0a62333
  type: flip
  front: ''
  back: ''
- id: 8a601789-9f27-44d6-95e2-c2b0797f7b48
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: User tenure is a confounder
    correct: false
  - key: b
    text: Treatment attracts new users (composition change)
    correct: false
  - key: c
    text: Stratum effects contradict aggregate (Simpson's paradox)
    correct: false
  - key: d
    text: All of the above
    correct: false
- id: 9644ca8f-6e44-4c64-bbf7-c9c4204780cd
  type: flip
  front: ''
  back: ''
---

## Intuition

**Simpson's paradox**: An aggregated effect can reverse when stratified (broken into subgroups). Overall, treatment looks good, but in every subgroup, it's bad (or vice versa).

## Detail

**Example (classical)**:
- **Aggregate**: Treatment +5% (win)
- **Stratum 1** (high-value users): Treatment -3% (lose)
- **Stratum 2** (low-value users): Treatment -2% (lose)
- **Paradox**: Aggregate is positive, but every subgroup is negative

**Why does this happen?**
Unbalanced group sizes confound the overall result.
- Treatment has more low-value users than control
- Low-value users have lower baseline metrics
- Treatment pulls aggregate down, but weighted average still appears positive due to imbalance

**Mechanism**:
1. Treatment affects group composition (e.g., treatment attracts low-value users)
2. Group composition changes the baseline metric
3. Aggregated metric mixes true effect + composition effect

**Examples in A/B tests**:
- **Geographic**: Aggregate shows revenue +2%, but every region shows -1%. Why? Treatment is rolled out in low-revenue regions first (geographic confound)
- **Device**: Aggregate shows engagement +1%, but desktop -2%, mobile -1%. Why? Mobile users increase in treatment (composition change)
- **Cohort**: Aggregate shows retention +1%, but every cohort-month shows flat. Why? Treatment attracts different cohorts (timing confound)

**Solutions**:
1. **Stratified analysis**: Break down by subgroup, report stratum-specific effects
2. **ANCOVA/adjustment**: Adjust for group membership (e.g., covariate for device type)
3. **Check balance**: Ensure treatment and control have same distribution of subgroups. If not, that's a confound
4. **Randomize at subgroup level**: Randomize within strata, balance composition

## Common gotchas / interview framings
- "Aggregate effect is positive, but desktop is negative. Do we launch?" → Investigate. If desktop is real confounder (imbalance), report stratum-specific effects. If it's noise, report aggregate. Need to understand *why* desktop differs
- "Should we always stratify by subgroup?" → Only if you suspect confounding. Unnecessary stratification loses power (multiple comparisons). Stratify only on pre-specified confounders
- "Our A/A test passed (no difference between control arms), so we don't have confounds." → Wrong. A/A test checks randomization, not confounds. Confounds appear when aggregating across subgroups
- "Stratum-specific effects contradict aggregate. Which is true?" → Both are true; depends on perspective. Aggregate is causal for overall population; stratum-specific is causal for subgroup. Report both

## See also
- [[survivorship-bias]]
- [[interference-and-network-effects]]

## Sources
See frontmatter `sources:`.
