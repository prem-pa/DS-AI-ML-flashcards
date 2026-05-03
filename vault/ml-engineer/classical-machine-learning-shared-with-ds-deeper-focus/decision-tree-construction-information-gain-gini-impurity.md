---
id: af10fe08-50e6-4a8b-b600-1cfa2b43c0bb
title: Decision tree construction (Information Gain, Gini impurity)
track: ml-engineer
topic: classical-machine-learning-shared-with-ds-deeper-focus
difficulty: 3
tags:
- trees
- splitting-criteria
- feature-selection
- entropy
- impurity
aliases:
- tree splitting
- feature selection in trees
- information theory
sources:
- url: https://scikit-learn.org/stable/modules/tree.html
  label: scikit-learn Tree docs
- url: https://en.wikipedia.org/wiki/Decision_tree_learning
  label: Wikipedia Decision Trees
cards:
- id: 55a7e011-10bb-4230-a586-a02a37ea18c0
  type: flip
  front: What is Gini impurity and how does it differ from entropy for tree splitting?
  back: Gini impurity = 1 - Σ(p_i²) measures probability of misclassification. Entropy = -Σ(p_i * log2(p_i))
    measures information content. Gini is faster (no log) and slightly less aggressive; both are used
    in practice. CART uses Gini; ID3/C4.5 use entropy.
- id: 870f3b29-4077-4b78-9cd7-c38d4fd8fcbd
  type: flip
  front: Why might histogram-based splitting in XGBoost/LightGBM be preferred over exact splits for large
    datasets?
  back: 'Histogram binning reduces memory footprint and speeds up split search. Exact splits require storing
    all feature values; histograms use O(bins) space instead of O(rows). Trade-off: slightly lower impurity
    reduction vs. major speed/memory gains, especially critical for distributed training.'
- id: 730dd157-f800-4d42-87a8-75212bdead50
  type: mcq
  front: Which splitting criterion does scikit-learn's DecisionTreeClassifier use by default?
  back: CART algorithm uses Gini. Entropy can be set via criterion='entropy' parameter.
  choices:
  - key: a
    text: Entropy
    correct: false
  - key: b
    text: Gini impurity
    correct: true
  - key: c
    text: Information gain ratio
    correct: false
  - key: d
    text: Chi-square
    correct: false
- id: 73876e0b-1ff6-4bee-8838-83f815058d4e
  type: mcq
  front: Information Gain biases toward features with high cardinality. What is the standard solution?
  back: Gain ratio divides information gain by the entropy of the split itself, penalizing high-cardinality
    features.
  choices:
  - key: a
    text: Use Gini instead
    correct: false
  - key: b
    text: Gain ratio (normalize by split info)
    correct: true
  - key: c
    text: Reduce cardinality via encoding
    correct: false
  - key: d
    text: All of the above
    correct: false
- id: 823b5cba-7165-4663-85dd-2b4e830b5df5
  type: flip
  front: In XGBoost/LightGBM, why is the split search cost nearly independent of the number of training
    rows?
  back: 'Histogram-based splitting: build histograms once (O(rows * bins)), then scan histograms for best
    split (O(bins²)). Contrast with exact splitting (O(rows * sort + rows * features)). This enables efficient
    distributed training on high-volume data.'
---

## Intuition

Decision trees recursively partition the feature space by selecting splits that maximally reduce impurity. Two common splitting criteria are Information Gain (based on entropy) and Gini impurity. Information Gain measures the reduction in entropy after a split, while Gini measures the probability of misclassification if you randomly labeled a sample according to the class distribution.

## Detail

**Information Gain (Entropy-based):**
- Entropy: H(S) = -Σ p_i * log2(p_i)
- Information Gain = H(parent) - (weighted_sum of H(children))
- Favors balanced splits; higher computational cost due to logarithm

**Gini Impurity:**
- Gini(S) = 1 - Σ p_i²
- Computationally simpler (no logarithm); slightly less aggressive than entropy
- CART algorithm uses Gini by default

**Engineering considerations:**
- For large datasets with many classes, Gini is faster (no log computation)
- Information Gain may rank features differently; choose based on domain knowledge
- Splitting cost scales with feature cardinality; categorical features need encoding or special handling
- In parallel tree construction (XGBoost), exact splits are approximated via histograms for memory efficiency

**sklearn/xgboost defaults:**
- sklearn Decision Tree: criterion='gini' by default
- XGBoost: uses second-order derivatives for splitting

```python
from sklearn.tree import DecisionTreeClassifier
dt = DecisionTreeClassifier(criterion='entropy', max_depth=5)
dt.fit(X_train, y_train)
```

## Common gotchas / interview framings
- Information Gain biases toward high-cardinality features (gain ratio is an alternative)
- Splitting is greedy; does not guarantee global optimality
- Ties in impurity reduction can lead to non-deterministic splits (random_state needed)
- Histogram-based approximations in XGBoost/LightGBM trade exact splits for speed/memory

## See also
- [[entropy-information-theory]]
- [[gini-impurity-vs-entropy]]
- [[optimal-binary-splits]]
- [[id3-c45-cart]]

## Sources
See frontmatter `sources:`.
