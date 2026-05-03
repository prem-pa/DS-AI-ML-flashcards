---
id: 54bf3010-699c-45c5-961c-f1aa965c2da6
title: Cross-validation (k-fold, stratified)
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- evaluation
- validation
- hyperparameter-tuning
- stability
aliases:
- k-fold CV
- stratified k-fold
- leave-one-out CV
sources:
- url: https://scikit-learn.org/stable/model_selection.html
  label: scikit-learn Model Selection
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 39f17e16-f6c7-499e-8b4e-3469c4baf6aa
  type: flip
  front: k-fold CV. Why more stable than holdout?
  back: 'Split into $k$ folds, average $k$ validation errors. All data used for train+val. Error estimate:
    average of $k$ estimates, lower variance than single holdout.'
- id: b397b3e2-6222-4727-9d20-734937362432
  type: flip
  front: Stratified k-fold CV. When use?
  back: Classification with imbalance. Each fold preserves class distribution. Avoids one fold 99% negative,
    another 50%.
- id: b816e618-ace4-410f-a53c-4b65c527b2af
  type: flip
  front: Nested CV hyperparameter tuning?
  back: 'Outer: final error estimate (5 folds). Inner: hyperparameter selection (3 folds on training of
    outer). Prevents data leakage.'
- id: 9945889b-02e0-473e-83f2-5b4aa5bc6ed1
  type: flip
  front: Feature selection within CV? Leakage?
  back: 'If select before CV, selection informed by all data. Correct: select on training folds only per
    CV iteration. Independent of test fold.'
---

## Intuition
Repeat split: train on $k-1$ folds, validate on 1 fold. Average $k$ errors. More stable than single split. Stratified CV preserves class distribution per fold.

## Detail
**k-Fold:** Divide into $k$ folds, rotate. **Stratified:** Each fold has same class distribution. **LOOCV:** $k=n$ (expensive). **Time-series:** Respect temporal order.

## See also
- [[trainvalidationtest-splits]]
- [[model-selection]]
- [[variance-reduction]]

## Sources
See frontmatter `sources:`.
