---
id: f7c9f030-cf9e-4880-8093-a647818f7380
title: Second-order optimization and Hessian-free methods
track: research-leaning
topic: optimization-learning-theory
difficulty: 5
tags:
- Newton-methods
- natural-gradient
- second-order-information
- Hessian-free
- BFGS-quasi-Newton
- Fisher-information
aliases:
- Newton optimization
- natural gradient descent
- second-order derivatives
- quasi-Newton methods
- Gauss-Newton
sources:
- url: https://arxiv.org/pdf/2509.18396
  label: 'Development of Deep Learning Optimizers: Approaches'
- url: https://keras.io/api/optimizers/
  label: Keras Optimizers Overview
cards:
- id: 9f0318f6-e5b4-4db7-b97c-5e86d2fb89b2
  type: flip
  front: State Newton's method update rule. What is the convergence rate near a minimum, and why is it
    fast?
  back: 'Newton''s method: $w_{t+1} = w_t - H_t^{-1} \nabla f(w_t)$ where $H_t = \nabla^2 f(w_t)$ is the
    Hessian. Convergence is **quadratic**: $\|w_{t+1} - w^*\| \lesssim \|w_t - w^*\|^2$. The doubling
    of digits (quadratic rate) occurs because the Newton step uses local curvature to perfectly scale
    the update direction; second-order Taylor expansion is exact (up to third-order terms) near minimum.
    This requires far fewer iterations than gradient descent ($O(\log \log \epsilon^{-1})$ vs. $O(\log
    \epsilon^{-1})$).'
- id: 770f76ad-0156-41ae-9d91-40e06a59a9fd
  type: mcq
  front: Why is Newton's method rarely used for training neural networks with billions of parameters,
    despite its superior convergence rate?
  back: The Hessian is a $d \times d$ matrix; storing it requires $O(d^2)$ memory (200 TB for a 10B parameter
    model!) and inverting costs $O(d^3)$ operations. Even with approximations like BFGS (which approximates
    with low-rank updates), the per-iteration cost can exceed the savings from fewer iterations. Modern
    Hessian-free methods (KFAC, conjugate gradient) try to recover benefits by using only implicit Hessian-vector
    products, but these are still rarely deployed at scale.
  choices:
  - key: a
    text: Neural networks are not differentiable; Newton's method requires second derivatives
    correct: false
  - key: b
    text: Computing and inverting the Hessian requires $O(d^2)$ memory and $O(d^3)$ time per iteration,
      which is prohibitive for $d \gg 10^6$
    correct: true
  - key: c
    text: Newton's method only works for convex functions
    correct: false
  - key: d
    text: First-order optimizers (SGD, Adam) always converge faster in wall-clock time
    correct: false
- id: 8c43cc6c-187b-4bad-9cd7-19af690b6c52
  type: flip
  front: What is the Fisher Information Matrix, and how does it relate to natural gradient descent?
  back: 'Fisher Information Matrix: $F_w = \mathbb{E}_{y \sim p(y|x,w)}[(\nabla_w \log p(y|x,w))(\nabla_w
    \log p(y|x,w))^T]$. It measures parameter sensitivity to changes in the likelihood. Natural gradient
    descent uses $w_{t+1} = w_t - \eta F_t^{-1} \nabla L(w_t)$, where $F_t$ is the Fisher evaluated at
    $w_t$. This is motivated by information geometry: the Fisher defines a Riemannian metric (KL divergence)
    on the parameter manifold. Natural gradient makes steps that are invariant to parametrization (unlike
    Euclidean gradient), providing better scaling for ill-conditioned problems.'
- id: eff065aa-cae9-464f-83b4-1b460e5d03c2
  type: flip
  front: Explain how L-BFGS (limited-memory BFGS) reduces memory complexity from $O(d^2)$ to $O(m \cdot
    d)$ while approximating the Newton step.
  back: L-BFGS stores only the last $m$ gradient differences $\{\Delta g_i, \Delta w_i\}_{i=1}^m$ (typically
    $m \approx 10$) instead of the full Hessian. The inverse Hessian is approximated via a sequence of
    rank-1 updates using these differences, following the BFGS formula. This allows computing $H^{-1}
    \nabla f$ in $O(md)$ time without storing the full $O(d^2)$ matrix. Memory drops from $O(d^2)$ to
    $O(md)$, making second-order methods practical for moderate-scale problems (thousands to tens of thousands
    of parameters).
- id: 196fee91-3af4-4ccc-9cbb-fb0c96ac7696
  type: flip
  front: What is KFAC, and how does it approximate the Fisher Information Matrix for neural networks?
  back: 'KFAC (Kronecker-Factored Approximate Curvature) approximates the Fisher as block-diagonal across
    layers and further factors each block as a Kronecker product:

    $$F_\ell \approx G_\ell \otimes H_\ell$$

    where $G_\ell$ is the outer product of activations (input statistics) and $H_\ell$ is the outer product
    of gradients (output statistics). This reduces memory from $O(d^2)$ to $O(d)$ for each layer and enables
    efficient inverse computation (using Kronecker product properties). KFAC has shown modest improvements
    in optimization speed and generalization for CNNs and RNNs, but adoption remains limited due to implementation
    complexity.'
---

## Intuition
First-order methods (SGD, Adam) use only gradient information. Second-order methods incorporate curvature (Hessian) to make smarter steps: Newton's method requires far fewer iterations to converge but is expensive per iteration. Hessian-free methods try to get the best of both worlds: leverage curvature information without explicitly computing or storing the full Hessian (which is $O(d^2)$ memory for $d$ parameters—infeasible for neural networks with billions of parameters).

## Detail
**Newton's Method**: Update $w_{t+1} = w_t - H_t^{-1} \nabla f(w_t)$ where $H_t = \nabla^2 f(w_t)$. Each step is scaled by inverse curvature: directions with high curvature get small steps, low curvature get large steps. Convergence is quadratic near optimum (super-fast) but requires $O(d^2)$ memory and $O(d^3)$ time for Hessian inversion.

**Natural Gradient Descent**: Replace Hessian with Fisher Information Matrix $F = \mathbb{E}[(\nabla \log p(y|x,w))^2]$. This has a Bayesian interpretation (KL-divergence geometry) and is more stable for neural networks. Update: $w_{t+1} = w_t - \eta F_t^{-1} \nabla f(w_t)$.

**Quasi-Newton Methods (BFGS, L-BFGS)**: Avoid explicit Hessian computation by approximating with low-rank updates. L-BFGS (limited-memory) stores only $m$ past gradient/parameter differences, reducing memory from $O(d^2)$ to $O(m \cdot d)$ (typically $m=10$).

**Hessian-Free Methods**: Use matrix-vector products $H \cdot v$ (Hessian-vector products) via automatic differentiation without materializing $H$. Enables conjugate gradient, KFAC (Kronecker-factored approximate curvature), and subspace methods. These avoid the $O(d^2)$ memory bottleneck.

**KFAC**: Approximates Fisher as block-diagonal (layer-wise): $F \approx \text{diag}(F_1, F_2, \ldots, F_L)$ and further approximates each $F_\ell$ as Kronecker product of activations and gradients. Enables efficient second-order updates with $O(d)$ memory.

## Common gotchas / interview framings
- **Hessian is huge**: $d \times d$ matrix for $d$ parameters (10B for modern LLMs) is prohibitive; must use approximations
- **Indefiniteness**: Away from minima, Hessian can have negative eigenvalues (non-convex); need damping to regularize
- **Batch size coupling**: Natural gradient's statistics (Fisher) depend on batch distribution; noisy estimates harm convergence
- **Convergence per-iteration vs. per-gradient**: Second-order methods converge in fewer iterations but cost more per iteration; wall-clock time may not improve
- **Empirical adoption**: Despite theory, second-order methods see limited use in deep learning due to implementation complexity and marginal wall-clock improvements over well-tuned first-order methods

## See also
- [[newton-method]]
- [[natural-gradient-descent]]
- [[fisher-information-matrix]]
- [[bfgs]]
- [[l-bfgs]]
- [[hessian-computation]]
- [[curvature-matrix]]

## Sources
See frontmatter `sources:`.
