---
id: 89b0e382-5eb7-411d-b91c-9d110be229bb
title: Logistic regression
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- classification
- supervised-learning
- probabilistic-model
- log-odds
aliases:
- logit model
- sigmoid regression
- binary classification
sources:
- url: https://scikit-learn.org/stable/modules/generated/sklearn.linear_model.LogisticRegression.html
  label: scikit-learn LogisticRegression
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 894b2fbc-6b24-4064-816a-596cbe71636e
  type: flip
  front: Derive logistic regression loss from Bernoulli likelihood.
  back: '$\prod_i \sigma^{y_i}(1-\sigma)^{1-y_i}$ → log: $\sum_i[y_i\log\sigma + (1-y_i)\log(1-\sigma)]$.
    Negate for loss.'
- id: 157fa137-809b-4dd0-b427-e415a2ebc7bf
  type: flip
  front: Log-odds interpretation of logistic regression coefficients?
  back: $\log(\text{odds}) = X\beta$. If $\beta_j=0.3$, unit increase in $X_j$ multiplies odds by $e^{0.3}
    \approx 1.35$ (35% increase). Relative risk for stakeholders.
- id: 9ea09830-5f53-4839-981a-b801e7ddbd52
  type: flip
  front: How tune classification threshold? When not 0.5?
  back: Use ROC/PR curve. High FP cost → raise threshold > 0.5. High FN cost → lower < 0.5. Default 0.5
    assumes equal costs, balanced classes.
- id: 768c6a12-c456-48ab-8904-8d1000a95f27
  type: mcq
  front: Logistic gradient vs linear regression gradient?
  back: Logistic gradient nonlinear (sigmoid inside), requires iterative solvers.
  choices:
  - key: a
    text: $X^T(X\beta-y)$ vs $X^T(\sigma(X\beta)-y)$.
    correct: true
  - key: b
    text: Same.
    correct: false
  - key: c
    text: Logistic linear.
    correct: false
  - key: d
    text: Neither.
    correct: false
---

## Intuition
Logistic regression models binary classification using sigmoid to map predictions into probabilities $P(y=1|X) = \sigma(X\beta) = \frac{1}{1+e^{-X\beta}}$.

## Detail
**Log-odds:** $\log(\text{odds}) = X\beta$. Unit increase in $X_j$ multiplies odds by $e^{\beta_j}$.
**Loss:** Cross-entropy $= -\sum_i [y_i \log\sigma(X_i\beta) + (1-y_i)\log(1-\sigma(X_i\beta))]$
No closed form; iterative solvers (Newton, L-BFGS, SGD).

## See also
- [[sigmoid-function]]
- [[log-odds]]
- [[cross-entropy-loss]]
- [[odds-ratio]]

## Sources
See frontmatter `sources:`.
