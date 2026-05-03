---
id: e5e45f4b-b1b5-48f5-b380-f89b4b9b78b9
title: Confounders, colliders, mediators
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- node-types
- d-separation
- conditioning
- bias
- selection-bias
aliases:
- confounding variables
- collider bias
- mediation paths
- blocking
sources:
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
- url: https://ftp.cs.ucla.edu/pub/stat_ser/r350.pdf
  label: 'Pearl: Causal Inference in Statistics Overview'
cards:
- id: 7d9c3465-6281-4ca4-bce9-78cceac61d97
  type: flip
  front: What is a confounder and how does it bias estimates?
  back: 'A variable that causally affects both treatment and outcome. It creates a backdoor path (non-causal
    association) between X and Y. Example: age affects both exercise and health. Unadjusted estimates
    confound treatment effect with age effect.'
- id: f43ac379-f920-484f-a597-d9a330faa4f6
  type: flip
  front: What is collider bias and why does conditioning on a collider create it?
  back: 'A collider is a variable caused by both X and Y. Conditioning on it opens a spurious path X -
    Collider - Y, inducing correlation between X and Y even if no causal effect exists. Example: college
    admission (collider of grades and test scores); conditioning on admission creates spurious negative
    correlation.'
- id: 98ba9cdb-2781-4c30-aef8-cdc680834d2a
  type: mcq
  front: You observe that correcting for a measured variable increases rather than decreases residual
    variance. Which is most likely?
  back: Conditioning on a collider opens spurious paths and can increase bias. If the variable is a true
    confounder, adjustment should reduce variance; worsening suggests collider bias.
  choices:
  - key: a
    text: The variable is a confounder
    correct: false
  - key: b
    text: The variable is a collider
    correct: true
  - key: c
    text: The variable is a mediator
    correct: false
  - key: d
    text: The variable is independent of X and Y
    correct: false
- id: 3b3c262a-7000-489d-9a03-c4f58b556cf3
  type: flip
  front: In mediation analysis, what is the natural indirect effect (NIE)?
  back: The portion of the total effect on Y transmitted through the mediator M. Total Effect = Direct
    Effect + Indirect Effect. Isolating NIE requires careful assumptions about treatment effect heterogeneity
    and no post-treatment confounding.
---

## Intuition

In a DAG, nodes play different roles depending on their position relative to treatment (X) and outcome (Y). These roles determine when we should condition on a variable:

- **Confounder**: Ancestor of both X and Y → creates spurious association
- **Collider**: Descendant of both X and Y → induces bias if conditioned on
- **Mediator**: X → M → Y → lies on causal path from X to Y

## Detail

**Confounders** (e.g., age in a treatment study):
- Create backdoor paths: X ← U → Y
- Adjustment (stratification or regression) blocks the path, reduces bias
- Failure to adjust → confounding bias

**Colliders** (e.g., Y is selected based on both X and U):
```
X → C ← U
```
- Conditioning on C *opens* the path X - C - U - Y, inducing spurious correlation
- Do NOT adjust for colliders (common pitfall in observational studies)
- Causes "collider bias" or "selection bias"

**Mediators** (e.g., X → mechanism → Y):
- Conditioning on mediators blocks the direct path, isolates natural direct effect
- Total effect = direct effect + indirect effect (through mediator)

## Common gotchas / interview framings

- "I controlled for age and results flipped sign"—age may be a collider or lie on a causal path
- "Why did sample selection hurt my estimate?"—likely conditioned on a collider
- Interview: "Draw and identify confounders, colliders, and mediators in [scenario]"

## See also
- [[dags-directed-acyclic-graphs-and-causal-assumptions]]
- [[backdoor-criterion]]
- [[sensitivity-analysis-for-hidden-bias]]

## Sources
See frontmatter `sources:`.
