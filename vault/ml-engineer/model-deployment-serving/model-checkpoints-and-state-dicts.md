---
id: 956e62c3-460a-4e0b-b154-9c37f599b747
title: Model checkpoints and state dicts
track: ml-engineer
topic: model-deployment-serving
difficulty: 1
tags:
- serialization
- model-format
- pytorch
- tensorflow
- persistence
- deployment
aliases:
- model-weights
- checkpoint-saving
- state-preservation
sources:
- url: https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/index.html
  label: NVIDIA Triton Inference Server Docs
- url: https://pytorch.org/docs/stable/notes/serialization.html
  label: PyTorch Serialization Guide
cards:
- id: 9b40ca74-f0a0-4105-a2bb-0ee0831f0e3a
  type: flip
  front: What is the difference between a full checkpoint and a state dict in PyTorch?
  back: 'A full checkpoint includes model.state_dict(), optimizer.state_dict(), epoch, loss—everything
    needed to resume training. A state dict is just {param_name: tensor}; used for inference deployment.'
- id: d3095de2-55c1-4327-aedd-d5dba0b991fc
  type: flip
  front: Why can deserialization fail across PyTorch versions?
  back: Parameter names, tensor shapes, or layer definitions may change between versions. Pickle loads
    fail if class definitions differ. Pin PyTorch version or use format-agnostic ONNX/SavedModel.
- id: 86e7bc77-c6a7-4662-8597-106ec5eae3da
  type: mcq
  front: You need to deploy a PyTorch model to production. Which approach minimizes cold-start latency?
  back: Quantization reduces file size, memory-mapping avoids eager loading, and streaming loaders parallelize
    I/O. These minimize model deserialization latency on cold start. Cloud fetches add network overhead.
  choices:
  - key: a
    text: Save the full checkpoint (model + optimizer state) as pickle
    correct: false
  - key: b
    text: Extract state_dict, quantize to INT8, use memory-mapped file or streaming loader
    correct: true
  - key: c
    text: Store weights in a cloud database and fetch during inference
    correct: false
  - key: d
    text: Use torch.jit.script on the training script directly
    correct: false
- id: b0309007-0640-41f0-bfe2-7fb9c3e68110
  type: flip
  front: How do you safely save a model for long-term deployment without version lock?
  back: Convert to ONNX (framework-agnostic) or SavedModel (TF) after training. These formats capture
    the computation graph and weights; deserialization doesn't depend on training framework version.
---

## Intuition
A model checkpoint is a snapshot of your trained neural network: weights, biases, architecture metadata, and optimizer state. Think of it as a save-file for a game—you capture the entire state so you can resume later without retraining.

## Detail
PyTorch's `torch.save()` stores the model state dict (parameter tensors) as a pickle file, while TensorFlow checkpoints use a checkpoint protocol with .ckpt metadata and .ckpt-data shards. For deployment, you extract just the inference-time weights (state dict), discarding optimizer state. Key considerations:

- **State dict format**: Dictionary mapping parameter names to tensor values; framework-agnostic structure
- **Serialization methods**: pickle (PyTorch), protobuf (TensorFlow), HDF5 (legacy), or standardized formats like ONNX
- **Version compatibility**: Mismatched PyTorch/TF versions can fail deserialization; pin versions in production
- **Size**: Large models (GPT-3, 175B params) require sharded checkpoints and streaming loaders to avoid OOM during load

## Common gotchas / interview framings
- "How do you handle checkpoint bloat?" → Use pruning, quantization pre-save, or only serialize inference weights (drop optimizer states)
- "Your checkpoint won't load in production." → Version mismatch; use ONNX or SavedModel for frame-agnostic export
- "Startup latency is killing us." → Use memory-mapped checkpoints or lazy-load model layers on-demand during warmup

## See also
- [[onnx-format-and-cross-framework-compatibility]]
- [[tensorflow-savedmodel-and-pytorch-torchscript]]
- [[container-orchestration-docker-kubernetes]]

## Sources
See frontmatter `sources:`.
