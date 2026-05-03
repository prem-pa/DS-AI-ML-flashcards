---
id: b670aeab-bc08-4f5c-ad93-cf5fe2da7f6d
title: Exploding/vanishing gradient analysis
track: research-leaning
topic: transformer-theory-internals
difficulty: 5
tags:
- exploding-gradients
- vanishing-gradients
- gradient-clipping
- learning-rate
- activation-growth
- numerical-stability
aliases:
- gradient-explosion
- gradient-decay
- numerical-stability-training
sources:
- url: https://arxiv.org/abs/1211.1541
  label: 'Bengio et al. (2013): On the difficulty of training deep recurrent networks'
- url: https://arxiv.org/abs/1502.01852
  label: 'Ba et al. (2016): Layer Normalization'
- url: https://arxiv.org/abs/1909.08383
  label: 'Zhang & Sennrich (2019): Root Mean Square Layer Normalization'
- url: https://arxiv.org/abs/2304.08918
  label: Stabilizing Training of Transformers - recent gradient analysis
cards:
- id: 01bfe69d-25dd-47ed-ba50-5ec0401c371e
  type: flip
  front: Write the chain rule expression for backpropagating gradients through $d$ layers. Why do vanishing
    gradients occur?
  back: $\frac{\partial \text{Loss}}{\partial x_0} = \frac{\partial \text{Loss}}{\partial x_d} \prod_{i=1}^{d}
    J_i$ where $J_i = \frac{\partial x_i}{\partial x_{i-1}}$. Vanishing gradients occur when $||J_i||_2
    < 1$ for many layers. The product becomes $||J||^d \approx \lambda^d$ for $\lambda < 1$, decaying
    exponentially. For $d=100$ and $\lambda=0.9$, we get $0.9^{100} \approx 10^{-5}$—early layers receive
    nearly zero gradient.
- id: 5261cdc1-b2b3-4f82-b451-4f066909bde5
  type: mcq
  front: How does layer normalization prevent vanishing gradients in transformers?
  back: 'Layer norm normalizes activations: $\text{LayerNorm}(x) = \gamma \frac{x - \mu}{\sigma}$. This
    ensures $||x||_2$ is controlled. The Jacobian of a layer depends on activation magnitudes and weight
    magnitudes. Stable activations (unit variance) + weight initialization (Kaiming) ensure $||J_i||_2
    \approx 1$, preventing the product $\prod J_i$ from decaying exponentially. Without normalization,
    activations can grow/shrink, causing gradient pathologies.'
  choices:
  - key: a
    text: By computing the gradient more efficiently
    correct: false
  - key: b
    text: By constraining activation magnitudes to unit variance, ensuring stable gradient propagation
      and preventing exponential decay
    correct: true
  - key: c
    text: By replacing the softmax function with a more stable alternative
    correct: false
  - key: d
    text: By adding skip connections that bypass all nonlinearities
    correct: false
- id: d2b7672e-b197-4036-8d41-c9bb2a6c3b48
  type: flip
  front: 'Explain gradient clipping: when is it applied, and why is it necessary in transformers despite
    layer norm?'
  back: 'Gradient clipping rescales the gradient vector: if $||\nabla \text{Loss}||_2 > \tau$, set $\nabla
    \leftarrow \tau \frac{\nabla}{||\nabla||_2}$. It''s applied after backprop, before weight updates.
    It''s necessary because: (1) layer norm controls activation magnitudes, not gradient magnitudes; (2)
    rare data patterns or adversarial examples can cause gradient spikes; (3) clipping acts as a safety
    mechanism, constraining step size. Typical threshold $\tau \approx 1.0$.'
- id: 7a5565dc-6043-44e6-b9cc-558819997888
  type: flip
  front: Why is learning rate warmup critical for training transformers, and how does it relate to gradient
    stability?
  back: Warmup gradually increases the learning rate from a small value (e.g., 0) to the target value
    over an initial period (e.g., 1000 steps). Early in training, the network is random, and gradient
    magnitudes can be volatile. Large learning rates combined with volatile gradients can cause divergence
    or numerical instability. Warmup allows gradients to 'settle' into a stable regime before full-scale
    updates. Without warmup, the first few steps can derail training entirely.
---

## Intuition

**Vanishing gradients**: In deep networks, gradients propagate backward via the chain rule. If each layer multiplies the gradient by a value $< 1$, the product decays exponentially: $\prod_{i=1}^{d} \lambda_i \approx \lambda^d$ where $\lambda < 1$ and $d$ is depth. For $d = 100$ and $\lambda = 0.9$, the gradient is $0.9^{100} \approx 2.7 \times 10^{-5}$—nearly zero. Early layers receive almost no gradient signal, preventing learning.

**Exploding gradients**: Conversely, if $\lambda > 1$, gradients grow exponentially: $\lambda^d$ can be enormous, causing numerical overflow or chaotic weight updates.

For transformers, these issues arise from:
1. Depth: 100+ layers multiply gradients
2. Attention structure: softmax can create small gradients if attention is concentrated
3. Large weight initializations or learning rates

## Detail

**Formal analysis**:
For a chain of layers with Jacobians $J_i = \frac{\partial x_i}{\partial x_{i-1}}$, the gradient backpropagates as:

$$\frac{\partial \text{Loss}}{\partial x_0} = \frac{\partial \text{Loss}}{\partial x_d} \prod_{i=1}^{d} J_i$$

For stability, we need $||J_i||_2 \approx 1$ (spectral norm). For a nonlinear layer with weight matrix $W$ and activation $\sigma$:

$$J_i = \text{diag}(\sigma'(z_i)) W$$

The spectral norm is $||J_i||_2 \leq ||\sigma'(z_i)||_\infty \cdot ||W||_2$. For ReLU, $||\sigma'||_\infty = 1$, so stability requires $||W||_2 \approx 1$.

**In transformers**:
- **Attention Jacobian**: For softmax, $\frac{\partial \text{Attn}}{\partial Q} = \text{softmax gradient}$. If attention is sharp (one token dominates), the gradient can be small. If diffuse, gradients are larger but noisier
- **MLP Jacobian**: For $\text{MLP}(x) = W_2 \text{ReLU}(W_1 x)$, the Jacobian is $W_2 \text{diag}(\text{ReLU}'(W_1 x)) W_1$. Stability requires both $||W_1||_2 \approx 1$ and $||W_2||_2 \approx 1$

**Solutions in transformers**:

1. **Layer normalization** (Ba et al., 2016):
   - Normalizes activations to unit variance: $\text{LayerNorm}(x) = \gamma \frac{x - \mu}{\sigma + \epsilon}$
   - This constrains $||x||_2$, preventing activation growth
   - Combined with proper weight initialization (Kaiming), ensures $||J_i||_2 \approx 1$

2. **Weight initialization**:
   - Kaiming (He) initialization: $W \sim \mathcal{N}(0, 2/n_\text{in})$
   - Ensures $||W||_2 \approx 1$ on average, stabilizing gradient propagation

3. **Gradient clipping**:
   - If $||\nabla_\theta \text{Loss}||_2 > \tau$, rescale: $\nabla \leftarrow \tau \frac{\nabla}{||\nabla||_2}$
   - Prevents exploding gradients from destabilizing updates
   - Common threshold: $\tau = 1.0$

4. **Learning rate warmup**:
   - Start with small learning rate, gradually increase
   - Allows the network to settle into a stable regime before large updates
   - Critical for transformers; prevents early gradient spikes from derailing training

## Common gotchas / interview framings

- **"Can't we just use smaller learning rates to prevent exploding gradients?"** Partially, but it's expensive—tiny learning rates slow training dramatically. Gradient clipping is more efficient, constraining the step size while allowing larger learning rates
- **"Why is gradient clipping needed if we have layer norm?"** Layer norm stabilizes *activations*, not gradients. Gradients depend on the data; adversarial examples or rare data patterns can still cause gradient spikes. Clipping is a safety mechanism
- **"How does warmup relate to gradient stability?"** Early in training, the network is random and poorly calibrated. Without warmup, large learning rates can cause gradient instability or divergence. Warmup allows gradients to "settle" before full-scale training
- **"Does attention cause vanishing gradients?"** Not inherently, but concentrated attention (sharp softmax) reduces gradient signal. Layer norm mitigates this by normalizing pre-attention activations
- **"Why do some papers report vanishing gradients in very deep models even with layer norm?"** Because layer norm controls activation magnitudes, not gradient magnitudes directly. In very deep models (>1000 layers), even with layer norm, gradients can degrade. More advanced techniques (like per-layer adaptive normalization) are needed

## See also
- [[gradient-flow-in-deep-transformers]]
- [[normalization-layers-layernorm-rmsnorm-prenorm]]
- [[implicit-regularization-in-transformers]]
- [[attention-as-kernel-method]]
- [[universal-approximation-of-transformers]]

## Sources
See frontmatter `sources:`.
