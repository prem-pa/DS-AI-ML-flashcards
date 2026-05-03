---
id: 5d93812a-84a5-449a-b8f1-083d99e51fe3
title: Scaling laws for long context
track: research-leaning
topic: scaling-laws-emergent-abilities
difficulty: 5
tags:
- scaling-laws
- context-length
- long-range-dependencies
- attention-mechanisms
- token-efficiency
aliases:
- context scaling
- long-sequence modeling
- context-dependent loss
sources:
- url: https://arxiv.org/abs/2402.17268
  label: Context length scaling in modern language models (2024)
- url: https://arxiv.org/abs/2406.04522
  label: On the efficiency of scaling long-context learning (2024)
cards:
- id: f69b5fb2-264c-4713-8331-5d1f21cd52c2
  type: flip
  front: Does loss scale linearly with context length in modern transformers? Explain.
  back: No, scaling is often sub-linear or logarithmic. Models show weaker loss improvement as context
    extends, especially for tasks without strong long-range dependencies. Task structure and attention
    efficiency heavily influence the scaling relationship.
- id: f27bc754-a975-414d-b8f0-981ff336183e
  type: mcq
  front: You want to compare a 4K-context and 32K-context version of the same model. What is the primary
    risk of naively extrapolating loss scaling laws?
  back: Context length scaling is empirically messier than parameter/data scaling. Some tasks show logarithmic
    improvement; others plateau. Models may be undertrained at long contexts or have architectural inefficiencies
    that violate power-law assumptions.
  choices:
  - key: a
    text: The tokenizer will break.
    correct: false
  - key: b
    text: Context scaling is often nonlinear and task-dependent, breaking power-law assumptions that work
      for model size.
    correct: true
  - key: c
    text: Longer context always linearly improves loss by a constant factor.
    correct: false
  - key: d
    text: Position embeddings are incompatible with extended contexts.
    correct: false
- id: e6f7d121-635b-49d9-972d-5d04117fd5c1
  type: flip
  front: Name one architectural technique that helps maintain good scaling as context length increases.
  back: Efficient attention mechanisms (Linear Attention, Ring Attention) or rotary position embeddings
    (RoPE) with interpolation (ALiBi, YaRN). These reduce compute costs from O(L^2) to O(L) or improve
    generalization to unseen lengths, maintaining scaling law properties.
- id: 1ffe1cc1-bcbd-48b6-aca4-b40278490fcf
  type: mcq
  front: A task involves token classification (labeling each token independently). How would you expect
    loss to scale with context length?
  back: Token classification depends primarily on local context (maybe 50–100 tokens). Extended context
    adds irrelevant information; models may not efficiently ignore it, leading to sublinear or plateauing
    loss improvements beyond task-relevant window sizes.
  choices:
  - key: a
    text: Linear improvement; longer context always helps.
    correct: false
  - key: b
    text: Logarithmic or plateau; token classification doesn't benefit from long-range dependencies.
    correct: true
  - key: c
    text: Loss actually increases because longer context adds noise.
    correct: false
  - key: d
    text: Context length is irrelevant for this task.
    correct: false
---

## Intuition

Does the power-law scaling (L ∝ N^{-β}, L ∝ D^{-δ}) still hold when you extend context length (L_ctx)? The question is critical: as models support longer contexts (4K → 32K → 100K tokens), does loss scale linearly with context, or do models hit efficiency cliffs?

## Detail

Recent work (2024–2025) shows:

- **Context length does affect loss, but not always proportionally.** Models often show sub-linear or logarithmic improvement with context length (especially for sparse, reference-heavy tasks)
- **Quality-context tradeoff:** The ability to leverage context (in-context learning, retrieval) requires both model size and sufficient data. Larger models extract more value from extended context
- **Attention efficiency:** Transformers with linear/efficient attention (ALiBi, Ring Attention) maintain scaling laws better as context grows; full attention becomes a compute bottleneck
- **Task-dependent scaling:** Tasks that benefit from long-range coherence (story generation, code with imports) show steeper context-loss slopes; token classification shows weaker dependence

## Common gotchas / interview framings

- **"Can I predict loss for a 100K-context model using a 4K-context fit?"** → Extrapolating context is risky. Context scaling is often nonlinear and task-dependent. Some models show logarithmic improvement; others hit plateaus beyond a certain window
- **"Does longer context always help?"** → No. If information is uniformly random or already compressed in the first few tokens, extended context adds noise. Task structure matters: code with imports benefits; language translation may not
- **"What about position bias and Rotary Embeddings (RoPE)?"** → RoPE generalizes well to longer contexts (ALiBi, YaRN interpolation), but models must be trained with exposure to those lengths to scale efficiently
- **"Why don't we just train on max possible context?"** → Compute cost scales ≥ linearly with context length (O(L^2) for standard attention, O(L) for efficient attention). Diminishing returns often kick in beyond task-relevant context length.

## See also
- [[attention-mechanism]]
- [[transformer-architecture]]
- [[loss-scaling]]
- [[extrapolation]]
- [[information-density]]
- [[compression]]

## Sources
See frontmatter `sources:`.
