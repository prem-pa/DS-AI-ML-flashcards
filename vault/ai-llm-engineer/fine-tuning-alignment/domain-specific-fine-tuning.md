---
id: 9efbf6fe-fb31-433f-a9c6-bcedd6cf6699
title: Domain-specific fine-tuning
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 3
tags:
- domain-adaptation
- specialization
- vertical-models
- in-domain-performance
- transfer-learning
aliases:
- Domain adaptation
- vertical tuning
- specialty models
sources:
- url: https://llamafactory.readthedocs.io/en/latest/
  label: LLaMA Factory Documentation
- url: https://www.jenova.ai/en/resources/llama-factory-complete-guide-to-llm-fine-tuning
  label: LLaMA Factory Complete Guide
- url: https://huggingface.co/docs/peft/en/overview
  label: HuggingFace PEFT Overview
cards:
- id: a879cc85-18eb-4492-bf24-f7f8356818da
  type: flip
  front: You're building a medical LLM from Llama-2. SFT on 50k medical notes or 200k synthetic medical
    Q&A first?
  back: Start with 200k synthetic (GPT-4 generated) medical Q&A for SFT, then optionally fine-tune on
    domain-annotated human data (50k notes) for preference tuning. Synthetic data cheaper, scales faster;
    human data provides ground truth. SFT on mixed sources, then DPO/RLHF on high-quality human preferences.
- id: b8cdad8c-0263-4e68-9f15-7cf59760bb47
  type: mcq
  front: You notice your domain-specific model scores 85% on in-domain medical Q&A but 55% on MMLU (general
    knowledge). Why?
  back: 'Over-tuning on narrow medical data causes catastrophic forgetting of general knowledge. Mitigation:
    mix 50% general instruction data into training. The MMLU drop signals loss of pretraining knowledge,
    not vocab issues.'
  choices:
  - key: a
    text: The domain vocab is too specialized
    correct: false
  - key: b
    text: Catastrophic forgetting from aggressive domain fine-tuning
    correct: true
  - key: c
    text: Medical pretraining is inherently weak at MMLU
    correct: false
  - key: d
    text: LoRA rank was too high
    correct: false
- id: e146110b-c618-48ca-a10e-6e01ff085221
  type: flip
  front: When is continued pretraining on domain corpus (before SFT) worth the cost?
  back: 'When the domain has **unique vocabulary or structure** not covered in pretraining. Examples:
    niche scientific terminology, proprietary code syntax, specialized notation. If domain examples use
    standard English + domain terminology, SFT alone suffices. Continued pretraining is expensive (days)
    and only pays off for truly novel token distributions.'
- id: d2c52038-ada4-44e6-bc99-d70ccf14eeee
  type: flip
  front: You built a legal Llama on contract data and it hallucinates non-existent clauses. Diagnosis?
  back: 'Likely SFT on noisy synthetic legal data or too-narrow human annotation. Legal reasoning requires
    **grounding**: each prediction tied to source text. Mitigation: (1) add retrieval-augmented generation
    (RAG) to fetch relevant clauses, (2) use preference tuning (DPO) to penalize unsourced claims, (3)
    audit training data for hallucination seeds.'
---

## Intuition
Domain-specific fine-tuning adapts a general LLM to a narrow vertical: medical, law, finance, code. The model learns domain vocabulary, reasoning patterns, and conventions. Example: adapting Llama-2 on 50k medical notes teaches it anatomy, drug interactions, ICD coding.

Key insight: general pretraining lacks domain signal. Supervised fine-tuning on domain examples is the simplest way to inject it. Improves both in-domain accuracy (e.g., USMLE scores on medical Llama) and reduces out-of-distribution hallucinations.

## Detail
Process: (1) Collect domain corpus (medical notes, legal contracts, code repos). (2) SFT on domain instruction-response pairs. (3) Optional: continued pretraining on domain text first to inject tokens/concepts, then SFT for task alignment. (4) Optional: DPO or RLHF on domain preferences (e.g., lawyer-annotated legal reasoning).

Challenges:
- **Data scarcity**: Medical/legal domains have <100k labeled examples. Mitigation: synthetic data (GPT-4 generates domain Q&A), data augmentation, transfer from related domains.
- **Vocabulary drift**: Domain jargon not well-represented in pretraining. Solution: continued pretraining on raw domain text to adapt embeddings.
- **Catastrophic forgetting**: Over-aggressive domain tuning degrades general knowledge. Balance: train on mixed data (50% domain + 50% general instruction data).

Best practice: LoRA on domain data is standard. Rank r=16–32 typical. Training time: 1–3 epochs on 10k–100k examples. Validation: task-specific metrics (e.g., F1 on medical NER, BLEU on legal summarization) + MMLU delta to catch forgetting.

## Common gotchas / interview framings
- **"In-domain" is relative**: A model trained on 1 hospital's data may fail on another due to notation differences. Validate cross-site, cross-provider.
- **Synthetic data quality**: LLM-generated training pairs are biased toward model's pretraining distribution. Use human-in-the-loop or domain expert review.
- **Task-specific overfitting**: A code model trained only on Python is brittle on Go or Rust. Diversify domains within the vertical.
- **Benchmark leakage**: Be careful fine-tuning on data overlapping with eval benchmarks (e.g., medical Llama on data from USMLE source corpora).

## See also
- [[domain-knowledge]]
- [[transfer-learning]]
- [[in-domain-data]]
- [[vertical-ai]]
- [[task-specific-models]]
- [[fine-tuning-strategies]]

## Sources
See frontmatter `sources:`.
