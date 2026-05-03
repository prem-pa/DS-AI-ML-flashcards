---
id: 5c50ffab-50fe-48bd-8de2-831b91fd081c
title: Cross-attention vs self-attention
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 3
tags:
- attention-mechanism
- cross-attention
- encoder-decoder
- self-attention
- information-routing
aliases:
- cross-attention
- encoder-decoder attention
sources:
- url: https://arxiv.org/abs/1706.03762
  label: Attention is All You Need
- url: https://towardsdatascience.com/cross-attention-in-transformer-architecture-d808ffbc8b8b
  label: Cross-Attention in Transformer Architecture
cards:
- id: ced9184a-154d-41a7-83c5-c84babe01632
  type: flip
  front: Explain the key difference between self-attention and cross-attention in terms of where Q, K,
    V come from.
  back: In self-attention, Q, K, and V all originate from the same input sequence. In cross-attention,
    Q comes from one sequence (e.g., decoder) while K and V come from a different sequence (e.g., encoder
    output). This allows one sequence to attend to and retrieve information from another.
- id: f8008f67-e128-46e3-ab0b-7d628c9ef711
  type: flip
  front: In an encoder-decoder architecture, why doesn't cross-attention need causal masking?
  back: Cross-attention connects decoder positions to encoder outputs. The encoder has already computed
    all positions in parallel (no autoregressive generation), so all encoder positions are available to
    the decoder. Causal masking only applies to self-attention within the decoder to enforce left-to-right
    generation.
- id: b9191518-3d06-4d67-83dc-4da54947155a
  type: mcq
  front: In a machine translation model, what does cross-attention allow the decoder to do?
  back: 'Correct: (b). Cross-attention allows the decoder to query the encoder''s output at each generation
    step, selectively retrieving relevant source information for translation. (a) is vocabulary-dependent,
    not cross-attention''s role. (c) is intra-decoder communication, not cross-sequence. (d) is not cross-attention''s
    purpose.'
  choices:
  - key: a
    text: Generate new vocabulary tokens not in training data
    correct: false
  - key: b
    text: Attend to and retrieve information from the encoder's representation of the source sentence
    correct: true
  - key: c
    text: Communicate with other decoder layers in parallel
    correct: false
  - key: d
    text: Reduce the total number of parameters
    correct: false
- id: 608fca86-670d-45a5-830b-de2e1a72bdcd
  type: flip
  front: How would you use cross-attention in a vision-language model to align text and image representations?
  back: Apply cross-attention where text tokens (Q) query over image patches (K, V). Each text token learns
    to attend to relevant image regions. Alternatively, image patches (Q) attend to text tokens (K, V)
    for grounding visual concepts. Bidirectional cross-attention can also be applied for mutual alignment.
---

## Intuition
Self-attention allows tokens to attend to other tokens within the same sequence. Cross-attention allows tokens from one sequence (e.g., decoder) to attend to tokens from another sequence (e.g., encoder output). This enables encoder-decoder architectures and multimodal fusion.

## Detail
In self-attention, Q, K, V all come from the same input sequence:
$$\text{SelfAttention}(X) = \text{Attention}(XW^Q, XW^K, XW^V)$$

In cross-attention, Q comes from one sequence (target/decoder), while K and V come from another (source/encoder):
$$\text{CrossAttention}(Q, KV) = \text{Attention}(QW^Q, KW^K, VW^V)$$

The decoder generates queries from its hidden states; the encoder provides the key-value pairs. This allows the decoder to selectively retrieve information from encoder outputs. In vision transformers or multimodal models, cross-attention fuses different modalities.

## Common gotchas / interview framings
- **Causality:** Self-attention in decoders needs causal masking. Cross-attention doesn't—decoder can look at all encoder tokens.
- **Sequence length mismatch:** Cross-attention handles different lengths for Q vs. KV. Crucial for sequence-to-sequence (translation, summarization).
- **Gradient flow:** Cross-attention directly connects encoder and decoder; gradients flow back to encoder outputs even if self-attention is blocked.
- **Vision-language models:** Cross-attention between text and vision tokens enables image captioning and VQA.
- **When to use which:** Self-attention for modeling within-sequence dependencies, cross-attention to fuse external information.

## See also
- [[scaled-dot-product-attention-derivation]]
- [[multi-head-attention-and-head-roles]]

## Sources
See frontmatter `sources:`.
