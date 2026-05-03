---
id: ab43594c-d181-440c-8d67-980386c64d95
title: Causal masking and autoregressive generation
track: ai-llm-engineer
topic: large-language-model-architectures
difficulty: 3
tags:
- causal-attention
- autoregressive
- left-to-right-generation
- masking
- decoder-only
- next-token-prediction
aliases:
- causal-mask
- left-to-right
- AR-generation
- future-masking
sources:
- url: https://arxiv.org/abs/1706.03762
  label: Attention Is All You Need (Vaswani et al. 2017)
- url: https://arxiv.org/abs/1810.04805
  label: 'BERT: Pre-training of Deep Bidirectional Transformers (Devlin et al. 2018)'
- url: https://github.com/openai/gpt-2/blob/master/src/model.py
  label: GPT-2 Reference Implementation
cards:
- id: 85cc74c5-a5cf-4d51-883a-3b2a71d95211
  type: flip
  front: What is causal masking and why is it necessary in autoregressive LLMs?
  back: Causal masking prevents attention to future tokens by setting future logits to $-\infty$ before
    softmax. It's necessary because autoregressive generation predicts tokens left-to-right, and the model
    can't look ahead. At inference, future tokens don't exist yet; at training, we enforce train-test
    consistency by never exposing them.
- id: dcc90610-f3b5-451e-a27b-a226738d7e3b
  type: flip
  front: How does causal masking enable parallel training while preserving autoregressive inference?
  back: During training, apply causal mask to the full sequence. Softmax over all positions simultaneously
    with masked-out future positions → you compute loss for all tokens in parallel (quadratic in sequence
    length, but still faster than sequential generation). At inference, generate one token at a time,
    recompute attention with growing context.
- id: 35020df6-7758-433a-9ee5-dbc06458e7f9
  type: mcq
  front: In a causal-masked attention computation, what happens to logits for positions $j > i$ when position
    $i$ attends?
  back: 'Setting future logits to $-\infty$ ensures softmax converts them to ~0 probability. This is differentiable:
    the gradient is well-defined (logits are never exactly $-\infty$ in practice, just very negative).
    Other options either break the model''s ability to learn or violate the autoregressive property.'
  choices:
  - key: a
    text: They are scaled by a learnable weight parameter
    correct: false
  - key: b
    text: They are set to $-\infty$ before softmax, causing near-zero attention weight
    correct: true
  - key: c
    text: They are zeroed out and have no gradient contribution
    correct: false
  - key: d
    text: They are replaced with the average of all past position logits
    correct: false
- id: 4cea6e2d-c179-4585-bd14-79ed01b40b51
  type: flip
  front: Contrast the training efficiency of a causal-masked decoder vs. naive sequential token generation
    during training.
  back: 'Causal masking: $O(n^2)$ complexity (n² pairwise attentions for n tokens), but all computed in
    one pass → high hardware utilization. Naive sequential: must do n forward passes (one per token),
    each computing attention from scratch → n× slower. Causal masking is why modern LLM training is feasible
    at scale.'
---

## Intuition

Causal masking prevents tokens from attending to future positions in the sequence. In autoregressive generation, the model predicts the next token given all previous tokens—it must never "cheat" by looking ahead. Imagine a person reading left-to-right, making a prediction about the next word: they can't see what comes after. Causal masking enforces this constraint in the attention mechanism.

## Detail

In a standard attention layer, the softmax over query-key similarities can attend to any position. Causal masking applies a binary mask $M$ where $M_{i,j} = 0$ (true) if $j \leq i$ and $M_{i,j} = -\infty$ (masked) if $j > i$. This is implemented in the attention logits before softmax:

$$\text{Attention}(Q, K, V) = \text{softmax}\left(\frac{QK^T}{\sqrt{d_k}} + M\right)V$$

During training, causal masking allows parallel computation of all token predictions simultaneously while maintaining the autoregressive property. At inference, you generate left-to-right: feed prompt, sample token $n$, append to context, repeat.

Key architectural patterns:
- **Decoder-only** (GPT, Llama, Mistral): Every layer uses causal attention
- **Training efficiency**: Parallel loss computation over all positions via causal masking
- **Inference pattern**: Sequential generation; each forward pass predicts one token

## Common gotchas / interview framings

- **"Why can't we just use bidirectional attention?"** → During training we could, but at inference you have no future tokens. Causal masking ensures train-test consistency.
- **"Doesn't causal masking hurt performance?"** → Yes, compared to bidirectional models on understanding tasks, but it's the only option for autoregressive generation. [[decoder-only-vs-encoder-decoder-tradeoff]] explores this tradeoff.
- **Prompt length matters**: Longer prompts = more context to attend over = better predictions (but also higher inference cost).
- **Efficiency folklore**: "We can parallelize training with causal masking." True, but inference remains sequential at generation time.

## See also
- [[decoder-only-vs-encoder-decoder-tradeoff]]
- [[gpt-llama-mistral-qwen-architectures]]
- [[scaling-laws-for-loss-and-compute]]

## Sources
See frontmatter `sources:`.
