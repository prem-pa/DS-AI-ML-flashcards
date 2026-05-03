---
id: c99a75fb-9bd1-4a7a-b77c-91d9608a227c
title: Learning curves
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- evaluation
- bias-variance
- diagnostics
- overfitting
aliases:
- training curves
- validation curves
- bias-variance diagnosis
sources:
- url: https://scikit-learn.org/stable/modules/learning_curve.html
  label: scikit-learn Learning Curves
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 808b6b5b-695d-4a53-9cad-3a3b90d019b6
  type: flip
  front: Learning curve construction. Train vs validation?
  back: 'For each $n$: train on $n$ samples (CV), eval training+validation. Training: goodness of fit.
    Validation: generalization. Gap=overfitting.'
- id: 0e604de3-b6b1-45e6-be21-13ef7d5abe88
  type: flip
  front: High bias signature + diagnosis?
  back: 'High training error + high validation error + small gap. Model too weak. Fix: increase complexity
    (add features, lower $\lambda$, deeper trees).'
- id: 6d3105d4-cc90-40e3-be63-78a19e3b519a
  type: flip
  front: High variance signature + diagnosis?
  back: 'Low training error + high validation error + large gap. Memorization. Fix: more data (closes
    gap), or reduce complexity.'
- id: b71dc158-9b9f-48bf-914a-4a4092b34087
  type: flip
  front: 'High variance: collect data or reduce complexity?'
  back: Both help. More data closes gap directly. Reducing complexity reduces overfitting but may introduce
    bias. Choose based on cost/time.
---

## Intuition
Plot training + validation error vs training set size. High bias: both high, small gap (complexity too low). High variance: low training, high validation, large gap (overfitting).

## Detail
**Construction:** For sizes $n=m_1,...,m_{max}$, train (CV), record train + validation error. **Interpretation:** High bias→increase complexity. High variance→more data or reduce complexity.

## See also
- [[bias-variance-tradeoff]]
- [[overfitting]]
- [[underfitting]]

## Sources
See frontmatter `sources:`.
