---
id: 0e92b53d-cff2-4ee0-8381-b17ca801b8bb
title: Grokking phenomenon
track: research-leaning
topic: scaling-laws-emergent-abilities
difficulty: 5
tags:
- grokking
- generalization-dynamics
- phase-transitions
- feature-learning
- optimization
aliases:
- delayed generalization
- sudden generalization
- late-stage learning
sources:
- url: https://arxiv.org/abs/2509.21519
  label: Provable Scaling Laws of Feature Emergence from Learning Dynamics of Grokking (2025)
- url: https://arxiv.org/abs/2201.02177
  label: 'Grokking: Generalization Beyond Overfitting on Small Algorithmic Datasets (Power et al.)'
cards:
- id: eb5350ee-38a4-4969-9c73-56b95ab90f52
  type: flip
  front: Define grokking and distinguish it from standard overfitting → generalization.
  back: Grokking is sudden generalization after long periods of memorization. A model trains with high
    training loss and poor test accuracy for 1000s of steps, then test accuracy jumps sharply (often to
    near-perfect) in a few steps. It reflects the discovery of an underlying algorithm, not gradual refinement.
- id: 1ee4cf8d-a702-4dcb-a9e7-dac764bfb0c4
  type: mcq
  front: According to the Li2 framework (2025), what are the three phases of grokking?
  back: 'The Li2 framework decomposes grokking into three phases: (I) lazy learning, where the model memorizes
    examples; (II) independent feature learning, where salient features emerge; and (III) interactive
    feature learning, where features compose and generalization occurs suddenly.'
  choices:
  - key: a
    text: Pre-training, fine-tuning, inference
    correct: false
  - key: b
    text: Lazy learning (memorization), independent feature learning, interactive feature learning (sudden
      generalization)
    correct: true
  - key: c
    text: Underfitting, optimal fit, overfitting
    correct: false
  - key: d
    text: Early stopping, convergence, divergence
    correct: false
- id: b41e80b3-2536-48ff-8c9c-66d8831c51f3
  type: flip
  front: Why is weight decay essential for grokking to occur?
  back: Weight decay penalizes large weights, making memorization (which requires large, example-specific
    weights) costly. Without decay, models memorize indefinitely. Decay forces discovery of simpler, generalizable
    solutions—the underlying algorithm.
- id: 5ff81000-4fc1-4158-92ea-f693866a4dec
  type: mcq
  front: In the context of grokking, what is the relationship between model size, data size, and grokking
    threshold?
  back: Recent theory shows larger models can grok faster due to greater representational capacity. For
    algorithmic tasks (e.g., modular arithmetic), O(M log M) training samples are necessary and sufficient
    for a model to learn the underlying rule, with grokking timing depending on both N and data size.
  choices:
  - key: a
    text: Larger models grok later; more data speeds up grokking.
    correct: false
  - key: b
    text: Larger models grok earlier; O(M log M) data samples suffice for modular arithmetic of order
      M.
    correct: true
  - key: c
    text: Model size is irrelevant; only learning rate matters.
    correct: false
  - key: d
    text: Grokking time is constant regardless of model and data size.
    correct: false
---

## Intuition

Grokking is a striking phenomenon: a model trains for a very long time (1000s of steps) with high training loss and near-zero test accuracy, then suddenly (in a few steps) test accuracy jumps to 100% while training loss plateaus. The model "grok"s the pattern—discovers the underlying algorithm rather than memorizing examples.

## Detail

Key observations (Power et al., 2019; recent 2025 theory):

- **Grokking occurs on algorithmic tasks** (modular arithmetic, sorting, algorithmic patterns), especially with weight decay and small batch sizes
- **Three phases (Li2 framework, 2025):**
  1. *Lazy learning* (steps 0–T₁): Model memorizes training examples; overfits. Loss decreases, generalization gap large
  2. *Independent feature learning* (steps T₁–T₂): Model begins extracting features (e.g., parity, divisibility) independently; still poor generalization
  3. *Interactive feature learning* (steps T₂–T₃): Features interact and compose; sudden generalization. Test loss drops sharply
- **Grokking threshold:** Model size and weight decay jointly determine when grokking occurs. Larger models grok earlier (fewer steps), with O(M log M) data samples sufficient for modular arithmetic of order M
- **Not a loss phenomenon:** Pre-grokking, training loss may be quite low (model has memorized). Grokking is about generalization (test loss), not training loss

## Common gotchas / interview framings

- **"Why don't models always grok? Why does it fail on some tasks?"** → Grokking requires tasks with clear underlying structure (algorithmic, rule-based). Real-world tasks with high noise or multiple valid heuristics often don't grok; models memorize and never generalize
- **"Is grokking relevant to LLM training?"** → Controversial. LLMs train on diverse, noisy, unstructured text. Grokking-like phenomena may occur on specific sub-distributions (e.g., arithmetic reasoning), but it's not universal
- **"Why is weight decay crucial for grokking?"** → Weight decay penalizes memorization (which requires large weights). It forces the model to find simpler, generalizable solutions—the underlying algorithm rather than example-specific weights
- **"How long should I train to see grokking?"** → Depends on task complexity and model size. Algorithmic tasks: 10–1000s of epochs. Real data: grokking is rare or imperceptible. Design of the task and hyperparameters (learning rate, weight decay) are critical

## See also
- [[loss-landscape]]
- [[memorization]]
- [[generalization]]
- [[weight-decay]]
- [[training-dynamics]]
- [[feature-emergence]]

## Sources
See frontmatter `sources:`.
