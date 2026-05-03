---
id: bfde03e6-f1f2-4dc9-95af-2ee411812c24
title: Imbalanced data and oversampling/undersampling
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- class-imbalance
- resampling
- weighted-loss
- stratification
- data-augmentation
aliases:
- class imbalance
- weighted sampling
- resampling strategies
sources:
- url: https://imbalanced-learn.org/
  label: imbalanced-learn Library
- url: https://scikit-learn.org/stable/modules/generated/sklearn.utils.class_weight.compute_class_weight.html
  label: scikit-learn compute_class_weight
- url: https://arxiv.org/abs/1801.05936
  label: Class Weight Paper (Survey)
cards:
- id: 2ebe0450-a56e-4b63-8938-0ef2b168f06c
  type: flip
  front: Dataset is 95% negative, 5% positive. What's wrong with just training on raw data?
  back: Model achieves 95% accuracy by predicting majority ('negative'), ignoring minority. Gradient updates
    dominated by majority class. Minority misclassifications have negligible impact on loss.
- id: fc8136c1-1a2b-4d6a-acde-7aff3f87e9e6
  type: mcq
  front: What does SMOTE do?
  back: ''
  choices:
  - key: a
    text: Randomly duplicates minority samples
    correct: false
  - key: b
    text: Interpolates new minority samples along lines between existing samples
    correct: true
  - key: c
    text: Removes outliers
    correct: false
  - key: d
    text: Applies data augmentation
    correct: false
- id: ab91df66-ec32-47e4-a39c-1408e87014a0
  type: flip
  front: In imbalanced classification, why is weighted loss preferred over oversampling?
  back: 'Weighted loss doesn''t duplicate data (no information loss), trains faster, avoids overfitting
    on synthetic duplicates. Only downside: slightly higher gradient variance.'
- id: 54b2740c-1fb1-4d76-a83c-07aadfb92c79
  type: mcq
  front: When performing train-test split on imbalanced data, which approach prevents leakage?
  back: ''
  choices:
  - key: a
    text: Oversample, then split
    correct: false
  - key: b
    text: Split, then oversample only training data
    correct: true
  - key: c
    text: Use StratifiedKFold
    correct: false
  - key: d
    text: Undersample before splitting
    correct: false
---

## Intuition
When dataset is severely imbalanced (e.g., 99% negative, 1% positive), model ignores minority class—it's easier to achieve 99% accuracy by predicting majority. Two approaches: resample data (oversample minority, undersample majority) or weight loss (penalize minority misclassifications more).

## Detail
**Oversampling:** Duplicate minority samples or synthesize via SMOTE (interpolate between minority samples). Increases dataset size; risks overfitting on duplicates if minority class is tiny.

**Undersampling:** Randomly remove majority samples. Faster training; loses information. Avoid unless majority class is huge.

**Weighted Loss (Preferred):** Assign weight ∝ 1/class_frequency to each class. `nn.CrossEntropyLoss(weight=class_weights)`. More data-efficient than resampling; no information loss. Cost: gradient variance increases slightly.

**Stratified Sampling:** In train-test split, maintain class distribution. `StratifiedKFold` ensures each fold has similar class balance.

**Threshold Adjustment:** Even with balanced training, threshold for binary classification can be tuned post-hoc to trade recall vs precision.

## Common gotchas / interview framings
- Oversample before train-test split → leakage (synthetic samples appear in test)
- Undersampling too aggressive → throw away useful majority examples
- Forget class weights when imbalance > 10:1 → model ignores minority, metrics (accuracy) misleading
- Use accuracy as metric with imbalanced data → misleading; use F1, AUC-ROC, or precision-recall curve

## See also
- [[image-augmentation-pipelines-torchvision-albumentations]]
- [[batch-size-effects-on-generalization]]
- [[validation-strategy-and-metric-selection]]

## Sources
See frontmatter `sources:`.
