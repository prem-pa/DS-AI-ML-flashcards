---
id: a3cb0477-d0e5-420b-9fe9-4a221362e715
title: Reasoning benchmarks (MATH, Codeforces)
track: ai-llm-engineer
topic: evaluation-benchmarking
difficulty: 3
tags:
- reasoning
- mathematics
- competitive-programming
- hard-tasks
- discriminative
aliases:
- MATH-500
- competitive-code
- step-by-step reasoning
- AIME
- GPQA
sources:
- url: https://lmmarketcap.com/benchmarks
  label: AI Benchmarks 2026
- url: https://www.codesota.com/llm
  label: LLM Benchmarks 2026
- url: https://tokenmix.ai/blog/llm-leaderboard-2026
  label: LLM Leaderboard 2026 Guide
cards:
- id: d477ce71-ec66-4571-a00f-8d19423df574
  type: flip
  front: Why does MATH-500 better discriminate frontier models than MMLU, even though both test knowledge?
  back: MATH-500 requires step-by-step derivation (reasoning), not recall. Models can't memorize paths;
    they must construct proofs. MMLU is pure knowledge recall, which saturates.
- id: 87e2b51e-49fc-408e-b0d1-c6ba0cf0a3bc
  type: mcq
  front: GPQA-Diamond currently achieves which performance range for frontier models in 2026?
  back: Claude Mythos leads at 94.6%; frontier models range 80–94%. GPQA-Diamond is one of the best discriminators
    at the frontier.
  choices:
  - key: a
    text: 50–60%
    correct: false
  - key: b
    text: 70–80%
    correct: false
  - key: c
    text: 80–94%
    correct: true
  - key: d
    text: 95%+
    correct: false
- id: c3c58120-99dd-4683-aa63-69ec49da4dfa
  type: flip
  front: What makes Codeforces more challenging for LLMs than HumanEval?
  back: HumanEval is function implementation (~164 tasks, 95%+ frontier). Codeforces requires algorithmic
    insight, design under constraints, and handling novel problem types. Can't be solved by pattern matching
    or templates.
- id: 47a4a928-44c2-4d71-b7a6-447f8b3be4eb
  type: mcq
  front: When evaluating an LLM's reasoning capability for a research paper, which factor has the largest
    impact on MATH/Codeforces scores?
  back: Reasoning tasks benefit heavily from sampling multiple CoT paths (best-of-N); single-pass evaluation
    significantly undershoots true capability.
  choices:
  - key: a
    text: Model size
    correct: false
  - key: b
    text: Chain-of-thought sampling and temperature
    correct: true
  - key: c
    text: Training data size
    correct: false
  - key: d
    text: Context window
    correct: false
---

## Intuition
Reasoning benchmarks isolate step-by-step problem-solving capability independent of broad knowledge. MATH, Codeforces, AIME, and GPQA-Diamond are frontier discriminators—models saturate knowledge benchmarks but diverge sharply on reasoning tasks. They identify models that can *think*, not just memorize.

## Detail
**MATH-500**: 500 pre-calculus/calculus competition math problems requiring multi-step derivations. Tests:
- Algebraic manipulation
- Logical proof construction
- Ability to backtrack when stuck
- Resistance to shortcuts (can't memorize answers; derivation required)

As of 2026: frontier models score 70–85%; mid-range models ~20–40%. Massive gap indicates strong reasoning differentiation.

**Codeforces**: Real competitive programming problems from Codeforces (online judge platform). Tests:
- Algorithm design
- Code correctness under time pressure
- Edge-case identification
- Debugging non-working code

Harder than HumanEval (which is ~95% for frontier models). Codeforces problems require insight; can't brute-force search space.

**GPQA / GPQA-Diamond**: Graduate-level Google-generated multiple-choice questions (biology, physics, chemistry). GPQA-Diamond is the hardest tier. Scores:
- Random chance: ~25%
- Frontier models: 80–94%
- Mid-range: 40–60%

One of the best discriminators at frontier in 2026 (Claude Mythos Preview 94.6%).

## Common gotchas / interview framings
- **Reasoning ≠ knowledge**: A model can know calculus without deriving solutions; MATH isolates derivation.
- **Saturation at frontier**: MATH doesn't saturate below 85% but has few models there; useful for cutting-edge but limited sample.
- **Sampling sensitivity**: Chain-of-thought (CoT) sampling heavily impacts MATH/Codeforces; single-pass evaluation undershoots true capability.
- **Interview framing**: "Why doesn't HumanEval saturate like MMLU, even at frontier?" Code generation is easier than Codeforces competitive programming; HumanEval lacks the algorithmic insight required.

## See also
- [[math-dataset]]
- [[codeforces]]
- [[step-by-step-reasoning]]
- [[hard-math-benchmarks]]
- [[gpqa-diamond]]
- [[reasoning-bottleneck]]

## Sources
See frontmatter `sources:`.
