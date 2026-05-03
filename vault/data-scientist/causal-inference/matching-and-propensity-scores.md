---
id: 529b8638-1998-4018-aadc-71dc4206b25f
title: Matching and propensity scores
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- matching
- propensity-score
- covariate-balance
- observational
- model-reduction
aliases:
- propensity score matching
- PSM
- covariate matching
- propensity weighting
- AIPW
sources:
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
- url: https://ftp.cs.ucla.edu/pub/stat_ser/r350.pdf
  label: 'Pearl: Causal Inference in Statistics Overview'
cards:
- id: 2e654a8a-3538-47fa-b7be-4f06a1657292
  type: flip
  front: What is the propensity score and what does it achieve in matching?
  back: The propensity score e(X) = P(X=1|Z) is the probability of treatment given observed confounders.
    Within levels of e(X), treated and control units are balanced on Z (by definition). Matching or weighting
    on the propensity score mimics randomization conditional on Z.
- id: 5dc7d150-544d-43a9-876f-1dc67340514e
  type: flip
  front: What is the difference between propensity score matching and inverse probability weighting?
  back: 'PSM: Select a matched sample of treated and control with similar propensity scores, then compare
    outcomes. IPW: Reweight the entire cohort so that treatment is independent of confounders in the weighted
    population. IPW uses all data; PSM may discard many units.'
- id: c12e0ca6-a227-4338-a621-e3a3068abb47
  type: mcq
  front: After propensity score matching, you check covariate balance and find age still differs between
    treated and control. What should you do?
  back: Tighten the caliper or re-specify the propensity model. Poor balance indicates the propensity
    model did not adequately balance confounders. Option (b) is also reasonable (doubly robust); (c) is
    the direct fix.
  choices:
  - key: a
    text: Ignore it; propensity matching handles it
    correct: false
  - key: b
    text: Adjust for age in the outcome regression
    correct: false
  - key: c
    text: Tighten the caliper (reduce tolerance) or use a different matching algorithm
    correct: true
  - key: d
    text: Conclude there is residual confounding
    correct: false
- id: 0de740f2-61b3-424c-8eae-16add3aba193
  type: flip
  front: What is a 'doubly robust' estimator and why is it useful?
  back: An estimator combining propensity-weighted outcome regression. It is doubly robust because it
    gives consistent estimates if either the propensity model OR the outcome model is correctly specified
    (but not necessarily both). Reduces sensitivity to model misspecification.
---

## Intuition

Matching and propensity scores are observational analogs to randomization. Instead of randomly assigning treatment, we pair or weight treated and control units so they are comparable on observed confounders. This reduces model dependence and can improve bias-variance tradeoff.

## Detail

**Propensity score**: The probability of treatment given observed confounders:
```
e(X) = P(X=1 | Z)
```
where Z are measured confounders.

**Key insight**: If unconfoundedness holds (all confounders measured), then within levels of e(X), treated and control are comparable on Z.

**Matching strategies**:
1. **Exact matching**: Match on exact values of Z (curse of dimensionality; rarely feasible)
2. **Propensity score matching (PSM)**: Match treated to control with similar e(X) within caliper (tolerance)
3. **Inverse probability weighting (IPW)**: Weight treated units by 1/e(X), controls by 1/(1−e(X)); reweights to a pseudo-population where X ⊥ Z
4. **Doubly robust**: Combine outcome regression + propensity weighting to reduce bias if either model misspecified

**Covariate balance**: Check that treated and control have similar distributions of Z after matching/weighting (key diagnostic).

## Common gotchas / interview framings

- "Propensity score matching controls for unmeasured confounding"—FALSE; only works if all confounders measured
- "Always do 1:1 matching"—may discard many units and increase variance; full cohort weighting often better
- "My regression adjustment and PSM gave different answers"—suggests model misspecification or overlap issues
- Interview: "Why might matching reduce variance compared to regression?" or "What is covariate balance and why check it?"

## See also
- [[randomized-experiments-as-gold-standard]]
- [[unconfoundedness-and-sutva-assumptions]]
- [[sensitivity-analysis-for-hidden-bias]]

## Sources
See frontmatter `sources:`.
