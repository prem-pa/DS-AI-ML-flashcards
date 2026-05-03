---
id: f348571f-99eb-4b60-9941-22dd6bfc443e
title: Model versioning and canary rollouts
track: ai-llm-engineer
topic: production-systems-mlops-for-llms
difficulty: 3
tags:
- deployment
- versioning
- canary
- safety
- monitoring
aliases:
- shadow traffic testing
- gradual rollout
- model A/B testing
- version comparison
sources:
- url: https://www.braintrust.dev/articles/ab-testing-llm-prompts
  label: Braintrust A/B Testing for LLM Prompts
- url: https://www.dynatrace.com/news/blog/the-rise-of-agentic-ai-part-6-introducing-ai-model-versioning-and-a-b-testing-for-smarter-llm-services/
  label: Dynatrace AI Model Versioning & A/B Testing
- url: https://www.traceloop.com/blog/the-definitive-guide-to-a-b-testing-llm-models-in-production
  label: Traceloop A/B Testing Guide
cards:
- id: dc42779e-310e-44e0-a6e9-7e5c60d47a68
  type: flip
  front: You're deploying a smaller, cheaper model (gpt-4o-mini) to replace gpt-4o. You want to detect
    if quality drops. Describe a canary rollout strategy.
  back: '1. Route 5% of traffic to mini; 95% to gpt-4o. 2. Log version ID and track metrics: latency,
    cost/token, user feedback, BLEU/ROUGE on a sample. 3. After 24h, compare metrics. If cost drops >30%
    but BLEU drops >5%, reconsider or invest in prompt tuning. If BLEU degradation < 2%, promote to 25%,
    repeat.'
- id: f3cdf452-b8cc-424e-8e49-2f63789ceb71
  type: flip
  front: What is shadow traffic and why is it useful for model rollouts?
  back: Shadow traffic duplicates production requests and sends them to the new model, but the response
    is discarded (user sees only the stable model's response). It lets you measure new model quality on
    real production data without affecting users. Useful for offline quality metrics (BLEU, hallucination
    detection).
- id: 3ad556b2-69f4-4aa6-8952-4606266c0c79
  type: mcq
  front: During a canary rollout, you notice the new model has 2x latency but users report slightly better
    answers. What should you do?
  back: SLOs define acceptable tradeoffs. If your SLO is 'p99 < 5s' and canary is 3s (within SLO), the
    latency increase is acceptable if it brings measurable quality gains. Blindly prioritizing speed or
    quality ignores business context.
  choices:
  - key: a
    text: Immediately roll back; latency is non-negotiable
    correct: false
  - key: b
    text: Promote to 100% because quality matters more
    correct: false
  - key: c
    text: 'Check your SLOs: if p99 latency SLO allows the increase and user satisfaction metric justifies
      it, gradually increase canary %'
    correct: true
  - key: d
    text: A/B test the latency increase with a subset of users
    correct: false
- id: 2e7ae23c-0112-4a43-89b7-fcd7c60a482d
  type: mcq
  front: You're versioning model checkpoints. Which approach is most maintainable for production rollouts?
  back: A model registry centralizes versioning, metadata (latency, accuracy, training date), and rollout
    history. MLflow and HuggingFace Model Hub integrate with CI/CD. Git LFS scales poorly for large models;
    manual copy-paste invites errors.
  choices:
  - key: a
    text: Store checkpoints in a shared S3 bucket with timestamped folders
    correct: false
  - key: b
    text: Use a model registry (e.g., HuggingFace Model Hub, MLflow) with semantic versioning and metadata
      tags
    correct: true
  - key: c
    text: Keep model files in git LFS with branch names for versions
    correct: false
  - key: d
    text: Manual copy-paste of model files to a deployment server
    correct: false
---

## Intuition
Model versioning answers: 'How do I safely roll out a new model without risking production quality?' Canary rollouts reduce risk by routing a small percentage (5–10%) of traffic to the new version ('canary') while the rest uses the stable ('control') version. You monitor both versions for quality/latency/cost, and if the canary looks good, gradually increase its traffic; if it degrades, roll back immediately.

## Detail
Key concepts:
- **Shadow traffic**: Route a copy of production traffic to the new model without returning its response to the user. Measure quality offline to compare against control.
- **Canary deployment**: Route small percentage (e.g., 5%) of real traffic to the new model; measure on-line metrics (latency, error rate, user feedback). If SLOs hold, gradually increase percentage (10% → 25% → 100%).
- **Version tags**: Label model releases with semantic versioning (v1.2.0) and metadata (e.g., v1.2.0-fast-inference, v1.2.0-low-cost). Maintain a git repo or registry of model checkpoints with commit history.

Common patterns:
- **A/B test harness**: Route requests to control (v_old) or canary (v_new) using feature flags or probabilistic assignment. Log version ID with every trace for later analysis.
- **Automated rollback**: If canary latency > control by >20% or error rate > 5%, automatically roll back.
- **Multi-metric evaluation**: Compare latency, token cost, error rate, user thumbs-up/down, and custom quality metrics (BLEU, exact-match) before promoting canary.

## Common gotchas / interview framings
- How long should a canary run? At least 24 hours to capture daily traffic patterns; overnight traffic patterns differ from daytime.
- What if the canary is better but slower? You're measuring latency vs quality tradeoff; define SLOs that allow acceptable latency increase for quality gain.
- How do you test new prompts without new model code? Prompt versioning (in Langfuse, etc.) allows A/B testing prompts against the same model.
- What's the rollout risk if you have 10k requests/second? A 1% canary = 100 req/s against new model. Unlikely to see rare bugs; consider 5–10% for safety.

## See also
- [[load-balancing-and-autoscaling-for-llm-endpoints]]
- [[model-output-drift-quality-change]]
- [[latency-throughput-error-rates-cost]]
- [[user-feedback-and-logging]]

## Sources
See frontmatter `sources:`.
