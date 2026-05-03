---
id: 3575d236-4bcc-4cdc-9507-833d016b096f
title: Randomized experiments as gold standard
track: data-scientist
topic: causal-inference
difficulty: 3
tags:
- RCT
- randomization
- gold-standard
- unconfoundedness
- experimental-design
aliases:
- randomized controlled trials
- RCT
- A/B testing
- experimental gold standard
sources:
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
- url: https://ftp.cs.ucla.edu/pub/stat_ser/r350.pdf
  label: 'Pearl: Causal Inference in Statistics Overview'
cards:
- id: 441589f1-8482-4d2d-97fe-271ab18e2c5d
  type: flip
  front: Why does random assignment eliminate confounding?
  back: Randomization makes treatment X independent of all pre-treatment variables (measured and unmeasured).
    In the causal DAG, X has no parents (no edges pointing into X), so no backdoor paths exist. The only
    association between X and Y is causal.
- id: f0fa01bb-ac87-4007-962f-638c23b6cb28
  type: flip
  front: What is the difference between intention-to-treat (ITT) and per-protocol (PP) estimates in an
    RCT?
  back: 'ITT: analyze subjects as randomized, regardless of adherence. Per-protocol: analyze only those
    who complied with assigned treatment. ITT is unbiased for the intent-to-treat effect but may underestimate
    efficacy if compliance is low. PP can be biased if non-compliance is correlated with outcomes.'
- id: a1f219d4-ac42-4657-9ff1-de77e059a544
  type: mcq
  front: An RCT with 5% of participants dropping out of one arm. Which estimate is most reliable?
  back: ITT is more reliable because it preserves the randomization. Non-random dropout can bias PP estimates.
    ITT estimates the effect in the population randomized, even if some don't comply.
  choices:
  - key: a
    text: Intention-to-treat (ITT), because randomization is maintained
    correct: true
  - key: b
    text: Per-protocol (PP), because it reflects real outcomes
    correct: false
  - key: c
    text: Both are equally valid
    correct: false
  - key: d
    text: Neither; dropout introduces selection bias
    correct: false
- id: 5549495d-8030-4310-85c9-93816d0ef191
  type: flip
  front: What does SUTVA stand for and why does it matter for RCTs?
  back: 'Stable Unit Treatment Value Assumption: (1) no interference between units (treatment of one doesn''t
    affect another''s outcome), and (2) consistent treatment (one version of treatment). Violations occur
    in social networks or clustered experiments; violations require design adjustments.'
---

## Intuition

Randomization is the 'gold standard' for causal inference because it breaks all backdoor confounding paths. By randomly assigning treatment, we ensure treatment is independent of all pre-treatment variables—measured and unmeasured.

## Detail

**Why randomization works**:
1. Random assignment severs all causal edges *into* treatment (X ← U becomes independent)
2. In the resulting DAG, X has no parents; no backdoor paths from X to Y
3. The association between X and Y is entirely causal (unconfounded)

**RCT assumptions** (core):
- **Random assignment**: P(X=1|U)=P(X=1) for all U (treatment independent of confounders)
- **SUTVA**: No interference between units, consistent treatment (covered separately)
- **Positivity**: All units have nonzero probability of receiving each treatment arm

**Causal estimate**: Simple comparison of means:
```
ATE = E[Y|X=1] - E[Y|X=0]
```
No adjustment for confounders needed (though variance can be reduced with covariate adjustment).

**Limitations in practice**:
- High cost, infeasible for many questions (e.g., long-term health effects)
- External validity: RCT populations may not represent target population
- Compliance/adherence issues (intention-to-treat vs. per-protocol)
- Ethical concerns (withholding beneficial treatment)

## Common gotchas / interview framings

- "RCT gives the causal effect"—only under SUTVA and full compliance; departures require careful analysis
- "We ran an A/B test, so no confounding"—true for assignment, but interpretation requires SUTVA
- Interview: "What assumptions does an RCT make?" or "When would you prefer observational to RCT?"

## See also
- [[unconfoundedness-and-sutva-assumptions]]
- [[matching-and-propensity-scores]]
- [[sensitivity-analysis-for-hidden-bias]]

## Sources
See frontmatter `sources:`.
