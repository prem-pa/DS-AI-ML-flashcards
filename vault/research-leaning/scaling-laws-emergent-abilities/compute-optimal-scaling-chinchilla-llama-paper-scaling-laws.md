---
id: e44ae1a1-f483-4706-b7a0-dc27ccff3cf0
title: Compute optimal scaling (Chinchilla, LLaMA paper scaling laws)
track: research-leaning
topic: scaling-laws-emergent-abilities
difficulty: 5
tags:
- scaling-laws
- compute-efficiency
- training-dynamics
- model-architecture
- hyperparameters
aliases:
- Chinchilla scaling
- compute-optimal training
- Hoffmann ratio
sources:
- url: https://arxiv.org/abs/2203.15556
  label: Hoffmann et al., Training Compute-Optimal Large Language Models (NeurIPS 2022)
- url: https://proceedings.neurips.cc/paper_files/paper/2022/file/c1e2faff6f588870935f114ebe04a3e5-Paper-Conference.pdf
  label: DeepMind Chinchilla Scaling Laws
cards:
- id: 6367ccbc-5234-4417-940d-94b00bf8c090
  type: flip
  front: State the Hoffmann et al. compute-optimal scaling rule (Chinchilla) in one sentence.
  back: 'For a fixed training compute budget, model size (N) and data tokens (D) should scale equally:
    D ≈ 20N, i.e., approximately 20 tokens per parameter.'
- id: 33d0fb47-4ce3-4b8b-a866-ebcbd89676db
  type: mcq
  front: Hoffmann et al. trained Chinchilla (70B params) using the same compute budget as Gopher. Why
    did Chinchilla outperform Gopher despite being 4× smaller?
  back: Chinchilla's key advantage was allocating compute toward data instead of model size. The Hoffmann
    et al. paper shows that scaling data equally with model size yields superior downstream performance,
    contradicting earlier assumptions that larger models with less data were optimal.
  choices:
  - key: a
    text: Chinchilla used a better optimizer (Adam vs SGD).
    correct: false
  - key: b
    text: Chinchilla was trained on 4× more tokens, making it less data-starved and more compute-efficient.
    correct: true
  - key: c
    text: Chinchilla used a fundamentally different architecture with more heads and wider hidden layers.
    correct: false
  - key: d
    text: Chinchilla benefited from instruction-tuning, while Gopher used only base pre-training.
    correct: false
- id: 5bd39adf-5287-4e17-8275-a63d049b9ceb
  type: flip
  front: What was the prevailing assumption about compute allocation *before* Chinchilla, and how did
    Hoffmann et al. challenge it?
  back: Before Chinchilla, researchers (e.g., Kaplan et al.) believed models should be data-efficient
    and parameter-heavy. Hoffmann et al. showed models were severely undertrained—data allocation was
    too low. Optimal scaling requires equal growth in both N (parameters) and D (tokens).
- id: 090ebf9c-c52d-40ab-91d8-6a9d770a1270
  type: mcq
  front: If you have a fixed compute budget C for training, and you decide to double model size from 70B
    to 140B parameters, how should you adjust training tokens (under Chinchilla)?
  back: Chinchilla's core finding is that to maintain compute-optimality, both N and D should scale proportionally.
    Doubling N (700B → 1.4T tokens) keeps the compute budget C = 6ND roughly constant while moving along
    the Pareto frontier of the loss-compute curve.
  choices:
  - key: a
    text: Keep tokens fixed; larger models learn from the same data faster.
    correct: false
  - key: b
    text: Double tokens to approximately 2.8T (roughly matching the 20 tokens-per-parameter rule).
    correct: true
  - key: c
    text: Reduce tokens by 50% to stay within compute budget.
    correct: false
  - key: d
    text: Increase tokens by 4× to match the parameter increase.
    correct: false
---

## Intuition

The Chinchilla scaling law answers a fundamental question: given a fixed compute budget, how should you allocate it between model size and training data? Contrary to earlier assumptions (like Kaplan et al.'s Scaling Laws for Neural Language Models), Hoffmann et al. found that models were vastly undertrained—the optimal ratio is roughly equal scaling: double model size, double data.

The magic number: approximately **20 tokens per parameter** for compute-optimal training.

## Detail

Hoffmann et al. (2022) trained 400+ language models ranging from 70M to 16B parameters on 5B–500B tokens. They discovered:

- For a fixed compute budget C, optimal model size N and token count D should satisfy: **N ≈ D/20**
- Chinchilla (70B params, ~1.4T tokens) matches Gopher's (280B params) compute but vastly outperforms it on downstream tasks
- The relationship is empirically smooth: loss follows power laws L = α/N^β + ε across model sizes
- Implications: Most large models (GPT-3, Jurassic, MT-NLG) were compute-inefficient; data-starved relative to parameter count

Key insight: **Compute budget is not just about FLOPs; training data matters equally.** A 7B-parameter model trained on 140B tokens beats a 280B model trained on 140B tokens at equivalent compute cost.

## Common gotchas / interview framings

- **"But Chinchilla predicted we'd train models on trillions of tokens—does that match practice?"** → LLaMA, Gemini confirmed the 20-token-per-parameter rule empirically, validating Hoffmann's estimates. Real models often train past the Chinchilla point for downstream performance or instruction-tuning gains.
- **"What compute budget are we optimizing?"** → Total FLOPs for training (6ND, where N is parameters, D is tokens). Inference cost, memory, and wall-clock time are separate optimization targets.
- **"Is the 20-token ratio universal?"** → Empirically holds across model families; slight variation with architecture (attention patterns, tokenizer), but the order of magnitude is robust.
- **Critique:** Recent work (2024–2025) questions tight confidence intervals in Hoffmann's data; practical considerations (instruction-tuning, preference data) push models beyond Chinchilla-optimal data quantities.

## See also
- [[scaling-laws]]
- [[transformer-architecture]]
- [[training-loss]]
- [[model-parameters]]
- [[data-efficiency]]
- [[inference-cost]]

## Sources
See frontmatter `sources:`.
