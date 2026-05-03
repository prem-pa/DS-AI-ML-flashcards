---
id: aa4d59b8-122f-4bb4-93e9-9443a9bd6227
title: Checkpointing and recovery
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 1
tags:
- checkpointing
- recovery
- state-dict
- fault-tolerance
- early-stopping
aliases:
- model saving
- checkpoint recovery
- best model tracking
sources:
- url: https://pytorch.org/tutorials/beginner/saving_loading_models.html
  label: PyTorch Saving and Loading Models
- url: https://pytorch.org/docs/stable/distributed.html#torch.distributed.checkpoint
  label: PyTorch Distributed Checkpoint
- url: https://lightning.ai/docs/pytorch/stable/common/checkpointing.html
  label: PyTorch Lightning Checkpointing
cards:
- id: 2171d2f8-b483-44ce-adbe-a6d9539b9bfa
  type: flip
  front: What should a checkpoint contain beyond model weights?
  back: Optimizer state (momentum buffers, adaptive LR), epoch number, validation metric, random seed.
    These let you resume training exactly where you left off.
- id: 8b7bea89-4a5b-41d0-a458-3c4f7877dc97
  type: mcq
  front: You load a checkpoint and resume training. Validation loss spikes. What likely went wrong?
  back: ''
  choices:
  - key: a
    text: Model weights corrupted
    correct: false
  - key: b
    text: Forgot to load optimizer state or set correct epoch
    correct: true
  - key: c
    text: Learning rate changed
    correct: false
  - key: d
    text: Batch size changed
    correct: false
- id: d35a6ed9-3d72-47cf-ab5d-892bd9d95dcd
  type: flip
  front: In distributed training with 8 GPUs, should each GPU save its own checkpoint?
  back: No. Either one rank saves the full consolidated state_dict, or use PyTorch Distributed Checkpoint
    (async, sharded). Multiple saves waste I/O and storage.
- id: 28013263-765f-4d0a-9244-a1f1557b0f35
  type: mcq
  front: What's the purpose of tracking the 'best' checkpoint during training?
  back: ''
  choices:
  - key: a
    text: Reduces storage
    correct: false
  - key: b
    text: Allows early stopping and recovery at best validation metric
    correct: true
  - key: c
    text: Speeds up training
    correct: false
  - key: d
    text: Prevents overfitting
    correct: false
---

## Intuition
Training large models takes days/weeks. Crashes happen (OOM, hardware failure, preemption). Checkpointing saves model weights + optimizer state periodically. On crash, resume from latest checkpoint instead of restarting from epoch 0.

## Detail
**State Dict Components:** torch.save({model.state_dict(), optimizer.state_dict(), epoch, best_metric}). State dict = all learnable parameters + buffers (BN running stats). Optimizer state = momentum buffers, adaptive LR terms (Adam's m, v).

**Checkpoint Frequency:** Save every N iterations or when validation metric improves (best model). Don't save every iteration (I/O bottleneck); every epoch is typical.

**Distributed Checkpointing:** Naive: all ranks save → N copies. Better: one rank saves, broadcasts to others. Best: PyTorch Distributed Checkpoint (async, sharded saves). For FSDP, save consolidated state dict via `FSDP.full_state_dict_to_cpu()` or sharded via `state_dict()` + DTensor APIs.

**Resume Training:** Load checkpoint: `state_dict = torch.load(path); model.load_state_dict(state_dict['model']); optimizer.load_state_dict(state_dict['optimizer']); start_epoch = state_dict['epoch']`. Note: optimizer state includes LR schedule state.

**Early Stopping:** Monitor validation metric, keep best checkpoint. Stop if validation metric doesn't improve for N epochs. Track (epoch, metric) for each checkpoint.

## Common gotchas / interview framings
- Save model only, not optimizer state → resume at epoch N, but optimizer (momentum, Adam m/v) reset, hurts convergence
- Load checkpoint but forget to set model.train() / model.eval() → wrong behavior (BN uses wrong stats)
- Checkpoint path not writable (permission/disk full) → training crashes silently
- Different model architecture between save/load → load_state_dict() fails
- Distributed: one rank checkpoints → others don't have checkpoint (use distributed checkpointing)

## See also
- [[loss-curves-and-convergence-diagnostics]]
- [[validation-strategy-and-metric-selection]]
- [[distributed-sampling-and-epoch-synchronization]]

## Sources
See frontmatter `sources:`.
