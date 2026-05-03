---
id: da989018-b8f9-439a-8630-9f41231d125a
title: Feature selection vs feature extraction
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- dimensionality-reduction
- feature-engineering
- interpretability
- overfitting
aliases:
- filter methods
- wrapper methods
- embedded methods
sources:
- url: https://scikit-learn.org/stable/modules/feature_selection.html
  label: scikit-learn Feature Selection
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: d60a2392-6973-4725-9fb9-13d0be3acb25
  type: flip
  front: Selection vs extraction trade-off?
  back: 'Selection: interpretable, preserves features. Extraction: flexible, hard-to-interpret. Choose
    selection if interpretability critical.'
- id: b926ee88-d9c1-406f-aa90-5211a065ad76
  type: flip
  front: 'Three selection methods: filter, wrapper, embedded?'
  back: 'Filter: univariate stats (fast). Wrapper: model performance search (slow, captures interactions).
    Embedded: model importance ([[LASSO]], trees, fast).'
- id: e0c81fe9-307e-42ff-9919-625ad4f11e55
  type: flip
  front: Selection WITHIN CV? Data leakage?
  back: 'If select before splitting, selection informed by test data. Correct: select on training set
    per CV fold, eval on validation.'
- id: b1bf9e8c-1726-479e-8082-483e35329300
  type: flip
  front: Selection vs extraction examples?
  back: 'Selection: medical diagnosis (keep interpretable features). Extraction: image classification
    (PCA reduces noise, captures variance).'
---

## Intuition
Selection: pick subset original features (interpretable). Extraction: create new features via combinations (flexible). Trade: selection interpretable/sparse, extraction flexible/opaque.

## Detail
**Filter:** Rank by univariate stats (correlation, MI). Fast, ignores interactions. **Wrapper:** Search subsets (forward/backward selection). Captures interactions, slow. **Embedded:** Feature importance from model ([[LASSO]], trees).

## See also
- [[filter-methods]]
- [[wrapper-methods]]
- [[embedded-methods]]
- [[lasso]]

## Sources
See frontmatter `sources:`.
