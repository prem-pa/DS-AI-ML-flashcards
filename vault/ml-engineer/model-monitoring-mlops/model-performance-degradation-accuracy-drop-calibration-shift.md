---
id: 365a8cff-d79d-49a9-b7a7-0c85b5149a7b
title: Model performance degradation (accuracy drop, calibration shift)
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 3
tags:
- performance-monitoring
- calibration
- validation-monitoring
- accuracy-tracking
- metric-degradation
- alerting
aliases:
- accuracy monitoring
- model performance tracking
- drift-driven degradation
sources:
- url: https://www.peerspot.com/categories/model-monitoring
  label: 'PeerSpot: Best Model Monitoring solutions 2026'
cards:
- id: e0d68a5f-a970-430e-bdc4-9f61acbb6341
  type: flip
  front: Your production model's AUC drops from 0.94 → 0.87 over two weeks. Feature distributions are
    stable. What are your first three investigation steps?
  back: '1. **Check for delayed labels**: Compare predictions from 2 weeks ago vs ground truth arriving
    now. If labels confirm AUC drop, it''s real performance degradation, not monitoring artifact.


    2. **Analyze by segment**: Does AUC drop uniformly or concentrate in specific subpopulations (e.g.,
    new users, specific region)? Label shift or covariate shift in subgroup.


    3. **Inspect calibration**: Is model overconfident (predicted 0.85, actual 0.65 win rate)? If yes,
    label shift is likely. If calibration stable, concept drift (P(Y|X) changed) or data quality issue
    (feature corruption).


    Next: Check data pipeline (missing values, schema changes), feature freshness, or trigger retraining.'
- id: c50d1468-5203-450d-9112-dc523f774866
  type: mcq
  front: Expected Calibration Error (ECE) increases from 0.05 → 0.18, but accuracy only drops from 0.92
    → 0.90. Prediction distribution is stable. What happened?
  back: '**Correct: b** Label shift scenario: train on P(Y)=0.5 balanced, prod P(Y)=0.8 (skewed). Model
    trained to output 0.5 confidence on average stays calibrated for 50-50 split. On 80-20 split, outputs
    (e.g., 0.5 confidence 100 times, but 80 are positive) = poorly calibrated despite maintaining accuracy
    (~2% drop in this scenario). ECE explodes. Fix: recalibrate (Platt scaling) or retrain with new label
    distribution.'
  choices:
  - key: a
    text: 'Concept drift: P(Y|X) changed'
    correct: false
  - key: b
    text: 'Label shift: P(Y) changed, model overconfident on new label distribution'
    correct: true
  - key: c
    text: Feature drift in input distribution
    correct: false
  - key: d
    text: Model overfitting on training data
    correct: false
- id: 6e5dc9b7-6f6e-4232-b571-b35bf3a4fd7d
  type: flip
  front: Design a monitoring system that detects accuracy drop without delayed ground truth labels. What
    signals compose your alert?
  back: '**Label-free performance monitoring**:


    1. **Prediction distribution drift**: Compute PSI on predicted class distribution. Sudden shift may
    signal label shift or concept drift.


    2. **Feature-model disagreement**: Train simple GLM on features; compare GLM predictions vs model
    predictions. High disagreement = model overfitting or concept drift.


    3. **Confidence degradation**: Monitor mean predicted confidence. Drop suggests model uncertainty
    increasing (data OOD or concept drift).


    4. **Prediction-feature correlation**: If feature X ↔ prediction correlation flips, P(Y|X) likely
    changed.


    5. **Hard negatives**: Track predictions on edge cases (low confidence, near decision boundary). If
    error rate on these spikes, concept drift likely.


    Combine via composite alerting: alert if (PSI > 0.15 AND confidence_drop > 10%) OR (feature_model_disagreement
    > 15%).'
- id: d6553882-8ddf-43a3-98df-688e814c9f13
  type: flip
  front: You detect AUC ↓ 5%, but micro-average F1 stable. Why might this happen and what should you investigate?
  back: 'AUC vs F1 mismatch suggests **class imbalance shift or minority-class performance collapse**.
    AUC (ROC curve) robust to class imbalance; F1 (precision-recall) sensitive to positives.


    Scenario: Positive rate P(Y=1) drops from 10% → 2%. Model''s recall on positives may drop 0.95 → 0.80.
    AUC ↓ (overall ranking quality), but if negatives still ranked well, F1 may stay stable (fewer positives
    to recall).


    Investigate: (1) Segment metrics by class; check recall, precision separately. (2) Monitor minority-class
    AUC separately. (3) Check for label shift in minority class. (4) If critical business metric depends
    on recall, retrain with new class balance or use threshold adjustment.'
---

## Intuition
Even if latency/throughput stay stable, model accuracy decays in production. **Accuracy drop**: F1, AUC, or top-1 accuracy ↓. **Calibration shift**: predicted confidence diverges from true frequency. A model predicting 0.9 confidence 100 times should be correct ~90 times; if only 60 correct, calibration drifted. These indicate data/concept drift, and require intervention (retrain, alert).

## Detail
Monitor two scenarios: (1) **Labeled**: Hold ground truth validation set; regularly score prod predictions against delayed labels. Expensive but precise. (2) **Label-free**: Monitor proxy signals (feature drift, prediction distribution shift, model disagreement ensemble). For calibration, track: ECE (expected calibration error, bin predictions, compare confidence vs accuracy), Brier score (MSE of predicted prob vs binary outcome), or log-loss. Thresholds: alert if accuracy ↓ >3%, ECE ↑ >0.1. Root cause: drift, label shift, feature engineering bug, or data pipeline corruption.

## Common gotchas / interview framings
- Confusing performance degradation with label noise—clean labels but distribution shifted
- Using only test-set accuracy; need continuous monitoring with holdout validation set or delayed labels
- Ignoring class imbalance; accuracy can stay stable while minority class F1 collapses
- Not separating calibration drift from accuracy drift; well-calibrated model can have lower accuracy on new data distribution

## See also
- [[latency-throughput-error-rates]]
- [[prediction-drift-and-label-shift]]
- [[feature-distribution-monitoring]]
- [[retraining-triggers-periodic-drift-based-performance-based]]

## Sources
See frontmatter `sources:`.
