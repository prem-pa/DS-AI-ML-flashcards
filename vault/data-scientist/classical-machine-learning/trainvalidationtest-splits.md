---
id: ba9cebcd-bc77-4c8a-8350-2c86c2baa05e
title: Train/validation/test splits
track: data-scientist
topic: classical-machine-learning
difficulty: 1
tags:
- evaluation
- validation
- hyperparameter-tuning
- generalization
aliases:
- hold-out set
- development set
- test set
sources:
- url: https://scikit-learn.org/stable/model_selection.html
  label: scikit-learn Model Selection
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 40330071-34ec-4446-9739-93c410dccf2b
  type: flip
  front: 'Roles: train, validation, test. Tune on test?'
  back: 'Train: fit. Validation: tune hyperparameters. Test: estimate generalization. Tuning on test=data
    leakage; test estimate becomes optimistic.'
- id: 0cc21596-688a-4592-b0bf-5b636b212c4d
  type: flip
  front: Stratified splitting utility?
  back: Preserve class distribution across splits. Use for imbalanced classification. Avoids one split
    99% majority, another 50%.
- id: c022f9fb-efe2-4f0d-8677-9285db23c4fa
  type: flip
  front: Retrain on train+validation after selection?
  back: Yes. After selecting hyperparameters, combine train+validation, retrain. Use all available data
    (except test). Eval on test.
- id: a9bfd51b-35d9-41e6-9087-00ae86b2e346
  type: flip
  front: Data leakage examples?
  back: '(1) Scale entire dataset before splitting. (2) Tune on test. (3) Remove outliers detected on
    test. Result: optimistic estimates.'
---

## Intuition
Train: fit parameters. Validation: tune hyperparameters. Test: final evaluation. Never use test for tuning (data leakage).

## Detail
**Split:** 60-70% train, 15-20% validation, 15-20% test. **Workflow:** (1) split, (2) tune on train/validation, (3) retrain on train+validation, (4) evaluate test. **Stratified:** Preserve class distribution.

## See also
- [[hyperparameter-tuning]]
- [[overfitting]]
- [[cross-validation]]

## Sources
See frontmatter `sources:`.
