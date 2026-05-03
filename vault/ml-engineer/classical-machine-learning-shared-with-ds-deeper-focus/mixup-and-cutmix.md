---
id: 20ccba86-36fe-4199-8feb-c5849cccf670
title: Mixup and CutMix
track: ml-engineer
topic: classical-machine-learning-shared-with-ds-deeper-focus
difficulty: 3
tags:
- data-augmentation
- training-regularization
- label-mixing
- spatial-operations
aliases:
- sample-interpolation
- spatial-cutmix
- soft-label-mixing
sources:
- url: https://arxiv.org/abs/1710.09412
  label: Mixup Paper (Zhang et al.)
- url: https://arxiv.org/abs/1905.04412
  label: CutMix Paper (Yun et al.)
cards:
- id: 0e015131-1c9e-4ba9-843a-562752744283
  type: flip
  front: What is the key difference between Mixup and CutMix?
  back: 'Mixup: blend entire images (convex combination) → soft targets. CutMix: paste rectangular region
    → spatially localized mixing. CutMix better preserves spatial information; empirically superior for
    vision tasks.'
- id: a32b339c-eb53-4a30-9734-9be9eb1f90cc
  type: mcq
  front: In Mixup, if λ ~ Beta(α, α), what happens as α → ∞?
  back: Beta(α, α) is symmetric. As α → ∞, distribution concentrates at 0.5. As α → 0, concentrates at
    edges (0, 1).
  choices:
  - key: a
    text: λ becomes 0 (minimal mixing)
    correct: false
  - key: b
    text: λ becomes 1 (full original)
    correct: false
  - key: c
    text: λ concentrates near 0.5 (maximum mixing)
    correct: true
  - key: d
    text: λ uniformly distributed
    correct: false
- id: 485307ff-a2a0-4060-b550-9064383ebedb
  type: flip
  front: Why does CutMix work better than Mixup for object detection/localization tasks?
  back: 'CutMix preserves spatial information: objects remain localized in pasted region. Mixup blends
    globally, destroying bounding box information. For detection, locality matters; CutMix more realistic
    (occlusion).'
- id: 904751a1-3f54-4651-bc3b-bd2cd2d8cbdf
  type: flip
  front: Can you combine Mixup with label smoothing? Are they redundant?
  back: 'Both soften targets, so partially redundant. Label smoothing: all classes get small mass. Mixup:
    two classes get λ and (1-λ). Empirically, using both can help; slight redundancy is acceptable.'
- id: 541765c4-e7e0-4ebe-bce5-2ffddb3511fc
  type: mcq
  front: In CutMix, how is the label adjusted when mixing two images?
  back: 'CutMix: region from image_i remains (area A), region from image_j pasted (area 1-A). Label: A*y_i
    + (1-A)*y_j.'
  choices:
  - key: a
    text: Both labels fully preserved
    correct: false
  - key: b
    text: 'Proportional to area: λ=(retained_area / total_area)'
    correct: true
  - key: c
    text: Average of labels
    correct: false
  - key: d
    text: Only one label used
    correct: false
---

## Intuition

Mixup: train on convex combinations of image pairs and their labels (soft interpolation). CutMix: mix spatial regions of images, label proportionally. Both improve generalization by encouraging linear behavior between examples and robustness to occlusion.

## Detail

**Mixup:**
- x̃ = λ * x_i + (1 - λ) * x_j (blend two images)
- ỹ = λ * y_i + (1 - λ) * y_j (soft labels)
- λ ~ Beta(α, α), where α ∈ [0, ∞)
- α small: λ ≈ 0 or 1 (near original samples)
- α large: λ ≈ 0.5 (strong mixing)

**Benefits of Mixup:**
- Encourages model to learn linear interpolation between decision boundaries
- Improves generalization by smoothing loss landscape
- Implicit regularization: reduces overfitting
- Works well with other losses (cross-entropy + Mixup effective)

**CutMix:**
- Replace rectangular region of image_i with region from image_j
- Label proportional to area: y = (Area_retained / Total_area) * y_i + (Area_pasted / Total_area) * y_j
- Spatial operation: preserves local structure better than Mixup
- More realistic augmentation than Mixup (humans recognize occluded objects)

**Cutmix implementation:**
- Random box region in image_i
- Crop corresponding region from image_j
- Paste j's region into i's box
- Compute λ = (H * W - H_box * W_box) / (H * W)

**Comparison (Mixup vs. CutMix):**
- Mixup: global blend, enforces linearity
- CutMix: local spatial mix, more interpretable, empirically better for vision
- Both: soft label mixing → regularization
- CutMix better for object detection/localization (spatial information retained)

**Interaction with other techniques:**
- Label smoothing + Mixup: both soften targets (can compound)
- Batch norm: Mixup works well (each sample still contributes independently)
- Dropout: orthogonal techniques (can combine)

**Engineering considerations:**
- **Mixup α**: α=1 typical (uniform Beta); higher α more aggressive mixing
- **CutMix box size**: fully random or constrained to reasonable range
- **Probability**: apply Mixup/CutMix with p ∈ [0.5, 1.0] per batch
- **Computational cost**: negligible (just interpolation/blending)
- **Distributed training**: independent per-sample mixing; no synchronization needed
- **Online learning**: Mixup/CutMix applied each epoch; compatible with streaming

```python
import torch
import torch.nn.functional as F

def mixup(x, y, alpha=1.0):
    batch_size = x.size(0)
    index = torch.randperm(batch_size)
    mixed_x = torch.zeros_like(x)
    mixed_y = torch.zeros_like(y)
    
    lam = np.random.beta(alpha, alpha)
    
    for i in range(batch_size):
        mixed_x[i] = lam * x[i] + (1 - lam) * x[index[i]]
        mixed_y[i] = lam * y[i] + (1 - lam) * y[index[i]]
    
    return mixed_x, mixed_y

def cutmix(x, y, alpha=1.0):
    batch_size = x.size(0)
    index = torch.randperm(batch_size)
    lam = np.random.beta(alpha, alpha)
    
    H, W = x.size(2), x.size(3)
    cut_ratio = np.sqrt(1.0 - lam)
    cut_h = int(H * cut_ratio)
    cut_w = int(W * cut_ratio)
    
    # Random box
    cx = np.random.randint(0, W)
    cy = np.random.randint(0, H)
    
    bbx1 = np.clip(cx - cut_w // 2, 0, W)
    bby1 = np.clip(cy - cut_h // 2, 0, H)
    bbx2 = np.clip(cx + cut_w // 2, 0, W)
    bby2 = np.clip(cy + cut_h // 2, 0, H)
    
    x[:, :, bby1:bby2, bbx1:bbx2] = x[index, :, bby1:bby2, bbx1:bbx2]
    lam = 1 - ((bbx2 - bbx1) * (bby2 - bby1) / (H * W))
    
    mixed_y = lam * y + (1 - lam) * y[index]
    return x, mixed_y
```

## Common gotchas / interview framings
- Mixup assumes data lies on manifold; works well for images, less for sparse/tabular
- CutMix requires spatial structure (images); doesn't apply directly to text/tabular
- Label mixing causes soft targets; loss must handle soft labels (cross-entropy does)
- Strong Mixup/CutMix can hurt clean accuracy; balance via probability p
- Mixup α=1 (uniform Beta) is standard; different α for different tasks
- Combination with label smoothing: both soften targets, can be redundant but often used together

## See also
- [[data-augmentation]]
- [[label-smoothing]]
- [[training-robustness]]

## Sources
See frontmatter `sources:`.
