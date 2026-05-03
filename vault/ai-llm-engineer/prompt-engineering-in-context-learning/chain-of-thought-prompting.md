---
id: 30606691-f7fe-4b57-b1d5-5d23aaa892eb
title: Chain-of-thought prompting
track: ai-llm-engineer
topic: prompt-engineering-in-context-learning
difficulty: 3
tags:
- reasoning
- step-by-step
- chain-of-thought
- cot
- complex-tasks
- intermediate-steps
aliases:
- CoT
- step-by-step reasoning
- thought trace
sources:
- url: https://www.promptingguide.ai/techniques/cot
  label: 'Prompting Guide: Chain-of-Thought'
- url: https://calmops.com/ai/prompt-engineering-patterns-cot-react-tot/
  label: 'Prompt Engineering Patterns: CoT, ReAct, ToT'
- url: https://medium.com/data-science-in-your-pocket/react-chain-of-thoughts-and-trees-of-thoughts-explained-with-example-b9ac88621f2c
  label: ReAct, CoT, and ToT Explained with Examples
cards:
- id: 1c671f85-f763-41c3-a832-eb3cc1d89af8
  type: flip
  front: What is the core mechanism of chain-of-thought (CoT) prompting?
  back: CoT instructs the model to decompose a complex task into explicit intermediate reasoning steps
    before generating the final answer. This 'show your work' approach activates the model's latent reasoning
    capabilities and reduces errors, especially on math, logic, and multi-hop tasks.
- id: 9e634d57-06b5-4098-b692-a595375b399d
  type: flip
  front: What is the performance difference between zero-shot CoT and few-shot CoT?
  back: Zero-shot CoT ('Let's think step by step') provides a modest boost over direct prompting. Few-shot
    CoT, which provides 1-3 worked examples with annotated reasoning, dramatically improves performance—often
    20-40% accuracy gain on reasoning tasks—by showing the desired reasoning format.
- id: ee30f1f7-c6ba-42e2-b017-7d477173d449
  type: flip
  front: Describe two limitations of chain-of-thought prompting and how to mitigate them.
  back: 1) **Token cost:** CoT responses are 1.5–2× longer, increasing latency and cost. Mitigate by using
    dynamic CoT only on hard queries. 2) **Hallucinated reasoning:** Intermediate steps may be plausible
    but incorrect. Mitigate by combining with self-consistency sampling—generate multiple CoT traces and
    vote on the answer.
- id: 0ae7887b-5e14-4da4-96b0-b8a36f02e465
  type: mcq
  front: When does chain-of-thought prompting provide the largest accuracy improvement?
  back: CoT is most effective on tasks requiring step-by-step reasoning. On simple/factual tasks, CoT
    adds cost with minimal accuracy gain. On reasoning tasks, intermediate steps catch errors that direct
    prediction misses.
  choices:
  - key: a
    text: Simple factual questions where the model already has high confidence
    correct: false
  - key: b
    text: Complex multi-step reasoning tasks like math problems, logic puzzles, and multi-hop inference
    correct: true
  - key: c
    text: Tasks requiring factual recall from training data
    correct: false
  - key: d
    text: Any task, regardless of complexity
    correct: false
- id: 56f4ea0c-bf01-44e1-8d0d-2ebbdbd6a505
  type: mcq
  front: How should you design a few-shot CoT prompt for best results?
  back: Few-shot CoT benefits from high-quality, domain-matched examples (1-3 is sufficient due to diminishing
    returns). Showing explicit reasoning steps is crucial. More examples don't necessarily help; example
    quality and diversity matter far more.
  choices:
  - key: a
    text: Provide as many examples as possible (10+) to cover all reasoning types
    correct: false
  - key: b
    text: Show 1-3 diverse, high-quality examples with clear step-by-step explanations that match the
      test task difficulty
    correct: true
  - key: c
    text: Use random examples from any domain to encourage generalization
    correct: false
  - key: d
    text: Provide only the final answer for each example, avoiding intermediate steps
    correct: false
---

## Intuition

Chain-of-Thought (CoT) prompting instructs LLMs to decompose complex reasoning tasks into intermediate steps, explaining their logic before producing a final answer. This "show your work" technique mimics human problem-solving and dramatically improves accuracy on math, logic, and multi-hop reasoning tasks.

## Detail

**Core mechanic:** Add phrases like "Let's think step by step" (zero-shot CoT) or provide examples with annotated reasoning steps (few-shot CoT). The model generates explicit intermediate reasoning, which:

1. **Reduces errors:** Forces the model to catch logical contradictions internally.
2. **Improves complex reasoning:** Breaks multi-step math, logic puzzles, and commonsense tasks into solvable substeps.
3. **Enhances explainability:** Reasoning traces make decisions auditable (critical for safety/compliance).

**Example:**

Prompt: "Which is larger: 9.11 or 9.9? Explain your reasoning step by step."

Output:
"First, align decimals: 9.11 vs 9.90. Compare first decimal place: both are 1, so equal. Compare second decimal place: 1 < 9. Therefore, 9.90 > 9.11. Answer: 9.9 is larger."

**Limitations:**
- Token cost increases (1.5–2× longer responses).
- [[tree-of-thoughts-and-graph-of-thoughts]] extends CoT to explore multiple reasoning branches when one path fails.
- Hallucinated intermediate steps can mislead; combine with [[self-consistency-and-majority-voting]] to reduce false confidence.

## Common gotchas / interview framings

- **CoT doesn't help easy tasks:** Performance gain is task-dependent; minimal on simple questions where the model already succeeds.
- **Token cost vs accuracy:** Each CoT trace costs 1.5–2× input tokens; balance against latency SLA and cost constraints.
- **Hallucinated reasoning:** Intermediate steps may sound plausible but contain errors; don't blindly trust the trace.
- **Few-shot CoT outperforms zero-shot:** Providing examples of reasoning dramatically boosts performance; simple "think step by step" is a baseline.
- **Reasoning quality varies by model:** Larger models (GPT-4, Claude) benefit more; smaller models sometimes regress with CoT.

## See also
- [[few-shot-vs-zero-shot-prompting]]
- [[tree-of-thoughts-and-graph-of-thoughts]]
- [[self-consistency-and-majority-voting]]

## Sources
See frontmatter `sources:`.
