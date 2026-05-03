---
id: 6fbfd2ab-2dd9-4e71-ac8f-b7ba103a0abc
title: Retraining triggers and refresh cadence
track: ai-llm-engineer
topic: production-systems-mlops-for-llms
difficulty: 3
tags:
- training
- operations
- cost-benefit
- continuous-learning
- mlops
aliases:
- fine-tuning schedule
- model refresh strategy
- retraining ROI
- automated retraining pipelines
sources:
- url: https://www.rohan-paul.com/p/ml-interview-q-series-handling-llm
  label: 'Rohan Paul: Handling LLM Model Drift'
- url: https://www.braintrust.dev/articles/what-is-llm-monitoring
  label: 'Braintrust: LLM Monitoring'
cards:
- id: 2daa4081-4eca-4eea-8ae5-baa9a008017b
  type: flip
  front: You estimate retraining costs $700 and improves BLEU by 2%. How do you decide if it's worth it?
  back: 'Quantify business impact: 2% BLEU improvement → estimate % of users who notice/care. If 10% of
    users benefit ($10 value per user) = $100 lifetime value gain. Break-even at 7 retrained requests
    (700 / 100). If you have >7 users and the improvement persists >1 month, retrain. If unsure, A/B test:
    deploy candidate to 5% of traffic, measure user satisfaction. If validated, retrain full model.'
- id: 7a37cd1e-1fe9-4c8b-b3ff-2716c043d265
  type: flip
  front: 'You want to implement continuous retraining: daily fine-tuning on yesterday''s data. What''s
    the risk?'
  back: 'Feedback data is biased (angry users comment more). Daily fine-tuning amplifies recent noise.
    Mitigations: (1) Filter data: only label ''clear wins'' (high-quality examples), skip edge cases.
    (2) Held-out test: validate candidate model on static test set before deployment. (3) Slow rollout:
    canary candidate to 5% for 24 hours. (4) Revert circuit: if error rate > 1%, auto-rollback to previous
    version.'
- id: 3085604e-f00b-4d8d-8d76-716b83d95898
  type: mcq
  front: Your LLM-as-Judge labels 10k responses daily. You want to fine-tune nightly on all labeled data.
    What concern should you address?
  back: 'LLM-as-Judge (e.g., GPT-4 grading responses) introduces systematic bias. If GPT-4 grades ''verbose''
    responses as low-quality, fine-tuning on those labels teaches your model to be terse (not necessarily
    good). Mitigation: (1) Validate LLM-as-Judge on human-labeled holdout set. (2) Filter out ambiguous/low-confidence
    labels. (3) A/B test candidate model before promoting.'
  choices:
  - key: a
    text: LLM-as-Judge introduces bias; labels may be noisy
    correct: true
  - key: b
    text: 10k samples is too much; you'll overfit
    correct: false
  - key: c
    text: Nightly retraining is too fast; you should retrain weekly
    correct: false
  - key: d
    text: Fine-tuning on auto-labeled data doesn't improve the base model
    correct: false
- id: c3b4f487-6b0f-45e8-9cf0-9f8a2aefb353
  type: mcq
  front: Retraining improved BLEU from 0.40 to 0.42, but user downvote rate increased from 2% to 2.5%.
    What happened?
  back: BLEU improved but user satisfaction worsened. BLEU measures n-gram overlap, not user preference.
    The fine-tuned model may produce paraphrases (high BLEU) that users dislike, or it may have improved
    on common cases but regressed on edge cases that users encounter. Rollback the fine-tuned version;
    investigate what changed.
  choices:
  - key: a
    text: BLEU improvement didn't translate to user satisfaction; the metric is misleading
    correct: true
  - key: b
    text: The model is worse despite higher BLEU
    correct: false
  - key: c
    text: This is expected; downvote rate always increases with retraining
    correct: false
  - key: d
    text: A/B test on more users to validate
    correct: false
---

## Intuition
Should you retrain/fine-tune when quality drops? The answer is: **only if the cost of retraining is justified by quality gains and business impact**. Retraining is expensive ($1k–10k per run); you need a clear trigger (quality below SLO) and monitoring to know if retraining helped. This section covers deciding when to retrain and how often.

## Detail
**Retraining triggers** (when to retrain):

1. **Quality below SLO**: If BLEU < 0.35 or user downvote rate > 5%, investigate. If root cause is distribution shift (new query type), retraining may help.

2. **Automated triggers**:
   - Run nightly: fine-tune candidate on new labeled data from past 7 days.
   - A/B test candidate vs. prod model on held-out test set.
   - If candidate is better (BLEU +2%, latency same), promote to canary.
   - Auto-promote if canary passes SLOs for 24 hours.

3. **Manual triggers**:
   - After major product launch (new feature → new query types).
   - After competitor analysis (we're losing to X on feature Y).
   - Quarterly refresh cycle (every 3 months, collect new feedback, fine-tune).

**Cost-benefit analysis**:
- Retraining cost: 8 hours GPU ($200), 4 hours eng time ($500) = ~$700 per run.
- Benefit: 2% quality improvement → more users retain, fewer refunds. Quantify: if 1% of users switch due to quality, and each user is worth $100/year, a 1% retention gain = $100k. Then retraining ROI is positive.
- Decision rule: If expected value (prob. of quality improvement × value gained) > cost, retrain.

**Refresh cadence** (how often to retrain):

- **High-traffic systems with fast feedback**: Retrain weekly. Users provide feedback constantly; you can A/B test candidate models. ROI is high because you iterate fast.
- **Medium-traffic with slow feedback**: Retrain monthly or quarterly. Data is sparse; wait to accumulate enough labeled examples for statistical significance.
- **Low-traffic or stable domains (e.g., FAQ bot)**: Retrain on-demand only. Quality is unlikely to degrade; retraining cost isn't justified.

**Data management**:
- **Collection**: Log all queries and responses. Sample high-quality responses for labeling (or use LLM-as-Judge for automatic labeling).
- **Filtering**: Only retrain on in-distribution, high-quality examples. Noisy data hurts fine-tuning. (E.g., don't fine-tune on user refusals; fine-tune on approved response samples.)
- **Versioning**: Track which retraining run produced which model version. If v1.2 is better, revert to v1.2 if v1.3 regresses.

**Online learning** (continuous retraining):
- Extreme case: retrain every request. Too expensive and unstable.
- Practical: retrain nightly on the past 24 hours of data. Test candidate model on held-out data before deploying.
- Risk: if you keep retraining, you may overfit to recent noisy data. Regularization (e.g., L2, early stopping) and held-out tests mitigate.

## Common gotchas / interview framings
- Retraining ≠ improvement. Bad training data (low-quality labels, noisy feedback) makes the model worse. Always validate before promotion.
- Data distribution shift: if you collect feedback from users, feedback distribution is biased (negative feedback more likely). Fine-tuning on biased data amplifies bias.
- Cold-start problem: first retraining run has no historical data. Use best-effort labels (e.g., LLM-as-Judge) to bootstrap.
- Compute budget: if you have limited GPU budget, prioritize high-impact retraining (e.g., new language support) over incremental fine-tuning.

## See also
- [[model-output-drift-quality-change]]
- [[distribution-shift-in-inputs]]
- [[token-usage-and-cost-tracking]]
- [[user-feedback-and-logging]]

## Sources
See frontmatter `sources:`.
