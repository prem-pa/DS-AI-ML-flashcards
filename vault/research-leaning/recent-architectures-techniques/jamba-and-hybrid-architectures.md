---
id: e3c4eae7-bd98-4587-8c70-c5732ea709e3
title: Jamba and hybrid architectures
track: research-leaning
topic: recent-architectures-techniques
difficulty: 5
tags:
- hybrid-models
- attention-and-SSM
- composition-design
- MoE-routing
- KV-cache-efficiency
aliases:
- hybrid-Transformer-SSM
- attention-SSM composition
- mixed-layer-types
sources:
- url: https://arxiv.org/abs/2403.19887
  label: 'Jamba: A Hybrid Transformer-Mamba Language Model'
- url: https://www.ai21.com/blog/announcing-jamba/
  label: AI21 Jamba Announcement
- url: https://www.ai21.com/blog/rise-of-hybrid-llms/
  label: The Rise of Hybrid LLMs
cards:
- id: 2aef2c0e-196a-4714-bdcd-fd92a4baaeca
  type: flip
  front: What is the core composition principle of Jamba, and why is the 1-attention-per-8-layers ratio
    chosen?
  back: 'Jamba uses 1 Transformer attention layer per every 8 total layers (1/8 ratio). This ratio balances
    two opposing forces: attention for dense, token-level reasoning (where it excels) and Mamba SSM layers
    for efficient long-range dependencies. The 1/8 ratio emerged from empirical sweeps; more attention
    reduces throughput too much, less attention hurts reasoning quality.'
- id: 4218479c-82c1-4cfb-8aa8-c541402a607e
  type: mcq
  front: How much smaller is Jamba's KV cache compared to a vanilla Transformer with equivalent context
    window?
  back: Since only 1/8 of layers require KV cache (the attention layers), and SSM layers don't cache key-value
    pairs, Jamba achieves 8× reduction in KV cache size. This is a major practical advantage for long-context
    inference with memory-constrained hardware.
  choices:
  - key: a
    text: 2× smaller
    correct: false
  - key: b
    text: 4× smaller
    correct: false
  - key: c
    text: 8× smaller
    correct: true
  - key: d
    text: 16× smaller
    correct: false
- id: b5ce5ec1-37a0-4a9d-bd4f-51e6e68fb50b
  type: flip
  front: In Jamba, what are the active and total parameters, and why use Mixture-of-Experts (MoE)?
  back: 'Jamba has 52B total parameters but only 12B active parameters at inference due to MoE routing.
    MoE selection allows sparse activation: each token is routed to a subset of expert MLPs, reducing
    compute per token. This makes the model efficient despite large total capacity, enabling better performance
    on diverse reasoning tasks.'
- id: 6f02077b-c034-4b0e-9bed-dd83238a592d
  type: mcq
  front: Which architectural property gives Jamba its 3× throughput advantage over Mixtral-8x7B on long
    sequences?
  back: 'The throughput gain comes from the hybrid design: SSM layers are extremely fast (linear complexity),
    and the reduced KV cache (only 1/8 layers need it) cuts memory bandwidth. On long contexts where attention
    KV cache dominates cost, this hybrid approach wins decisively.'
  choices:
  - key: a
    text: Larger model size (52B vs 47B)
    correct: false
  - key: b
    text: SSM layers for most positions + 8× smaller KV cache from reduced attention
    correct: true
  - key: c
    text: Better MoE router training
    correct: false
  - key: d
    text: Hardware-optimized attention kernels
    correct: false
- id: 709bd8ed-1ca0-487b-88a8-89ccd897aedf
  type: flip
  front: What is the claimed context window length of Jamba, and how does it challenge the traditional
    Transformer scaling pattern?
  back: Jamba achieves a 256K context window, which is competitive with long-context Transformers but
    with significantly lower memory cost due to the hybrid design. This challenges the assumption that
    context length requires a tradeoff with efficiency; hybrid models can have both.
---

## Intuition

Jamba answers: "What if we combine Transformer attention and Mamba SSMs in one model?" By mixing layers strategically—1 attention layer per 8 total layers—it retains attention's reasoning capability while gaining SSM's efficiency. The name stands for Joint Attention and Mamba architecture.

## Detail

Jamba architecture:
- Repeating blocks with 1 Transformer attention layer + 7 Mamba SSM layers
- Mixture-of-Experts (MoE) after each layer for selective parameter routing
- 52B total parameters, 12B active at inference
- 256K context window with 8× smaller KV cache than vanilla Transformers
- 3× throughput vs Mixtral-8x7B on long contexts

Design rationale: Attention excels at dense, token-level dependencies (reasoning); SSM excels at long-range, sparse patterns (efficiency). Hybrid composition lets each handle what it's best at.

Jamba 1.5 extends this with improved layer composition and performance scaling.

## Common gotchas / interview framings
- Q: "Why only 1/8 attention layers?" A: Empirical sweep; 1/8 balances reasoning (attention) with efficiency (SSM). More attention = worse throughput; less = worse reasoning
- Q: "Does KV cache advantage disappear with hybrid?" A: No; KV cache only needed for 1/8 layers, so 8× reduction vs pure Transformer
- Q: "Is this just Transformer+Mamba concatenation?" A: No; interleaved layer design allows information fusion mid-pass, not post-hoc fusion
- MoE interaction: Not obvious how to route across hybrid layers; Jamba uses global routing
- Context window tradeoff: 256K is large but uses more SSM memory than 4K windows

## See also
- [[mamba]]
- [[transformer-attention]]
- [[moe]]
- [[kv-cache]]
- [[long-context-models]]
- [[architectural-composition]]

## Sources
See frontmatter `sources:`.
