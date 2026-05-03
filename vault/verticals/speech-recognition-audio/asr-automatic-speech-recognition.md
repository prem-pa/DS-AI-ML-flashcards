---
id: 84133a7c-837b-4ffd-9890-827dbe092682
title: ASR (Automatic Speech Recognition)
track: verticals
topic: speech-recognition-audio
difficulty: 3
tags:
- speech-models
- sequence-modeling
- encoder-decoder
- end-to-end
- alignment
aliases:
- speech-to-text
- automatic-speech-recognition
- ASR-systems
sources:
- url: https://arxiv.org/html/2510.12827
  label: 'Automatic Speech Recognition in the Modern Era: Architectures, Training, and Evaluation'
- url: https://www.merl.com/publications/docs/TR2017-190.pdf
  label: Hybrid CTC/Attention Architecture for End-to-End Speech Recognition
- url: https://medium.com/intel-student-ambassadors/attention-in-end-to-end-automatic-speech-recognition-9f9e42718d21
  label: Attention in end-to-end Automatic Speech Recognition
cards:
- id: 6daff785-cc1d-4c1f-a21f-a155e381eb48
  type: flip
  front: What is the blank token in CTC and why is it necessary?
  back: The blank token (ε) is a special symbol used to collapse repeated predictions. Sequences [a, ε,
    a] or [a, a, a] both collapse to "aa" because CTC removes blanks and consecutive repeats. Without
    it, CTC couldn't distinguish repeated characters (e.g., "book" would be hard to encode if every 'o'
    frame predicts 'o').
- id: a04eb6f9-1454-48ed-b4f3-1b76e1f8d993
  type: flip
  front: Compare frame-level independence in CTC vs. the autoregressive nature of attention-based ASR.
  back: 'CTC: P(y_t | x) independent of other y predictions given input x; allows parallel frame processing.
    Attention-based: P(y_t | y_1...y_{t-1}, x); autoregressive decoding depends on previous tokens. CTC
    is faster; attention learns linguistic patterns better.'
- id: 13814b6d-c95c-442d-9d4e-5239fa0167f6
  type: mcq
  front: Why do CTC-based ASR systems typically require a strong external language model while attention-based
    systems do not?
  back: 'CTC''s strength (parallel, frame-independent predictions) is also its weakness: it cannot model
    word dependencies, grammatical constraints, or n-gram patterns. AED''s decoder generates tokens sequentially
    conditioned on previous tokens, naturally learning linguistic structure from the training data.'
  choices:
  - key: a
    text: CTC models have lower training data requirements
    correct: false
  - key: b
    text: CTC frame-level predictions are conditionally independent, so it cannot learn linguistic constraints;
      AED's autoregressive decoder learns P(next_token | previous_tokens), capturing language patterns
      implicitly
    correct: true
  - key: c
    text: Attention mechanisms are inherently linguistic
    correct: false
  - key: d
    text: External language models cannot be used with attention-based systems
    correct: false
  - key: e
    text: CTC is older and predates neural language models
    correct: false
- id: 3c3c01c6-0c8d-4daf-9fd6-923f16660bad
  type: flip
  front: Describe the architecture and loss function of a hybrid CTC/Attention model.
  back: 'Shared encoder processes mel-spectrogram → context vectors. Two decoder heads: (1) CTC head:
    frame-wise softmax over tokens, CTC loss supervises frame-label alignment; (2) Attention head: queries
    encoder context, AED loss supervises sequence generation. Total loss = α·L_CTC + (1−α)·L_AED. The
    CTC branch regularizes alignment; attention learns linguistic patterns.'
- id: 3ba0e97e-bee5-4f32-91f7-22da869ea721
  type: flip
  front: What is a Conformer architecture in ASR and what are its key components?
  back: 'Conformer stacks Convolution and Attention layers: (1) Multi-head self-attention for long-range
    dependencies, (2) Feed-forward networks, (3) Depthwise convolutions for local context. Position-wise
    feed-forward before and after. This combination captures both local (conv) and global (attention)
    context efficiently, achieving state-of-the-art results in speech recognition.'
---

## Intuition
Automatic Speech Recognition (ASR) converts audio (mel-spectrograms or waveforms) into text. The core challenge: align variable-length audio frames with variable-length text sequences, and learn both acoustic-to-phonetic mappings and linguistic constraints. Modern ASR uses two main paradigms: CTC (Connectionist Temporal Classification) and attention-based encoder-decoder (AED) models, each with distinct trade-offs.

## Detail
**CTC (Connectionist Temporal Classification):**
- Maps every audio frame to a token (character, phoneme, subword)
- Uses a blank token ("collapse") to handle repetitions: [a, blank, a] → "aa"
- Learns alignment implicitly through loss; frame-level predictions are conditionally independent
- Fast inference (parallel decoding); no language model required
- Weakness: cannot model linguistic constraints; requires strong external LM for competitive performance

**Attention-Based Encoder-Decoder (AED):**
- Encoder: compresses audio frames to context vectors (CNN/RNN/Transformer)
- Decoder: autoregressively generates text, using attention to focus on relevant audio frames
- Learns both acoustic and linguistic patterns (implicit LM from data)
- Flexible alignment; better performance without external LM
- Weakness: slower inference (sequential decoding); prone to attention errors (deletion, insertion)

**Hybrid CTC/Attention:**
- Combines both: CTC branch supervises alignment; attention branch learns linguistic structure
- Shared encoder; joint loss = CTC_loss + attention_loss
- Often outperforms pure CTC or pure AED, especially with limited LM

**Modern architectures:**
- Transducers (RNN-T, Conformer-Transducer): predict output token at each frame, with recurrent state
- Conformers: stack convolution + self-attention blocks for local and global context
- Transformer-based: scaled dot-product attention replaces RNNs for parallelization

## Common gotchas / interview framings
- **Alignment:** CTC assumes monotonic alignment (left-to-right); AED is more flexible but slower
- **LM dependency:** CTC relies heavily on external LM; AED learns implicit LM. Know why
- **Error analysis:** CTC makes few deletions (never skips frames) but more substitutions; AED makes more flexible errors
- **Decoding:** CTC uses greedy or prefix beam search; AED uses beam search with LM rescoring
- **Latency vs. accuracy:** CTC is faster; AED is more accurate—candidate should understand why
- **Multilingual ASR:** Different languages have different optimal architectures; not one-size-fits-all

## See also
- [[ctc]]
- [[attention-mechanism]]
- [[seq2seq]]
- [[encoder-decoder]]
- [[language-model]]
- [[beam-search]]
- [[transducer]]

## Sources
See frontmatter `sources:`.
