---
id: 65d44eb4-9850-41d2-8b90-1614e989a023
title: Arize Phoenix for agent evaluation
track: ai-llm-engineer
topic: evaluation-benchmarking
difficulty: 3
tags:
- agent-evaluation
- multi-step-tracing
- decision-making
- observability
- ML-heritage
aliases:
- Phoenix observability
- agent tracing
- agentic AI evaluation
sources:
- url: https://arize.com/docs/phoenix/evaluation/llm-evals
  label: Evaluation — Phoenix
- url: https://www.digitalapplied.com/blog/agent-observability-platforms-langsmith-langfuse-arize-2026
  label: 'Agent Observability: LangSmith, Langfuse, Arize 2026'
- url: https://github.com/Arize-ai/phoenix
  label: 'GitHub — Arize-ai/phoenix: AI Observability & Evaluation'
cards:
- id: 0b145259-fafb-4676-b8e0-29f3f556a042
  type: flip
  front: What makes agent evaluation different from LLM output evaluation?
  back: Agents make multi-step decisions. Evaluation must assess decision quality at each step (correct
    tool? right arguments? sound reasoning?), not just final output. Poor intermediate decisions can coincidentally
    lead to correct answers.
- id: a00a986b-c9ae-4f10-a911-91446641a4ad
  type: mcq
  front: An agent claims to have called a database lookup tool to retrieve information, but the trace
    shows no tool call. Which evaluation challenge does this reveal?
  back: 'Tool hallucination: agents can claim tool calls without actually making them. Traces reveal this
    discrepancy; always verify against traces.'
  choices:
  - key: a
    text: Poor retrieval results
    correct: false
  - key: b
    text: Tool hallucination
    correct: true
  - key: c
    text: Low answer relevance
    correct: false
  - key: d
    text: Slow execution
    correct: false
- id: 0ef082bb-5e40-470e-9605-a9f919b87573
  type: flip
  front: Why is step-level evaluation critical for multi-step agents?
  back: An agent might make poor reasoning decisions at intermediate steps but still reach correct answers
    by accident. Step-level evaluation isolates decision quality; evaluating only final output masks poor
    intermediate reasoning.
- id: e98d363a-b707-43a4-9bc3-4b699add956d
  type: mcq
  front: Phoenix's strength over general observability tools (Langfuse, Datadog) for agent evaluation
    is based on which factor?
  back: Arize comes from ML-observability (model monitoring). Phoenix's eval primitives are deeper than
    general observability tools, purpose-built for agent assessment.
  choices:
  - key: a
    text: Cheaper pricing
    correct: false
  - key: b
    text: ML-observability heritage with deeper eval primitives
    correct: true
  - key: c
    text: Better UI
    correct: false
  - key: d
    text: Faster tracing
    correct: false
---

## Intuition
Agents are multi-step systems: observe state → reason → call tools → observe results → repeat. Evaluating agents requires understanding *why* they made decisions at each step, not just final output quality. Phoenix (from ML-observability company Arize) specializes in capturing complete agent traces and providing tools to assess decision quality.

## Detail
**Core capabilities** (2026):
- **Multi-step tracing**: Capture agent state, reasoning, tool calls, tool results at each step. Full trajectory visibility.
- **LLM-based evaluators**: Pre-built evaluators for common agent patterns:
  - Tool call accuracy: Did the agent call the right tool with correct arguments?
  - Goal completion: Did the agent achieve the stated goal?
  - Hallucination detection: Did the agent claim to use tools it didn't actually call?
- **OpenTelemetry integration**: Unified observability; instrument agents alongside other services.
- **Comparison to competitors**: Phoenix wins on evaluation rigor. Arize's ML-observability heritage provides deeper eval primitives than general observability tools.

**Practical example**: Multi-step retrieval + code execution agent
```
Step 1: Query → Retrieve docs (evaluate: document relevance)
Step 2: Reason → Plan solution (evaluate: reasoning quality)
Step 3: Call tool → Execute code (evaluate: tool call accuracy)
Step 4: Integrate results → Answer (evaluate: answer relevance, groundedness)
```
Phoenix captures each step, enables per-step evaluation.

**Positioning vs. competitors** (2026):
- **vs. Langfuse**: Langfuse is general tracing. Phoenix specializes in agent evaluation; deeper support for multi-step decision assessment.
- **vs. LangSmith**: LangSmith (LangChain) is tightly coupled. Phoenix is framework-agnostic.

## Common gotchas / interview framings
- **Tool hallucination**: Agents can claim to have called tools without actually doing so. Always verify tool calls in traces; don't trust agent self-reporting.
- **Step-level vs. end-level evaluation**: An agent might make poor intermediate decisions but still reach correct answer by luck. Evaluate each decision step, not just final output.
- **Trace completeness**: Incomplete traces hide failures. Ensure every decision point, tool call, and result is logged.
- **Interview framing**: "How would you evaluate a multi-step reasoning agent?" Trace each step (question → plan → tool call → result), evaluate decision quality at each point, not just final answer.

## See also
- [[arize-phoenix]]
- [[agent-evaluation]]
- [[multi-step-tracing]]
- [[decision-assessment]]
- [[ml-observability]]
- [[tool-calling]]

## Sources
See frontmatter `sources:`.
