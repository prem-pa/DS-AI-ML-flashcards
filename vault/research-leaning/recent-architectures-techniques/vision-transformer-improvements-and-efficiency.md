---
id: 9ed55fa2-a287-45a1-94bd-2ad371230fbd
title: Vision transformer improvements and efficiency
track: research-leaning
topic: recent-architectures-techniques
difficulty: 5
tags:
- vision-transformers
- ViT
- patch-embeddings
- hierarchical-vision
- efficient-tokens
- vision-efficiency
aliases:
- ViT optimization
- visual attention efficiency
- dense-prediction-transformers
sources:
- url: https://arxiv.org/abs/2010.11929
  label: 'An Image is Worth 16x16 Words: Transformers for Image Recognition at Scale'
- url: https://arxiv.org/abs/2203.09556
  label: Vision Transformers Need Registers
- url: https://arxiv.org/abs/2307.08248
  label: Vision Transformers are Robust Learners
cards:
- id: 7c96261a-5991-4d8e-8f4e-891165f0b162
  type: flip
  front: What is the fundamental insight of Vision Transformers (ViTs), and how do they differ from CNNs
    in terms of inductive bias?
  back: ViTs treat images as sequences of patches (e.g., 16×16 tokens), enabling pure Transformer-based
    processing. Unlike CNNs which have built-in locality (local filters) and translation invariance, ViTs
    have no spatial inductive bias. This makes them weaker on small datasets but stronger on large-scale
    pretraining where they learn their own spatial structure.
- id: 3bb13fa9-c46b-4975-9298-2bb41bb91eaa
  type: mcq
  front: Which technique improves ViT efficiency by removing redundant patches during forward pass?
  back: Token pruning dynamically removes patches deemed unimportant (e.g., uniform background) mid-forward
    pass, reducing attention computation. This is particularly effective for high-resolution images where
    many patches are redundant. The tradeoff is the cost of computing importance scores.
  choices:
  - key: a
    text: Patch aggregation
    correct: false
  - key: b
    text: Token pruning
    correct: true
  - key: c
    text: Sparse embeddings
    correct: false
  - key: d
    text: Patch downsampling
    correct: false
- id: 60511186-86bb-4b62-8fa6-269002bfb518
  type: flip
  front: What are register tokens in Vision Transformers, and why were they introduced?
  back: Register tokens are learnable token buffers added to the sequence during ViT forward pass (not
    tied to image patches). They were introduced to prevent activation collapse in deep ViTs, providing
    stable attention targets and improving representation quality in later layers. They act as a stabilization
    mechanism for deep vision models.
- id: aeddc5e6-a247-4e30-aa55-e5e08e930b2a
  type: mcq
  front: What architectural pattern do hierarchical Vision Transformers (e.g., Swin) use to improve efficiency?
  back: Hierarchical ViTs like Swin use local window attention (restricting attention to spatial neighborhoods)
    and shifted windows to enable cross-window communication, reducing complexity from O(n²) to O(n log
    n). They also downsample tokens across layers for multi-scale representations, making them much more
    efficient than naive ViTs.
  choices:
  - key: a
    text: Global attention at all layers with dropout
    correct: false
  - key: b
    text: Local window attention with shifted windows, multi-scale resolution
    correct: true
  - key: c
    text: Convolutional layers interleaved with attention
    correct: false
  - key: d
    text: Sparse global attention using learned importance scores
    correct: false
- id: 4964fc57-e5dd-4605-b59c-15083ec0b8f2
  type: flip
  front: Compared to CNNs, what are the key advantages of ViTs on transfer learning tasks, and what is
    the main limitation?
  back: 'Advantage: ViTs transfer much better from large-scale pretraining (ImageNet-21K, proprietary
    datasets) because they learn less dataset-specific inductive bias. Limitation: ViTs require more data
    to train from scratch and are weaker on small datasets where CNN''s locality inductive bias is beneficial.
    This makes ViTs preferable for transfer learning scenarios.'
---

## Intuition

Vision Transformers (ViTs) apply pure Transformer architecture to images by dividing them into non-overlapping patches and treating patches as sequence tokens. Recent improvements focus on three angles: (1) making patches adaptive/hierarchical, (2) pruning redundant tokens, (3) improving efficiency without sacrificing accuracy.

## Detail

Key improvements in 2024-2025:

**Patch efficiency:**
- Non-overlapping patches reduce token count vs dense embeddings
- Adaptive/variable-sized patches based on image regions
- Hierarchical designs (e.g., Swin, BEiT) reduce computation in early layers

**Token efficiency:**
- Token pruning: removing redundant patches mid-forward pass
- Sparse attention: only attending to nearby spatial regions
- Efficient token selection: learned routing of which patches matter

**Architectural patterns:**
- Hierarchical ViTs with downsampling (multi-scale representations)
- Register tokens as learnable buffers for stability
- Combining local (CNN-like) and global (Transformer) paths

**Performance gains:**
- Competitive with CNNs on ImageNet while handling variable resolutions
- Superior to CNNs on fine-grained tasks (medical imaging, remote sensing)
- Better transfer learning to downstream tasks

## Common gotchas / interview framings
- Q: "Why not just use CNNs for vision?" A: ViTs scale better to large datasets, have better transfer learning, no inductive bias toward locality
- Q: "Aren't 16×16 patches too coarse?" A: Patches are flexible; can use 8×8 or variable sizes. Hierarchical designs use smaller patches early
- Q: "How does ViT handle variable resolution inputs?" A: Needs retraining or position interpolation; patch-based design is flexible but position encoding isn't adaptive by default
- Token pruning overhead: deciding which tokens to prune adds computation; net gains emerge only for longer sequences
- Efficiency vs accuracy tradeoff: aggressive pruning hurts dense tasks like segmentation

## See also
- [[vision-transformer]]
- [[attention-mechanism]]
- [[image-patches]]
- [[hierarchical-architectures]]
- [[token-pruning]]
- [[sparse-vision]]

## Sources
See frontmatter `sources:`.
