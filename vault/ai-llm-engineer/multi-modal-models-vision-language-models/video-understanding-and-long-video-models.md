---
id: 8bcba60a-1e25-46ed-a038-687bf355c37c
title: Video understanding and long-video models
track: ai-llm-engineer
topic: multi-modal-models-vision-language-models
difficulty: 5
tags:
- video-understanding
- temporal-reasoning
- frame-sampling
- video-llms
- long-context-video
aliases:
- temporal VLMs
- video reasoning
- motion understanding
- frame fusion
sources:
- url: https://developers.googleblog.com/en/gemini-2-5-video-understanding/
  label: Advancing the frontier of video understanding with Gemini 2.5
- url: https://ai.google.dev/gemini-api/docs/video-understanding
  label: Video understanding | Gemini API | Google AI
- url: https://medium.com/google-cloud/video-understanding-with-gemini-notes-from-the-field-82dd0cd130ea
  label: 'Video Understanding with Gemini: Notes From the Field'
cards:
- id: d357925c-b45f-4622-bed1-5718984d02de
  type: flip
  front: Why is video understanding fundamentally harder than image understanding?
  back: 'Videos require **temporal reasoning**: tracking object identity across frames, understanding
    causality (A causes B), and predicting duration. Images are independent; videos have sequential structure
    and motion.'
- id: bf945458-4ecf-4dfe-9d34-33064d0f9b53
  type: flip
  front: What is the key insight behind Gemini 2.5's efficient long-video processing (6+ hours)?
  back: 'Low-res parameter: reduce frame resolution adaptively (~66 tokens/sec instead of 258). Combined
    with 2M context window, fits 6+ hours of video. Trade-off: lower resolution but maintains audio-visual
    synchronization.'
- id: 3c23ff22-24ca-4eaa-9079-4ae6a61cf630
  type: flip
  front: Describe the difference between fixed FPS, keyframe extraction, and learned frame selection.
  back: '**Fixed FPS** (1 fps): every frame sampled uniformly; simple but redundant. **Keyframe** (optical
    flow): extract high-motion frames; efficient but hand-crafted. **Learned** (attention): model predicts
    which frames matter; adaptive to content.'
- id: 47da6f03-597b-4e94-9a7b-c564c1c524af
  type: mcq
  front: How does audio-visual fusion improve moment retrieval in videos?
  back: 'Audio provides temporal landmarks: speech, music, silence. Aligning audio embeddings with visual
    frames via contrastive loss or CTC enables precise event localization. Finding "when person says X"
    combines audio detection + visual confirmation.'
  choices:
  - key: a
    text: Audio is converted to visual spectrograms and processed identically
    correct: false
  - key: b
    text: Audio provides temporal landmarks (speech, music) that align with visual events, enabling precise
      moment retrieval
    correct: false
  - key: c
    text: Audio is ignored; only visual frames are processed
    correct: false
  - key: d
    text: Audio is used only for speech-to-text, not for alignment
    correct: false
- id: 83eb36ab-2c41-4468-a6bf-57f03c6f0e3f
  type: flip
  front: What is the computational bottleneck for dense (24-30 fps) video understanding, and how do models
    mitigate it?
  back: 'Dense frames → $O(T)$ tokens where $T$ = duration. A 1-hour video at 30 fps = 108k frames = 108k+
    tokens (exceeds most context windows). Mitigations: sparse sampling (1-2 fps), keyframe extraction,
    learned pruning, or hierarchical processing.'
---

## Intuition

Videos add a **temporal dimension** to vision-language understanding. Instead of independent frames, models must reason about motion, causal relationships ("A happens, then B"), and duration ("how long is the event?"). Modern approaches split into: (1) frame sampling (efficient), (2) dense frame processing (accurate but expensive), and (3) hierarchical fusion (balance both). Gemini 2.5 (Dec 2024) achieves SOTA by fusing audio+visual cues for moment retrieval (e.g., finding the exact second when "person enters room").

## Detail

**Frame Sampling Strategies**
1. **Fixed FPS**: extract frames at constant rate (e.g., 1 fps for 1-hour video = 3600 frames). Misses key moments if FPS too low.
2. **Keyframe extraction**: detect high-motion frames (optical flow, scene cuts). Reduces redundancy, captures events.
3. **Learned selection**: transformer attends to frames; gradient-based pruning identifies salient frames. GPT-4V samples keyframes; Gemini 2.5 learns selection end-to-end.
4. **Hierarchical**: pyramid of resolutions; coarse pass identifies regions of interest, fine pass examines them.

**Temporal Reasoning**
- **Optical flow**: compute motion between frames; represents as features or learned embeddings
- **Temporal attention**: self-attention over frame embeddings with learned temporal positional encoding
- **Causal reasoning**: RNNs (LSTM) or TCN (temporal convolutional networks) capture sequential dependencies
- **Moment retrieval**: align text query to specific timestamp. Gemini 2.5 fuses audio+visual embeddings, predicts (start, end) timestamps with temporal attention

**Gemini 2.5 Architecture**
- Unified transformer over video frames + audio
- Low-res parameter: adaptive frame resolution to reduce tokens (~66 tokens/sec vs. 258 normal)
- Audio-visual alignment: CTC loss or contrastive learning to synchronize audio events with visual frames
- Long-context: 2M token window enables processing 6+ hours of video
- Moment retrieval: temporal attention over aligned embeddings predicts event boundaries

**Trade-offs**
- **Dense frames** (24-30 fps): high accuracy but $O(T)$ tokens where $T$ = video duration. Infeasible for hours-long content
- **Sparse sampling** (1 fps): efficient but may miss brief events
- **Learned selection**: adaptive to content but requires pretraining on labeled datasets

## Common gotchas / interview framings
- Temporal understanding is not image understanding repeated: models must track object identity across frames ("is this person the same in frame 5 and frame 10?")
- Audio is critical for moment retrieval: speech/music provides temporal cues (e.g., "speaker says X at timestamp Y"). Models ignoring audio fail on podcast/audiobook tasks
- Long-video efficiency: Gemini 2.5 reduces token usage from 258 to 66 tokens/sec via low-res; further reductions via sparse sampling (e.g., 1 frame per 10 seconds for static scenes)
- Domain shift: models trained on internet videos (action, faces, objects) may fail on specialized domains (medical, surveillance, industrial)
- Benchmark bias: VQA datasets favor simple questions ("what color is the car?"); harder temporal questions ("how many times does X happen?") underexplored

## See also
- [[temporal-reasoning]]
- [[video-llms]]
- [[frame-sampling]]
- [[optical-flow]]
- [[moment-retrieval]]
- [[gemini-25]]
- [[clip-video]]

## Sources
See frontmatter `sources:`.
