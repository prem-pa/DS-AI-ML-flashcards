---
id: 0fe4b757-0270-409d-92d7-1a0b80326fa6
title: Semantic segmentation (FCN, U-Net, DeepLab)
track: verticals
topic: computer-vision
difficulty: 3
tags:
- segmentation
- dense-prediction
- pixel-classification
- encoder-decoder
- atrous-convolution
aliases:
- FCN
- U-Net
- DeepLab
- semantic segmentation
sources:
- url: https://arxiv.org/abs/1411.4038
  label: Fully Convolutional Networks for Semantic Segmentation
- url: https://arxiv.org/abs/1505.04597
  label: 'U-Net: Convolutional Networks for Biomedical Image Segmentation'
- url: https://arxiv.org/abs/1606.02128
  label: 'DeepLab: Semantic Image Segmentation with Deep Convolutional Nets, Atrous Convolution, and Fully
    Connected CRFs'
- url: https://arxiv.org/abs/1706.05587
  label: 'DeepLabv3: Rethinking Atrous Convolution for Semantic Image Segmentation'
cards:
- id: 8dbba28b-6abc-434d-9790-78fe557f7bbd
  type: mcq
  front: What is the core innovation of U-Net compared to standard FCN?
  back: U-Net concatenates encoder features to decoder layers at matching resolutions, preserving spatial
    detail and gradients. This dramatically improves edge preservation and enables training on small datasets
    (100 images) unlike FCN.
  choices:
  - key: a
    text: Using atrous convolution for large receptive fields
    correct: false
  - key: b
    text: Adding skip connections between encoder and decoder at corresponding resolution levels
    correct: true
  - key: c
    text: Replacing convolutions with attention
    correct: false
  - key: d
    text: Using CRF post-processing
    correct: false
- id: 706efbfc-e15b-4359-9d1e-b0a0bd9083ff
  type: flip
  front: Explain atrous (dilated) convolution and why DeepLab uses it instead of pooling.
  back: 'Atrous convolution applies a kernel with gaps (dilation rate d): stride d between kernel elements.
    A 3x3 kernel with dilation=3 sees a 7x7 effective receptive field. Benefits: increases receptive field
    without downsampling (no resolution loss), enabling per-pixel prediction at full resolution.'
- id: 1a5adfbb-2705-4d06-b62d-ffa97ed56176
  type: flip
  front: What is mIoU and why is it the standard metric for semantic segmentation?
  back: 'mIoU = mean Intersection over Union. For each class: IoU = TP / (TP + FP + FN). Average across
    all classes. Handles class imbalance (rare objects weighted equally to common ones) and penalizes
    false positives and negatives fairly.'
- id: e1eec981-1db9-4d0b-a7eb-ff8fcf5b3e7c
  type: mcq
  front: Which architecture is best suited for medical image segmentation with limited training data?
  back: 'U-Net: skip connections enable effective training on ~100 images. FCN: needs more data. DeepLabv3:
    designed for large-scale semantic segmentation (Cityscapes). U-Net''s strength is parameter efficiency
    + edge preservation on small datasets.'
  choices:
  - key: a
    text: FCN
    correct: false
  - key: b
    text: U-Net
    correct: true
  - key: c
    text: DeepLabv3
    correct: false
  - key: d
    text: All equally suitable
    correct: false
- id: d97fc1fd-7274-49f0-9fd8-acbbd6c5b79f
  type: flip
  front: Name two design choices that address boundary precision in segmentation.
  back: '1) Skip connections (U-Net): preserve fine details from encoder. 2) Atrous convolution (DeepLab):
    maintains resolution while expanding receptive field. Both prevent information loss during downsampling.'
---

## Intuition

Semantic segmentation labels every pixel in an image with a class (e.g., "road", "tree", "car"). Unlike classification (one label per image) or object detection (boxes), segmentation is pixel-level dense prediction. Key challenge: preserve spatial resolution while capturing context.

## Detail

**FCN (Fully Convolutional Networks):**
- Replace fully connected layers with 1x1 convolutions
- Downsampling path (encoder) extracts features; upsampling path (decoder) restores resolution
- Skip connections from early layers preserve fine details
- Transposed convolutions / bilinear upsampling restore resolution
- Simple but suffers from coarse predictions due to information loss

**U-Net:**
- Encoder-decoder with skip connections between corresponding levels
- Contracting path: 3x3 conv → ReLU → 2x2 max pool (downsample)
- Expanding path: transposed conv → concatenate with encoder feature → 3x3 conv (upsample)
- Excellent for medical imaging (small datasets); skip connections preserve edges
- Minimal parameters; works with ~100 training images (transfer learning)

**DeepLab:**
- Atrous (dilated) convolution: increases receptive field without pooling (no resolution loss)
- ASPP (Atrous Spatial Pyramid Pooling): multi-scale context via parallel atrous convolutions (rates 1, 6, 12, 18)
- CRF (Conditional Random Field) post-processing refines boundaries
- DeepLabv3: removes CRF; improves ASPP design; achieves ~82% mIoU on Cityscapes

**Comparison:**
- FCN: simplest, slower on boundary precision
- U-Net: best for small medical datasets, strong edge preservation
- DeepLab: SOTA on large scenes (Cityscapes, ADE20K); atrous convolution scales context

## Common gotchas / interview framings
- Downsampling loses spatial detail; atrous/dilated convolution recovers context without pooling
- Skip connections are critical: without them, decoder can't recover fine structure
- Evaluation metric: mIoU (mean Intersection over Union) per class; IoU = TP / (TP + FP + FN)
- Class imbalance: rare classes (person) vs common (background); weighted loss or hard mining helps
- Postprocessing: CRF improves boundaries but adds latency; learned refinement (e.g., DeepLabv3) preferred
- Full resolution output is expensive; most models output 1/4 resolution, then upsample
- Boundary errors common: interleave low-level (encoder skip) and high-level (decoder) features

## See also
- [[httpsarxivorgabs14114038]]
- [[httpsarxivorgabs150504597]]
- [[httpsarxivorgabs160602128]]
- [[httpsarxivorgabs170605587]]
- [[httpspaperswithcodecomtasksemantic-segmentation]]

## Sources
See frontmatter `sources:`.
