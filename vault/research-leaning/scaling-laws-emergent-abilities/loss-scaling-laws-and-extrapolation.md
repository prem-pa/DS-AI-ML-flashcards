---
id: ce947a5f-c730-487e-b23a-661e05493278
title: Loss scaling laws and extrapolation
track: research-leaning
topic: scaling-laws-emergent-abilities
difficulty: 5
tags:
- scaling-laws
- loss-prediction
- extrapolation
- power-laws
- empirical-methods
aliases:
- scaling law prediction
- loss curves
- power-law fitting
sources:
- url: https://arxiv.org/abs/2203.15556
  label: Hoffmann et al., Loss scaling in compute-optimal models (NeurIPS 2022)
- url: https://arxiv.org/abs/1907.04165
  label: Kaplan et al., Scaling Laws for Neural Language Models (OpenAI 2020)
cards:
- id: d03fc65e-973e-40ce-926c-a3b17156f39e
  type: flip
  front: Write a power-law formula for how training loss L scales with model size N.
  back: L(N) = α N^{-β} + ε, where β ≈ 0.07–0.09 and ε is floor loss. For every 10× increase in N, loss
    decreases by roughly 8–9%.
- id: 7bcb563a-c114-4cef-bcd4-acbfb786ff62
  type: mcq
  front: Why are loss scaling laws useful for planning very large model training runs?
  back: Loss scaling laws are empirically smooth and predictable. Researchers can fit small models (10M–1B
    params), identify the power-law exponent β, and predict loss for 100B+ parameter models—critical for
    compute budgeting before spending millions on GPUs.
  choices:
  - key: a
    text: They allow researchers to fit power laws on small models and extrapolate loss for vastly larger
      models before training them.
    correct: true
  - key: b
    text: They guarantee that all models will reach the same final loss regardless of size.
    correct: false
  - key: c
    text: They prove that larger models always generalize better on downstream tasks.
    correct: false
  - key: d
    text: They eliminate the need to tune hyperparameters like learning rate.
    correct: false
- id: 01a305f9-3742-4eff-a3b8-577f4e31e5ea
  type: flip
  front: Approximately what is the power-law exponent β for model size N? (i.e., L ∝ N^{-β})
  back: β ≈ 0.07–0.09. This means loss decreases by roughly 8–9% for every 10× increase in model parameters.
- id: 72e37c7e-7b81-4d21-b9ab-22c04fecfe97
  type: mcq
  front: If you fit a power law L = α N^{-0.08} + ε on small models and extrapolate to predict loss for
    a 1T-parameter model, what is the main risk?
  back: Power laws work well for 1–3 orders of magnitude extrapolation, but large jumps (1B→1T) accumulate
    error. Real models may use different architectures, tokenizers, or domain shifts, breaking the fitted
    relationship.
  choices:
  - key: a
    text: The model will train slower than predicted.
    correct: false
  - key: b
    text: Accumulated fitting error and violated assumptions (new architecture, tokenizer changes) can
      make the prediction inaccurate.
    correct: true
  - key: c
    text: The floor loss ε will become negative.
    correct: false
  - key: d
    text: Inference latency increases, invalidating loss predictions.
    correct: false
---

## Intuition

Loss scaling laws describe smooth, predictable relationships between model size (N), data quantity (D), and language modeling loss (L). Empirically, these relationships follow power laws: L ∝ N^{-α} or L ∝ D^{-β}, allowing you to predict loss at scales you haven't trained yet—critical for planning billion-dollar GPU clusters.

## Detail

Training loss typically obeys a power-law relationship:

**L(N) = α N^{-β} + ε** (model-size dependence)
**L(D) = γ D^{-δ} + ε** (data dependence)

Where:
- β ≈ 0.08 (loss improves by ~8% for every 10× increase in parameters)
- δ ≈ 0.09 (loss improves by ~9% for every 10× increase in data)
- ε = floor loss (irreducible loss, often near zero for large C)

Key properties:
- **Smoothness**: No phase transitions or sudden jumps in loss; continuous improvement
- **Universality**: Exponents hold across model families and datasets (within same domain)
- **Joint scaling**: L(N, D) combines both terms; compute budget C = 6ND constrains the frontier
- **Extrapolation**: Fit small models (1M–1B params), predict loss for 100B–1T param models with reasonable accuracy

## Common gotchas / interview framings

- **"If loss scales smoothly, how do emergent abilities happen suddenly?"** → Loss and task accuracy are different metrics. Smooth loss improvement can produce discontinuous task performance if evaluation uses nonlinear metrics (e.g., threshold-based accuracy). This is central to the "emergence mirage" debate.
- **"Can you extrapolate loss indefinitely?"** → Power laws work well within ~2–3 orders of magnitude. Very large extrapolations (1B→1T params) accumulate error; breaking assumptions (new architectures, tokenizers) invalidate the fit.
- **"Why is β ≈ δ (both ~0.08–0.09)?"** → This is why Chinchilla says N and D scale equally; Compute budget C = 6ND is optimal when ∂L/∂N ≈ ∂L/∂D, which occurs when exponents match.
- **Practical challenge:** Floor loss ε is hard to estimate accurately, leading to overoptimistic extrapolations for very large models.

## See also
- [[training-loss]]
- [[loss-landscape]]
- [[generalization-gap]]
- [[downstream-performance]]
- [[model-capacity]]
- [[data-regime]]

## Sources
See frontmatter `sources:`.
