---
id: 4a5ace44-278d-44c8-a64d-6176e14fc06b
title: Mediation analysis
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- mediation
- direct-indirect-effects
- mechanism
- path-analysis
- decomposition
aliases:
- mediation
- indirect effect
- natural indirect effect
- path analysis
sources:
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
- url: https://ftp.cs.ucla.edu/pub/stat_ser/r350.pdf
  label: 'Pearl: Causal Inference in Statistics Overview'
cards:
- id: d2de2d44-4bd8-4636-82ea-48dfbfe25c41
  type: flip
  front: What is the natural indirect effect (NIE) and how does it differ from the total effect?
  back: The natural indirect effect is the portion of the total effect transmitted through the mediator
    M (X → M → Y). Total Effect = Direct Effect (X → Y direct) + Indirect Effect (X → M → Y). NIE isolates
    the mechanism-specific pathway.
- id: 195ae494-990b-48d3-a112-63536cf81cf0
  type: flip
  front: What are the key assumptions for identifying NDE and NIE?
  back: '1. No unmeasured confounding of X-Y, 2. No unmeasured confounding of M-Y, 3. No confounding of
    X-M, and critically, 4. No post-treatment confounding: no variable affected by X confounds the M-Y
    relationship. Assumption 4 is often violated in practice.'
- id: 9b106f09-6f3c-4992-89bc-5608c8d4203d
  type: mcq
  front: In a study examining whether education (X) improves income (Y) via social network (M), the relationship
    between M and Y is confounded by job-search effort (affected by education). What is the consequence?
  back: Job-search effort is a post-treatment confounder (affected by X, confounds M-Y). This violates
    the core mediation assumption and biases the NIE estimate. Sensitivity analysis or additional data
    on effort is needed.
  choices:
  - key: a
    text: The NDE is overestimated
    correct: false
  - key: b
    text: The NIE is biased due to post-treatment confounding
    correct: true
  - key: c
    text: Mediation analysis is still valid
    correct: false
  - key: d
    text: The total effect is zero
    correct: false
- id: f9486076-54d3-4dcc-b6d5-0786bcd26564
  type: flip
  front: Why is the Baron-Kenny approach to mediation (regress Y on X, M, then check if coefficient shrinks)
    problematic?
  back: It assumes no unmeasured confounding of M-Y and no post-treatment confounding, which are often
    violated. The regression approach does not account for treatment effect heterogeneity or confounding
    structures. Modern causal mediation uses g-formula or IPW with explicit assumptions.
---

## Intuition

Mediation analysis decomposes the causal effect of X on Y into two pathways: (1) the direct effect of X on Y not through the mediator M, and (2) the indirect effect of X on Y through M. Understanding mechanisms is valuable for both science and intervention design (if you want to change Y, do you target X or M?).

## Detail

**Effect decomposition**:
```
Total Effect (TE) = Direct Effect (DE) + Indirect Effect (IE)
  E[Y(1)] - E[Y(0)] = E[Y(1) - Y(1,M(0))] + E[Y(1,M(1)) - Y(0)]
```
where M(x) is the mediator under treatment x, and Y(x, m) is the outcome under treatment x and mediator level m.

**Natural Direct Effect (NDE)**: Effect of X on Y if the mediator were held at the level it would take under control (x=0).
**Natural Indirect Effect (NIE)**: Effect of X on Y transmitted through changes in the mediator.

**Assumptions** (strong):
1. No unmeasured confounding of X-Y
2. No unmeasured confounding of M-Y
3. No confounding of X-M relationship
4. No post-treatment confounding (no variable affected by X that confounds M-Y)

**Typical (flawed) approach** (Baron & Kenny, 1986):
- Regress Y on X (total effect)
- Regress M on X
- Regress Y on X and M ("controlled" for M)
- If effect shrinks, conclude mediation

**Problems with this**: Does not account for confounding of M-Y, treatment effect heterogeneity, or post-treatment confounding.

**Modern approach**:
- Use causal assumptions (DAG) to identify NDE/NIE
- Conduct sensitivity analysis for post-treatment confounding
- Estimate using g-formula, IPW, or doubly robust methods

## Common gotchas / interview framings

- "Mediation analysis tests if the effect is causal"—no, all methods assume X → Y is causal
- "Controlling for M gives the direct effect"—incorrect without additional assumptions (especially no post-treatment confounding)
- "Add more mediators and total effect sums to 100%"—not true if mediators are correlated (overlap)
- Interview: "What are the assumptions for mediation analysis?" or "Design a mediation study for [scenario]."

## See also
- [[dags-directed-acyclic-graphs-and-causal-assumptions]]
- [[confounders-colliders-mediators]]
- [[frontdoor-criterion]]

## Sources
See frontmatter `sources:`.
