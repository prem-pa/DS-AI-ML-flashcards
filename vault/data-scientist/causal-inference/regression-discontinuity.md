---
id: 481d4ff9-c9a1-463a-85dc-e387af6a4b36
title: Regression discontinuity
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- RDD
- sharp-discontinuity
- fuzzy-discontinuity
- cutoff
- quasi-experimental
aliases:
- regression discontinuity design
- RDD
- sharp RDD
- fuzzy RDD
- cutoff design
sources:
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
- url: https://www.mostlyharmlesseconometrics.com/
  label: 'Mostly Harmless Econometrics: Angrist & Pischke'
cards:
- id: dd5be943-6110-4948-8006-20d2c9cafed1
  type: flip
  front: What is the key identifying assumption in regression discontinuity?
  back: 'Local randomization: units just above and below the cutoff on the running variable are similar
    in all respects except treatment assignment. This makes them a valid comparison, as if they were randomized.
    The discontinuity in outcomes at the cutoff is causal.'
- id: d4af7fe2-b75e-4ac2-97c9-731c25062fc4
  type: flip
  front: What is the difference between sharp and fuzzy RDD?
  back: 'Sharp RDD: treatment is deterministic at the cutoff (e.g., test score ≥ 80 → always treated).
    Fuzzy RDD: eligibility changes at the cutoff, but compliance is imperfect (e.g., some eligible don''t
    participate). Fuzzy RDD is estimated using IV methods (cutoff as the instrument).'
- id: e80737c3-abe7-46a6-905b-efa372b77258
  type: mcq
  front: In an RDD, the McCrary test shows a large jump in density of the running variable at the cutoff.
    What does this suggest?
  back: A jump in density suggests units manipulated their running variable to reach a desired treatment
    status (e.g., bunching just above the cutoff). This violates the local randomization assumption and
    invalidates the RDD.
  choices:
  - key: a
    text: The treatment effect is large
    correct: false
  - key: b
    text: Possible manipulation of the running variable; RDD assumptions violated
    correct: true
  - key: c
    text: The sample size is adequate
    correct: false
  - key: d
    text: The bandwidth is too small
    correct: false
- id: 75f8f356-c2be-45c7-a7ec-ae37e6c64bf4
  type: flip
  front: Why does RDD estimate a local average treatment effect (LATE) rather than the ATE?
  back: RDD identifies the effect only at the cutoff where there is a discontinuity. Units far from the
    cutoff are not directly compared. The effect at the cutoff may differ from the population average
    (e.g., marginal vs. inframarginal units).
---

## Intuition

Regression discontinuity design (RDD) exploits a sharp cutoff in eligibility for treatment. If assignment is determined by a 'running variable' crossing a threshold (e.g., test score ≥ 80 → enter program), individuals just above and below the cutoff are similar in all respects except treatment. The discontinuity in outcomes at the cutoff estimates the causal effect.

## Detail

**Setup**:
- Running variable R (e.g., test score, income, age)
- Cutoff c: if R ≥ c, receive treatment; else control
- Outcome Y

**Sharp RDD**: Treatment is deterministic at the cutoff.
```
X = 1 if R ≥ c; 0 otherwise
```

**Fuzzy RDD**: Treatment eligibility changes at the cutoff, but not deterministically (e.g., 90% treated above cutoff, 10% below).
```
P(X = 1 | R) has a jump at c
```

**Identification**: Units just above and below c have similar pre-treatment characteristics (local randomization). The jump in E[Y|R] at c estimates the local average treatment effect (LATE) at the cutoff.

**Estimation**:
- Local polynomial regression (fit polynomial on each side of cutoff; compare values at c)
- Nonparametric methods (local linear, local polynomial)
- Bandwidth selection crucial (too wide → bias; too narrow → high variance)

**Validity conditions**:
- Running variable manipulable (if units can game the system, fails McCrary test)
- No other treatment changes at the cutoff
- Effect only local to the cutoff (may not generalize)

## Common gotchas / interview framings

- "RDD gives the ATE"—no, only LATE at the cutoff; often not representative
- "Manipulation around the threshold"—test with density histogram (McCrary 2008)
- "Bandwidth selection"—sensitivity check with alternative bandwidths
- Interview: "Design an RDD for [scenario]" or "How do you test the validity of an RDD?"

## See also
- [[randomized-experiments-as-gold-standard]]
- [[instrumental-variables-iv]]
- [[sensitivity-analysis-for-hidden-bias]]

## Sources
See frontmatter `sources:`.
