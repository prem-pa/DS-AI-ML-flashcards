---
id: 1351dc14-d6a0-46b0-bc5b-8234e1ee19de
title: Confusion matrix and threshold selection
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- classification
- evaluation
- threshold-tuning
- ROC-curve
aliases:
- decision threshold
- ROC curve
- cost-sensitive classification
sources:
- url: https://scikit-learn.org/stable/modules/model_evaluation.html
  label: scikit-learn Metrics
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 14e71f7b-09fa-4fba-b6ae-a312a04ac92c
  type: flip
  front: Confusion matrix cells. TPR vs specificity relation?
  back: 'Matrix: [[TN,FP],[FN,TP]]. TPR=TP/(TP+FN). Specificity=TN/(TN+FP)=1-FPR. ROC: TPR vs (1-Specificity).'
- id: 0fb8dd85-6c80-4f44-af12-2cfb434a88b0
  type: flip
  front: ROC curve as threshold varies?
  back: 'threshold=1: TPR=0,FPR=0 (bottom-left). Decreases: both TPR,FPR increase. threshold=0: TPR=1,FPR=1
    (top-right). AUC = integral.'
- id: 834c4209-c718-49bd-a29a-a8d4666e66f8
  type: flip
  front: FN cost 10× FP. Raise or lower threshold?
  back: Lower. More positive predictions → higher TPR (catch positives, reduce FN) → higher FPR. Cost=$10·FN+FP$
    favors low FN.
- id: ec48cbe4-ca28-4f35-91c2-bfba59233f2f
  type: flip
  front: Why PR curve > ROC on imbalance?
  back: 'ROC: FPR=FP/(FP+TN). Imbalanced: TN huge, FPR small. PR focuses on TP,FP,FN (minority).'
---

## Intuition
Confusion matrix shows all four outcomes. ROC curve visualizes FP/FN tradeoff across thresholds. Default 0.5 is arbitrary.

## Detail
**Confusion matrix:** TN, FP, FN, TP. **ROC:** TPR vs FPR. **Threshold tuning:** High FP cost → raise; high FN cost → lower.

## See also
- [[precision-recall-tradeoff]]
- [[roc-curve]]
- [[business-metrics]]

## Sources
See frontmatter `sources:`.
