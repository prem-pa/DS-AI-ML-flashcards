---
id: ba7d3f1f-edac-4de7-ad17-541ad1904ea4
title: GPT, Llama, Mistral, Qwen architectures
track: ai-llm-engineer
topic: large-language-model-architectures
difficulty: 3
tags:
- decoder-only
- architecture-variations
- normalization
- activation-functions
- attention-variants
- model-comparison
aliases:
- GPT-family
- Llama-family
- Mistral
- Qwen
- LLM-variants
sources:
- url: https://arxiv.org/abs/2005.14165
  label: Language Models are Unsupervised Multitask Learners (Radford et al. 2019, GPT-2)
- url: https://arxiv.org/abs/2302.13971
  label: 'LLaMA: Open and Efficient Foundation Language Models (Touvron et al. 2023)'
- url: https://arxiv.org/abs/2310.06825
  label: Mistral 7B (Jiang et al. 2023)
- url: https://arxiv.org/abs/2309.16609
  label: Qwen Technical Report (2023)
- url: https://www.cosmicjs.com/blog/best-ai-for-developers-claude-vs-gpt-vs-gemini-technical-comparison-2026
  label: 'Claude vs GPT-5.2 vs Gemini 3: Technical Comparison 2026'
cards:
- id: 4cb508de-3421-44de-9711-e35ab85283ee
  type: flip
  front: Name three architectural differences that distinguish Llama/Mistral/Qwen from GPT-2 era models.
  back: '1. Pre-normalization (compute → normalize) vs post-normalization. Pre-LN is more training-stable.
    2. Activation function: SwiGLU (gated, better) vs. GELU. 3. Positional embeddings: RoPE (rotary) generalizes
    to longer sequences vs. learned absolute positions. All three improve training stability, generalization,
    and inference efficiency.'
- id: 1e5ed510-c355-4af9-98d0-c3ea0b27a996
  type: flip
  front: What is grouped-query attention (GQA) and why is it critical for inference serving in models
    like Mistral?
  back: 'GQA: multiple query heads but fewer key/value heads. Example: 32 query heads, 8 KV heads. Reduces
    KV-cache size by 4× (cache is linear in num_heads × d_model) during generation, cutting memory and
    latency with <1% capability loss. Essential for serving large models on limited VRAM [[inference-scaling-and-test-time-compute-o1-reasoning-models]].'
- id: 2ea525a2-6f03-4ef8-97de-8e5e6175e4cd
  type: mcq
  front: 'Pre-normalization differs from post-normalization in that:'
  back: Pre-norm (compute norm(x), then attention) stabilizes gradients and enables training without warmup.
    Post-norm (attention(x), then norm) is simpler but risks gradient vanishing in deep models. Post-LN
    became mainstream after pre-LN proved superior; modern models use pre-LN. Answer (b) is correct; (d)
    is outdated folklore.
  choices:
  - key: a
    text: Pre-norm applies layer norm before attention/FFN; post-norm applies it after
    correct: false
  - key: b
    text: Pre-norm is more training-stable and allows deeper models without warmup
    correct: true
  - key: c
    text: Pre-norm is used only in decoder-only; post-norm only in encoder-decoder
    correct: false
  - key: d
    text: Post-norm has better gradient flow; pre-norm has better stability—both used in 2025
    correct: false
- id: 84d9ef8e-448b-4eb5-bf34-547e97d9a73f
  type: flip
  front: Mistral's sliding-window attention operates over a limited context window per layer. How does
    this improve efficiency?
  back: Each attention layer only attends to the last W tokens (e.g., 4K), not the full sequence. This
    reduces per-layer attention from O(n²) to O(n·W), where W << n. Over deep stacks, lower layers capture
    local patterns, higher layers compose them. Results in 4-7× faster inference than full attention on
    long sequences with minimal accuracy loss.
- id: 71c487e3-770e-4399-9311-6f851ed70857
  type: flip
  front: RoPE (Rotary Position Embeddings) has a key advantage over learned absolute positions. What is
    it?
  back: RoPE encodes position as rotation in the complex plane. The rotation is relative (invariant to
    absolute position), so models trained on position $i$ generalize to inference at position $i'$ without
    retraining. Learned absolute positions don't generalize; RoPE extends naturally to sequences longer
    than training length (key for long-context).
---

## Intuition

All modern LLMs are decoder-only transformers, but they differ in architectural details: layer normalization placement (pre vs. post), activation function choice (ReLU, GELU, SwiGLU), attention variants (multi-query, grouped-query), and expert routing (sparse MoE). These choices compound in impact at scale, affecting training stability, inference speed, and final performance.

## Detail

**GPT family (OpenAI):**
- GPT-2 (2019): Pre-LN, GELU activation, standard multi-head attention
- GPT-3 (2020): Scaled up, similar architecture
- GPT-4 (2023): Rumored architecture details withheld; frontier performance
- GPT-5 (2025): Released August 2025; GPT-5.5 (April 2026) with agentic capabilities
- Signature features: Exceptional in-context learning, instruction-following

**LLaMA (Meta):**
- Pre-normalization, SwiGLU activation, RoPE positional embeddings
- Explicit focus on efficiency: strong performance per parameter
- LLaMA-2 (July 2023): Openly released, enabled massive community models
- Architecture: Simple, elegant; proven scaling efficiency [[optimal-batch-sizes-and-datacompute-tradeoff]]

**Mistral (Mistral AI):**
- Grouped-query attention (GQA) for faster inference
- Sliding-window attention (limited context per layer, efficient for long sequences)
- Mistral 7B: Outperforms Llama-2 13B on many tasks
- Mistral Large: MoE variant, Mixture of Experts [[mixture-of-experts-moe]]

**Qwen (Alibaba):**
- Pre-normalization with layer-wise scaling
- Rotary positional embeddings (RoPE)
- Strong multilingual capabilities (default 80+ languages)
- Qwen-2 (2024): Competitive with Mistral, better for non-English tasks

**Key architectural innovations across models:**
- **Layer normalization:** Pre-LN (normalize before computation) vs. post-LN (normalize output). Pre-LN is more stable and is standard post-2022.
- **Activation:** GELU (smooth), ReLU (fast, unstable), SwiGLU (gated, best stability/performance)
- **Attention variants:** MHA (standard) → GQA (grouped-query, fewer KV heads) → MQA (single KV head, ultra-fast)
- **Positional embeddings:** Learned → RoPE (rotary, scales to longer sequences better)

**2025-2026 landscape:** All frontier models (GPT-5.5, Claude 4.7, Gemini 3.1) are decoder-only but use proprietary architecture refinements. Open-source leaders (DeepSeek-R1, Qwen) compete on efficiency and reasoning.

## Common gotchas / interview framings

- "Which architecture is 'best'?" → Depends on constraints (latency, throughput, long-context). GQA/MQA great for inference; SwiGLU great for training stability.
- **Grouped-query attention (GQA):** Multiple query heads but fewer key/value heads. Reduces KV-cache size (linear in hidden dim × num heads) without major capacity loss.
- **Scaling laws hold across architectures:** Core properties (loss ~ 1/N) stable; architecture choice is 10-20% efficiency delta.
- **RoPE vs. learned embeddings:** RoPE generalizes better to longer sequences than training saw; crucial for long-context models.

## See also
- [[causal-masking-and-autoregressive-generation]]
- [[decoder-only-vs-encoder-decoder-tradeoff]]
- [[scaling-laws-for-loss-and-compute]]
- [[frontier-model-performance-gpt-4-claude-37-gemini-3]]

## Sources
See frontmatter `sources:`.
