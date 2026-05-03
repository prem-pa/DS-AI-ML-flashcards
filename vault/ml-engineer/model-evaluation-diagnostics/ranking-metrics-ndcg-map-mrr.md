---
id: 0d7c8c67-0d89-4eab-ab19-f68210027e09
title: Ranking metrics (NDCG, MAP, MRR)
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 3
tags:
- ranking
- information retrieval
- position-aware
- recommendation
- search
aliases:
- normalized discounted cumulative gain
- mean average precision
- mean reciprocal rank
sources:
- url: https://www.ml4devs.com/what-is/model-evaluation-metrics/
  label: 'ML4Devs: Model Evaluation Metrics'
- url: https://developers.google.com/machine-learning/crash-course/classification/accuracy-precision-recall
  label: Google ML Crash Course
cards:
- id: 53daf79a-124c-440b-b7d8-8c5f30fca4f9
  type: flip
  front: Why does DCG divide by log₂(i+1) instead of just using position i?
  back: 'Logarithmic discount reduces penalty for low-rank items. Using position i directly makes position
    2 twice as bad as position 1, which is too harsh. Log₂(i+1) makes: position 2 is 1/log₂(3)≈0.63x discount,
    position 10 is 1/log₂(11)≈0.29x discount. This reflects that user attention drops but not as sharply
    as linear.'
- id: 02504562-9b41-4e48-a479-2ceb0cc07e56
  type: flip
  front: Compute NDCG@3 for a query ranking [relevant, not, relevant] vs [not, relevant, relevant].
  back: 'Ranking 1: DCG = 1/log₂(2) + 0/log₂(3) + 1/log₂(4) = 1 + 0 + 0.5 = 1.5

    Ranking 2: DCG = 0/log₂(2) + 1/log₂(3) + 1/log₂(4) = 0 + 0.63 + 0.5 = 1.13

    IDCG = [1,1,0] (ideal) = 1 + 0.63 + 0 = 1.63

    NDCG₁ = 1.5/1.63 ≈ 0.92, NDCG₂ = 1.13/1.63 ≈ 0.69. Position matters.'
- id: a99a271a-4bff-43fd-88f0-dc161f522587
  type: mcq
  front: Which metric best evaluates a search engine where users care mostly about finding *any* correct
    answer?
  back: 'MRR focuses on rank of *first* relevant result; perfect for ''first correct answer'' scenarios
    (QA, fact lookup). NDCG rewards having many relevant items; MAP averages precision of all relevant.
    MRR is interpretable: MRR=0.5 means ''on average, first correct answer at rank 2''.'
  choices:
  - key: a
    text: NDCG@10
    correct: false
  - key: b
    text: MRR@5
    correct: true
  - key: c
    text: MAP@10
    correct: false
  - key: d
    text: All equally valid
    correct: false
- id: f4671640-1606-4ae0-bf2c-59aa244205f8
  type: flip
  front: What's the relationship between MAP and NDCG? When would NDCG be preferable?
  back: MAP sums precision@k for each relevant item (unordered by position). NDCG explicitly weights position
    with logarithmic decay. NDCG preferred when position matters (e.g., users scan top-10), MAP preferred
    for evaluating retrieval across many relevant items. NDCG more interpretable (0-1, normalized); MAP
    requires relevance count.
- id: d7a76623-31ae-4b4f-a542-731849fee518
  type: flip
  front: A search system has MRR@5 = 0.2. Interpret this metric.
  back: MRR = 0.2 = 1/(average first-hit rank) → average first hit at rank 5. One of every five queries
    has first correct result at rank 5 (or scattered distribution). Performance is poor; ideal MRR > 0.8.
    System likely needs ranking model retraining or relevance data improvement.
---

## Intuition
Ranking metrics penalize placing relevant items lower in the list. Unlike classification metrics, order matters: returning 1 relevant item at rank 1 is better than at rank 100. These metrics are essential for search engines, recommendation systems, and information retrieval pipelines.

## Detail
For ranked list of $k$ items with relevance $rel_i \in \{0,1\}$ or $\{0,1,2,\ldots\}$ (graded):

- **DCG@k** = $\sum_{i=1}^{k} \frac{rel_i}{\log_2(i+1)}$ → discounts by position; log denominator flattens steep falloff
- **IDCG@k** = optimal DCG (perfect ranking); NDCG@k = $\text{DCG@k} / \text{IDCG@k}$ → normalized to [0,1]
- **AP@k** = $\frac{1}{\text{# relevant}(k)} \sum_{i=1}^{k} P(i) \cdot rel_i$ → precision@i averaged over relevant items
- **MAP** = mean of AP across queries
- **MRR** = $\frac{1}{|Q|} \sum \frac{1}{\text{rank of first relevant}}$ → average reciprocal of first hit rank

NDCG example: Perfect rank [1,1,1] → DCG = 1/log2(2) + 1/log2(3) + 1/log2(4) ≈ 2.63. Shuffled [1,0,1] → DCG ≈ 1.63. NDCG = 1.63/2.63 ≈ 0.62.

## Common gotchas / interview framings
- NDCG plateau: Top-5 differences minor; k-choice important (NDCG@5 vs @100)
- Binary vs graded: binary (0/1) vs multi-level (0-5) relevance; formulas identical, interpretation changes
- AP ambiguity: macro (average per query) vs micro (global count); typically report mean
- MRR sensitivity: zero-shot QA uses MRR@5 (first correct answer rank); fails if multiple correct answers
- Position bias: metrics assume equal exposure; in practice, users click position 1 more (selection bias in labeling)

## See also
- [[discounted-cumulative-gain]]
- [[average-precision]]
- [[reciprocal-rank]]
- [[search-ranking]]
- [[recommendation-systems]]

## Sources
See frontmatter `sources:`.
