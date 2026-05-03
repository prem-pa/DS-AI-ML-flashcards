---
id: 720020c9-0d00-4007-b1fe-bb75a2c18f72
title: ONNX format and cross-framework compatibility
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- serialization
- interoperability
- standards
- onnx
- model-format
- framework-agnostic
aliases:
- onnx-export
- model-standardization
- framework-neutral-serving
sources:
- url: https://docs.nvidia.com/deeplearning/transformer-engine/user-guide/examples/onnx/onnx_export.html
  label: NVIDIA TensorRT ONNX Export Guide
- url: https://onnx.ai/onnx/intro/
  label: ONNX.ai Introduction
cards:
- id: af7671e0-9502-4c61-9003-0116c3940acd
  type: flip
  front: What is the ONNX computation graph, and why is it useful for serving?
  back: ONNX is a DAG of operators (MatMul, Conv, ReLU, etc.) with fixed I/O tensors. It's useful because
    inference engines can optimize the graph once (fuse ops, reorder, allocate buffers) and execute on
    any hardware without re-exporting.
- id: 42fa5657-8d50-4aed-bec0-696bbaa62831
  type: mcq
  front: You train a model in PyTorch but want to serve it on a mobile device and a GPU cluster. What's
    the best path?
  back: ONNX is the universal intermediate format. Export once, then convert the ONNX to platform-specific
    formats (CoreML, TensorRT, TFLite) or runtimes. Avoids maintaining multiple training-framework-specific
    versions.
  choices:
  - key: a
    text: 'Save two versions: .pth for GPU, .pt for mobile'
    correct: false
  - key: b
    text: Export to ONNX, then convert ONNX to CoreML (mobile) and load into TensorRT (GPU)
    correct: true
  - key: c
    text: Use torch.jit.script and compile separately for each platform
    correct: false
  - key: d
    text: Keep the PyTorch model and use framework-specific deployment tools only
    correct: false
- id: 99b3b6d8-608a-4699-897c-fc1eae2f32dc
  type: flip
  front: Why might ONNX inference produce slightly different outputs than the original PyTorch model?
  back: Precision changes (float32 → float16), unsupported custom ops that fallback to slower implementations,
    shape inference mismatches between tracing (static) vs scripting (dynamic), or missing attributes
    on mapped operators.
- id: b82dab04-4e17-4e13-a863-017dd4ff32cc
  type: flip
  front: Describe the typical ONNX export workflow for a PyTorch model.
  back: 1) Call torch.onnx.export() with dummy inputs; 2) PyTorch traces/scripts the forward pass and
    converts ops to ONNX; 3) ONNX graph is validated; 4) .onnx file (protobuf) is serialized; 5) Post-export
    optimization (quantization, simplification) is optional.
---

## Intuition
ONNX (Open Neural Network Exchange) is a standardized file format for neural networks—think of it as a universal binary that any inference engine can run, regardless of whether the model was trained in PyTorch, TensorFlow, or scikit-learn. Export once, serve anywhere.

## Detail
ONNX represents a model as a computation graph of operators (ops), with input/output tensors and attribute constants. Each operator maps to a standard operation (e.g., MatMul, Conv, ReLU). Advantages:

- **Framework independence**: Train in PyTorch, export to ONNX, serve with TensorRT, ONNX Runtime, TFLite, CoreML, or Triton
- **Graph optimization**: Static shape inference, constant folding, op fusion reduce compute at serving time
- **Runtime selection**: Run on CPU (ONNX Runtime), GPU (TensorRT, cuDNN), mobile (CoreML), or edge (ONNX Runtime)
- **Version stability**: ONNX opset versions allow forward/backward compatibility

Export workflow: `torch.onnx.export()` traces or scripts the model, converts PyTorch ops to ONNX ops, validates the graph, and serializes to a .onnx protobuf file. TensorFlow uses `tf2onnx` converter. After export, quantization (INT8) or graph simplification tools optimize further.

## Common gotchas / interview framings
- "ONNX export failed on a custom layer." → Custom ops require manual mapping or fallback to framework-specific serving
- "Serving latency improved 3x with ONNX+TensorRT—why?" → Graph optimization, op fusion, and kernel tuning unlock hardware-specific speedups
- "ONNX model gives different outputs than the original." → Precision changes (float32 → float16), missing attributes on ops, or shape mismatches in tracing vs scripting

## See also
- [[tensorflow-savedmodel-and-pytorch-torchscript]]
- [[quantization-for-storage-and-inference-speedup]]
- [[tensorflow-serving-torchserve-seldon-core-bentoml]]

## Sources
See frontmatter `sources:`.
