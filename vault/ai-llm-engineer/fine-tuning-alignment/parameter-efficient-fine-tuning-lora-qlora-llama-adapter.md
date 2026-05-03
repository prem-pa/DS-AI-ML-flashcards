---
id: ab88b6ef-8c9f-4215-bdf7-11bdbcf51517
title: Parameter-efficient fine-tuning (LoRA, QLoRA, LLaMA-Adapter)
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 3
tags:
- peft
- memory-efficiency
- lora
- qlora
- adapter-methods
- quantization
aliases:
- LoRA
- QLoRA
- PEFT
- DoRA
- adapters
sources:
- url: https://introl.com/blog/fine-tuning-infrastructure-lora-qlora-peft-scale-guide-2025
  label: 'Introl: LoRA, QLoRA, PEFT at Scale'
- url: https://explore.n1n.ai/blog/fine-tune-llm-lora-qlora-guide-2026-2026-04-17
  label: 'N1N AI: Fine-Tuning Guide 2026'
- url: https://kaitchup.substack.com/p/advanced-lora-fine-tuning-how-to
  label: Advanced LoRA Fine-Tuning Comparison
- url: https://github.com/ZrrSkywalker/LLaMA-Adapter
  label: LLaMA-Adapter GitHub
cards:
- id: 0e88862c-47e1-40b3-a1a8-4fb52484df10
  type: flip
  front: You have a 7B model and 8xA100 for DPO. Would you use full FT, LoRA, or QLoRA? Justify.
  back: Use full FT on this setup. With 8xA100 (640GB VRAM), you have enough memory. Full FT avoids LoRA
    rank bottleneck and inference merge overhead. QLoRA is overkill and adds quantization noise. Full
    FT is fastest to converge and simplest to deploy (no adapter merging needed).
- id: a78836cb-c679-4c45-b92a-4d03d9c6f66d
  type: flip
  front: 'LoRA rank selection: what do r=4, r=16, r=64 trade off?'
  back: 'r=4: minimal VRAM, fast training, but underfits complex domain shifts; recovers ~70–80% quality.
    r=16: sweet spot for most tasks; recovers ~90–95%; standard default. r=64: approaches full FT memory/time,
    risks overfitting on small datasets, diminishing returns on quality. Start r=16, adjust if task complexity
    demands it.'
- id: d0ef068b-79f1-4848-ae4b-5c6901eed883
  type: mcq
  front: Which statement about QLoRA is FALSE?
  back: QLoRA is NOT compatible with FSDP. Quantization and full-parameter sharding conflict. QLoRA is
    designed for single/multi-GPU setups. For large-scale distributed training, use full FT or LoRA without
    quantization.
  choices:
  - key: a
    text: QLoRA uses 4-bit NF4 quantization to reduce base model size
    correct: false
  - key: b
    text: QLoRA is compatible with FSDP for 1000+ GPU distributed training
    correct: true
  - key: c
    text: QLoRA requires paged optimizers to handle memory spikes
    correct: false
  - key: d
    text: QLoRA typically achieves 90%+ of full fine-tune performance
    correct: false
- id: aacd01e2-0904-4e30-a870-c446d94b5f96
  type: flip
  front: You deploy a LoRA model. Why must you merge W + AB^T before inference if latency is critical?
  back: 'At inference, separate A, B matrices require an extra matrix-vector product (AB^T @ x), adding
    ~10% latency and complexity. Merging (W'' = W + AB^T) pre-computation fuses this into a single matrix.
    Trade: inference speed gains vs. serving multiple LoRA variants (no merge → swap adapters cheaply).'
---

## Intuition
LoRA (Low-Rank Adaptation) replaces full weight updates with low-rank deltas. Instead of training all $n \times m$ parameters in a weight matrix, train only $r(n + m)$ parameters (where $r \ll n, m$). Update: $W' = W + AB^T$, where $A \in \mathbb{R}^{n \times r}$, $B \in \mathbb{R}^{m \times r}$. This recovers ~90–95% of full fine-tune quality at 1/10th the memory.

QLoRA pushes further: quantize the base model to 4-bit NF4 precision, keeping only LoRA weights in full precision. A 70B model drops from 140 GB to ~46 GB. LLaMA-Adapter uses zero-initialized attention: learnable adapter tokens injected per-layer, only 1.2M parameters for 7B model.

## Detail
LoRA mathematics: Forward pass is $$h = W_0 x + \Delta W x = W_0 x + AB^T x$$
Gradient updates only flow through $A, B$. Key hyperparameters: $r$ (rank, typically 8–64), $\alpha$ (scaling, usually $\alpha = 2r$), and target modules (typically Q, V in attention, or all linear layers).

QLoRA adds: (1) 4-bit NF4 quantization of frozen base, (2) double quantization of quantization constants, (3) paged optimizers to handle memory spikes. This requires ~0.04x the VRAM of full fine-tune.

LLaMA-Adapter: learnable adapters in attention; can tune in <1 hour on single GPU with 52k examples.

DoRA (2024): decompose LoRA into magnitude and direction; slight improvements over LoRA on some tasks, same rank budget.

## Common gotchas / interview framings
- **Rank selection**: Too low (r=4) → underfitting. Too high (r=64+) → approaching full FT cost, overfitting risk. Start with r=16, grid-search if needed.
- **Target module choice**: Adapter on Q, V only misses critical updates to other layers. Modern best: all linear layers (query, value, up_proj, down_proj, gate).
- **QLoRA + FSDP**: Quantization incompatible with full-parameter distributed training. Use for single/multi-GPU setups, not 1k-GPU clusters.
- **LoRA drift**: Rank too high + loose KL constraint → indistinguishable from full FT. Monitor model divergence from base.
- **Inference overhead**: LoRA adds ~10% latency (AB^T matrix-vector product). Merge if latency-critical: $W' = W + AB^T$.

## See also
- [[low-rank-decomposition]]
- [[quantization]]
- [[gpu-memory]]
- [[fine-tuning-approaches]]
- [[parameter-efficiency]]
- [[multi-task-learning]]
- [[inference-efficiency]]

## Sources
See frontmatter `sources:`.
