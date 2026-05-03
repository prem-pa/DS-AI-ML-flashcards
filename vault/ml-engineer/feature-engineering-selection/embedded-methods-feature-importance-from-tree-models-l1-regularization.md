---
id: 5092613e-3f89-4695-9bcd-40099fb28adb
title: Embedded methods (feature importance from tree models, L1 regularization)
track: ml-engineer
topic: feature-engineering-selection
difficulty: 3
tags:
- feature-selection
- embedded-methods
- feature-importance
- L1-regularization
- lasso
- tree-models
aliases:
- lasso-selection
- tree-importance
- regularization-based-selection
- coeff-pruning
sources:
- url: https://sebastianraschka.com/faq/docs/feature_sele_categories.html
  label: Sebastian Raschka - Feature Selection Categories
- url: https://medium.com/@abhaysingh71711/efficient-feature-selection-a-guide-to-embedded-methods-for-optimized-machine-learning-45ea3c83e622
  label: Medium - Embedded Methods for Feature Selection
- url: https://www.blog.trainindata.com/feature-selection-with-wrapper-methods/
  label: Train in Data - Embedded and Wrapper Methods
cards:
- id: f26f1b6a-40a3-4735-aa72-90a7d909e29a
  type: flip
  front: Why does L1 regularization (Lasso) force weak coefficients to zero, while L2 (Ridge) only shrinks
    them?
  back: 'L1 penalty: λ * Σ|coef|. The absolute value creates a ''corner'' in the penalty geometry; optimal
    solutions often hit zero. L2 penalty: λ * Σ(coef²). The quadratic shape has no corner; solutions smoothly
    approach zero but rarely reach it exactly. L1 → sparse; L2 → dense but small.'
- id: dcbe476e-57e1-45be-92c2-6dfbeb597ba4
  type: mcq
  front: 'Tree feature importance is unreliable when:'
  back: With correlated features, a tree may use either feature A or B (both carry same signal); importance
    is split between them, and which one 'wins' in a split is somewhat arbitrary. Permutation importance
    avoids this by measuring actual prediction impact.
  choices:
  - key: a
    text: The target is continuous (regression)
    correct: false
  - key: b
    text: Features are correlated; importance is arbitrarily divided among them
    correct: true
  - key: c
    text: The tree depth is > 10
    correct: false
  - key: d
    text: You use entropy instead of Gini impurity
    correct: false
- id: ba6db36a-5712-4aab-b73f-3b76174194a5
  type: flip
  front: When would you prefer permutation importance over tree feature importance?
  back: 'Permutation importance is model-agnostic, accounts for feature interactions, and handles multicollinearity
    better (both correlated features show high importance if both are used). Use it when: (1) model is
    not a tree, (2) features are correlated, (3) you want a post-hoc explanation.'
- id: 3b09b8ee-9488-4027-8519-ea0c3645289b
  type: flip
  front: How do you use LassoCV to automatically select features?
  back: Fit LassoCV(cv=k) on X, y. It cross-validates to find optimal λ. Features with coef_ != 0 are
    selected. Access via X_selected = X[:, lasso.coef_ != 0]. Quick, efficient, and handles multicollinearity.
- id: e5c21c80-e46e-4ce9-b696-cc53134e203c
  type: mcq
  front: A Random Forest ranks Feature A (importance=0.08) and Feature B (importance=0.07) as top-2. They
    are highly correlated. What should you do?
  back: Permutation importance measures actual prediction impact; one feature may dominate in a holdout
    set. Tree importance doesn't distinguish which of two correlated features is 'truly' useful. Permutation
    clarifies.
  choices:
  - key: a
    text: Keep both; they have the highest importance scores
    correct: false
  - key: b
    text: Use permutation importance to break the tie and determine which is truly predictive
    correct: true
  - key: c
    text: Randomly drop one and retrain
    correct: false
  - key: d
    text: Encode them as a single interaction feature
    correct: false
---

## Intuition

Embedded methods perform feature selection during model training, not before or after. L1 regularization (Lasso) shrinks weak coefficients to zero, effectively removing features. Tree-based feature importance ranks features by their contribution to splits and prediction, identifying which features the model actually uses. Embedded methods are efficient (single model training) and couple selection to the final model, ensuring features are chosen for the task at hand.

## Detail

**L1 regularization (Lasso)**: Adds penalty term λ * Σ|coefficient_i| to the loss. As λ increases, weak coefficients → 0 exactly (sparse solution). Features with coeff ≠ 0 are selected. Linear model; interpretable; fast.

**Tree feature importance**: Computed as the decrease in impurity (Gini, entropy) each feature contributes across all splits. High importance → feature is used often and effectively. Works for Random Forests, Gradient Boosting. Not reliable for high-cardinality categoricals or when features are correlated (importance is shared arbitrarily).

**Permutation importance**: Shuffle each feature, measure performance drop. Model-agnostic and stable under multicollinearity (both correlated features show high importance if both are useful).

**Elastic Net**: L1 + L2 (ridge) penalty. Balances sparsity and stability.

```python
from sklearn.linear_model import LassoCV, LogisticRegression
from sklearn.ensemble import RandomForestClassifier
import numpy as np

# Lasso with cross-validation to find optimal lambda
lasso = LassoCV(cv=5).fit(X, y)
selected_features = np.where(lasso.coef_ != 0)[0]

# Tree feature importance
rf = RandomForestClassifier(n_estimators=100, random_state=42).fit(X, y)
feature_importance = rf.feature_importances_

# Permutation importance
from sklearn.inspection import permutation_importance
perm_importance = permutation_importance(rf, X_test, y_test, n_repeats=10, random_state=42)
```

## Common gotchas / interview framings
- "Why does tree importance differ from permutation importance?"
- "How does L1 differ from L2 regularization in terms of feature selection?"
- "When is tree importance unreliable, and what should you use instead?"
- "Compare computational cost: L1 selection vs. wrapper methods."

## See also
- [[feature-selection]]
- [[l1-regularization]]
- [[lasso]]
- [[tree-importance]]
- [[regularization]]
- [[feature-importance]]

## Sources
See frontmatter `sources:`.
