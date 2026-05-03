---
id: 57a450d1-443d-42bb-98bf-6d308ad76715
title: T5 and BART models
track: ai-llm-engineer
topic: large-language-model-architectures
difficulty: 3
tags:
- encoder-decoder
- t5
- bart
- seq2seq
- transfer-learning
- text-to-text
aliases:
- T5
- BART
- seq2seq-models
- text-to-text
sources:
- url: https://arxiv.org/abs/1910.13461
  label: Exploring the Limits of Transfer Learning with a Unified Text-to-Text Transformer (Raffel et
    al. 2020, T5)
- url: https://arxiv.org/abs/1910.13461
  label: 'BART: Denoising Sequence-to-Sequence Pre-training (Lewis et al. 2019)'
- url: https://huggingface.co/docs/transformers/model_doc/t5
  label: Hugging Face T5 Documentation
cards:
- id: 02efda62-3469-4a85-8516-7a9219e99619
  type: flip
  front: What is T5's key innovation in how it frames NLP tasks, and why does this matter?
  back: 'T5 frames all tasks as "text-to-text": input prefix + source text → target text. This unification
    means one model, one training objective, one fine-tuning approach. Examples: "translate English to
    French: Hello" → "Bonjour". This enables transfer learning and few-shot adaptation across diverse
    tasks with the same architecture.'
- id: 13267485-33c5-40ee-9b2c-f2ad404d911b
  type: flip
  front: Compare T5's span-corruption pretraining to BART's denoising corruption strategy.
  back: 'T5: masks spans of text (e.g., 10% of positions as spans); predict the spans. More efficient
    (fewer predictions per token). BART: corrupts input more aggressively (delete, permute, rotate, mask);
    reconstruct the original. BART''s corruption is more diverse, leading to better robustness; T5''s
    is more efficient for large-scale training.'
- id: a9a3c633-67dd-4ad7-8ba3-5f22e5c529c3
  type: mcq
  front: Why do encoder-decoder models like T5 and BART remain competitive despite the rise of decoder-only
    models (GPT-3, Llama)?
  back: For constrained seq2seq tasks (summarization, translation, QA), T5/BART achieve good performance
    with 2-10× fewer parameters than decoder-only models. This makes them ideal for resource-constrained
    deployments. Decoder-only models are better for open-ended generation and scale better overall, but
    T5/BART still win on efficiency for structured tasks.
  choices:
  - key: a
    text: They produce higher-quality generations than decoder-only models
    correct: false
  - key: b
    text: They require fewer parameters to match decoder-only performance on summarization and translation
    correct: true
  - key: c
    text: Decoder-only models cannot perform seq2seq tasks
    correct: false
  - key: d
    text: They always faster at inference than decoder-only
    correct: false
- id: a6ddc916-3787-471c-ab14-ec6cb1f824f7
  type: flip
  front: Explain the difference between T5's and BART's approach to pretraining and their implications
    for downstream fine-tuning.
  back: 'T5: span masking on clean text; learns to predict masked spans. Task prefix in pretraining. Downstream:
    append task prefix, fine-tune. BART: aggressive corruption (delete, permute, etc.); learns robust
    reconstruction. Downstream: standard fine-tuning (loss on target). T5''s prefix approach enables more
    direct transfer; BART''s robustness helps with noisy or out-of-distribution inputs.'
---

## Intuition

T5 and BART are encoder-decoder models designed for sequence-to-sequence tasks (translation, summarization, question answering). T5 treats all NLP tasks as "text-to-text"; BART is a denoising autoencoder that corrupts input and reconstructs it. Both leverage large-scale pretraining followed by task-specific fine-tuning, striking a middle ground between [[decoder-only-vs-encoder-decoder-tradeoff]]: bidirectional understanding via encoder, autoregressive generation via decoder.

## Detail

**T5 (Text-to-Text Transfer Transformer):**
- Unified framework: all tasks reframed as text → text
  - Translation: "translate English to German: [en_text]" → [de_text]
  - Summarization: "summarize: [document]" → [summary]
  - QA: "question: [q] context: [c]" → [answer]
- Pretraining: Masked language modeling (span corruption) on 750GB of text (C4 corpus)
- Architecture: Standard encoder-decoder, 12 layers each (base), 24 layers (large), 24 layers (3B)
- Scaled version: T5-11B
- Key insight: Unified task prefix + text-to-text paradigm enables few-shot transfer

**BART (Bidirectional and Auto-Regressive Transformers):**
- Hybrid of BERT (encoder) + GPT (decoder)
- Pretraining: Corrupt input (delete, mask, permute, rotate) → reconstruct
- More aggressive corruption than T5's span masking
- Architecture: 12 encoder + 12 decoder layers (base), 24+24 (large)
- Strong on summarization and abstractive paraphrase tasks
- Strengths: Faster training, simpler fine-tuning

**Architectural comparison:**

| Aspect | T5 | BART |
|--------|-----|------|
| Encoder | Standard | BERT-like (bidirectional) |
| Decoder | Standard causal | GPT-like (causal) |
| Pretraining | Span masking | Denoising |
| Task format | Text-to-text (unified) | Task-specific fine-tune |
| Strengths | Scalability, transfer | Summarization, speed |

**Why they matter (2023-2025 context):**
- Dominated pre-decoder-only era (2019-2021)
- Still strong for constrained seq2seq (summarization, translation, QA with retrieval)
- Decoder-only models (GPT-3, Llama) have largely superseded them for open-ended generation
- T5/BART useful for efficiency on short-range seq2seq tasks

## Common gotchas / interview framings

- "T5/BART are 'obsolete' because decoder-only models exist." → Not obsolete; good for low-latency seq2seq (summarization, translation) and parameter-efficient fine-tuning on specific tasks.
- **Transfer learning:** Both designed for transfer: pretrain on C4 / unsupervised corruption, fine-tune on downstream tasks (e.g., GLUE, SQuAD) with small labeled data.
- **Span corruption (T5):** Mask spans of tokens; predict them. Cheaper than predicting every token (BERT style).
- **Decoder conditioning:** Both use encoder output as cross-attention context; decoder never sees input directly.

## See also
- [[encoder-decoder-architecture-and-cross-attention]]
- [[decoder-only-vs-encoder-decoder-tradeoff]]
- [[scaling-laws-for-loss-and-compute]]

## Sources
See frontmatter `sources:`.
