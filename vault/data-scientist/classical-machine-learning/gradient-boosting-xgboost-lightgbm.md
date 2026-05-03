---
id: c292e4f8-08ac-4075-9f6a-3bcb367f35e8
title: Gradient Boosting (XGBoost, LightGBM)
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- ensemble-methods
- boosting
- gradient-descent
- tabular-data
aliases:
- sequential ensemble
- additive model
- boosted trees
sources:
- url: https://scikit-learn.org/stable/modules/ensemble.html#gradient-boosting
  label: scikit-learn Gradient Boosting
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 3339e266-e297-4038-91b1-dbab5a170c27
  type: flip
  front: Gradient boosting sequential idea?
  back: '$F_0=\bar{y}$. Step $m$: fit $h_m$ to negative gradient. $F_m=F_{m-1}+\eta h_m$. Repeat until
    convergence.'
- id: 77235593-eefb-4264-b391-7f5058453419
  type: flip
  front: 'Squared loss: what tree fit?'
  back: Gradient=$-2(y-\hat{y})$. Fit to residuals $y-F_{m-1}(x)$.
- id: b7524f68-9f4c-40e2-a034-332d689ee866
  type: flip
  front: Early stopping in boosting?
  back: Monitor validation loss; stop if no improvement $N$ rounds. Prevents overfitting.
- id: 032e85b4-7013-4211-9ff6-c5c74ea107b8
  type: mcq
  front: Boosting vs forests?
  back: 'Forests: parallel bagging (variance). Boosting: sequential fitting (bias).'
  choices:
  - key: a
    text: Boosting variance, forests bias.
    correct: false
  - key: b
    text: Boosting bias (sequential), forests variance (parallel).
    correct: true
  - key: c
    text: Equivalent.
    correct: false
  - key: d
    text: Boosting faster.
    correct: false
---

## Intuition
Sequential: each tree corrects previous errors. Fit tree to residuals, add to ensemble with learning rate $\eta$.

## Detail
$F_m = F_{m-1} + \eta h_m$. **For squared loss:** Fit to residuals. **General loss:** Fit to negative gradient. **Hyperparameters:** learning_rate (0.01-0.3), n_estimators (100-1000), max_depth (3-5), subsample.

## See also
- [[boosting]]
- [[gradient-descent]]
- [[loss-function]]
- [[residuals]]

## Sources
See frontmatter `sources:`.
