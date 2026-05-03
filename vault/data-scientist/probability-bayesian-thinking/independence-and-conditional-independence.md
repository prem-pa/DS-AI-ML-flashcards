---
id: 20452046-11e7-46e4-959d-b829b7fa499a
title: Independence and conditional independence
track: data-scientist
topic: probability-bayesian-thinking
difficulty: 3
tags:
- d-separation
- causal-graphing
- factorization
- dimensionality-reduction
- graphical-structure
- inference-complexity
aliases:
- statistical independence
- conditional independence
- CI
sources:
- url: https://en.wikipedia.org/wiki/Conditional_independence
  label: Conditional Independence - Wikipedia
- url: https://bayesball.github.io/BOOK/bayesian-hierarchical-modeling.html
  label: Probability and Bayesian Modeling - Chapter on CI
cards:
- id: 3373a2b6-9d9c-484c-9a30-c2c64c2aadcc
  type: flip
  front: In a Naive Bayes classifier $P(Y|X_1,...,X_n) \propto P(Y)\prod_i P(X_i|Y)$, what independence
    assumption is encoded? Why is it called "naive"?
  back: 'The assumption is that features $X_i$ are conditionally independent given the label $Y$: $X_i
    \perp X_j | Y$ for $i \neq j$. This is "naive" because in real data, features are almost always conditionally
    dependent (e.g., correlations in text or images). Despite this violation, Naive Bayes often works
    well because conditional independence reduces parameter count from exponential to linear and provides
    regularization.'
- id: 132299df-e115-4ff7-b2ed-c02ce2d88bfc
  type: mcq
  front: Two students' exam scores $S_1$ and $S_2$ are marginally correlated. A teacher discovers both
    studied together with a tutor $T$. Now, what is most likely true?
  back: '$T$ is a common cause of both $S_1$ and $S_2$. Once you condition on $T$ (tutor quality), the
    variation in $S_1$ is explained independently, making them conditionally independent. This is d-separation
    in a DAG: the path $S_1 \leftarrow T \rightarrow S_2$ is blocked by conditioning on $T$.'
  choices:
  - key: a
    text: $S_1 \perp S_2 | T$ (conditionally independent given tutor quality)
    correct: true
  - key: b
    text: $S_1$ and $S_2$ remain dependent even given $T$
    correct: false
  - key: c
    text: $T$ is independent of both $S_1$ and $S_2$
    correct: false
  - key: d
    text: The marginal correlation is fake; they're always independent
    correct: false
- id: 96adf23f-90f3-47b6-a28f-abd067985d29
  type: flip
  front: 'Explain the collider paradox: why does conditioning on a collider induce dependence between
    its parents?'
  back: 'Suppose $X$ and $Y$ are independent, but both cause $Z$ (a collider: $X \rightarrow Z \leftarrow
    Y$). Marginally, $X \perp Y$. But if you condition on $Z=1$ (e.g., $Z$ is high), and $X$ is low, then
    $Y$ must be high to explain $Z$. This backward reasoning induces negative dependence: $P(Y|X,Z) \neq
    P(Y|Z)$ when $X$ and $Y$ are parents of $Z$.'
- id: f0be3966-3caf-4856-abdf-7df2cbbaf232
  type: mcq
  front: In Gibbs sampling for a Bayesian network, we update $X_i$ by sampling from $P(X_i | X_{\text{Markov
    blanket}})$. What independence property makes this efficient?
  back: The Markov blanket of $X_i$ consists of its parents, children, and co-parents (other parents of
    its children). By conditional independence in the graphical model, $X_i \perp (\text{all other variables})
    | \text{Markov blanket}$. This means you only need the Markov blanket to update $X_i$, enabling local
    Gibbs sampling.
  choices:
  - key: a
    text: Each variable is independent of the full posterior given its Markov blanket
    correct: true
  - key: b
    text: Variables are marginally independent
    correct: false
  - key: c
    text: The likelihood factorizes across variables
    correct: false
  - key: d
    text: All variables are conditionally independent
    correct: false
- id: 12c90a80-ac91-46d2-b34b-0d3f6b190d59
  type: flip
  front: Suppose you want to factorize $P(A,B,C,D,E)$ to minimize computational cost. You know that $A
    \perp B, D | C$ and $B \perp E | A$. Write a factorization exploiting these CIs.
  back: One valid factorization is $P(A,B,C,D,E) = P(C)P(A|C)P(B|C)P(D|C)P(E|A)$ or $P(C)P(D|C)P(B|C)P(A|C)P(E|A)$.
    The CI $A \perp B,D|C$ allows us to separate them when $C$ is known, and $B \perp E|A$ means $E$ depends
    only on $A$, not $B$. This reduces parameters and enables faster inference via belief propagation.
---

## Intuition
Independence means one variable's distribution is unaffected by another: $X \perp Y \iff P(X,Y) = P(X)P(Y)$. Conditional independence is more subtle and more common in Bayesian networks: $X \perp Y | Z$ means $X$ and $Y$ become independent once you know $Z$. This is the engine of dimensionality reduction—it allows factorization of complex joint distributions into products of simpler terms.

## Detail
**Marginal Independence:** $X \perp Y$ holds iff $P(X|Y)=P(X)$ and $P(Y|X)=P(Y)$. In graphical models, this is rare unless there's no path between them.

**Conditional Independence:** $X \perp Y | Z$ holds iff $P(X|Y,Z)=P(X|Z)$. Intuitively, once you observe $Z$, information about $Y$ is irrelevant for $X$. In a DAG, this occurs when $Z$ "blocks" all paths from $Y$ to $X$ (d-separation). 

Key insight: **$X$ and $Y$ can be marginally dependent but conditionally independent given $Z$** (e.g., both caused by $Z$—a collider), or **marginally independent but conditionally dependent** (e.g., $Z$ is a common cause). This asymmetry is why conditioning matters.

## Common gotchas / interview framings
- Confusing marginal and conditional independence; they are distinct concepts
- Assuming conditional independence without causal/graphical justification
- Forgetting that conditioning on a collider (child of two parents in a DAG) induces dependence between its parents
- Using independence assumptions to drop terms from a likelihood without verifying the graphical structure
- In MCMC, mistaking "variables that don't interact in the likelihood" for independent, when they may be dependent via the prior or posterior

## See also
- [[conditional_independence]]
- [[d-separation]]
- [[graphical_models]]
- [[markov_blanket]]
- [[causal_inference]]
- [[directed_acyclic_graph]]

## Sources
See frontmatter `sources:`.
