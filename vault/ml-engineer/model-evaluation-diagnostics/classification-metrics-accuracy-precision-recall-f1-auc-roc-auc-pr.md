---
id: ad1cf751-f9ba-4653-94dd-f6ad9b0a637a
title: Classification metrics (accuracy, precision, recall, F1, AUC-ROC, AUC-PR)
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 3
tags:
- classification
- metrics
- threshold
- imbalance
- ROC
- AUC-PR
aliases:
- precision-recall tradeoff
- ROC curve
- threshold selection
sources:
- url: https://www.deepchecks.com/f1-score-accuracy-roc-auc-and-pr-auc-metrics-for-models/
  label: 'DeepChecks: F1, Accuracy, ROC-AUC & PR-AUC Metrics'
- url: https://developers.google.com/machine-learning/crash-course/classification/accuracy-precision-recall
  label: 'Google ML Crash Course: Classification Metrics'
- url: https://www.aiclouddatapulse.com/auc-vs-f1-score/
  label: 'AUC vs F1 Score: Which Metric to Use'
- url: https://www.evidentlyai.com/classification-metrics
  label: 'Evidently AI: Classification Metrics Guide'
cards:
- id: 5d0f6123-b0e7-4cd7-8b58-8c90feef9fb4
  type: flip
  front: When should you use AUC-PR instead of AUC-ROC?
  back: AUC-PR is preferred when the positive class is rare (severe class imbalance <5%). ROC-AUC can
    be misleading because it's dominated by the majority class and doesn't reflect precision-recall tradeoff
    that matters for rare events. In fraud detection, disease screening, or anomaly detection, AUC-PR
    gives clearer performance signal.
- id: 562b160e-bece-4809-b011-8992cf48bcbd
  type: flip
  front: What's the relationship between F1 and F-beta? Give an example where F2 > F1.
  back: 'F-beta = $(1+\beta^2) \cdot \frac{P \cdot R}{\beta^2 \cdot P + R}$. F1 ($\beta=1$) weights precision
    and recall equally. F2 ($\beta=2$) weights recall 2x higher. Use F2 in medical diagnosis (missed positives
    costly), F0.5 in spam filtering (false positives costly). Example: Recall=0.95, Precision=0.6 → F1≈0.74,
    F2≈0.86.'
- id: 30d7b0dc-5dbe-4110-89f5-5e28c716407f
  type: mcq
  front: Your model predicts 98% negative class. Accuracy is 98%. What's likely true?
  back: High accuracy with imbalanced data is meaningless. If positive class is 2%, predicting 'negative'
    always gives 98% accuracy. Recall (TP/actual positives) is likely near zero because the model rarely
    flags positives.
  choices:
  - key: a
    text: Model is excellent
    correct: false
  - key: b
    text: Model always predicts negative (98% base rate)
    correct: false
  - key: c
    text: Recall is probably very low
    correct: false
  - key: d
    text: Both B and C
    correct: true
- id: 1423a3a8-5b9e-4e54-90cd-7f342c09735f
  type: flip
  front: Explain the precision-recall tradeoff. Which metric changes as you move the threshold?
  back: 'As you lower classification threshold (more lenient), more samples become positive: TP and FP
    both increase. Recall (TP/(TP+FN)) increases, Precision (TP/(TP+FP)) decreases. AUC-PR/AUC-ROC curves
    capture this tradeoff. Single threshold optimization (e.g., F1) picks one point; AUC is threshold-agnostic.'
- id: aae9037a-a2eb-4e74-8222-f9e7932c8719
  type: mcq
  front: In imbalanced binary classification, which metric is most vulnerable to misleading conclusions?
  back: AUC-ROC can remain high (0.85+) even when model performs poorly on the rare class, because it
    averages across all FPR values. Majority-class negatives dominate the curve. AUC-PR is more honest
    about positive-class performance.
  choices:
  - key: a
    text: Precision
    correct: false
  - key: b
    text: Recall
    correct: false
  - key: c
    text: AUC-ROC
    correct: true
  - key: d
    text: AUC-PR
    correct: false
---

## Intuition
Classification metrics quantify prediction quality across different dimensions. Accuracy counts correct predictions but misleads on imbalanced data. Precision/Recall isolate false positives vs false negatives, enabling cost-aware threshold tuning. AUC-ROC measures ranking ability across thresholds; AUC-PR focuses on the positive class when rare.

## Detail
For confusion matrix $[TP, FP, FN, TN]$:
- **Accuracy** = $(TP+TN)/(TP+FP+FN+TN)$ → fails when classes imbalanced
- **Precision** = $TP/(TP+FP)$ → minimize false alarms
- **Recall** = $TP/(TP+FN)$ → minimize missed positives
- **F1** = $2 \cdot (Precision \cdot Recall)/(Precision + Recall)$ → harmonic mean, single-threshold summary
- **F-beta** = $(1+\beta^2) \cdot (Precision \cdot Recall)/(\beta^2 \cdot Precision + Recall)$ → weight recall ($\beta>1$) or precision ($\beta<1$)
- **AUC-ROC** = area under curve plotting TPR vs FPR across thresholds; good for balanced data, can be insensitive under severe imbalance
- **AUC-PR** = area under Precision-Recall curve; clearer signal for rare positive class

ROC-AUC misleads when: (1) extreme class imbalance (positive class <1%), (2) cost asymmetry (FN cost >> FP cost), (3) need interpretability at a single threshold.

## Common gotchas / interview framings
- Accuracy trap: 95% accuracy on 99%-negative data = always predict negative
- ROC AUC false confidence: AUC=0.9 doesn't mean precision=0.9; depends on threshold
- Threshold selection: moving threshold trades precision↔recall; F1 is point estimate, AUC is curve
- F-beta weighted: F2 emphasizes recall (finding anomalies), F0.5 emphasizes precision (avoiding false alarms)
- AUC-PR vs AUC-ROC: prefer PR-AUC when positive class is rare; ROC AUC is inflated by easy-to-classify negatives

## See also
- [[receiver-operating-characteristic]]
- [[precision-recall-curve]]
- [[class-imbalance]]
- [[threshold-tuning]]
- [[binary-classification]]

## Sources
See frontmatter `sources:`.
