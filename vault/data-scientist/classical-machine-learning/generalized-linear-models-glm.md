---
id: 3d170546-db45-464f-a72a-7d42a4d98660
title: Generalized Linear Models (GLM)
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- regression
- classification
- probabilistic-models
- link-functions
aliases:
- exponential family
- link function
- GLM framework
sources:
- url: https://scikit-learn.org/stable/modules/linear_model.html
  label: scikit-learn Linear Models
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: f29928ed-b6fc-422e-86b0-8d061fa0f4cc
  type: flip
  front: 'Three components of GLM? Example: logistic.'
  back: '1. Binomial dist. 2. $\eta = X\beta$. 3. Logit link: $g(\mu) = \log(\mu/(1-\mu))$, so $E[y] =
    1/(1+e^{-X\beta})$.'
- id: 465a15e4-96ec-4155-9f5b-034b8a4c2d67
  type: flip
  front: Poisson regression link function? Why natural?
  back: 'Log link: $g(\mu) = \log(\mu)$, so $\mu = e^{X\beta}$. Natural: (1) ensures $\mu>0$ (counts),
    (2) $e^{\beta_j}$ is rate ratio, (3) aligns with Poisson.'
- id: 4e7548d7-079f-4d17-acf8-e62a08895b16
  type: flip
  front: Overdispersion in Poisson regression?
  back: 'Poisson assumes $\text{Var}=\mu$. Overdispersion: observed $>$ model. Indicates misspecification.
    Fix: quasi-Poisson or negative binomial.'
- id: c5b8c517-e622-4814-af95-278a628a6353
  type: mcq
  front: Why 'generalized'?
  back: GLMs generalize by allowing exponential family distributions + link functions.
  choices:
  - key: a
    text: Faster.
    correct: false
  - key: b
    text: Extends regression to non-Gaussian responses via link functions.
    correct: true
  - key: c
    text: Deep learning.
    correct: false
  - key: d
    text: No assumptions.
    correct: false
---

## Intuition
GLMs extend linear regression from Gaussian to any exponential family (Poisson, binomial, gamma). Link function transforms linear predictor to expected response.

## Detail
**Components:** Random component ($y \sim$ distribution), systematic ($\eta = X\beta$), link function ($g(E[y]) = \eta$).
**Examples:** Linear (Gaussian, identity), logistic (binomial, logit), Poisson (log link), Gamma (inverse).
**Fitting:** MLE via IRLS or Newton-Raphson.

## See also
- [[logistic-regression]]
- [[poisson-regression]]
- [[link-function]]
- [[deviance]]

## Sources
See frontmatter `sources:`.
