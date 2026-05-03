---
id: 7445700f-9ee9-4287-bed3-6d0d4b842fe3
title: Pooling and spatial downsampling
track: ml-engineer
topic: deep-learning-architectures
difficulty: 3
tags:
- cnn
- spatial-downsampling
- max-pooling
- average-pooling
aliases:
- pool-layer
- stride-vs-pool
sources:
- url: https://arxiv.org/abs/1311.0472
  label: Visualizing and Understanding Convolutional Networks
- url: https://arxiv.org/abs/1412.6806
  label: 'Striving for Simplicity: The All Convolutional Net'
cards:
- id: 3f327cde-6e57-4ba6-87a1-111c5408ebfb
  type: flip
  front: 'Gradient flow difference: max pooling vs strided convolution during backprop.'
  back: 'Max pooling: gradients flow only through max-position neuron (one hot). Strided conv: gradients
    distributed across learned kernel; smoother, learnable.'
- id: 243a584d-73b0-4106-96a4-467565eba5a9
  type: mcq
  front: Building a semantic segmentation model. Should you use max pooling or strided convolutions for
    downsampling?
  back: Strided convs are learnable and allow finer gradient control. Max pooling (A) destroys too much
    spatial structure for pixel-level tasks. Average pooling (C) dilutes gradients. Dilated convs (D)
    expand RF, not replace downsampling.
  choices:
  - key: a
    text: Max pooling; it preserves feature sharpness
    correct: false
  - key: b
    text: Strided convolutions; they're learnable and preserve more gradient info for dense tasks
    correct: true
  - key: c
    text: Average pooling; it's softer and doesn't lose spatial detail
    correct: false
  - key: d
    text: Neither; use dilated convolutions instead
    correct: false
- id: 22c2a10b-b70a-4b66-9994-025979c19c16
  type: flip
  front: Why does modern ResNet sometimes skip pooling and use stride-2 convolutions?
  back: Stride-2 convs are learnable, allowing the model to learn what to discard. Pooling is fixed and
    uninformed; convolutions adapt downsampling to the task.
- id: 201afe76-4170-40b9-a80b-66607e6ec399
  type: mcq
  front: Pooling is invariant to small spatial shifts. What is the practical downside?
  back: Invariance to shifts (A) is actually good. Pooling intentionally discards fine spatial info (C),
    useful for classification but harmful for localization tasks. (B) contradicts the premise.
  choices:
  - key: a
    text: Small shifts in input can cause large shifts in output classification
    correct: false
  - key: b
    text: Invariance is beneficial; there's no downside
    correct: false
  - key: c
    text: Information loss means fine-grained spatial detail is destroyed early
    correct: true
  - key: d
    text: Pooling makes networks too deep
    correct: false
---

## Intuition

**Pooling** reduces spatial dimensions while retaining important features. **Max pooling** picks the strongest activation in a region (noise robust); **average pooling** smooths by averaging (preserves soft information). Pooling is not learnable, unlike strided convolutions.

## Detail

Max pooling at stride 2 over a 2×2 window: $y_{ij} = \max_{(i', j') \in [i:i+2, j:j+2]} x_{i'j'}$

Average pooling: $y_{ij} = \frac{1}{4} \sum_{(i',j') \in [i:i+2, j:j+2]} x_{i'j'}$

**Gradient flow perspective**: Max pooling backprop routes gradients only through the max-valued neuron (sparse gradients). Average pooling distributes gradients uniformly, aiding optimization but potentially diluting strong signals.

**Modern trend**: Strided convolutions are often preferred over pooling because:
- Learnable parameter interaction during downsampling
- Smoother gradient flow (not just one hot neuron)
- Less information destruction

## Common gotchas / interview framings
- *"Does pooling destroy important information?"* Yes, intentionally—the trade-off is spatial invariance (small shifts don't affect output) for information loss
- *"Should I use max or average pooling?"* Max is sharper/more selective (good for classification); average is softer (better for dense tasks like segmentation). Strided conv is learnable middle ground
- *"Why no pooling in modern ViTs or dense models?"* Patch-based processing + learned strides avoid hard information destruction; crucial for pixel-level tasks
- *"Pooling layers don't have weights, so they don't help backprop"* True, but they break gradient paths, potentially hurting deep networks

## See also
- [[stride]]
- [[feature-map-size]]
- [[spatial-hierarchy]]
- [[strided-conv]]

## Sources
See frontmatter `sources:`.
