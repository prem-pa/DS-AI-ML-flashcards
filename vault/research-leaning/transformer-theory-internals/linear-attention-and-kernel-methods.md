---
id: 5b85768b-5ae9-4983-b90b-24879f0b8e38
title: Linear attention and kernel methods
track: research-leaning
topic: transformer-theory-internals
difficulty: 5
tags:
- linear-attention
- kernel-methods
- computational-complexity
- low-rank-approximation
- efficiency
- expressiveness-trade-off
aliases:
- fast-attention
- kernel-attention
- linear-complexity-attention
sources:
- url: https://arxiv.org/abs/2006.16236
  label: 'Katharopoulos et al. (2020): Transformers are RNNs - Fast Autoregressive Transformers with Linear
    Attention'
- url: https://arxiv.org/abs/2009.14794
  label: 'Tsai et al. (2019): Transformer Dissection - Unified Understanding through Decomposition'
- url: https://arxiv.org/abs/2305.10338
  label: 'Peng et al. (2023): Retentive Networks: Successor to Transformers for Large Language Models'
- url: https://arxiv.org/abs/2401.00554
  label: 'Yang et al. (2024): The Expressive Power of State Space Models and Mamba'
cards:
- id: 8fad4662-63fe-413b-87ce-beff65da4499
  type: flip
  front: Explain why linear attention achieves O(n) complexity. What algebraic property enables this?
  back: 'Linear attention replaces softmax with an explicit kernel $\phi$, then rearranges using associativity:
    $\text{LinAttn} = \frac{\phi(Q)[\phi(K)^T V]}{\phi(Q)[\phi(K)^T \mathbf{1}]}$. The bracketed terms
    $\phi(K)^T V$ and $\phi(K)^T \mathbf{1}$ are computed once in $O(nm)$ (over all tokens), then each
    query (n queries) applies them in $O(m)$. Total: $O(nm)$ instead of $O(n^2)$. Softmax cannot use this
    trick because its normalization is query-dependent.'
- id: bdba15de-84ba-4e60-999e-aea0b84e61e4
  type: mcq
  front: What is the key expressiveness limitation of linear attention kernels compared to softmax?
  back: 'Softmax''s normalization $e^{q \cdot k_j} / \sum_k e^{q \cdot k_k}$ is query-context-dependent:
    it sharpens or broadens based on the magnitudes of all key-value pairs. Linear kernels like $\phi(q)
    \phi(k)$ depend only on the individual query and key, not their context. This rigidity prevents in-context
    learning—the model cannot learn to sharpen attention on task-defining tokens when they appear near
    distractors.'
  choices:
  - key: a
    text: Linear attention kernels have lower rank
    correct: false
  - key: b
    text: Linear attention kernels are context-independent, so they cannot adapt the bandwidth or focus
      based on data statistics, unlike softmax's adaptive normalization
    correct: true
  - key: c
    text: Linear attention kernels cannot express attention over all positions
    correct: false
  - key: d
    text: Softmax has implicit regularization while linear attention does not
    correct: false
- id: 028909b0-25be-4b4a-8978-e9f3f3236a86
  type: flip
  front: Can linear attention approximate softmax? What is the approximation cost?
  back: Yes, exponential kernels $\phi(x) = e^x$ give $\phi(q) \phi(k)^T = e^{q \cdot k}$, which approximates
    $e^{q \cdot k / \sqrt{d}}$ (softmax's kernel). However, the approximation quality is controlled by
    the kernel dimension $m$ (number of random features or polynomial degree). To achieve $\epsilon$-approximation,
    $m = O(d/\epsilon^2)$ or worse, which can be $O(\sqrt{n})$ for meaningful accuracy. This largely defeats
    the linear-complexity advantage.
- id: 3cb3c970-0c2b-4885-bfc4-a4d55acb3314
  type: flip
  front: Why is context-dependence of softmax essential for in-context learning?
  back: 'In-context learning requires the model to adapt its behavior based on in-prompt examples. If
    a key represents a task-defining example and a key represents a distractor, softmax''s normalization
    must *sharpen* on the task key when they appear together. This sharpening is context-dependent: $e^{q
    \cdot k_\text{task}} / (e^{q \cdot k_\text{task}} + e^{q \cdot k_\text{distractor}})$ depends on both
    magnitudes. Linear kernels cannot do this—they assign fixed, data-independent weights. This is why
    linear attention underperforms on tasks requiring task-adaptive attention.'
---

## Intuition

Softmax attention is $O(n^2)$ in sequence length because it computes all pairwise query-key similarities. Linear attention replaces softmax with an explicit kernel function, enabling $O(n)$ complexity by exploiting the associativity of matrix multiplication:

$$\text{LinAttn}_\phi(Q, K, V) = \frac{\phi(Q) [\phi(K)^T V]}{\phi(Q) \mathbf{1}^T}$$

Instead of computing $\text{softmax}(QK^T)V$ (quadratic), we compute $\phi(Q) [\phi(K)^T V]$ (linear) by rearranging the order of operations.

## Detail

The key insight is **associativity of matrix multiplication**. Softmax attention:
$$\text{SoftAttn}(Q, K, V) = \text{softmax}(QK^T/\sqrt{d}) V$$

Cannot be rearranged because the softmax normalization depends on all queries and keys globally. But if we replace softmax with an *explicit, context-independent kernel* $\phi : \mathbb{R}^d \to \mathbb{R}^m$:

$$\text{LinAttn}_\phi(Q, K, V) = \frac{(\phi(Q) \phi(K)^T) V}{\phi(Q) (\phi(K)^T \mathbf{1})}$$

We can now rearrange:
$$= \frac{\phi(Q) [\phi(K)^T V]}{\phi(Q) [\phi(K)^T \mathbf{1}]}$$

The bracketed terms can be computed *once* in $O(n)$ for each sequence, and then each query applies the precomputed result. This is $O(nm)$ where $m$ is the kernel dimension (typically $m \ll n$).

**Kernel choices**:
1. **Exponential kernel**: $\phi(x) = e^{x}$, giving $\phi(q) \phi(k)^T = e^{q \cdot k}$ (approximates softmax)
2. **Polynomial kernel**: $\phi(x) = (1 + x/\sqrt{d})_+^p$ (faster, interpretable)
3. **Random features**: $\phi(x) = e^{i \Omega x}$ for random $\Omega$ (Fourier features)

**Expressiveness trade-off**:
- Softmax kernels are *context-dependent*: the attention weight $e^{q \cdot k / \sqrt{d}} / \sum_j e^{q \cdot k_j / \sqrt{d}}$ adapts to the magnitudes and statistics of all keys in context
- Linear attention kernels are *context-independent*: $\phi(q) \phi(k)^T$ does not depend on other keys, only on the query-key pair
- This loses the adaptive bandwidth of softmax, limiting expressiveness

**Practical implications** (Katharopoulos et al., 2020; Peng et al., 2023):
- Linear attention performs comparably on some tasks (length modeling, synthetic patterns)
- But underperforms on tasks requiring context-adaptive attention (long-range dependencies, rare pattern matching)
- Recent work (Mamba, Retentive Networks) combines linear attention's efficiency with other mechanisms (e.g., selective scan) to recover expressiveness

## Common gotchas / interview framings

- **"Why not use linear attention everywhere? It's faster!"** Because it sacrifices expressiveness for speed. Softmax's context-dependent normalization is crucial for in-context learning and rare-pattern detection. Linear attention kernels are too rigid
- **"Can linear attention approximate softmax?"** Yes, but you need more kernel dimensions. Exponential kernels like $e^{q \cdot k}$ approximate $e^{q \cdot k / \sqrt{d}}$ (softmax scaled), but the quality depends on the approximation rank $m$. For exact softmax, $m = O(\sqrt{n})$ or worse—defeating the speed advantage
- **"How does this relate to state-space models?"** State-space models (Mamba, S4) use linear-time recurrent updates but retain selectivity (context-dependent gating). They're a middle ground: faster than full softmax, more expressive than fixed kernels
- **"Why is context-dependence crucial for transformers to work?"** See [[attention-as-kernel-method]]. Softmax's normalization $\sum_j e^{q \cdot k_j}$ creates a *data-adaptive* kernel that sharpens on task-relevant keys and broadens on noise. This adaptivity is essential for in-context learning, where the model must learn to focus on task-defining examples

## See also
- [[attention-as-kernel-method]]
- [[efficient-attention-sparse-local]]
- [[theoretical-analysis-of-softmax-bottleneck]]
- [[implicit-regularization-in-transformers]]
- [[gradient-flow-in-deep-transformers]]

## Sources
See frontmatter `sources:`.
