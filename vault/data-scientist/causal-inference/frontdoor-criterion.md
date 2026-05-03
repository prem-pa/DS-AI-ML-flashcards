---
id: 38adef61-e820-4534-8ee8-6223aefbd973
title: Frontdoor criterion
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- unmeasured-confounding
- mediator
- identification
- Pearl
- mediation
aliases:
- frontdoor paths
- frontdoor formula
- unobserved confounder
sources:
- url: https://ftp.cs.ucla.edu/pub/stat_ser/r350.pdf
  label: 'Pearl: Causal Inference in Statistics Overview'
- url: https://www.amazon.com/Causality-Reasoning-Inference-Judea-Pearl/dp/052189560X
  label: 'Pearl: Causality: Models, Reasoning and Inference'
cards:
- id: 1af98136-b792-4857-89ef-68a33aec4aff
  type: flip
  front: What is the frontdoor criterion and when does it apply?
  back: An identification strategy for causal effects when there is unmeasured confounding of X and Y,
    but all causal effect flows through an observed mediator M, and M has no unmeasured confounder affecting
    its path to Y. Frontdoor 'goes forward' through M, avoiding the unmeasured confounder.
- id: f0b629bc-4470-4b7a-b551-92a437c7d384
  type: flip
  front: In a frontdoor setup, what role does the mediator M play?
  back: 'M is the sole intermediary of the causal effect: all paths from X to Y must pass through M. M
    acts as a ''bottleneck'' that allows us to decompose the effect of X on Y into identifiable parts
    despite unmeasured confounding.'
- id: 4647296f-83c3-41dc-9d2b-15cea5b888b0
  type: mcq
  front: In the frontdoor DAG with unmeasured U → X, U → Y, X → M → Y, M → Y, can we identify the causal
    effect of X on Y?
  back: Yes, if M fully mediates and M has no unmeasured confounder affecting M → Y. The frontdoor formula
    leverages the causal pathway through M to identify the effect despite the unmeasured U.
  choices:
  - key: a
    text: Yes, using the frontdoor formula
    correct: true
  - key: b
    text: No, because U confounds both X and Y
    correct: false
  - key: c
    text: Only if M is a collider
    correct: false
  - key: d
    text: Yes, by adjusting for U
    correct: false
- id: 8151d376-5d75-4897-833b-979b6102e795
  type: flip
  front: Why is the frontdoor criterion rarely used in practice?
  back: 'Its conditions are strict: (1) all effect must flow through M, (2) no unmeasured confounder of
    M → Y, and (3) no alternative paths from X to Y. Most real problems violate at least one assumption.'
---

## Intuition

The frontdoor criterion (Pearl, 1993) solves a tough problem: identifying causal effects when there is an unmeasured confounder, *but* the entire effect flows through an observed mediator. It is rarer than backdoor adjustment but crucial when unobserved confounding is unavoidable.

## Detail

**Frontdoor condition**: The causal effect of X on Y is identified if:
1. X has no unmeasured confounders (or unmeasured confounders are blocked by other variables)
2. M (mediator) is on all causal paths from X to Y (X → M → Y)
3. All paths from M to Y that do not go through X are blocked by some variables (no backdoor from M to Y through unmeasured U)

DAG:
```
U (unmeasured) ← (confounds X and Y)
  ↓
  X → M → Y
      ↑
    No unmeasured confounder of M → Y
```

**Frontdoor formula** (simplified):
```
P(Y|do(X)) = Σ_m P(M|X) × Σ_x' P(Y|M,X') × P(X')
```
It uses the *path* through M to infer the total effect despite unmeasured U.

## Common gotchas / interview framings

- "Frontdoor is rare in practice"—most unmeasured confounding violates all three conditions
- "Does M fully mediate the effect?"—frontdoor requires M to capture *all* pathways from X to Y
- Interview: "Can you identify the causal effect if there's an unmeasured confounder U?" → Teach frontdoor criterion in response

## See also
- [[dags-directed-acyclic-graphs-and-causal-assumptions]]
- [[backdoor-criterion]]
- [[mediation-analysis]]

## Sources
See frontmatter `sources:`.
