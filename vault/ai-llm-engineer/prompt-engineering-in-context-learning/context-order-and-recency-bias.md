---
id: 261ebfe4-6829-49cb-b1d9-0cd0a09b8505
title: Context order and recency bias
track: ai-llm-engineer
topic: prompt-engineering-in-context-learning
difficulty: 3
tags:
- context-order
- recency-bias
- example-ordering
- position-effects
- attention-mechanics
aliases:
- example ordering
- position bias
- primacy-recency
sources:
- url: https://www.promptingguide.ai/techniques/cot
  label: 'Prompting Guide: Chain-of-Thought'
- url: https://capabl.in/blog/advanced-prompt-engineering-mastering-tree-of-thoughts-react-and-role-based-prompts-for-superior-llm-performance
  label: Advanced Prompt Engineering Guide
- url: https://www.meta-intelligence.tech/en/insight-prompt-engineering
  label: Prompt Engineering Guide 2026
cards:
- id: 45a6c829-abd5-4085-83d9-f52a1b7456ba
  type: flip
  front: What is recency bias in in-context learning and how does it differ from human learning?
  back: 'Recency bias: Examples placed near the test query have 2-3× higher influence than earlier examples.
    Reordering can swing accuracy 5-15%. This contrasts with human primacy effects (first examples dominate
    human memory). Transformers naturally weight recent tokens higher due to positional encodings and
    gradient flow.'
- id: b01be65c-f41d-4cd2-bb31-8c265ebc32ff
  type: flip
  front: Why do transformers exhibit recency bias in in-context learning?
  back: 'Three mechanisms: (1) **Positional encodings** track position; later tokens may be attended more
    due to next-token prediction structure. (2) **Gradient flow** is shorter from later tokens to the
    query. (3) **Optimization dynamics:** Models overwrite earlier examples with later ones, similar to
    SGD momentum—later examples are ''fresher'' in the model''s representation.'
- id: 3d2feaca-01b2-46f3-9dec-df5a664d94a8
  type: flip
  front: How should you arrange in-context examples to maximize performance?
  back: Place your most important/diverse examples immediately before the test query (last positions).
    Order examples by relevance to test query if possible. Avoid placing examples at the very start of
    context. Empirically test different orderings on a validation set—reordering can yield 5-15% accuracy
    gains.
- id: 6ba3c79c-70f7-4ad7-a1a8-f72b92b6ddfa
  type: mcq
  front: You have three in-context examples for a classification task. Where should you place them for
    maximum accuracy?
  back: 'Recency bias is strong: examples closest to the query have highest influence. Placing them immediately
    before the test query maximizes their weight. Scattering or placing at the start reduces their impact
    and wastes their potential.'
  choices:
  - key: a
    text: At the start of the prompt, after system instructions but before the test query
    correct: false
  - key: b
    text: Immediately before the test query, in order of relevance to the test case
    correct: true
  - key: c
    text: Scattered randomly throughout the prompt
    correct: false
  - key: d
    text: All in the middle of the context window
    correct: false
- id: 3042da18-f005-474a-b59d-fb2873d6f2d7
  type: mcq
  front: You test two example orderings and find a 7% accuracy difference. What does this tell you?
  back: 7% gap is substantial and consistent with recency bias findings. Position matters significantly.
    The empirical result shows examples are position-sensitive; optimize ordering by moving high-quality
    examples near the query and validating on a held-out set.
  choices:
  - key: a
    text: One ordering is objectively better; always use it
    correct: false
  - key: b
    text: Position effects are significant for your task; recency bias dominates—place examples closest
      to the query
    correct: true
  - key: c
    text: The difference is random noise; orderings don't matter
    correct: false
  - key: d
    text: You should use more examples to eliminate position bias
    correct: false
---

## Intuition

Transformers are position-sensitive: the order in which you place in-context examples dramatically affects task performance. Empirically, recent examples (closer to the query) have higher influence than earlier ones—a recency bias. This counterintuitive finding (contrary to human learning, where primacy often dominates) reveals how transformer attention mechanisms process sequential context.

## Detail

**Empirical observations:**

1. **Recency bias dominates:** Examples placed immediately before the test query (last 1-2 positions) have 2-3× higher influence than examples at the start of context. Reordering can change accuracy by 5-15%.

2. **Primacy effects are weak:** The first example often has minimal impact compared to the last. This differs from human learning where first examples set a strong prior.

3. **Scrambling hurts:** Random reordering reduces performance ~5-10%; consistent order (even arbitrary) is better than jumbling.

**Mechanism (from mechanistic interpretation):**

Transformer attention exhibits strong position biases:
- **Positional encodings:** Track absolute positions; attention may naturally weight later tokens higher due to causality (next token prediction).
- **Gradient flow:** Information flows forward in transformers; examples at the end have shorter gradient paths to the query.
- **Optimization dynamics:** Models may overwrite earlier examples with later ones (similar to SGD with momentum), making recent examples "fresher" in the model's memory.

**Practical implications:**

```
Weak setup (examples at start):
Example 1: ...
Example 2: ...
Example 3: ...
[Test Query] → Accuracy: 82%

Strong setup (examples at end):
[Test Query]
Example 1: ...
Example 2: ...
Example 3: ...
[Test Query] → Accuracy: 87% (5% boost)

Best setup (interleaved):
[Context] → Example 1 + Example 2 + [Test Query 1] → Example 3 + [Test Query 2]
Accuracy depends on query placement; closest examples dominate.
```

## Common gotchas / interview framings

- **Recency bias is opposite to human primacy effect:** Models learn from recent examples more; humans remember first examples better. Design prompts accordingly.
- **Position matters more than content:** Even with identical examples, reordering can swing accuracy 5-15%; systematic testing required.
- **Interleaving can degrade:** Alternating examples with queries works for some tasks but confuses others; requires empirical validation.
- **Context window constraint amplifies bias:** With limited examples, recency dominance is strong. With 20+ examples, earlier ones are further from the query and weak.
- **Task dependency:** Reasoning tasks show stronger recency than classification; task type affects position sensitivity.

## See also
- [[few-shot-vs-zero-shot-prompting]]
- [[scaling-laws-for-in-context-learning]]
- [[why-icl-works-gradient-descent-interpretation]]

## Sources
See frontmatter `sources:`.
