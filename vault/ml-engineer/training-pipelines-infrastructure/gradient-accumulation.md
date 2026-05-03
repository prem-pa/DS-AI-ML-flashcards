---
id: 1777a938-e910-411b-8a09-0ebce2ff8ad9
title: Gradient accumulation
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- gradient-accumulation
- effective-batch-size
- memory-efficient
- micro-batch
- gradient-scaling
aliases:
- virtual batch size
- gradient summing
- micro-batching
sources:
- url: https://pytorch.org/docs/stable/notes/amp_examples.html
  label: PyTorch AMP with Gradient Accumulation
- url: https://huggingface.co/docs/transformers/training
  label: Hugging Face Trainer (gradient_accumulation_steps)
- url: https://lightning.ai/docs/pytorch/stable/common/trainer.html
  label: PyTorch Lightning Trainer (accumulate_grad_batches)
cards:
- id: 45932673-93e8-464d-9319-46f5a43f34cf
  type: flip
  front: You want effective batch size = 1024 but GPU only fits 256. How do you use gradient accumulation?
  back: Set batch_size=256, accumulation_steps=4. Train 4 micro-batches, accumulate gradients, update
    once. Effective = 256 × 4 = 1024.
- id: fd12d6c0-b33e-450b-a6be-ce8cec086b39
  type: mcq
  front: In gradient accumulation with 4 steps, how should you scale the loss before backward?
  back: ''
  choices:
  - key: a
    text: Don't scale, sum losses
    correct: false
  - key: b
    text: Divide by 4
    correct: true
  - key: c
    text: Multiply by 4
    correct: false
  - key: d
    text: Divide by 2
    correct: false
- id: db2877bd-cd3e-470c-8f7f-4af55ea6e1a3
  type: flip
  front: Why does batch norm in a gradient accumulation loop cause issues?
  back: BN computes mean/var per micro-batch. Over 4 micro-batches, you're computing BN stats on 4 different
    256-sample groups. Use SyncBatchNorm (accumulate stats across all micro-batches) or apply BN per full
    accumulated batch.
- id: 084ecdd9-2c9a-4fd3-996b-7daf072d4bd4
  type: mcq
  front: Gradient accumulation allows you to simulate larger batches. Does this also mean you should adjust
    your learning rate?
  back: ''
  choices:
  - key: a
    text: No, learning rate is independent of batch size
    correct: false
  - key: b
    text: 'Yes, use linear scaling rule: LR *= sqrt(effective_batch_size/reference_size)'
    correct: true
  - key: c
    text: Yes, always divide LR by accumulation_steps
    correct: false
  - key: d
    text: No clear answer; depends on optimizer
    correct: false
---

## Intuition
GPU memory limits batch size. With gradient accumulation, train on smaller batches but accumulate gradients over N micro-batches before optimizer step. Effective batch size = batch_size * accumulation_steps, matching large-batch training without running out of memory.

## Detail
**Mechanics:** Zero gradients → loop 4 times {forward(micro_batch), backward()}→ loss += loss/4 (normalize) → optimizer.step()→zero_grad(). Effective batch size = micro_batch_size * 4.

**Normalization Critical:** Must scale loss by 1/accumulation_steps to maintain correct gradient magnitudes. Without scaling, accumulated gradients 4× too large.

**Learning Rate Unaffected:** Effective batch size increases but learning rate stays same (or may scale with batch size per literature). Warmup schedule should account for effective batch size.

**Momentum Interaction:** With momentum (SGD, Adam), longer accumulation intervals → momentum dampens less frequently. Some empirical studies suggest slight LR increase helps (linear scaling rule).

**Mixed Precision Impact:** Loss scaling in AMP interacts with gradient accumulation. Gradient scaling should be scaled by 1/accumulation_steps to avoid numerical issues.

## Common gotchas / interview framings
- Forgot to normalize loss (divide by accumulation_steps) → gradients 4× too large, training unstable
- Changed accumulation_steps mid-training without adjusting LR → convergence changes
- Batch norm in accumulated loop → BN statistics computed per micro-batch, incorrect (use SyncBN or single-batch BN)
- Validation every micro-batch instead of every full accumulation → wastes compute, corrupts metrics

## See also
- [[mixed-precision-training-fp16-bfloat16]]
- [[data-parallelism-ddp]]
- [[pipeline-parallelism-and-micro-batching]]

## Sources
See frontmatter `sources:`.
