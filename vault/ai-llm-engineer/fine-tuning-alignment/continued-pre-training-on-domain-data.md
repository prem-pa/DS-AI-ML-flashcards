---
id: c19271bf-1354-499a-8abc-9260a452c9a7
title: Continued pre-training on domain data
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 3
tags:
- pretraining
- domain-corpus
- token-distribution
- vocabulary-adaptation
- continued-learning
aliases:
- Domain pretraining
- further pretraining
- adaptive pretraining
sources:
- url: https://llamafactory.readthedocs.io/en/latest/getting_started/pretraining.html
  label: LLaMA Factory Continued Pretraining
- url: https://www.jenova.ai/en/resources/llama-factory-complete-guide-to-llm-fine-tuning
  label: LLaMA Factory Fine-Tuning Guide
- url: https://huggingface.co/blog/mlabonne/sft-llama3
  label: HuggingFace LLaMA 3 SFT Guide
cards:
- id: d5070072-faab-41a1-909f-2eb5b29e1ca1
  type: flip
  front: 'You have 20 GPU-days of budget. Choose: continued pretraining on 50B code tokens OR SFT on 50k
    code Q&A pairs?'
  back: Choose SFT. 20 GPU-days continued pretraining on code barely scratches 50B tokens (~1/10th needed
    for real impact). Instead, SFT on clean 50k code Q&A examples teaches task-specific reasoning with
    massive time savings and faster iteration. Reserve continued pretraining for post-hoc analysis, not
    critical path.
- id: f8a10251-466f-4ff2-aec2-a15aafc19572
  type: mcq
  front: When is continued pretraining on domain data most justified?
  back: Continued pretraining pays off **only** when domain has truly novel tokens or structure. Standard
    terminology can be learned via SFT. Code, math notation, legal markup justify the cost.
  choices:
  - key: a
    text: Domain has standard English + 10 new terminology words
    correct: false
  - key: b
    text: Domain has unique symbolic notation, rare tokens, or structural patterns (e.g., scientific papers,
      code, legal markup)
    correct: true
  - key: c
    text: You want to improve MMLU scores
    correct: false
  - key: d
    text: You have unlimited compute budget
    correct: false
- id: 0dd8818b-fd64-4eb2-b014-89a525eb03fa
  type: flip
  front: After 10B tokens of continued pretraining on medical texts, loss jumps 2x on your general eval
    set. Diagnosis?
  back: '**Catastrophic forgetting**: learning rate too high or data quality too low. Immediate fix: (1)
    lower learning rate by 2–5x, (2) validate data quality (remove noisy scans, OCR errors), (3) mix 20–30%
    general data into batches to retain base knowledge. Future: continued pretraining is risky; SFT +
    domain-specific preference tuning is safer.'
- id: c583de83-c96a-4091-beed-23fd55d7c4e9
  type: flip
  front: Should you expand the vocab before or after continued pretraining on a new domain?
  back: '**Before**. Expanding vocab after pretraining wastes learned embeddings for new tokens. If you
    expand mid-stream, new tokens initialize randomly and never benefit from subsequent training. Plan
    vocab additions upfront, initialize embeddings (e.g., from similar tokens), then continue pretraining.'
---

## Intuition
Continued pretraining extends causal language modeling (next-token prediction) on domain-specific text without instruction labels. Useful when the domain has **unique vocabulary or structure** absent in general pretraining. Example: continued pretraining a general LLM on ArXiv papers teaches it scientific writing patterns, mathematical notation, and domain concepts.

## Detail
Objective: standard causal language modeling loss on domain corpus,
$$\mathcal{L} = -\sum_t \log p_\theta(x_t | x_{<t})$$
where $x$ is raw domain text (no instruction labels needed).

Key parameters:
- **Learning rate**: 0.1–0.5x the SFT rate (e.g., $1 \times 10^{-5}$ instead of $2 \times 10^{-5}$) to avoid divergence from pretrained distribution.
- **Data volume**: 10–100B tokens typical. Costs 5–50x more compute than SFT (~1M tokens).
- **Vocab extension**: If domain uses novel tokens (e.g., new symbols, rare scientific notation), expand vocab and reinitialize embeddings before continued pretraining.
- **Checkpointing strategy**: Save intermediate checkpoints (every 5B tokens) to avoid loss if training interrupts.

After continued pretraining, the model is "warmed up" on domain signal and ready for task-specific SFT. Combined: continued pretraining → SFT → DPO/RLHF.

## Common gotchas / interview framings
- **When NOT to do it**: If domain vocab is standard English + a few specialized terms, SFT alone is 90% effective at 10% the cost. Skip continued pretraining unless you have >10B domain tokens.
- **Catastrophic forgetting of base**: Aggressive learning rate or low-quality data can degrade general knowledge. Monitor loss on base pretraining eval set (e.g., pile) to ensure no divergence.
- **Vocabulary size mismatch**: If domain has rare symbols, you must add tokens before continued pretraining. Adding tokens *after* wastes learned embeddings.
- **Diminishing returns**: Beyond ~50B domain tokens, loss improvement plateaus. Compute budget better spent on SFT + DPO.

## See also
- [[pretraining]]
- [[language-modeling]]
- [[domain-corpus]]
- [[token-embeddings]]
- [[causal-language-modeling]]
- [[computational-cost]]

## Sources
See frontmatter `sources:`.
