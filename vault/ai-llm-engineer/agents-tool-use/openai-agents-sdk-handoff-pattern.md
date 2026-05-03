---
id: fa658eff-057c-47fe-a552-b38eef0aa599
title: OpenAI Agents SDK (handoff pattern)
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- OpenAI-native
- handoff-pattern
- agent-routing
- stateless
- model-locked
- explicit-delegation
aliases:
- Agent handoff
- OpenAI agent transfers
- Explicit routing protocol
sources:
- url: https://medium.com/@atnoforgenai/10-ai-agent-frameworks-you-should-know-in-2026-langgraph-crewai-autogen-more-2e0be4055556
  label: 10 AI Agent Frameworks You Should Know in 2026
- url: https://softmaxdata.com/blog/definitive-guide-to-agentic-frameworks-in-2026-langgraph-crewai-ag2-openai-and-more/
  label: Definitive Guide to Agentic Frameworks in 2026
- url: https://qubittool.com/blog/ai-agent-framework-comparison-2026
  label: AI Agent Framework Comparison 2026
- url: https://happycapyguide.com/blog/ai-agent-frameworks-compared-2026
  label: AI Agent Frameworks Compared 2026
cards:
- id: c4405fe4-f538-4dfa-ae2c-bc78265a067e
  type: flip
  front: How does OpenAI Agents SDK implement agent-to-agent communication?
  back: 'Via explicit handoff function: Agent A calls `transfer_to_agent(agent_name, messages)`. Full
    conversation history passed to Agent B. Agents are stateless; routing is responsibility of caller.
    No built-in graph or orchestration.'
- id: 82b0dce4-2500-44b9-bf49-e67718b52e38
  type: flip
  front: What is a key difference between OpenAI SDK's handoff and LangGraph's edge routing?
  back: 'OpenAI: Explicit function call (handoff) within agent logic; message-based; stateless. LangGraph:
    Declarative edge routing at graph definition time; state-based; checkpointed. OpenAI is imperative
    and flexible; LangGraph is declarative and reproducible.'
- id: 7be55f63-bb56-4e21-98fc-dc5df541d31d
  type: mcq
  front: 'You have a 3-agent OpenAI SDK team: Researcher → Coder → Reviewer. How do you prevent Reviewer
    from handing back to Researcher (cycle)?'
  back: Handoff is a function call in agent logic. OpenAI SDK provides no built-in cycle detection. You
    must code guards (e.g., 'if previous_agent == "researcher", don't handoff to researcher') or use external
    orchestrator.
  choices:
  - key: a
    text: OpenAI SDK enforces acyclic handoffs automatically
    correct: false
  - key: b
    text: Manually code guards in each agent's handoff logic (e.g., Reviewer never calls transfer_to_agent)
    correct: true
  - key: c
    text: Use a central orchestrator to track handoff history
    correct: false
  - key: d
    text: Impossible with OpenAI SDK design
    correct: false
- id: 892aabd9-3a49-41c6-94dc-c226c66545b7
  type: flip
  front: ''
  back: ''
- id: edacc778-5507-4408-9e2f-d5aba61b791a
  type: flip
  front: Why is OpenAI Agents SDK called 'model-locked'?
  back: Only supports OpenAI models (gpt-4o, gpt-4-turbo). No flexibility for Claude, Gemini, or open-source
    models. If OpenAI pricing changes or outages occur, no fallback. LangGraph, CrewAI are model-agnostic.
---

## Intuition

OpenAI's Agents SDK models agents as **stateless functions that can hand off to other agents**. Each agent is a thin wrapper around gpt-4o or gpt-4-turbo that receives a context (messages), calls tools, and either completes or explicitly hands off to another agent by name. No graph orchestration—just explicit delegation. Model-locked to OpenAI; low learning curve.

## Detail

**Core Concept: Handoff Pattern**
- Agent A processes request, recognizes it needs Agent B's expertise
- Calls `transfer_to_agent(agent_name="agent_b", context=messages)`
- OpenAI SDK routes messages to Agent B; Agent B resumes with full context
- No state machine; no checkpointing—purely message-based

**Agent Definition (minimal):**
```python
agent_a = Agent(model="gpt-4o", tools=[tool_1, tool_2])
agent_b = Agent(model="gpt-4o", tools=[tool_3, tool_4])
# Agent A can call transfer_to_agent("agent_b", context)
```

**Message Threading**: Full conversation history passed with each handoff. Agents inherit prior context but make independent decisions. No shared memory—each agent is stateless w.r.t. prior agents.

**Tool Calling**: Standard OpenAI format (tool_calls in response). Agent calls tools, receives results, reasons, decides to (1) continue, (2) handoff, or (3) return final response.

## Common gotchas / interview framings

- **Model lockdown**: OpenAI models only (gpt-4o, gpt-4-turbo). No flexibility for Claude, Gemini, or custom models.
- **Handoff verbosity**: Context (full message history) sent with each handoff. For long conversations, this inflates token usage and latency.
- **No checkpointing**: No snapshots or rollback. If Agent B fails mid-task, restarting from handoff point requires replaying Agent A's entire work.
- **Implicit routing**: Handoff is a function call—framework doesn't enforce who is allowed to hand off to whom. Risk of cycles (A→B→A).
- **Low observability**: SDK provides minimal tracing. Hard to debug why Agent B didn't do what Agent A requested.
- **Interview scenario**: "Design a 3-agent team (researcher, coder, reviewer) using OpenAI SDK. How do you prevent handoff loops? Where do you cache intermediate results?"

## See also
- [[agent-to-agent-handoff]]
- [[openai-api-tool-use]]
- [[agent-routing-patterns]]
- [[stateless-agent-design]]
- [[model-specific-apis]]
- [[conversation-threading]]

## Sources
See frontmatter `sources:`.
