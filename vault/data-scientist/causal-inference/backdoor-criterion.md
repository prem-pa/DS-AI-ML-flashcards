---
id: e80e1161-218e-4471-8b72-514d31c42428
title: Backdoor criterion
track: data-scientist
topic: causal-inference
difficulty: 5
tags:
- d-separation
- adjustment-sets
- confounding
- identification
- Pearl
aliases:
- backdoor paths
- confounding paths
- valid adjustment
sources:
- url: https://ftp.cs.ucla.edu/pub/stat_ser/r350.pdf
  label: 'Pearl: Causal Inference in Statistics Overview'
- url: https://miguelhernan.org/whatifbook
  label: 'Hernán & Robins: Causal Inference: What If'
cards:
- id: 963168d7-d3ed-4acb-b5fc-d34e394089e1
  type: flip
  front: What is a backdoor path?
  back: 'A non-causal association pathway from X to Y that starts with an arrow into X (i.e., X is the
    target of an edge). Example: X ← U → Y creates a backdoor path through confounder U. It is called
    ''backdoor'' because it enters X from the back.'
- id: ff0cf594-9064-423f-a6ea-dc17fa0b86ea
  type: flip
  front: What does it mean to 'block' a path in a DAG?
  back: 'A path is blocked by an adjustment set Z if at least one of these holds on the path: (1) Z contains
    a non-collider on the path, or (2) a collider on the path has no descendant in Z. Blocked paths do
    not transmit association.'
- id: bbe71fce-ee2f-4ea7-9744-5a2c11567d6c
  type: mcq
  front: 'In DAG: U → X → Y, U → Y, is adjusting for U sufficient to identify the causal effect of X on
    Y?'
  back: 'Yes. The backdoor path is U → X ← (no arrow back), which is actually U → Y. Wait, re-reading:
    U → X → Y is a direct path (not backdoor). The confounder path is U → X and U → Y. Adjusting for U
    blocks the confounding path U → Y that doesn''t go through X.'
  choices:
  - key: a
    text: Yes, U blocks the backdoor path
    correct: true
  - key: b
    text: No, U is a parent of X and blocks the forward path
    correct: false
  - key: c
    text: Yes, there is no collider on the backdoor path
    correct: false
  - key: d
    text: No, U is a mediator
    correct: false
- id: 6bef2eba-5e95-4a6c-bc4e-e534add93687
  type: flip
  front: Why can't we always 'adjust for everything' to estimate causal effects?
  back: Adjusting for descendants of X (mediators, colliders) can increase bias by blocking causal paths
    or opening spurious paths. The backdoor criterion specifies exactly which variables to adjust for.
---

## Intuition

The backdoor criterion is an algorithm (Pearl, 1993) to identify when and how to adjust for variables to eliminate confounding bias. A backdoor path is a non-causal path from treatment X to outcome Y that goes 'backward' through a confounder.

## Detail

**Backdoor path**: A path from X to Y that:
1. Starts with an arrow into X (X ← ...)
2. Does not follow the causal edge X → Y

Example: X ← U → Y is a backdoor path (U confounds X and Y).

**Backdoor criterion**: A set Z blocks all backdoor paths if:
1. No variable in Z is a descendant of X (to avoid cutting causal paths)
2. Z blocks every backdoor path from X to Y

**Blocking a path**: A path is blocked by a set Z if:
- Z contains a non-collider on the path, OR
- A collider on the path has no descendant in Z

**Algorithm**: 
```
1. Find all backdoor paths from X to Y
2. Identify a sufficient adjustment set Z that blocks all backdoor paths
3. Adjust for Z using regression, matching, or other methods
```

Example DAG with backdoor path:
```
U → X → Y
↑
U → X ← (direct)
Backdoor path: X ← U → Y
Fix: Adjust for U
```

## Common gotchas / interview framings

- "Just add all measured confounders"—may include descendants of X (mediators) or colliders
- "The regression coefficient changed when I added controls"—sign of collider or mediator conditioning
- Interview: "What is the minimal sufficient adjustment set for this DAG?" or "Will adjusting for Z work?"

## See also
- [[dags-directed-acyclic-graphs-and-causal-assumptions]]
- [[confounders-colliders-mediators]]
- [[frontdoor-criterion]]

## Sources
See frontmatter `sources:`.
