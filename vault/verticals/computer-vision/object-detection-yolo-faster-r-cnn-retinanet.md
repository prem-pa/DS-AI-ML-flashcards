---
id: 5bff351b-bdfc-442b-8ed9-d361b64be1b0
title: Object detection (YOLO, Faster R-CNN, RetinaNet)
track: verticals
topic: computer-vision
difficulty: 3
tags:
- detection
- real-time
- anchor-free
- bounding-boxes
- speed-accuracy-tradeoff
aliases:
- YOLO
- Faster R-CNN
- RetinaNet
- YOLOv12
sources:
- url: https://arxiv.org/abs/1506.02640
  label: 'You Only Look Once: Unified, Real-Time Object Detection'
- url: https://arxiv.org/abs/1612.08242
  label: 'Faster R-CNN: Towards Real-Time Object Detection with Region Proposal Networks'
- url: https://arxiv.org/abs/1708.02002
  label: Focal Loss for Dense Object Detection (RetinaNet)
- url: https://docs.ultralytics.com/models/yolo12/
  label: 'YOLOv12: Attention-Centric Real-Time Object Detectors'
cards:
- id: 8bcfddf7-70a1-4279-8994-be16c1d3c33c
  type: mcq
  front: Which detector is fastest but trades accuracy, and which is slowest but most accurate?
  back: 'YOLO: ~100 fps, ~50 mAP (speed-first). Faster R-CNN: ~0.5 fps, ~76 mAP (accuracy-first). RetinaNet:
    ~5 fps, ~59 mAP (middle ground via focal loss).'
  choices:
  - key: a
    text: YOLO fastest, RetinaNet slowest
    correct: false
  - key: b
    text: YOLO fastest, Faster R-CNN slowest
    correct: true
  - key: c
    text: RetinaNet fastest, Faster R-CNN slowest
    correct: false
  - key: d
    text: Faster R-CNN fastest, YOLO slowest
    correct: false
- id: 16bd4470-a222-4afb-b420-c841d4ad9a77
  type: flip
  front: What is focal loss and why does it help RetinaNet work better than YOLO?
  back: Focal loss downweights easy negative examples (background) that dominate training, focusing on
    hard positives. YOLO suffers from class imbalance (999:1 background:object); focal loss rebalances
    this, improving accuracy without complex proposal mechanisms.
- id: 80a3cdc1-69f2-4928-a594-6dd05a68213b
  type: flip
  front: Explain the two stages in Faster R-CNN and why they are separate.
  back: 'Stage 1 (RPN): generates ~2000 region proposals from anchors. Stage 2 (classification): ROI pooling
    extracts fixed-size features, then classifies proposals and refines boxes. Separation allows RPN to
    learn region quality independently, improving proposal quality before final refinement.'
- id: a2221fb5-d94b-4929-8d7b-afb177a55e0b
  type: mcq
  front: 'YOLOv12''s key innovation for efficient large receptive fields is:'
  back: Area Attention avoids O(n²) complexity of full self-attention by dividing feature maps into equal-sized
    regions and computing attention within/across regions. Maintains large effective receptive field with
    low computational cost.
  choices:
  - key: a
    text: Stacked dilated convolutions
    correct: false
  - key: b
    text: 'Area Attention: dividing feature maps into regions for efficient self-attention'
    correct: true
  - key: c
    text: Replacing convolutions with GraphConvNets
    correct: false
  - key: d
    text: Multi-head grouped convolutions
    correct: false
- id: d76c5945-75d7-42e5-8d86-55804742914c
  type: flip
  front: Name the primary bottleneck in traditional YOLO pipelines that YOLO26 addresses.
  back: 'Non-Maximum Suppression (NMS): post-processing step that sequentially suppresses overlapping
    boxes. YOLO26 removes NMS entirely via end-to-end learnable decoding, reducing latency and simplifying
    deployment.'
---

## Intuition

Object detection finds and labels objects in images using bounding boxes. Three paradigms: (1) one-stage detectors (YOLO) regress boxes directly—fast but less accurate; (2) two-stage detectors (Faster R-CNN) propose regions first, then refine—slower but more accurate; (3) attention-based detectors (DETR, YOLOv12) use transformers for set prediction.

## Detail

**YOLO (You Only Look Once):**
- Single-stage: divide image into grid, predict class + bbox offset per grid cell
- Real-time: 45-155 FPS on GPU depending on variant
- YOLOv12 (2025): introduces Area Attention for efficient large receptive fields; R-ELAN for feature aggregation; ~2% mAP improvement over v11
- Trade-off: lower accuracy than two-stage but massively faster

**Faster R-CNN (Two-Stage):**
- Region Proposal Network (RPN) generates candidate boxes; ROI pooling extracts fixed-size features
- Second stage classifies proposals and refines boxes
- Higher accuracy (~76 mAP COCO) but 0.1–1 fps depending on backbone
- Standard for object detection in research

**RetinaNet (Focal Loss):**
- Single-stage detector addressing class imbalance via focal loss
- Focal loss downweights easy negatives, focuses on hard examples
- Achieves ~59 mAP at 5 fps; bridges one-stage/two-stage gap
- Outperforms YOLO and matches Faster R-CNN accuracy at higher speed

**Modern Evolution (YOLOv12):**
- Area Attention: divides feature maps into equal-sized regions for efficient self-attention
- Removes positional encoding; uses FlashAttention for speed
- Handles multi-scale detection, instance segmentation, pose estimation
- YOLO26 (Sept 2025): removes NMS bottleneck, introduces MuSGD optimizer

## Common gotchas / interview framings
- YOLO trades accuracy for speed; Faster R-CNN opposite
- Class imbalance is critical in dense detection; focal loss fixes this
- Anchor-free detectors (YOLO center point, FCOS) simplify design
- NMS post-processing is a bottleneck; recent models explore NMS-free decoding
- Evaluate on COCO metrics: AP (average precision), AR (recall), APsmall/APmedium/APlarge
- YOLOv12 attention mechanism avoids quadratic complexity of standard self-attention

## See also
- [[httpsarxivorgabs150602640]]
- [[httpsarxivorgabs161208242]]
- [[httpsarxivorgabs170802002]]
- [[httpsdocsultralyticscommodelsyolo12]]
- [[httpsgithubcomsunsmarterjieyolov12]]

## Sources
See frontmatter `sources:`.
