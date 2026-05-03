---
id: cacecd44-127d-49c6-a10a-d210d96940bb
title: SHAP and feature importance
track: ml-engineer
topic: classical-machine-learning-shared-with-ds-deeper-focus
difficulty: 3
tags:
- interpretability
- explainability
- feature-importance
- shapley-values
- model-agnostic
aliases:
- shapley-additive-explanations
- permutation-importance
- tree-shap
sources:
- url: https://github.com/slundberg/shap
  label: SHAP GitHub
- url: https://arxiv.org/abs/1705.07874
  label: SHAP Paper (Lundberg & Lee)
cards:
- id: b3659d76-406c-4eab-b070-13ea6aa78ed5
  type: flip
  front: What is the key advantage of SHAP over permutation importance and MDI?
  back: 'SHAP satisfies Shapley value axioms (efficiency, symmetry, null player): theoretically justified
    fair allocation of prediction to features. Permutation importance biased by feature correlation; MDI
    biased toward high-cardinality features.'
- id: b8705e74-b961-4664-8c2e-e87929be91a2
  type: mcq
  front: What is the computational complexity of TreeSHAP for tree models?
  back: TreeSHAP exploits tree structure to compute Shapley values efficiently. Can be computed in polynomial
    time by traversing paths in the tree ensemble.
  choices:
  - key: a
    text: O(2^n_features) exponential
    correct: false
  - key: b
    text: O(n_trees * n_features) polynomial
    correct: true
  - key: c
    text: O(n_samples * n_features)
    correct: false
  - key: d
    text: O(n_trees^2)
    correct: false
- id: d2d2f0e5-7bde-4e24-8eef-3ea531c3f6f8
  type: flip
  front: Why can SHAP values be counterintuitive when features are highly correlated?
  back: Shapley values depend on ordering of features in coalitions. Correlated features share credit
    based on marginal contribution within coalitions, not on correlation strength. Swapping correlated
    features' order can swap their SHAP value signs.
- id: c5b95fae-c4f5-4572-ad7a-cb90741846ea
  type: mcq
  front: Which SHAP approximation is most suitable for fast GPU-accelerated deep neural networks?
  back: Gradient SHAP uses model gradients, fast on GPU. TreeSHAP requires trees; Kernel/Permutation SHAP
    require many model evaluations.
  choices:
  - key: a
    text: TreeSHAP
    correct: false
  - key: b
    text: Kernel SHAP
    correct: false
  - key: c
    text: Gradient SHAP / DeepSHAP
    correct: true
  - key: d
    text: Permutation SHAP
    correct: false
- id: fdcab6bd-7398-4ca1-ba95-f25def5d3f7a
  type: flip
  front: How do you interpret SHAP values quantitatively in a binary classification task?
  back: prediction = base_value + Σ SHAP_i. Base value is average model output (e.g., 0.5 for balanced
    dataset). Positive SHAP_i pushes toward class 1; negative toward class 0. |SHAP_i| indicates magnitude
    of impact.
---

## Intuition

SHAP (SHapley Additive exPlanations) assigns each feature a value indicating its contribution to pushing the prediction away from the base value. Based on game theory: Shapley values fairly distribute the prediction among features.

## Detail

**Shapley Value concept:**
- Coalition game: each feature is a 'player'; prediction is the 'payoff'
- Shapley value: average marginal contribution across all orderings of features
- Fair allocation: satisfies efficiency, symmetry, null player, additivity axioms
- Computation: O(2^n_features) for exact values (intractable for high-dim)

**TreeSHAP (fast for tree models):**
- Polynomial-time algorithm O(n_trees * n_features) for trees
- Exactly computes Shapley values by traversing tree structure
- Available in xgboost.explain, lightgbm, sklearn

**SHAP approximations:**
- Kernel SHAP: model-agnostic, samples feature coalitions
- Permutation SHAP: samples permutation orders
- Gradient SHAP: uses model gradients (fast for neural networks)
- DeepSHAP: combines Integrated Gradients + SHAP

**Interpretation:**
- SHAP value > 0: pushes prediction up (toward positive class)
- SHAP value < 0: pushes prediction down
- |SHAP value|: magnitude of impact
- Base value: average model output; prediction = base_value + Σ SHAP_i

**vs. other importance measures:**
- **Permutation importance**: Measures error increase when feature is shuffled; biased by feature correlation
- **MDI (Mean Decrease Impurity)**: Counts split frequency; biased toward high-cardinality features
- **SHAP**: Model-agnostic, theoretically justified, computationally expensive

**Engineering considerations:**
- **Computational cost**: TreeSHAP is fast for tree models. Kernel/Permutation SHAP slow for large n or complex models
- **Feature interactions**: SHAP captures interactions implicitly; Interaction SHAP values available but expensive
- **Baseline selection**: Base value choice affects interpretation; typically use training mean
- **Local vs. global**: Individual SHAP values (local); mean |SHAP| across dataset (global feature importance)
- **Distributed computation**: SHAP values independent per sample; parallelizable

```python
import shap
import xgboost as xgb

model = xgb.XGBClassifier().fit(X_train, y_train)
explainer = shap.TreeExplainer(model)
shap_values = explainer.shap_values(X_test)

shap.summary_plot(shap_values, X_test, plot_type='bar')
shap.force_plot(explainer.expected_value, shap_values[0], X_test.iloc[0])
```

## Common gotchas / interview framings
- SHAP is not causal; correlation vs. causation applies
- TreeSHAP fast for trees; Kernel/Permutation SHAP slow; choose algorithm based on model type
- Feature correlation: if features correlated, SHAP values can be counterintuitive (allocation depends on ordering)
- Baseline value choice matters: change it, SHAP values change; select consistently
- Imbalanced classes: SHAP base value is class-agnostic; consider stratified baseline

## See also
- [[interpretable-ml]]
- [[game-theory-shapley]]
- [[feature-selection-vs-importance]]

## Sources
See frontmatter `sources:`.
