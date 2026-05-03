---
id: d5781876-29bb-41a6-9b27-fa27741cce8d
title: LLM-as-judge evaluation
track: ai-llm-engineer
topic: evaluation-benchmarking
difficulty: 3
tags:
- LLM-judge
- automatic-scoring
- bias-risk
- cost-efficiency
- reference-answer
aliases:
- LLM judge
- automatic evaluation
- GPT-4-as-judge
- learned judges
sources:
- url: https://www.meta-intelligence.tech/en/insight-llm-evaluation
  label: LLM Evaluation Framework 2026
- url: https://arxiv.org/abs/2306.05685
  label: Judging LLM-as-a-Judge with MT-Bench and Chatbot Arena
- url: https://deepeval.com/docs/metrics-ragas
  label: DeepEval Ragas Metrics
cards:
- id: 9e483757-c5ba-43ef-adcd-0ee9c6276481
  type: flip
  front: What is the primary source of bias in LLM-as-judge evaluation?
  back: Judge LLMs have systematic preferences shaped by training. GPT-4o might favor verbose outputs;
    Claude conciseness. Position bias, anchoring effects, and reference-answer dependency also introduce
    bias. Always validate on manual samples.
- id: 98b37abf-2844-4fd1-bcb6-93905d5a0ebf
  type: mcq
  front: You're evaluating a summarization task with 5K examples using GPT-4o as judge. To reduce judge
    bias, what technique would you use?
  back: Multi-agent judges (Critic, Psychologist, Scientist) reduce single-judge bias through role diversity.
    LMSYS-Chat uses this approach in 2026.
  choices:
  - key: a
    text: Use default temperature
    correct: false
  - key: b
    text: Use multi-agent judges with different roles
    correct: true
  - key: c
    text: Skip validation
    correct: false
  - key: d
    text: Increase judge context
    correct: false
- id: 21b50219-8c99-480e-9fff-34e5cd0e7465
  type: flip
  front: Why is reference-answer quality critical in reference-based LLM-as-judge evaluation?
  back: Judge rates the model output relative to reference answer. If reference is wrong, judge might
    rate incorrect outputs as correct (garbage-in, garbage-out). For RAG, this is less problematic; for
    generation, reference quality matters greatly.
- id: e04b42b7-3001-4b20-b2c2-37d3baf90191
  type: mcq
  front: When comparing LLM-as-judge scores for the same output from different judge LLMs (GPT-4o vs.
    Claude), what caveat applies?
  back: Different judge LLMs have different training styles and biases. Scores across judges cannot be
    directly compared; calibrate each separately.
  choices:
  - key: a
    text: Scores are directly comparable
    correct: false
  - key: b
    text: Scores are not directly comparable; judges have different biases
    correct: true
  - key: c
    text: Use average of both judges
    correct: false
  - key: d
    text: Discard one judge's scores
    correct: false
---

## Intuition
Hiring a human to annotate 10K test cases is expensive ($100K+). LLM-as-judge uses a powerful LLM (GPT-4o, Claude 3.5) to automatically score outputs, reducing cost to ~$0.01 per evaluation while enabling high throughput. Trade-off: judge LLMs have systematic biases that skew results.

## Detail
**How it works**:
1. **Single-answer grading**: Judge sees question Q, model output O, and reference answer R (optional). Rates O on 1–10 scale. Pros: fast, cheap. Cons: reference-dependent (if R is wrong, judge might rate wrong outputs correctly).
2. **Multi-agent judges**: Assign roles (Critic, Psychologist, Scientist) to independent judge instances. Each debates the output, then aggregate scores. Reduces single-judge bias via diversity (2026 advancement).
3. **Reference-answer dependency**: LMSYS-Chat framework: judge generates its own reference answer first, then rates model output conditioned on that reference. Higher quality but more expensive (~2x cost).

**Bias and calibration**:
- **Judge preference**: LLM judges favor outputs similar to their training style. GPT-4o might rate verbose answers higher; Claude might favor conciseness.
- **Anchoring**: First answer rated influences subsequent ratings. Randomize presentation order.
- **Position bias**: Judge might favor first option in pairwise comparisons (Chatbot Arena uses random assignment to mitigate).

**Practical guidance** (2026):
- **For RAG**: Use reference answer (groundedness doesn't require reference; faithfulness does).
- **For generation**: Use multi-agent judges or aggregate multiple judge LLMs for robustness.
- **Validation**: Always compare judge scores on manual samples (10–20 examples). If correlation < 0.85 with humans, recalibrate prompts or change judge.

## Common gotchas / interview framings
- **Judge consistency**: Same judge, same output, different runs can give different scores. Temperature, randomness in judge impact evaluation. Use greedy decoding for reproducibility.
- **Cost illusion**: Judge cost is low per example but scales with test set size. 10K-example evaluation with GPT-4 = $10–20; budget accordingly.
- **Learned judges**: Some teams train custom judges on human annotations. More accurate but require labeled training data (expensive).
- **Interview framing**: "Design an evaluation approach for a customer support LLM. Manual annotation costs $50K; LLM judge costs $1K. What trade-offs would you accept?" Discuss judge bias, validation on manual samples, and tying business outcomes (customer satisfaction) to metrics.

## See also
- [[llm-as-judge]]
- [[automatic-evaluation]]
- [[judge-bias]]
- [[reference-answers]]
- [[multi-agent-judges]]
- [[calibration]]

## Sources
See frontmatter `sources:`.
