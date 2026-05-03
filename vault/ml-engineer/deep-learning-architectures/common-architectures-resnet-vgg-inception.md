---
id: 14e00e47-c5d2-4dcb-85d1-2efc27dc99ac
title: Common architectures (ResNet, VGG, Inception)
track: ml-engineer
topic: deep-learning-architectures
difficulty: 3
tags:
- cnn
- skip-connections
- residual-learning
- inception-module
- architecture-design
aliases:
- resnet
- vgg
- inception
- skip-connection-motivation
sources:
- url: https://arxiv.org/abs/1512.03385
  label: Deep Residual Learning for Image Recognition (ResNet)
- url: https://arxiv.org/abs/1409.1556
  label: Very Deep Convolutional Networks for Large-Scale Image Recognition (VGG)
- url: https://arxiv.org/abs/1409.4842
  label: Going Deeper with Convolutions (Inception)
cards:
- id: 9cf99cf6-5c1e-43d4-9e46-3c551f143540
  type: flip
  front: ResNet skip connection enables training deeper networks. Explain the gradient path.
  back: $\nabla_x L = \nabla_F(x) L + \nabla_x L$ (identity contributes $\nabla_x L$ directly). Gradients
    always flow through identity, bypassing vanishing bottlenecks in deep layers.
- id: 07bf0d5d-dca3-4ee4-954f-c508d9141b3f
  type: flip
  front: 'Bottleneck block in ResNet-50: why compress to d/4, then expand back to d?'
  back: 'Reduces FLOPs: $d \to d/4 \to d/4 \to d$ makes the 3×3 cheaper. Learned low-rank approximation
    of the transformation. Enables very deep networks within computational budget.'
- id: 38a5f6bd-117e-473f-b401-e5637cb14df7
  type: mcq
  front: You train VGG-16 and ResNet-50 from scratch on ImageNet. What do you expect?
  back: ResNet's skip connections alleviate vanishing gradients; deeper is trainable. VGG-16 (A) is actually
    slower due to gradient signal loss. (C) wrong; skip connections matter. (D) more params ≠ better generalization.
  choices:
  - key: a
    text: VGG-16 converges faster due to simpler architecture
    correct: false
  - key: b
    text: ResNet-50 converges faster and reaches higher accuracy; VGG may plateau or degrade
    correct: true
  - key: c
    text: Both reach identical accuracy; architecture doesn't matter much
    correct: false
  - key: d
    text: VGG-16 generalizes better due to more parameters
    correct: false
- id: b8556a75-6b84-4c1a-b476-df59403bcbe8
  type: flip
  front: In Inception module, why place 1×1 convolutions BEFORE 3×3 and 5×5?
  back: '1×1 reduces channel dimension before expensive convs. Example: 256→64→64 (3×3) costs less than
    256→256→256 (3×3). Dimensionality reduction without losing expressiveness via learned projection.'
- id: 9a0423d6-3fb8-48f9-b3b6-1a0d625e8792
  type: mcq
  front: Modifying ResNet-50 for semantic segmentation (dense predictions). What must you change?
  back: '(B) essential: dense tasks need fine spatial resolution. Stride-32 output is too coarse. Dilated
    convs recover RF without resolution loss. Skip connections (A) are still helpful. (C) & (D) don''t
    address the core issue.'
  choices:
  - key: a
    text: Remove skip connections; they hurt dense prediction
    correct: false
  - key: b
    text: Reduce stride from 32× downsampling to 8× or 4×; use dilated convs in later stages
    correct: true
  - key: c
    text: Increase number of skip connections
    correct: false
  - key: d
    text: Replace ReLU with Sigmoid
    correct: false
---

## Intuition

**VGG**: Simple, deep stack of 3×3 convolutions. No tricks; pure depth. Proven that depth > receptive field tricks.

**ResNet**: Introduces **skip connections**: $y = F(x) + x$. Transforms the learning problem from mapping $x \to y$ to residual $x \to F(x)$. Gradients bypass deep layers via identity, enabling very deep networks (152+ layers).

**Inception**: Multi-scale convolutions in parallel (1×1, 3×3, 5×5, pooling) concatenated. Reduces parameters via 1×1 bottlenecks before expensive convolutions.

## Detail

**ResNet bottleneck block**:
```
Input → Conv(1×1, d/4) → ReLU → Conv(3×3, d/4) → ReLU → Conv(1×1, d) → Add with Input → ReLU
```

Key insight: $\nabla_{x} L = \nabla_{F(x)} L + \nabla_x L$ (chain rule). Gradient always flows directly through skip, preventing vanishing gradients. **Parameter count**: ResNet-50 has ~25M params; VGG-16 has ~138M (6× more) for comparable accuracy.

**Inception module**:
- Parallel branches capture multi-scale patterns
- 1×1 convs reduce dimension before 3×3, 5×5 (huge efficiency gain)
- Concatenate outputs; later layers learn which scale is relevant

## Common gotchas / interview framings
- *"Why can't we just stack VGG layers deeper?"* Vanishing gradients + degradation problem: even with batch norm, deeper VGG plateaus or regresses
- *"ResNets work because...?"* Skip connections change optimization landscape. Easier to learn near-identity maps (residuals small) than gross transformations
- *"Inception module seems complex. Is it better than ResNet?"* No; ResNet is simpler and performs similarly or better. Inception served as proof-of-concept for multi-scale fusion
- *"Can I use ResNet for dense tasks (segmentation)?"* Yes, but striding must be reduced or dilated convs used to preserve spatial resolution
- *"Why 1×1 convolutions in ResNet?"* Bottleneck reduces dimension by 4×, making the 3×3 cheaper. Learned low-rank decomposition

## See also
- [[skip-connections]]
- [[gradient-flow]]
- [[bottleneck-block]]
- [[residual-learning]]
- [[depthwise-separable-conv]]

## Sources
See frontmatter `sources:`.
