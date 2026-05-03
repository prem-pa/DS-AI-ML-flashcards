---
id: b566c232-8e3c-494f-9241-476b6eb0d506
title: Manual evaluation and annotation
track: ai-llm-engineer
topic: evaluation-benchmarking
difficulty: 1
tags:
- ground-truth
- manual-review
- annotation
- gold-standard
- human-judgment
aliases:
- human evaluation
- gold standard
- annotation workflows
- inter-rater agreement
sources:
- url: https://www.braintrust.dev/articles/deepeval-alternatives-2026
  label: DeepEval Alternatives 2026
- url: https://www.confident-ai.com/knowledge-base/compare/best-ai-evaluation-tools-2026
  label: Best AI Evaluation Tools 2026
cards:
- id: e7194435-8e1c-4831-b997-fc43dedf6c54
  type: flip
  front: Why is manual evaluation still necessary even though automated frameworks like DeepEval and Ragas
    exist?
  back: Automated metrics (LLM-as-judge, learned models) are validated against gold-standard manual labels.
    All automation ultimately trusts human judgment as ground truth. Manual evaluation also reveals metric
    failure modes.
- id: 1abe2b97-e1aa-4cc9-9c3c-06ad153c6d68
  type: mcq
  front: You're building a gold-standard test set for a medical LLM. Multiple annotators gave conflicting
    correctness labels. What should you do?
  back: Disagreement signals ambiguity. Discuss to understand different perspectives; use domain expert
    as tiebreaker. Document reasoning for future reference.
  choices:
  - key: a
    text: Use majority vote
    correct: false
  - key: b
    text: Resolve via discussion or expert
    correct: true
  - key: c
    text: Exclude the example
    correct: false
  - key: d
    text: Use either label randomly
    correct: false
- id: 9c6b8a13-9697-4955-bc27-8718b0332636
  type: flip
  front: What metric quantifies consistency among multiple human annotators?
  back: Cohen's kappa (2 annotators) or Fleiss' kappa (3+ annotators). Kappa accounts for chance agreement;
    high kappa (>0.80) indicates strong consistency, validating annotation quality.
- id: bba0c23d-3273-4c0e-9765-12d0c43e9caa
  type: mcq
  front: Typical cost per manual annotation is approximately which range?
  back: Manual annotation typically costs $10–50 per example depending on task complexity. A 100-example
    set = $1K–5K, limiting practical scale.
  choices:
  - key: a
    text: $0.50–$2
    correct: false
  - key: b
    text: $10–$50
    correct: true
  - key: c
    text: $100–$500
    correct: false
  - key: d
    text: $1000+
    correct: false
---

## Intuition
All automated evaluation (benchmarks, LLM-as-judge, metrics) is validated against manual evaluation. Humans remain the gold standard for assessing LLM quality, but manual evaluation is expensive, slow, and inconsistent. The challenge is scaling human judgment without sacrificing quality.

## Detail
**When to use manual evaluation**:
- **Initial dataset creation**: Building test sets; humans define correct answers.
- **Metric calibration**: After selecting automated metrics (e.g., LLM-as-judge), compare to manual samples to validate.
- **Error analysis**: When automated metrics disagree or models perform unexpectedly, humans resolve ambiguities.
- **High-stakes domains**: Medical, legal, financial LLMs require human review; no benchmark fully captures correctness.

**Practical workflow**:
1. Define annotation guidelines (clarity matters for consistency).
2. Sample test set (10–50 examples for metric calibration).
3. Multiple annotators per sample (3+ for controversial tasks).
4. Compute inter-rater agreement (Cohen's kappa, Fleiss' kappa).
5. Resolve disagreements via discussion or gold-standard annotator.
6. Compare automated metrics to manual labels; calibrate thresholds.

**Cost**: ~$10–50 per annotation depending on task complexity. A 100-sample test set = $1K–5K per round. Limits practical use but essential for validation.

## Common gotchas / interview framings
- **Annotation ambiguity**: Unclear guidelines lead to low agreement. Spend time writing examples; test on small sample first.
- **Annotator fatigue**: Human raters drift over time. Rotate tasks, embed control questions to detect drift.
- **Selection bias**: Manual evaluation is biased toward examples humans are confident about. Use systematic sampling, not cherry-picked examples.
- **Interview framing**: "How would you establish a gold-standard test set for an LLM product?" Define clear guidelines, recruit domain experts, measure agreement, then use this set to validate automated metrics.

## See also
- [[manual-evaluation]]
- [[annotation]]
- [[gold-standard]]
- [[inter-rater-agreement]]
- [[cost-vs-quality]]
- [[human-in-the-loop]]

## Sources
See frontmatter `sources:`.
