---
id: 3f183513-96e7-4e6b-b65e-44b704e9d063
title: Adam convergence and limitations
track: research-leaning
topic: optimization-learning-theory
difficulty: 5
tags:
- adaptive-learning-rates
- Adam-optimizer
- convergence
- bias-correction
- schedule-free
- practical-optimization
aliases:
- adaptive moment estimation
- Adam algorithm theory
- AdamW variants
sources:
- url: https://arxiv.org/html/2508.05408v1
  label: Cumulative Learning Rate Adaptation (2024)
- url: https://keras.io/api/optimizers/adam/
  label: Keras Adam documentation
- url: https://www.emergentmind.com/topics/schedule-free-adamw
  label: Schedule-Free AdamW
cards:
- id: 278ca3c1-da87-4d9c-bfde-31a28c0d50ae
  type: flip
  front: State the update rule for Adam. What are the roles of the first moment $m_t$ and second moment
    $v_t$?
  back: 'Adam update: $w_{t+1} = w_t - \eta \frac{\hat{m}_t}{\sqrt{\hat{v}_t} + \epsilon}$, where:

    - $m_t = \beta_1 m_{t-1} + (1-\beta_1) g_t$ (exponential average of gradients—momentum)

    - $v_t = \beta_2 v_{t-1} + (1-\beta_2) g_t^2$ (exponential average of squared gradients—per-coordinate
    scaling)

    - $\hat{m}_t = m_t/(1-\beta_1^t)$, $\hat{v}_t = v_t/(1-\beta_2^t)$ (bias correction for early iterations)


    First moment gives direction and momentum; second moment gives per-parameter learning rates (coordinates
    with large gradient variance get smaller effective rates).'
- id: b3d956ec-e803-4af7-a5b4-26a0cb5ae3bb
  type: mcq
  front: What is the main theoretical issue with Adam's convergence, and which algorithm fixes it?
  back: Reddi et al. (2018) showed Adam can diverge on $f(x) = c x$ with certain hyperparameter choices.
    The exponential moving average of second moments can become stale, creating overly aggressive updates.
    AMSGrad (AMS = 'AMSGrad') replaces $v_t$ with $v_t^{\max} = \max(v_t^{\max}, g_t^2)$, ensuring monotonic
    increase and guaranteeing convergence.
  choices:
  - key: a
    text: Adam converges too slowly; SGD with momentum is faster
    correct: false
  - key: b
    text: Adam diverges on simple convex problems when $\beta_2$ is large due to biased variance estimates;
      AMSGrad fixes it by using $v_t = \max(v_t, g_t^2)$
    correct: true
  - key: c
    text: Adam requires more memory; AdamW reduces memory usage
    correct: false
  - key: d
    text: Adam has no convergence issues; the paper by Reddi et al. was disproven
    correct: false
- id: 0b920071-4e46-400c-91d8-e9fd767a0e7e
  type: flip
  front: Why does Adam typically find sharper minima than SGD, and why is this a problem for generalization?
  back: 'Adam''s second-moment scaling ($1/\sqrt{v_t}$) adapts to local geometry: where variance is high,
    step sizes shrink. This rapid adaptation can lead to convergence at sharper minima (high curvature)
    with lower training loss. Sharp minima have worse generalization due to PAC-Bayes bounds: $\text{gen
    gap} \propto \|w\|^2 \rho_{min}^{-d}$ where $\rho_{min}$ is the margin to the decision boundary—sharp
    minima are fragile to perturbations. SGD''s noise naturally prefers flatter regions, achieving better
    test performance despite higher training loss.'
- id: 6b33e12a-38c6-4030-a2d7-4ef98b6a0c86
  type: flip
  front: What is weight decay decoupling in AdamW, and why is it important?
  back: 'Standard Adam applies L2 regularization: $g_t \leftarrow g_t + \lambda w_t$, mixing it into the
    adaptive scaling. This interacts badly with second-moment estimates: directions with small variance
    get very small scaled updates. AdamW applies weight decay directly: $w_{t+1} \leftarrow (1-\lambda\eta)
    w_t - \eta \frac{\hat{m}_t}{\sqrt{\hat{v}_t}+\epsilon}$, decoupling regularization from adaptive rates.
    This prevents the optimizer from ''discounting'' regularization based on gradient statistics, making
    Adam''s regularization more predictable and effective.'
- id: 32c64f38-38e0-4fd7-952b-11c46c5c03e1
  type: flip
  front: What does 'bias correction' mean in Adam, and why is it necessary?
  back: 'Exponential moving averages $m_t = \beta_1 m_{t-1} + (1-\beta_1) g_t$ are initialized at zero.
    Early iterations have biased estimates (e.g., $m_1 = (1-\beta_1)g_1 \ll g_1$ if $\beta_1=0.9$). Bias
    correction computes $\hat{m}_t = m_t/(1-\beta_1^t)$: at $t=1$, $\hat{m}_1 = (1-\beta_1)g_1 / (1-\beta_1)
    = g_1$, recovering the true first-order estimate. Without correction, early iterations take tiny steps,
    slowing learning significantly.'
---

## Intuition
Adam (Adaptive Moment Estimation) is the default optimizer in modern deep learning because it combines first-moment (momentum) and second-moment (adaptive scaling) estimates, eliminating the need for careful learning rate tuning. However, Adam has surprising pathologies: it can diverge even on simple convex problems and requires explicit learning rate schedules to work reliably.

## Detail
Adam maintains running averages:
- First moment: $m_t = \beta_1 m_{t-1} + (1-\beta_1)g_t$ (momentum, typically $\beta_1=0.9$)
- Second moment: $v_t = \beta_2 v_{t-1} + (1-\beta_2)g_t^2$ (RMSprop, typically $\beta_2=0.999$)
- Bias-corrected: $\hat{m}_t = m_t/(1-\beta_1^t)$, $\hat{v}_t = v_t/(1-\beta_2^t)$
- Update: $w_{t+1} = w_t - \eta \hat{m}_t / (\sqrt{\hat{v}_t} + \epsilon)$

**Theoretical Issues**:
1. **Non-convergence (Reddi et al., 2018)**: Adam diverges on simple quadratic functions when $\beta_2$ is large, due to biased variance estimates. AMSGrad fixes this by taking $v_t = \max(v_t, g_t^2)$.
2. **Learning rate schedule dependence**: Despite being "adaptive", Adam requires careful schedules (cosine annealing, warmup) to converge reliably.
3. **Generalization gap**: Adam often produces sharp minima that generalize worse than SGD (Wilson et al., 2017).

**AdamW & Schedule-Free Variants**: Weight decay decoupling (L2 regularization applied directly) prevents adaptive rates from interfering with regularization. Schedule-Free AdamW (Defazio et al., 2024) eliminates the need to specify schedule and total steps in advance using implicit averaging and interpolation—a major practical advance.

## Common gotchas / interview framings
- **Not schedule-free by default**: Despite the name, standard Adam requires explicit learning rate decay schedules
- **Sharp minima**: Adam finds sharper minima than SGD; combine with SAM (Sharpness-Aware Minimization) for better generalization
- **Epsilon matters**: The $\epsilon$ term (typically $10^{-8}$) prevents division by zero but affects final performance; smaller $\epsilon$ can destabilize training
- **Warmup is essential**: First few iterations can have unreliable second-moment estimates; linear warmup helps
- **Bias correction in early iterations**: The $(1-\beta_t)$ terms correct initialization; omitting them causes poor early training

## See also
- [[momentum]]
- [[adaptive-methods]]
- [[learning-rate-scheduling]]
- [[amsgrad]]
- [[schedule-free-optimization]]
- [[second-moment-bias]]

## Sources
See frontmatter `sources:`.
