---
id: bcdb322c-b63f-4b72-a71f-91f89256bfca
title: Sharpness vs flatness of minima
track: research-leaning
topic: optimization-learning-theory
difficulty: 5
tags:
- loss-landscape
- generalization
- sharpness-aware-minimization
- PAC-Bayes
- flatness-heuristic
- overparameterization
aliases:
- sharp minima
- flat minima
- Hessian curvature
- loss geometry
- minimum flatness
sources:
- url: https://arxiv.org/abs/2010.01412
  label: Sharpness-Aware Minimization for Efficiently Improving Generalization (Foret et al., 2020)
- url: https://www.inference.vc/sharp-vs-flat-minima-are-still-a-mystery-to-me/
  label: Sharp vs Flat Minima Analysis
- url: https://arxiv.org/pdf/2511.03548
  label: 'Flat Minima and Generalization: Insights from Stochastic Convex Optimization (2025)'
cards:
- id: 2bbafdfb-b855-42d5-957d-1895236f5909
  type: flip
  front: Define a sharp vs. flat minimum in terms of the Hessian matrix. Why do flat minima tend to generalize
    better?
  back: 'A minimum $w^*$ is **sharp** if the Hessian $H = \nabla^2 f(w^*)$ has large eigenvalues (high
    curvature); **flat** if eigenvalues are small. Flat minima generalize better because: (1) small changes
    in weights cause small changes in predictions (robustness), (2) they correspond to wide "valleys"
    in loss space that are harder to escape under distribution shift, and (3) PAC-Bayes bounds scale with
    Hessian norm, yielding tighter guarantees for flat minima.'
- id: 4910e332-8933-4ac7-ae78-0d582180c97f
  type: mcq
  front: What is the core idea of Sharpness-Aware Minimization (SAM)?
  back: 'SAM performs an inner maximization: $w_{t+1} = w_t - \eta \nabla f(w_t + \rho \frac{\nabla f(w_t)}{\|\nabla
    f(w_t)\|})$. This perturbs $w_t$ by a small radius $\rho$ in the steepest gradient direction, then
    computes gradient at the perturbed point. The effect is to penalize regions where loss is high in
    the neighborhood, naturally seeking flat (low-curvature) minima. Empirically, SAM improves generalization
    across vision, NLP, and RL without explicit Hessian computation.'
  choices:
  - key: a
    text: Add $L_2$ regularization to penalize large weights
    correct: false
  - key: b
    text: Seek parameters whose $\rho$-neighborhoods have uniformly low loss, implicitly penalizing sharp
      minima
    correct: true
  - key: c
    text: Compute the Hessian and set a hard constraint on its norm
    correct: false
  - key: d
    text: Use second-order (Newton) optimization to directly minimize Hessian eigenvalues
    correct: false
- id: 6830804b-9ea0-40de-95ca-9fea8ced92df
  type: flip
  front: Explain the reparametrization sensitivity of sharpness. Why does this complicate the story that
    'flatter minima generalize better'?
  back: 'Sharpness is parametrization-dependent. Scaling weights $w \leftarrow \lambda w$ transforms the
    Hessian: $H(\lambda w) = H(w)/\lambda^2$. A sharp minimum can be made arbitrarily flat by choosing
    $\lambda$ large. This suggests sharpness alone is not the fundamental cause of generalization—it may
    merely be a proxy for more intrinsic properties like large margins, implicit bias towards simple solutions,
    or stability to perturbations. Current research (2024-2025) seeks scale-invariant measures of flatness
    (e.g., tracing over the loss gradient manifold).'
- id: 43a56e9c-894e-46a7-a5ec-c09c929c5597
  type: flip
  front: How does overparameterization interact with SAM's effectiveness? Why do flat minima play a larger
    role in overparameterized models?
  back: 'Overparameterized models have many minima, allowing the optimizer to choose among solutions with
    very different generalization properties. SAM''s perturbation mechanism becomes more powerful: it
    can steer optimization away from sharp minima toward flatter regions where multiple minima coexist
    (the loss surface is more degenerate). In the non-overparameterized case, all minima might be equally
    sharp (limited choice), making SAM less useful. Thus, SAM achieves near-linear convergence rates in
    overparameterized settings while maintaining improved generalization.'
- id: 8ae85b76-703e-46e7-a14b-bd1b35a6631b
  type: flip
  front: State a generalization bound involving the Hessian (or sharpness measure). What term scales with
    flatness/sharpness?
  back: 'PAC-Bayes bound (simplified): $\mathbb{E}[L_{test}] \lesssim L_{train} + \sqrt{\frac{\text{KL}
    + \log(1/\delta)}{n}}$ where $\text{KL} = E_{w \sim Q}[\log(Q(w)/P(w))]$. When $Q$ is a Gaussian centered
    at optimal $w^*$ with covariance $\propto H^{-1}$, the KL divergence scales as $\log \det(H) \approx
    \sum \log \lambda_i$ (sum of log-eigenvalues). Flatter minima with smaller eigenvalues have lower
    KL, yielding better bounds. The dependence on spectral properties of the Hessian is the formal link
    between geometry and generalization.'
---

## Intuition
The loss landscape of neural networks has many minima. Some are sharp (curvature is high, loss rises steeply away from the minimum) and some are flat (gentle curvature, loss barely changes). Empirically, flatter minima generalize better: they're less sensitive to perturbations (weight noise, covariate shift). This motivates Sharpness-Aware Minimization (SAM), which explicitly seeks flat minima and has shown consistent improvements across vision, NLP, and RL tasks.

## Detail
**Sharpness Measures**: A minimum $w^*$ is sharp if the Hessian $H = \nabla^2 f(w^*)$ has large eigenvalues. Sharpness can be quantified as:
- $\rho$-sharpness: $\max_{\|\delta\| \le \rho} f(w^* + \delta) - f(w^*) = O(\|H\| \rho^2)$
- Max eigenvalue: $\lambda_{\max}(H)$ (principal curvature)
- Spectrum width: condition number $\kappa = \lambda_{max}/\lambda_{min}$

**SAM Algorithm**: Seeks parameters in neighborhoods with uniformly low loss:
$$w_{t+1} = w_t - \eta \nabla f(w_t + \rho \frac{\nabla f(w_t)}{\|\nabla f(w_t)\|})$$
This first moves in gradient direction by distance $\rho$ (perturbation radius), then updates using that perturbed gradient. Effect: penalizes sharp minima implicitly.

**PAC-Bayes Connection**: Generalization bound depends on sharpness (via Hessian trace or spectral norm). Flatter minima with smaller $\|H\|$ yield tighter bounds:
$$\text{gen gap} \lesssim \sqrt{\frac{1}{n}} \|H\|^{1/2} + \text{other terms}$$

**Critical Caveat—Reparametrization Sensitivity**: Sharpness is NOT intrinsic! Rescaling $w \leftarrow \lambda w$ changes Hessian ($H \leftarrow H/\lambda^2$) without changing function. This means "sharp" and "flat" are parametrization-dependent, raising questions about whether flatness is truly the right generalization principle or merely a proxy for other factors (large margins, implicit bias of SGD).

## Common gotchas / interview framings
- **Parametrization invariance**: A sharp minimum can be made arbitrarily flat by reparametrization; thus flatness alone doesn't fully explain generalization
- **SAM convergence**: SAM is more expensive than SGD (requires gradient computation at perturbed point), but improves generalization at modest cost
- **Eigenvalue vs. norm**: Using trace($H$) vs. spectral norm $\|H\|$ gives different sharpness orderings; choice matters empirically
- **Robustness interpretation**: Flat minima are more robust to label noise, distribution shift, and adversarial perturbations—this may be more fundamental than the Hessian view
- **Trade-off with optimization**: Seeking flatness can slow convergence; SAM requires tuning $\rho$ (perturbation radius) carefully

## See also
- [[sam-sharpness-aware-minimization]]
- [[pac-bayes-bounds]]
- [[loss-landscape-geometry]]
- [[hessian-matrix]]
- [[generalization-bounds]]
- [[reparametrization-invariance]]
- [[noise-injection]]

## Sources
See frontmatter `sources:`.
