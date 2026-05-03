---
id: 7da33e7d-f524-4f45-b636-45aaecd86c5e
title: PAC learning framework
track: research-leaning
topic: optimization-learning-theory
difficulty: 5
tags:
- learning-theory
- sample-complexity
- probably-approximately-correct
- PAC-bounds
- hypothesis-class
- worst-case-analysis
aliases:
- PAC learning
- PAC theory
- distribution-free learning
- sample-efficient learning
- learning complexity
sources:
- url: https://en.wikipedia.org/wiki/Vapnik%E2%80%93Chervonenkis_dimension
  label: VC Dimension and PAC Learning - Wikipedia
cards:
- id: 26c20049-6d60-4543-85b5-778ba6e4c593
  type: flip
  front: Define PAC learning. What are the roles of $\epsilon$ and $\delta$?
  back: 'A hypothesis class $\mathcal{H}$ is PAC-learnable if an algorithm can find $h \in \mathcal{H}$
    such that, with probability $\ge 1-\delta$, the generalization loss is $\le \epsilon$ above the best
    in class:

    $$\mathbb{P}[L_D(h) \le \min_{h^*} L_D(h^*) + \epsilon] \ge 1-\delta$$

    using a number of samples polynomial in $1/\epsilon$, $1/\delta$, and class complexity. Here: $\epsilon$
    = **accuracy** (how close to optimal), $\delta$ = **confidence failure probability** (how often we
    might fail). PAC = "Probably (with prob $1-\delta$) Approximately (within error $\epsilon$) Correct."'
- id: 250d0f7a-01be-4596-ae5a-9b0f61531a56
  type: mcq
  front: For a finite hypothesis class of size $M$, what is the PAC sample complexity?
  back: 'Union bound over $M$ hypotheses: probability that empirical risk minimizer fails is $\le M \cdot
    e^{-2n\epsilon^2}$. Setting this $\le \delta$ and solving: $n \ge \frac{\log(M/\delta)}{2\epsilon^2}
    = O(\log(M/\delta)/\epsilon^2)$. Sample complexity grows *logarithmically* in class size (due to union
    bound), not linearly. This is why finite classes are easier to learn than infinite classes (VC dimension).'
  choices:
  - key: a
    text: $O(M/\epsilon)$
    correct: false
  - key: b
    text: $O(\log(M/\delta)/\epsilon^2)$
    correct: true
  - key: c
    text: $O(M \log(M/\delta)/\epsilon^2)$
    correct: false
  - key: d
    text: $O(1/\epsilon^2)$ (independent of $M$)
    correct: false
- id: b0f9100d-de1f-4dca-9b86-c5957df88457
  type: flip
  front: Relate PAC learning to VC dimension. How does sample complexity scale for an infinite class?
  back: '**Blumer-Ehrenfeucht-Haussler-Warmuth theorem**: For an infinite hypothesis class with VC dimension
    $d$, the PAC sample complexity is:

    $$n = \tilde{O}\left(\frac{d}{\epsilon^2} + \frac{\log(1/\delta)}{\epsilon^2}\right)$$

    This is analogous to the finite case but replaces $\log M$ with VC dimension $d$. The VC dimension
    measures effective size: a class with VC dim $d$ behaves like a finite class of size $\sim 2^d$ for
    sample complexity purposes. This result is tight: $\Omega(d/\epsilon^2)$ lower bounds also hold for
    most natural classes.'
- id: 961e98bf-0517-499d-893f-1fe4d514306d
  type: flip
  front: Explain the difference between PAC learning and agnostic PAC learning. How does sample complexity
    change?
  back: '**Realizable PAC**: Assumes the target function $f^* \in \mathcal{H}$ (the class is ''correct'').
    Sample complexity is $\tilde{O}(d/\epsilon^2)$.


    **Agnostic PAC**: Allows $f^* \notin \mathcal{H}$ (the class is ''wrong''). Algorithm must find $h
    \in \mathcal{H}$ close to the best-in-class $h^*$. Sample complexity becomes $\tilde{O}(d/\epsilon^2)$
    as well, but with worse constants due to noise and bias (approximation error $\approx \min_{h \in
    \mathcal{H}} L_D(h) - L_D(f^*)$ becomes irreducible). Agnostic learning is more realistic but harder.'
- id: 52db9214-255d-4aa9-b0a8-f735c5e2ad4f
  type: flip
  front: What is the distinction between PAC sample complexity and computational complexity? Give an example
    where they differ.
  back: '**Sample complexity**: Number of examples needed. **Computational complexity**: Time/resources
    to process examples and find a good hypothesis. For some classes, sample complexity is polynomial
    but finding a hypothesis is NP-hard. Classic example: learning **DNF formulas** (disjunctive normal
    form). Any DNF on $n$ variables has VC dimension $\Omega(n)$, requiring $\tilde{O}(n/\epsilon^2)$
    samples (polynomial PAC-learnable). However, finding a consistent DNF is NP-complete, so no polynomial-time
    algorithm is known. This separation shows PAC-learnability doesn''t guarantee efficient learning.'
---

## Intuition
PAC (Probably Approximately Correct) learning formalizes the question: given a hypothesis class, how many i.i.d. samples do we need to guarantee that a learned hypothesis is nearly optimal with high confidence? This is the foundation of modern learning theory and provides worst-case sample complexity bounds.

## Detail
**PAC Learning Definition**: A hypothesis class $\mathcal{H}$ is PAC-learnable if there exists an algorithm and a polynomial function $\text{poly}(\cdot)$ such that for any $\epsilon, \delta > 0$ (error and confidence parameters) and any distribution $D$:
$$n = \text{poly}(1/\epsilon, 1/\delta, \text{complexity}(\mathcal{H})) \text{ samples suffice}$$
to find $h \in \mathcal{H}$ with $\mathbb{P}[L_D(h) \le \min_{h^* \in \mathcal{H}} L_D(h^*) + \epsilon] \ge 1-\delta$.

**Key Results**:
1. **Finite class**: If $|\mathcal{H}| = M$, then $n = O(\log(M/\delta)/\epsilon^2)$ samples suffice (empirical risk minimization is PAC).
2. **Infinite class**: If $\text{VC}(\mathcal{H}) = d$, then $n = \tilde{O}(d/\epsilon^2 + \log(1/\delta)/\epsilon^2)$ samples (polynomial in VC dimension).
3. **Lower bounds**: For many natural classes, $\Omega(d/\epsilon^2)$ samples are necessary (tight up to logs).

**Agnostic PAC**: Extension allowing $\min_{h^* \in \mathcal{H}} L_D(h^*) > 0$ (target not in class). Sample complexity becomes $\tilde{O}(d/\epsilon^2)$ for VC dimension $d$.

**Computational Complexity**: PAC learning is about sample complexity, not computational complexity. An algorithm might require exponential time even if sample complexity is polynomial—this distinction matters for hardness results (e.g., learning DNFs is PAC-hard but sample-efficiently learnable).

## Common gotchas / interview framings
- **Distribution-free**: PAC bounds hold for any distribution $D$; thus they are worst-case and often loose
- **Depends on class complexity, not data dimension**: Sample complexity depends on $\text{VC}(\mathcal{H})$, not directly on data dimensionality $d$
- **Agnostic vs. realizability**: Agnostic PAC (target not in class) requires more samples; the gap is the approximation error
- **Not all PAC-learnable classes are efficiently learnable**: Computational hardness (e.g., DNF) can prevent polynomial-time PAC algorithms
- **Modern deep learning is mostly outside PAC scope**: Neural networks are complex (high VC dim), yet modern practices (SGD, regularization, early stopping) achieve good generalization in ways PAC theory doesn't fully explain

## See also
- [[pac-learning]]
- [[sample-complexity]]
- [[mistake-bound-learning]]
- [[agnostic-pac]]
- [[vc-dimension-pac]]

## Sources
See frontmatter `sources:`.
