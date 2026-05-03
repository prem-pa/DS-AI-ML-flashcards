---
id: 29babde3-4f1c-48b9-95c3-0989c1dce2c5
title: Hyperparameter tuning (grid search, random search, Bayesian optimization)
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- hyperparameter-tuning
- model-selection
- optimization
- search-strategy
aliases:
- grid search
- random search
- Bayesian optimization
sources:
- url: https://scikit-learn.org/stable/model_selection.html
  label: scikit-learn Model Selection
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: cf865a24-59a6-4067-ad59-0ed5fcc4d847
  type: flip
  front: Grid vs random search. When random preferred?
  back: 'Grid: exhaustive, comprehensive. Cost exponential in $d$. Random: sample fixed number, often
    beats grid on high-dim ($d$ large). Fewer evaluations needed.'
- id: 10801027-c38d-432e-9d03-b168ef5e0f2f
  type: flip
  front: Bayesian optimization. Sample efficiency?
  back: 'Fit surrogate (Gaussian process) to learn landscape. Iteratively select promising configs (balance
    exploration/exploitation). Sample-efficient: few evaluations vs grid/random.'
- id: 85997251-1b39-4dc5-b5a6-a0c31c267b76
  type: flip
  front: Nested CV hyperparameter tuning?
  back: 'Outer CV: final error estimate. Inner CV: tune hyperparameters on training of outer. Prevents
    data leakage. Honest eval.'
- id: 19e7a252-310b-4500-8afa-fb882bc40c19
  type: flip
  front: Log-spaced ranges. Why important?
  back: For learning_rate, regularization, efficient coverage of magnitudes. E.g., [0.0001, 0.001, 0.01,
    0.1, 1] vs linear.
---

## Intuition
Tune hyperparameters by searching. Grid: exhaustive. Random: sample fixed number (often beats grid on high-dim). Bayesian: learn from evaluations, sample-efficient.

## Detail
**Grid:** Cartesian product of ranges. $O(m_1 \cdot m_2 \cdot ... \cdot k)$ fits. **Random:** Sample fixed configs (e.g., 100). Better on high-dim. **Bayesian:** Surrogate (GP), iteratively select promising configs.

## See also
- [[cross-validation]]
- [[parameter-vs-hyperparameter]]
- [[sample-efficiency]]

## Sources
See frontmatter `sources:`.
