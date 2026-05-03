---
id: a01de269-a1c1-4a1a-a5bd-ec2c26bcc353
title: LSTM/GRU for sequential modeling
track: verticals
topic: time-series-forecasting
difficulty: 3
tags:
- RNN
- LSTM
- GRU
- sequential
- deep-learning
- gradient-flow
aliases:
- recurrent neural networks
- long short-term memory
- gated recurrent unit
- sequence-to-sequence
sources:
- url: https://en.wikipedia.org/wiki/Long_short-term_memory
  label: LSTM on Wikipedia
- url: https://colah.github.io/posts/2015-08-Understanding-LSTMs/
  label: Understanding LSTMs (Colah)
- url: https://arxiv.org/abs/1406.1078
  label: Empirical Evaluation of Gated RNNs (Cho et al.)
- url: https://keras.io/api/layers/recurrent_layers/lstm/
  label: Keras LSTM API
cards:
- id: f039985b-3c6f-4067-accf-797a01b18938
  type: flip
  front: What problem do LSTM gates solve that standard RNNs face?
  back: 'Standard RNNs suffer from vanishing gradients: ∂L/∂h_0 involves products of Jacobians ≤ 1, shrinking
    exponentially backward. LSTM gates (forget, input, output) learn multiplicative masks allowing gradients
    to flow relatively unchanged through the cell state path. This enables learning dependencies across
    hundreds of steps without gradient collapse.'
- id: 426ecd47-1600-403b-bb8c-b056fb782b42
  type: mcq
  front: You train an encoder-decoder LSTM for 30-step ahead forecasting. During training you use ground
    truth values; during inference you feed predictions back. What issue does this cause?
  back: 'Training on ground truth (teacher forcing) gives a different input distribution than inference
    (autoregressive predictions). The model doesn''t learn to recover from its own mistakes. Solutions:
    scheduled sampling (gradually replace ground truth with predictions), stochastic feeding, or training
    with predictions from the start.'
  choices:
  - key: a
    text: The model trains faster
    correct: false
  - key: b
    text: 'Exposure bias: model never sees its own errors during training, distribution mismatch at inference'
    correct: true
  - key: c
    text: The model cannot learn seasonality
    correct: false
  - key: d
    text: No issue; this is standard practice
    correct: false
- id: 49e4bb8e-d613-4ebe-a06a-da401079274d
  type: flip
  front: How does GRU differ from LSTM? When would you choose GRU?
  back: 'GRU has 3 gates (reset, update, candidate) vs. LSTM''s 4 (forget, input, output, candidate) and
    no separate cell state. GRU is simpler, faster to train, and uses fewer parameters (≈75% of LSTM).
    Choose GRU when: computational budget tight, sequence length moderate (<300 steps), or empirical comparison
    shows comparable performance. LSTM often better for very long sequences or when expressiveness critical.'
- id: d4092220-25a7-4a94-adc2-db17ad0999e0
  type: flip
  front: Why have Transformers largely displaced LSTMs for time series forecasting?
  back: Transformers use self-attention (O(n²) with parallelizable) vs. LSTM recurrence (O(n) sequential).
    On GPUs, Transformers train 10-100x faster and scale to longer sequences. Attention directly models
    long-range dependencies without gradient flow issues. However, LSTMs remain useful for online (streaming)
    forecasting where you can't afford quadratic memory, and for small datasets where Transformers may
    overfit.
---

## Intuition
LSTM (Long Short-Term Memory) and GRU (Gated Recurrent Unit) are RNN variants designed to handle long-range dependencies in sequences by introducing gating mechanisms. Standard RNNs suffer from vanishing/exploding gradients during backpropagation; gates learn what information to keep and discard, enabling learning of patterns across hundreds of time steps.

## Detail
**LSTM Architecture:**
- **Forget gate**: σ(W_f · [h_{t-1}, x_t] + b_f)—controls what to discard from cell state
- **Input gate**: σ(W_i · [h_{t-1}, x_t] + b_i)—controls what new info to store
- **Cell state**: C_t = f_t ⊙ C_{t-1} + i_t ⊙ tanh(W_c · [h_{t-1}, x_t] + b_c)—memory carrier
- **Output gate**: σ(W_o · [h_{t-1}, x_t] + b_o)—controls what to output as hidden state
- **Hidden state**: h_t = o_t ⊙ tanh(C_t)

**GRU (Simpler Alternative):**
- Combines forget and input gates into single "update gate" → fewer parameters
- Has reset gate controlling info flow from previous state
- Often comparable performance to LSTM with faster training

**Time Series Forecasting Usage:**
1. Encoder-decoder: encode historical context, decode future steps
2. Many-to-one: use entire sequence to predict next step (autoregressive)
3. Many-to-many: predict multiple steps ahead (multistep forecasting)
4. Stacked LSTMs for deeper feature learning

## Common gotchas / interview framings
- LSTM memory capacity fixed by hidden size; may bottleneck on very long sequences (>500 steps)
- Gating mechanisms don't prevent all gradient issues; careful initialization and learning rate still needed
- Autoregressive LSTM forecasting (feeding predictions back as input) can accumulate error over horizon
- Teacher forcing (using ground truth during training) masks prediction error; exposure bias during inference
- Vanishing gradient partially solved by LSTM but not eliminated; attention mechanisms often more effective
- GRU faster than LSTM but slightly less expressive; choice is empirical
- Transformers (parallel, better long-range) increasingly preferred over LSTM for modern forecasting
- RNNs poorly parallelizable vs. Transformers; slow on large datasets

## See also
- [[vanishing-gradient-problem]]
- [[gating-mechanisms]]
- [[hidden-state]]
- [[cell-state]]
- [[forget-gate]]
- [[bptt]]
- [[attention-mechanisms]]
- [[seq2seq]]

## Sources
See frontmatter `sources:`.
