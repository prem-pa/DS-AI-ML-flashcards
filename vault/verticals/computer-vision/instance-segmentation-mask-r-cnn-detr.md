---
id: ac19c779-9e8b-44db-9d9c-a1a8d689d037
title: Instance segmentation (Mask R-CNN, DETR)
track: verticals
topic: computer-vision
difficulty: 3
tags:
- instance-segmentation
- object-detection
- mask-prediction
- set-prediction
- transformer-detection
aliases:
- Mask R-CNN
- DETR
- instance segmentation
sources:
- url: https://arxiv.org/abs/1703.06870
  label: Mask R-CNN (He et al., 2017)
- url: https://arxiv.org/abs/2005.12677
  label: End-to-End Object Detection with Transformers (DETR)
- url: https://github.com/facebookresearch/detectron2
  label: 'Detectron2: A PyTorch-based modular object detection library'
- url: https://arxiv.org/abs/2211.09437
  label: 'Segment Anything (SAM): zero-shot instance segmentation'
cards:
- id: e793ced7-4406-429b-b56d-fd30193ea576
  type: mcq
  front: How does Mask R-CNN extend Faster R-CNN for instance segmentation?
  back: 'Mask R-CNN: Faster R-CNN box head outputs (class, bbox) + additional mask head outputs per-pixel
    mask. ROI Align (bilinear interpolation) preserves spatial precision for mask prediction, critical
    for high-quality masks.'
  choices:
  - key: a
    text: Replaces RPN with a transformer
    correct: false
  - key: b
    text: Adds a mask prediction branch (FCN head) alongside box classification and regression
    correct: true
  - key: c
    text: Uses atrous convolution instead of pooling
    correct: false
  - key: d
    text: Removes the RPN entirely
    correct: false
- id: 576569f3-bb89-4185-a451-422cd89302b6
  type: flip
  front: Explain the key difference between ROI Pooling (Faster R-CNN) and ROI Align (Mask R-CNN).
  back: 'ROI Pooling: divides ROI into grid, max pools each cell (lossy quantization). ROI Align: bilinear
    interpolation samples ROI at sub-pixel locations, preserving alignment. ROI Align improves mask quality
    by ~10% mAP because mask prediction requires precise spatial correspondence.'
- id: 8cb1fa1c-b543-4992-850b-c548c77af111
  type: flip
  front: What is the Hungarian matching algorithm in DETR, and why is it necessary?
  back: DETR generates N=100 object queries; ground-truth has M objects (M << N). Hungarian algorithm
    finds optimal 1-1 matching between queries and targets (minimizes cost). Loss computed only on matched
    queries; unmatched queries penalized as background. Avoids NMS; enables end-to-end training.
- id: 95e1c54b-5ac9-412d-8b8c-355e4f3b9ad8
  type: mcq
  front: Why is DETR slower to train and converge than Faster R-CNN despite being end-to-end?
  back: DETR's global attention and set-based loss lack the inductive biases of RPN (proposal filtering).
    Requires augmentation (random crops, strong color jitter), large batch sizes (16+), and 500 epochs
    vs Faster R-CNN's ~120. Learning rate scheduling (warmup crucial) and late fusion backbone needed.
  choices:
  - key: a
    text: DETR uses more parameters
    correct: false
  - key: b
    text: Transformer self-attention is slower than convolution
    correct: false
  - key: c
    text: Hungarian matching is computationally expensive
    correct: false
  - key: d
    text: Requires strong augmentation, large batches, and ~500 epochs to converge; harder optimization
      landscape
    correct: true
- id: 4518d75d-c5a2-4e01-a445-a4027508eab8
  type: flip
  front: How does SAM (Segment Anything Model) achieve zero-shot instance segmentation?
  back: 'SAM trained on 1B masks with diverse prompts (points, boxes, text). Encoder embeds image; prompt
    encoder embeds prompt. Decoder predicts mask conditioned on both. At test time: provide any prompt
    (e.g., click a point), SAM outputs mask—no finetuning needed. Recently SAM 2 adds video support via
    streaming memory.'
---

## Intuition

Instance segmentation combines object detection (find objects + bounding boxes) with semantic segmentation (label each pixel). Goal: identify and segment each individual object with a mask, not just one class label per pixel. More complex than semantic segmentation (per-pixel class only) or detection (boxes only).

## Detail

**Mask R-CNN:**
- Extends Faster R-CNN: adds a mask prediction branch alongside box classification/regression
- Two-stage: (1) RPN generates proposals, (2) ROI head classifies + regresses boxes + predicts masks
- Mask head: small FCN applied to each ROI, predicts per-pixel class (binary mask per class per object)
- ROI Align: bilinear interpolation (not ROI Pooling's harsh quantization) preserves spatial precision
- SOTA for instance segmentation; ~42 mAP on COCO
- Flexible: easily adapted for keypoint detection, panoptic segmentation

**DETR (Detection Transformer):**
- End-to-end object detection without NMS or hand-crafted components (anchors, etc.)
- Vision transformer encoder + learned object queries (N=100 learnable embeddings)
- Transformer decoder: cross-attention between queries and encoder features
- Bipartite matching loss (Hungarian algorithm) matches predictions to ground-truth targets
- Output: set of (class, bbox) predictions; naturally extends to masks with DETR-Segm
- Slower than YOLO (slower convergence, requires strong augmentation, large batch), but elegant and modular

**Mask DETR / DETR with mask prediction:**
- Adds FPN + mask head to DETR
- Predicts masks for each object query via iterative refinement
- Similar accuracy to Mask R-CNN but end-to-end differentiable

**SAM (Segment Anything Model):**
- Zero-shot instance segmentation: given any prompt (point, box, text), predicts mask
- Transformer-based encoder-decoder; trained on 1B masks
- Can segment novel objects without finetuning
- Recently SAM 2: extends to videos with streaming memory (6 frames, 1 prompted)

## Common gotchas / interview framings
- Instance vs semantic: semantic = all people are "person"; instance = person1, person2, person3
- Mask R-CNN ROI Align crucial: standard ROI Pooling loses alignment, hurting mask quality
- DETR slower than Faster R-CNN / YOLO; needs warmup, large batches, longer training
- DETR: queries are learnable; each query predicts one object (or background if unused)
- Hungarian matching: many queries won't match any target; loss from background queries
- SAM zero-shot requires prompt engineering; fine-tuning on domain-specific data often needed
- Panoptic segmentation: combines semantic (stuff = background) + instance (things = objects)

## See also
- [[httpsarxivorgabs170306870]]
- [[httpsarxivorgabs200512677]]
- [[httpsgithubcomfacebookresearchdetectron2]]
- [[httpspaperswithcodecomtaskinstance-segmentation]]
- [[httpsarxivorgabs221109437]]

## Sources
See frontmatter `sources:`.
