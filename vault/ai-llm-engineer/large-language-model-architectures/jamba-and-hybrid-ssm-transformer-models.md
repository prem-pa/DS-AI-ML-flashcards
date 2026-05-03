---
id: 8381b4b6-9ced-4ef5-8654-a766714e2f72
title: Jamba and hybrid SSM-Transformer models
track: ai-llm-engineer
topic: large-language-model-architectures
difficulty: 5
tags:
- hybrid-architecture
- state-space-models
- mamba
- ssm-transformer
- long-context
- efficiency
aliases:
- Jamba
- SSM-hybrid
- Mamba-transformer
- state-space-models
sources:
- url: https://arxiv.org/abs/2403.19887
  label: 'Jamba: A Hybrid Transformer-Mamba Language Model (AI21, 2024)'
- url: https://www.ai21.com/blog/announcing-jamba/
  label: 'Introducing Jamba: AI21''s Groundbreaking SSM-Transformer Model'
- url: https://github.com/state-spaces/mamba
  label: 'Mamba: State-Space Models for Efficient Sequence Modeling (GitHub)'
- url: https://gregrobison.medium.com/architectural-evolution-in-large-language-models-a-deep-dive-into-jambas-hybrid-transformer-mamba-c3efa8ca8cae
  label: 'Architectural Evolution in Large Language Models: Jamba''s Hybrid Design (Medium)'
cards:
- id: c7fd90ea-82fd-4e0e-8e40-6351261af6ec
  type: flip
  front: What is the core motivation for hybrid Transformer-Mamba architectures like Jamba?
  back: 'Transformers: $O(n^2)$ attention, strong reasoning (in-context learning), slow at long sequences.
    Mambas: $O(n)$ per-token cost, fast, but weaker reasoning (no explicit query-key matching). Jamba:
    interleave both—Mamba layers for efficiency, attention layers for reasoning. Net result: sub-quadratic
    cost with strong performance.'
- id: eadfa8d7-6470-4526-beaa-3dc009400a1f
  type: flip
  front: In Jamba's architecture, what is the layer ratio (attention vs. Mamba), and why was this ratio
    chosen?
  back: Jamba uses 1 attention layer per 8 total layers (12.5% attention, 87.5% Mamba). Empirically optimized;
    this ratio balances reasoning (attention) with efficiency (Mamba). At 256K context, the Mamba-heavy
    design dominates cost; for shorter sequences, attention layers are occasional but impactful for complex
    reasoning.
- id: 20351530-ebdf-4896-ad95-a6687a00ffa5
  type: mcq
  front: Jamba empirically found that Mamba-1 + attention outperformed Mamba-2 + attention in the hybrid
    architecture. What does this suggest?
  back: Mamba-2 (selective transitions) aims to improve implicit reasoning in pure SSMs. In a hybrid,
    explicit attention layers handle complex reasoning; the SSM's job is efficient sequence processing.
    Mamba-1's simplicity and compatibility with attention is sufficient. Mamba-2 shines in pure-SSM settings,
    not in hybrids.
  choices:
  - key: a
    text: Mamba-2 is a regression and should not be used
    correct: false
  - key: b
    text: Mamba-2's selectivity improvements are redundant with explicit attention; Mamba-1's simplicity
      suffices in hybrid
    correct: true
  - key: c
    text: Hybrid models should avoid SSMs and use only attention
    correct: false
  - key: d
    text: Mamba-1 is fundamentally superior to Mamba-2
    correct: false
- id: 50500055-3c69-4362-8ab5-124c06faf606
  type: flip
  front: Estimate the asymptotic complexity of Jamba's inference as a function of sequence length n, given
    1 attention + 7 Mamba layers per 8-layer block.
  back: 'Per-token cost: $O(1/8 \cdot n) + O(7/8 \cdot 1) = O(n/8 + 7/8)$ (attention is $O(n)$ per new
    token at inference, Mamba is $O(1)$). For full sequence computation (training): $O(n^2/8 + 7n/8) \approx
    O(n^2/8)$. At n=256K, Mamba dominates; at n=4K, attention contributes ~30%.'
- id: 769fcf41-e69d-4da2-a9ef-57d2c103f5dd
  type: flip
  front: What practical advantage does Jamba's 256K context window provide compared to typical Transformer
    models (4K-128K)?
  back: '256K context (256,000 tokens) enables: (1) full documents (papers, books, code repos) in one
    prompt, (2) long multi-turn conversations with full history, (3) retrieval-augmented generation with
    large retrieved sets. Practical impact: reduce context switching, improve consistency over long reasoning,
    enable new use cases.'
---

## Intuition

Jamba and similar hybrid models interleave traditional Transformer attention layers with State-Space Model (SSM/Mamba) layers. Transformers excel at in-context learning (dense attention over all tokens) but scale quadratically; SSMs are linear in sequence length but historically weaker on tasks requiring global reasoning. Hybrids combine strengths: attention for complex reasoning, SSMs for efficient long-context. Jamba-1.5 (2025) scales to 398B params with 94B active, supporting 256K context.

## Detail

**State-Space Models (SSM/Mamba) recap:**
- Model sequence as state-space system: $h_t = Ah_{t-1} + Bx_t$, $y_t = Ch_t$
- Linear recurrence: $O(n)$ in sequence length (not $O(n^2)$ like attention)
- Mamba (2023): selective SSM, selective matrix $A_t$ per token → competitive with attention on language modeling
- Limitation: attention weight per input is implicit in hidden state; no explicit "which token did I look at?" like attention

**Hybrid approach (Jamba):**
- Interleave attention and Mamba layers in a block structure
- Example: layer sequence: Attention → FFN → Mamba → FFN → Attention → FFN → ...
- Jamba's ratio: **1 attention layer per 8 total layers** (7 Mamba layers per attention layer)
- Total: 72 layers (64 Mamba + 8 attention)
- MoE on both: Mamba layers have sparse experts, attention layers have sparse experts

**Architecture specifics:**
- **Jamba-1.5** (2025): 398B total params, 94B active (with MoE)
- **Context window:** 256K tokens (vs. 4K-128K for typical Transformers)
- **Throughput:** Higher than pure Transformer (Mamba layers are faster)
- **Memory:** Sub-quadratic vs. Transformer's quadratic (key advantage for long-context)

**Why this ratio?**
- Empirically, Jamba found **1/8 ratio** (12.5% attention) optimal
- Surprising finding: Mamba-1 (not Mamba-2) + attention outperforms Mamba-2 + attention
- Suggests: Mamba-2's improvements (selective state transitions) redundant with attention; Mamba-1's simplicity works in hybrid

**Performance vs. pure architectures:**
- **Pure Transformer (e.g., Llama):** $O(n^2)$ attention, excellent in-context, but slow at long sequences
- **Pure Mamba:** $O(n)$ per-token cost, fast, but weaker on reasoning (lower in-context performance)
- **Jamba:** Hybrid cost: $O(n^2 \cdot \frac{1}{8} + n \cdot \frac{7}{8}) \approx O(n^2/8 + 7n/8)$. For n=256K, Mamba cost dominates; for n<4K, attention cost is noticeable but acceptable.

## Common gotchas / interview framings

- **"SSMs are 'linear' so they're faster."** → Linear in theory, but Mamba's selectivity adds overhead. In practice, Mamba is 2-3× faster than attention on long sequences, not 256× faster (because of constant factors and implementation).
- **Inference scaling:** At 256K context, Jamba's inference is ~100× faster than pure Transformer (quadratic attention). This enables new applications (full-document reasoning, retrieval with context).
- **Mamba-2 paradox:** Mamba-2 is better in isolation (higher quality SSM), but in Jamba, Mamba-1 works better. Suggests: with explicit attention, SSM doesn't need to do everything.
- **Training:** Hybrid still requires different initialization, gradient scaling for Mamba vs. attention layers; more complex than pure Transformer [[scaling-laws-for-loss-and-compute]].
- **When to use:** Jamba excels for long-context tasks (full documents, multi-turn memory, retrieval-augmented generation). For short-context (<4K), pure Transformer likely better per-token.

## See also
- [[gpt-llama-mistral-qwen-architectures]]
- [[mixture-of-experts-moe]]
- [[scaling-laws-for-loss-and-compute]]

## Sources
See frontmatter `sources:`.
