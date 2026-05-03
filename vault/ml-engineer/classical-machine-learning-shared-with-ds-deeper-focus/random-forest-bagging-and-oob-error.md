---
id: 4e64a43d-c22b-452d-b68f-e205e6ac4605
title: 'Random forest: bagging and OOB error'
track: ml-engineer
topic: classical-machine-learning-shared-with-ds-deeper-focus
difficulty: 3
tags:
- ensemble
- bagging
- bootstrap
- variance-reduction
- oob-validation
aliases:
- bootstrap aggregation
- out-of-bag error
- ensemble trees
sources:
- url: https://scikit-learn.org/stable/modules/ensemble.html#forests
  label: sklearn Random Forest
- url: https://en.wikipedia.org/wiki/Random_forest
  label: Wikipedia Random Forest
cards:
- id: 696563f8-d8ca-4098-94a2-191d0d4b0ec3
  type: flip
  front: What fraction of rows are out-of-bag in a bootstrap sample of size n?
  back: Approximately 36.8% (1 - (1 - 1/n)^n ≈ 1 - 1/e ≈ 0.368). Useful for free validation without holdout
    set.
- id: 07aa49a3-e7b1-4735-9511-2fd1fdc78aa9
  type: flip
  front: Why does random forest feature subsampling improve generalization?
  back: Subsampling decorrelates trees. If one feature is very strong, all trees use it, leading to correlated
    predictions. Forcing feature diversity reduces correlation, lowering ensemble variance. sqrt(n_features)
    for classification is a principled choice.
- id: ba708d79-e9e0-46c0-8f6e-ab3e76c64a0c
  type: mcq
  front: In scikit-learn's RandomForestClassifier, what is the default max_depth?
  back: Unlike single Decision Trees, Random Forests allow unlimited depth by default. Variance is controlled
    via ensemble averaging and feature subsampling, not depth limits.
  choices:
  - key: a
    text: '6'
    correct: false
  - key: b
    text: '10'
    correct: false
  - key: c
    text: None (unlimited)
    correct: true
  - key: d
    text: sqrt(n_samples)
    correct: false
- id: 11bef4ef-48cc-46b3-9a7a-8e90b8137ace
  type: flip
  front: How does OOB error relate to cross-validation? Can it replace CV?
  back: 'OOB is an unbiased estimator like CV but with higher variance (no aggregation across folds).
    Good for quick estimates, but higher variance makes it risky for hyperparameter tuning. Typically
    use both: OOB for quick checks, CV for final validation.'
- id: 4c5347b7-ae9f-490d-a9b2-707f5945c3d8
  type: mcq
  front: 'Which is more reliable for feature importance in Random Forest: Mean Decrease Impurity (MDI)
    or Permutation Importance?'
  back: MDI biases toward high-cardinality features. Permutation importance uses OOB samples to measure
    actual prediction change when a feature is shuffled.
  choices:
  - key: a
    text: MDI (faster)
    correct: false
  - key: b
    text: Permutation (less biased)
    correct: true
  - key: c
    text: Both equally
    correct: false
  - key: d
    text: Neither
    correct: false
---

## Intuition

Random forests reduce variance by training many deep trees on bootstrap samples and averaging predictions. OOB error estimates generalization without a separate validation set: ~1/3 of rows are excluded from each tree and used for validation.

## Detail

**Bootstrap Aggregation (Bagging):**
- Sample rows with replacement n_samples = n rows
- On average, each bootstrap contains ~63.2% unique rows; ~36.8% are out-of-bag (OOB)
- Train independent trees on each bootstrap
- Aggregate via majority vote (classification) or mean (regression)

**OOB Error Estimation:**
- For each sample, predict using only trees where it was OOB
- Average OOB predictions = unbiased generalization estimate
- Free validation; no need for separate holdout set
- Variance is higher than cross-validation but unbiased

**Feature Subsampling:**
- sqrt(n_features) for classification, n_features/3 for regression
- Decorrelates trees; increases stability and generalization
- Reduces computational cost per tree

**Engineering considerations:**
- **Parallelization**: Trees are independent → embarrassingly parallel
- **Memory**: n_trees * tree_size; 100-1000 trees typical; large datasets → reduced max_depth
- **Warm-start**: sklearn supports iterative tree addition (useful for online scenarios)
- **GPU acceleration**: Tree training parallelizable, but Random Forest sequential aggregation bottleneck
- **Feature importance**: Mean decrease in impurity (MDI) or OOB permutation importance

```python
from sklearn.ensemble import RandomForestClassifier

rf = RandomForestClassifier(
    n_estimators=100,
    max_depth=15,
    max_samples=0.632,
    oob_score=True,
    n_jobs=-1
)
rf.fit(X_train, y_train)
print(f"OOB score: {rf.oob_score_}")
```

## Common gotchas / interview framings
- OOB error estimate has higher variance than CV; don't rely solely for hyperparameter tuning
- Feature importance via MDI can be biased toward high-cardinality features (use permutation importance)
- Warm-start reduces variance incrementally; useful for adaptive ensemble sizing
- Max_depth in Random Forest is not a bottleneck (unlike single trees); default is unlimited
- Correlated features: feature subsampling helps, but redundant features still reduce performance

## See also
- [[ensemble-methods]]
- [[bootstrap-sampling]]
- [[variance-reduction]]
- [[feature-subsampling]]

## Sources
See frontmatter `sources:`.
