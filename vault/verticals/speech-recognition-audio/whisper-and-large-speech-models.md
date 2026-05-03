---
id: deaec39b-c312-4ab7-b609-a35a76fa5659
title: Whisper and large speech models
track: verticals
topic: speech-recognition-audio
difficulty: 3
tags:
- large-models
- multilingual-asr
- zero-shot
- whisper
- speech-pretrain
aliases:
- OpenAI-Whisper
- speech-foundation-models
- robust-asr
sources:
- url: https://openai.com/index/whisper/
  label: Introducing Whisper - OpenAI
- url: https://github.com/openai/whisper
  label: Whisper GitHub Repository
- url: https://huggingface.co/openai/whisper-large-v3
  label: Whisper Large-v3 on Hugging Face
cards:
- id: 7771bb06-1df4-4e04-8fcf-39e14713ebee
  type: flip
  front: What is 'weak supervision' in the context of Whisper training and why is it effective?
  back: 'Whisper was trained on 680,000 hours of web audio with automatically-generated (noisy) captions,
    not manually-labeled data. Weak supervision works because: (1) sheer scale (680K hours) overwhelms
    label noise, (2) diverse data covers many accents/languages/domains, (3) modern neural networks learn
    robust features despite noisy targets. This avoids expensive human annotation.'
- id: 1cb4fb85-9c28-40af-bc9c-438a5c1e656c
  type: flip
  front: Describe the role of task tokens in Whisper (e.g., <|transcribe|>, <|translate|>) and their impact
    on model behavior.
  back: Task tokens are prefix tokens that signal the decoding task to the Whisper decoder. <|transcribe|>
    → transcribe audio in its original language. <|translate|> → transcribe audio and translate to English.
    These tokens are learned during training and direct the shared model to different behaviors; omitting
    them or using wrong tokens degrades performance.
- id: 18ae3f7b-f5a3-4b36-a4d5-2a459e292cab
  type: mcq
  front: Why is Whisper considered 'robust' compared to traditional fine-tuned ASR systems?
  back: Whisper's robustness comes from massive scale and diversity. Training on 680K hours across 99
    languages and diverse acoustic conditions (street noise, music, various accents) naturally improves
    generalization. Traditional systems fine-tuned on clean, domain-specific data are brittle to distribution
    shift; Whisper has seen it all.
  choices:
  - key: a
    text: It uses larger models with more parameters
    correct: false
  - key: b
    text: It was trained on diverse 680K-hour dataset spanning 99 languages, accents, and acoustic conditions;
      this diversity makes it tolerant of background noise, accents, and domain shifts without task-specific
      fine-tuning
    correct: true
  - key: c
    text: It uses external language models
    correct: false
  - key: d
    text: It always achieves lower WER than fine-tuned systems
    correct: false
  - key: e
    text: It uses CTC instead of attention
    correct: false
- id: 2d5928f6-e790-4356-86d0-cb726fbe850c
  type: flip
  front: What is the Whisper model size vs. accuracy trade-off? Mention specific variants.
  back: 'Tiny (~39M params): fastest, lowest accuracy. Base (~74M): good speed-accuracy. Small (~244M):
    moderate latency. Medium (~769M): slower. Large (~1.5B): slowest, highest accuracy (10–20% WER reduction
    vs. v2). Typical production choice: Medium balances accuracy and latency; Large used for offline/batch
    processing.'
- id: a1688349-c924-4587-b8bf-7193f88cba99
  type: flip
  front: Explain Whisper's hallucination problem and when it occurs.
  back: 'Whisper sometimes generates repeated text or fictional output (e.g., transcribing silence as
    repeated words or lyrics). Occurs in: (1) silent/low-SNR segments where model is uncertain, (2) long
    pauses in audio, (3) certain languages with training data artifacts. Mitigation: use logit_filters
    to suppress repetition, or add voice activity detection before transcription.'
---

## Intuition
Whisper is a large-scale, multilingual speech recognition model trained by OpenAI on 680,000 hours of weakly-labeled audio in 99 languages. Unlike typical ASR models fine-tuned on specific languages, Whisper learns to recognize speech across languages and even transcribe code or translate speech to English—all through large-scale unsupervised pretraining (weak supervision from automatic captions).

## Detail
**Whisper architecture:**
- Encoder: convolutional feature extractor + Transformer encoder blocks
- Decoder: Transformer decoder with causal masking, multi-task learning heads
- Input: mel-spectrograms (80 mel-bins, 3000ms context windows)
- Output: text tokens (English + 99 languages) + task tokens (transcribe, translate)
- Task tokens: <|transcribe|>, <|translate|> prefix tokens signal task type

**Training paradigm:**
- Weak supervision: 680K hours from web audio with auto-generated captions (noisy labels)
- Multitask learning: transcription, translation, language identification all in one model
- Encoder-decoder Seq2Seq with cross-entropy loss
- Data scale overcomes label noise; minimal fine-tuning needed

**Model variants:**
- Tiny, Base, Small, Medium, Large (largest ~1.5B parameters)
- Trade-off: larger models slower but significantly more accurate
- Whisper Large-v3 (2023): 10–20% WER reduction vs. v2 through improved pretraining

**Key capabilities:**
- Zero-shot: handles multiple languages and accents without task-specific training
- Robustness: tolerates background noise, accents, technical language better than traditional ASR
- Multilingual: single model for 99 languages; no per-language models needed
- Translate to English: non-English audio → English transcription in one model

**Modern alternatives:**
- wav2vec 2.0: self-supervised on raw waveforms; excellent for low-resource fine-tuning
- Conformer-based models: lighter-weight, optimized for specific languages
- GPT-4o speech models (2025): newer OpenAI models with lower error rates than Whisper

## Common gotchas / interview framings
- **Weak supervision myth:** Whisper doesn't require perfect labels; it learns from noisy web data—explains why fine-tuning is often optional
- **Hallucination:** Whisper sometimes generates repeated or fictional text, especially in silence—known issue, not present in all use cases
- **Multilingual cost:** Using one model for 99 languages means each language's performance may be slightly lower than a language-specific model
- **Task tokens:** Forgetting to prefix with <|transcribe|> or <|translate|> changes behavior; important for reproducibility
- **Inference latency:** Large model is slow; optimization techniques (quantization, distillation) essential for production
- **Comparison:** Whisper is not perfect; GPT-4o-based speech models (2025) have lower error rates but require API access

## See also
- [[whisper]]
- [[large-scale-pretraining]]
- [[multilingual-models]]
- [[zero-shot-transfer]]
- [[weak-supervision]]
- [[wav2vec]]

## Sources
See frontmatter `sources:`.
