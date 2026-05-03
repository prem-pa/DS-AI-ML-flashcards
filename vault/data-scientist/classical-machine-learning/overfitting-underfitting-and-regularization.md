---
id: 397ba608-d2fc-4022-9af3-93c96eaa7094
title: Overfitting, underfitting, and regularization
track: data-scientist
topic: classical-machine-learning
difficulty: 1
tags:
- bias-variance
- overfitting
- regularization
- model-complexity
aliases:
- bias-variance tradeoff
- model complexity
- generalization
sources:
- url: https://scikit-learn.org/stable/modules/learning_curve.html
  label: scikit-learn Learning Curves
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 38c7ff8b-ac0d-4043-bc98-14927824c3b5
  type: flip
  front: Bias-variance tradeoff. Simple vs complex models?
  back: 'Simple: high bias (underfit), low variance. Complex: low bias, high variance (overfit). Optimal:
    balance via regularization/constraints.'
- id: 572f5f7b-175b-4add-b30d-d7ebf8d880e6
  type: flip
  front: Overfitting signature. How to detect?
  back: 'Large gap between training and test error. Training accurate, test poor. Learning curve: low
    training error, high validation error.'
- id: 4dec7ea8-4504-46ea-b09f-b237caf367f1
  type: flip
  front: Regularization mechanisms. Reduce overfitting?
  back: L1/L2 penalties (shrink/zero coefficients), early stopping (halt before memorization), max depth
    (limit tree), dropout (neural). Trade bias increase for variance decrease.
- id: cadd7fc0-918f-4b09-9e52-eb55a1ce1e21
  type: mcq
  front: If training error high + test error high, bias or variance?
  back: Both high + small gap = model too weak. Increase complexity (reduce bias).
  choices:
  - key: a
    text: High bias (underfitting).
    correct: true
  - key: b
    text: High variance.
    correct: false
  - key: c
    text: Both equally.
    correct: false
  - key: d
    text: Neither.
    correct: false
---

## Intuition
Bias-variance tradeoff: simple models underfit (high bias), complex overfit (high variance). Regularization (L1/L2 penalties, early stopping) reduces overfitting by trading small bias increase for variance reduction.

## Detail
**Overfitting:** Model memorizes noise; high training accuracy, low test accuracy. **Underfitting:** Model too simple; both accuracies low. **Regularization:** Add penalty to loss function or use constraints (depth, early stopping).

## See also
- [[bias-variance-tradeoff]]
- [[learning-curves]]
- [[regularization]]

## Sources
See frontmatter `sources:`.
