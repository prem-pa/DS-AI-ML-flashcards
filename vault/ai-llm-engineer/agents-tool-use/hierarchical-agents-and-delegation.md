---
id: a01a4563-fba9-4990-a746-5fd953cdce5f
title: Hierarchical agents and delegation
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 5
tags:
- agent-hierarchy
- delegation-patterns
- multi-level-abstraction
- management-agents
- subgoal-decomposition
- scalability
aliases:
- Manager-worker pattern
- Hierarchical decomposition
- Multi-level agents
sources:
- url: https://servicesground.com/blog/agentic-reasoning-patterns/
  label: Agentic Reasoning Patterns 2026
- url: https://towardsdatascience.com/recap-of-all-types-of-llm-agents/
  label: Recap of All Types of LLM Agents
- url: https://softmaxdata.com/blog/definitive-guide-to-agentic-frameworks-in-2026-langgraph-crewai-ag2-openai-and-more/
  label: Definitive Guide to Agentic Frameworks 2026
- url: https://qubittool.com/blog/ai-agent-framework-comparison-2026
  label: AI Agent Framework Comparison 2026
cards:
- id: f4efd99c-08a2-492e-916a-d41be6bbdcfb
  type: flip
  front: What is the manager-worker pattern in hierarchical agents?
  back: Manager receives goal, decomposes into subtasks, assigns each to a specialized worker agent. Manager
    monitors progress, aggregates results, escalates blockers. Workers execute independently but report
    back. Scales to large teams.
- id: c1083f09-7eca-47d6-bd71-1e034b39e3f4
  type: flip
  front: Compare sequential vs parallel delegation in hierarchical agents.
  back: 'Sequential: Manager assigns Task 1 → Worker A completes → Task 2 → Worker B. Each task depends
    on prior. Latency: sum of all task times. Parallel: Manager assigns Tasks 1, 2, 3 simultaneously to
    Workers A, B, C. Latency: max task time. Trade-off: complexity (handling partial failures).'
- id: 7e406a91-4297-45aa-b47b-b67f374d0a19
  type: mcq
  front: You have 15 subtasks and 5 workers. How do you assign tasks to minimize latency while avoiding
    Manager bottleneck?
  back: Parallel assignment with batching (3 per worker) minimizes Manager reasoning overhead. If subtasks
    have dependencies, use 2-level hierarchy to avoid single Manager bottleneck. Larger teams benefit
    from tree-structured management.
  choices:
  - key: a
    text: Manager assigns all 15 sequentially (1 per worker, then next batch)
    correct: false
  - key: b
    text: Manager assigns all 15 in parallel (3 per worker), workers report back
    correct: false
  - key: c
    text: 'Use 2-level hierarchy: top manager splits tasks among sub-managers, each manages 2-3 workers'
    correct: true
  - key: d
    text: Assign tasks randomly
    correct: false
- id: 06a6c8e0-0aae-4d49-bb16-333b5e732820
  type: flip
  front: ''
  back: ''
- id: 44c49473-3d87-4b12-ac57-274ee9b87999
  type: flip
  front: What is the main cost of hierarchical agents vs flat teams?
  back: 'Overhead: Manager reasoning (decomposition, monitoring, aggregation), inter-agent messages (context
    passing), coordination (enforce ordering). Benefit: scalability (teams > 5 agents become unwieldy
    without hierarchy). Trade-off: small tasks (< 3 agents) don''t need hierarchy.'
---

## Intuition

**Hierarchical agents** decompose complex tasks into subtasks and assign them to specialized sub-agents. A **manager agent** receives the goal, breaks it into subtasks, assigns each to a worker agent, monitors progress, and escalates blockers. This pattern scales to large teams and complex workflows.

## Detail

**Manager-Worker Pattern:**
```
Manager: "Plan a trip to Japan for 10 days, budget $5K."
Manager breaks down:
  - Worker A: "Book flights (search, compare, reserve)"
  - Worker B: "Find hotels (search by city, check reviews, reserve)"
  - Worker C: "Create itinerary (research attractions, plan days)"
Manager: "Worker A: done (flights $800). Worker B: need reviews API. Worker C: in progress."
Manager escalates: "Worker B, we need better hotel data. Try this API endpoint."
Manager synthesizes: "Trip planned! $4.2K total."
```

**Implementation Patterns:**
1. **Sequential Delegation**: Manager assigns tasks in order (Task 1, then Task 2 based on Task 1 results)
2. **Parallel Delegation**: Manager assigns multiple tasks simultaneously (fetch flights + hotels in parallel)
3. **Adaptive Delegation**: Manager monitors progress, adjusts task assignments if workers fail

**Challenges & Solutions:**
- **Context passing**: Each subtask needs full context + prior results. Manager summarizes or passes relevant excerpts.
- **Failure handling**: If Worker B fails, does Manager retry, reassign, or escalate? Design policy upfront.
- **Prompt tuning**: Manager and worker prompts must be tuned separately. Manager focuses on decomposition; worker on execution.
- **Cost tracking**: Hierarchical systems incur overhead (manager reasoning, inter-agent messages). Track token usage per level.

## Common gotchas / interview framings

- **Over-delegation**: Breaking into too many subtasks adds latency (each worker cycles multiple times). Sweet spot: 3-5 workers per manager.
- **Context loss**: Manager summarizes for workers; workers may miss nuance. Risk: Worker misunderstands subtask, fails silently.
- **Manager bottleneck**: If manager is LLM-based, its reasoning time scales with team size. Large teams may need multiple managers (tree of managers).
- **Coordination overhead**: Workers may need to coordinate (Worker A's output feeds Worker B). Manager must enforce ordering or buffering.
- **Interview scenario**: "Design a 10-person agent team for a complex research project. How do you structure the hierarchy? Where do you place checkpoints? How do you handle cascading failures?"

## See also
- [[agent-to-agent-delegation]]
- [[manager-agent-pattern]]
- [[task-decomposition]]
- [[subgoal-hierarchies]]
- [[agent-communication-protocols]]
- [[scalable-multi-agent-systems]]

## Sources
See frontmatter `sources:`.
