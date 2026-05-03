---
id: dc19c34e-9ec9-470a-94e6-1f1fedef4220
title: Scaling laws for loss and compute
track: ai-llm-engineer
topic: large-language-model-architectures
difficulty: 5
tags:
- scaling-laws
- loss-prediction
- compute-optimal
- power-law
- chinchilla
- empirical
aliases:
- scaling-laws
- loss-scaling
- compute-scaling
- power-law-scaling
sources:
- url: https://arxiv.org/abs/2001.08361
  label: Scaling Laws for Neural Language Models (Kaplan et al., OpenAI 2020)
- url: https://arxiv.org/abs/2203.15556
  label: Training Compute-Optimal Large Language Models (Hoffmann et al., DeepMind 2022, Chinchilla)
- url: https://arxiv.org/abs/2302.13971
  label: 'LLaMA: Open and Efficient Foundation Language Models (Touvron et al., 2023)'
cards:
- id: 3ae4f187-3c57-4f77-a709-c116027acfe5
  type: flip
  front: State the empirical power-law scaling relationship for LLM loss as a function of parameters N.
  back: $L(N) \approx E + A/N^\alpha$ where $\alpha \approx 0.076$ (Kaplan et al.). This means loss scales
    as inverse power of model size. To halve loss, multiply N by $2^{1/0.076} \approx 100×$. Power law
    is remarkably consistent across architectures and regimes, enabling loss prediction before training.
- id: 8a1d1c25-a29d-46e5-8986-0918fd7455d4
  type: flip
  front: What is Chinchilla scaling, and what is the key insight about allocating compute between model
    size and data?
  back: 'Chinchilla (Hoffmann et al., 2022) optimizes compute allocation. Given total FLOPs budget $C$:
    train with $N \approx D$ (model params ≈ training tokens), not the classical $N \ll D$ (overparameterize
    with few tokens). This balances model expressiveness and data diversity, minimizing loss for fixed
    compute. Implication: data quality/quantity as critical as model size.'
- id: 0adf6a31-3595-4957-ae7d-9d0d0a1aa0eb
  type: mcq
  front: According to Chinchilla scaling, if you have compute budget C and want to train a model, how
    should you allocate it?
  back: Chinchilla's optimization shows equal allocation (N ≈ D in tokens) minimizes loss for fixed compute.
    Pre-Chinchilla (Kaplan et al.) suggested N ≪ D (overparameterize, underfit). Frontier models have
    adopted Chinchilla-style scaling, leading to better efficiency and loss. D ≈ 10×N (option d) is outdated.
  choices:
  - key: a
    text: Maximize model size N; minimize data D (few epochs)
    correct: false
  - key: b
    text: Minimize model size N; maximize data D (many epochs over few tokens)
    correct: false
  - key: c
    text: 'Allocate compute equally: N (params) ≈ D (tokens), roughly equal FLOPs to model and data'
    correct: true
  - key: d
    text: Use 10× more data than model size (D ≈ 10×N)
    correct: false
- id: 3f91b075-36d4-4d56-946e-b5df54a8f4f6
  type: flip
  front: A model trained with Chinchilla scaling has higher inference cost per token than an overparameterized
    model at the same training compute budget. Explain why this is acceptable.
  back: 'Chinchilla scales model size and data equally (N ≈ D in tokens). More N → higher inference cost.
    But: (1) you reach target loss faster in training (fewer wasted FLOPs), (2) generalization is better
    (less overfitting), (3) inference-time compute is separate from training-time cost [[inference-scaling-and-test-time-compute-o1-reasoning-models]].
    The training efficiency gain outweighs inference overhead.'
- id: 58c09373-4abc-4347-8ad8-bb99158e19ea
  type: flip
  front: How do sparse models (Mixture of Experts) interact with scaling laws? Does Chinchilla still apply?
  back: 'Chinchilla law holds for *active* compute, not total parameters. MoE models (e.g., DeepSeek-V3:
    671B params, 37B active) scale better than dense Chinchilla would predict because sparsity reduces
    effective compute per token. You can train larger models (more total params) with same FLOPs if sparse.
    This is why MoE dominates 2025 frontier models [[mixture-of-experts-moe]].'
---

## Intuition

Scaling laws describe how model loss improves as you increase compute (training FLOPs), model size (parameters), or data size (tokens). Empirically, loss follows power laws: $L(N) \approx a/N^\alpha$ where $\alpha \in (0.07, 0.15)$ depending on what you scale. This predictability has driven LLM progress: you can forecast how much compute you need for target loss before training. Key insight: optimal training allocates compute equally to model and data (Chinchilla).

## Detail

**Core empirical law (Kaplan et al., 2020):**
$$L(N, D) = E + \frac{A}{N^\alpha} + \frac{B}{D^\beta}$$

Where:
- $L$: loss (validation perplexity or cross-entropy)
- $N$: model parameters
- $D$: training tokens (data)
- $A, B, E$: constants
- $\alpha \approx 0.076$, $\beta \approx 0.103$ (empirically measured)

**Implications:**
1. **Power-law scaling:** Loss decreases as $\sim 1/N^{0.076}$. To halve loss, increase $N$ by $2^{1/0.076} \approx 100×$.
2. **Data is as important as model:** $\beta \approx \alpha$, so scaling data by 10× ≈ scaling model 10×.
3. **Compute constraint:** Total FLOPs $= C \approx 6 \times N \times D$ (standard transformer). Allocate C between N and D.

**Chinchilla (Compute-Optimal) scaling (Hoffmann et al., 2022):**

Given fixed compute budget $C$, what split between $N$ (params) and $D$ (tokens) minimizes loss?

$$C = 6 \times N \times D$$

Differentiate loss w.r.t. $N$ and $D$ subject to constraint → **optimal allocation: equal FLOPs to model and data**

$$N^\text{opt} = \frac{C}{6D^\text{opt}}, \quad D^\text{opt} = \frac{C}{6N^\text{opt}}$$

Result: $N \approx D$ (number of params ≈ number of training tokens)

**Concrete example:**
- Compute budget: $10^{21}$ FLOPs
- Model size: $N \approx 7B$ params
- Data size: $D \approx 1.4T$ tokens (each token ~1.4 training steps)
- Classical rule (pre-Chinchilla): $N \approx 70B$ for this compute → undertrained (not enough data)
- Chinchilla: reduce N, increase D → same compute, better loss

**Frontier model scaling (2025):**
- **GPT-5** (2025): $\sim 10^{24}$ FLOPs, $\sim 40B$ params (reported), $\sim 4-5T$ tokens
- **Claude 4.7** (2026): $\sim 10^{24}$ FLOPs
- **DeepSeek-V3** (2024): $671B$ params but sparse MoE ($37B$ active); $14.8T$ tokens; $2.79M$ GPU hours

**Architectural considerations:**
- Chinchilla law holds across architectures (Transformers, Llama, GPT-style)
- Sparse models (MoE): effective params lower than total params; scale law still holds on *active* compute
- Training efficiency: Chinchilla-scaled models train to target loss faster than overparameterized ones

## Common gotchas / interview framings

- **"Bigger model is always better."** → No; overscaling model while underfitting on data wastes compute. Chinchilla forces discipline: scale evenly.
- **Inference cost vs. training cost:** Scaling laws predict training loss. Inference cost (per-token) scales with model size only, not data. A Chinchilla-scaled model may have higher inference cost (more params) for same training FLOPs.
- **Double descent / peaking:** Loss follows power law in normal regime; no mysterious second-descent observed in standard supervised learning (common misconception).
- **Test-time compute changes the game:** [[inference-scaling-and-test-time-compute-o1-reasoning-models]] shows o1/o3 achieve better loss with more reasoning steps (violates training-time scaling law). Separate frontier.
- **MoE and scaling laws:** Sparse models have different effective compute/param ratios. DeepSeek-V3 scales better than Chinchilla would predict (sparsity gives edge).

## See also
- [[optimal-batch-sizes-and-datacompute-tradeoff]]
- [[inference-scaling-and-test-time-compute-o1-reasoning-models]]
- [[gpt-llama-mistral-qwen-architectures]]

## Sources
See frontmatter `sources:`.
