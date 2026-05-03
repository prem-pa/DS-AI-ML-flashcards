---
id: 99bd2a3d-d6f0-4dbf-8d37-ae28ddef1ecc
title: Decoder-only vs encoder-decoder tradeoff
track: ai-llm-engineer
topic: large-language-model-architectures
difficulty: 3
tags:
- architecture-design
- decoder-only
- encoder-decoder
- seq2seq
- bidirectional-context
- parameter-efficiency
aliases:
- decoder-only
- encoder-decoder
- seq2seq-models
- architectural-choice
sources:
- url: https://arxiv.org/abs/1706.03762
  label: Attention Is All You Need (Vaswani et al. 2017)
- url: https://arxiv.org/abs/1910.13461
  label: 'ELECTRA: Pre-training Text Encoders as Discriminators (Clark et al. 2020)'
- url: https://huggingface.co/blog/encoder-decoder
  label: 'Hugging Face: Encoder-Decoder Models'
cards:
- id: 75ef8d27-986d-43c3-b593-c1f94fee1e0b
  type: flip
  front: What is the core architectural difference between decoder-only and encoder-decoder models?
  back: 'Decoder-only: single causal stack (all tokens attend left-to-right). Encoder-decoder: encoder
    has bidirectional attention over input; decoder is autoregressive with cross-attention to encoder
    outputs. Decoder-only is simpler (single path); encoder-decoder explicitly separates input understanding
    from output generation.'
- id: 1b077020-ff36-49c9-b7a4-9fa069387830
  type: flip
  front: Why do decoder-only models dominate frontier LLMs (GPT-5, Claude 4.7, Gemini 3) despite encoder-decoder
    models' bidirectional advantage?
  back: Decoder-only is simpler (single unified architecture), scales more efficiently (all parameters
    devoted to generation), and works better with large-scale unsupervised pretraining on plain text.
    Encoder-decoder's bidirectional advantage is marginal in large models where scale provides equivalent
    or better reasoning. Decoder-only's unified architecture also enables better in-context learning.
- id: 1ba69760-2616-4c57-9c55-ce2277942231
  type: mcq
  front: In an encoder-decoder model, which component is responsible for bidirectional understanding of
    the input?
  back: 'The encoder explicitly computes bidirectional attention (no causal mask) over input tokens. The
    decoder is autoregressive (causal attention to its own outputs) but attends to encoder outputs via
    cross-attention, which are already bidirectionally contextualized. This design separates concerns:
    input understanding (encoder) vs. generation (decoder).'
  choices:
  - key: a
    text: The decoder's causal attention layers
    correct: false
  - key: b
    text: The encoder's bidirectional self-attention layers
    correct: true
  - key: c
    text: The cross-attention layer bridging encoder and decoder
    correct: false
  - key: d
    text: A separate bidirectional preprocessing module
    correct: false
- id: e83f0984-9798-4dd8-be54-e87d81a35d17
  type: flip
  front: For a given parameter budget N, how does a decoder-only model allocate parameters vs. an encoder-decoder
    model?
  back: 'Decoder-only: all N params in the single causal generation stack. Encoder-decoder: splits into
    encoder (~N/2), decoder (~N/2), plus cross-attention bridges. This means a decoder-only model has
    2× the depth/width in its single generation path, potentially better for scaling laws [[scaling-laws-for-loss-and-compute]].'
- id: ade04090-0705-454d-924c-d348b9e83118
  type: flip
  front: Name a task where encoder-decoder's bidirectional design provides a clear advantage over decoder-only.
  back: Machine translation or abstractive summarization with explicit reference documents. The encoder
    can bidirectionally understand the source sentence or reference doc all at once, then the decoder
    generates conditioned on that rich representation. Decoder-only must generate while reading left-to-right,
    which can miss backward dependencies in the input.
---

## Intuition

Decoder-only models (GPT-style) read the prompt and generate left-to-right in one pass. Encoder-decoder models (T5-style) explicitly separate input understanding (encoder) from output generation (decoder). The tradeoff: decoder-only is simpler and more parameter-efficient for pure generation; encoder-decoder excels when you need bidirectional reasoning over input.

## Detail

**Decoder-only (autoregressive, causal attention):**
- Single stack of layers; each layer attends to previous tokens only (causal mask)
- Token generation: prompt → [predict token 1, append, predict token 2, append, ...]
- Parameters: All devoted to next-token prediction
- Example: GPT-2, Llama, Mistral, Claude

**Encoder-decoder (seq2seq with cross-attention):**
- Encoder: Bidirectional attention over input; processes full context
- Decoder: Autoregressive, attends to own previous outputs + encoder outputs (cross-attention)
- Examples: Transformer, T5, BART, mT5

**Parameter comparison:**
For same total params $N$:
- Decoder-only: All $N$ parameters in generation path
- Encoder-decoder: Split into encoder ($N/2$) and decoder ($N/2$) plus cross-attention layers

Decoders-only dominates modern LLMs because:
1. Single architecture path = simpler code, training, serving
2. Everything learned for generation; encoder parameters not reused for context understanding
3. Works well with large-scale pretraining on unsupervised text

Encoder-decoder advantages:
- Bidirectional context over input helps constrained tasks (translation, summarization with specific refs)
- Better for long-context reasoning over structured input
- Can freeze encoder, fine-tune decoder for transfer learning

## Common gotchas / interview framings

- "Encoder-decoder is 'better' because bidirectional." → True for some tasks, but decoder-only scales better to frontier model sizes.
- "Decoder-only can't do bidirectional reasoning." → Unfair; it can, just during full prompt processing, not in layers. At inference, prompt is processed bidirectionally (all prompt tokens available).
- **In-context learning:** Decoder-only excels because the prompt is pure context; no architectural asymmetry.
- **Modern trend:** 2025 frontier models (GPT-5.5, Claude 4.7, Gemini 3) are all decoder-only with [[causal-masking-and-autoregressive-generation]].

## See also
- [[causal-masking-and-autoregressive-generation]]
- [[gpt-llama-mistral-qwen-architectures]]
- [[t5-and-bart-models]]
- [[frontier-model-performance-gpt-4-claude-37-gemini-3]]

## Sources
See frontmatter `sources:`.
