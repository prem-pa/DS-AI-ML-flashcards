---
id: 878dbb38-7a16-4c01-91e2-c4b8d28f9e6a
title: Unified serving frameworks (vLLM distributed, TensorRT-LLM multi-GPU)
track: ai-llm-engineer
topic: serving-infrastructure-inference-optimization
difficulty: 3
tags:
- serving-frameworks
- multi-gpu-serving
- distributed-inference
- orchestration
- production-serving
aliases:
- vLLM-distributed
- TensorRT-LLM-multi-GPU
- LLM-serving
- Distributed-systems
sources:
- url: https://docs.vllm.ai/en/stable/serving/parallelism_scaling/
  label: 'vLLM: Distributed Serving Guide'
- url: https://nvidia.github.io/TensorRT-LLM/advanced/speculative-decoding.html
  label: 'NVIDIA: TensorRT-LLM Distributed'
cards:
- id: 54c96720-07ab-4efd-ba67-c36664f3b166
  type: flip
  front: Why is tensor parallelism (TP) the default for distributed LLM inference, over pipeline or data
    parallelism?
  back: TP splits layers across GPUs with AllReduce per layer (~20ms overhead). PP introduces pipeline
    bubbles (idle GPUs). DP replicates model, requiring large VRAM per GPU. For inference workloads (memory-bandwidth-bound,
    small batch), TP balances throughput vs latency vs simplicity best.
- id: 334bfc83-109c-4f60-b552-f964398ccdad
  type: mcq
  front: Llama 70B needs 140GB VRAM. You have 4x A100 80GB. vLLM auto-config?
  back: ''
  choices:
  - key: a
    text: A) TP-2 (fits on 2 GPUs, replicates to 4 with DP-2)
    correct: false
  - key: b
    text: B) TP-4 (splits across 4 GPUs, one model total)
    correct: true
  - key: c
    text: C) PP-4 (pipes layers)
    correct: false
  - key: d
    text: 'D) Error: doesn''t fit'
    correct: false
- id: a4a29a87-ce3f-4a7e-9caa-3a9bfefa5b4c
  type: flip
  front: "What does this vLLM config achieve?\n```python\nllm = LLM('meta-llama/Llama-2-70b',\n      \
    \    tensor_parallel_size=4,\n          pipeline_parallel_size=1)\n```"
  back: 'TP-4: splits model layers across 4 GPUs. No PP (single pipeline). vLLM orchestrates batch processing
    with AllReduce per layer, targeting high throughput + moderate latency.'
---

## Intuition
Unified serving frameworks abstract away parallelism complexity. Users specify model + hardware; framework handles TP/PP/DP automatically.

**vLLM**: Defaults to TP (tensor parallelism) — best for inference. Minimal changes needed.
**TensorRT-LLM**: Custom distributed runtime; requires explicit parallelism config.

## Detail
### vLLM Distributed
```python
from vllm import LLM

# Framework auto-selects TP-2 if model doesn't fit on one GPU
llm = LLM('meta-llama/Llama-2-70b',
          tensor_parallel_size=2,  # explicit TP-2
          max_num_seqs=64)         # concurrent sequences

# Behind the scenes:
# - Split model via TP across 2 GPUs
# - Continuous batching within each GPU
# - AllReduce per layer (20-30ms total per forward pass)
# - Streaming output over gRPC/HTTP
```

Design choice: **TP is standard**. PP/DP rarely used (TP sufficient for most scales).

### TensorRT-LLM Distributed
```bash
# Compile for distributed inference
trtllm-build --checkpoint_dir ./model \
  --output_dir ./engine \
  --world_size 2 --rank 0 \
  --tp_size 2 --pp_size 1 \
  ...

# Run 2 engine instances (rank 0 on GPU 0, rank 1 on GPU 1)
mpirun -n 2 python inference.py
```

TensorRT requires explicit MPI setup; less automatic than vLLM.

### Scaling Guidelines
| Scenario | Framework | Config |
|----------|-----------|--------|
| 7B on 1 GPU | vLLM | No parallelism |
| 70B on 2 GPU | vLLM | TP-2 (default) |
| 70B on 4 GPU | vLLM | TP-4 |
| 70B on 8 GPU | vLLM | TP-4 + DP-2 (or TP-8, depends on latency SLA) |
| 540B on 16 GPU | vLLM | TP-8 (or TP-4 + DP-4) |

**Rule of thumb**: Start with TP; add DP (data parallelism) only if TP saturates.

## Common gotchas
- **"Should I manually tune parallelism?"** Usually no; vLLM's default TP is near-optimal. Manual tuning for specific latency/throughput targets.
- **"TensorRT-LLM vs vLLM for distributed?"** vLLM is simpler (no compilation, auto-TP). TensorRT is faster (10-30%) but requires MPI/compilation hassle.

## See also
- [[parallelism-strategies]]
- [[serving-architecture]]
- [[load-balancing]]
- [[fault-tolerance]]

## Sources
See frontmatter `sources:`.
