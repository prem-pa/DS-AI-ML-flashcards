---
id: c08be75a-ebf2-4e0e-a644-fc0627305852
title: CNN architectures (ResNet, EfficientNet, Vision Transformers)
track: verticals
topic: computer-vision
difficulty: 3
tags:
- classification
- backbone
- efficiency
- vision-transformers
- architecture-design
aliases:
- ResNet
- EfficientNet
- ViT
sources:
- url: https://arxiv.org/abs/1512.03385
  label: Deep Residual Learning for Image Recognition (ResNet)
- url: https://arxiv.org/abs/1905.11946
  label: 'EfficientNet: Rethinking Model Scaling for Convolutional Neural Networks'
- url: https://arxiv.org/abs/2010.11929
  label: 'An Image is Worth 16x16 Words: Transformers for Image Recognition at Scale (ViT)'
cards:
- id: 8479d24a-8abf-4a15-87c6-6db2e2197e61
  type: flip
  front: What is the key innovation of ResNet that enables training of 100+ layer networks?
  back: Residual (skip) connections that allow gradients to flow directly across layers, preventing vanishing
    gradients. F(x) + x instead of just F(x).
- id: fa70c5f3-ef6a-4373-9767-4d71047ec4d5
  type: mcq
  front: 'EfficientNet improves over ResNet/Inception primarily by:'
  back: Compound scaling jointly optimizes network depth (# layers), width (# channels), and input resolution,
    achieving better accuracy-efficiency tradeoffs. EfficientNet-B0 matches ResNet-50 accuracy with ~10x
    fewer parameters.
  choices:
  - key: a
    text: Adding more convolutional layers
    correct: false
  - key: b
    text: Using compound scaling of depth, width, and resolution uniformly
    correct: true
  - key: c
    text: Replacing convolutions with attention
    correct: false
  - key: d
    text: Removing batch normalization
    correct: false
- id: 2c51349a-98e1-48ac-bd35-46ea58672350
  type: flip
  front: When should you prefer CNNs over Vision Transformers, and why?
  back: Small-scale datasets (<10k images) favor CNNs due to inductive bias (locality, weight sharing).
    ViTs require large pretraining datasets (ImageNet-21k or 400M+ image-text pairs) to match CNN performance;
    smaller datasets lead to overfitting.
- id: 80b15345-dfce-4212-acf0-22737b7b17fb
  type: mcq
  front: Vision Transformers divide an image into patches and embed them as tokens. For a 224x224 image
    with 16x16 patches, how many tokens are created?
  back: 224 / 16 = 14 patches per dimension. 14 × 14 = 196 patches. ViT processes these as a sequence
    of tokens, plus 1 [CLS] token, for 197 total.
  choices:
  - key: a
    text: '14'
    correct: false
  - key: b
    text: '196'
    correct: true
  - key: c
    text: '224'
    correct: false
  - key: d
    text: '784'
    correct: false
---

## Intuition

Image classifiers form the foundation of computer vision. ResNet introduced residual connections to train very deep networks. EfficientNet optimizes the tradeoff between accuracy, latency, and model size through compound scaling. Vision Transformers (ViT) replace convolutions entirely with pure self-attention, showing that "an image is worth 16x16 words" when you patch it into tokens.

## Detail

**ResNet (Residual Networks):**
- Skip connections allow gradients to flow through deep networks (50, 101, 152 layers)
- Bottleneck blocks reduce computation: 1x1 conv → 3x3 conv → 1x1 conv
- Enables training of networks 100+ layers deep without vanishing gradients

**EfficientNet:**
- Compound scaling: jointly scale depth, width, and resolution
- Mobile-friendly variants (B0–B7) for different compute budgets
- Achieves SOTA accuracy with fewer parameters than ResNet/Inception

**Vision Transformers (ViT):**
- Divide image into patches, embed as tokens, apply standard transformer
- Requires large pretraining datasets (ImageNet-21k) to match CNN performance
- Excels with transfer learning; scales better than CNNs on large datasets
- Enables multimodal learning (CLIP, ALIGN) via shared embedding space

## Common gotchas / interview framings
- ResNets still strong baseline; don't assume ViT always better
- EfficientNet FLOPs vs. latency: hardware matters (GPU vs. mobile)
- ViT needs big data; small-scale datasets favor CNNs
- Hybrid architectures (e.g., Swin Transformer) combine convolution and attention
- Batch norm in ResNet is critical; different optimizers for ViT (warmup, learning rate scheduling)

## See also
- [[httpsarxivorgabs151203385]]
- [[httpsarxivorgabs190511946]]
- [[httpsarxivorgabs201011929]]
- [[httpspaperswithcodecommethodresidual-connection]]
- [[httpspaperswithcodecommethodvision-transformer]]

## Sources
See frontmatter `sources:`.
