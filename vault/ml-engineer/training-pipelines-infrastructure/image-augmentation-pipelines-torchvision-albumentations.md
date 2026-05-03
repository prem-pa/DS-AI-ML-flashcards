---
id: fb8b423f-4206-44b7-9b57-8065d7953e6d
title: Image augmentation pipelines (torchvision, albumentations)
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- augmentation
- regularization
- torchvision
- albumentations
- online-augmentation
aliases:
- data augmentation
- online augmentation
- augmentation strategy
sources:
- url: https://pytorch.org/vision/stable/transforms.html
  label: torchvision.transforms
- url: https://albumentations.ai/
  label: Albumentations Library
- url: https://arxiv.org/abs/1805.09501
  label: AutoAugment Paper
cards:
- id: 7855e836-b689-47de-8fc9-8266660ed8fc
  type: flip
  front: Why augment training data online (per batch) instead of pre-generating augmented copies?
  back: 'Online augmentation: infinite diversity (each epoch, each sample augmented differently). Pre-generated:
    finite diversity (limited by storage). Online also avoids duplicating storage.'
- id: 2d0c298d-bde8-4b7b-a493-d0e14cfc3e39
  type: mcq
  front: What's the key difference between torchvision.transforms and albumentations?
  back: ''
  choices:
  - key: a
    text: torchvision is slower
    correct: false
  - key: b
    text: Albumentations uses OpenCV, faster, better bbox/mask support
    correct: true
  - key: c
    text: torchvision supports GPU
    correct: false
  - key: d
    text: No practical difference
    correct: false
- id: b4ebef02-6c44-47fe-a3a9-8dd27eef31cb
  type: flip
  front: You're training a model for medical image classification. What augmentation strategy is safest?
  back: Light augmentations only (small rotations, slight zoom, small translations). Avoid aggressive
    rotations/flips that could flip anatomy (e.g., left-right mirror in chest X-rays changes meaning).
- id: 93534a4b-4d25-43e9-a684-43eb8c0afe84
  type: mcq
  front: Should you apply augmentation during validation?
  back: ''
  choices:
  - key: a
    text: Yes, for consistency
    correct: false
  - key: b
    text: No, validation should be deterministic
    correct: true
  - key: c
    text: Only light augmentation
    correct: false
  - key: d
    text: Yes, but different augmentations than training
    correct: false
---

## Intuition
Models overfit to training set specifics (e.g., camera angle, lighting). Online augmentation (rotation, flip, crop) on-the-fly during training creates synthetic diversity. Model never sees exact same image twice, generalizing better to test data without collecting more images.

## Detail
**torchvision.transforms:** Compose transformations in pipeline. `Compose([RandomHorizontalFlip(), RandomRotation(45), ColorJitter()])` applies sequentially. Each sample augmented differently each epoch (stochastic).

**Albumentations:** More efficient (OpenCV backend vs PIL), more transforms, GPU support. Handles bounding boxes/masks. Preferred for production pipelines with complex augmentations.

**Light vs Heavy Augmentation:** Light (flip, small crop) preserves semantics; heavy (aggressive rotation, color shift) risks breaking labels (e.g., rotate 90° with horizontal text). Domain-dependent.

**Augmentation Intensity & Regularization:** Stronger augmentation = more regularization, delays overfitting. But too much hurts convergence. Typical approach: start conservative, increase if overfitting.

**Train vs Validation:** Augment training data only. Validation/test use deterministic resize + center crop (no randomness) for reproducibility.

## Common gotchas / interview framings
- Augment both train and validation → invalid validation metrics
- Geometric augmentation (rotate) without updating labels → incorrect targets
- Augmentation + Dropout + BatchNorm together → over-regularization, slow convergence
- Test-time augmentation (TTA) useful for production but adds latency

## See also
- [[data-loading-and-preprocessing-at-scale]]
- [[imbalanced-data-and-oversamplingundersampling]]
- [[validation-strategy-and-metric-selection]]

## Sources
See frontmatter `sources:`.
