---
id: ff4f468a-e3a1-4043-b607-6f53ccb7a512
title: Multi-head attention and head roles
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 3
tags:
- mha
- attention-heads
- multi-head
- representation-capacity
- interpretability
aliases:
- MHA
- multi-head
- attention heads
sources:
- url: https://arxiv.org/abs/1706.03762
  label: Attention is All You Need
- url: https://distill.pub/2016/attention-and-augmentation/
  label: 'Distill.pub: Attention and Augmentation'
cards:
- id: 99b151fd-ee19-446d-b7e0-ef4e5281642b
  type: flip
  front: Explain what multi-head attention computes and why it uses h separate projection matrices instead
    of one.
  back: Multi-head attention applies h independent attention mechanisms in parallel, each with separate
    Q, K, V projections. The outputs are concatenated and projected. Multiple projections allow the model
    to learn h different representation subspaces; a single large attention head with one projection cannot
    simultaneously learn these diverse attention patterns.
- id: bdbf463c-6a62-414f-bfcd-6d76a0a9761c
  type: flip
  front: What do different attention heads typically specialize in learning?
  back: 'Different heads learn complementary patterns: some attend to nearby tokens (local syntax), others
    to distant tokens (long-range dependencies); some focus on subject-verb relations, others on adjective-noun
    modifiers. This specialization emerges during training without explicit supervision.'
- id: 9410eda6-496c-450b-884c-ed5be97bc74f
  type: mcq
  front: If you have 8 attention heads with d_model=512, what is the dimension of each head?
  back: 'Correct: (b). Standard multi-head attention divides the model dimension equally among heads.
    With 8 heads and d_model=512, each head operates on dimension 512/8=64. This keeps total computation
    similar to a single-head model while enabling diversity.'
  choices:
  - key: a
    text: 512 (each head is full dimension)
    correct: false
  - key: b
    text: 64 (512 / 8)
    correct: true
  - key: c
    text: 4096 (512 * 8)
    correct: false
  - key: d
    text: 256 (512 / 2)
    correct: false
---

## Intuition
Multi-head attention runs h independent attention mechanisms in parallel, each focusing on different parts of the representation space. This allows the model to attend to multiple input positions simultaneously in different subspaces.

## Detail
Multi-head attention computes:

$$\text{MultiHead}(Q, K, V) = \text{Concat}(\text{head}_1, \ldots, \text{head}_h)W^O$$

where each head $\text{head}_i = \text{Attention}(Q W_i^Q, K W_i^K, V W_i^V)$. Each head uses separate learned projection matrices $W_i^Q, W_i^K, W_i^V \in \mathbb{R}^{d \times d/h}$. The output projections $W^O$ recombine information from all heads. Common head counts: 8 (BERT-base), 12 (BERT-large), 32 (GPT-3).

## Common gotchas / interview framings
- **Head specialization:** Different heads learn different patterns—some attend locally, others globally; some focus on syntax, others semantics.
- **Not all heads are equal:** Pruning studies show many heads contribute little; importance varies widely across heads and layers.
- **Why not one big head?** h heads of size d/h is roughly equivalent to one head of size d computationally, but multi-head learns richer features due to multiple independent projections.
- **Interpretability trap:** While individual heads sometimes have clear interpretations (e.g., subject-object binding), they often don't—ensemble effects matter more than individual head meanings.
- **Head redundancy:** Studies show high redundancy; architectures like GQA reduce head counts without much accuracy loss.

## See also
- [[scaled-dot-product-attention-derivation]]
- [[grouped-query-attention-gqa-and-multi-query-attention-mqa]]
- [[cross-attention-vs-self-attention]]

## Sources
See frontmatter `sources:`.
