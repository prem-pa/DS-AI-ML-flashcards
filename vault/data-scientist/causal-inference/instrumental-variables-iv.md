---
id: 8ede242b-65c4-4dde-945c-99c22e804d99
title: Instrumental variables (IV)
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- endogeneity
- IV
- two-stage-least-squares
- exclusion-restriction
- relevance
aliases:
- instrumental variable
- 2SLS
- TSLS
- endogenous regressor
sources:
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
- url: https://www.mostlyharmlesseconometrics.com/
  label: 'Mostly Harmless Econometrics: Angrist & Pischke'
cards:
- id: ebc19c1c-d708-4e7b-8c8e-c81d222b330d
  type: flip
  front: What is an instrumental variable and what problem does it solve?
  back: 'An IV is a variable Z that causally affects treatment X but does not directly affect the outcome
    Y (except through X). It solves endogeneity: when treatment X is confounded by unmeasured variables
    U, IV exploits exogenous variation in X to estimate the causal effect.'
- id: c13cd3ea-7e3a-4e3b-ab08-d5c4001f9a7d
  type: flip
  front: What is the difference between the average treatment effect (ATE) and local average treatment
    effect (LATE) in IV?
  back: ATE is the effect for the average unit in the population. LATE (estimated by IV) is the effect
    for 'compliers'—units whose treatment is causally affected by the instrument. IV estimates LATE, not
    ATE, unless treatment effects are constant across units.
- id: 5a460c25-58b0-4769-93fd-8a72c2bee64d
  type: mcq
  front: In 2SLS, the F-statistic in stage 1 is 3 (instrument weak). What is the implication?
  back: 'Weak instruments lead to finite-sample bias and inflated standard errors. Rule of thumb: F >
    10 for one instrument. Use IV-robust inference (Anderson-Rubin) if weak.'
  choices:
  - key: a
    text: The instrument is valid
    correct: false
  - key: b
    text: Weak instrument bias occurs; standard errors are unreliable
    correct: true
  - key: c
    text: The exclusion restriction is violated
    correct: false
  - key: d
    text: The second stage is valid despite weak first stage
    correct: false
- id: 0288d78a-2acf-43b1-9db8-6d2d6c811ec2
  type: flip
  front: What is the exclusion restriction and why is it untestable?
  back: The exclusion restriction requires the instrument Z to affect Y only through X, not directly.
    It is untestable because we cannot observe the direct effect of Z on Y conditional on X (it is a causal
    assumption). Validity depends on domain knowledge and careful reasoning about mechanisms.
---

## Intuition

Instrumental variables (IV) identify causal effects in the presence of unmeasured confounding and endogeneity. An IV is a variable that causally affects treatment but does not directly affect the outcome (except through treatment). IV extracts the exogenous variation in treatment to estimate the causal effect.

## Detail

**Problem**: X is endogenous—confounded by unmeasured U:
```
U → X → Y
↓ _______↑
```
Regression of Y on X is biased.

**Solution**: Find an instrument Z:
```
Z → X → Y
    ↑ _________↑
    U (does not affect Y directly)
```

**IV assumptions**:
1. **Relevance**: Z causally affects X; Cov(Z, X) ≠ 0
2. **Exclusion restriction**: Z affects Y only through X; no direct effect Z → Y
3. **Exogeneity**: Z is unconfounded (independent of U)

**Estimation** (two-stage least squares, 2SLS):
- Stage 1: Regress X on Z; get predicted X̂
- Stage 2: Regress Y on X̂
- The coefficient on X̂ estimates the local average treatment effect (LATE)

**Interpretation**:
- IV estimates LATE: the effect among units whose treatment is affected by Z (compliers)
- Assumes monotonicity: Z always pushes in the same direction for all units
- Requires constant treatment effect (or estimates LATE, not ATE)

## Common gotchas / interview framings

- "My instrument is weak"—F-stat < 10 in stage 1; use IV-robust inference
- "Exclusion restriction is untestable"—domain knowledge required; sensitivity analysis helpful
- "IV estimates different quantities than regression"—yes, LATE vs ATE; which is your target?
- Interview: "Propose an instrument for [scenario]" or "How would you test the exclusion restriction?"

## See also
- [[randomized-experiments-as-gold-standard]]
- [[unconfoundedness-and-sutva-assumptions]]
- [[sensitivity-analysis-for-hidden-bias]]

## Sources
See frontmatter `sources:`.
