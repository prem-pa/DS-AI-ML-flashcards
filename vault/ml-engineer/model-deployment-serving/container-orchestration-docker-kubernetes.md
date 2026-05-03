---
id: 314ce6d1-2419-4095-b034-11c90ae4217c
title: Container orchestration (Docker, Kubernetes)
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- devops
- containerization
- orchestration
- scaling
- deployment
- infrastructure
aliases:
- docker
- kubernetes
- k8s
- container-runtime
- pod-scheduling
sources:
- url: https://www.docker.com/
  label: Docker Official Site
- url: https://kubernetes.io/docs/
  label: Kubernetes Official Docs
cards:
- id: 977261d5-d9a9-40ae-94af-ff0283dfab9c
  type: flip
  front: What is a Dockerfile, and what should it include for serving a PyTorch model?
  back: 'Dockerfile defines: base image (python:3.11), install deps (pip install torch), copy model and
    code, expose port, set entrypoint (python serving_script.py). Build produces immutable image; run
    produces container.'
- id: 45c42ea7-345f-4c06-92e9-a3163b78c759
  type: mcq
  front: Your model server container takes 30s to start up, delaying rolling updates. How do you speed
    it up?
  back: Preloading model into image avoids download on startup. Readiness probe is good practice but doesn't
    reduce startup time. Light base images also help. SSD is not under your control in cloud K8s.
  choices:
  - key: a
    text: Reduce Docker image size by using lighter base image (python:3.11-slim, not full Ubuntu)
    correct: false
  - key: b
    text: Add readiness probe to detect when model is loaded; K8s won't route traffic until ready, but
      doesn't reduce startup time itself
    correct: false
  - key: c
    text: Preload model in container image (bake model into image, not download at startup)
    correct: true
  - key: d
    text: Use SSD instead of HDD to speed up model I/O
    correct: false
- id: 9a21de0c-c2da-4674-8f2b-af71c4848dc6
  type: flip
  front: What is a Kubernetes Pod, Deployment, and Service? How do they relate?
  back: 'Pod: container(s) in shared network namespace. Deployment: declares desired state (N replicas
    of pod). Service: stable IP/DNS + load balancer pointing to pods. Deployment creates/replaces pods;
    Service routes traffic to live pods.'
- id: 3d2d9b9a-0c52-465b-9fbf-8e69e953388e
  type: flip
  front: Your model server needs a GPU. How do you configure the Kubernetes pod spec?
  back: 'In pod spec, set resource request: `resources: {limits: {nvidia.com/gpu: 1}}`. Node must have
    GPU; K8s scheduler places pod on GPU node. Without request, scheduling may fail or exceed node capacity.'
---

## Intuition
Docker packages model + code + dependencies in a container image. Kubernetes orchestrates (schedules, scales, updates) containers across a cluster. Docker ensures "works on my laptop" → "works in production"; Kubernetes ensures the container scales automatically as traffic changes.

## Detail
**Docker**:
- Dockerfile defines base OS, installs dependencies (torch, tf), copies model, sets entry point
- `docker build` creates image; `docker run` spawns container (isolated process with own filesystem, network)
- Image is immutable; container is ephemeral. Push image to registry (Docker Hub, ECR, GCR); pull and run anywhere
- Overhead: ~100MB per image (OS + libs), ~50-200ms startup (vs. native ~10ms)

**Kubernetes**:
- Cluster: nodes (VMs) + master (scheduler, API server). Pod: smallest unit, wraps 1+ containers
- Deployment: describes desired replicas of a pod; Kubernetes auto-creates, replaces failed pods
- Service: load balancer + DNS; routes traffic to pods
- Autoscaling: HPA (Horizontal Pod Autoscaler) scales replicas based on CPU, memory, or custom metrics (model latency)

**ML-specific setup**:
- Model stored in pod's container image, or mounted from shared storage (NFS, S3, PVC)
- GPU support: node must have GPU; pod requests `nvidia.com/gpu: 1`
- Readiness probe: check if model is loaded; don't route traffic until ready (avoids cold-start).
- Namespace for isolation, RBAC for access control

## Common gotchas / interview framings
- "Container startup is slow; why?" → Large image (→ reduce layer, use .dockerignore), slow model loading (→ use readiness probe), or network latency pulling image
- "Pod keeps crashing." → Check logs: `kubectl logs pod_name`. Common: OOM (model too large), missing dependencies (pip install in Dockerfile), or network issues
- "Autoscaling doesn't kick in." → Check HPA target (metrics-server running?), pod resource requests (must be set for CPU-based scaling), or custom metrics not scraped

## See also
- [[tensorflow-serving-torchserve-seldon-core-bentoml]]
- [[load-balancing-and-horizontal-scaling]]
- [[cold-start-and-model-loading-time]]

## Sources
See frontmatter `sources:`.
