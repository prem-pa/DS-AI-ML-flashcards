---
id: 3fb7d411-60c4-4adb-b1bc-b89276bf8018
title: Ragas for RAG evaluation
track: ai-llm-engineer
topic: evaluation-benchmarking
difficulty: 3
tags:
- RAG
- evaluation
- retrieval
- groundedness
- context-precision
aliases:
- Ragas metrics
- retrieval augmentation evaluation
- faithfulness scoring
sources:
- url: https://www.braintrust.dev/articles/deepeval-alternatives-2026
  label: DeepEval Alternatives 2026
- url: https://deepeval.com/docs/metrics-ragas
  label: DeepEval Ragas Metrics Documentation
- url: https://atlan.com/know/llm-evaluation-frameworks-compared/
  label: 'RAGAS, TruLens, DeepEval: LLM Evaluation Frameworks'
cards:
- id: b030ac88-5732-4abb-b7a3-78bdb01d1466
  type: flip
  front: What is the difference between context precision and context recall in Ragas evaluation?
  back: 'Context precision: fraction of retrieved docs that are relevant (false-positive rate). Context
    recall: fraction of relevant docs the retriever found (false-negative rate). Precision for false positives,
    recall for false negatives.'
- id: 43c2c2e8-caa5-4335-b658-c21b61f90bb2
  type: mcq
  front: You're evaluating a RAG system that returns relevant documents but the LLM output contains information
    not in those documents. Which Ragas metric would directly detect this?
  back: Groundedness evaluates whether the generated answer is supported only by retrieved context, detecting
    hallucinations.
  choices:
  - key: a
    text: Context precision
    correct: false
  - key: b
    text: Answer relevance
    correct: false
  - key: c
    text: Groundedness (faithfulness)
    correct: true
  - key: d
    text: Context recall
    correct: false
- id: 4f1a403d-36a5-4e8b-ac26-c10b0deb773f
  type: flip
  front: Why does Ragas' dependence on LLM-as-judge (e.g., GPT-4) for scoring introduce evaluation risk?
  back: The judge LLM has its own biases and limitations. If the judge is lenient, all RAG outputs appear
    high-quality. Always validate judge scores against manual samples to calibrate.
- id: 8e2405f4-878e-4007-96c1-a34809c0630c
  type: mcq
  front: A Ragas evaluation shows high groundedness but low answer relevance. What does this indicate
    about your RAG system?
  back: High groundedness = answers are faithful to context. Low relevance = answers don't address the
    question. So context itself is off-topic.
  choices:
  - key: a
    text: Retriever is broken
    correct: false
  - key: b
    text: LLM is hallucinating
    correct: false
  - key: c
    text: Retrieved context is irrelevant to the question
    correct: true
  - key: d
    text: Perfect system
    correct: false
---

## Intuition
RAG systems have unique failure modes: (1) retriever returns irrelevant context, (2) LLM generates hallucinations (answers not supported by context), (3) LLM answers the wrong question. Ragas provides metrics to isolate each failure independently, enabling root-cause diagnosis.

## Detail
Ragas is a pure Python evaluation library (no dashboards, no platforms) focused on RAG-specific metrics:

**Core metrics:**
- **Retrieval (context relevance)**: Do retrieved documents answer the question? Evaluated via LLM-as-judge: "Given question Q and document D, is D relevant?" Isolates retriever quality.
- **Groundedness (faithfulness)**: Is the answer supported by retrieved context? LLM evaluates: "Does generated answer only use facts from context?" Detects hallucinations.
- **Answer relevance**: Does the answer address the question (independent of retrieval)? Evaluates LLM's ability to focus on the question despite irrelevant context.
- **Context precision**: What fraction of retrieved context is actually relevant? Measures retriever false-positive rate.
- **Context recall**: What fraction of relevant documents did the retriever find? Measures retriever false-negative rate.

**Usage pattern** (as of 2026): Ragas integrates directly into Langfuse. You define:
```python
from ragas import evaluate
from ragas.metrics import faithfulness, answer_relevance, context_precision

results = evaluate(
    dataset=my_rag_dataset,
    metrics=[faithfulness, answer_relevance, context_precision]
)
```

## Common gotchas / interview framings
- **LLM-as-judge bias**: Ragas scores depend on the judge LLM (typically GPT-4). Biased judges inflate scores; always validate against manual samples.
- **Context recall ≠ retrieval quality**: A retriever returning 100 docs has high recall but poor precision; Ragas isolates this via context precision.
- **Groundedness is hard**: Faithfulness evaluation requires the judge to distinguish hallucinations from inferences. Models disagree; use multiple judges or manual verification.
- **Interview framing**: "How would you use Ragas to debug a RAG system returning hallucinated answers?" Separate groundedness (hallucination) from answer relevance (off-topic) from context precision (noisy retrieval).

## See also
- [[rag]]
- [[retrieval-evaluation]]
- [[groundedness]]
- [[answer-relevance]]
- [[context-precision]]
- [[llm-evaluation-frameworks]]

## Sources
See frontmatter `sources:`.
