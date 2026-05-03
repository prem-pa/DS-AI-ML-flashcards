---
id: 206be026-9986-4b75-9937-65f1ab038abc
title: Scaling laws for in-context learning
track: ai-llm-engineer
topic: prompt-engineering-in-context-learning
difficulty: 5
tags:
- scaling-laws
- few-shot
- sample-efficiency
- performance-curves
- empirical-analysis
aliases:
- scaling behavior
- example efficiency
- performance plateaus
sources:
- url: https://www.promptingguide.ai/techniques/cot
  label: 'Prompting Guide: Chain-of-Thought'
- url: https://www.getmaxim.ai/articles/a-practitioners-guide-to-prompt-engineering-in-2025/
  label: Practitioner's Guide to Prompt Engineering 2025
- url: https://www.meta-intelligence.tech/en/insight-prompt-engineering
  label: Prompt Engineering Guide 2026
cards:
- id: a0f76614-d5b1-4aee-84e5-aaa38a2cef32
  type: flip
  front: Describe the typical scaling law for in-context learning performance.
  back: 'Accuracy follows a power law: each doubling of examples yields ~5-10% gain until saturation.
    Performance curve: steep initial gains (1-2 examples = 50-70% of max), then diminishing returns (3-5
    examples = 70-85%). Most tasks saturate by 5-10 examples due to task simplicity or model capacity
    limits.'
- id: d91e98ae-77b9-4c6c-b17e-67bca1e8ba99
  type: flip
  front: Why do few-shot scaling curves plateau, and what determines the asymptotic accuracy?
  back: 'Plateaus occur because: (1) the model infers the task pattern from a few examples, (2) additional
    examples provide redundant information, and (3) context window limits prevent adding infinitely many
    examples. Asymptotic accuracy is determined by task complexity and model capacity, not example count.'
- id: bbf7c4cc-6769-4646-84df-0f8747f0e3fa
  type: flip
  front: How does task difficulty affect the scaling law for in-context learning?
  back: Easy tasks (binary classification) saturate at 1-2 examples (steep curve, early plateau). Hard
    tasks (multi-step reasoning) have shallower curves and benefit from 5-10 examples. Task complexity,
    not model size, drives the shape and saturation point of the scaling curve.
- id: d67b1251-bf89-4ed3-a79f-b658359e635d
  type: mcq
  front: You have a budget of 1000 tokens for examples in a classification task. How many examples should
    you choose?
  back: Scaling law shows 1-2 examples capture 50-70% of maximum accuracy gain. Typical tasks saturate
    by 5 examples. For classification, 1-2 well-chosen examples are highly cost-effective. Save remaining
    tokens for instructions, test queries, or output formatting.
  choices:
  - key: a
    text: As many as possible (e.g., 20 examples) to cover all cases
    correct: false
  - key: b
    text: 1-2 diverse examples; diminishing returns after that; use remaining tokens for other parts of
      the prompt
    correct: true
  - key: c
    text: At least 10 examples to ensure statistical significance
    correct: false
  - key: d
    text: The number doesn't matter; token cost is the same either way
    correct: false
- id: 7c3b1916-bd5e-40cc-bb10-1c25b407ec14
  type: mcq
  front: A model achieves 70% accuracy with 1 example and 78% with 5 examples. What should you predict
    for 20 examples?
  back: Power law curves show diminishing returns. 1→5 examples = 8% gain; 5→20 examples likely yields
    only 2-4% more. Model hits asymptote (saturation) around 78-80%. Accuracy doesn't improve linearly
    or exceed saturation; overfitting doesn't occur in ICL (saturating is the bound).
  choices:
  - key: a
    text: 92% (linear extrapolation)
    correct: false
  - key: b
    text: ~80-82% (approaching saturation; diminishing returns)
    correct: true
  - key: c
    text: 100% (all data points covered)
    correct: false
  - key: d
    text: Below 78% (overfitting kicks in)
    correct: false
---

## Intuition

Adding more in-context examples improves task performance, but with diminishing returns. Empirically, accuracy follows a power law: each doubling of examples yields ~5-10% performance gain until saturation. Understanding these curves helps practitioners choose how many examples to include, balancing cost (token count) against accuracy gains.

## Detail

**Empirical scaling law (approximate):**

```
Accuracy = A - B * (n_examples)^(-α)
```

where:
- **A** = asymptotic (max) accuracy
- **B** = proportionality constant
- **n_examples** = number of in-context examples
- **α** ≈ 0.3–0.5 (varies by task)

**Typical performance curves:**

```
Accuracy
   |     _______ (saturates here)
   |   /
   | /
   |/______________________________ n_examples
   0   1   2   4   8  16
```

Key observations:
- **1-2 examples:** 50-70% of max gain (most cost-effective).
- **3-5 examples:** 70-85% of max gain.
- **5-10+ examples:** Marginal gains; often not worth token cost.

**Task and model dependence:**
- **Easy tasks (e.g., binary classification):** Saturate at 1-2 examples; 10+ examples add minimal value.
- **Hard tasks (e.g., math, multi-hop reasoning):** Benefit from 5-10 examples; curve is less steep.
- **Larger models:** Typically have higher asymptotic accuracy but similar saturation point (shaped by task complexity, not model size).

**Context window as hard limit:**
The context window (e.g., 4K, 200K tokens) is an absolute ceiling. 4K tokens ≈ 2-4 examples for a Q&A task; 200K ≈ 50-100 examples. Exceeding this forces example dropping or task reformulation.

## Common gotchas / interview framings

- **Saturation ≠ overfitting:** In ICL, saturation is due to task simplicity and model capacity, not overtraining. Adding more examples past saturation wastes tokens but doesn't hurt accuracy.
- **Distribution matters:** Scaling law assumes examples are representative. Out-of-distribution or mislabeled examples shift the curve downward or create plateaus earlier.
- **Task complexity drives curve:** Easy tasks saturate quickly; hard tasks have shallower curves and higher α (slower saturation). Estimate task difficulty before choosing example count.
- **Cost-quality tradeoff:** 1-2 examples are cost-optimal; 5-10 is diminishing returns. Quantify your accuracy target vs. token budget.
- **Model size != example efficiency:** Larger models don't need fewer examples; they just have higher asymptotic accuracy.

## See also
- [[few-shot-vs-zero-shot-prompting]]
- [[why-icl-works-gradient-descent-interpretation]]
- [[context-order-and-recency-bias]]

## Sources
See frontmatter `sources:`.
