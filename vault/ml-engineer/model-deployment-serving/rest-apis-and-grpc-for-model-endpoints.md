---
id: 5d663629-ffd9-4d28-8a5c-702f7d8a14b1
title: REST APIs and gRPC for model endpoints
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- api
- networking
- protocol
- grpc
- rest
- performance
aliases:
- http-inference
- grpc-serving
- protobuf
- binary-protocol
sources:
- url: https://grpc.io/docs/
  label: gRPC Documentation
- url: https://en.wikipedia.org/wiki/Representational_state_transfer
  label: REST API Overview
cards:
- id: 1b6ba22e-5afb-4ebc-b496-422d0586260f
  type: flip
  front: Compare the serialization overhead of REST (JSON) vs. gRPC (Protocol Buffers).
  back: 'REST: JSON is human-readable but verbose (~30-50 bytes for small data). gRPC/Protobuf: compact
    binary (~5-10 bytes). Trade-off: gRPC is 3-5x smaller but requires proto schema + code generation.'
- id: be7269dd-6320-4ba5-ad77-ff2307895c34
  type: mcq
  front: You need to serve predictions at 10,000 requests/second with <5ms latency. REST or gRPC?
  back: gRPC + HTTP/2 is required for <5ms latency at 10k req/s. HTTP/2 multiplexes requests on one connection;
    Protobuf is binary and fast. REST + gzip still has JSON parsing overhead.
  choices:
  - key: a
    text: REST with JSON—easy to debug and widely supported
    correct: false
  - key: b
    text: gRPC with Protocol Buffers and HTTP/2 multiplexing
    correct: true
  - key: c
    text: REST with JSON compression (gzip)
    correct: false
  - key: d
    text: Either; latency is determined by model, not protocol
    correct: false
- id: c6387e7e-2f0a-4741-b971-92f3c683a158
  type: flip
  front: Name one scenario where you'd use REST instead of gRPC for a model endpoint.
  back: Browser-based UI (requires HTTP), external third-party integrations (may not support gRPC), debugging
    (curl is easier), or low-throughput services where latency is not critical.
- id: 5fcddbe4-e4c7-4701-874c-932db4e96301
  type: flip
  front: How does HTTP/2 improve gRPC performance compared to HTTP/1.1 REST?
  back: HTTP/2 multiplexes multiple streams over one connection (no head-of-line blocking), uses binary
    framing (faster parsing), and enables server push. Reduces latency by ~50% compared to serial HTTP/1.1
    requests.
---

## Intuition
REST uses HTTP (text-based JSON), simple but verbose and slow. gRPC uses HTTP/2 with binary Protocol Buffers, faster and streaming-capable. REST is common for web UIs; gRPC is preferred for high-throughput ML services.

## Detail
**REST API**: Client sends HTTP POST with JSON body `{"inputs": [...]}`, server parses JSON, runs inference, returns JSON response. Overhead: JSON serialization/parsing (slow), plain-text HTTP headers, connection reuse (HTTP/1.1 limited). Latency per call: 10-50ms including network roundtrip.

**gRPC**: Client calls remote method as if it were local (stub), request is serialized to Protocol Buffers (compact binary), sent over HTTP/2 (multiplexed streams), server deserializes and responds. Overhead: minimal (binary format), connection reuse built-in, streaming supported. Latency: 1-5ms for same hardware as REST.

Trade-offs:
- REST: Easy to debug (curl), browser-friendly, polyglot (any language). gRPC: Fast, type-safe (proto schemas), streaming, but requires gRPC clients.
- Bandwidth: gRPC ~10x smaller for same data (binary vs. JSON)
- Latency: gRPC ~5-10x lower (fewer roundtrips, no parsing)

Practice: Use REST for browser-based UIs, external APIs. Use gRPC for internal services, high-frequency requests, or streaming predictions. Mix both if needed (gRPC backend, REST gateway in front).

## Common gotchas / interview framings
- "gRPC latency is higher than REST—why?" → Possible proxy/networking issue, misconfiguration (HTTP/1.1 instead of HTTP/2), or Protobuf encoding overhead on small messages
- "We switched to gRPC and lost 30% throughput." → Likely custom serialization overhead; benchmark with profiler
- "Can we use REST for real-time predictions?" → Yes, but expect higher latency; batch requests if possible

## See also
- [[batch-inference-vs-online-real-time-inference]]
- [[load-balancing-and-horizontal-scaling]]
- [[model-versioning-and-ab-testing-in-production]]

## Sources
See frontmatter `sources:`.
