---
id: 4cd6623f-b277-487b-a435-7b67f7b53737
title: Downstream scaling laws
track: research-leaning
topic: scaling-laws-emergent-abilities
difficulty: 5
tags:
- scaling-laws
- transfer-learning
- downstream-tasks
- generalization
- few-shot-learning
aliases:
- task-specific scaling
- transfer scalability
- downstream generalization
sources:
- url: https://arxiv.org/abs/2203.15556
  label: Hoffmann et al., downstream task evaluation in compute-optimal models (NeurIPS 2022)
- url: https://arxiv.org/abs/1910.07324
  label: 'Kaplan et al., Scaling Laws II: Downstream tasks (OpenAI 2020)'
cards:
- id: 16d7a0d1-3925-45fd-926b-a1dcf55f5465
  type: flip
  front: Define downstream scaling law and explain why it matters beyond pre-training loss.
  back: A downstream scaling law quantifies how task-specific metrics (e.g., MMLU accuracy) improve with
    model size. It matters because pre-training loss is only a proxy; actual performance on real tasks
    may not improve smoothly even if loss does, and different tasks have different scaling rates.
- id: f628fddc-793f-414c-b704-f4381865d7bb
  type: mcq
  front: Why do some downstream tasks show 'sudden jumps' in performance while pre-training loss improves
    smoothly?
  back: Task performance often uses threshold-based metrics (multiple-choice scoring, exact-match accuracy).
    When these metrics are applied to smooth underlying improvements, small changes in loss can cause
    large jumps in accuracy if they push examples across the decision boundary.
  choices:
  - key: a
    text: Larger models have qualitatively different internal representations.
    correct: false
  - key: b
    text: The evaluation metric (e.g., binary accuracy) is nonlinear; small loss improvements can push
      borderline examples over a threshold.
    correct: true
  - key: c
    text: Pre-training loss and task accuracy are fundamentally unrelated.
    correct: false
  - key: d
    text: Downstream tasks require different training data that doesn't follow power laws.
    correct: false
- id: 9e3a3f48-e887-4e5d-9c4f-7d4cb409909d
  type: flip
  front: Approximately how much do zero-shot and few-shot downstream task accuracies differ in their dependence
    on model size?
  back: Few-shot learning typically shows steeper scaling (larger exponent α_task) with model size than
    zero-shot; this is interpreted as emergent in-context learning. Zero-shot plateaus faster; few-shot
    improves steeply, suggesting ICL is a learned capability.
- id: 11d87ec1-872e-4b68-8f9d-bb44bfe0f08b
  type: mcq
  front: You observe that a model has lower pre-training loss than a baseline but worse downstream accuracy
    on MMLU. What is the most likely explanation?
  back: Lower pre-training loss is necessary but not sufficient for downstream performance. Instruction-tuning,
    RLHF, and prompt engineering are critical for converting raw language understanding into task-specific
    capability. Downstream scaling laws hold on average but have high variance for individual models.
  choices:
  - key: a
    text: The model overfitted to the pre-training dataset.
    correct: false
  - key: b
    text: The model needs instruction-tuning or task-specific adaptation to convert pre-training knowledge
      into task performance.
    correct: true
  - key: c
    text: Pre-training loss is useless for predicting downstream performance.
    correct: false
  - key: d
    text: The model is larger but uses a worse tokenizer.
    correct: false
---

## Intuition

A model's pre-training loss is just a proxy for real capability. The core question is: **how much do improvements in pre-training loss translate to improvements on real downstream tasks?** Downstream scaling laws quantify this transfer, showing whether a 10% reduction in pre-training loss yields 10% improvement on MMLU, code, or instruction-following—or if the relationship is weaker.

## Detail

Downstream scaling laws show that task performance typically improves as a power law with model size:

**Accuracy(N) ∝ N^{α_{task}}** (for zero-shot or few-shot evaluation)

Key findings:
- **Not all tasks scale equally:** Hard tasks (rare patterns, reasoning) show steep slopes (α_task ≈ 0.07–0.1); easy tasks (token classification) show shallow slopes (α_task ≈ 0.01–0.03)
- **Pre-training loss is predictive but imperfect:** A model with lower pre-training loss usually performs better on downstream tasks, but the relationship is noisy
- **Few-shot learning benefits more:** Zero-shot accuracy plateaus faster than few-shot; few-shot improves steeply with model size (reflects emergent ICL)
- **Task-specific phenomena:** Some tasks show sudden jumps (apparent emergence), others scale smoothly—often due to evaluation metrics (continuous vs. binary accuracy)

## Common gotchas / interview framings

- **"If pre-training loss is smooth, why do downstream tasks show sudden jumps?"** → Task-level metrics (threshold accuracy, multiple-choice scoring) can amplify small loss changes into large accuracy jumps. A 5% loss improvement might cause 50% of borderline examples to flip from wrong to right.
- **"Should we always optimize for pre-training loss?"** → Yes, as a proxy, but downstream performance can diverge. Instruction-tuning and RLHF add task-specific alignment that pre-training loss doesn't capture.
- **"How do we know if a model is 'undertrained' on downstream tasks?"** → Compare actual downstream performance to pre-training loss predictions. Large gaps suggest the model needs fine-tuning or task-specific adaptation.
- **Practical note:** Different downstream datasets (MMLU, MATH, HumanEval) may have different scaling exponents; no single exponent fits all tasks.

## See also
- [[pre-training-loss]]
- [[fine-tuning]]
- [[zero-shot-learning]]
- [[few-shot-prompting]]
- [[multi-task-learning]]
- [[task-transfer]]

## Sources
See frontmatter `sources:`.
