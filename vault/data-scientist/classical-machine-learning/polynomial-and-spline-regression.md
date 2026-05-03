---
id: 45da9e4f-8ee3-4c68-ab68-5bd45d2efc43
title: Polynomial and spline regression
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- regression
- nonlinear-models
- basis-expansion
- overfitting
aliases:
- basis functions
- polynomial features
- cubic splines
sources:
- url: https://scikit-learn.org/stable/modules/linear_model.html
  label: scikit-learn Linear Models
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: ca6fb0dd-9d4d-485e-8b61-d8f2c995eae9
  type: flip
  front: What is Runge's phenomenon? How splines avoid it?
  back: 'High-degree polynomials oscillate wildly at boundaries. Splines: (1) low-degree (cubic), (2)
    local intervals (knots), (3) continuity/smoothness constraints.'
- id: ae622c06-4a32-4fab-9712-3bcbc5d411c4
  type: flip
  front: DOF for cubic spline with $K$ knots?
  back: '$K$ interior knots → $K+1$ pieces, 4 coefficients each ($4K+4$). Constraints: $3K$ (continuity
    + 1st/2nd derivatives). DOF = $4K+4-3K = K+4$.'
- id: 48df0c95-abb6-4bea-a8e1-077a2c1ce514
  type: flip
  front: Smoothing spline penalty $\lambda\int[f'']^2$ achieves?
  back: 'Penalizes curvature. Low $\lambda$: fits closely (overfitting). High $\lambda$: smooth (underfitting).
    Cross-validate to select $\lambda$.'
- id: 94305b9b-42f0-4ca5-a1f7-2688dbd15952
  type: mcq
  front: Prefer splines over polynomials when?
  back: Splines capture local variation without global oscillations. Polynomials simpler but high degrees
    cause Runge.
  choices:
  - key: a
    text: Small data.
    correct: false
  - key: b
    text: Locally complex but globally smooth, avoid Runge.
    correct: true
  - key: c
    text: Need global coefficients.
    correct: false
  - key: d
    text: Independent features.
    correct: false
---

## Intuition
Polynomial regression adds polynomial features ($x^2, x^3, ...$) for nonlinearity while staying in linear framework. Splines fit piecewise polynomials locally, avoiding Runge's phenomenon.

## Detail
**Polynomial:** $y = \beta_0 + \beta_1 x + \beta_2 x^2 + ... + \beta_d x^d + \epsilon$. Degree $d$ controls flexibility; high $d$ risks oscillations.
**Splines:** Divide range into knots, fit cubic polynomials in each interval, enforce continuity/smoothness.
**Smoothing splines:** Minimize $\sum(y-f)^2 + \lambda\int[f'']^2$ (penalize curvature).

## See also
- [[basis-expansion]]
- [[degrees-of-freedom]]
- [[smoothing-splines]]

## Sources
See frontmatter `sources:`.
