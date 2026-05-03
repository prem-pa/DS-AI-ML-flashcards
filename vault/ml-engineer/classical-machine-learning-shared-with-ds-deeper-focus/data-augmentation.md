---
id: a63f6899-6092-4c83-80ef-62d5f9adb767
title: Data augmentation
track: ml-engineer
topic: classical-machine-learning-shared-with-ds-deeper-focus
difficulty: 3
tags:
- training-data-expansion
- robustness
- task-specific-transforms
- synthetic-data
aliases:
- synthetic-data-generation
- domain-specific-augmentation
- augmentation-policies
sources:
- url: https://en.wikipedia.org/wiki/Data_augmentation
  label: Wikipedia Data Augmentation
- url: https://github.com/albumentations-team/albumentations
  label: Albumentations Library
cards:
- id: 62d22a26-e9b0-489d-9f43-72a20ce8bea6
  type: flip
  front: Why is online data augmentation preferred over offline (pre-computed) augmentation?
  back: 'Online: apply transforms each epoch → different augmented views of same sample, unlimited effective
    data, lower memory. Offline: fixed augmented dataset stored → less variation, higher memory, but faster
    (no on-the-fly computation).'
- id: b2742512-d460-42ec-8ab5-83ff236c870d
  type: flip
  front: How does SMOTE help with imbalanced classification, and what is a key pitfall?
  back: 'SMOTE oversamples minority class by interpolating between nearest neighbors → balances class
    distribution. Pitfall: if done before train/test split, creates data leakage (synthetic samples can
    appear in test). Use within CV pipeline only.'
- id: c7bf1740-d924-489c-940d-c6f47025da49
  type: mcq
  front: What is RandAugment and how does it differ from AutoAugment?
  back: 'AutoAugment: search space is large (operation selection + magnitude). RandAugment: randomly pick
    operations, search magnitude M and probability p (2D search, faster).'
  choices:
  - key: a
    text: Same thing
    correct: false
  - key: b
    text: 'RandAugment simplified: search magnitude + probability; AutoAugment searches full policy space'
    correct: true
  - key: c
    text: RandAugment requires manual tuning; AutoAugment automatic
    correct: false
  - key: d
    text: Both automated; RandAugment faster
    correct: false
- id: fdcedbbc-1434-451c-807b-1b96546fb143
  type: flip
  front: Why should data augmentation be task-preserving, and how does this constraint affect design?
  back: 'Augmentation must not change ground truth label. Rotations okay for natural images (label invariant);
    rotation ≠ okay for digit recognition (6 → 9). Domain knowledge required: understand what variations
    are valid for your task.'
- id: e4ef02b4-41f0-4c78-8509-465ac60212a9
  type: mcq
  front: Should you apply augmentation to test/validation sets?
  back: Augmentation is a training technique. Test/val sets evaluate on realistic, clean examples to get
    unbiased generalization estimates. Augmentation in eval introduces artificial noise.
  choices:
  - key: a
    text: Always, to be consistent
    correct: false
  - key: b
    text: Never; evaluate on clean data for calibration and stability
    correct: true
  - key: c
    text: Only for classification, not regression
    correct: false
  - key: d
    text: Depends on dataset size
    correct: false
---

## Intuition

Data augmentation: apply task-preserving transformations (rotations, crops, brightness changes) to training data, creating synthetic examples. Increases effective dataset size, improves robustness to variations, reduces overfitting.

## Detail

**Image augmentation:**
- Geometric: rotation, scaling, translation, shear, flip
- Photometric: brightness, contrast, saturation, hue shifts
- Mixup, CutMix: blend/mix images
- RandAugment: randomly select from predefined ops, apply sequentially

**Text augmentation:**
- Synonym replacement, random insertion, random swap, random deletion (EDA)
- Backtranslation: translate to another language and back
- Token masking: mask/replace tokens (used in BERT pretraining)
- Paraphrase generation

**Tabular augmentation:**
- Feature engineering / synthetic features
- SMOTE (Synthetic Minority Over-sampling): interpolate minority class samples
- Noise injection: add Gaussian noise to features
- Mixup: interpolate feature vectors

**Online vs. offline:**
- Online: apply transforms each epoch → different views, unlimited effective data
- Offline: pre-computed augmented samples, stored → fixed augmented dataset
- Online preferred: lower memory, more variation

**AutoAugment & RandAugment:**
- AutoAugment: search policy space to find best augmentation ops
- RandAugment: simplified version; search magnitude + probability
- TrivialAugment: even simpler; random magnitude per op

**Engineering considerations:**
- **Computational cost**: Online augmentation adds CPU/GPU overhead each epoch
- **Memory**: Pre-computed augmentation increases dataset size
- **Batch normalization**: Can undo some augmentation effects (normalize away variations)
- **Class imbalance**: SMOTE for minority class; careful not to overfit
- **Domain knowledge**: Task-specific augmentation (medical imaging ≠ natural images)

```python
from albumentations import Compose, Rotate, GaussNoise, Normalize
import albumentations as A

transform = Compose([
    A.Rotate(limit=30, p=0.8),
    A.GaussNoise(p=0.2),
    A.Normalize(),
])

# SMOTE for tabular imbalanced data
from imblearn.over_sampling import SMOTE
smote = SMOTE()
X_resampled, y_resampled = smote.fit_resample(X_train, y_train)
```

## Common gotchas / interview framings
- Augmentation must preserve labels; overly aggressive transforms invalidate annotations
- Test set: augmentation typically not used (evaluate on clean data for calibration)
- SMOTE creates interpolated synthetic samples; can cause data leakage if not done within CV
- Augmentation and batch norm interaction: strong augmentation with batch norm can hurt (norm changes per batch)
- Domain mismatch: augmentation valid only if it represents real variations in deployment data

## See also
- [[mixup-and-cutmix]]
- [[label-smoothing]]
- [[dropout-and-batch-normalization]]

## Sources
See frontmatter `sources:`.
