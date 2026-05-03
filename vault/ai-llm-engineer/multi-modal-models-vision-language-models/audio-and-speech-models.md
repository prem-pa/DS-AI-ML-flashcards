---
id: 7084ba49-5c72-4d87-9a2a-5f685b058d43
title: Audio and speech models
track: ai-llm-engineer
topic: multi-modal-models-vision-language-models
difficulty: 3
tags:
- audio-understanding
- speech-to-text
- multimodal-audio
- Whisper
- audio-embedding
aliases:
- speech recognition
- audio-language models
- audio LLMs
- Whisper model
sources:
- url: https://ai.google.dev/gemini-api/docs/video-understanding
  label: Video understanding | Gemini API
- url: https://developers.googleblog.com/en/gemini-2-5-video-understanding/
  label: Advancing the frontier of video understanding with Gemini 2.5
- url: https://www.labellerr.com/blog/top-open-source-vision-language-models/
  label: Best Open-Source Vision Language Models of 2026
cards:
- id: ecbd724b-d0ff-44c8-a6ab-d3a5a336597a
  type: flip
  front: What is the key advantage of native audio input (e.g., Gemini 3) over Whisper STT + LLM?
  back: 'Avoids cascading errors: Whisper mistakes → LLM mistakes. Single unified model can leverage acoustic+linguistic
    context simultaneously, improving accuracy on ambiguous speech.'
- id: 490d2af4-e8a1-40e4-a7e0-a651f8b9b9a2
  type: flip
  front: Explain Whisper's training approach in one sentence.
  back: 'Unsupervised multitask learning on 680k hours of multilingual web audio: simultaneously train
    speech-to-text, language ID, and timestamp prediction, enabling robust zero-shot recognition across
    accents/noise.'
- id: 9b2c0eba-3bdf-4b3c-9604-148a95b4d6ac
  type: flip
  front: How do audio-visual models enable moment retrieval in videos?
  back: Align audio embeddings (speech, music) with visual frame embeddings via contrastive loss. Temporal
    attention over aligned embeddings predicts when events occur. E.g., query "applause" → find audio+visual
    agreement on timestamp.
- id: 0923cd07-efe9-47ad-b644-0a192a230a6e
  type: mcq
  front: What is a limitation of Whisper on non-English speech?
  back: Training data skews English; 680k hours are mostly English speeches. Non-English languages may
    have lower quality STT output. Languages with less internet presence (rare dialects, endangered languages)
    perform worst.
  choices:
  - key: a
    text: It doesn't support non-English languages at all
    correct: false
  - key: b
    text: Training data heavily skewed toward English; non-English and low-resource languages have lower
      quality
    correct: false
  - key: c
    text: It requires manual tuning for each language
    correct: false
  - key: d
    text: It uses separate models for each language, increasing inference cost
    correct: false
- id: 05b06689-512d-4cfc-856f-030dd868f6ba
  type: flip
  front: Why is speaker diarization a separate problem from speech-to-text?
  back: 'STT predicts text from audio; diarization identifies speaker boundaries ("speaker A: 0-5s, speaker
    B: 5-10s"). Requires tracking voice identity, not just recognizing words. Often done via clustering
    embeddings after STT.'
---

## Intuition

Audio adds a fourth modality (beyond image, text, video). Models can process speech, music, ambient sound, and sync them with video. **Whisper** (OpenAI) pioneered robust speech-to-text; **Gemini 3** extends this: native audio input to LLM (no separate STT), audio-visual alignment for video understanding. Interview prep focuses on: (1) when to use dedicated STT (Whisper) vs. end-to-end, (2) audio embeddings for retrieval, (3) multimodal fusion with vision.

## Detail

**Whisper Architecture**
- Encoder: mel-spectrogram → CNN → transformer encoder; learns acoustic representations
- Decoder: transformer decoder with cross-attention to encoder outputs; generates text tokens
- Training: 680k hours of multilingual audio from web; unsupervised (no human labels needed)
- Robustness: handles accents, background noise, technical terms without explicit fine-tuning
- Inference: can emit punctuation, language ID, timestamps

**Gemini Native Audio**
- Skip STT: send audio directly to LLM; model processes acoustic features + text simultaneously
- Audio-visual alignment: fuse audio embeddings with video frames via cross-attention
- Moment retrieval: "find when speaker says X" combines audio keyword detection + visual confirmation
- Efficiency: avoid cascading errors (STT error → LLM error); single model responsible

**Audio Embeddings**
- Contrastive learning: similar to vision embeddings (CLIP-style). Train pairs of (audio, text description) to share embedding space
- Speaker identification: embeddings capture voice identity; useful for tracking speakers in video
- Music embeddings: encode melody, rhythm, genre; enable music retrieval
- Example: query "find moments where uplifting music plays" → audio embedding matches "uplifting" text, temporal attention finds frames

**Multimodal Fusion** (Audio + Vision + Text)
- Early fusion: project all modalities to $d$-dim space, concatenate, process jointly
- Late fusion: process each modality separately, combine at output layer
- Cross-modal attention: audio queries, visual values (find visual events triggered by audio)

## Common gotchas / interview framings
- Cascade vs. end-to-end: STT → LLM risks compounding errors. Native audio reduces cascading but requires larger models
- Language coverage: Whisper handles 99 languages but skews toward English (more training data). Non-English speech may be lower quality
- Real-time vs. batch: Whisper is batch-optimized; real-time streaming requires buffering strategies (sliding window)
- Speaker diarization: identifying "who said what" is separate from speech-to-text; often requires post-processing
- Audio quality: Whisper robust to noise, but extremely noisy audio (crowds, machinery) may degrade. Denoising preprocessing helps
- Context window: audio is long (1 hour = 3.6M audio samples); downsampling to 16 kHz still yields large sequences. Hierarchical pooling needed

## See also
- [[whisper-model]]
- [[speech-to-text]]
- [[audio-embeddings]]
- [[gemini-audio]]
- [[multimodal-fusion]]
- [[speech-understanding]]

## Sources
See frontmatter `sources:`.
