---
id: cf691905-ec07-426f-889c-3d4fdf66a3cb
title: Scaled dot-product attention derivation
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- attention-mechanism
- sdpa
- query-key-value
- softmax-norm
- transformers
- backprop
aliases:
- SDPA
- scaled attention
- QKV attention
sources:
- url: https://arxiv.org/abs/1706.03762
  label: Attention is All You Need (Vaswani et al.)
- url: https://learnopencv.com/attention-mechanism-in-transformers/
  label: 'Learn OpenCV: Attention in Transformers'
- url: https://github.com/Dao-AILab/flash-attention
  label: FlashAttention Implementation
cards:
- id: 209be39e-3d3a-44bc-8913-f66ab2975d19
  type: flip
  front: Write the scaled dot-product attention formula and explain why the sqrt(d) term is essential.
  back: $\text{Attention}(Q, K, V) = \text{softmax}\left(\frac{QK^T}{\sqrt{d}}\right)V$. The sqrt(d) term
    normalizes the dot product magnitude so it remains O(1) as d grows, keeping softmax gradients stable
    during backprop. Without it, large d causes dot products to explode, pushing softmax into flat regions
    with vanishing gradients.
- id: d1c4e918-404c-4669-8d8f-6f6fdc68499a
  type: flip
  front: How do Q, K, V projections enable different semantic roles in attention?
  back: 'Q, K, V are learned linear transformations of the input. Q captures what the position is looking
    for, K and V come from the same source but carry different roles: K for matching (similarity), V for
    the actual information retrieved. This separation lets the model learn to route queries to different
    information subspaces.'
- id: 64c56e38-af5f-445f-8088-3424d02efcf6
  type: mcq
  front: In scaled dot-product attention, what is the primary reason we divide by sqrt(d) before softmax?
  back: 'Correct: (b). Dot products of d-dimensional vectors have variance proportional to d. Dividing
    by sqrt(d) ensures the variance stays ~1 regardless of embedding dimension, keeping gradients in the
    steep region of softmax for effective learning. (a) is wrong—softmax already sums to 1. (c) is positional
    encoding''s job. (d) is not the purpose of scaling.'
  choices:
  - key: a
    text: To make the attention weights sum to exactly 1
    correct: false
  - key: b
    text: To keep dot-product variance O(1) and maintain stable softmax gradients
    correct: true
  - key: c
    text: To enforce positional biasing in sequence models
    correct: false
  - key: d
    text: To reduce memory usage in transformer layers
    correct: false
- id: 0c1164fe-2eb7-4ba2-b446-edc3b8928b4d
  type: flip
  front: How would you modify scaled dot-product attention to enforce causality in a decoder (prevent
    attending to future tokens)?
  back: 'Apply a causal mask: set attention logits (QK^T / sqrt(d)) to -infinity for all future positions
    (i > j) before softmax. This forces softmax to assign zero probability to attending to tokens that
    come after the current position, enforcing autoregressive generation.'
---

## Intuition
Scaled dot-product attention computes a probability distribution over input values weighted by the similarity between a query and each key. The scaling term sqrt(d) prevents gradient collapse in the softmax.

## Detail
Given query matrix $Q \in \mathbb{R}^{n \times d}$, key matrix $K \in \mathbb{R}^{n \times d}$, and value matrix $V \in \mathbb{R}^{n \times d}$, scaled dot-product attention is:

$$\text{Attention}(Q, K, V) = \text{softmax}\left(\frac{QK^T}{\sqrt{d}}\right)V$$

The three linear projections (Q, K, V from input) allow the model to learn different subspaces. The sqrt(d) scaling ensures dot-product magnitudes remain O(1) regardless of embedding dimension, stabilizing softmax gradient flow during backpropagation. The softmax converts scaled similarities to attention weights summing to 1 per query.

## Common gotchas / interview framings
- **Why sqrt(d) and not just d?** Variance of dot products grows with d. With scaling sqrt(d), variance remains ~1, keeping softmax in its steep region for better gradients.
- **Why not just use cosine similarity?** Dot product is cheaper computationally and differentiable everywhere (unlike some normalized forms). Scaling achieves similar effect.
- **Masking for causality:** In decoder self-attention, set future positions to -infinity before softmax to enforce autoregressive dependency.
- **Numerical stability:** In practice, subtract max(QK^T / sqrt(d)) before exp() to avoid overflow in softmax.
- **When does this break?** With very long sequences, O(n^2) memory becomes prohibitive—sparse or linear attention variants address this.

## See also
- [[multi-head-attention-and-head-roles]]
- [[cross-attention-vs-self-attention]]
- [[efficient-attention-flashattention-flashattention-2-flashattention-3]]

## Sources
See frontmatter `sources:`.
