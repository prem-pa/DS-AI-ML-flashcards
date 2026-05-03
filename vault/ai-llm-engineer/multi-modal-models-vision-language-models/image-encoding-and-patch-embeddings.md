---
id: 1588f300-4431-4393-b097-f57aed62bfb0
title: Image encoding and patch embeddings
track: ai-llm-engineer
topic: multi-modal-models-vision-language-models
difficulty: 3
tags:
- patch-embeddings
- vision-encoder
- ViT
- image-encoding
- tokenization
aliases:
- image tokenization
- ViT patches
- spatial embeddings
sources:
- url: https://jina.ai/vision-encoder-survey.pdf
  label: 'Vision Encoders in Vision-Language Models: A Survey'
- url: https://github.com/huggingface/pytorch-image-models
  label: PyTorch Image Models (timm) - ViT & Vision Encoders
- url: https://github.com/huggingface/transformers/blob/main/src/transformers/models/siglip/modeling_siglip.py
  label: HuggingFace SigLIP Implementation
cards:
- id: 2db54e15-77dc-4348-b009-69872c72b454
  type: flip
  front: What is the core idea behind patch-based image encoding in ViTs?
  back: Divide the image into non-overlapping patches (e.g., 16×16), linearly project each patch to a
    fixed dimension, and add positional encodings. This converts the image into a sequence of tokens that
    transformer layers can process, similar to word tokens in NLP.
- id: 4ccab698-6a51-499d-8534-76b7d40561be
  type: flip
  front: How do vision encoders balance resolution vs. token count in VLMs?
  back: '**Trade-off**: Higher resolution (e.g., 1344×1344) with adaptive patching (e.g., smaller patches
    on key regions) increases quality but token usage. SigLIP-2 and GPT-4o use ~2000-10000 tokens for
    high-res images. Lower resolution (336×336) uses ~576 tokens. Some models compress via learned pooling
    or dynamic patch selection.'
- id: 44a2d644-f585-4652-96ab-929a062100f5
  type: flip
  front: What role does positional encoding play in patch embeddings?
  back: 'Positional encodings preserve spatial relationships between patches. Methods include: absolute
    (learned grids), relative (2D-RoPE), or implicit (spatial transformer). Without it, the model loses
    2D structure and treats patches as an unordered set.'
- id: c3d3d107-4331-4555-a0c5-a0e8fa2e0391
  type: mcq
  front: Why does SigLIP-2 (Feb 2025) outperform larger ViT backbones on VLM benchmarks despite having
    fewer parameters?
  back: 'SigLIP-2 uses superior pretraining: dual-tower contrastive learning + captioning-based objectives
    + self-supervised losses (masked prediction, self-distillation) + online data curation. Training methodology
    now outperforms parameter scaling.'
  choices:
  - key: a
    text: It uses a deeper transformer architecture
    correct: false
  - key: b
    text: Superior pretraining methodology (contrastive + captioning + self-supervised objectives) outweighs
      parameter disadvantage
    correct: false
  - key: c
    text: It processes images at 10x higher resolution
    correct: false
  - key: d
    text: It uses ensembling with 4 different encoders
    correct: false
- id: 495dd3bb-2d4c-45d4-b218-ab86e06fdba8
  type: flip
  front: What is the relationship between patch size and downstream task performance?
  back: 'Smaller patches (8×8): better for OCR, dense tasks (segmentation), but high token cost. Larger
    patches (16×16): efficient, good for image classification. Adaptive/dynamic patching balances both
    but adds complexity.'
---

## Intuition

Vision-language models must project images into a shared token space with text. Rather than flattening entire images, modern approaches divide images into fixed-size patches (typically 14×14 or 16×16 pixels), linearly embed each patch into a hidden dimension, and optionally add positional encodings. This mirrors how text is tokenized.

## Detail

Vision Transformers (ViT) pioneered patch-based encoding: divide $H \times W \times C$ image into $P^2$ patches, project each to $D$ dimensions: $z^{(0)} = W_p x_p^{(l)} \in \mathbb{R}^{N \times D}$ where $N = (HW)/P^2$. Modern variants (SigLIP-2, EVA) adapt this with different backbone scales (ViT-B/16 86M → ViT-g/16 1B parameters) and resolution strategies. SigLIP-2 adds self-supervised pretraining objectives to stabilize dense features. Some models use 2D-RoPE or complex positional embeddings to preserve spatial locality. Resolution impacts efficiency: 336×336 = 576 tokens vs 1344×1344 = 14,400 tokens.

## Common gotchas / interview framings
- Patch size trade-off: smaller patches (8×8) preserve fine detail for OCR but explode token count; larger patches (16×16) are efficient but lose small object detection
- Linear projection assumes patch independence; learned projections or CNNs can capture local spatial structure
- Position encoding can be frozen (ROPE) or learnable; absolute vs relative encodings affect scaling to new resolutions
- SigLIP-2 (Feb 2025) outperforms older ViT-based encoders on VLM tasks despite fewer parameters via superior pretraining
- Many models discard patch embeddings after alignment layer; others preserve them for dense prediction (segmentation, localization)

## See also
- [[vision-transformer]]
- [[patch-embedding]]
- [[spatial-encoding]]
- [[siglip-2]]
- [[image-feature-extraction]]
- [[token-projection]]

## Sources
See frontmatter `sources:`.
