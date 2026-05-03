---
id: de0f7087-4a22-45a4-8c1b-42c2ffbf5ec1
title: Self-consistency and majority voting
track: ai-llm-engineer
topic: prompt-engineering-in-context-learning
difficulty: 5
tags:
- self-consistency
- majority-voting
- sampling
- variance-reduction
- ensemble
aliases:
- ensemble prompting
- vote aggregation
- multiple-sample averaging
sources:
- url: https://www.promptingguide.ai/techniques/cot
  label: 'Prompting Guide: Chain-of-Thought'
- url: https://calmops.com/ai/prompt-engineering-patterns-cot-react-tot/
  label: 'Prompt Engineering Patterns: CoT, ReAct, ToT'
- url: https://medium.com/data-science-in-your-pocket/react-chain-of-thoughts-and-trees-of-thoughts-explained-with-example-b9ac88621f2c
  label: ReAct, CoT, and ToT Explained
cards:
- id: da10e038-c3d4-456c-b8ef-476d00489de7
  type: flip
  front: What is the core mechanism of self-consistency prompting?
  back: Self-consistency generates k independent CoT samples from the model (with temperature > 0 for
    variation), extracts the final answer from each trace, and returns the majority vote. This ensemble
    approach reduces variance and detects uncertainty by examining answer spread.
- id: c6f323bd-f187-4963-aa40-455a4f449501
  type: flip
  front: Why does self-consistency improve accuracy over a single sample, and when does it provide the
    most benefit?
  back: LLM sampling is stochastic; averaging multiple samples smooths random errors. Benefit is largest
    on reasoning tasks (5-25% gain) where the model is uncertain but capable; minimal on tasks where single-sample
    accuracy is already high or where all samples are wrong.
- id: be227831-8ac9-4fd1-8313-e5bbd5e3fad2
  type: flip
  front: Explain the cost-quality tradeoff for self-consistency sampling with k samples.
  back: Token cost increases k-fold (k=5 → 5× tokens). Latency can be parallelized (wall-clock time ~same
    as single sample if enough GPU capacity). Accuracy gains diminish after k=5-10 (5-15% typical improvement).
    Best for batch/analytical systems; too expensive for real-time latency-critical applications.
- id: a66b3408-6fac-4f20-9261-748661b5b8d3
  type: mcq
  front: What temperature setting is essential for self-consistency to work?
  back: At temperature=0, all samples are identical (deterministic), defeating self-consistency. Moderate
    temperature (0.7-1.0) induces useful variation while keeping outputs coherent. Very high temperature
    produces nonsensical diversity.
  choices:
  - key: a
    text: Temperature must be 0 (greedy decoding) to ensure consistent sampling
    correct: false
  - key: b
    text: Temperature should be 0.7-1.0 to enable output variation across samples
    correct: true
  - key: c
    text: Temperature doesn't matter; voting is immune to repetition
    correct: false
  - key: d
    text: Temperature should be very high (>2.0) to maximize diversity
    correct: false
- id: ab3f5dc6-ce68-4897-a717-4ef46b67dba8
  type: mcq
  front: When designing a self-consistency system, what is a critical limitation to plan for?
  back: Self-consistency requires clean answer parsing. If traces have varied formats or you can't reliably
    extract the final answer, voting breaks down. Open-ended tasks (essay generation) are hard; structured
    answers (math, multiple-choice) are ideal.
  choices:
  - key: a
    text: Majority voting only works on classification tasks, not reasoning
    correct: false
  - key: b
    text: Answer extraction and standardization must be reliable; voting fails if you can't parse diverse
      answer formats robustly
    correct: true
  - key: c
    text: Self-consistency is immune to wrong answers; any vote is correct
    correct: false
  - key: d
    text: The approach requires custom training of the LLM
    correct: false
---

## Intuition

Self-consistency addresses the problem that LLM sampling is stochastic—the same prompt with the same temperature produces different outputs due to randomness in token generation. Rather than trusting a single response, sample multiple independent CoT traces, extract the final answer from each, and vote on the most common answer. This ensemble approach dramatically reduces variance and improves accuracy, especially on reasoning tasks.

## Detail

**Core mechanism:**

1. **Sample diversity:** Generate k independent samples from the model (typically k=5-20) with temperature > 0 to induce variation.
2. **Extract answers:** Parse the final answer from each trace (e.g., the number before the period in a math problem).
3. **Vote:** Count answer frequencies and return the majority vote (or weighted by confidence if available).

**Why it works:**
- **Reduces variance:** Averaging over multiple samples smooths out random errors.
- **Detects uncertainty:** If answers are scattered, confidence should be low; if clustered, high.
- **Improves reasoning:** CoT + self-consistency (5-25% accuracy gain) rivals more complex methods on some tasks.

**Example:**
```
Question: "If a train travels 200 km at 80 km/h, how long does the journey take?"

Sample 1: "Let's think... 200 ÷ 80 = 2.5 hours. Answer: 2.5"
Sample 2: "Distance is 200 km, speed 80 km/h... time = 200/80 = 2.5 hours. Answer: 2.5"
Sample 3: "The train goes 80 km in 1 hour, so 200 km takes 2.5 hours. Answer: 2.5"

Vote: 3/3 for 2.5 → HIGH CONFIDENCE
```

**Cost-quality tradeoff:**
- **Token cost:** k× the single-sample cost (k=5 → 5× tokens).
- **Latency:** Can be parallelized (k independent calls) for wall-clock speedup.
- **Accuracy:** Often 5-25% gain on reasoning tasks; diminishing returns after k=5-10.

## Common gotchas / interview framings

- **Requires answer extraction:** Majority voting only works if you can parse/standardize answers from diverse outputs; fragile for open-ended tasks.
- **Temperature must enable variation:** At temperature=0 (greedy), all samples are identical; temperature 0.7-1.0 required.
- **Outlier answers:** Some samples may have different (wrong) answers; voting is robust to 1-2 outliers but fails if majority is wrong.
- **Confidence calibration:** Voting confidence (unanimous vs. split) doesn't always correlate with correctness; don't overweight agreement metrics.
- **Cost-prohibitive for large k:** Sampling 20+ times on high-traffic systems is expensive; balance cost vs. accuracy gain.

## See also
- [[chain-of-thought-prompting]]
- [[scaling-laws-for-in-context-learning]]
- [[tree-of-thoughts-and-graph-of-thoughts]]

## Sources
See frontmatter `sources:`.
