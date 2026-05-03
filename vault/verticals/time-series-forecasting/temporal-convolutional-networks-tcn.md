---
id: 76c100b5-7cd8-4769-b96c-c6e89b887130
title: Temporal convolutional networks (TCN)
track: verticals
topic: time-series-forecasting
difficulty: 3
tags:
- TCN
- convolution
- causal
- dilated
- receptive-field
- deep-learning
aliases:
- dilated convolution
- causal convolution
- WaveNet
- temporal CNN
sources:
- url: https://arxiv.org/abs/1803.01271
  label: An Empirical Evaluation of Generic Convolutional and Recurrent Networks (Bai et al.)
- url: https://arxiv.org/abs/1612.08083
  label: 'WaveNet: A Generative Model for Raw Audio (van den Oord et al.)'
- url: https://github.com/locuslab/TCN
  label: TCN GitHub Implementation
- url: https://towardsdatascience.com/temporal-convolutional-networks-for-time-series-forecasting-5f23f2d75ce0
  label: TCN for Time Series Forecasting (Towards Data Science)
cards:
- id: 18e4693d-dc90-436c-ab03-18b2f8f9f3fb
  type: flip
  front: What is causal convolution and why is it essential for time series forecasting?
  back: 'Causal convolution pads input on the left (past) and masks on the right (future), ensuring output
    at time t depends only on t and earlier. Without it, the model would access future values during training,
    causing information leakage and poor generalization. Mathematically: pad left by k-1, apply k-width
    filter to positions [t-k+1, t], mask future steps to -∞.'
- id: a950183f-7e42-4bba-a2a6-85236f41d06a
  type: mcq
  front: In a TCN with 8 layers and dilation rates [2^0, 2^1, ..., 2^7], what is approximately the receptive
    field?
  back: 'With kernel size 3 and exponential dilation doubling each layer, receptive field grows exponentially:
    R_i ≈ 2^(i+1). At layer 8, R ≈ 2^9 = 512. This allows capturing dependencies hundreds of steps back
    with only 8 layers and ~3k parameters, far more efficient than fully-connected or RNN approaches.'
  choices:
  - key: a
    text: 8 (number of layers)
    correct: false
  - key: b
    text: 256 (sum of dilations)
    correct: false
  - key: c
    text: ~256-512 depending on kernel size 3
    correct: true
  - key: d
    text: Infinite (attends all history)
    correct: false
- id: 410d13c8-dbf2-4e25-a41c-ec1aeed6af16
  type: flip
  front: How does TCN compare to LSTM for time series forecasting in terms of speed and interpretability?
  back: 'TCN: fully parallelizable (fast on GPU), interpretable receptive field (know how far back each
    output looks). LSTM: sequential (slow), black-box (hard to know which time steps mattered). Trade-off:
    TCN requires entire history in memory; LSTM can process streaming. For batch forecasting, TCN usually
    5-10x faster on GPU. For online/streaming, LSTM better.'
- id: d8ee4e5e-4866-45f5-8952-8b9253231e52
  type: flip
  front: What happens if a TCN's receptive field is smaller than the longest temporal dependency in the
    data?
  back: 'The model cannot access necessary historical context, underfitting the data. For example, if
    yearly seasonality needs ~365 steps back but receptive field is only 100, the model cannot learn the
    seasonal pattern. Solution: increase depth, kernel size, or adjust dilation schedule. This is a key
    design trade-off: larger receptive field needs more layers but also more memory and parameters.'
---

## Intuition
TCNs apply 1D convolutions with dilation (gaps between kernel elements) to capture temporal patterns at multiple scales. Causal masking ensures each output depends only on past inputs (no lookahead). Stacking dilated layers exponentially expands receptive field, enabling capture of long-range dependencies while keeping parameters low and computation highly parallelizable.

## Detail
**Key Components:**

1. **Causal Convolution:**
   - Padding: zero-pad left (past), mask right (future)
   - Ensures y_t depends only on {y_{t-k}, ..., y_{t-1}, x_t} for kernel size k
   - Critical for valid forecasting without lookahead bias

2. **Dilated Convolution:**
   - Dilation rate d: sample inputs with spacing d
   - Layer i typically uses dilation d=2^i: exponential growth
   - Receptive field R_i = 1 + 2(kernel_size - 1) × (2^i - 1) / (2 - 1)
   - With 8 layers, kernel size 3: receptive field ≈2000 steps

3. **Residual Blocks:**
   - Each block: Dilated Conv → ReLU → 1×1 Conv → Skip connection
   - Skip connections stabilize training and enable very deep networks
   - Often include gating: output = gate ⊙ conv + skip

4. **Architecture:**
   - Stack blocks with increasing dilation
   - Optionally add context attention or external regressors
   - Output layer: dense or skip connections to reconstruct forecast

**Advantages:**
- Parallel computation: all outputs computed in one forward pass
- Interpretable receptive field
- Fewer parameters than LSTM
- Easily handle multivariate inputs and external regressors

## Common gotchas / interview framings
- Receptive field must exceed longest temporal dependency; under-parameterization hurts accuracy
- Memory still quadratic in sequence length (all time steps in GPU memory) unlike true online processing
- Causal masking critical; bug in masking leaks information and inflates accuracy
- Dilation schedule (2^i) is one choice; other schedules can work but 2^i is standard
- Very deep TCNs (>10 layers) need careful initialization and normalization
- TCNs beat LSTMs/GRUs on many benchmarks but Transformers competitive on speed
- TCNs less suitable for online/streaming forecasting (need full history)
- WaveNet (generative model) motivated TCNs; similar architecture but different loss (likelihood vs. MSE)

## See also
- [[dilated-convolution]]
- [[receptive-field]]
- [[causal-masking]]
- [[residual-blocks]]
- [[skip-connections]]
- [[wavenet-architecture]]

## Sources
See frontmatter `sources:`.
