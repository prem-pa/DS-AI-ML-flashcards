---
id: 95fce89e-cb8d-4cc7-bec6-2def1f091ad6
title: Confusion matrix and per-class performance
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 3
tags:
- confusion matrix
- multiclass
- per-class metrics
- F1
- macro
- weighted
aliases:
- error matrix
- TP/FP/TN/FN
- class imbalance
sources:
- url: https://developers.google.com/machine-learning/crash-course/classification/accuracy-precision-recall
  label: 'Google ML Crash Course: Classification Metrics'
- url: https://www.evidentlyai.com/classification-metrics
  label: 'Evidently AI: Classification Metrics'
cards:
- id: abfd70b0-dd96-4608-902c-e60782960bdc
  type: flip
  front: For a 3-class problem, what does a confusion matrix entry C[i,j] represent?
  back: 'C[i,j] = number of samples with true class i that were predicted as class j. Diagonal C[i,i]
    = correct predictions for class i. Off-diagonal entries show which classes are confused. Example:
    C[1,2]=15 means 15 class-1 samples mispredicted as class-2.'
- id: b90c68ff-ed35-4cc3-8ce8-086fb39a9fdc
  type: flip
  front: 'Your model achieves 93% accuracy on multiclass data. Per-class F1: {Class A: 0.95, Class B:
    0.92, Class C: 0.10}. Diagnose.'
  back: 'Class C (minority) is failing. Likely causes: (1) severe imbalance (C rare), (2) overlapping
    feature space with other classes, (3) insufficient training data for C. Check confusion matrix to
    see if C samples are always mispredicted as A or B. Solution: class-weighted loss, oversampling minority,
    or stratified cross-validation.'
- id: bf227884-2dd9-4ee7-a9ce-6a41649d78e1
  type: mcq
  front: When should you report macro-F1 vs weighted-F1 for imbalanced multiclass?
  back: Report both. Macro treats all classes equally—highlights minority performance. Weighted averages
    by support—reflects real-world distribution. For medical diagnosis (rare disease), use macro. For
    product classification (imbalanced labels), use weighted to match business impact.
  choices:
  - key: a
    text: Always weighted
    correct: false
  - key: b
    text: Always macro
    correct: false
  - key: c
    text: Report both; macro if minority matters, weighted if overall accuracy matters
    correct: true
  - key: d
    text: Neither; use accuracy only
    correct: false
- id: 9be9e9f9-85db-4fa5-b2dd-7e79a79c023d
  type: flip
  front: A confusion matrix shows class A is confused with class B frequently. What does this signal?
  back: 'Class A and B likely share similar feature distributions. Options: (1) collect more discriminative
    features, (2) inspect mislabeled training samples (label noise), (3) increase model capacity to learn
    decision boundary, (4) merge classes if they''re truly similar. Confusion matrix directs data collection
    and feature engineering.'
- id: b39e627b-81c8-4dc3-be69-928cb5fec029
  type: flip
  front: 'Compute macro-F1 and weighted-F1 given: Class A (support=90): F1=0.90, Class B (support=10):
    F1=0.50.'
  back: Macro-F1 = (0.90 + 0.50)/2 = 0.70. Weighted-F1 = (0.90×90 + 0.50×10) / (90+10) = (81 + 5) / 100
    = 0.86. Macro penalizes class B failure more; weighted emphasizes A's dominance. Choose macro to highlight
    minority problems in rare-class scenarios.
---

## Intuition
Confusion matrix breaks down predictions by true and predicted class. For multiclass (K>2), it's a K×K matrix: rows = true, columns = predicted. Per-class metrics reveal imbalances: e.g., model may excel on majority class but fail on minority. Macro/weighted averaging aggregates per-class metrics with different strategies.

## Detail
Binary confusion matrix:
```
           Predicted Pos  Predicted Neg
Actual Pos       TP              FN
Actual Neg       FP              TN
```

Multiclass K×K matrix: entry (i,j) = # samples true class i predicted as j. Diagonal = correct predictions.

Per-class metrics for class k (one-vs-rest):
- **TP_k**: true class k predicted as k
- **FP_k**: class ≠k predicted as k
- **FN_k**: class k predicted as ≠k
- **Precision_k** = TP_k/(TP_k + FP_k)
- **Recall_k** = TP_k/(TP_k + FN_k)
- **F1_k** = 2·Precision_k·Recall_k/(Precision_k + Recall_k)

**Macro averaging**: unweighted mean of per-class metrics (treats rare classes equally). **Weighted averaging**: average weighted by class support (class frequency).

Example 3-class: classes A(100), B(100), C(10). Macro F1 weights all equal; weighted F1 weights A,B 10x more than C. Macro highlights minority problems; weighted reflects overall accuracy.

## Common gotchas / interview framings
- Imbalanced multiclass: macro-F1 sensitive to minority class errors, weighted-F1 can hide minority problems
- Diagonal obsession: high accuracy without checking confusion matrix; off-diagonal reveals specific confusions
- Per-class variance: class k may have precision 0.95 but recall 0.30 (confident but misses many)
- Averaging choice: report both macro and weighted; clarify which for stakeholder context
- Minority class collapse: class with 1% support can collapse to near-zero support in predictions; confusion matrix exposes this

## See also
- [[confusion-matrix]]
- [[multiclass-classification]]
- [[macro-averaging]]
- [[weighted-averaging]]
- [[class-specific-error]]

## Sources
See frontmatter `sources:`.
