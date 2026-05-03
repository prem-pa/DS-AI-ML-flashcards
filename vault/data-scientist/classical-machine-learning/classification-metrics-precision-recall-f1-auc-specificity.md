---
id: 1895a5da-9262-4ff9-b7c0-fc282c953cba
title: Classification metrics (precision, recall, F1, AUC, specificity)
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- classification
- evaluation
- imbalanced-data
- business-metrics
aliases:
- confusion matrix metrics
- sensitivity
- PPV
- ROC-AUC
sources:
- url: https://scikit-learn.org/stable/modules/model_evaluation.html
  label: scikit-learn Metrics
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 55c15f03-cec7-41a9-bb38-9b55851e001b
  type: flip
  front: Define precision, recall. Tradeoff? When each critical?
  back: 'Precision=TP/(TP+FP). Recall=TP/(TP+FN). High precision: few FP (fraud). High recall: few FN
    (disease). F1 balances.'
- id: f24d1550-314d-44dc-880f-f7c8c9f72924
  type: flip
  front: What AUC measure? Differ from accuracy?
  back: 'AUC: rank ability (positives > negatives), threshold-agnostic. 0.5=random, 1=perfect. Accuracy:
    single threshold fraction correct. AUC robust to threshold.'
- id: 1cf9cf37-42e9-4daa-8f3f-ba52d81b7a67
  type: flip
  front: Why PR-AUC > ROC-AUC on imbalance?
  back: 'ROC includes FPR=FP/(TN+FP). Imbalanced: TN huge, FPR small, easy TNR. PR focuses on minority
    class (TP, FP, FN).'
- id: 36c52ac4-1dae-41c1-b403-10e8c7b0051e
  type: mcq
  front: 98% accuracy, 99% negative class. Good?
  back: Dummy classifier (always negative) = 99% accuracy. Use precision/recall/F1 on minority.
  choices:
  - key: a
    text: Excellent.
    correct: false
  - key: b
    text: No; likely predicts all negatives. Use P/R/F1.
    correct: true
  - key: c
    text: Depends on features.
    correct: false
  - key: d
    text: Accuracy always best.
    correct: false
---

## Intuition
Accuracy misleading on imbalance. Precision (avoid FP), recall (avoid FN) capture different costs. AUC ranks; F1 balances both.

## Detail
**Precision:** $TP/(TP+FP)$. **Recall:** $TP/(TP+FN)$. **Specificity:** $TN/(TN+FP)$. **F1:** $2 \cdot P \cdot R / (P+R)$. **AUC-ROC:** Area under TPR vs FPR. **PR-AUC:** Better on imbalance.

## See also
- [[confusion-matrix]]
- [[roc-curve]]
- [[precision-recall-curve]]
- [[threshold-tuning]]

## Sources
See frontmatter `sources:`.
