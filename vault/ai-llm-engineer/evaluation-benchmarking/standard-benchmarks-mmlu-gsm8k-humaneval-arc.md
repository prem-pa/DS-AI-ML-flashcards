---
id: d66a56db-3bf7-4573-ad7b-1012f6139209
title: Standard benchmarks (MMLU, GSM8K, HumanEval, ARC)
track: ai-llm-engineer
topic: evaluation-benchmarking
difficulty: 3
tags:
- benchmarks
- standardized
- knowledge
- coding
- reasoning
aliases:
- MMLU
- GSM8K
- HumanEval
- general knowledge benchmarks
sources:
- url: https://lmmarketcap.com/benchmarks
  label: AI Benchmarks 2026
- url: https://tokenmix.ai/blog/llm-leaderboard-2026
  label: LLM Leaderboard Guide 2026
- url: https://www.lxt.ai/blog/llm-benchmarks/
  label: LLM Benchmarks Compared 2026
- url: https://www.codesota.com/llm
  label: LLM Benchmarks 2026 Comparison
cards:
- id: c6505710-7d4f-4255-a147-9c61e06508fd
  type: mcq
  front: Which benchmark would you use to assess whether an LLM can generate correct Python functions
    with proper edge-case handling?
  back: HumanEval+ includes expanded test cases for edge cases; basic HumanEval can have false positives.
  choices:
  - key: a
    text: MMLU
    correct: false
  - key: b
    text: GSM8K
    correct: false
  - key: c
    text: HumanEval+
    correct: true
  - key: d
    text: ARC
    correct: false
- id: 0a0f5c16-84db-4923-b53f-bf01b768607c
  type: flip
  front: Why is MMLU becoming less useful for frontier model evaluation as of 2026?
  back: MMLU saturates above 90%, making it unable to discriminate between state-of-the-art models. Harder
    variants like MMLU-Pro are required for differentiation.
- id: f9bd37cc-1a25-413c-8734-8b49a287695d
  type: mcq
  front: GSM8K tests which aspect of LLM capability most directly?
  back: GSM8K grade-school math requires reasoning chains, not just knowledge recall.
  choices:
  - key: a
    text: Breadth of knowledge
    correct: false
  - key: b
    text: Multi-step mathematical reasoning
    correct: true
  - key: c
    text: Code synthesis
    correct: false
  - key: d
    text: Domain-specific reasoning
    correct: false
- id: e772b22c-0165-420f-8640-05269b3c3980
  type: flip
  front: What distinguishes ARC from MMLU in terms of what they measure?
  back: ARC specifically isolates reasoning over domain knowledge via grade-science QA, while MMLU tests
    breadth across subjects. ARC separates memorization from reasoning.
---

## Intuition
Standard benchmarks provide reproducible, objective measurement of LLM capabilities across core domains. They form the foundation of model comparison—codifying what it means for an LLM to be "better."

## Detail
Four pillars of standard evaluation:

**MMLU** (Massive Multitask Language Understanding): 16,000 multiple-choice questions across 57 academic subjects (STEM, humanities, social sciences). Measures breadth of knowledge. However, MMLU saturates above 90% as of 2026, limiting differentiation between frontier models. See MMLU-Pro for harder variant.

**GSM8K** (Grade School Math 8K): 8.5K grade-school arithmetic problems requiring multi-step reasoning. Tests mathematical capability and intermediate problem-solving. Discriminates well at mid-range but saturates at frontier.

**HumanEval**: 164 Python code generation tasks. Measures program synthesis. HumanEval+ extends this with edge-case tests, catching false positives in basic HumanEval. A model passing HumanEval may fail HumanEval+ on the same task.

**ARC** (AI2 Reasoning Challenge): 7.8K grade science questions (multiple choice). Tests reasoning over domain knowledge. Specifically separates memorization from reasoning capability.

## Common gotchas / interview framings
- **Saturation myth**: MMLU reaching 90%+ doesn't mean LLMs "understand" subjects—it indicates the benchmark needs harder variants (MMLU-Pro).
- **False negatives in code**: HumanEval pass rate can mask poor edge-case handling; always pair with HumanEval+.
- **Benchmark contamination**: Models may have been trained on test data; always verify train/test splits and use newer benchmarks for cutting-edge evaluation.
- **Breadth vs. depth**: MMLU tests breadth; for depth assessment pair with domain-specific benchmarks.
- **Interview framing**: "When would you use MMLU vs. GSM8K?" MMLU for general knowledge, GSM8K for mathematical reasoning, HumanEval for code generation.

## See also
- [[mmlu]]
- [[gsm8k]]
- [[humaneval]]
- [[arc-benchmark]]
- [[model-evaluation]]
- [[benchmark-saturation]]

## Sources
See frontmatter `sources:`.
