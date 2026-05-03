---
id: 1cc47f54-2e9e-4ce2-bb07-8fc3352a0c24
title: Data normalization (z-score, min-max)
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 1
tags:
- normalization
- standardization
- preprocessing
- feature-scaling
- numerical-stability
aliases:
- feature scaling
- standardization
- z-score normalization
sources:
- url: https://scikit-learn.org/stable/modules/preprocessing.html
  label: scikit-learn StandardScaler & MinMaxScaler
- url: https://pytorch.org/vision/stable/transforms.html
  label: torchvision.transforms.Normalize
- url: https://en.wikipedia.org/wiki/Feature_scaling
  label: 'Wikipedia: Feature Scaling'
cards:
- id: 11dde9aa-0739-47db-8428-99e6094f2851
  type: flip
  front: Why normalize input features before training?
  back: Unnormalized inputs cause poor conditioning of the loss landscape—features with large magnitudes
    dominate gradients, slowing convergence. Normalization flattens the landscape, enabling faster learning
    with smaller learning rates.
- id: 44188cc0-a902-4462-a737-3ef337592aaf
  type: mcq
  front: What is z-score normalization formula?
  back: ''
  choices:
  - key: a
    text: (x - min) / (max - min)
    correct: false
  - key: b
    text: (x - mean) / std
    correct: true
  - key: c
    text: x / ||x||
    correct: false
  - key: d
    text: log(x)
    correct: false
- id: 457146b6-41e7-4a64-a1d8-9857afda3296
  type: flip
  front: You're using a pretrained ResNet (ImageNet weights). What normalization should you apply?
  back: 'ImageNet normalization: `mean=[0.485, 0.456, 0.406], std=[0.229, 0.224, 0.225]` per RGB channel.
    Pretrained weights expect these statistics.'
- id: 70b484ae-3162-48f2-827d-437a90dd3ce4
  type: mcq
  front: Min-max scaling is sensitive to which problem?
  back: ''
  choices:
  - key: a
    text: Large batch sizes
    correct: false
  - key: b
    text: Outliers shifting max/min
    correct: true
  - key: c
    text: Negative values
    correct: false
  - key: d
    text: Non-square matrices
    correct: false
---

## Intuition
Neural networks converge faster when input features are centered (mean ~0) and scaled (std ~1). Unnormalized inputs with wildly different magnitudes cause steep loss landscapes and slow learning. Normalization flattens the landscape.

## Detail
**Z-score (Standardization):** `(x - mean) / std`. Centers features to mean=0, scales to std=1. Best for normally distributed features. Assumes Gaussian distribution; outliers can inflate std.

**Min-Max Scaling:** `(x - min) / (max - min)`. Bounds features to [0, 1]. Sensitive to outliers; if max/min change at test time, breaks calibration.

**ImageNet Normalization:** Images normalized per channel: `mean=[0.485, 0.456, 0.406], std=[0.229, 0.224, 0.225]` (ImageNet RGB stats). Pretrained models expect this—essential when using transfer learning.

**Batch Normalization ≠ Input Normalization:** Batch Norm normalizes layer activations during training; input normalization is a preprocessing step. Both can be used together (normalization + BatchNorm).

## Common gotchas / interview framings
- Normalize training data, then use training stats (mean/std) on test data—leakage if test stats differ
- Outliers in min-max scaling → compress 99% of data into [0, 0.01], losing signal
- Forget to normalize inputs for transfer learning → accuracy drops significantly
- Z-score vs Min-max choice depends on downstream layer (BatchNorm tolerates raw inputs; small LR needs norm)

## See also
- [[data-loading-and-preprocessing-at-scale]]
- [[mixed-precision-training-fp16-bfloat16]]
- [[loss-curves-and-convergence-diagnostics]]

## Sources
See frontmatter `sources:`.
