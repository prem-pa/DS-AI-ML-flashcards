---
id: 4495e4e4-76b4-4aaf-b0da-39303f49a0e6
title: Difference-in-differences
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- DiD
- panel-data
- time-fixed-effects
- quasi-experimental
- parallel-trends
aliases:
- difference-in-difference
- DiD
- panel regression
- parallel trends
sources:
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
- url: https://bookdown.org/ccolonescu/ad4/differences-in-differences.html
  label: 'Econometric Methods for Causal Inference: Differences-in-Differences'
cards:
- id: c608c32d-44a4-4bef-8438-cdb82b9953fd
  type: flip
  front: What is the parallel trends assumption in DiD and why is it crucial?
  back: Absent treatment, the treated and control groups would have followed the same trend in outcomes
    over time. If the assumption holds, the divergence in post-treatment trends is causal. If pre-treatment
    trends differ, the control group is not a valid counterfactual.
- id: 8ae44e5c-befe-417f-be9c-cafbc7c605ce
  type: flip
  front: How does DiD eliminate time-invariant confounders?
  back: DiD estimates the difference in *changes* (ΔY_treated - ΔY_control), not levels. Any constant
    confounder (e.g., permanent group difference) cancels out in the difference. Only time-varying confounders
    that differ between groups bias the estimate.
- id: 2195f905-c2f9-4f3a-a0f5-6ed9827a17ba
  type: mcq
  front: In a DiD setup, treated and control groups have different baseline outcomes. Does this invalidate
    the estimate?
  back: No. DiD estimates the difference in trends, not levels. A constant level difference is a time-invariant
    confounder that cancels out in the double difference.
  choices:
  - key: a
    text: Yes, because the groups are not comparable
    correct: false
  - key: b
    text: No, because DiD eliminates level differences
    correct: true
  - key: c
    text: Only if the difference is growing over time
    correct: false
  - key: d
    text: It depends on the cause of the difference
    correct: false
- id: 57d3d8f2-c606-4ee9-9c97-e79b1a7dfcc3
  type: flip
  front: What is 'staggered' or 'cascading' treatment adoption and how does it affect DiD?
  back: Treatment is rolled out to different groups at different times (e.g., policy adopted by some states
    before others). Standard DiD can be biased; newer methods (e.g., Goodman-Bacon, Callaway-Sant'Anna)
    account for multiple treatment times and heterogeneous treatment effects.
---

## Intuition

Difference-in-differences (DiD) compares the *change* in outcomes over time between a treated group and a control group. If both groups follow parallel trends absent treatment, the difference in trends isolates the causal effect. DiD eliminates time-invariant confounders (e.g., permanent differences between groups).

## Detail

**Setup**: Panel data with two time periods (pre/post) and two groups (treated/control).

```
Estimate = (Y_treated_post - Y_treated_pre) - (Y_control_post - Y_control_pre)
          = ΔY_treated - ΔY_control
```

**Core assumption: Parallel trends**
Absent treatment, the treated group's outcome would have followed the same trend as the control group. Violating this (e.g., treated group was improving faster even without treatment) biases the estimate.

**Model** (for multiple groups/periods):
```
Y_it = β_0 + β_t × Time + β_g × Group + β_{DiD} × (Time × Group) + ε_it
```
where β_{DiD} is the treatment effect.

**Advantages**:
- Removes time-invariant confounders (group-level heterogeneity)
- Requires no parametric assumptions on outcome model
- Can extend to staggered treatment adoption

**Limitations**:
- Requires parallel trends, often unverifiable (test with pre-period trends)
- Treatment effects may vary over time; assumes constant effect
- Time-varying confounders violate assumptions

## Common gotchas / interview framings

- "Our groups have different levels but same trends"—DiD handles level differences; trends are key
- "Treatment rolled out to some groups earlier"—use staggered DiD (heterogeneous treatment timing)
- "Parallel trends test: compare pre-period slopes"—weak test if treatment effects appear immediately
- Interview: "How would you check the parallel trends assumption?" or "Why is DiD powerful for observational data?"

## See also
- [[randomized-experiments-as-gold-standard]]
- [[unconfoundedness-and-sutva-assumptions]]
- [[regression-discontinuity]]

## Sources
See frontmatter `sources:`.
