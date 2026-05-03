---
id: 95a0b058-ba47-4846-b6dd-55c44d68e916
title: Token interleaving and mixed modality
track: ai-llm-engineer
topic: multi-modal-models-vision-language-models
difficulty: 3
tags:
- multimodal-fusion
- token-interleaving
- early-fusion
- cross-modal-reasoning
- mixed-tokens
aliases:
- interleaved tokens
- early fusion
- cross-modal fusion
- image-text mixing
sources:
- url: https://developers.openai.com/cookbook/examples/multimodal/document_and_multimodal_understanding_tips
  label: Getting the Most out of GPT-5.4 for Vision and Document Understanding
- url: https://ai.google.dev/gemini-api/docs/video-understanding
  label: Video understanding | Gemini API
- url: https://jina.ai/vision-encoder-survey.pdf
  label: 'Vision Encoders in Vision-Language Models: A Survey'
cards:
- id: e2b0d7ab-c150-43c2-ad9d-ca0fdd8ff0c3
  type: flip
  front: What is the core advantage of token interleaving over late fusion (separate encoders + concatenation)?
  back: 'Token interleaving allows early-layer cross-modal attention: image patches can attend to text
    context and vice versa. Late fusion only fuses at the final layer, missing fine-grained cross-modal
    reasoning opportunities.'
- id: 694a83d8-9791-43a3-a8d2-1374ea14ebf1
  type: flip
  front: Describe a simple token interleaving strategy for an image with caption.
  back: 'Convert image to $N$ patch embeddings $[v_1, \ldots, v_N]$, tokenize caption to $[t_1, \ldots,
    t_M]$, then concatenate or group: e.g., $[\text{[CLS]}, v_1, v_2, \ldots, v_N, \text{[SEP]}, t_1,
    t_2, \ldots, t_M]$. Feed to unified transformer.'
- id: a9445fd2-4287-4e7c-aa66-21bdf1d70abf
  type: flip
  front: What computational challenge arises from dense interleaving (many image + text tokens)?
  back: 'Self-attention is $O(n^2)$ where $n = N + M$. For high-res images (10k+ patches) + long text,
    this explodes. Solutions: sparse attention, hierarchical pooling, multi-head attention with local
    windows, or query-key compression.'
- id: fb42a9eb-2f60-4c19-a4aa-739aa399c0dc
  type: mcq
  front: Why is modality normalization important in interleaved architectures?
  back: Image tokens (e.g., 100k+ dimensions summed) can dominate text tokens (e.g., 10k dimensions) in
    magnitude. Layer norm per modality or learnable scaling factors prevent one modality from drowning
    out the other.
  choices:
  - key: a
    text: To ensure tokens from different modalities have similar gradient scales
    correct: false
  - key: b
    text: To reduce the vocabulary size of the model
    correct: false
  - key: c
    text: To enable better compression for storage
    correct: false
  - key: d
    text: To speed up attention computation
    correct: false
- id: fa0aa516-3ccc-4f8d-9d0b-85a9c8874f8e
  type: flip
  front: How do GPT-4o and Gemini 2.5 handle dense video frames without exploding token counts?
  back: '**Selective sampling**: key-frame extraction (e.g., 1 frame per second), learned frame selection
    based on motion, or temporal pooling. GPT-4o samples keyframes; Gemini 2.5 uses hierarchical fusion
    with audio-visual cues for moment retrieval.'
---

## Intuition

Instead of processing images and text separately then combining them (late fusion), token interleaving mixes image patches and text tokens in a single sequence fed to a unified transformer. This allows early layers to see cross-modal context, enabling richer fusion and better reasoning about how visual and textual information relate.

## Detail

Token interleaving architectures treat all modalities uniformly:
1. Convert image → patch embeddings: $[v_1, v_2, \ldots, v_N]$
2. Tokenize text: $[t_1, t_2, \ldots, t_M]$
3. Interleave: $[v_1, t_1, v_2, t_2, v_3, \ldots]$ or grouped: $[t_1, v_1, t_2, v_2, \ldots]$
4. Process jointly through transformer with self-attention

**Cross-attention variants**: Some models use separate vision & text transformers, then fuse via:
- Cross-attention layers: text queries, image values/keys
- Gated fusion: learnable gates control flow between modalities
- Early + late fusion: patch embeddings in transformer, but final hidden states also fused

Benefits:
- Allows attending to specific image regions when predicting text tokens
- Enables reasoning about relationships (e.g., "what text labels does this region contain?")
- Unified representation supports streaming (interleave tokens as they arrive)

## Common gotchas / interview framings
- Computational cost: cross-attention over all patch-token pairs is $O(N \cdot M)$ where $N$ is patches, $M$ is text length. GPT-4o/Gemini 2.5 use selective attention or hierarchical fusion to reduce cost
- Modality imbalance: if image tokens >> text tokens, text may be drowned out; needs careful normalization
- Interleaving order matters: grouped by region, temporal order, or learned soft assignment
- Vision transformers (ViT) already have self-attention; adding cross-modal self-attention creates bidirectional flow (image→text and text→image)
- Dense video frames require sparse sampling or hierarchical pooling; GPT-4o samples key frames, Gemini 2.5 uses learnable frame selection

## See also
- [[token-fusion]]
- [[early-fusion]]
- [[cross-modal-reasoning]]
- [[multimodal-transformer]]
- [[vision-language-pretraining]]
- [[modality-mixing]]

## Sources
See frontmatter `sources:`.
