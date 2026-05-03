---
id: 86fc8907-3ddb-4456-aeee-9b5baad88727
title: Visual question answering (VQA)
track: verticals
topic: computer-vision
difficulty: 3
tags:
- vision-language
- multimodal
- vqa
- attention
- reasoning
aliases:
- VQA
- visual reasoning
- image-question-answer
sources:
- url: https://arxiv.org/abs/1505.00468
  label: 'VQA: Visual Question Answering (Antol et al., 2015)'
- url: https://visualqa.org/
  label: 'VQA: Large-Scale Visual Question Answering Dataset'
- url: https://arxiv.org/abs/1704.03162
  label: Bottom-Up and Top-Down Attention for Image Captioning and Visual Question Answering
- url: https://arxiv.org/abs/2010.01057
  label: 'LXMERT: Learning Cross-Modality Encoder Representations from Transformers'
cards:
- id: 1f61509f-36df-4822-93a1-427760bf2f69
  type: mcq
  front: What is the primary advantage of bottom-up attention over grid-based features in VQA?
  back: 'Bottom-up attention (Faster R-CNN proposals): ~36 object regions per image. More interpretable:
    each region corresponds to an object (e.g., "person", "car"). Grid features: spatial but may attend
    to background or edges. Bottom-up regions better align with semantic VQA questions.'
  choices:
  - key: a
    text: Grid features are more interpretable
    correct: false
  - key: b
    text: Bottom-up attention (region proposals) focuses on objects, reducing noise and improving reasoning
    correct: true
  - key: c
    text: Bottom-up attention is faster
    correct: false
  - key: d
    text: Both approaches are equivalent
    correct: false
- id: 700df176-dad8-482f-8247-cbf6c37d5b4c
  type: flip
  front: Describe the VQA v2 dataset and why it addressed a critical flaw in VQA v1.
  back: 'VQA v1: answer bias (e.g., "yes" appears in 50% of QA pairs). Model could achieve high accuracy
    by guessing "yes". VQA v2: balanced dataset; 3 different questions per image to ensure diversity.
    Reduced dataset artifacts, more faithful evaluation of reasoning.'
- id: 605c733b-4ec0-4339-81ae-de74d7afc231
  type: flip
  front: How do modern VQA models like LXMERT achieve cross-modality understanding?
  back: 'LXMERT: separate transformer streams for vision (image regions) and language (question tokens).
    Co-attentional transformer layer enables vision-to-language and language-to-vision attention. Fusion
    happens in latent space, not at input level.'
- id: 508ef33f-1c6f-4f2f-8e2d-f79e90fa3976
  type: mcq
  front: Which of these VQA question types is most challenging for current models?
  back: 'Complex reasoning: requires multi-hop attention, negation handling, implicit comparison. Models
    tend to exploit dataset artifacts or surface patterns. Single-hop (object detection) and basic counting
    are more solved.'
  choices:
  - key: a
    text: Object detection ("What color is the car?")
    correct: false
  - key: b
    text: Counting ("How many people?")
    correct: false
  - key: c
    text: 'Complex reasoning (negation, comparison, multiple hops: "Is there a larger dog than a cat?")'
    correct: true
  - key: d
    text: Spatial ("Where is the person?")
    correct: false
- id: 9e958473-eb06-407e-bbdf-db28bdcc156c
  type: flip
  front: How does CLIP transfer to VQA, and what modifications are needed?
  back: 'CLIP zero-shot: embed question as text (e.g., "a dog"), embed image, compute similarity. Naive
    VQA uses CLIP but accuracy ~50% (below fine-tuned models ~70%). Modification: add question-image fusion
    layers, finetuning adapter (LoRA) on VQA dataset; combine CLIP features with task-specific reasoning.'
---

## Intuition

Visual Question Answering (VQA) combines vision and language understanding: given an image and a question ("How many people are in the image?"), the model generates an answer. Requires object detection, scene understanding, counting, reasoning over relationships, and commonsense knowledge. Benchmark: VQA v2 dataset (65k images, 614k questions).

## Detail

**Problem Formulation:**
- Input: image I, question Q (text)
- Output: answer A (text or from vocabulary of ~3k answers)
- Challenges: compositional reasoning ("What color is the object on the left?"), counting, spatial relationships, commonsense

**Architecture Components:**

**1. Image Encoder:**
- CNN (ResNet, ViT) extracts features
- Bottom-up attention (Faster R-CNN): detects objects, outputs region features (e.g., 36 regions per image)
- Region features preserve object information (more interpretable than grid features)

**2. Question Encoder:**
- LSTM or Transformer encodes question text to embedding
- Attention mechanism learns to focus on question words relevant to visual features

**3. Fusion / Reasoning:**
- Bilinear pooling: multiply image features by question features (captures interactions)
- Transformer cross-attention: questions attend to visual regions
- Iterative reasoning: apply transformer decoder multiple times (ViLBERT, LXMERT)

**4. Answer Decoder:**
- MLP classifier: output probability over answer vocabulary (~3k answers)
- Or generative: LSTM decodes answer token-by-token

**Modern Approaches:**
- LXMERT: cross-modality transformer; separate streams for image/text, cross-attention layer
- ViLBERT: vision + language BERT; co-attentional layer between modalities
- Large LMs + vision encoders (LLaVA, GPT-4V): leverage pretrained LLM for reasoning

## Common gotchas / interview framings
- VQA v1 had answer bias (e.g., "yes" appears in 50% of questions); VQA v2 balanced
- Reasoning types: object detection ("what"), counting ("how many"), spatial ("where"), comparison ("which")
- Attention visualization: show which image regions the model attended to for each question word
- Compositionality: models struggle with complex questions (negation, comparison, multiple hops)
- Bias & fairness: VQA trained on web images; gender bias in person-related questions
- Generalization: VQA trained on natural images; fails on charts, diagrams, medical images
- Dataset artifacts: models exploit statistical patterns rather than true reasoning (e.g., "What color?" → "white" likely correct)
- Evaluation: accuracy (exact match to ground truth) or BLEU/CIDEr for generative answers

## See also
- [[httpsarxivorgabs150500468]]
- [[httpsvisualqaorg]]
- [[httpsarxivorgabs170403162]]
- [[httpsarxivorgabs201001057]]
- [[httpspaperswithcodecomtaskvisual-question-answering]]

## Sources
See frontmatter `sources:`.
