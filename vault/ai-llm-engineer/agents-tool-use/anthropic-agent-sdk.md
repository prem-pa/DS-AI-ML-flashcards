---
id: 6026c251-a5cb-45b1-9f40-38f24ae22bd9
title: Anthropic Agent SDK
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- Claude-native
- MCP-integration
- safety-first
- extended-thinking
- lightweight-orchestration
- reliability-focused
aliases:
- Claude Agent SDK
- Anthropic agents
- MCP tooling
sources:
- url: https://blog.modelcontextprotocol.io/posts/2026-mcp-roadmap/
  label: The 2026 MCP Roadmap
- url: https://qubittool.com/blog/ai-agent-framework-comparison-2026
  label: AI Agent Framework Comparison 2026
- url: https://happycapyguide.com/blog/ai-agent-frameworks-compared-2026
  label: AI Agent Frameworks Compared 2026
- url: https://platform.claude.com/docs/en/agents-and-tools/tool-use/computer-use-tool
  label: Claude Computer Use Tool Documentation
cards:
- id: 25df0c6f-4b2e-436c-b5f4-26eeafefdb03
  type: flip
  front: What is the Anthropic Agent SDK's core loop?
  back: Send messages + tools to Claude. Claude responds with tool_calls (JSON). Execute tools, append
    results as assistant messages, repeat until stop_reason='end_turn'. Simple, reliable, no orchestration
    overhead. Integrated with MCP for auto-discovered tools.
- id: 7546ca59-7d03-4ef6-a5a3-eb6e36354575
  type: flip
  front: How does Anthropic Agent SDK's Extended Thinking improve agent reliability?
  back: 'Claude allocates up to 10K tokens for internal reasoning before generating response. Enables
    better planning, error recovery, multi-step problem decomposition. Cost: 2-3x token usage. Tradeoff:
    better quality vs. faster execution.'
- id: 03c5fafb-0244-4965-8c0b-4cb8682f4f68
  type: mcq
  front: You're designing a code-review agent using Anthropic SDK. An agent loop fails mid-execution.
    What does the SDK provide to recover?
  back: Anthropic Agent SDK is lightweight—no native checkpointing. You must store messages and tool results
    yourself (DB, file, memory). LangGraph handles this automatically.
  choices:
  - key: a
    text: Built-in checkpointing like LangGraph
    correct: false
  - key: b
    text: Nothing; you must implement own message log/persistence
    correct: true
  - key: c
    text: Automatic retry with exponential backoff
    correct: false
  - key: d
    text: Rollback to last successful tool call
    correct: false
- id: 9babf1c5-6e71-4544-a626-fc3658450f45
  type: flip
  front: ''
  back: ''
- id: 23ca5984-012f-4809-955e-8574e9060d8e
  type: flip
  front: What is an advantage of Anthropic Agent SDK over OpenAI Agents SDK?
  back: 'Native MCP integration auto-discovers tools from MCP servers (no manual schema). Computer use
    for desktop automation. Extended thinking for better reasoning. Safety-first design. Tradeoff: lighter
    orchestration than LangGraph; less control flow flexibility.'
---

## Intuition

AnthropicAgent SDK is a lightweight agent framework deeply integrated with Claude. Instead of building complex state graphs or explicit handoffs, it focuses on **reliable tool-use loops**: Claude reasons, calls tools, receives results, repeats. Built-in [[Extended Thinking]] for complex reasoning. Native [[Model Context Protocol (MCP)]] support. No learning curve—start with few lines of Python.

## Detail

**Core Loop (3 steps):**
1. Send messages + tools to Claude
2. Claude responds with tool_calls (JSON)
3. Execute tools, append results as assistant messages, repeat until stop_reason="end_turn"

**Key Features:**
- **Extended Thinking**: Claude can use up to 10K tokens for internal reasoning before responding. Improves plan quality and error recovery.
- **MCP Server Integration**: Tools auto-discovered from MCP servers (e.g., GitHub, file systems, APIs). No manual schema definition.
- **Computer Use**: Claude can see and control desktop (screenshots, clicks, keyboard). Enables web automation, spreadsheet work, etc.
- **Token Budget**: Explicit budget management—set max_tokens for reasoning + completion. Prevents runaway loops.
- **Safety Defaults**: Claude's constitutional AI mitigates prompt injection, tool misuse. Safer for unsupervised agents.

**Example Loop:**
```python
from anthropic import Anthropic

client = Anthropic()
tools = client.beta.tools.list()  # Discover MCP tools
messages = [{"role": "user", "content": "Research and summarize..."}]

while True:
    response = client.messages.create(model="claude-3-5-sonnet-20241022", 
        max_tokens=4096, tools=tools, messages=messages)
    if response.stop_reason == "end_turn": break
    # Handle tool calls, append to messages
```

## Common gotchas / interview framings

- **No checkpointing**: Unlike LangGraph, no built-in snapshots. Agent failure loses intermediate work. Must implement own persistence.
- **MCP server reliability**: Discovering tools from MCP servers adds latency and failure modes. Network issues crash agent loop.
- **Computer use cost**: Screenshots are high-token (1500+ tokens/image). Long automation tasks balloon token usage.
- **Reasoning budget trade-off**: Extended thinking improves quality but doubles or triples token cost per step. Balance planning vs. speed.
- **Interview scenario**: "Build a code review agent. Design error recovery: if tool fails, retry? skip? escalate? How do you bound token costs for 1000s of PRs/day?"

## See also
- [[claude-extended-thinking]]
- [[model-context-protocol-integration]]
- [[claude-computer-use]]
- [[safety-in-agents]]
- [[tool-use-best-practices]]
- [[multi-turn-agent-loops]]
- [[token-budget-management]]

## Sources
See frontmatter `sources:`.
