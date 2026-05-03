---
id: f96e76e1-3870-4cac-aed1-6e3cc8873a50
title: Mamba and state-space models (SSMs)
track: research-leaning
topic: recent-architectures-techniques
difficulty: 5
tags:
- sequence-modeling
- linear-time-complexity
- selective-computation
- state-space-models
- transformers-alternative
aliases:
- selective-SSM
- linear attention
- structured state spaces
sources:
- url: https://arxiv.org/abs/2312.00752
  label: 'Mamba: Linear-Time Sequence Modeling with Selective State Spaces'
- url: https://github.com/state-spaces/mamba
  label: Official Mamba GitHub
- url: https://goombalab.github.io/blog/2024/mamba2-part1-model/
  label: State Space Duality (Mamba-2)
cards:
- id: 703f469a-4712-4e08-b14d-9d1f4873c42d
  type: flip
  front: What is the key architectural innovation that makes Mamba faster than Transformers, and what
    is its time complexity?
  back: Mamba makes SSM parameters (A, B, C matrices) input-dependent (selective), allowing the model
    to decide which information to propagate per token. This enables O(n) linear time complexity vs Transformer's
    O(n²) attention, while maintaining performance on information-dense tasks. Selectivity is the mechanism
    that preserves accuracy where other linear models fail.
- id: cd3137b5-3f3c-478c-b5d9-ed0b0c7f495d
  type: mcq
  front: In Mamba SSMs, what is the relationship between selectivity and performance degradation on long
    sequences?
  back: Selectivity is the core mechanism that allows Mamba to maintain performance as sequences grow.
    By making state transitions input-dependent, the model learns when to propagate information and when
    to reset state, mimicking attention gating but at linear cost.
  choices:
  - key: a
    text: Selectivity causes performance degradation as sequences get longer
    correct: false
  - key: b
    text: Selectivity prevents degradation by allowing the model to forget irrelevant information
    correct: true
  - key: c
    text: Selectivity is orthogonal to sequence length; performance depends only on model size
    correct: false
  - key: d
    text: Selectivity is a post-hoc regularization applied only to very long sequences
    correct: false
- id: f586d223-c397-4182-9e29-d5adb6f9e2e7
  type: flip
  front: How does Mamba-2's state space duality relate to Transformers, and what efficiency gain does
    it unlock?
  back: Mamba-2 proved that Transformers can be understood as special cases of SSMs through state space
    duality. This theoretical connection enables structured algorithms that combine the efficiency of
    SSMs with the expressiveness of attention, improving both memory and computational efficiency.
- id: 0cb899bc-6229-426e-a2af-bad0cf4d2073
  type: mcq
  front: Which of these modalities has Mamba been successfully applied to with state-of-the-art or competitive
    results?
  back: 'Mamba has demonstrated strong performance across multiple modalities: language (LLMs), audio
    (speech, music), genomics (DNA sequences), and vision tasks. This suggests SSM selectivity is a general
    principle for sequence modeling, not language-specific.'
  choices:
  - key: a
    text: Language modeling only
    correct: false
  - key: b
    text: Language and audio only
    correct: false
  - key: c
    text: Language, audio, genomics, and vision
    correct: true
  - key: d
    text: Language, vision, and medical imaging, but not audio
    correct: false
- id: 9df96f48-10d6-4c94-a3b7-537ea4d59681
  type: flip
  front: What inference throughput advantage does Mamba achieve over Transformers, and why?
  back: 'Mamba achieves 5× higher inference throughput compared to Transformers. This stems from: (1)
    linear scaling without KV cache quadratic growth, (2) single-pass generation without attention matrix
    materialization, (3) reduced memory bandwidth requirements for longer sequences.'
---

## Intuition

Mamba reformulates SSMs by making their state transition matrices input-dependent, allowing the model to selectively propagate or forget information based on current tokens. Unlike traditional Transformers with quadratic attention, Mamba achieves linear time complexity in sequence length while maintaining competitive or superior performance on real data.

## Detail

Core innovation: SSM parameters (A, B, C matrices) become explicit functions of input at each sequence position. This selectivity enables:
- O(n) time complexity vs O(n²) for attention
- 5× higher inference throughput than Transformers
- Strong performance on long sequences (up to 1M tokens)
- Applications across language, audio, genomics, vision

Mamba-2 (2024) introduced state space duality showing connections to Transformers and enabling efficient algorithms. Mamba-3 extended with improved sequence modeling.

## Common gotchas / interview framings
- Q: "Isn't linear attention already O(n)?" A: True, but Mamba's selectivity maintains accuracy where other linear attention schemes degrade
- Q: "How does Mamba handle bidirectional context?" A: Single-pass causal by design; bidirectional variants use state merging
- Q: "When should you use Mamba vs Transformers?" A: Mamba excels on very long sequences; Transformers still better for short dense sequences requiring fine-grained dependencies
- Scaling behavior: Performance improves with sequence length (unlike some alternatives)
- MoE integration: Recent work combines Mamba with mixture-of-experts routing

## See also
- [[ssm]]
- [[transformer]]
- [[sequence-length-scaling]]
- [[linear-recurrent-models]]
- [[mamba-2]]
- [[mamba-3]]
- [[selective-state-transitions]]

## Sources
See frontmatter `sources:`.
