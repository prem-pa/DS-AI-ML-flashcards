---
id: b02606b4-79bb-4232-b2cd-3a7b5fb4d65c
title: Optimal batch sizes and data/compute tradeoff
track: ai-llm-engineer
topic: large-language-model-architectures
difficulty: 5
tags:
- batch-size
- training-dynamics
- gradient-noise
- throughput
- training-efficiency
- chinchilla
aliases:
- batch-size-optimization
- data-compute-tradeoff
- critical-batch-size
sources:
- url: https://arxiv.org/abs/1711.00489
  label: Don't Decay the Learning Rate, Increase the Batch Size (Smith et al., 2017)
- url: https://arxiv.org/abs/2203.15556
  label: Training Compute-Optimal Large Language Models (Hoffmann et al., DeepMind 2022)
- url: https://arxiv.org/abs/2001.08361
  label: Scaling Laws for Neural Language Models (Kaplan et al., OpenAI 2020)
cards:
- id: c27326f7-1ca2-4773-bd5e-5a331a559ee6
  type: flip
  front: What is the critical batch size in LLM training, and why does loss plateau beyond it?
  back: Critical batch size ($B_c$) is the threshold beyond which larger batches don't reduce loss further.
    Gradient noise scales as $1/B$; at $B_c$, noise is low enough that loss is curvature-limited (determined
    by loss landscape, not gradient variance). Increasing batch further trains faster (fewer steps to
    same loss) but doesn't improve final loss.
- id: cc700377-e955-4284-b361-d144ff710562
  type: flip
  front: How does batch size relate to learning rate in stochastic gradient descent, and why?
  back: 'Learning rate should scale with batch size: $\eta \propto \sqrt{B}$. Intuition: larger batch
    → lower gradient noise → can take bigger steps. If you double batch, increase LR by $\sqrt{2}$. This
    maintains the noise-to-signal ratio, ensuring stable convergence across different batch sizes.'
- id: 92d96850-f05a-4003-9e78-d6df31907dda
  type: mcq
  front: In Chinchilla-scaled training, what is the relationship between batch size, model size N, and
    training data D?
  back: 'Chinchilla allocates compute equally to model and data (N ≈ D). Critical batch size is $B_c \sim
    D/N$. For optimal efficiency, use $B \approx B_c$: below it, gradient noise limits loss; above it,
    you''re wasting compute. Larger models need larger batches (more gradient signal).'
  choices:
  - key: a
    text: Batch size should equal model size N
    correct: false
  - key: b
    text: Batch size should equal training data tokens D
    correct: false
  - key: c
    text: Batch size should be chosen near critical batch size $B_c \approx (const \times D)/N$ to balance
      noise and throughput
    correct: true
  - key: d
    text: Batch size is independent of N and D; fixed at 4096
    correct: false
- id: 2fd453a7-229d-40f0-92c4-e6f77a14192d
  type: flip
  front: Explain the difference between small-batch and large-batch training regimes in terms of loss
    convergence and limiting factors.
  back: 'Small batch (<$B_c$): loss converges as $1/\sqrt{\text{steps}} \propto 1/\sqrt{B}$; noise-limited.
    More gradient samples → faster convergence. Large batch (>$B_c$): loss plateaus; curvature-limited.
    Increasing batch only speeds up training (fewer steps), not final loss. Trade: small batch is data-efficient
    but slow; large batch is fast but hits ceiling.'
- id: 82a6bfb7-2868-41c3-ad20-8a1946eddef9
  type: flip
  front: Modern LLM training uses 'gradient accumulation.' How does this relate to batch size constraints
    and memory?
  back: 'Gradient accumulation: run forward+backward for $k$ smaller batches, accumulate gradients, then
    update parameters. Effective batch size = $k \times$ per-device batch, but memory footprint ≈ per-device
    batch (reuse GPU memory). This lets you simulate large batches on hardware with limited VRAM, crucial
    for training massive models [[scaling-laws-for-loss-and-compute]].'
---

## Intuition

Batch size determines how many gradients are averaged before a parameter update. Large batches (thousands) reduce gradient noise but require more GPU memory; small batches (tens) are noisier but memory-light. There's a "critical batch size" beyond which increasing batch size doesn't improve loss (only throughput). Understanding this tradeoff lets you maximize training efficiency: process more data per unit compute, or hit target loss faster.

## Detail

**Gradient noise and batch size:**
Stochastic gradient descent updates on minibatches:
$$\theta_{t+1} = \theta_t - \eta \nabla L(\theta_t; B_t)$$
where $B_t$ is a batch of size $B$.

Gradient noise (variance) scales with batch size:
$$\text{Var}[\nabla L] \propto 1/B$$

Small batch → high noise → slow convergence (more steps to reach same loss). Large batch → low noise → faster convergence (fewer steps). But:

**Critical batch size ($B_c$):**
Beyond $B_c$, increasing batch size doesn't reduce loss further (only speeds up training). This is because the loss landscape has irreducible noise at the boundary. Empirically:
$$B_c \approx \frac{\text{const} \times D}{N}$$
where $D$ = training data (tokens), $N$ = model params. Larger models have larger critical batch size (more gradient signal needed).

**Training dynamics (Smith et al.):**
- Small batch regime: loss decreases with $1/\sqrt{\text{steps}} \propto 1/\sqrt{B}$ (noise-limited)
- Large batch regime: loss decreases but plateaus (curvature-limited); diminishing returns
- Practical: batch sizes 512-4096 for LLMs cover most of the benefit

**Compute efficiency formula (from Chinchilla scaling):**
Total compute $C = 6 \times N \times D$ (assumes no loss from increased batch size). If batch size $B > B_c$, you're wasting compute (not learning more, just training faster). Optimal: $B \approx B_c$ or slightly above.

**Practical LLM training (2025):**
- Global batch size: 1M - 8M tokens per step (accumulate across devices)
- Gradient accumulation: if per-device batch is 32, and you accumulate 128 steps, effective batch is 32×128
- Chinchilla rule: allocate FLOPs equally between N and D; batch size adjusts accordingly

**Memory vs. throughput:**
- Large batch: high GPU utilization, high throughput (tokens/sec), but high memory
- Small batch: low memory, lower throughput
- Trade: increase batch within hardware limits; recompute activations (gradient checkpointing) to reduce memory at small cost in compute

## Common gotchas / interview framings

- **"Always use maximum batch size."** → Beyond critical batch size, loss doesn't improve. You just train faster (fewer iterations) but same compute-to-loss relationship. Trade throughput/memory vs. data efficiency.
- **Batch size and learning rate:** Learning rate often scales with batch size ($\eta \propto \sqrt{B}$). Critical detail: if you double batch, increase LR by $\sqrt{2}$, not 2×.
- **Gradient noise vs. curvature:** At small batch, noise dominates loss (stochastic descent helps escape sharp minima). At large batch, curvature dominates (limited by loss landscape). Different optimization regimes.
- **Distributed training:** Batch size is *global* (sum across all devices/nodes). Local per-device batch might be 32, but global is 32×1000 GPUs = 32K. Gradient accumulation and sync matter.
- **Critical batch size empirical rule:** $B_c \approx (\text{const} \times D) / N$. For 7B model on 1T tokens, $B_c \approx 10K$ (rough). Frontier 70B on 1T might have $B_c \approx 100K$.

## See also
- [[scaling-laws-for-loss-and-compute]]
- [[gpt-llama-mistral-qwen-architectures]]

## Sources
See frontmatter `sources:`.
