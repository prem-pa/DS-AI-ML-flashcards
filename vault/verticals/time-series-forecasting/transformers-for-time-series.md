---
id: 47309c8b-1b2e-4d17-8e8a-4afea32dfbca
title: Transformers for time series
track: verticals
topic: time-series-forecasting
difficulty: 3
tags:
- transformer
- attention
- self-attention
- multihead
- deep-learning
- efficiency
aliases:
- Temporal Fusion Transformer
- TFT
- Informer
- Autoformer
- seq2seq with attention
sources:
- url: https://arxiv.org/abs/1706.03762
  label: Attention is All You Need (Vaswani et al.)
- url: https://arxiv.org/abs/1907.01332
  label: Temporal Fusion Transformers (Lim et al.)
- url: https://arxiv.org/abs/2012.07436
  label: Informer (Zhou et al.)
- url: https://arxiv.org/abs/2202.07125
  label: Autoformer (Wu et al.)
cards:
- id: 41a0fed5-63e8-4c1e-a84c-618b28a7d702
  type: flip
  front: Why is self-attention particularly useful for time series forecasting compared to RNNs?
  back: 'Self-attention computes weights for all pairs (i, j) in parallel, allowing the model to directly
    compare time step t with any past step without sequential dependency. RNNs must propagate information
    step-by-step, suffering gradient decay. Attention: O(L²) but parallelizable on GPUs. RNN: O(L) sequential.
    For L>100, Transformer typically much faster.'
- id: bb0322cf-1cda-43df-8594-e09cc9f5c8de
  type: mcq
  front: In a Transformer encoder-decoder for time series, what is the purpose of lookahead (causal) masking
    in the decoder?
  back: Without causal masking, the decoder can 'cheat' by looking at future ground truth during training
    (information leakage). Masking sets attention scores to -∞ for future positions, forcing autoregressive
    prediction. This ensures the model generalizes to inference where future values are unknown.
  choices:
  - key: a
    text: To speed up training
    correct: false
  - key: b
    text: To prevent the decoder from attending to future ground truth tokens during training
    correct: true
  - key: c
    text: To reduce memory usage
    correct: false
  - key: d
    text: To improve robustness to outliers
    correct: false
- id: e52d63eb-2515-4560-874d-018e672f05c8
  type: flip
  front: What is the key difference between Informer and standard Transformers, and why does it matter?
  back: 'Informer uses ProbSparse attention (O(L log L)) instead of full attention (O(L²)) by sampling
    the most important keys and sparsifying attention. This enables handling sequences >1000 steps without
    excessive memory. Trade-off: sparse attention may miss some dependencies. Use Informer for very long
    contexts; standard Transformer usually better for <500 steps.'
- id: da72685c-4644-46d2-8b6e-1436022861ca
  type: flip
  front: How does Temporal Fusion Transformer (TFT) achieve interpretability in time series forecasting?
  back: 'TFT uses variable selection networks (attention masks) and gating mechanisms at the input and
    intermediate levels. Each output gate learns to combine temporal and static features; attention weights
    show contribution of each variable and time step. This lets practitioners see which variables drove
    predictions, unlike black-box Transformers. Trade-off: more components and complexity.'
---

## Intuition
Transformers use self-attention to directly compute relationships between all time steps in parallel, avoiding the sequential bottleneck of RNNs. Attention weights learn which past observations matter most for predicting each future step, naturally capturing long-range dependencies and seasonality.

## Detail
**Self-Attention:**
Attention(Q, K, V) = softmax(QK^T / √d_k)V
- Queries (Q): what info are we looking for
- Keys (K): what info is available
- Values (V): info content
- Attention weights normalize dot products to focus on relevant time steps

**Multihead Attention:**
- Run h parallel attention heads with different learned projections
- Concatenate outputs: MultiHead(Q,K,V) = Concat(head_1, ..., head_h)W^O
- Different heads learn different temporal patterns (trend, seasonality, noise)

**Key Architectures for Time Series:**

1. **Temporal Fusion Transformer (TFT):**
   - Static covariate encoders for context
   - Variable selection networks (attention-based) to filter irrelevant inputs
   - Gating mechanisms at multiple levels
   - Interpretable: see which variables/time steps matter

2. **Informer:**
   - Sparse attention (ProbSparse) to reduce O(L²) complexity to O(L log L)
   - Distilling: compress encoder outputs before feeding to decoder
   - Suitable for very long sequences (>1000 steps)

3. **Autoformer:**
   - Explicit decomposition: trend and seasonal components
   - Attention applied to each component separately
   - Better interpretability and sometimes better accuracy

**Training Tips:**
- Positional encoding: add sinusoidal or learnable position embeddings to break permutation invariance
- Normalization: layer norm crucial for stable training
- Warmup: gradual learning rate increase helps
- Lookahead bias: ensure masks prevent seeing future

## Common gotchas / interview framings
- Quadratic memory in sequence length; long sequences (>2000) may exceed GPU memory
- Attention weights can be unintuitive to interpret; high weights don't always indicate causality
- Positional encoding matters; weak encoding can lose temporal information
- Over-parameterized; needs regularization/early stopping on small datasets
- Sparse variants (Informer) useful but add complexity; baseline Transformer often sufficient
- Seasonal patterns require careful encoding (e.g., Autoformer's decomposition helps)
- Foundation models (Chronos, Moirai) use Transformers as backbone, often as decoder-only

## See also
- [[self-attention]]
- [[multihead-attention]]
- [[positional-encoding]]
- [[temporal-encoding]]
- [[sparse-attention]]
- [[linear-attention]]
- [[tft-architecture]]
- [[informer]]

## Sources
See frontmatter `sources:`.
