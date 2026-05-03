---
id: 16c1a6d5-6b3a-4ffb-abd3-59d0bfdfae83
title: Polynomial and interaction features
track: ml-engineer
topic: feature-engineering-selection
difficulty: 3
tags:
- nonlinearity
- feature-creation
- polynomial-expansion
- interactions
- feature-explosion
aliases:
- interaction-terms
- polynomial-expansion
- higher-order-features
- cross-features
sources:
- url: https://medium.com/@bhatadithya54764118/day-33-feature-selection-techniques-filter-wrapper-and-embedded-methods-00fb2bd04aa3
  label: Medium - Feature Selection Techniques Overview
- url: https://www.geeksforgeeks.org/machine-learning/feature-selection-filter-methods/
  label: GeeksforGeeks - Filter Methods
cards:
- id: 4f385bcf-d7ab-44f3-b97f-90e8959cc9d2
  type: flip
  front: Why does increasing polynomial degree from 2 to 3 cause more than a 3x increase in features?
  back: 'Combinatorial explosion. Degree-2 adds all pairs (choose 2 from n variables); degree-3 adds all
    triples. With n=10: degree 2 ≈ 55 features, degree 3 ≈ 175. Each additional degree multiplies by roughly
    (n/degree).'
- id: 1e7e1fba-2fdc-45b5-8c60-56cfa27c0334
  type: flip
  front: When would you prefer tree-based models over polynomial features?
  back: Trees capture interactions automatically without explicit feature engineering. They avoid multicollinearity,
    scale better, and require less domain knowledge. Use polynomials for linear models or interpretability.
- id: b1c40e00-6032-419d-95cf-9d8081c328b3
  type: mcq
  front: What is the primary risk of using PolynomialFeatures without feature selection?
  back: Polynomials create collinear features (e.g., x and x² are correlated) and blow up the feature
    space, increasing variance and overfitting. Use SelectKBest or L1 regularization to prune.
  choices:
  - key: a
    text: Reduced training speed due to convexity
    correct: false
  - key: b
    text: Overfitting due to high dimensionality and multicollinearity
    correct: true
  - key: c
    text: Loss of feature interpretability in tree models
    correct: false
  - key: d
    text: Inability to capture interactions
    correct: false
- id: b2d6ed48-3e6f-4487-a4e4-e6aba9096eac
  type: flip
  front: How would you select only the most important interactions without exhaustive PolynomialFeatures?
  back: Use correlation analysis or mutual information to identify pairs of features with high joint predictiveness.
    Manually create interactions for those pairs, or use wrapper methods like RFE to prune polynomial
    features.
- id: 719ca9f8-9564-403d-8bd2-4e48a4a398f8
  type: mcq
  front: In which scenario would an interaction feature x_i * x_j be most useful?
  back: Interaction terms capture synergistic effects where two weak individual predictors combine to
    strongly predict y. This is common in dose-response models, pricing, and click-through rates (placement
    × user_quality).
  choices:
  - key: a
    text: When x_i and x_j are both normally distributed
    correct: false
  - key: b
    text: When the target depends multiplicatively on both inputs (e.g., dose × exposure time in pharmacology)
    correct: true
  - key: c
    text: When the model is a decision tree with depth > 5
    correct: false
  - key: d
    text: When x_i and x_j are uncorrelated with the target individually
    correct: false
---

## Intuition

Polynomial and interaction features let linear models capture nonlinear relationships and synergistic effects. If y depends on the product of two inputs (e.g., height × weight = body mass proxy), a polynomial expansion creates that feature explicitly. The cost: exponential feature explosion. A degree-3 polynomial with 10 features yields 1000+ features; you must regularize or select.

## Detail

Polynomial features: x_new = [x, x², x³, ...]. Useful for curve fitting but prone to overfitting and multicollinearity.

Interaction features: x_new includes [x_i * x_j] for all pairs (or selected pairs). Example: in pricing, price elasticity may depend on both volume AND season (volume × season_flag).

Manual interaction selection is safer than exhaustive: domain knowledge or correlation analysis identify the most likely pairwise interactions. PolynomialFeatures with low degree + feature selection (filter or wrapper) is a practical pipeline.

```python
from sklearn.preprocessing import PolynomialFeatures
from sklearn.pipeline import Pipeline
from sklearn.feature_selection import SelectKBest, f_regression

pipe = Pipeline([
    ('poly', PolynomialFeatures(degree=2, include_bias=False)),
    ('select', SelectKBest(f_regression, k=10)),
    ('model', LinearRegression())
])
```

## Common gotchas / interview framings
- "Why does polynomial degree 3 blow up faster than you'd expect?"
- "How would you avoid multicollinearity when adding x²?"
- "When should you use interaction features vs. letting a tree capture interactions?"
- "Describe the trade-off between feature explosion and nonlinearity capture."

## See also
- [[polynomial-regression]]
- [[nonlinearity]]
- [[feature-explosion]]
- [[overfitting]]
- [[dimensionality]]
- [[regularization]]

## Sources
See frontmatter `sources:`.
