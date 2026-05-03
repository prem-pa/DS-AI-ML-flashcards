---
id: 72b472a4-b18a-48a0-b4a2-49c4edf55c0e
title: TensorFlow SavedModel and PyTorch TorchScript
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- serialization
- computation-graph
- inference
- optimization
- framework-specific
aliases:
- savedmodel
- torchscript
- graph-export
- serving-format
sources:
- url: https://www.tensorflow.org/guide/saved_model
  label: TensorFlow SavedModel Documentation
- url: https://pytorch.org/docs/stable/jit.html
  label: PyTorch TorchScript Documentation
cards:
- id: 10b83d5f-91bf-4572-b3ea-692e0dad563d
  type: flip
  front: What is the key difference between TorchScript tracing and scripting?
  back: Tracing records tensor ops during a forward pass (good for pure data flow, bad for control flow).
    Scripting parses Python code directly (handles if/for loops, but requires type annotations). Tracing
    is simpler; scripting is more flexible.
- id: 2fe7e5c8-f624-4804-8a0e-16a171b5eae4
  type: mcq
  front: 'You have a PyTorch model with conditional logic (if self.training: ...). How do you export it
    for serving?'
  back: Tracing records only the inference-time path (if not self.training). For correct behavior, use
    scripting to preserve conditional logic. Then deploy via TorchServe or custom inference code.
  choices:
  - key: a
    text: Use torch.jit.trace() to record the inference path
    correct: false
  - key: b
    text: Use torch.jit.script() with type annotations to preserve control flow
    correct: true
  - key: c
    text: Convert to SavedModel (TensorFlow only)
    correct: false
  - key: d
    text: Keep the .pth checkpoint and load at serving time
    correct: false
- id: de0fb8fa-0772-46e3-be35-2e4297e41060
  type: flip
  front: Why does TensorFlow Serving prefer SavedModel over raw checkpoints?
  back: SavedModel bundles the computation graph (saved_model.pb), signatures (input/output shapes), and
    weights in one package. Serving can load, validate, and optimize the graph without the original training
    code or TensorFlow eager-mode overhead.
- id: 679afc23-8568-4b96-a8ee-a99e8c232e0f
  type: flip
  front: Describe the directory structure of a TensorFlow SavedModel.
  back: "saved_model/\n  ├─ saved_model.pb (protobuf graph definition)\n  ├─ assets/ (e.g., vocabulary\
    \ files)\n  └─ variables/ (variable shards: checkpoint index, data files)"
---

## Intuition
TensorFlow SavedModel and PyTorch TorchScript are framework-specific serialization formats that capture both the computation graph and weights. Unlike raw checkpoints, they bundle the model definition (layers, operations) so inference engines can optimize and execute without the original training code.

## Detail
**TensorFlow SavedModel**: Represents a TF model (graph, variables, metadata) as a directory with an assets folder, variables (weights), and a saved_model.pb (graph definition in protobuf). Supports eager execution, tf.function tracing, and concrete functions for multiple input signatures. Benefit: TensorFlow Serving natively loads SavedModel; no conversion needed.

**PyTorch TorchScript**: A statically-typed subset of Python compiled to an intermediate representation (IR). Two modes:
- Tracing: Records tensor operations during a forward pass; works for pure data-flow models but fails on control flow (if/for)
- Scripting: Parses Python syntax directly; handles control flow but requires type annotations

Ownage: SavedModel works out-of-box with TensorFlow Serving; TorchScript must be manually integrated into TorchServe or custom inference code. Both enable graph optimization (op fusion, memory planning) for faster inference than eager execution.

## Common gotchas / interview framings
- "SavedModel is huge; why?" → Includes full graph definition + variable shards; compress with quantization or convert to ONNX
- "TorchScript tracing lost my control flow logic." → Use scripting mode instead; annotate types (List[Tensor], Dict[str, float])
- "Eager execution is 2x faster than my TorchScript; why?" → Possible graph bug; benchmark both; scripting overhead may dominate on small models

## See also
- [[model-checkpoints-and-state-dicts]]
- [[onnx-format-and-cross-framework-compatibility]]
- [[tensorflow-serving-torchserve-seldon-core-bentoml]]

## Sources
See frontmatter `sources:`.
