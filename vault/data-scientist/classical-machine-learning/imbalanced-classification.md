---
id: bfe40057-e181-4868-a325-536325e5997a
title: Imbalanced classification
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- classification
- class-imbalance
- resampling
- evaluation-metrics
aliases:
- minority class
- oversampling
- undersampling
- SMOTE
sources:
- url: https://scikit-learn.org/stable/modules/model_evaluation.html
  label: scikit-learn Metrics
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 02784e62-6e66-4a4e-89e5-772095e16317
  type: flip
  front: Why accuracy poor on imbalance? Use instead?
  back: Accuracy=(TP+TN)/total. 99% negative → predict all negative = 99%. Use precision/recall/F1/PR-AUC.
- id: 3eb9703b-ee7e-47f4-af08-617019281869
  type: flip
  front: Class weighting `balanced` mechanism?
  back: $w_c = n_{total}/(n_{classes}·n_c)$. Minority higher weight; misclassification penalized more.
    Shifts boundary.
- id: 12d73754-f951-497a-861f-c69afeef7cc0
  type: flip
  front: SMOTE vs oversampling?
  back: 'SMOTE: $x_{new} = x_i + \alpha(x_j - x_i)$ (k-NN neighbor). Avoids duplication, reduces overfit.
    Oversample: duplicate (risk: overfit). Apply SMOTE only to training set.'
- id: 770a15fd-321a-407f-9544-d8f56a34528a
  type: flip
  front: 'Correct procedure: resampling + CV?'
  back: 'StratifiedKFold. Each fold: (1) SMOTE only on train, (2) train, (3) eval on validation. Never
    resample before splitting (data leakage).'
---

## Intuition
Imbalanced data (99% negative) makes accuracy useless. Fix: appropriate metrics, reweight classes, or resample.

## Detail
**Reweighting:** `class_weight='balanced'` (auto: $w_c \propto 1/n_c$). **Resampling:** Oversample minority (risk: overfit), undersample majority (risk: info loss), SMOTE (synthesize). **Metrics:** Precision, recall, F1, PR-AUC. **Stratified splits:** Preserve distribution.

## See also
- [[stratified-sampling]]
- [[smote]]
- [[class-weights]]

## Sources
See frontmatter `sources:`.
