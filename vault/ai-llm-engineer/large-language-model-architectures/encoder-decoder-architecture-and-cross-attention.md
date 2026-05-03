---
id: bd77ed98-0c5f-4d5e-921d-69759df7db4d
title: Encoder-decoder architecture and cross-attention
track: ai-llm-engineer
topic: large-language-model-architectures
difficulty: 3
tags:
- encoder-decoder
- cross-attention
- seq2seq
- bidirectional
- source-target
- attention-mechanism
aliases:
- seq2seq
- cross-attention
- encoder-decoder
- source-target
sources:
- url: https://arxiv.org/abs/1706.03762
  label: Attention Is All You Need (Vaswani et al. 2017)
- url: https://arxiv.org/abs/1409.0473
  label: Sequence to Sequence Learning with Neural Networks (Sutskever et al. 2014)
- url: https://huggingface.co/blog/encoder-decoder
  label: 'Hugging Face: Understanding Encoder-Decoder Models'
cards:
- id: babce0b7-9ba9-4a34-8936-e6305db0d7a7
  type: flip
  front: Explain the difference between self-attention in the encoder vs. cross-attention in the decoder.
  back: 'Encoder self-attention: bidirectional; each token attends to all input tokens (no causal mask).
    Cross-attention: decoder positions query the encoder''s keys/values; it''s how the decoder "looks
    up" relevant input information. Decoder also has its own self-attention (causal) to attend to previously
    generated tokens.'
- id: 3405b692-e5d3-4c06-82da-2b20650142fe
  type: flip
  front: In the cross-attention layer, what are the sources of queries, keys, and values?
  back: Queries (Q) come from the decoder's hidden states (what we're generating now). Keys (K) and Values
    (V) come from the encoder's output (the full input representation). The encoder processes the input
    once and its outputs are reused as K, V for all decoder steps.
- id: 46b47bb6-4989-45e7-9769-3ccc9055ee74
  type: mcq
  front: Why is the encoder's bidirectional attention crucial for encoder-decoder models?
  back: Bidirectional attention lets the encoder see the full input context, capturing dependencies that
    flow both forward and backward. This is safe because we're not generating tokens yet—we're understanding
    input. The decoder's autoregressive (causal) generation is conditioned on this rich bidirectional
    representation.
  choices:
  - key: a
    text: It allows the encoder to predict future tokens
    correct: false
  - key: b
    text: It enables the encoder to capture long-range dependencies in the input without causal constraints
    correct: true
  - key: c
    text: It reduces the number of parameters needed in the decoder
    correct: false
  - key: d
    text: It allows the encoder to generate multiple outputs in parallel
    correct: false
- id: 208fda96-6d77-4d2f-83e0-10be2487bd4b
  type: flip
  front: During encoder-decoder inference, the encoder is computed once per input. Why is reusing encoder
    outputs (as K, V in cross-attention) efficient?
  back: The encoder output is static for a given input. Cross-attention uses this fixed (K, V) pair for
    every decoder step (generating each output token). If we recomputed the encoder for each generated
    token, we'd waste compute. Modern deployments cache encoder (K, V) to avoid recomputation; this is
    critical for serving [[inference-scaling-and-test-time-compute-o1-reasoning-models]].
---

## Intuition

Encoder-decoder architecture explicitly separates input processing from output generation. The encoder reads the full input bidirectionally, building rich representations. The decoder generates tokens one at a time, conditioning on both its own prior outputs (causal attention) and the encoder's representation (cross-attention). Think of it as: encoder = understanding, decoder = speaking.

## Detail

**Architecture:**
```
Input sequence → [Encoder blocks: self-attention (bidirectional)]  
                    ↓
              Encoder output (context vectors)
                    ↓
Target prefix → [Decoder blocks: self-attention (causal) + cross-attention to encoder]
                    ↓
                Predicted next token
```

**Encoder:**
- $N$ stacked transformer layers with **bidirectional** (non-causal) self-attention
- Each position attends to all other positions; no masking
- Produces key-value tensors $(K_e, V_e)$ passed to decoder

**Decoder:**
- $N$ stacked transformer layers, each with two attention sub-layers:
  1. **Self-attention (causal):** Attends to own previous outputs; standard causal mask
  2. **Cross-attention:** Queries from decoder states; keys/values from encoder outputs

**Cross-attention mechanism:**
$$\text{CrossAttention}(Q_d, K_e, V_e) = \text{softmax}\left(\frac{Q_d K_e^T}{\sqrt{d_k}}\right)V_e$$

Where:
- $Q_d$: Decoder hidden states (queries)
- $K_e, V_e$: Encoder hidden states (fixed keys/values for the whole sequence)

Cross-attention allows the decoder to "look up" relevant parts of the encoded input for each generated token.

**Why it works:**
- Encoder bidirectional attention captures long-range dependencies in input (no causal constraint)
- Decoder can focus on relevant input regions via cross-attention without reprocessing
- Asymmetry mirrors problem structure: understand input fully, then generate output

## Common gotchas / interview framings

- **"Encoder-decoder is slower because of two stacks."** → Not really; encoder runs once, decoder runs autoregressively. Per-token cost similar to decoder-only if depths are comparable.
- **Cross-attention optimization:** Keys/values are from encoder (fixed), reuse them across all decoder predictions → no need to recompute encoder for each token.
- **Training vs. inference:** Training uses teacher forcing (feed gold target tokens to decoder); inference generates auto-regressively (decoder output feeds back as input).
- **Compare to decoder-only:** Decoder-only attends over [prompt, all-generated-so-far]; encoder-decoder attends over [encoded-input, generated-so-far] separately.

## See also
- [[decoder-only-vs-encoder-decoder-tradeoff]]
- [[t5-and-bart-models]]
- [[causal-masking-and-autoregressive-generation]]

## Sources
See frontmatter `sources:`.
