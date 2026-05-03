---
id: 92f43ac5-258c-4481-8c22-929972cd4a3c
title: Model stealing and intellectual property
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 5
tags:
- model extraction
- IP protection
- API security
- query throttling
- surrogate model
aliases:
- model extraction attacks
- knowledge distillation attack
- model piracy
sources:
- url: https://www.ml4devs.com/what-is/model-evaluation-metrics/
  label: 'ML4Devs: Model Evaluation Metrics'
- url: https://www.evidentlyai.com/classification-metrics
  label: 'Evidently AI: Classification Metrics'
cards:
- id: 6cf46d43-cace-4017-aac3-e640a302a30b
  type: flip
  front: Explain model extraction attack. How many queries does an attacker need to steal a binary classifier?
  back: 'Attacker queries model API with inputs, collects predictions, trains surrogate model. Query complexity
    ~O(d) to O(2^d) where d = input dimensions (features). For binary classifier with d=100 features,
    ~1000-10000 queries suffice to approximate decision boundary locally. Global extraction ~100K+ queries.
    Cost: if $0.01/query, extraction costs $1000+. Feasible for valuable models.'
- id: d3e5d645-bd7b-4f34-896a-eaa8b75839eb
  type: flip
  front: Propose three mitigations for model stealing. Which is most practical?
  back: '1. Query throttling (100/day/IP): slows extraction but doesn''t prevent if budget large. 2. Output
    quantization (round confidence): leaks less info/query. 3. Prediction watermarking (embed signal):
    ownership proof but detection requires model access. Most practical: combine (1) + (2). Throttling
    is cheapest; quantization acceptable user-experience tradeoff. Watermarking adds detection but doesn''t
    prevent extraction.'
- id: 3086d818-debd-4b7b-8e79-3dd08583337c
  type: mcq
  front: Your API returns only top-3 predictions instead of full softmax. How does this mitigate model
    stealing?
  back: Reduces information leaked per query. Attacker gets top-3 class probabilities, not full softmax
    (~10 classes). Knowledge distillation (core of extraction) relies on soft labels; fewer outputs =
    harder to train surrogate. Combined with throttling, effective defense.
  choices:
  - key: a
    text: Prevents extraction entirely
    correct: false
  - key: b
    text: Reduces info per query (less soft-label information)
    correct: true
  - key: c
    text: Increases query cost proportionally
    correct: false
  - key: d
    text: No effect
    correct: false
- id: 95f66b4b-c8d9-40ca-87ef-c74a67a394cb
  type: flip
  front: An attacker extracted your text classifier via API queries. How would you detect this?
  back: 'Monitor for: (1) unusual query patterns (systematic exploration of input space, boundary-crossing
    inputs), (2) high volume from single IP (e.g., 10K queries in 1 hour), (3) low human-like diversity
    (attacker often reuses features), (4) sudden surge in failed predictions (attacker testing edge cases).
    Implement query logging + anomaly detection (clustering of query patterns).'
- id: a49de113-b398-48b3-913b-6a6f85973de8
  type: flip
  front: Model A is embedded in web app (no API). Model B is available via API. Which is more vulnerable
    to stealing?
  back: Model B (API) much more vulnerable. Attacker can query freely, collect predictions, train surrogate.
    Model A is client-side (black-box but no query interface); extraction requires reverse-engineering
    or local probing. In practice, web app models still extractable (via repeated interactions) but require
    more work. API = easy access = high risk. For sensitive IP, keep model server-side, limit query exposure.
---

## Intuition
Model stealing exploits API access to extract intellectual property. Attacker queries model API (black-box), collects input-output pairs, trains surrogate model mimicking original. Similar to knowledge distillation but adversarial. Mitigations: query throttling, output quantization, prediction watermarking, or input perturbation.

## Detail
**Extraction attack**:
1. Attacker queries API with inputs X, collects predictions Ŷ
2. Trains surrogate model on (X, Ŷ) pairs
3. Surrogate approximates original model's decision boundary
4. Cost: queries × price per query; can be thousands to millions

**Attack query complexity**:
- Decision boundary region: requires O(d) to O(2^d) queries (d = feature dimensions) to approximate locally
- Global extraction: millions of queries for high-fidelity copy
- Example: text classifier extracted with ~500K queries (Carlini et al., 2020)

**Mitigations**:
- **Query throttling**: limit queries/user/time (e.g., 100 queries/day/IP). Slows extraction but doesn't prevent if budget large.
- **Output quantization**: round predictions (e.g., confidence 0.876 → 0.88). Reduces information leaked per query.
- **Confidence masking**: return only top-k classes, not full softmax. Query attacker learns less per query.
- **Prediction watermarking**: embed imperceptible signal in predictions; owns extracted models. Backdoor-like trigger.
- **Input perturbation**: add noise to inputs before processing; disrupts extraction. Risks accuracy degradation.
- **Model ensemble + disagreement**: use multiple models, return majority vote. Harder to extract individual models.

**Detection**:
- Monitor query patterns: adversarial attacker shows systematic exploration (grid search, boundary crossing)
- Sudden surge in failed predictions: attacker testing edge cases
- Unusual input distribution: attacker may not follow natural distribution

## Common gotchas / interview framings
- Trade-off accuracy vs security: strong throttling/quantization degrades user experience
- Query budget visible: if API charges per query, cost is signal; expensive extraction deters amateur but not well-funded attacker
- Surrogate accuracy: surrogate rarely matches original exactly (~5-10% gap typical); but sufficient for piracy in many domains
- Watermarking effectiveness: if attacker retrains, watermark may be lost; combine multiple defenses
- Regulatory gap: no international IP protection for ML models (unlike software); legal remedies limited

## See also
- [[model-extraction]]
- [[knowledge-distillation]]
- [[api-security]]
- [[output-quantization]]
- [[prediction-watermarking]]

## Sources
See frontmatter `sources:`.
