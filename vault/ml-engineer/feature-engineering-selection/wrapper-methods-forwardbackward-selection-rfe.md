---
id: 62bb8129-03c0-4170-a606-f7672d1dd6a4
title: Wrapper methods (forward/backward selection, RFE)
track: ml-engineer
topic: feature-engineering-selection
difficulty: 3
tags:
- feature-selection
- wrapper-methods
- recursive-feature-elimination
- forward-selection
- backward-selection
aliases:
- sequential-feature-selection
- RFE
- greedy-selection
- model-based-selection
sources:
- url: https://sebastianraschka.com/faq/docs/feature_sele_categories.html
  label: Sebastian Raschka - Feature Selection Categories
- url: https://www.blog.trainindata.com/feature-selection-with-wrapper-methods/
  label: Train in Data - Wrapper Methods
- url: https://www.frontiersin.org/journals/big-data/articles/10.3389/fdata.2025.1624507/pdf
  label: Frontiers - Comprehensive Feature Selection Review
cards:
- id: 302fd175-9ff4-4ab2-b71d-3c74ef035e23
  type: flip
  front: Why might forward selection and backward elimination select different feature subsets for the
    same model?
  back: Forward selection builds greedily from scratch, so early choices lock in. Backward elimination
    starts with all features and accounts for redundancy/multicollinearity early. Removal of a weak feature
    in backward may expose a strong feature, but forward might have skipped that path. The greedy algorithms
    find local optima, not global.
- id: 039f3d97-47c7-401b-85bb-92605108eda5
  type: mcq
  front: How do you prevent wrapper-based feature selection from overfitting to the training set?
  back: Nested CV prevents selection overfitting. Inner loop (e.g., 5-fold) selects features on each fold's
    training data. Outer loop (e.g., 3-fold) evaluates the final model on truly held-out data. Without
    nesting, you optimize to noise.
  choices:
  - key: a
    text: Use the same train/test split repeatedly (no cross-validation)
    correct: false
  - key: b
    text: 'Use nested cross-validation: inner loop for selection, outer loop for evaluation'
    correct: true
  - key: c
    text: Disable regularization to ensure features are 'pure'
    correct: false
  - key: d
    text: Select features only from highly correlated features
    correct: false
- id: 1fbe3b31-03f5-47ba-83fc-4014a1f83065
  type: flip
  front: A dataset has 100 features. Would you use forward selection or backward elimination first, and
    why?
  back: Start with backward elimination. Removing obviously weak features from 100 is faster than building
    forward from 0. Forward selection is more practical when feature count is manageable (~20-50). For
    very high dimensions, use filter + RFE + L1 regularization for efficiency.
- id: 53660ce8-2529-4d06-b4e6-9ee5b3a10434
  type: flip
  front: Describe a scenario where RFE would fail but forward selection would succeed.
  back: RFE relies on single-model importance scores (e.g., tree feature importance). If a feature is
    weak in isolation but powerful in combination with another, RFE may prune it early. Forward selection,
    by iteratively building combos, would recognize the synergy and keep it.
- id: fa64d5f8-4b82-4207-aa90-d5ba78d45193
  type: mcq
  front: What is the computational complexity of forward selection with n features and k-fold CV?
  back: 'Forward selection: iterate k times (target features), each iteration trains a model on O(n) features
    with k-fold CV. Total: O(k * n * training_cost). With m target features, O(m * n * k * model_cost).
    For n=100, m=20, k=5, this is ~10k model trains—expensive but feasible.'
  choices:
  - key: a
    text: O(n) - linear, very fast
    correct: false
  - key: b
    text: O(n*m) where m is target number of features - quadratic in the worst case
    correct: true
  - key: c
    text: O(2^n) - exponential, only for small n
    correct: false
  - key: d
    text: O(log n) - logarithmic
    correct: false
---

## Intuition

Wrapper methods train a model repeatedly on different subsets of features, evaluating each subset by model performance (e.g., cross-validation score). Unlike filters, wrappers account for model-specific interactions and can uncover synergistic feature combinations. The cost: computationally expensive (O(n²) or worse for exhaustive search), prone to overfitting if validation is loose, and model-dependent (the selected features are optimal for that specific model, not universally).

## Detail

**Forward selection**: Start with empty set. Iteratively add the feature that most improves CV score. Stop when improvement plateaus.

**Backward elimination**: Start with all features. Iteratively remove the feature whose removal hurts CV score least. Stop when removal degrades performance significantly.

**Recursive feature elimination (RFE)**: Train model, rank features by importance (coefficients, tree importance), remove bottom k%, repeat. Linear in iterations, efficient for high-dimensional data.

**SequentialFeatureSelector**: sklearn's flexible forward/backward selection with cross-validation.

Best practices:
- Use cross-validation (StratifiedKFold for classification) to avoid overfitting the selection.
- Set a minimum CV score threshold or patience to stop early.
- Pair with regularization to avoid selecting noise.

```python
from sklearn.feature_selection import RFE, SequentialFeatureSelector
from sklearn.ensemble import RandomForestClassifier
from sklearn.linear_model import LogisticRegression

# RFE: recursive elimination
rf = RandomForestClassifier(n_estimators=100, random_state=42)
rfe = RFE(rf, n_features_to_select=10, step=5)
X_rfe = rfe.fit_transform(X, y)

# SequentialFeatureSelector: forward selection with CV
lr = LogisticRegression(max_iter=1000)
sfs = SequentialFeatureSelector(lr, n_features_to_select=10, cv=5, direction='forward')
X_sfs = sfs.fit_transform(X, y)
```

## Common gotchas / interview framings
- "When is forward selection preferable to backward elimination?"
- "How do you avoid overfitting during wrapper feature selection?"
- "Why does RFE sometimes select different features than forward selection for the same model?"
- "Compare computational cost of forward selection (n²) vs. filter methods (O(n))."

## See also
- [[feature-selection]]
- [[recursive-feature-elimination]]
- [[model-training]]
- [[cross-validation]]
- [[computational-cost]]
- [[feature-interactions]]

## Sources
See frontmatter `sources:`.
