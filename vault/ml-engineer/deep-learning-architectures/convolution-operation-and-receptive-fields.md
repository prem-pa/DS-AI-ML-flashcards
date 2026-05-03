---
id: 45e31533-3752-4d3f-bfff-07afd0c98dc3
title: Convolution operation and receptive fields
track: ml-engineer
topic: deep-learning-architectures
difficulty: 3
tags:
- cnn
- spatial-locality
- weight-sharing
- receptive-field
aliases:
- conv-receptive-field
- spatial-context-window
sources:
- url: https://arxiv.org/abs/1311.0472
  label: Visualizing and Understanding Convolutional Networks
- url: https://distill.pub/2016/deconv-checkerboard/
  label: Deconvolution and Checkerboard Artifacts
cards:
- id: 8bd6b177-50a7-4da7-beca-e079225539dd
  type: flip
  front: Receptive field formula at layer $\ell$ with kernel $k$, stride $s$.
  back: $RF_\ell = RF_{\ell-1} + (k - 1) \prod_{i=1}^{\ell-1} s_i$. Each stride multiplier compounds the
    effective dilation. Start $RF_0 = 1$.
- id: b523e11e-df18-4d66-b8aa-5525fc5f7eb3
  type: flip
  front: Why does stride-2 stacking expand receptive field so aggressively?
  back: Each stride-2 layer multiplies the prior layers' stride product. A stride-2 after stride-2 gives
    $(k-1) \times 2 \times 2$ growth, then $(k-1) \times 4$, etc. Exponential in depth.
- id: 2d7b16df-3852-4f1e-8c48-2047cbdfe30d
  type: mcq
  front: You're building a small-object detector. Current model has RF=128×128 pixels but fails on 8×8
    objects. Which is most practical?
  back: Option A balances RF growth (dilated convs later) with spatial preservation (stride-1 early).
    Stride-1 throughout (C) is prohibitively slow. Kernel size alone (B) is weak; strides compound much
    faster.
  choices:
  - key: a
    text: Reduce stride early, use dilated convs in middle, keep receptive field high
    correct: true
  - key: b
    text: Increase kernel size to 7×7 throughout—bigger kernels = better RF
    correct: false
  - key: c
    text: Use stride-1 throughout; computational cost is secondary
    correct: false
  - key: d
    text: Add more layers; depth alone increases RF
    correct: false
- id: dd79344f-dbd1-487d-9064-2fd942561bff
  type: flip
  front: Padding mode (zero vs reflect) impact on convolution filters.
  back: Zero-padding biases filters to learn boundary suppression; reflect padding distributes gradients
    more evenly. Reflect better for dense tasks; zero acceptable for classification.
---

## Intuition

Convolution applies a learned filter across spatial dimensions with **weight sharing**—the same kernel weights reuse across positions. This exploits the locality assumption: nearby pixels are more related than distant ones. The **receptive field** is the region of input each neuron "sees"; larger receptive fields aggregate broader context.

## Detail

A $k \times k$ filter at stride $s$ with padding $p$ produces output size: $(H - k + 2p) / s + 1$. The receptive field at layer $\ell$ is:
$$RF_\ell = RF_{\ell-1} + (k - 1) \prod_{i=1}^{\ell-1} s_i$$

Dilation (or "atrous" convolution) increases RF without losing resolution: $RF = 1 + (k - 1) \cdot d$ per layer.

### Interview angle: tradeoff matrix
- **Larger receptive field**: better for tasks needing global context (detection, segmentation) but requires deeper nets → vanishing gradients
- **Stride-2 stacking**: quickly expands RF but destroys fine spatial detail early
- **Dilated convs**: expand RF at same depth, critical for dense tasks (segmentation) but can create "holes" in feature maps

## Common gotchas / interview framings
- *"How big is your effective receptive field?"* Many engineers overestimate due to not accounting for striding; use effective formula $1 + (k-1) \sum_{i} s_i$ product
- *"Why does my detection model fail on small objects?"* Late striding means small objects map to 1-2 neurons—insufficient information
- *"Dilated convs seem magical—when to use?"* Only when you need context without resolution loss (e.g., semantic segmentation). For classification, standard stride-2 is faster
- Padding mode matters: zero-padding introduces boundary artifacts; reflection padding better for edge-case robustness

## See also
- [[weight-sharing]]
- [[spatial-locality]]
- [[kernel-size]]
- [[dilation]]
- [[stride]]

## Sources
See frontmatter `sources:`.
