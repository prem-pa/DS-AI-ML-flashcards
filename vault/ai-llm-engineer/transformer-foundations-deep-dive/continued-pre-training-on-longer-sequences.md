---
id: bf13efd1-23d4-4c48-bbc4-02615430f10c
title: Continued pre-training on longer sequences
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- continued-pretraining
- context-extension
- fine-tuning
- training
- curriculum
aliases:
- CPT
- continued training
- long-context training
sources:
- url: https://arxiv.org/abs/2309.16039
  label: 'YaRN: Efficient Context Window Extension'
- url: https://arxiv.org/abs/2404.10630
  label: 'Longform: Context Length Extension Methods for LLMs'
cards:
- id: b8f86181-c88e-43d8-b8be-8e64201d7e61
  type: flip
  front: What is continued pre-training (CPT), and how does it differ from RoPE interpolation for context
    extension?
  back: CPT trains the model further on longer-sequence data (e.g., 16K tokens) using causal language
    modeling. The model gradually learns long-context patterns and adapts internal representations. RoPE
    interpolation rescales position encodings to map longer sequences into the learned distribution, requiring
    only brief fine-tuning (2-10B tokens). CPT is slower (100B+ tokens) but may achieve better final quality
    through full adaptation. Interpolation is faster but less thorough.
- id: 71920e10-66f0-42a0-acf4-49e151c77f6f
  type: flip
  front: How does curriculum learning in CPT (gradually increasing sequence length) improve convergence?
  back: 'Instead of jumping from 2K-trained model directly to 16K sequences, curriculum training: (1)
    continues on 4K sequences for 10B tokens, (2) then 8K for 10B tokens, (3) then 16K for 10B tokens.
    This gradual exposure allows the model to adapt layer-by-layer, learning short-to-long dependencies
    progressively. Jumping directly to 16K causes training instability; curriculum stabilizes convergence
    and improves final quality.'
- id: ac3b159f-d6f1-425e-84d4-9668e6606e35
  type: mcq
  front: What is the main risk of continued pre-training for context extension without careful learning
    rate tuning?
  back: 'Correct: (b). High learning rates during CPT can overwrite learned patterns from original pretraining,
    degrading performance on 2K-token contexts while improving 16K. Careful learning rate scheduling (typically
    0.1-0.01x of original) preserves original performance. (a) is incorrect—CPT learns long-context if
    learning rate is appropriate. (c) and (d) are not CPT risks.'
  choices:
  - key: a
    text: The model will not learn longer-context patterns
    correct: false
  - key: b
    text: The model may catastrophically forget performance on original context lengths
    correct: true
  - key: c
    text: The training will take infinite time
    correct: false
  - key: d
    text: Longer sequences will run out of memory
    correct: false
- id: 05e24d6c-f15d-46d1-985b-c27fc85f3be7
  type: flip
  front: Compare the token budget and compute cost of RoPE interpolation vs. continued pre-training for
    extending from 4K to 32K context.
  back: 'RoPE interpolation: ~5-10B tokens, ~1-3 days on A100. Continued pre-training: ~100B tokens, ~2-4
    weeks on A100 or a small cluster. CPT is 10-50x more expensive but may achieve 1-3% better quality
    in long-context tasks. For production systems needing speed, interpolation is preferred; for research
    or ultimate quality, CPT is worth the investment.'
---

## Intuition
Continued pre-training (CPT) on longer sequences trains the model on longer context lengths post-training, allowing it to adapt internal representations and attention patterns to longer-range dependencies. CPT is slower than RoPE interpolation but may achieve better final quality by fully learning long-context patterns rather than interpolating existing ones.

## Detail
CPT workflow:
1. Start from a model pre-trained on context length L (e.g., 2K)
2. Collect or generate data with longer sequences (e.g., 8K, 16K, 32K)
3. Continue pre-training (causal language modeling) on this longer-sequence data
4. Use curriculum: gradually increase sequence length (e.g., 4K for 10B tokens, then 8K for 10B tokens) to aid convergence
Key considerations:
- **Learning rate:** Typically 10x smaller than initial pretraining to avoid catastrophic forgetting
- **Token budget:** 10-100B tokens depending on target length and quality
- **Data diversity:** Ensure longer-sequence data covers similar domains as original training
Benefit: model fully adapts, improving quality more than interpolation. Cost: 10-50x more compute than interpolation methods (which require 2-10B tokens).

## Common gotchas / interview framings
- **Catastrophic forgetting:** CPT can degrade performance on original context lengths if learning rate is too high. Careful scheduling needed.
- **Compute cost prohibitive:** 100B+ tokens of training is expensive (~weeks on A100 cluster). Interpolation is preferred for speed.
- **Curriculum learning helps:** Gradually increasing length (4K -> 8K -> 16K) converges faster than jumping to target.
- **Data quality:** Longer-sequence data quality matters; synthetic or low-quality data can hurt.
- **Complementary with interpolation:** CPT can follow RoPE interpolation fine-tuning for maximum quality.

## See also
- [[extrapolation-on-longer-sequences-post-training]]
- [[rope-interpolation-and-extrapolation]]
- [[rotary-position-embeddings-rope]]

## Sources
See frontmatter `sources:`.
