---
id: af380181-efc5-42ef-907a-a1db6f8d45c9
title: Pipeline parallelism (PP)
track: ai-llm-engineer
topic: serving-infrastructure-inference-optimization
difficulty: 5
tags:
- pipeline-parallelism
- layer-sharding
- bubbles
- micro-batching
- async-communication
aliases:
- PP
- GPipe
- PipelineParallel
- Layer-parallelism
sources:
- url: https://docs.vllm.ai/en/stable/serving/parallelism_scaling/
  label: 'vLLM: Pipeline Parallelism Guide'
- url: https://bentoml.com/llm/inference-optimization/data-tensor-pipeline-expert-hybrid-parallelism
  label: 'BentoML: Pipeline Parallel (2026)'
cards:
- id: ea06a1a7-1137-4810-9819-7410bfa07499
  type: flip
  front: What is a pipeline bubble, and how does it affect pipeline parallelism efficiency?
  back: A pipeline bubble is idle GPU time when a layer is waiting for input activations from the previous
    layer. With N GPUs and small batch size, startup and shutdown require N-1 steps of bubbles. Efficiency
    = 1 / (1 + 2(N-1)/batch_size). Large batches hide bubbles; small batches suffer.
- id: 566cd7d7-cc8c-4994-a893-b0913b9693f3
  type: mcq
  front: Llama 70B with PP-4 across 4 GPUs, batch_size=1. Efficiency?
  back: ''
  choices:
  - key: a
    text: A) 100%, all GPUs busy
    correct: false
  - key: b
    text: B) ~25%, large startup/shutdown bubbles relative to batch size
    correct: true
  - key: c
    text: C) 50%
    correct: false
  - key: d
    text: D) Depends on model
    correct: false
- id: cb321151-3fd9-408b-aa32-1921574249c8
  type: flip
  front: "What does this vLLM config do?\n```python\nllm = LLM('meta-llama/Llama-2-70b',\n          pipeline_parallel_size=4,\n\
    \          tensor_parallel_size=1)\n```"
  back: Splits 70B into 4 pipeline stages (20 layers each) across 4 GPUs. No tensor parallelism within
    stages (each stage computes full layers). Enables micro-batching and asynchronous pipelining. Bubbles
    present but acceptable for batch inference.
---

## Intuition
**Pipeline parallelism (PP)**: Divide model **layers** sequentially across GPUs. Each GPU processes a **chunk of layers**, passing activations to the next GPU. Enables **asynchronous pipelining**.

**Tradeoff**: More flexible than TP, but pipeline **bubbles** (idle GPUs waiting for data) reduce efficiency.

## Detail
### PP Setup
```
Llama-2 70B: 80 layers total

PP-4 across 4 GPUs:
  GPU 0: layers 1-20   → output activation → GPU 1
  GPU 1: layers 21-40  → output activation → GPU 2
  GPU 2: layers 41-60  → output activation → GPU 3
  GPU 3: layers 61-80  → output (logits)
```

### Pipeline Bubbles
Without pipelining (sequential):
```
GPU 0 computes, sends to GPU 1
GPU 1 waits for GPU 0 → computes → sends to GPU 2
GPU 2 waits → computes → sends to GPU 3
GPU 3 waits → computes

Utilization: Each GPU busy 25% of time (3 GPUs idle at each step)
Efficiency: 25%
```

With micro-batching (pipeline):
```
Time step 1: GPU0 processes batch1
Time step 2: GPU0 processes batch2; GPU1 processes batch1's output
Time step 3: GPU0 processes batch3; GPU1 processes batch2; GPU2 processes batch1
            (All GPUs busy!)

Utilization: 100% (with enough micro-batches)
But: pipeline bubbles at start/end; efficiency ≈ 70-80%
```

### Bubble Cost
With N GPUs and batch_size = B:
- Steady-state throughput: B tokens per N-step cycle.
- Startup: N-1 steps of bubbles (GPUs waiting to fill pipeline).
- Shutdown: N-1 steps of bubbles (last GPUs idle).

Total bubbles: 2(N-1) out of B×N steps.
Efficiency: B×N / (B×N + 2(N-1)) ≈ B×N / (B×N) = 100% for large B.
For small B, bubbles dominate; e.g., B=1 → efficiency = 1 / (4+6) = 14%.

### PP vs TP
| Parallelism | Memory per GPU | Latency | Communication | Bubbles |
|-------------|--------|---------|--------|----------|
| Single GPU | Full (140GB) | 50ms | None | None |
| TP-4 | 35GB | 70ms | AllReduce/layer | None |
| PP-4 | 35GB | 50ms (ideal) | Sequential | Yes (10-20%) |
| TP-2 + PP-2 | 70GB | 80ms | Both | Yes |

**In inference, PP is less common than TP** because:
1. Bubbles waste GPU capacity (inference already memory-bound).
2. TP's AllReduce overhead is lower for small batches.
3. Combined TP+PP (2D parallelism) is overkill for most cases.

## Common gotchas
- **"Is PP better than TP?"** TP is dominant in 2026 for inference. PP shines in training (where bubbles are less critical). For inference, TP > PP.
- **"Can I use PP-8 to avoid large single-GPU memory?"** Technically yes, but bubbles kill throughput. TP is better.

## See also
- [[model-parallelism]]
- [[pipeline-bubbles]]
- [[micro-batching]]
- [[activation-memory]]

## Sources
See frontmatter `sources:`.
