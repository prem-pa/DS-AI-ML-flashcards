---
id: 5bcfa1ae-2e61-4805-8db3-7b2afd4cc871
title: Model versioning and A/B testing in production
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- deployment
- testing
- canary
- shadow-traffic
- version-control
- a-b-testing
aliases:
- canary-rollout
- shadow-traffic
- model-comparison
- blue-green-deployment
sources:
- url: https://kserve.github.io/website/docs/model-serving/predictive-inference/frameworks/overview
  label: KServe Model Serving Overview
- url: https://en.wikipedia.org/wiki/Canary_release
  label: Canary Release Strategy
cards:
- id: 71a1ad9b-4ba7-4cd7-a17d-004e1825e776
  type: flip
  front: Explain the difference between canary rollout and shadow traffic deployment.
  back: 'Canary: route 1% → 10% → 100% traffic to new model; users see predictions; fast rollback if metrics
    degrade. Shadow: run new model on 100% requests but don''t show predictions; compare offline; no user
    impact but more infrastructure.'
- id: a2b9ba76-2b7d-43c2-a526-d4df8b739ca9
  type: mcq
  front: Your new model has higher accuracy but 20ms higher latency. How would you deploy it safely?
  back: Both canary and shadow are valid. Canary exposes risk to users (acceptable if SLA buffer exists).
    Shadow is safer (users unaffected) but requires 2x infrastructure. Choose based on SLA tolerance and
    infrastructure budget.
  choices:
  - key: a
    text: 'Canary rollout: 1% → 10% → 100% over days; monitor latency, only increase % if p99 latency
      <SLA+5ms'
    correct: false
  - key: b
    text: 'Shadow traffic: run new model parallel to old, compare latency/accuracy offline, roll out if
      differences are acceptable'
    correct: true
  - key: c
    text: Full rollout immediately; users will accept latency increase for better accuracy
    correct: false
  - key: d
    text: Keep old model; don't deploy because latency is unacceptable
    correct: false
- id: a9d54a62-7236-4cc6-ac97-59a129fd5d80
  type: flip
  front: What metrics would you monitor during a canary rollout to decide whether to increase traffic
    % or roll back?
  back: Latency (p50, p99), error rate, model accuracy (if ground truth is available quickly), business
    metrics (conversion, revenue), and resource usage. Rollback if any metric breaches SLA by >5%.
- id: 823b005f-2da4-4787-b347-25d1bfc20715
  type: flip
  front: Describe the blue/green deployment strategy and its pros/cons.
  back: 'Two identical environments (blue, green); run new model on green, route 100% traffic from blue→green
    instantly. Pros: instant rollback. Cons: 2x infrastructure cost, no gradual validation, higher blast
    radius if bug exists.'
---

## Intuition
A/B testing means running two model versions (old, new) simultaneously and comparing metrics (accuracy, latency, business KPI) before fully switching. Canary rollout: route 1% traffic to new model, monitor for errors/latency, increase % over hours/days. Shadow traffic: silently run new model in parallel, compare outputs without affecting users.

## Detail
**A/B testing strategies**:
- **Canary**: 1% → 10% → 50% → 100% traffic over time; fast rollback if error rate spikes
- **Shadow/dark launch**: New model runs on 100% requests but predictions are not returned to user; compare accuracy, latency offline
- **Blue/green**: Two identical environments (blue, green); switch router instantly; enables fast rollback but uses 2x resources
- **Multi-armed bandit**: Dynamically adjust routing based on real-time reward signal (e.g., click-through rate)

**Implementation**:
- Load balancer or proxy (e.g., Envoy, Nginx, Kubernetes Ingress) splits traffic by percentage
- Log new model predictions alongside ground truth for offline analysis
- Monitor: latency, error rate, business metrics (revenue, precision, recall)
- Rollback threshold: if latency +50% or error rate >1%, revert to old model

**Versioning**: Tag models (v1.0, v1.1, v2.0); store metadata (training date, dataset, hyperparams) in model registry. Associate version with git commit, training run, and deployment timestamp for reproducibility.

## Common gotchas / interview framings
- "Canary test shows no difference, but full rollout is slow." → Sample size too small in canary; increase % or duration before incrementing
- "A/B test is biased: new model only served to subset of users." → Use stratified sampling or synchronized time-based comparison
- "Old model crashed; can't quickly roll back." → Blue/green or hot standby required; canary alone is risky without fast rollback mechanism

## See also
- [[rest-apis-and-grpc-for-model-endpoints]]
- [[load-balancing-and-horizontal-scaling]]
- [[caching-and-memoization]]

## Sources
See frontmatter `sources:`.
