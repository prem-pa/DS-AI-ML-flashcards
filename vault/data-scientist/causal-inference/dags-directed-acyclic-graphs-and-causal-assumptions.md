---
id: 05e2cb91-65ed-4374-b5e4-d2058c34c407
title: DAGs (Directed Acyclic Graphs) and causal assumptions
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- graphical-models
- causal-structure
- d-separation
- identification
- Pearl
aliases:
- directed acyclic graphs
- causal graphs
- structural graphs
sources:
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
- url: https://ftp.cs.ucla.edu/pub/stat_ser/r350.pdf
  label: 'Pearl: Causal Inference in Statistics Overview'
- url: https://www.amazon.com/Causality-Reasoning-Inference-Judea-Pearl/dp/052189560X
  label: 'Pearl: Causality: Models, Reasoning and Inference'
cards:
- id: f6f4f82d-9b1b-4049-953a-717c0fd62602
  type: flip
  front: What does a directed edge in a DAG mean?
  back: 'A direct causal effect: the source variable has a direct causal impact on the target, all else
    equal. An edge X → Y means X affects Y; absence of X → Z means no direct effect.'
- id: 876ae850-ae08-4fe4-ad41-4bddadc9102c
  type: flip
  front: Why must a causal DAG be acyclic?
  back: 'Causality has temporal order: causes precede effects. Cycles would mean X causes Y and Y causes
    X, violating temporal precedence. Acyclicity ensures we can assign causal direction.'
- id: 94b04848-5cf3-40b1-950e-b98a92336362
  type: mcq
  front: In the DAG U → X → Y with U unmeasured, which adjustment set identifies the causal effect of
    X on Y?
  back: Adjust for nothing. U is a confounder (parent of both X and Y), but it's unmeasured. However,
    there's no backdoor path through U (since U → X → Y is the frontdoor). Adjusting would bias estimates.
  choices:
  - key: a
    text: Adjust for nothing (empty set)
    correct: true
  - key: b
    text: Adjust for U
    correct: false
  - key: c
    text: Adjust for Y
    correct: false
  - key: d
    text: Adjust for X
    correct: false
- id: c08b97a8-8c19-4992-975c-911ad8ea241d
  type: flip
  front: What is the causal Markov condition in a DAG?
  back: Each variable is independent of its non-descendants conditional on its parents. Parents 'screen
    off' all other influences on that variable (in the causal model).
---

## Intuition

DAGs visually encode causal relationships as nodes (variables) and directed edges (causal arrows). The absence of an edge encodes a causal assumption: no direct effect. Crucially, DAGs must be acyclic—no feedback loops—to represent causal order.

## Detail

A DAG formalizes three core causal assumptions:

1. **Causal completeness**: All relevant confounders appear in the graph
2. **No unmeasured confounding**: Unobserved variables causing multiple nodes are absent (or their effects are captured)
3. **Causal markov condition**: Each node is independent of its non-descendants given its parents

DAGs enable:
- **d-separation rules**: Determine when variables are conditionally independent given adjustment sets
- **Identification**: Which causal effects are theoretically estimable from observational data
- **Backdoor/Frontdoor criteria**: Algorithmic tests for valid adjustment sets

Example DAG:
```
U (unmeasured confounder)
↓
X → Y
↑
M (mediator)
```

## Common gotchas / interview framings

- "I added control variables but got worse estimates"—you may have conditioned on a collider or mediator (bad)
- "Which variables should I control for?" requires reading the DAG, not folklore (e.g., "always control for demographics" is naive)
- DAGs encode causal assumptions; they are not learned from data alone—they come from domain knowledge
- Interview: "Draw the DAG for a pricing experiment" or "What assumption does this DAG violate?"

## See also
- [[confounders-colliders-mediators]]
- [[backdoor-criterion]]
- [[frontdoor-criterion]]
- [[do-calculus]]

## Sources
See frontmatter `sources:`.
