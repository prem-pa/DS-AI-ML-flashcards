---
id: 03e18738-b8c8-4e9a-ba09-d35a836752c5
title: Agent-to-Agent Protocol (A2A)
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- agent-communication
- agent-discovery
- inter-agent-handoff
- decentralized-agents
- swarms
- emerging-standard
aliases:
- A2A protocol
- Inter-agent communication
- Agent-agent handoff
sources:
- url: https://a2a-mcp.org/blog/mcp-full-form
  label: 'MCP Full Form: Model Context Protocol Explained for AI Agents in 2026'
- url: https://blog.modelcontextprotocol.io/posts/2026-mcp-roadmap/
  label: The 2026 MCP Roadmap
- url: https://qubittool.com/blog/ai-agent-framework-comparison-2026
  label: AI Agent Framework Comparison 2026
- url: https://blog.equinix.com/blog/2025/08/06/what-is-the-model-context-protocol-mcp-how-will-it-enable-the-future-of-agentic-ai/
  label: What Is MCP? How Will it Enable Agentic AI?
cards:
- id: 01a20ca5-e2b8-4384-b844-c72b4e2ed1f0
  type: flip
  front: How does A2A extend MCP?
  back: 'MCP: Agents discover and call tools (functions). A2A: Agents discover and invoke other agents
    (as services). Agent B is callable with goals/tasks, returns results. Enables agent-to-agent coordination
    without central orchestrator.'
- id: 4b4e1f5b-672e-49b9-970f-7efc8238c4a5
  type: flip
  front: What is the difference between A2A and explicit handoff (e.g., OpenAI SDK)?
  back: 'Explicit handoff: Agent A code explicitly calls transfer_to_agent("B"). A2A: Agent A invokes
    Agent B via A2A interface (no hardcoding). A2A is declarative, dynamic; explicit is imperative, static.'
- id: 5ebcdec5-39c0-4ed8-831d-b7e399046736
  type: mcq
  front: Agent A invokes Agent B (via A2A). Agent B fails. What should Agent A do?
  back: A2A is a protocol; frameworks implement resilience. Agent A should have retry logic + fallback
    options. If max retries exhausted, escalate (user, parent agent, or return partial result).
  choices:
  - key: a
    text: Crash immediately
    correct: false
  - key: b
    text: Retry Agent B with backoff, then try fallback Agent B', or escalate to user
    correct: true
  - key: c
    text: Continue with incomplete data
    correct: false
  - key: d
    text: A2A handles this automatically
    correct: false
- id: 2ef7b18d-d0fb-42a7-b052-e171c82f2a1b
  type: flip
  front: ''
  back: ''
- id: cb60476b-5e7d-43ef-ba51-914fb44c0c98
  type: flip
  front: Why is A2A still emerging (2026) unlike MCP servers?
  back: 'MCP servers are tools (simple, stateless). A2A agents are complex (state, long-running, coordination).
    Protocol needs consensus on: agent discovery, messaging format, failure semantics, security model.
    Still evolving; no single standard yet.'
---

## Intuition

**Agent-to-Agent Protocol (A2A)** extends [[Model Context Protocol (MCP)]] to enable agents to **discover and communicate with other agents** as first-class entities. Instead of agents only calling tools (functions), they can invoke other agents as services. Enables **agent swarms** and **mesh architectures** where agents are nodes in a network.

## Detail

**A2A extends MCP:**
- MCP servers expose tools (functions). A2A extends to expose agents (callable with goals/tasks, return results)
- Agent A discovers Agent B via A2A registry
- Agent A invokes Agent B with a task: `invoke_agent(agent_id="B", goal="Summarize this document")`
- Agent B executes, returns result to Agent A
- No explicit handoff code needed; agents coordinate autonomously

**Architecture:**
```
Agent A (Researcher) --discovers--> A2A Registry
   |                                     |
   |--discovers Agent B (Writer)---------|  
   |--discovers Agent C (Reviewer)-------|
   |--invokes B with research data------>
Agent B executes, returns summary
   |--passes summary to C for review----->
Agent C completes, returns feedback
   |--synthesizes final report---------->
```

**2026 Status:**
- Emerging standard (part of MCP 2026 roadmap)
- Not yet widely adopted (unlike MCP servers which are at 78% enterprise adoption)
- Early implementations in LangGraph, AG2
- Use cases: multi-agent research teams, decentralized task networks, swarms

**A2A Benefits:**
- **Decoupling**: Agents don't know each other's implementation; call via A2A interface
- **Dynamic composition**: New agents join network; existing agents auto-discover
- **Scalability**: 100s of agents without central orchestrator
- **Fault tolerance**: If Agent B fails, caller can retry or escalate

## Common gotchas / interview framings

- **Protocol immaturity**: A2A is emerging; spec still evolving. Implementations vary (no standard yet).
- **Latency**: Agent-to-agent communication adds overhead (discovery, serialization, network). Each handoff ~500ms.
- **Ordering & synchronization**: If Agent A invokes B and C in parallel, order of results not guaranteed. Caller must handle async coordination.
- **Circular dependencies**: Agent A → Agent B → Agent A (cycle). Frameworks must detect and prevent.
- **Security**: Agent A invokes Agent B; what permissions does B have? What data does A trust B with? Needs explicit trust model.
- **Interview scenario**: "Design 10-agent team using A2A. How do you prevent cycles? ensure fault tolerance? set SLOs for inter-agent latency?"

## See also
- [[mcp-agent-extensions]]
- [[agent-swarms]]
- [[decentralized-agents]]
- [[agent-service-discovery]]
- [[agent-agent-messaging]]
- [[protocol-specification]]

## Sources
See frontmatter `sources:`.
