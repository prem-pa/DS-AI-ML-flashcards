---
id: 28cde7a9-c00b-4c0c-8cd5-1155779e883e
title: Data parallelism for inference
track: ai-llm-engineer
topic: serving-infrastructure-inference-optimization
difficulty: 3
tags:
- data-parallelism
- multi-gpu
- request-parallelism
- horizontal-scaling
- synchronization
aliases:
- Request-parallelism
- Model-replication
- Horizontal-scaling
sources:
- url: https://docs.vllm.ai/en/stable/serving/parallelism_scaling/
  label: 'vLLM: Parallelism & Scaling Guide'
- url: https://bentoml.com/llm/inference-optimization/data-tensor-pipeline-expert-hybrid-parallelism
  label: 'BentoML: Parallelism Types (2026)'
cards:
- id: 1bf24292-0a1f-499c-9d4f-187e04c8d3c7
  type: flip
  front: Why does data parallelism for inference scale perfectly (linear) while training does not?
  back: In inference, each GPU processes an independent request; no synchronization needed. Throughput
    scales linearly. In training, gradients must be synchronized across replicas (AllReduce), which adds
    communication overhead, limiting scaling efficiency.
- id: d413d852-d83f-4eb3-990c-402e83f9b905
  type: mcq
  front: 'Llama 70B on 4x A100 80GB with data parallelism: each GPU holds a full model. Why not split
    the model (TP)?'
  back: ''
  choices:
  - key: a
    text: A) Because model fits on one 80GB GPU (140GB / 80GB ≈ split needed)
    correct: false
  - key: b
    text: B) Because model doesn't fit; must use TP
    correct: false
  - key: c
    text: C) Because data parallelism is always better
    correct: false
  - key: d
    text: D) Model fits; data parallelism is simpler and scales perfectly
    correct: true
- id: 263d9ca9-0462-46e8-91dc-0271abedcb80
  type: flip
  front: You have 4x A100 80GB and 100 concurrent requests. Data or tensor parallelism?
  back: 'Data parallelism. Model fits on one GPU (70B = 70-140GB BF16 on 80GB A100 possible with quantization).
    Data parallelism: 4 requests per iteration, 25 iterations to clear queue. TP would halve throughput
    per-GPU but allow serving larger models.'
---

## Intuition
**Data parallelism for inference**: Replicate the model across N GPUs. Each GPU processes a **different request**. Zero synchronization needed (unlike training). Throughput scales linearly with GPUs.

**Example**: Llama 70B on 4x A100s.
- 1 GPU: 50 tok/s, queue time 100ms.
- 4 GPUs (data parallel): 200 tok/s, queue time 25ms. (4x throughput, 4x lower latency.)

## Detail
### Setup
```python
# vLLM handles data parallelism transparently
from vllm import LLM

llm = LLM('meta-llama/Llama-2-70b',
          tensor_parallel_size=1,  # no TP
          pipeline_parallel_size=1,  # no PP
          # Data parallelism is implicit across remaining GPUs
         )

# Each GPU holds full model, processes different request
# GPU 0: request A
# GPU 1: request B
# GPU 2: request C
# GPU 3: request D
```

### Why Data Parallelism Works for Inference
- **Training**: Gradients must synchronize across replicas (AllReduce). Expensive.
- **Inference**: No gradients. Each request is independent. No sync needed (except load balancing).

Result: Perfect linear scaling (no communication overhead).

### Limitations
1. **Model must fit on one GPU**: 70B model needs A100 80GB (not A100 40GB).
2. **Diminishing returns**: If requests are long-tail (diverse lengths), load balancing becomes harder. Some GPUs finish early, idle.
3. **Batching still important**: Within each GPU, continuous batching extracts throughput.

## Common gotchas
- **"Data parallelism vs tensor parallelism?"** Data parallel: each GPU full model, different request. Tensor parallel: split model across GPUs, same request. TP needed when model doesn't fit on one GPU.
- **"Can I data-parallel a 70B on 2x A100 40GB?"** No; model doesn't fit on 40GB. Must use tensor parallelism (split model).

## See also
- [[parallelism-strategies]]
- [[gpu-utilization]]
- [[load-balancing]]
- [[batch-processing]]

## Sources
See frontmatter `sources:`.
