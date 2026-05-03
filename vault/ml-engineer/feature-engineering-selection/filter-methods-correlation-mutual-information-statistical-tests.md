---
id: d126d90e-b86e-4c1f-897b-32fe92534d37
title: Filter methods (correlation, mutual information, statistical tests)
track: ml-engineer
topic: feature-engineering-selection
difficulty: 3
tags:
- feature-selection
- filter-methods
- correlation
- mutual-information
- statistical-tests
aliases:
- univariate-selection
- correlation-filtering
- information-gain
- chi-squared-tests
sources:
- url: https://sebastianraschka.com/faq/docs/feature_sele_categories.html
  label: Sebastian Raschka - Feature Selection Categories
- url: https://www.geeksforgeeks.org/machine-learning/feature-selection-filter-methods/
  label: GeeksforGeeks - Filter Methods
- url: https://www.blog.trainindata.com/feature-selection-with-wrapper-methods/
  label: Train in Data - Feature Selection Methods
cards:
- id: 7dd30ee1-d964-48f3-9407-aaf8874f2f70
  type: flip
  front: Why might a feature with high correlation to the target still be useless in a model?
  back: Correlation measures linear association, but targets often depend on interactions or nonlinear
    combinations. A feature may be correlated due to confounding (a third variable drives both). Always
    validate in a held-out set; filter is a screening, not a guarantee.
- id: 38e3a3b5-566e-4670-b365-a08f7b3dbd12
  type: mcq
  front: Which filter method best detects nonlinear relationships between feature and target?
  back: Mutual information measures information content without assuming linearity. Pearson misses nonlinear
    patterns. If target = x², MI will flag x as informative; Pearson correlation ≈ 0.
  choices:
  - key: a
    text: Pearson correlation (always linear)
    correct: false
  - key: b
    text: Mutual information (captures both linear and nonlinear dependencies)
    correct: true
  - key: c
    text: Variance threshold (ignores target entirely)
    correct: false
  - key: d
    text: T-test (univariate only)
    correct: false
- id: 202a1dce-baa8-4ae5-8535-b1e40c108c05
  type: flip
  front: What is the primary limitation of filter-based feature selection?
  back: Filters rank features independently without considering interactions or redundancy. Two highly
    correlated features both rank high even though you need only one. Feature A alone may be weak, but
    A+B together predict well; filters miss this synergy.
- id: 927e108c-7599-40d5-90ae-17e08109876e
  type: flip
  front: How would you combine Pearson correlation and mutual information for robust feature screening?
  back: 'Rank features by both metrics. Keep features in top-k by MI (catches nonlinearity), then subset
    those by high |Pearson correlation| (captures strong linear effects). Or: union of top-k by each method,
    then apply a wrapper to refine.'
- id: 5d7925db-1d56-476c-8c1c-da8e3b487d72
  type: mcq
  front: Why is SelectKBest(chi2) unsuitable for a dataset with negative values?
  back: 'Chi-squared tests require non-negative feature values. Preprocessing step: shift all features
    >= 0 if using chi2. Alternatively, use mutual_info_classif or f_classif, which handle negative values.'
  choices:
  - key: a
    text: Chi-squared assumes non-negative inputs (it squares values)
    correct: true
  - key: b
    text: Negative values always reduce feature importance
    correct: false
  - key: c
    text: Chi-squared only works for categorical targets
    correct: false
  - key: d
    text: SelectKBest doesn't support chi2 in sklearn
    correct: false
---

## Intuition

Filter methods rank features by their intrinsic statistical properties (correlation with target, mutual information, variance) without training a model. They're fast, model-agnostic, and interpretable but can miss interactions and dependencies between features. Use them as a first-pass screening step to remove obviously weak features before wrapper or embedded methods.

## Detail

**Pearson correlation**: For continuous targets, measure linear association. High |r| → strong linear relationship. Simple and fast; misses nonlinear dependencies.

**Mutual information (MI)**: Captures nonlinear dependencies. MI(X; Y) measures bits of information X contains about Y. High MI → strong association (linear or nonlinear). More powerful than correlation but requires binning/estimation.

**Chi-squared test**: For categorical targets, tests independence of categorical features. High χ² statistic + low p-value → feature is predictive.

**ANOVA F-test**: For continuous targets with categorical features. Tests if mean target differs across feature categories. High F-statistic → feature is predictive.

**Variance threshold**: Remove near-zero-variance features (little information content).

```python
from sklearn.feature_selection import SelectKBest, mutual_info_classif, f_classif, chi2, VarianceThreshold
from sklearn.pipeline import Pipeline

# Mutual information for classification
select_mi = SelectKBest(mutual_info_classif, k=10)

# ANOVA F-test for regression
select_f = SelectKBest(f_regression, k=15)

# Chi-squared for non-negative features
select_chi2 = SelectKBest(chi2, k=10)

# Variance threshold
var_threshold = VarianceThreshold(threshold=0.01)
```

## Common gotchas / interview framings
- "Why does high correlation not guarantee predictiveness?"
- "When would mutual information outperform Pearson correlation?"
- "How do you combine filter results (e.g., top-10 by MI + top-5 by ANOVA)?"
- "What's the drawback of filter methods in capturing feature interactions?"

## See also
- [[feature-selection]]
- [[correlation]]
- [[mutual-information]]
- [[pearson-correlation]]
- [[p-values]]
- [[statistical-testing]]

## Sources
See frontmatter `sources:`.
