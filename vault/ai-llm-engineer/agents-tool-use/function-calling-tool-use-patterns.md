---
id: 49da3a17-effd-4b0c-9210-4a901743a261
title: Function calling / tool use patterns
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- function-calling
- tool-use
- JSON-schemas
- parallel-calls
- response-format
- tool-execution-loop
aliases:
- Tool calling
- Function definitions
- Tool response loop
sources:
- url: https://medium.com/@atnoforgenai/10-ai-agent-frameworks-you-should-know-in-2026-langgraph-crewai-autogen-more-2e0be4055556
  label: 10 AI Agent Frameworks You Should Know in 2026
- url: https://softmaxdata.com/blog/definitive-guide-to-agentic-frameworks-in-2026-langgraph-crewai-ag2-openai-and-more/
  label: Definitive Guide to Agentic Frameworks in 2026
- url: https://happycapyguide.com/blog/ai-agent-frameworks-compared-2026
  label: AI Agent Frameworks Compared 2026
- url: https://sureprompts.com/blog/react-prompting-guide
  label: ReAct Prompting Guide 2026
cards:
- id: 87c6787b-0e09-467c-a8dd-815f6c0a13e4
  type: flip
  front: Describe the function calling loop in an agent.
  back: Agent sends message + tool definitions (JSON schema) to model. Model responds with tool_call (name
    + parameters). Agent executes tool, receives result. Agent appends result to conversation. Model reasons
    on result and either calls another tool, hands off, or responds with answer. Loop repeats.
- id: 610b4228-ed7c-44bc-9862-058ebbdc35c2
  type: flip
  front: Why is tool definition schema quality critical?
  back: Vague or ambiguous schemas lead to incorrect parameter bindings. Model may hallucinate parameters
    not in schema. Clear descriptions (e.g., 'city name as ISO 3166 code' vs 'city') reduce errors. Test
    schemas with adversarial queries.
- id: 64f14de8-e158-484f-ad48-93db9aaf85ae
  type: mcq
  front: 'Your tool schema has parameters: {"name", "email", "phone"}. Model invents "fax_number" not
    in schema and calls the tool. What went wrong?'
  back: 'Models can hallucinate parameters, especially if schema is loose. Use `additionalProperties:
    false` in JSON schema and validate inputs before execution. Both schema strictness and execution-time
    validation prevent surprises.'
  choices:
  - key: a
    text: The model is malfunctioning
    correct: false
  - key: b
    text: Schema was not strict (missing additionalProperties=false or validation)
    correct: false
  - key: c
    text: Tool execution should have rejected invalid parameters
    correct: false
  - key: d
    text: Both B and C
    correct: true
- id: 698ffa54-80d3-43df-85b4-3091e15dd223
  type: flip
  front: ''
  back: ''
- id: 365c714a-587f-4367-b903-5f96c2711132
  type: flip
  front: Can a model call multiple tools in a single response?
  back: Yes, most modern models (Claude, GPT-4, Gemini) support parallel tool calls. Response contains
    array of tool_calls. Agent executes all in parallel (if independent), batches results, appends to
    conversation. Improves efficiency for multi-step tasks.
---

## Intuition

**Function calling** is the mechanism by which LLMs invoke external tools. Agent sends a message to the model, model responds with a **tool_call** (name + JSON-encoded parameters). Agent executes the tool, receives result, appends result to conversation, repeats. This is the atomic building block of agentic AI.

## Detail

**Call-Result Cycle:**
1. Agent: "Fetch weather for Seattle"
2. Model: `tool_call(name="get_weather", args={"city": "Seattle"})`
3. Agent: Execute tool, receive `{"temp": 62, "condition": "cloudy"}`
4. Model: "The weather in Seattle is 62°F and cloudy."

**Tool Definition (JSON Schema):**
```json
{
  "type": "object",
  "properties": {
    "city": {"type": "string", "description": "City name"},
    "units": {"type": "string", "enum": ["C", "F"]}
  },
  "required": ["city"]
}
```

All major models (Claude, GPT-4, Gemini) use JSON schemas to define tools. Schema quality directly impacts call accuracy.

**Parallel Tool Calls:** Modern models can issue multiple tool calls in a single response:
```json
[{"name": "get_weather", "args": {"city": "Seattle"}},
 {"name": "get_weather", "args": {"city": "Portland"}}]
```
Agent executes both in parallel, batches results, feeds back to model.

**Tool Response Format:** Different models expect different response formats. Claude: `tool_result` role with `content`. GPT: `function` role. Frameworks abstract this.

## Common gotchas / interview framings

- **Schema ambiguity**: Vague descriptions lead to incorrect parameter bindings. Example: `"name"` vs `"full_name"` confuses model. Use clear, specific descriptions.
- **Parameter hallucination**: Model may invent parameters not in schema. Validate before execution.
- **Error recovery**: If tool fails (e.g., API timeout), how does agent retry? Most frameworks re-append error as tool_result.
- **Streaming tool calls**: Models stream tool calls token-by-token. Incomplete JSON arrives mid-stream—need buffering before parsing.
- **Interview scenario**: "Design a tool for `create_database(name, schema, backup_enabled)`. How do you prevent the model from inventing `backup_retention_days` not in your schema? How do you handle validation errors?"

## See also
- [[tool-definition-schemas]]
- [[parallel-tool-execution]]
- [[tool-response-parsing]]
- [[error-handling-in-tool-calls]]
- [[streaming-tool-calls]]

## Sources
See frontmatter `sources:`.
