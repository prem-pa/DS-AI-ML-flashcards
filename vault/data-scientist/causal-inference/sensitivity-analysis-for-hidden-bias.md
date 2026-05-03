---
id: 7d8b0c71-40f0-4ab3-88f4-1b18c93acc5b
title: Sensitivity analysis for hidden bias
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- unmeasured-confounding
- robustness
- Rosenbaum
- bias-bounds
- hidden-bias
aliases:
- sensitivity to unmeasured confounding
- bias bounds
- Rosenbaum bounds
- robustness check
sources:
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
- url: https://ftp.cs.ucla.edu/pub/stat_ser/r350.pdf
  label: 'Pearl: Causal Inference in Statistics Overview'
cards:
- id: fa5d7234-72f3-48d1-b468-a9a57fe06d66
  type: flip
  front: What is the goal of sensitivity analysis in causal inference?
  back: 'To quantify how robust causal conclusions are to violations of unconfoundedness. Rather than
    assume unmeasured confounding is zero, sensitivity analysis asks: ''How strong would an unmeasured
    confounder need to be to overturn the causal conclusions?'' A reassuring answer means high confidence;
    a weak bound means skepticism is warranted.'
- id: d719c88e-0c13-4648-9fb8-22d709872804
  type: flip
  front: What is an E-value and how do you interpret it?
  back: The E-value is the minimum relative risk (RR) an unmeasured confounder must have with both treatment
    and outcome to explain away the observed effect. E-value = 1 means no confounding needed (effect near
    null). E-value = 2 means the confounder must associate with both X and Y at RR ≥ 2. Higher E-values
    indicate stronger robustness.
- id: d23b8bc6-2e87-4e12-80d3-d8dd6798d5e8
  type: mcq
  front: 'A treatment effect estimate is RR = 1.8 (95% CI: 1.2–2.7). The E-value is 2.4. What does this
    mean?'
  back: An unmeasured confounder would need to have a relative risk ≥ 2.4 with both treatment and outcome
    to explain away the observed effect. This is moderately strong confounding; depending on the context,
    this may or may not be plausible.
  choices:
  - key: a
    text: The treatment effect is 2.4 times stronger than placebo
    correct: false
  - key: b
    text: Unmeasured confounding with RR ≥ 2.4 with both X and Y could explain away the effect
    correct: true
  - key: c
    text: The estimate is robust to unmeasured confounding
    correct: false
  - key: d
    text: The unmeasured confounder has RR = 2.4
    correct: false
- id: 41d579c8-9ae8-4056-aefd-83dff31752eb
  type: flip
  front: What is a Rosenbaum bound and when is it used?
  back: 'Rosenbaum bounds assess sensitivity to unmeasured confounding in matched studies. Given matched
    pairs and the magnitude of unmeasured confounding Γ, the bound tests whether the treatment effect
    remains significant. It answers: ''For matched pairs differing by Γ in odds of treatment, does the
    treatment effect estimate overturn?'' Used in matching-based studies.'
---

## Intuition

Sensitivity analysis quantifies how robust causal conclusions are to violations of unconfoundedness. Rather than assume unmeasured confounding is zero (unverifiable), we ask: "How much unmeasured confounding would be required to reverse the conclusions?" If a small amount suffices, we should be skeptical; if a large amount is required, we can be more confident.

## Detail

**Rosenbaum bounds** (Rosenbaum, 1987):
For matched observational studies, suppose matched pairs differ in treatment by Γ (relative odds ratio), modeling unmeasured confounding. The question: for what values of Γ does the treatment effect estimate become insignificant?

**Example**:
- Estimate: treatment reduces outcome by 10 units (p < 0.01)
- Sensitivity: results robust to Γ ≤ 1.5 (unmeasured confounder increases odds of treatment by ≤ 50% in matched pairs)

**E-value** (VanderWeele & Ding, 2017):
For observational studies with a point estimate and confidence interval, the E-value is the minimum strength of association (relative risk) an unmeasured confounder must have with both treatment and outcome to explain away the observed effect.

**Interpretation**:
- E-value = 1: no unmeasured confounding needed (estimate near null)
- E-value = 2: unmeasured confounder must have RR ≥ 2 with both X and Y to reverse the result
- Higher E-value → more robust to unmeasured confounding

**Other methods**:
- **Benchmark sensitivity analysis**: Model unmeasured confounding using observed confounders as benchmarks
- **Causal bounds**: Derive lower and upper bounds on the true causal effect under partial assumptions

## Common gotchas / interview framings

- "Sensitivity analysis proves the result is causal"—no, it quantifies robustness; high values are reassuring but not proof
- "We did sensitivity analysis; no unmeasured confounding"—wrong interpretation; analysis shows how much would overturn results
- Interview: "How would you assess robustness to unmeasured confounding?" or "Calculate/interpret an E-value."

## See also
- [[unconfoundedness-and-sutva-assumptions]]
- [[matching-and-propensity-scores]]
- [[difference-in-differences]]

## Sources
See frontmatter `sources:`.
