---
id: 56388325-8d34-45d2-8124-b0289b279dac
title: SGD convergence analysis (convex and non-convex)
track: research-leaning
topic: optimization-learning-theory
difficulty: 5
tags:
- optimization
- convergence
- stochastic-methods
- convex-analysis
- non-convex-optimization
- learning-theory
aliases:
- stochastic gradient descent convergence
- convergence rates
- optimization analysis
sources:
- url: https://arxiv.org/html/2508.05408v1
  label: 'Cumulative Learning Rate Adaptation: Revisiting Path-Based Schedules'
- url: https://arxiv.org/pdf/2509.18396
  label: Development of Deep Learning Optimizers
- url: https://en.wikipedia.org/wiki/Stochastic_gradient_descent
  label: SGD - Wikipedia
cards:
- id: 7b0dccaf-1afa-4736-998c-5df0353178e4
  type: flip
  front: In the convex smooth case, what is the convergence rate of SGD with learning rate schedule $\eta_t
    = c/\sqrt{t}$?
  back: 'SGD achieves $O(1/\sqrt{T})$ convergence in expectation: $E[f(\bar{w}_T)] - f(w^*) = O(\sigma^2/\sqrt{T})$
    where $\sigma^2$ is the variance of stochastic gradients. The $1/\sqrt{T}$ rate reflects the fundamental
    limit of first-order methods with noisy gradients in the convex setting.'
- id: 1675f65e-484a-4975-9341-5f60487c1312
  type: flip
  front: For non-convex optimization, what condition on the learning rate schedule $\{\eta_t\}$ ensures
    SGD convergence to a stationary point?
  back: 'The schedule must satisfy: (1) $\sum_{t=1}^\infty \eta_t = \infty$ (sum diverges), and (2) $\sum_{t=1}^\infty
    \eta_t^2 < \infty$ (sum of squares converges). This allows the learning rate to decay slowly enough
    to escape local issues but fast enough for variance to vanish. Common choices: $\eta_t = O(1/\sqrt{t})$
    or $\eta_t = O(1/t^{0.6})$.'
- id: 0d89f104-2ca1-4bc9-b391-a8bd76a08dc8
  type: mcq
  front: How does batch size $B$ affect the convergence rate of SGD? If you increase batch size from $B$
    to $4B$, how should the learning rate $\eta$ be adjusted to maintain the same effective progress?
  back: 'Variance scales as $\sigma^2/B$. Increasing batch size reduces noise variance, allowing larger
    learning rate steps. The scaling rule $\eta \propto \sqrt{B}$ (linear scaling rule) is widely used:
    doubling batch size justifies $\sqrt{2}\approx 1.41$ times larger learning rate. This keeps the ''effective''
    noise level constant relative to learning.'
  choices:
  - key: a
    text: No adjustment needed; batch size does not affect the asymptotic rate
    correct: false
  - key: b
    text: Increase $\eta$ to $2\eta$ (proportional to $\sqrt{B}$)
    correct: true
  - key: c
    text: Increase $\eta$ to $4\eta$ (proportional to $B$)
    correct: false
  - key: d
    text: Decrease $\eta$ to $\eta/2$ to prevent divergence
    correct: false
- id: 62f969df-3b7b-464d-89a8-d987818f8ca1
  type: flip
  front: What is the difference between SGD convergence to a stationary point vs. convergence to a minimum?
    Why does this distinction matter for deep learning?
  back: SGD guarantees convergence to gradient norm $\|\nabla f(w)\| \to 0$ (stationary point), which
    could be a local minimum, saddle point, or plateau. For non-convex deep networks, most critical points
    are saddle points (Dauphin et al., 2014); SGD's noise helps escape them. However, convergence theory
    does not guarantee finding *global* minima—the final model quality depends on landscape structure
    and initialization, making deep learning fundamentally empirical.
- id: bca6fb51-ca14-4d2c-b372-84a6a87a58b6
  type: flip
  front: In non-convex SGD analysis, the convergence bound includes a term proportional to the variance
    of stochastic gradients. Why is this term unavoidable, and how does momentum help?
  back: 'Noisy gradients prevent arbitrarily small steps near stationary points: the best achievable bound
    includes $O(\sigma^2)$ error. Momentum reduces effective variance by aggregating past gradients, smoothing
    oscillations and allowing adaptive step sizes that respond to gradient direction—this is formalized
    in acceleration theory (Nesterov momentum achieves $O(1/T)$ for convex problems vs. $O(1/\sqrt{T})$
    for SGD).'
---

## Intuition
SGD is the workhorse of deep learning, but its convergence properties depend critically on the problem geometry (convex vs non-convex) and hyperparameter choices. Unlike full-batch gradient descent, SGD introduces noise from mini-batch sampling, which can both help (escape sharp minima) and hurt (slower convergence).

## Detail
**Convex Case**: For convex smooth functions with learning rate $\eta_t = O(1/\sqrt{t})$, SGD achieves $O(1/\sqrt{T})$ convergence in expectation under bounded variance assumption. Strongly convex problems get $O(1/T)$ rates with appropriate schedules.

**Non-convex Case**: For non-convex smooth objectives (deep neural networks), SGD converges to a stationary point (gradient norm $< \epsilon$) at rate $O(1/\sqrt{T})$ under standard assumptions. The noise variance $\sigma^2$ directly scales convergence: $E[\|\nabla f(w_T)\|^2] = O(\sigma^2/\sqrt{T} + \sigma^2/\eta_T)$.

**Key Dependencies**:
- Learning rate schedule $\eta_t$ must decay appropriately (diminishing but non-summable for convergence)
- Batch size $B$ affects variance: variance $\propto 1/B$
- Momentum (Nesterov or classical) can improve rates by reducing oscillation
- Loss landscape geometry matters: condition number $\kappa$ affects constants

## Common gotchas / interview framings
- **Exact convergence**: SGD converges to stationary points, not optima—true minima require careful initialization
- **Variance ≠ batch size**: Learning rate must scale with batch size; doubling batch size requires schedule adjustment
- **Non-convexity is the rule**: Deep networks are highly non-convex; convex bounds give loose guarantees
- **Warm-up phase**: Initial high learning rate can diverge; warmup schedules (linear, exponential) are essential
- **Generalization vs optimization**: Convergence to training loss ≠ good test performance; sharp minima found by aggressive schedules may generalize poorly

## See also
- [[stochastic-gradient-descent]]
- [[convex-optimization]]
- [[convergence-rate]]
- [[subgradient-methods]]
- [[non-convex-optimization]]
- [[batch-size-effects]]
- [[noise-robust-methods]]

## Sources
See frontmatter `sources:`.
