---
id: e5b4b37f-57f3-4acd-9299-0091b49b93fd
title: Model Context Protocol (MCP)
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- standardization
- tool-discovery
- MCP-servers
- interoperability
- ecosystem-adoption
- Linux-Foundation
aliases:
- MCP servers
- Tool protocol
- Context protocol
sources:
- url: https://blog.modelcontextprotocol.io/posts/2026-mcp-roadmap/
  label: The 2026 MCP Roadmap
- url: https://blog.equinix.com/blog/2025/08/06/what-is-the-model-context-protocol-mcp-how-will-it-enable-the-future-of-agentic-ai/
  label: What Is MCP? How Will it Enable Agentic AI?
- url: https://sureprompts.com/blog/model-context-protocol-mcp-complete-guide-2026
  label: 'Model Context Protocol (MCP): The Complete 2026 Guide'
- url: https://a2a-mcp.org/blog/mcp-full-form
  label: 'MCP Full Form: Model Context Protocol Explained for AI Agents in 2026'
cards:
- id: 7ec58b28-7c4f-48c3-bffa-697be99bdbb5
  type: flip
  front: What problem does MCP solve in agentic AI?
  back: 'Pre-MCP: Tool integrations hardcoded per framework (LangGraph, CrewAI, etc.). Scaling to 100s
    of tools is manual, duplicated, brittle. MCP: Standardized protocol for servers to advertise tools.
    Agents auto-discover. 92% of frameworks support it by 2026.'
- id: 893e102f-3e87-4df1-9f04-5ceec05930b1
  type: flip
  front: How does MCP tool discovery work?
  back: Agent connects to MCP server (via stdio, HTTP, WebSocket). Server advertises available tools (JSON-RPC).
    Agent reads tool definitions, adds to its tool registry. Can now call any tool from that server without
    hardcoding.
- id: c7c4eea4-d111-4aa6-be8a-9bcb037c5acb
  type: mcq
  front: You have 50 MCP servers. Discovering all tools at agent startup takes 5s. How do you optimize?
  back: Cache discovered tools locally (file or Redis). On agent startup, load cache (< 100ms). Periodically
    refresh (async background job). If tool not found in cache, discover on-demand. Balances freshness
    vs speed.
  choices:
  - key: a
    text: Accept 5s startup latency
    correct: false
  - key: b
    text: Cache tool definitions locally; only rediscover on demand or periodic refresh
    correct: true
  - key: c
    text: Remove servers to reduce latency
    correct: false
  - key: d
    text: Use only 5 most popular servers
    correct: false
- id: 61ae32de-29ad-4dac-a00b-8aedc482f011
  type: flip
  front: ''
  back: ''
- id: 4fe437c3-9094-4946-964e-37d13a401fef
  type: flip
  front: Why is MCP important for multi-agent ecosystems?
  back: 'Enables agent-to-agent discovery: Agent A exposes tools via MCP server → Agent B discovers and
    uses Agent A''s tools. 2026 roadmap includes agent-to-agent communication protocols. MCP is foundation
    for composable agent networks.'
---

## Intuition

**Model Context Protocol (MCP)** is a standardized protocol (introduced by Anthropic in Nov 2024, backed by Linux Foundation) for agents to **discover and use tools**. Instead of hardcoding tool definitions (JSON schema + function), agents connect to MCP servers that advertise available tools. Enables dynamic tool discovery, interoperability, and ecosystem scale.

## Detail

**Problem MCP Solves:**
Pre-MCP: Each agent framework hardcoded tool integrations (LangGraph has Tool Registry, CrewAI has Tool Catalog, etc.). Tool definitions duplicated across frameworks. Scaling to 100s of tools per agent is manual and brittle.

**MCP Architecture:**
- **MCP Server**: Exposes tools (resources, functions) via standardized API
- **MCP Client**: Agent framework that connects to servers
- **Transport**: JSON-RPC over stdio, HTTP, or WebSocket

**Example MCP Server (GitHub):**
Server exposes: `search_issues(repo, query)`, `create_pr(repo, title, body)`, etc.
Agent connects, auto-discovers tools, can call without hardcoding.

**2026 Adoption Stats:**
- 78% of enterprise AI teams report MCP-backed agents in production (April 2026)
- 9,400+ public MCP servers registered (up from 1,200 in Q1 2025)
- 92% of new agent frameworks ship with MCP support by default
- OpenAI, Microsoft, Google all added MCP support to their platforms

**MCP vs Custom Tools:**
| Aspect | MCP | Custom |
|--------|-----|--------|
| Discovery | Auto (server advertises) | Manual |
| Interop | Works across frameworks | Framework-specific |
| Scaling | 100s of tools easy | Manual per-framework |
| Maintenance | Server owner maintains | Agent owner maintains |

## Common gotchas / interview framings

- **Server reliability**: MCP server outage → tools unavailable. Need fallback or caching.
- **Network latency**: Discovering tools from MCP server adds latency (~100ms per discovery). Cache aggressively.
- **Permission model**: MCP servers may require auth (GitHub token, API key). Agent must handle securely. Risk: leaking credentials.
- **Protocol versioning**: MCP evolving (2026 roadmap includes transport scalability, agent-to-agent communication). Compatibility testing needed.
- **Interview scenario**: "Design agent with 50 MCP servers. How do you discover tools at startup? cache? Handle server failures? Manage auth tokens?"

## See also
- [[mcp-server-registry]]
- [[tool-auto-discovery]]
- [[mcp-vs-custom-tools]]
- [[mcp-adoption-by-frameworks]]
- [[mcp-transport-layers]]

## Sources
See frontmatter `sources:`.
