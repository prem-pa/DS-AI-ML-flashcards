---
id: 0ffd00cc-56aa-473d-b24f-079be3d45e47
title: TensorFlow Serving, TorchServe, Seldon Core, BentoML
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- serving-frameworks
- deployment
- orchestration
- model-management
- inference-server
aliases:
- tf-serving
- pytorch-serving
- model-server
- serving-platform
sources:
- url: https://www.truefoundry.com/blog/model-deployment-tools
  label: Model Deployment Tools Comparison 2026
- url: https://www.kunalganglani.com/learning-paths/ml-engineer/ml-serving-frameworks
  label: ML Serving Frameworks Guide
cards:
- id: 67ce21b9-0a72-4af1-979a-13073028045c
  type: flip
  front: Compare TensorFlow Serving and TorchServe in terms of model format support and customization.
  back: 'TF Serving: SavedModel only, minimal custom logic, configuration-based. TorchServe: .pth/TorchScript,
    custom Python handlers for pre/post-processing, more flexible. Trade-off: TF Serving is simpler; TorchServe
    is more extensible.'
- id: 615c4325-017a-42e8-9cca-47c6fef146cb
  type: mcq
  front: Your team uses PyTorch, but you want framework-agnostic serving to future-proof. Should you use
    TorchServe, BentoML, or Seldon Core?
  back: 'Seldon Core is most future-proof: wrap any model in a container, serve via Kubernetes. BentoML
    is simpler but less battle-tested. TorchServe locks you to PyTorch ecosystem.'
  choices:
  - key: a
    text: TorchServe; it's the official PyTorch server
    correct: false
  - key: b
    text: BentoML; framework-agnostic, easy to use, export to Docker
    correct: false
  - key: c
    text: Seldon Core with Kubernetes; full framework agnosticity, A/B testing built-in
    correct: true
  - key: d
    text: Build custom inference server to avoid vendor lock-in
    correct: false
- id: 7f0cc634-3098-4b0d-af6f-423839c80e1b
  type: flip
  front: What is KServe, and how does it relate to TensorFlow Serving and TorchServe?
  back: KServe is a Kubernetes operator that standardizes serving for multiple backends (TF Serving, TorchServe,
    Triton, Seldon). Use KServe to deploy, version, and canary models across frameworks on Kubernetes
    with a unified API.
- id: 341fd2de-b295-405c-8f9a-afe1d3fb26df
  type: flip
  front: When would you choose Triton Inference Server over TensorFlow Serving or TorchServe?
  back: 'Triton: supports ONNX, TensorRT, TF, PyTorch in one server; highly optimized for GPU; streaming
    RPC; industry standard for high-throughput inference. Choose Triton if you need multi-framework support
    or GPU optimization is critical.'
---

## Intuition
These are pre-built servers that load models and expose HTTP/gRPC endpoints. Instead of building inference from scratch, you point them to a SavedModel or .pth, and they handle batching, versioning, autoscaling, monitoring. Different frameworks support different model types and have different operational overhead.

## Detail
**TensorFlow Serving**: Optimized for TensorFlow SavedModel. Features: multi-model serving, batching, versioning (A/B), gRPC/REST endpoints, model warming. Drawback: TensorFlow-only; complex Kubernetes setup; harder to customize inference logic.

**TorchServe** (AWS/PyTorch): For PyTorch models (.pth, TorchScript). Features: custom inference handlers (pre/post-process), batch inference, multi-worker scaling, metrics/logging. Drawback: PyTorch-only; less mature than TF Serving; custom logic requires Python code.

**Seldon Core**: Kubernetes-native, framework-agnostic. Deploy any model (TF, PyTorch, scikit-learn) in a Docker container. Features: A/B testing, shadow traffic, canary, monitoring via Prometheus. Drawback: requires Kubernetes; more ops overhead; slower inference (containerized model).

**BentoML**: Framework-agnostic, Python-based. Package model + custom code as a service. Features: easy to use, multi-framework, built-in A/B/canary, Docker export. Drawback: Python runtime (slower than native), less battle-tested than TensorFlow Serving.

**Positioning (2025-2026)**:
- **KServe**: Kubernetes operator for TF Serving, TorchServe, Triton, Seldon; emerging standard, best for cloud-native Kubernetes deployments
- **Triton Inference Server** (NVIDIA): Multi-framework, GPU-optimized, gRPC/HTTP, batching; industry standard for high-performance inference
- **Ray Serve**: Distributed inference framework, integrates with Triton; good for custom Python logic and distributed workloads

## Common gotchas / interview framings
- "TF Serving is slow; why?" → Possible network latency (gRPC), large model load time, or misconfigured batching timeout
- "Customizing inference in TF Serving is hard." → Use SavedModel with custom ops, or switch to TorchServe/BentoML for easier pre/post-processing
- "Which framework for multi-framework shop?" → Seldon Core or KServe on Kubernetes; BentoML if no K8s available

## See also
- [[tensorflow-savedmodel-and-pytorch-torchscript]]
- [[onnx-format-and-cross-framework-compatibility]]
- [[container-orchestration-docker-kubernetes]]

## Sources
See frontmatter `sources:`.
