---
id: 7d119207-c33e-47e6-97eb-20d546172074
title: Learning curves and bias-variance decomposition
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 3
tags:
- learning curves
- bias-variance
- sample size
- overfitting
- underfitting
- data scaling
aliases:
- training curves
- convergence diagnosis
- model capacity
sources:
- url: https://developers.google.com/machine-learning/crash-course/classification/accuracy-precision-recall
  label: Google ML Crash Course
- url: https://www.ml4devs.com/what-is/model-evaluation-metrics/
  label: 'ML4Devs: Model Evaluation Metrics'
cards:
- id: 5bde8e0c-3690-496f-a26b-b392397bb59d
  type: flip
  front: You observe that both training and validation error plateau at 20% error with a small gap. Diagnose.
  back: 'High bias (underfitting). Model is too simple; it cannot fit the training data well, so it generalizes
    poorly. Small gap means minimal overfitting. Solutions: (1) increase model complexity (more layers,
    features), (2) reduce regularization strength, (3) try different architecture, (4) add polynomial
    features or interactions. More data won''t help much here.'
- id: 90ec8b61-8916-4881-a984-096b1c6ef240
  type: flip
  front: 'Your learning curve shows: training error 2%, validation error 25%, large gap. Diagnose and
    recommend fixes.'
  back: 'High variance (overfitting). Model memorizes training data but doesn''t generalize. Solutions
    ranked by cost-effectiveness: (1) collect more training data (if possible), (2) increase regularization
    (L1/L2, dropout), (3) reduce model complexity (fewer features, shallower network), (4) cross-validation
    with more folds, (5) ensemble to reduce variance. Prioritize data.'
- id: bf1b44f2-1c64-493a-8fcf-23bc33c21e69
  type: mcq
  front: 'Bias-variance decomposition: MSE = Bias² + Var + σ². Which term cannot be reduced?'
  back: 'σ² is inherent label noise; cannot be removed. Bias² reduced by increasing model complexity.
    Variance reduced by regularization/more data. Trade-off: complex model = low bias, high variance;
    simple = high bias, low variance.'
  choices:
  - key: a
    text: Bias²
    correct: false
  - key: b
    text: Variance
    correct: false
  - key: c
    text: σ² (irreducible noise)
    correct: true
  - key: d
    text: All can be reduced
    correct: false
- id: a726c92f-4f29-4b6f-9d63-426e91864e64
  type: flip
  front: Your validation error increases as you add more training samples. What's wrong?
  back: 'Likely data quality/distribution shift issue: (1) label noise in new data, (2) different source
    distribution than validation, (3) data entry errors in batch addition. Check: plot feature distributions
    (new vs old), inspect label agreement (inter-annotator agreement), check for outliers. Learning curves
    should be monotonically decreasing or flat; increasing suggests external problem.'
- id: 8ea82178-3c7c-43f1-96b8-e67c55e31ed8
  type: flip
  front: For a decision tree, how does learning curve change as you reduce max_depth from 20 to 5?
  back: 'Reducing depth increases bias (underfitting): training error increases, but gap between train/test
    shrinks (less overfitting). Combined effect depends on bias-variance tradeoff. If current depth=20
    overfits, depth=5 improves test error despite higher training error. Optimal depth found via learning
    curves or cross-validation.'
---

## Intuition
Learning curves plot train/test error vs training set size (or iterations). Patterns diagnose whether model needs more data, regularization, or architecture changes. Bias-variance decomposition decomposes test error into reducible bias (underfitting), variance (overfitting), and irreducible noise.

## Detail
**Learning curve patterns**:
1. Both curves plateau at high error → high bias (underfitting). Gap is small. Solution: increase model complexity, add features, reduce regularization.
2. Training error low, test high, large gap → high variance (overfitting). Solution: more data, regularization (L1/L2, dropout), simpler model.
3. Both curves decrease, gap shrinking as n increases → ideal; model improving with more data.
4. Test error increases with n → data quality issue, distribution shift, or label noise.

**Bias-Variance Decomposition** (for squared loss):
$$E[(y - \hat{y})^2] = \text{Bias}^2[\hat{y}] + \text{Var}[\hat{y}] + \sigma^2$$
- Bias²: systematic error from model assumptions (e.g., linear model on nonlinear data)
- Var: sensitivity to training data fluctuations (small-sample overfitting)
- σ²: irreducible noise in y (cannot be reduced)

**Bias-Variance tradeoff**: increasing complexity reduces bias but increases variance. U-shaped total error; minimum at sweet spot.

**Early stopping**: monitor test error during training; stop when test error stops decreasing. Prevents variance accumulation without regularization parameter tuning.

## Common gotchas / interview framings
- Learning curve convergence: if plateau far from test performance, fix data quality/distribution shift first
- Sample size scaling: increasing n helps high-variance models more than high-bias. Rule of thumb: 10:1 ratio features:samples minimum
- Cross-validation learning curves: use CV folds to smooth noise in learning curves
- Plateau interpretation: high train/test plateau → underfitting; gap → overfitting. Both plateau = accept performance or add signal
- Training data size vs computational cost: diminishing returns; doubling data for 1% improvement rarely worth it

## See also
- [[bias-variance-tradeoff]]
- [[learning-curve]]
- [[overfitting]]
- [[underfitting]]
- [[sample-complexity]]

## Sources
See frontmatter `sources:`.
