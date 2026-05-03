---
id: 52429266-dff4-46db-9e22-66225743ff69
title: Multicollinearity and correlation cleanup
track: ml-engineer
topic: feature-engineering-selection
difficulty: 3
tags:
- multicollinearity
- correlation
- feature-redundancy
- variance-inflation
- feature-selection
- linear-models
aliases:
- collinearity-reduction
- VIF-filtering
- correlation-clustering
- redundancy-cleanup
sources:
- url: https://sebastianraschka.com/faq/docs/feature_sele_categories.html
  label: Sebastian Raschka - Feature Selection and Multicollinearity
- url: https://www.geeksforgeeks.org/machine-learning/feature-selection-filter-methods/
  label: GeeksforGeeks - Feature Selection
- url: https://www.frontiersin.org/journals/big-data/articles/10.3389/fdata.2025.1624507/pdf
  label: Frontiers - Feature Selection Review (2025)
- url: https://link.springer.com/article/10.1007/s11069-024-06878-6
  label: Springer - Comparison of Feature Selection Methods
cards:
- id: b64ac88d-91a3-4018-8511-4de0fd496905
  type: flip
  front: Why does multicollinearity inflate variance in linear regression coefficients?
  back: In y = b0 + b1*x1 + b2*x2 + noise, if x1 and x2 are nearly collinear, the design matrix X'X is
    nearly singular. (X'X)^-1 explodes, so small noise in data causes large swings in b1 and b2. Model
    fits training data, but coefficients are unstable under new data.
- id: 6ee0bc7c-6897-4163-86a5-b57fbc8fe3b6
  type: mcq
  front: Which model is most robust to multicollinearity?
  back: Trees make binary splits on one feature, so collinearity doesn't inflate variance. OLS and logistic
    regression require matrix inversion, which is ill-conditioned when features are correlated. L1/L2
    regularization can mitigate in linear models.
  choices:
  - key: a
    text: Ordinary Least Squares (OLS) linear regression
    correct: false
  - key: b
    text: Decision trees and random forests (split on one feature at a time)
    correct: true
  - key: c
    text: Logistic regression without regularization
    correct: false
  - key: d
    text: Naive Bayes
    correct: false
- id: 93bcc8db-2f1d-4032-83cb-59bbc6710469
  type: flip
  front: You have two features with correlation r=0.96. How would you decide which to drop?
  back: 'Consider: (1) domain relevance (which is more interpretable/actionable?), (2) missing values
    (drop the one with more NaNs), (3) external validity (which is more reliably measured?), (4) correlation
    with target (keep the one more correlated with y). In a pinch, compute VIF and drop the higher one.'
- id: 0854bdd7-17ed-49fb-b154-f8c2fa95452f
  type: flip
  front: What is the Variance Inflation Factor (VIF), and when is it problematic?
  back: 'VIF = 1 / (1 - R² from regressing feature i on all others). VIF > 5 (some say 10) is a red flag:
    feature i is predictable from other features, so its coefficient is unreliable. Iteratively remove
    highest-VIF features until all VIF < 5.'
- id: 9a3a732a-ba1d-4d55-ab97-91eb97039532
  type: mcq
  front: Does L1 regularization (Lasso) solve multicollinearity or just mask it?
  back: 'Lasso arbitrarily selects one of two correlated features (the other → 0). This is useful for
    sparsity and prediction, but doesn''t address the fundamental redundancy. Elastic Net (L1+L2) is more
    stable: L2 stabilizes collinear pairs, L1 shrinks.'
  choices:
  - key: a
    text: Solves it by forcing one coefficient to zero
    correct: false
  - key: b
    text: Masks it; Lasso shrinks one correlated feature arbitrarily while the other survives, reducing
      apparent collinearity but not the underlying redundancy
    correct: true
  - key: c
    text: Exacerbates it; L1 amplifies variance
    correct: false
  - key: d
    text: Neutral; Lasso has no effect on collinearity
    correct: false
---

## Intuition

Multicollinearity occurs when two or more features are highly correlated. In linear models, this inflates coefficient variance: small changes in data cause large coefficient swings, reducing stability and interpretability. A 0.95-correlated pair of features both contribute to predictions, but neither alone explains much variance. Removing one redundant feature simplifies the model without sacrificing predictive power. Tree-based models and L1-regularized models are more robust to multicollinearity, but linear models (OLS regression, logistic regression without regularization) suffer.

## Detail

**Variance Inflation Factor (VIF)**: For each feature, regress it on all others. VIF = 1 / (1 - R²). VIF > 5-10 signals problematic multicollinearity. Iteratively remove the feature with highest VIF until all VIF < 5.

**Correlation matrix**: Identify feature pairs with |r| > 0.95. Remove one from each pair (usually the one with lower domain relevance or external validity).

**Principal Component Analysis (PCA)**: Projects correlated features into orthogonal principal components. Solves multicollinearity but sacrifices interpretability.

**Domain-driven removal**: If two features are correlated because one is a derived/scaled version of the other, drop the redundant one (e.g., keep price_usd, drop price_eur if it's just a conversion).

```python
from statsmodels.stats.outliers_influence import variance_inflation_factor
import pandas as pd
import numpy as np

# Compute VIF for each feature
vif_data = pd.DataFrame()
vif_data['feature'] = X.columns
vif_data['VIF'] = [variance_inflation_factor(X.values, i) for i in range(X.shape[1])]

# Remove features with VIF > 5 iteratively
while (vif_data['VIF'] > 5).any():
    max_vif_idx = vif_data['VIF'].idxmax()
    X = X.drop(columns=vif_data.loc[max_vif_idx, 'feature'])
    vif_data = pd.DataFrame()
    vif_data['feature'] = X.columns
    vif_data['VIF'] = [variance_inflation_factor(X.values, i) for i in range(X.shape[1])]

# Correlation matrix cleanup
corr_matrix = X.corr().abs()
mask = np.triu(np.ones_like(corr_matrix, dtype=bool))
corr_pairs = corr_matrix.where(~mask).stack().reset_index()
corr_pairs.columns = ['Feature1', 'Feature2', 'Correlation']
high_corr = corr_pairs[corr_pairs['Correlation'] > 0.95]
```

## Common gotchas / interview framings
- "Why does multicollinearity hurt linear models more than tree models?"
- "How do you choose which feature to drop from a correlated pair?"
- "Does L1 regularization (Lasso) solve multicollinearity?"
- "When is it okay to keep correlated features (e.g., in an ensemble)?"

## See also
- [[multicollinearity]]
- [[variance-inflation-factor]]
- [[pearson-correlation]]
- [[linear-regression]]
- [[model-stability]]
- [[feature-redundancy]]

## Sources
See frontmatter `sources:`.
