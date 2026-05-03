---
id: b41b3719-5b72-4021-96ff-abcfa97b06a5
title: Visual-language alignment (CLIP, ALIGN)
track: ai-llm-engineer
topic: multi-modal-models-vision-language-models
difficulty: 3
tags:
- contrastive-learning
- CLIP
- alignment
- zero-shot
- image-text-pairs
aliases:
- CLIP contrastive loss
- dual-tower alignment
- image-text matching
sources:
- url: https://jina.ai/vision-encoder-survey.pdf
  label: 'Vision Encoders in Vision-Language Models: A Survey'
- url: https://arxiv.org/pdf/2502.14786
  label: 'SigLIP2: Dual-Tower Multilingual Vision-Language Encoders (Feb 2025)'
- url: https://www.emergentmind.com/topics/siglip2-encoder
  label: SigLIP2 Encoder for Multimodal AI
cards:
- id: 319574e6-24b0-4a49-9b03-59fd871fff40
  type: flip
  front: Explain the CLIP contrastive loss in one sentence.
  back: Learn embeddings where matched (image, text) pairs have high dot product similarity, while unmatched
    pairs have low similarity, using cross-entropy loss over all in-batch negatives.
- id: 105faf8b-c9f9-4fc5-9ad4-ab9928045462
  type: flip
  front: Why does CLIP enable zero-shot classification?
  back: The shared embedding space allows you to encode class descriptions as text, then compute their
    embeddings. An image's embedding is compared to all class embeddings; the nearest class wins. No task-specific
    training needed.
- id: e4775865-75f9-4d54-990d-091c7255e7e7
  type: flip
  front: What is the key architectural difference between dual-tower (CLIP) and merged (late-fusion) approaches?
  back: 'Dual-tower: separate vision & text encoders, align in a shared space, **efficient** for retrieval
    (pre-compute all embeddings). Merged: concatenate patches + text tokens early, process jointly, **better
    reasoning** but no pre-computation.'
- id: d7259c25-03fd-44c6-90e7-02e3923f14fb
  type: mcq
  front: What problem does SigLIP-2 solve that original CLIP had?
  back: 'CLIP sigmoid loss vs softmax: SigLIP uses sigmoid (binary classification per pair), not softmax
    over all negatives. More stable at large batch sizes, doesn''t require synchronized batching across
    GPUs, better for multilingual pretraining.'
  choices:
  - key: a
    text: Adds video support with temporal fusion
    correct: false
  - key: b
    text: Sigmoid loss is more stable than softmax at scale; doesn't require synchronized batching
    correct: false
  - key: c
    text: Uses a CNN instead of ViT backbone
    correct: false
  - key: d
    text: Automatically generates captions during inference
    correct: false
- id: 61d76861-9661-4ecf-a8b5-9b9231ee4900
  type: flip
  front: How does hard negative mining improve contrastive alignment?
  back: Instead of random negatives, explicitly sample image-text pairs that are hard to distinguish (e.g.,
    similar images with different captions). Forces the model to learn fine-grained distinctions, improving
    downstream performance.
---

## Intuition

CLIP revolutionized vision-language understanding by learning a shared embedding space where images and their text descriptions cluster together. Instead of labeling images directly, it trains on internet-scale (image, text) pairs and learns to align their embeddings via contrastive loss. This enables zero-shot classification: describe a class in text, compute its embedding, and find nearest image embeddings.

## Detail

CLIP uses a **dual-tower** architecture:
- Vision encoder $V_{\theta}$ projects image $x$ to embedding $\mathbf{v} = V_{\theta}(x) \in \mathbb{R}^{d}$
- Text encoder $T_{\phi}$ projects text $t$ to embedding $\mathbf{t} = T_{\phi}(t) \in \mathbb{R}^{d}$
- Contrastive loss: $\mathcal{L} = -\log \frac{\exp(\text{sim}(\mathbf{v}, \mathbf{t}) / \tau)}{\sum_k \exp(\text{sim}(\mathbf{v}, \mathbf{t}_k) / \tau)}$ where $\tau$ is temperature.

Modern variants (SigLIP-2, ALIGN) add:
- **Captioning losses**: learn to predict full captions, not just binary matching
- **Self-supervised losses**: masked prediction, self-distillation from a momentum encoder
- **Multilingual pretraining**: SigLIP-2 aligns embeddings across 100+ languages
- **Hard negative mining**: explicit sampling of difficult negatives improves convergence

## Common gotchas / interview framings
- CLIP alignment is **not transitive**: aligned image-text doesn't guarantee aligned text-text embeddings (requires separate text-only pretraining)
- Temperature $\tau$ is critical: too high → all pairs equally probable, too low → collapsed gradients
- Zero-shot classification requires careful prompt engineering (e.g., "a photo of a {class}" vs. raw class name)
- SigLIP uses sigmoid loss instead of softmax, more stable at scale
- Downstream task performance depends heavily on pretraining data distribution (e.g., CLIP struggles on medical images not in training set)

## See also
- [[clip-model]]
- [[contrastive-learning]]
- [[zero-shot-classification]]
- [[dual-tower-architecture]]
- [[siglip]]
- [[alignment-loss]]

## Sources
See frontmatter `sources:`.
