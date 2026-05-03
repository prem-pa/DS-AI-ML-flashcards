---
id: 4156e31c-348a-4f70-a10e-bdbc24135d77
title: Agent evaluation and reliability
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 5
tags:
- benchmarking
- metrics
- cost-tracking
- reliability-engineering
- production-readiness
- user-satisfaction
aliases:
- Agent metrics
- Benchmarks
- Reliability SLOs
sources:
- url: https://www.swebench.com/
  label: SWE-bench Leaderboards
- url: https://rapidclaw.dev/blog/ai-agent-benchmarks-2026
  label: AI Agent Benchmarks 2026
- url: https://galileo.ai/blog/agent-evaluation-framework-metrics-rubrics-benchmarks/
  label: Agent Evaluation Framework 2026
- url: https://arxiv.org/html/2601.01743v1
  label: 'AI Agent Systems: Architectures and Evaluation'
cards:
- id: 7451f5c9-f0e7-485a-ad1e-742ef7169dad
  type: flip
  front: What are the top 3 metrics for evaluating an agent?
  back: '1. Success rate (% tasks completed correctly). 2. Cost per task (tokens, API calls). 3. Latency
    (wall-clock time). Trade-offs: higher success often costs more. Design Pareto frontier (cost vs quality)
    and choose based on use case.'
- id: b0c72ae1-8290-4299-875a-5f81b3fee344
  type: flip
  front: Name 3 major 2026 agent benchmarks and their focus areas.
  back: 'SWE-bench: software engineering (GitHub issues). GAIA: multi-step reasoning with tool use. WebArena:
    web automation (forms, e-commerce). Each measures different capability. Never collapse into one ranking.'
- id: b3eb4224-5673-42e3-b8b5-ca2e3d04d933
  type: mcq
  front: 'Agent A: 90% SWE-bench, 1000 tokens/issue. Agent B: 80% SWE-bench, 100 tokens/issue. Which to
    ship?'
  back: No universal answer. If cost is constraint (millions of issues/day), Agent B wins. If quality
    is critical (mission-critical systems), Agent A wins. Design Pareto frontier and choose based on business
    requirements. Benchmark results inform but don't dictate.
  choices:
  - key: a
    text: Agent A (higher success)
    correct: false
  - key: b
    text: Agent B (lower cost)
    correct: false
  - key: c
    text: 'Depends on use case: cost vs quality trade-off'
    correct: true
  - key: d
    text: Benchmark results are unreliable; flip a coin
    correct: false
- id: 0b2143a2-789d-439f-a47e-f953674bd6b4
  type: flip
  front: ''
  back: ''
- id: 149e69e1-e029-4b42-ae80-62e6705ed8d7
  type: flip
  front: Why is the 2026 benchmark gaming crisis important for agent evaluation?
  back: 'UC Berkeley showed all 8 major benchmarks can be reward-hacked to ~100%. Implies benchmark results
    may not reflect real-world performance. Solution: use multiple benchmarks, continuous live evaluation,
    holdout test sets, human evaluation with real users.'
---

## Intuition

Like any system, agents must be evaluated for **reliability** (do they work?), **cost** (are they efficient?), and **quality** (how good are results?). No single metric captures all dimensions. 2026 industry consensus: use multiple specialized benchmarks (SWE-bench for coding, GAIA for reasoning, WebArena for web autonomy) but never collapse them into one ranking.

## Detail

**Key Metrics:**

1. **Success Rate**: % of tasks completed correctly. Example: SWE-bench = "% of GitHub issues resolved"
2. **Cost per Task**: Tokens, API calls, time. Example: Claude Opus 4.7 vs Sonnet 4.5 on SWE-bench—Opus higher cost but higher success rate
3. **Latency**: Wall-clock time to complete. SLO: 95th percentile < 30s
4. **Reliability**: Consistency across runs. Same query should give similar results (variance < 10%)

**Major 2026 Benchmarks:**
- **SWE-bench Verified**: 500 real GitHub issues. Tests software engineering competence. Claude Opus 4.7 leads at 87.6%
- **GAIA (Princeton HAL)**: 466 open-ended questions requiring multi-step reasoning, tool use. Claude Sonnet 4.5 leads at 74.6%
- **WebArena**: 812 long-horizon web automation tasks (form filling, e-commerce, etc.). Claude Mythos Preview leads at 68.7%
- **TAU-bench, AgentBench, OSWorld**: Measure different things (code agents, general reasoning, operating systems)

**Gotcha: Benchmark Gaming (2026 Crisis)**
UC Berkeley (April 2026) showed all 8 major benchmarks can be reward-hacked to ~100%. Implies benchmark results may not reflect real-world performance. Solutions: continuous live evaluation, holdout test sets, human evaluation.

**Cost Ignored**: Leading benchmarks ignore cost. Agent A: 90% success, 500 tokens/task. Agent B: 85% success, 50 tokens/task. Benchmarks favor A, but B is better for most use cases.

## Common gotchas / interview framings

- **Single-metric fallacy**: Optimizing for success rate alone ignores cost and latency. Design Pareto frontiers (cost vs quality tradeoff).
- **Benchmark overfitting**: Agents tuned for SWE-bench don't generalize to GAIA. Multi-benchmark evaluation is essential.
- **User satisfaction vs benchmarks**: An agent that scores 70% on SWE-bench but is cheap and fast may be better than 85% expensive agent. A/B test with real users.
- **Reproducibility**: Agents have stochasticity (sampling, tool behavior). Run multiple seeds, report confidence intervals.
- **Interview scenario**: "Design evaluation for a production agent. How do you measure success? cost? latency? reliability? What SLOs do you set? How do you catch regressions?"

## See also
- [[swe-bench]]
- [[gaia-benchmark]]
- [[webarena]]
- [[agent-cost-analysis]]
- [[latency-slos]]
- [[success-rate-measurement]]

## Sources
See frontmatter `sources:`.
