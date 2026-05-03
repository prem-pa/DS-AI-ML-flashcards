---
id: 4cfd9083-357b-47b3-9d3e-49fc7917982e
title: Graphical models and d-separation
track: data-scientist
topic: probability-bayesian-thinking
difficulty: 5
tags:
- causal-inference
- DAGs
- conditional-independence
- causal-structure
- backdoor-criterion
- do-calculus
aliases:
- d-separation criterion
- Bayesian networks
- directed graphical models
sources:
- url: https://en.wikipedia.org/wiki/Bayesian_network
  label: Bayesian Networks - Wikipedia
- url: https://machinelearningmastery.com/bayes-theorem-for-machine-learning/
  label: Bayes Theorem in ML Applications
cards:
- id: 83a30104-8a2a-4e41-8062-a09ae80b3fc9
  type: flip
  front: 'Draw and explain the three basic d-separation structures: chain, fork, and collider. Which ones
    are *blocked* by conditioning on the middle node?'
  back: '**Chain** ($A \to B \to C$): conditioning on $B$ blocks the path; $A \perp C | B$. **Fork** ($A
    \leftarrow B \to C$): conditioning on $B$ blocks; $A \perp C | B$. **Collider** ($A \to B \leftarrow
    C$): conditioning on $B$ *unblocks* the path; $A \not\perp C | B$ (they become dependent). The fork
    and chain are intuitive ($B$ is a "valve"); the collider is the gotcha: conditioning on a collider
    opens a spurious path.'
- id: 0a1154e0-dd1e-4bc7-b98d-87218df7a5e6
  type: mcq
  front: In a DAG $X \to Z \to Y$ with a confounder $U \to X, U \to Y$, which set $S$ would d-separate
    $X$ from $Y$?
  back: 'Paths from $X$ to $Y$ are: (1) $X \to Z \to Y$ and (2) $X \leftarrow U \to Y$ (confounding).
    Conditioning on $Z$ alone blocks path (1) but not (2). Conditioning on $U$ blocks path (2) and also
    blocks (1) because $U$ is on the path. So $U$ d-separates $X$ and $Y$.'
  choices:
  - key: a
    text: $S = \{\}$ (no conditioning)
    correct: false
  - key: b
    text: $S = \{Z\}$
    correct: false
  - key: c
    text: $S = \{U\}$
    correct: true
  - key: d
    text: $S = \{Z, U\}$
    correct: false
- id: e80f240d-0815-4e7d-bb59-fa7391d905cb
  type: flip
  front: Explain Simpson's Paradox using d-separation. Why does a confounder reverse the direction of
    association?
  back: Simpson's Paradox occurs when a confounder $C$ causes both the treatment $T$ and outcome $Y$,
    with opposite effects. The marginal association between $T$ and $Y$ can be opposite to the conditional
    association $P(Y|T,C)$ because the confounder induces a spurious association via the fork $T \leftarrow
    C \to Y$. Conditioning on $C$ blocks this path, revealing the true causal effect. Without conditioning,
    the data mixes populations with different baseline rates of $C$.
- id: 2af05226-6b03-4ab5-8df2-599f87d83a49
  type: mcq
  front: In the DAG $X \to Y \to Z$ with $U \to Y, U \to W$, and $W \to Z$, is $X \perp_d W | Y$?
  back: The path $X \to Y \leftarrow U \to W$ is a chain of a collider ($Y$ with parents $X$ and $U$)
    followed by a fork ($U \to W$). Conditioning on $Y$ unblocks the collider (since $Y$ is collided on),
    so the path $X \to Y \leftarrow U \to W$ is open. Thus $X \not\perp_d W | Y$.
  choices:
  - key: a
    text: Yes, $Y$ blocks all paths
    correct: false
  - key: b
    text: No, path $X \to Y \leftarrow U \to W$ is unblocked
    correct: true
  - key: c
    text: No, path $X \to Y \to Z \leftarrow W$ is unblocked
    correct: false
  - key: d
    text: Yes, $X$ and $W$ are not neighbors
    correct: false
- id: 013bf14c-4cb8-48c1-9bdb-c37064bbfbe9
  type: flip
  front: Explain the backdoor criterion and how it identifies confounders in a DAG. How does it relate
    to d-separation?
  back: 'The backdoor criterion (Pearl, 2000) identifies valid adjustment sets. For causal effect of $T$
    on $Y$, a set $S$ satisfies the backdoor criterion if: (1) no element of $S$ is a descendant of $T$,
    and (2) $S$ d-separates $T$ and $Y$ after removing all edges from $T$. Removing edges from $T$ leaves
    only "backdoor" (non-causal) paths through confounders. Any set satisfying backdoor can be adjusted
    for to identify the causal effect, e.g., via stratification or regression.'
---

## Intuition
Graphical models encode conditional independencies as a directed acyclic graph (DAG). A variable is conditionally independent of its non-descendants given its parents. D-separation ("directed separation") is a criterion to read off conditional independencies directly from the graph without computing probabilities. It answers: which variables become independent when you condition on a set?

## Detail
In a DAG, $X$ and $Y$ are d-separated by $Z$ (written $X \perp_d Y | Z$) if every path from $X$ to $Y$ is blocked by $Z$. A path is blocked if:
1. It contains a chain $A \to B \to C$ with $B \in Z$, or
2. It contains a fork $A \leftarrow B \rightarrow C$ with $B \in Z$, or
3. It contains a collider $A \rightarrow B \leftarrow C$ with $B \notin Z$ and no descendant of $B$ in $Z$

D-separation in a DAG is equivalent to conditional independence in the Markov factorization: $P(X_1,...,X_n) = \prod_i P(X_i | \text{Parents}_i)$. This enables reading causal structure from data and identifying confounders, mediators, and colliders without statistical testing.

## Common gotchas / interview framings
- Forgetting that d-separation is a *sufficient* condition for CI in a DAG, not necessary in general (though it is sufficient for Markov factorizations)
- Confusing the direction of arrows; $A \to B$ means $A$ is a parent of $B$, and $P(A,B) = P(B|A)P(A)$, not the reverse
- Failing to recognize colliders ($A \to C \leftarrow B$) as inducers of spurious dependence when conditioned on
- In causal inference, using observational data without verifying that causal assumptions (no unmeasured confounding) hold in the DAG
- Confusing association (correlation) with causation; a DAG encodes *assumed* causal structure, not proof

## See also
- [[d-separation]]
- [[bayesian_network]]
- [[causal_graph]]
- [[structural_causal_model]]
- [[backdoor_adjustment]]
- [[confounding]]

## Sources
See frontmatter `sources:`.
