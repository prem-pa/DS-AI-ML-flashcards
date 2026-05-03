---
id: 53495327-bcc8-4f30-acb7-137b625bbfac
title: Tensor parallelism (TP)
track: ai-llm-engineer
topic: serving-infrastructure-inference-optimization
difficulty: 5
tags:
- tensor-parallelism
- model-sharding
- layer-splitting
- allreduce
- latency
aliases:
- TP
- Model-sharding
- Layer-wise-parallelism
- Row-col-parallel
sources:
- url: https://docs.vllm.ai/en/stable/serving/parallelism_scaling/
  label: 'vLLM: Tensor Parallelism Guide'
- url: https://pytorch.org/tutorials/intermediate/TP_tutorial.html
  label: 'PyTorch: Tensor Parallel Tutorial'
- url: https://engineering.fb.com/2025/10/17/ai-research/scaling-llm-inference-innovations-tensor-parallelism-context-parallelism-expert-parallelism/
  label: 'Meta: Scaling LLM Inference (2025)'
cards:
- id: 7248866f-1346-40df-9603-a96aa35bf425
  type: flip
  front: 'Explain tensor parallelism: how does splitting a linear layer across GPUs work?'
  back: 'Split weight matrix row-wise across GPUs. Each GPU computes a partial matmul: `input @ weight_i^T`.
    Concatenate results via AllReduce, reconstructing full output. Cost: AllReduce communication per layer
    (~10-20ms overhead).'
- id: 933acdd0-f5c1-4e7a-975f-57a73e7b48b1
  type: mcq
  front: Llama 70B on 2x A100 80GB. Model needs 140GB (FP16). Use TP or data parallel?
  back: ''
  choices:
  - key: a
    text: A) Data parallel (replicate full model on each GPU)
    correct: false
  - key: b
    text: B) TP-2 (split model across 2 GPUs)
    correct: true
  - key: c
    text: C) Either works equally
    correct: false
  - key: d
    text: D) Use smaller model
    correct: false
- id: 7b661b97-66de-4b9f-bb84-ea1a138a6037
  type: flip
  front: 'TP-4 on 4x H100s adds 20ms AllReduce latency (vs 50ms single-GPU token time). Total: 70ms per
    token. Accept or go back to single GPU (50ms, but OOM)?'
  back: Accept TP-4. You must use TP to fit the model (single GPU OOMs). 70ms is acceptable for many apps
    (batch inference, RAG, non-interactive). If latency SLA is <60ms, reconsider smaller model or optimizations
    (quantization).
---

## Intuition
**Tensor parallelism (TP)**: Split model **layers** across GPUs. Each GPU computes a **partial matmul**. Aggregate results via AllReduce.

**Use case**: Model doesn't fit on one GPU. Trade latency (+AllReduce cost) for memory (split model).

## Detail
### TP Mechanics
Consider a linear layer: `output = input @ weight^T`.

**Single GPU**: `output = [batch, seq, hidden] @ [hidden, hidden] → [batch, seq, hidden]`

**TP across 4 GPUs** (split weight row-wise):
```
GPU 0: weight_0 [hidden/4, hidden]
GPU 1: weight_1 [hidden/4, hidden]
GPU 2: weight_2 [hidden/4, hidden]
GPU 3: weight_3 [hidden/4, hidden]

Forward pass:
  output_0 = input @ weight_0^T  [batch, seq, hidden/4]  (GPU 0)
  output_1 = input @ weight_1^T  [batch, seq, hidden/4]  (GPU 1)
  output_2 = input @ weight_2^T  [batch, seq, hidden/4]  (GPU 2)
  output_3 = input @ weight_3^T  [batch, seq, hidden/4]  (GPU 3)
  
  AllReduce (Concatenate):
    output = [output_0; output_1; output_2; output_3]  [batch, seq, hidden]
```

Cost: 1 AllReduce per layer = N-1 tensor exchanges (for N GPUs).

### Latency Impact
```
Single GPU latency: 100ms per token
TP-4 latency: 100ms + 4 × 10ms (AllReduce) ≈ 140ms per token
                              ↑ communication overhead
```

TP introduces ~20-40% latency overhead but enables 4x memory reduction.

### When to Use TP
- **Model doesn't fit on one GPU**: Mandatory.
- **Memory available but latency SLA loose**: TP is acceptable.
- **Low-bandwidth cluster**: Avoid TP (AllReduce expensive on slow interconnect).

### TP in vLLM
```python
llm = LLM('meta-llama/Llama-2-70b',
          tensor_parallel_size=4)  # split across 4 GPUs
```

Memory per GPU: 140GB / 4 ≈ 35GB per A100 80GB (fits).
Latency per token: ~40ms (vs 20ms on single H100, but needed for fit).

## Common gotchas
- **"TP-8 vs TP-4?"** TP-8 halves memory but doubles AllReduce cost. Sweet spot usually TP-2 or TP-4 (communication scales superlinearly).
- **"Can I use TP with quantization?"** Yes; quantized weights split the same way. AllReduce still needed.

## See also
- [[model-parallelism]]
- [[allreduce]]
- [[communication-cost]]
- [[latency-optimization]]

## Sources
See frontmatter `sources:`.
