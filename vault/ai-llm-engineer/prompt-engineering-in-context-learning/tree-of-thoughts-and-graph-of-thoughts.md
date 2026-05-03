---
id: 2bf7018a-7ba2-4587-afc8-eff3805844c0
title: Tree-of-Thoughts and graph-of-thoughts
track: ai-llm-engineer
topic: prompt-engineering-in-context-learning
difficulty: 5
tags:
- tree-of-thoughts
- reasoning
- search
- exploration
- graph-of-thoughts
- advanced-prompting
aliases:
- ToT
- GoT
- branching reasoning
- multi-path exploration
sources:
- url: https://www.promptingguide.ai/techniques/tot
  label: 'Prompting Guide: Tree of Thoughts'
- url: https://calmops.com/ai/prompt-engineering-patterns-cot-react-tot/
  label: 'Prompt Engineering Patterns: CoT, ReAct, ToT'
- url: https://capabl.in/blog/advanced-prompt-engineering-mastering-tree-of-thoughts-react-and-role-based-prompts-for-superior-llm-performance
  label: 'Advanced Prompt Engineering: ToT, ReAct, Role-Based Prompts'
cards:
- id: af5f55aa-9409-4d21-ba5b-ca2007fc9dbb
  type: flip
  front: How does Tree-of-Thoughts (ToT) differ from Chain-of-Thought (CoT) in structure and capability?
  back: 'CoT follows a single linear reasoning path. ToT explores multiple branching paths in parallel:
    (1) generate 2-5 candidate thoughts at each step, (2) evaluate their quality, (3) prune weak branches,
    (4) backtrack on failure. This enables robust exploration of complex problems where CoT gets stuck.'
- id: 5cf19fb0-2b15-4bee-8458-c9c29643c31e
  type: flip
  front: Describe the four main steps of a Tree-of-Thoughts algorithm.
  back: 1) **Decompose:** Identify decision points in the problem. 2) **Generate:** Create 2-5 candidate
    thoughts (reasoning steps) at each node. 3) **Evaluate:** Score thoughts for feasibility and progress
    using self-evaluation or auxiliary LLM. 4) **Select & backtrack:** Expand high-scoring branches; prune
    low-scoring ones; backtrack if stuck.
- id: 026b7756-29e0-4317-94fd-5e8114d4d650
  type: flip
  front: What is the cost-accuracy tradeoff for Tree-of-Thoughts compared to Chain-of-Thought?
  back: 'ToT requires 3-10× more tokens (multiple thought branches) and sequential evaluation rounds (higher
    latency). Accuracy gain is task-dependent: 5-25% on hard reasoning (math, logic puzzles); minimal
    on easy tasks (classification, summarization). Best for batch/analytical systems, not real-time latency-critical
    applications.'
- id: cf907f0a-735d-45e2-81af-a5feaedb9872
  type: mcq
  front: When should you choose Tree-of-Thoughts over Chain-of-Thought?
  back: ToT excels on hard reasoning (5-25% accuracy gain) but is expensive (3-10× tokens, sequential
    rounds). It's uneconomical for easy tasks or latency-critical systems. Save ToT for batch/analytical
    work where accuracy outweighs cost.
  choices:
  - key: a
    text: For all tasks, because more exploration is always better
    correct: false
  - key: b
    text: For hard reasoning tasks (math, logic puzzles, planning) where single-path reasoning fails;
      latency/cost permits
    correct: true
  - key: c
    text: For simple classification tasks where accuracy is already >90%
    correct: false
  - key: d
    text: For real-time systems where response latency must be <100ms
    correct: false
- id: ab77cfd0-6f73-419a-8b07-8105092dc722
  type: mcq
  front: In a ToT implementation, what is the role of the evaluation/scoring function?
  back: Evaluation quality directly impacts pruning decisions and token efficiency. A weak evaluator may
    discard promising branches or expand dead-ends, wasting cost and missing solutions. The evaluator
    is the 'intelligence' guiding the search.
  choices:
  - key: a
    text: It generates new thoughts at each node
    correct: false
  - key: b
    text: It decides which branches to expand and which to prune; poor evaluation leads to wasted token
      spend and suboptimal solutions
    correct: true
  - key: c
    text: It formats the final answer in JSON
    correct: false
  - key: d
    text: It verifies that the grammar is correct
    correct: false
---

## Intuition

Tree-of-Thoughts (ToT) extends [[chain-of-thought-prompting]] by exploring multiple reasoning branches in parallel, evaluating partial solutions, and pruning unpromising paths. Where CoT follows a single linear path, ToT treats reasoning as a search problem: generate candidate "thoughts" (intermediate steps), evaluate their promise, and backtrack if stuck.

Graph-of-Thoughts (GoT) generalizes further, allowing arbitrary connections between nodes (thoughts) rather than strictly hierarchical tree structures. This enables representation of dependencies and shortcuts in complex reasoning.

## Detail

**Tree-of-Thoughts framework:**

1. **Decompose:** Break problem into decision points where multiple choices exist.
2. **Generate:** At each node, generate 2-5 candidate "thoughts" (next reasoning steps).
3. **Evaluate:** Score each thought for feasibility and progress toward goal (using self-evaluation or auxiliary evaluator LLM).
4. **Select:** Prune low-scoring branches; expand high-scoring ones via depth-first or breadth-first search.
5. **Backtrack:** If a path fails, explore alternatives.

**Example (Sudoku solving):**
```
                        [Initial puzzle]
                    /        |         \
         [Try 1,1=1]   [Try 1,1=2]   [Try 1,1=3]
            /              |  \            /  \
     [Try 1,2=4]    [deadlock]   ...  [Try 1,2=5] ...
      /       \                              \
   [valid]   [conflict]                   [continue]
```

**Cost-accuracy tradeoff:**
- **Token cost:** 3-10× higher than CoT (multiple branch exploration).
- **Latency:** Requires sequential evaluation rounds (inherently slower).
- **Accuracy gain:** 5-25% on hard reasoning tasks (math, puzzles) where single-path CoT gets stuck.

**Graph-of-Thoughts:** Enables non-tree structures (merging thoughts, parallel dependencies). Useful for tasks requiring "global" reasoning patterns (constraint satisfaction, planning) vs. sequential decomposition.

## Common gotchas / interview framings

- **Expensive for online systems:** ToT requires multiple model calls per query; best for batch/analytical tasks, not real-time latency-critical systems.
- **Evaluation function is critical:** Pruning decisions depend on thought quality scoring; weak evaluators lead to backtracking errors and wasted tokens.
- **Hyperparameter tuning:** Branching factor (thoughts per node), depth limit, and search strategy (DFS vs BFS) heavily affect cost/quality.
- **Hallucination risk:** Intermediate thoughts can be plausible-sounding but incorrect; combine with verifiable checkpoints (e.g., checking math).
- **Problem-dependent:** Helps hard reasoning (math, logic puzzles); minimal gain on classification or summarization tasks.

## See also
- [[chain-of-thought-prompting]]
- [[self-consistency-and-majority-voting]]
- [[why-icl-works-gradient-descent-interpretation]]

## Sources
See frontmatter `sources:`.
