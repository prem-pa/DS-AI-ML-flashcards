---
id: 1a3a0682-b528-406a-a6b2-92d4581efd12
title: Unconfoundedness and SUTVA assumptions
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- identifying-assumptions
- unconfoundedness
- SUTVA
- causal-model
- no-interference
aliases:
- no unmeasured confounding
- conditional independence
- SUTVA
- no spillovers
sources:
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
- url: https://ftp.cs.ucla.edu/pub/stat_ser/r350.pdf
  label: 'Pearl: Causal Inference in Statistics Overview'
cards:
- id: 64b87d64-63ed-4e4b-ab32-3f87371d2abd
  type: flip
  front: What does unconfoundedness mean and why is it crucial for causal inference?
  back: 'Unconfoundedness means all variables that confound the treatment-outcome relationship are measured
    (no unmeasured confounders U). Formally: Y(1), Y(0) ⊥ X | Z. It is the key assumption allowing valid
    causal estimates from observational data via regression, matching, or weighting.'
- id: e3bcf229-721f-4f8e-9c83-fd2b3ea96517
  type: flip
  front: Can unconfoundedness be tested? How do we assess it?
  back: 'Unconfoundedness cannot be directly tested because unmeasured confounders are, by definition,
    unobserved. Assessment relies on: (1) subject-matter knowledge (have we measured the key confounders?),
    (2) sensitivity analysis (how robust are results to unmeasured confounding?), and (3) triangulation
    (do multiple methods agree?).'
- id: d46ae2b1-1969-44c0-89b1-881ae1a5c7c5
  type: mcq
  front: In a study of work-from-home's effect on productivity, employees self-select into WFH. Which
    assumption is most likely violated?
  back: Unconfoundedness. Employees who choose WFH may differ in motivation, which causally affects productivity.
    This unmeasured confounder biases estimates. Sensitivity analysis would quantify the required degree
    of confounding to reverse results.
  choices:
  - key: a
    text: Positivity
    correct: false
  - key: b
    text: Unconfoundedness (unmeasured motivation)
    correct: true
  - key: c
    text: SUTVA
    correct: false
  - key: d
    text: Consistency
    correct: false
- id: a5a11ab7-0a0c-4a80-b9b5-753296056fd9
  type: flip
  front: What is SUTVA and what does it require?
  back: 'Stable Unit Treatment Value Assumption: (1) no interference—one unit''s treatment does not affect
    another''s outcome, and (2) consistency—the outcome under treatment x is well-defined and unique across
    contexts. Violations occur in networked settings (interference) and complex/context-dependent treatments
    (inconsistency).'
---

## Intuition

Unconfoundedness (no unmeasured confounding) and SUTVA (Stable Unit Treatment Value Assumption) are the two core assumptions that allow us to identify causal effects from observational or experimental data. They are strong, often unverifiable, and rarely fully satisfied. Understanding their scope is essential for honest inference.

## Detail

**Unconfoundedness** (Rubin, 1974):
```
Y(1), Y(0) ⊥ X | Z
```
Treated and control units with the same observed covariates Z have the same potential outcome distribution. No unmeasured variable U confounds X and Y. Formally, all confounders are measured.

**Implications**:
- Allows regression adjustment, matching, or weighting on Z to estimate causal effects
- Violation (unmeasured U) → biased estimates; sensitivity analysis quantifies bias

**SUTVA** (Stable Unit Treatment Value Assumption) has two parts:
1. **No interference**: One unit's treatment does not affect another's outcome
   - Violated in social networks, cluster studies, peer effects
2. **Consistency**: Potential outcome Y(x) is well-defined and unique; no multiple "versions" of treatment
   - Violated if treatment is complex/heterogeneous or context-dependent

**Violations and remedies**:
- Unmeasured confounding → sensitivity analysis, triangulation
- Interference → use causal graph for interference, network methods
- Inconsistent treatment → define treatment precisely; consider mediation analysis

## Common gotchas / interview framings

- "We measured everything, so unconfoundedness holds"—no: unmeasured confounding is unknown by definition
- "Users don't interact, so SUTVA holds"—even spillovers in aggregate metrics (e.g., platform load)
- "Causal claims require unconfoundedness and SUTVA"—true; sensitivity analysis measures robustness
- Interview: "List causal assumptions and explain when you'd worry about violations."

## See also
- [[randomized-experiments-as-gold-standard]]
- [[matching-and-propensity-scores]]
- [[sensitivity-analysis-for-hidden-bias]]

## Sources
See frontmatter `sources:`.
