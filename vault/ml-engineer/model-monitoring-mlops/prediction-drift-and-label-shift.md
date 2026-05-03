---
id: aff03060-a19a-4493-ae15-05732561aa33
title: Prediction drift and label shift
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 5
tags:
- drift-detection
- distribution-shift
- covariate-shift
- label-shift
- divergence-metrics
- monitoring
aliases:
- covariate shift
- output distribution shift
- prior shift
sources:
- url: https://www.evidentlyai.com/ml-in-production/data-drift
  label: 'Evidently AI: What is data drift and how to detect and handle it'
- url: https://www.datacamp.com/tutorial/understanding-data-drift-model-drift
  label: 'DataCamp: Understanding Data Drift and Model Drift'
cards:
- id: 7768c711-ff49-450d-bfb7-4dbdc689a55a
  type: flip
  front: Explain the difference between prediction drift (covariate shift) and label shift. Why is label
    shift harder to detect in production?
  back: '**Prediction drift (covariate shift)**: P(X) changes but P(Y|X) stable. Example: customer age
    distribution shifts. Detection: Monitor feature distributions vs training baseline.


    **Label shift**: P(Y) changes but P(Y|X) stable. Example: fraud rate drops 5% → 0.5%. Harder to detect
    because you see fewer positives, but P(fraud|features) unchanged. Requires:

    - Delayed labels (hard in real-time systems)

    - Label-free signals: prediction confidence collapse, model disagreement

    - Indirect: monitor model''s predicted class distribution drift separately from features'
- id: 494623c8-f976-4e83-9396-07ced8fa8c6a
  type: mcq
  front: You compute PSI=0.15 on model input features between train and production. Accuracy drops from
    0.94 → 0.89. What is the most likely explanation?
  back: '**Correct: b** PSI=0.15 (moderate drift) + accuracy ↓ 5pts suggests the model''s decision boundary
    is sensitive to the shifted feature distribution. This is covariate shift where P(Y|X) was not stable
    enough. If drift were purely label shift (P(Y) changed), input features stable, you''d see different
    symptoms: miscalibration, not uniform accuracy loss.'
  choices:
  - key: a
    text: PSI=0.15 indicates moderate drift, but accuracy drop is due to label shift (P(Y) changed), not
      input drift
    correct: false
  - key: b
    text: Covariate shift (P(X) changed) + non-stable conditional P(Y|X); model was brittle to distribution
      changes
    correct: true
  - key: c
    text: 'Concept drift: the relationship between X and Y completely reversed'
    correct: false
  - key: d
    text: Model is overfitting; PSI is irrelevant
    correct: false
- id: 588b0b09-1091-475f-87d8-b7f13ca275f4
  type: flip
  front: 'You compute KL divergence on 10 features. 9 have KL < 0.05, but Feature #7 (rare event flag,
    1% positive) has KL = 0.8. Why is this spurious and what should you do?'
  back: 'KL divergence is **sensitive to zero/rare bins**. Feature #7 with rare events: if train has 1
    occurrence and prod has 0, KL explodes even if the meaningful distribution is stable.


    **Solution**: (1) Use PSI instead (symmetric, less sensitive). (2) Apply Laplace/add-one smoothing:
    P_smooth = (count + 1) / (total + k). (3) Use binned KL (group rare events). (4) Set KL thresholds
    higher for rare features (e.g., KL < 0.2 for features < 5% support).'
- id: 10b0ad82-9150-44a6-b2d7-7594c855f5ed
  type: flip
  front: 'In a recommendation system, your model''s predicted CTR (click-through rate) distribution shifts:
    train mean=0.15, prod mean=0.08. But feature distributions are stable. What happened and how do you
    confirm?'
  back: 'This suggests **label shift**: P(click) dropped from 15% → 8% in prod (e.g., users are less engaged,
    or you changed ranking). Features unchanged (covariate shift ≠ 0).


    Confirm: (1) Check if delayed ground truth labels show P(Y) actually changed. (2) If labels unavailable,
    compare model''s predicted distribution shift vs feature drift—prediction shift >> feature drift =
    label shift signal. (3) Monitor calibration: if model was well-calibrated in train, post-shift performance
    should stay ~0.08 (label shift alone won''t break calibration), but accuracy on 8% base rate will
    look worse than on 15% base rate.'
---

## Intuition
Your model trains on data distribution P(X, Y) in development. In production, the distribution shifts to Q(X, Y). **Prediction drift** = input distribution Q(X) ≠ P(X). **Label shift** = P(Y|X) stays same but P(Y) changes. Covariate shift: P(Y|X) unchanged but Q(X) ≠ P(X). These orthogonal shifts degrade performance in different ways.

## Detail
**Prediction drift**: Features shift (e.g., user income drops in recession). Train on 2020 customers; predict 2024 customers. Detected via PSI (Population Stability Index) or KL divergence on feature distributions. KL divergence: D_KL(Q||P) measures bits of extra info needed to encode Q using P's codebook; asymmetric, sensitive to tail. PSI: symmetric relative entropy divergence; preferred in practice (less sensitive to rare bins).

**Label shift**: P(Y) changes (e.g., fraud rate drops due to fraud prevention). True label distribution P(Y) assumed stable in many algorithms, but in fraud, P(fraud)=5% in train, 0.5% in prod. Causes miscalibration. Detect by monitoring pred distribution drift independently from feature drift. Label shift with covariate shift is hardest to detect without labels.

## Common gotchas / interview framings
- Assuming P(X) drift always harms model—covariate shift may have no impact if P(Y|X) stable
- KL divergence sensitive to zero frequencies; requires smoothing (add-one smoothing, Laplace)
- Confusing prediction shift (monitored via input features) with model performance degradation
- Label shift invisible without delayed labels; suggests label-free monitoring (e.g., prediction confidence drop)

## See also
- [[feature-distribution-monitoring]]
- [[model-performance-degradation-accuracy-drop-calibration-shift]]
- [[data-freshness]]
- [[retraining-triggers-periodic-drift-based-performance-based]]

## Sources
See frontmatter `sources:`.
