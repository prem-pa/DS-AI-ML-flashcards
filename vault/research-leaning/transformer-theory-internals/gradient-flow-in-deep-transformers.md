---
id: f70ffb6e-7409-4f09-88e0-3963bd025671
title: Gradient flow in deep transformers
track: research-leaning
topic: transformer-theory-internals
difficulty: 5
tags:
- gradient-flow
- layer-norm
- residual-connections
- skip-connections
- deep-networks
- trainability
aliases:
- skip-connections-gradient
- residual-networks-gradient
- layer-norm-gradient
sources:
- url: https://arxiv.org/abs/1512.03385
  label: 'He et al. (2015): Deep Residual Learning for Image Recognition'
- url: https://arxiv.org/abs/1906.02414
  label: 'Wang et al. (2019): Understanding the Difficulty of Training Deep Feedforward Neural Networks'
- url: https://arxiv.org/abs/2002.05957
  label: Transformers as Bayesian Neural Networks - Smith & Gal (2018) + Transformers gradient flow analysis
- url: https://arxiv.org/abs/2304.08918
  label: 'Merrill et al. (2023): The Implicit Bias of Gradient Descent on Separable Data'
cards:
- id: dbbe2710-035e-4da9-a394-60c21b7fa39e
  type: flip
  front: How do residual connections prevent vanishing gradients in deep networks? Write the gradient
    expression.
  back: In a residual block $x' = x + f(x)$, the gradient is $\frac{\partial \text{Loss}}{\partial x}
    = \frac{\partial \text{Loss}}{\partial x'} (I + \frac{\partial f(x)}{\partial x})$ where $I$ is the
    identity matrix. The identity term ensures a gradient contribution of 1 flows backward through each
    layer. Without residuals (just $x' = f(x)$), the gradient is $\frac{\partial \text{Loss}}{\partial
    x} = \frac{\partial \text{Loss}}{\partial x'} \frac{\partial f(x)}{\partial x}$, which vanishes if
    the Jacobian has small eigenvalues.
- id: 3ead3623-f8e5-4ccf-9130-d415fd4c9481
  type: mcq
  front: In a 100-layer transformer, what is the combined effect of residuals and layer norm on gradient
    flow?
  back: 'Both are necessary and complementary. Residuals provide the identity bypass: $x'' = x + f(x)$
    ensures $\nabla x \supseteq 1$ (through the skip). Layer norm stabilizes $f(x)$ outputs: if $||f(x)||_2$
    is controlled, then $|\nabla f|$ is bounded, preventing the Jacobians from having extremely small
    or large eigenvalues. Together, they ensure gradients neither vanish nor explode over 100 layers.'
  choices:
  - key: a
    text: Residuals allow bypass of nonlinearities; layer norm stabilizes activation magnitudes, preventing
      exponential gradient decay or growth
    correct: false
  - key: b
    text: Layer norm replaces the need for residuals, making skip connections redundant
    correct: false
  - key: c
    text: Residuals reduce the need for normalization by implicitly normalizing gradients
    correct: false
  - key: d
    text: Neither residuals nor normalization significantly affect gradient flow in attention layers
    correct: false
- id: 5d848634-33f2-447f-a8da-10dc1b61e459
  type: flip
  front: What is the difference between PostNorm and PreNorm in transformers, and which is preferred for
    very deep models?
  back: 'PostNorm applies layer norm after the attention/MLP block: $x'' = \text{LayerNorm}(x + f(x))$.
    PreNorm applies it before: $x'' = x + f(\text{LayerNorm}(x))$. PreNorm has better gradient flow in
    very deep networks because norm stabilizes the input to nonlinearities. PostNorm has better optimization
    signal propagation near convergence but can cause gradient degradation with depth. Modern large models
    favor PreNorm for stability, though the best choice depends on depth and learning rate.'
- id: 678f2be6-ef23-42ed-aca3-3b9f7e3b2f82
  type: flip
  front: Why is RMSNorm preferred over LayerNorm in modern LLMs like GPT-3?
  back: 'RMSNorm normalizes by RMS (root mean square) without subtracting the mean: $\text{RMSNorm}(x)
    = \gamma \frac{x}{\text{RMS}(x)}$ where $\text{RMS}(x) = \sqrt{\mathbb{E}[x^2] + \epsilon}$. This
    is simpler to compute than LayerNorm (no mean subtraction). Empirically, RMSNorm achieves similar
    gradient stability with lower computational cost. This is especially important for scaling to very
    large models where every operation counts.'
---

## Intuition

Training deep neural networks is difficult: gradients computed via backpropagation either explode or vanish as they propagate backward through many layers. This is especially problematic in transformers, which can have 100+ layers. Residual connections and layer normalization solve this problem by providing a *stable gradient highway*: gradients can flow directly backward through skip connections, avoiding multiplicative accumulation.

## Detail

**Residual connections (skip connections)**:
Each transformer block computes:
$$x' = x + \text{Attention}(x) + \text{MLP}(x)$$

Instead of:
$$x' = \text{Attention}(x) + \text{MLP}(x)$$

The residual form $x' = x + f(x)$ enables stable backpropagation. The gradient with respect to $x$ is:
$$\frac{\partial \text{Loss}}{\partial x} = \frac{\partial \text{Loss}}{\partial x'} \left(1 + \frac{\partial f(x)}{\partial x}\right)$$

The $+1$ term ensures that even if $\frac{\partial f}{\partial x}$ vanishes (or explodes modestly), the gradient is not completely killed. This is the *skip connection effect*: information can bypass the nonlinear layers.

**Layer normalization**:
Layer norm (or RMSNorm, PreNorm) normalizes activations to unit variance before the attention/MLP:
$$\text{LayerNorm}(x) = \gamma \frac{x - \mathbb{E}[x]}{\sqrt{\text{Var}(x) + \epsilon}} + \beta$$

This stabilizes activation magnitudes, preventing them from growing or shrinking dramatically. When activation magnitudes are stable, gradients computed via the chain rule have consistent magnitude—neither exploding nor vanishing.

**Composition effect**:
For a $d$-layer transformer, the gradient from layer $d$ back to layer 1 is:
$$\frac{\partial \text{Loss}}{\partial x_0} = \frac{\partial \text{Loss}}{\partial x_d} \prod_{i=1}^{d} \frac{\partial x_i}{\partial x_{i-1}}$$

Without residuals, $\frac{\partial x_i}{\partial x_{i-1}} \approx \nabla f(x_{i-1})$ (the Jacobian of the nonlinearity). For ReLU, this is sparse (many zeros and ones), causing vanishing gradients if the chain multiplies small values.

With residuals, $\frac{\partial x_i}{\partial x_{i-1}} = I + \nabla f(x_{i-1})$, where $I$ is the identity. The identity term ensures a gradient contribution of 1 through each layer, preventing complete vanishing.

**Normalization variants**:
- **PostNorm** ("Attention -> LayerNorm"): Normalizes outputs. Simpler, but gradients can still degrade with depth
- **PreNorm** ("LayerNorm -> Attention"): Normalizes inputs. Gradient flow is more stable, but may hurt optimization near convergence
- **RMSNorm**: Simpler version (no mean subtraction), computationally efficient, used in GPT-3+

## Common gotchas / interview framings

- **"Why do residuals prevent vanishing gradients?"** Because the identity $I$ in $\frac{\partial x_i}{\partial x_{i-1}} = I + \nabla f(x_{i-1})$ ensures a gradient contribution of 1 flows backward. Without residuals, you multiply Jacobians, which can decay exponentially
- **"Why is layer norm before or after attention important (PreNorm vs PostNorm)?"** PreNorm stabilizes gradient flow but may hurt optimization signal propagation. PostNorm preserves signal but can cause gradient decay. Recent work suggests PreNorm is better for very deep networks
- **"Can transformers work without layer norm?"** Not really. Without normalization, activations grow, causing gradient overflow. Skip connections alone are insufficient for very deep networks
- **"How does this relate to the implicit regularization story?"** Layer norm + residuals enable SGD to escape the NTK lazy regime and enter feature learning (see [[implicit-regularization-in-transformers]]). By stabilizing gradients, they allow larger weight updates, which enables the network to learn features from scratch
- **"Why do modern LLMs use RMSNorm instead of LayerNorm?"** RMSNorm removes mean subtraction, making it cheaper to compute. Empirically, it achieves similar gradient stability with simpler arithmetic. GPT-3 and later models use RMSNorm

## See also
- [[explodingvanishing-gradient-analysis]]
- [[implicit-regularization-in-transformers]]
- [[normalization-layers-layernorm-rmsnorm-prenorm]]
- [[attention-as-kernel-method]]
- [[gradient-flow-in-deep-transformers]]

## Sources
See frontmatter `sources:`.
