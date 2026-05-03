---
id: d26f3fc3-e747-4b6e-85cf-5d0b0a48fcb1
title: Few-shot vs zero-shot prompting
track: ai-llm-engineer
topic: prompt-engineering-in-context-learning
difficulty: 1
tags:
- in-context-learning
- prompt-design
- few-shot
- zero-shot
- examples
- performance
aliases:
- few-shot learning
- zero-shot
- example-based prompting
sources:
- url: https://developers.openai.com/api/docs/guides/prompt-engineering
  label: OpenAI Prompt Engineering Guide
- url: https://www.promptingguide.ai/techniques/cot
  label: 'Prompting Guide: Chain-of-Thought'
- url: https://www.getmaxim.ai/articles/a-practitioners-guide-to-prompt-engineering-in-2025/
  label: Practitioner's Guide to Prompt Engineering 2025
cards:
- id: 5a4ff9af-abec-4028-839a-28a34e7f58f4
  type: flip
  front: What is the key performance difference between zero-shot and few-shot prompting?
  back: Few-shot prompting provides labeled examples before the task, enabling in-context learning (ICL)
    that improves accuracy 10-30% on specialized/OOD tasks. Zero-shot relies purely on pre-trained knowledge
    and instruction understanding—faster and cheaper but less accurate on unfamiliar domains.
- id: 1a6b1f52-746d-49b1-be69-a8d3a3f0fe7b
  type: flip
  front: When should you prefer zero-shot over few-shot prompting?
  back: 'Zero-shot is preferred when: (1) latency is critical, (2) cost per request must be minimized,
    (3) the task is familiar/well-represented in training data (e.g., basic summarization, standard classification),
    or (4) no high-quality examples are available.'
- id: f2a2061a-cdc9-4743-b0bf-c32fce0f398c
  type: flip
  front: Explain the scaling law for few-shot performance. Why does adding more examples plateau?
  back: 'Performance typically jumps with 1-2 examples (10-30% gain), then diminishes after 3-5 examples.
    Plateau occurs because: (1) the model rapidly infers the task pattern, (2) additional redundant examples
    consume tokens without new information, and (3) context window limits force dropping examples or earlier
    context.'
- id: b879bd08-08f2-4339-b9ff-af26730f719f
  type: mcq
  front: You're designing a prompt for a domain-specific entity extraction task. Which approach minimizes
    cost while maintaining 80%+ accuracy?
  back: 'Option 1 (zero-shot + detailed instructions) often yields <70% on OOD tasks. Option 2 is optimal:
    2-3 domain-matched examples provide most performance gains (diminishing returns after) while keeping
    input tokens low. Options 3-4 waste tokens and add noise.'
  choices:
  - key: a
    text: Zero-shot with a detailed instruction template
    correct: false
  - key: b
    text: Few-shot with 2-3 hand-selected, representative examples from the target domain
    correct: true
  - key: c
    text: Few-shot with 10+ random examples from the training set
    correct: false
  - key: d
    text: Few-shot with examples from a different domain to encourage generalization
    correct: false
- id: f66196f4-dcb9-4b5c-87fe-0c71a5ff2b1a
  type: mcq
  front: Which factor most strongly affects few-shot accuracy on held-out test data?
  back: Performance depends critically on example selection (distribution match, label correctness) and
    ordering (recent/last examples carry more weight due to recency bias in transformers). Raw count and
    length are secondary; well-chosen diverse examples beat many mediocre ones.
  choices:
  - key: a
    text: Total number of examples (more always better)
    correct: false
  - key: b
    text: Quality and domain relevance of examples, plus their order in the prompt
    correct: true
  - key: c
    text: The length of each example (longer examples are more informative)
    correct: false
  - key: d
    text: Whether examples are shuffled randomly before the task
    correct: false
---

## Intuition

Zero-shot prompting asks an LLM to perform a task with no examples, relying purely on its pre-trained knowledge and instruction understanding. Few-shot prompting provides 1-5 labeled examples before the actual task, leveraging in-context learning (ICL) to improve accuracy, consistency, and domain adaptation.

## Detail

**Zero-shot:** Simple, fast, cheap. Works well for familiar tasks (classification, summarization) where the model has seen similar patterns during training. Example:

```
Classify sentiment: "I love this product!"
Answer: Positive
```

**Few-shot:** Demonstrates the expected input/output format and reasoning style. Dramatically improves performance on specialized, ambiguous, or out-of-distribution tasks. Trades cost (more tokens) for accuracy.

```
Classify sentiment:
Sentence: "Great movie, highly recommend."
Answer: Positive

Sentence: "Terrible experience, waste of money."
Answer: Negative

Sentence: "It was okay."
Answer:
```

The performance gain follows a scaling law: 1-2 examples often provide 10-30% accuracy boost; diminishing returns after 4-5 examples. Order matters—recent examples [[context-order-and-recency-bias]] weighted more heavily.

## Common gotchas / interview framings

- **Cost-quality tradeoff:** Few-shot increases input tokens linearly; balance example count against latency/cost constraints.
- **Distribution mismatch:** Examples from training distribution may not help OOD tasks; validate empirically.
- **Label quality:** Incorrect examples harm more than help; ensure human-verified, representative samples.
- **Selection bias:** Random examples underperform; use clustering or semantic similarity to pick diverse, relevant examples.
- **Prompt sensitivity:** Reordering or rewording examples can swing accuracy 5-15%; systematize template design.

## See also
- [[chain-of-thought-prompting]]
- [[instruction-following-and-task-formatting]]
- [[scaling-laws-for-in-context-learning]]
- [[context-order-and-recency-bias]]

## Sources
See frontmatter `sources:`.
