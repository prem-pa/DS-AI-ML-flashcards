---
id: 57d01c46-da8f-43df-b851-21a35c6429e9
title: Parallel tool execution
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- concurrency
- batching
- tool-independence
- async-execution
- latency-optimization
- dependency-tracking
aliases:
- Concurrent tool calls
- Batch tool execution
- Multi-tool parallelism
sources:
- url: https://medium.com/@atnoforgenai/10-ai-agent-frameworks-you-should-know-in-2026-langgraph-crewai-autogen-more-2e0be4055556
  label: 10 AI Agent Frameworks You Should Know in 2026
- url: https://sureprompts.com/blog/react-prompting-guide
  label: ReAct Prompting Guide 2026
- url: https://servicesground.com/blog/agentic-reasoning-patterns/
  label: Agentic Reasoning Patterns 2026
- url: https://towardsdatascience.com/recap-of-all-types-of-llm-agents/
  label: Recap of All Types of LLM Agents
cards:
- id: 6d10d7a9-ef44-4559-a282-d7ce6f16f727
  type: flip
  front: When can tool calls be executed in parallel?
  back: 'When tools are independent—output of one doesn''t feed input to another. Examples: fetch from
    3 APIs, search 2 databases, check 2 conditions. Cannot parallelize: tool 1 → tool 2 (sequential) or
    tool 1 & tool 2 share mutable state.'
- id: f8e6a6fc-cc68-4232-8e96-ec16c3028eed
  type: flip
  front: How do modern LLMs enable parallel tool execution?
  back: In one response, model emits array of tool_calls (not nested/sequential). Framework receives all
    calls, executes in parallel via async/await or thread pool, collects results, appends all to conversation.
    Model sees batched results in next turn.
- id: e6310ac1-4202-491e-a8f2-6def20e1e267
  type: mcq
  front: You have 10 parallel API calls, each with 1s timeout. Global timeout is 5s. One call hangs. How
    do you handle it?
  back: Use `asyncio.wait(timeout=global_timeout)` to collect all completed results within deadline. Return
    partial successes + timeouts as results. Model adapts and reruns only failed calls if needed. Graceful
    degradation.
  choices:
  - key: a
    text: Wait 10s for all to complete (ignore global timeout)
    correct: false
  - key: b
    text: Use asyncio.wait(timeout=5s) to enforce global deadline; return partial results
    correct: true
  - key: c
    text: Retry the hanging call individually
    correct: false
  - key: d
    text: Cancel all and return error
    correct: false
- id: 164e7865-c117-45fb-968d-4f0767be14b1
  type: flip
  front: ''
  back: ''
- id: 0fc24283-0a7c-4b88-ba57-47d81ef7115b
  type: flip
  front: What is a risk of parallel execution with shared state?
  back: 'Race conditions. If two tools modify same resource (e.g., database row), order matters. Parallel
    execution removes ordering guarantee. Solution: ensure tools are truly independent (no shared mutable
    state) or use locks/transactions.'
---

## Intuition

**Parallel tool execution** reduces latency in multi-step agent loops. Instead of waiting for tool 1 to complete before calling tool 2, if tools are independent (e.g., fetching data from 3 APIs), issue all calls simultaneously. Trade-off: complexity (error handling, resource management) vs speed.

## Detail

**Dependency vs Independence:**
- **Independent**: `search_api("weather")` + `search_api("traffic")` can run in parallel
- **Dependent**: `get_user_id(email)` → `get_user_profile(user_id)` must run sequentially

**Model-Level Parallelism:** Modern models (Claude 3.5+, GPT-4o) can emit multiple tool_calls in one response:
```json
{
  "tool_calls": [
    {"name": "get_weather", "args": {"city": "Seattle"}},
    {"name": "get_traffic", "args": {"location": "Seattle"}},
    {"name": "get_news", "args": {"query": "Seattle tech"}}
  ]
}
```

**Framework Implementation:**
```python
async def execute_tools_parallel(tool_calls):
    tasks = [execute_tool(tc) for tc in tool_calls]
    results = await asyncio.gather(*tasks, return_exceptions=True)
    return results
```

**Handling Partial Failures:**
- If 2/3 tools succeed, append successful results + error for failed one
- Model sees both success and failure, decides whether to retry or proceed
- Graceful degradation better than aborting entire step

## Common gotchas / interview framings

- **Resource limits**: Parallel calls to same API may hit rate limits. Need circuit breakers, backoff logic.
- **Timeout handling**: One slow tool blocks the batch (gather with timeout). Use `asyncio.wait(timeout=X)` to enforce deadline.
- **Error correlation**: If parallel call 2 fails, which retry strategy? Retry only failed, retry all, or propagate error?
- **Model expectation**: Not all models reliably emit parallel calls. May need prompt coaching or in-context examples.
- **Interview scenario**: "You have 10 independent tool calls. Design execution with 5s global timeout, 1s per-call timeout, and max 3 retries. Handle partial failures."

## See also
- [[async-tool-execution]]
- [[dependency-graphs-for-tools]]
- [[tool-latency-optimization]]
- [[concurrent-api-calls]]
- [[batching-strategies]]

## Sources
See frontmatter `sources:`.
