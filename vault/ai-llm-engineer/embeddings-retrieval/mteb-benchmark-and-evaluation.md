---
id: 0c26d2b0-2c1f-4fcb-85e9-6ec29e652d6c
title: MTEB Benchmark and Evaluation
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- benchmarking
- evaluation
- embeddings
- leaderboard
aliases:
- MTEB leaderboard
- embedding evaluation
- Massive Text Embedding Benchmark
sources:
- url: https://huggingface.co/spaces/mteb/leaderboard
  label: MTEB Leaderboard
- url: https://github.com/embeddings-benchmark/mteb
  label: MTEB GitHub Repository
- url: https://huggingface.co/mteb
  label: MTEB on HuggingFace
cards:
- id: 46e02b71-cd43-497a-be55-bde18771b5fc
  type: flip
  front: What are the main task categories in MTEB and why is retrieval considered the most important?
  back: 'MTEB includes 8+ task categories: Retrieval, Clustering, Classification, Semantic Textual Similarity
    (STS), Reranking, Pair Classification, and Summarization. Retrieval is most important for RAG and
    production systems because: (1) it''s the largest dataset category, (2) it directly measures core
    use case (find relevant documents), (3) it correlates well with downstream RAG quality, and (4) retrieval
    performance is most impactful for end-user relevance.'
- id: d2141319-a186-412a-8293-6466827806f3
  type: flip
  front: Explain why MTEB v1 and MTEB v2 scores are not directly comparable.
  back: MTEB v2 (released 2026) introduced updated evaluation protocols, new datasets, and refined task
    definitions that differ from v1. Models evaluated on v1 had different scoring scales and may not have
    been fine-tuned for v2 tasks. When comparing models, always verify which MTEB version was used; mixing
    v1 and v2 results invalidates the comparison.
- id: 603c99b6-39ba-4f6d-8155-6cf34abcd2fe
  type: mcq
  front: A model scores 72 MTEB overall (55 retrieval, 85 classification). Your use case is retrieval-focused
    RAG. What should you consider?
  back: Task weighting is critical. MTEB averages hide performance variation across tasks. A model excelling
    at classification but weak at retrieval will perform poorly for retrieval-focused RAG. Always drill
    down to task-specific scores. The 30-point gap suggests excellent classification generalization but
    weaker retrieval performance, which is the bottleneck for your use case. Consider [[leading-embedding-models-bge-m3-e5-voyage-cohere-openai-text-embedding-3]]
    or [[specialized-embeddings-medical-code-domain-specific]] if you need retrieval optimization.
  choices:
  - key: a
    text: 72 average is strong; use this model
    correct: false
  - key: b
    text: The 55 retrieval score is what matters most; high classification doesn't help your use case
    correct: true
  - key: c
    text: The gap between classification (85) and retrieval (55) indicates the model is unstable
    correct: false
  - key: d
    text: You need to retrain on your domain first
    correct: false
- id: ea23c2dd-6232-47de-8517-a18bb71d4fe6
  type: flip
  front: What does NDCG@10 measure in MTEB retrieval tasks?
  back: 'NDCG@10 (Normalized Discounted Cumulative Gain at 10) measures ranking quality of the top 10
    retrieved documents. It rewards ranking relevant documents higher: relevant items at position 1 score
    higher than at position 10. The formula is: $\text{NDCG@10} = \frac{1}{Z} \sum_{i=1}^{10} \frac{2^{rel_i}
    - 1}{\log_2(i+1)}$ where $rel_i$ is relevance at rank $i$ and $Z$ is the ideal ranking. Score range:
    0–1, with 1.0 being perfect.'
- id: ade9427d-2a45-4606-84db-891af00f2a6d
  type: flip
  front: Why is it important to check for training data overlap between your embeddings model and MTEB
    datasets?
  back: If an embedding model was trained on data that includes MTEB evaluation datasets, its MTEB scores
    will be artificially inflated (data leakage). The model may memorize or overfit to those specific
    datasets, reducing generalization to new domains. Always check the model's training data documentation
    and, when possible, verify performance on held-out datasets or your own domain-specific data to get
    a true sense of generalization capability.
---

## Intuition
MTEB (Massive Text Embedding Benchmark) is the standard evaluation suite for embedding models, covering 8+ diverse tasks (retrieval, clustering, classification, reranking) across 100+ languages. It solves the problem: "How do you fairly compare embeddings across domains?" by providing a common benchmark that prevents overfitting to narrow use cases.

## Detail
MTEB v1 includes 200+ datasets grouped into task categories:
- **Retrieval:** Find relevant documents for a query (largest task by count, most important for RAG).
- **Clustering:** Group semantically similar texts without labels.
- **Classification:** Text classification using embeddings.
- **STS (Semantic Textual Similarity):** Rate similarity between sentence pairs.
- **Reranking, Pair Classification, Summarization:** Specialized tasks.

Scores are reported as NDCG@10 for retrieval, accuracy for classification, and average precision for others. MTEB v2 (2026) introduced new evaluation protocols and tasks but scores are not directly comparable to v1.

Leading models as of early 2026: Qwen3-Embedding-8B (70.58), NV-Embed-v2, Google Gemini Embedding 001, Cohere embed-v4 (65.2), [[leading-embedding-models-bge-m3-e5-voyage-cohere-openai-text-embedding-3]] (OpenAI text-3-large: 64.6, BGE-M3: 63.0).

## Common gotchas / interview framings
- **MTEB v1 vs v2 incomparability:** Published numbers from 2024 won't match 2026 runs; always check the version.
- **Task weighting:** Average MTEB scores hide performance variation; a model may excel at retrieval but lag at clustering.
- **Retrieval bias:** MTEB retrieval tasks emphasize English; [[multilingual-and-cross-lingual-embeddings]] are needed for other languages.
- **Training data leakage:** Some MTEB datasets may overlap with training corpora; check source documentation.
- Interviewers ask: "Is a 0.5-point MTEB difference significant?" Answer: depends on task and data size; <0.1 is noise, >1.0 is meaningful.

## See also
- [[dense-embeddings-and-contrastive-learning]]
- [[leading-embedding-models-bge-m3-e5-voyage-cohere-openai-text-embedding-3]]
- [[multilingual-and-cross-lingual-embeddings]]

## Sources
See frontmatter `sources:`.
