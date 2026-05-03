---
id: afe11a80-97a1-4b2b-87d8-2693adec6f1e
title: Load balancing and horizontal scaling
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- scaling
- load-balancing
- performance
- autoscaling
- availability
- qps
aliases:
- horizontal-scaling
- replica-management
- autoscaling
- qps-scaling
sources:
- url: https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/
  label: Kubernetes HPA Documentation
- url: https://en.wikipedia.org/wiki/Load_balancing_(computing)
  label: Load Balancing Overview
cards:
- id: 701d0261-d334-434a-baad-bad8df17c35d
  type: flip
  front: You have 5 replicas of a model server. Requests arrive at 500 req/s, and each takes 100ms to
    complete. Is load balancing the bottleneck?
  back: No. 5 replicas × 10 req/s per replica = 50 req/s max. You're receiving 500 req/s. You need ~50
    replicas, not better load balancing. Bottleneck is replica count, not load balancer.
- id: e04f9c64-46da-45c2-a24b-90a2a6138f5c
  type: mcq
  front: 'Your autoscaler is flapping: scaling up to 20 replicas, then down to 5, then back up. How do
    you fix it?'
  back: Both mechanisms prevent flapping. Cool-down delays the next scale decision; wider thresholds create
    hysteresis (must drop to 30% to scale down, not 70%). Together, they stabilize.
  choices:
  - key: a
    text: Increase cool-down period (e.g., 5-10min) so HPA doesn't re-evaluate too quickly
    correct: false
  - key: b
    text: 'Widen thresholds: scale up at 80% CPU, down at 30%, not 70% and 50%'
    correct: false
  - key: c
    text: Both A and B; flapping is often caused by too-sensitive thresholds + quick re-evaluation
    correct: true
  - key: d
    text: Disable autoscaling; manually set replica count
    correct: false
- id: 96d946e7-e4fe-4501-bedc-beaf8fc9fe55
  type: flip
  front: Compare round-robin vs. least-connections load balancing for model inference servers.
  back: 'Round-robin: simple, fair if all replicas identical. Least-connections: better if request duration
    varies (don''t overload slow replicas). For ML inference, least-connections is often better because
    some requests may take longer.'
- id: e3809af2-e2ed-4f5b-bc1f-0596af59c1bf
  type: flip
  front: What custom metric would you use for autoscaling a model server, instead of just CPU?
  back: Model inference latency (p99), or queue depth (requests waiting for free replica), or throughput
    (req/s). These directly reflect service quality. CPU can be misleading if inference is I/O-bound (waiting
    for GPU/network).
---

## Intuition
One model replica can handle ~100 requests/second. To serve 1000 req/s, run 10 replicas. Load balancer distributes incoming requests round-robin or least-connections. Autoscaler monitors latency/CPU; if latency rises above SLA, spawn new replicas; if drops, scale down to save cost.

## Detail
**Load balancing**:
- **Round-robin**: Send requests 1 → replica1, 2 → replica2, ..., N+1 → replica1. Simple, works if all replicas are equal
- **Least-connections**: Send request to replica with fewest active connections. Better if request duration varies
- **Weighted**: Some replicas get more traffic (e.g., new hardware 70%, old hardware 30% during upgrade)
- Sticky sessions: Route same client to same replica (used for stateful services; not needed for stateless ML servers)

**Horizontal autoscaling** (Kubernetes HPA):
- Metrics: CPU usage, memory, or custom metrics (latency, QPS from Prometheus)
- Trigger: if avg latency > 100ms, scale replicas × 1.5; if < 50ms, scale down
- Cool-down period: wait 5min after scaling before evaluating again (avoid flapping)
- Min/max: e.g., min 2 (availability), max 50 (cost/quota)

**Vertical scaling** (scaling up): Bigger machine (GPU → more GPUs). Harder than horizontal; usually done once at deployment time.

**Bottlenecks**:
- Model inference latency: parallelize with batching or add replicas
- I/O (loading model): cache in memory, use memory-mapped files
- Networking: optimize protocol (gRPC vs. REST), reduce payload size

## Common gotchas / interview framings
- "We added 10 replicas; throughput only increased 2x." → Not a load-balancing issue; likely model is I/O-bound (GPU, disk) or network-bound (not compute-bound)
- "Autoscaler keeps flapping (up-down-up)." → Increase cool-down period, or adjust thresholds (e.g., scale up at 80% CPU, down at 20%)
- "P99 latency is still high despite scaling." → Possible: (a) requests are not evenly distributed (sticky sessions, hashing), (b) some replicas are slow (node overloaded), or (c) queueing latency (requests wait for free replica)

## See also
- [[batch-inference-vs-online-real-time-inference]]
- [[rest-apis-and-grpc-for-model-endpoints]]
- [[container-orchestration-docker-kubernetes]]

## Sources
See frontmatter `sources:`.
