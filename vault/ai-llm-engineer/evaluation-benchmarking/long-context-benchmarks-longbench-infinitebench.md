---
id: 5fb4e769-bedc-4205-b045-ccb07b3b2283
title: Long-context benchmarks (LongBench, InfiniteBench)
track: ai-llm-engineer
topic: evaluation-benchmarking
difficulty: 3
tags:
- long-context
- context-window
- benchmarks
- retrieval
- needle-haystack
aliases:
- context length evaluation
- long-range dependency
- extended context
sources:
- url: https://lmmarketcap.com/benchmarks
  label: AI Benchmarks 2026
- url: https://llm-stats.com/benchmarks
  label: LLM Benchmarks 2026
- url: https://www.lxt.ai/blog/llm-benchmarks/
  label: LLM Benchmarks Compared
cards:
- id: ebf410e1-5ef3-4407-8ef3-d800ea5d0934
  type: flip
  front: What is position bias in long-context evaluation, and how do benchmarks like InfiniteBench expose
    it?
  back: 'Position bias: models attend unevenly to information at different positions (start/end bias).
    InfiniteBench strategically places facts at varying positions to measure uniform retrieval accuracy.'
- id: f700c5bf-3a7a-46c4-915a-171a58ee8958
  type: mcq
  front: When would you use InfiniteBench instead of LongBench for evaluating a production RAG system?
  back: InfiniteBench specifically targets extreme context lengths and synthetic adversarial tasks; LongBench
    is more applied.
  choices:
  - key: a
    text: Always use LongBench for production
    correct: false
  - key: b
    text: Use InfiniteBench for 100K+ token stress tests and synthetic failure cases
    correct: true
  - key: c
    text: Both measure the same thing
    correct: false
  - key: d
    text: InfiniteBench is only for researchers
    correct: false
- id: 64f283f7-11f5-4ba6-ac4b-1a6cb751215a
  type: flip
  front: Why does a model supporting 200K tokens not guarantee faithful information retrieval at 100K
    depth?
  back: Context window size is hardware capacity; faithful retrieval depends on attention mechanism and
    position embeddings. Models can interpolate positions without truly understanding uniform context,
    exposing via InfiniteBench retrieval tasks.
- id: 87a0f103-b35b-4c94-b570-305872d317f4
  type: mcq
  front: 'LongBench covers which of the following task categories? (Select all that apply for interview:
    mention 3+)'
  back: LongBench includes single-doc QA, multi-doc QA, summarization, few-shot, synthetic, and code completion—not
    adversarial attacks or multimodal.
  choices:
  - key: a
    text: Single-document QA
    correct: false
  - key: b
    text: Code completion
    correct: false
  - key: c
    text: Adversarial attacks
    correct: false
  - key: d
    text: Image captioning
    correct: false
---

## Intuition
Not all LLM capabilities matter at 2K-token windows. Long-context benchmarks isolate whether models maintain coherence, retrieval accuracy, and reasoning quality when operating on 4K–100K+ token inputs. Critical for RAG, document analysis, and reasoning over large codebases.

## Detail
**LongBench**: Multi-task long-document benchmark covering 6 categories—single-document QA, multi-document QA, summarization, few-shot learning, synthetic task, and code completion—over documents ranging 4K–20K tokens. Tests whether the model can:
- Locate relevant information across large context
- Maintain coherence over extended spans
- Perform few-shot learning with long exemplars

**InfiniteBench**: Tests context up to 100K tokens with synthetic and real tasks. Includes "retrieval" (find a fact buried in irrelevant context), "counting" (count occurrences), and code tasks. Specifically designed to stress-test context window limits.

**Key differences from short-context benchmarks**: Traditional benchmarks (MMLU, HumanEval) assume ~2K context. Long-context benchmarks expose degradation ("context length" or "position bias") where models struggle to:
- Attend to information at sequence ends (suffix bias)
- Retrieve facts uniformly across the span
- Maintain reasoning coherence over 50K+ tokens

## Common gotchas / interview framings
- **Position bias**: Models often attend better to information at document start/end; InfiniteBench exposes this via strategic fact placement.
- **Synthetic vs. real**: InfiniteBench synthetic tasks (counting) differ from real tasks (QA); both test different failure modes.
- **Context window ≠ capability**: A 200K token context window doesn't guarantee faithful retrieval at 100K; benchmark to validate.
- **Interview framing**: "How would you evaluate whether an LLM truly supports 100K context or just linearly interpolates position embeddings?" Use InfiniteBench's retrieval tasks.

## See also
- [[long-context]]
- [[context-window]]
- [[needle-in-haystack]]
- [[retrieval-in-long-documents]]
- [[token-efficiency]]

## Sources
See frontmatter `sources:`.
