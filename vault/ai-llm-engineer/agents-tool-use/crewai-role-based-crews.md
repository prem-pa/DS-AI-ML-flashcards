---
id: efdbf1e1-fc88-4e0c-89a0-1ca883d0b839
title: CrewAI (role-based crews)
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- role-based
- DSL
- crew-orchestration
- low-learning-curve
- multi-agent
- task-based
aliases:
- Crew orchestration
- Role assignments
- Task-driven agents
sources:
- url: https://gurusup.com/blog/best-multi-agent-frameworks-2026
  label: Best Multi-Agent Frameworks in 2026
- url: https://dev.to/emperorakashi20/crewai-vs-langgraph-vs-autogen-which-multi-agent-framework-should-you-use-in-2026-5h2f
  label: CrewAI vs LangGraph vs AutoGen Comparison
- url: https://openagents.org/blog/posts/2026-02-23-open-source-ai-agent-frameworks-compared
  label: Open-Source AI Agent Frameworks Compared 2026
- url: https://fungies.io/ai-agent-frameworks-comparison-2026-langchain-crewai-autogen/
  label: AI Agent Frameworks Comparison 2026
cards:
- id: 6060ecb9-20b4-40a7-8877-ab5c53360160
  type: flip
  front: What is CrewAI's core mental model?
  back: Crews are teams of role-based agents. Each agent has a role, backstory, goal, and assigned tools.
    Tasks describe what to accomplish and assign agents. The crew processes tasks via a chosen process
    (sequential, hierarchical, custom). Simpler mental model than state graphs.
- id: 9e5f6ff1-0038-4fef-b171-4c833ffef153
  type: flip
  front: Compare CrewAI's sequential vs. hierarchical process.
  back: 'Sequential: Task 1 (Agent A) → Task 2 (Agent B) → ... Output of one feeds next. Hierarchical:
    Manager agent receives all tasks, decides order, prioritizes, delegates. Sequential is predictable;
    hierarchical is flexible but adds LLM overhead for decision-making.'
- id: e7302540-8450-4361-bb6b-b3dcdb14ffde
  type: mcq
  front: In a 3-agent CrewAI crew, all agents have the search_tool. Which agent calls it?
  back: Tool assignment is implicit. If multiple agents have search_tool and neither task nor model output
    specifies which agent calls it, behavior is non-deterministic. Design tasks to be agent-specific.
  choices:
  - key: a
    text: The agent whose task explicitly uses search_tool
    correct: false
  - key: b
    text: The first agent in the crew definition
    correct: false
  - key: c
    text: Depends on model behavior and task assignment—non-deterministic
    correct: true
  - key: d
    text: CrewAI forbids duplicate tools
    correct: false
- id: 73d243a8-b3b1-49e7-883b-35c504269168
  type: flip
  front: ''
  back: ''
- id: d867bae2-d591-49b9-8e43-5d2390e0a0f8
  type: flip
  front: What is a key limitation of CrewAI's memory?
  back: 'Long-term memory grows unbounded with no native pruning. Agents store learned patterns but there''s
    no automated cleanup or summarization. Interview question: How do you manage memory across 100s of
    crew runs without bloat?'
---

## Intuition

CrewAI abstracts multi-agent systems as **crews of role-based agents**. Each agent has a role (e.g., "Research Lead"), backstory, goal, and assigned tools. A task describes what to accomplish. The crew processes tasks via a chosen process (sequential, hierarchical, or custom). This is a simpler mental model than state graphs: assign roles, define tasks, agents collaborate to solve them.

## Detail

**Core Components:**
- **Agent**: Role, backstory, goal, tools, LLM model, memory
- **Task**: Description, expected output, assigned agent(s), tools
- **Crew**: Collection of agents, process type, shared context
- **Process**: Sequential (Agent 1 → Agent 2 → ...), Hierarchical (manager agent), Custom callback

**Typical Setup (20 LoC example):**
```python
researcher = Agent(role="Research Lead", goal="Uncover facts", tools=[search_tool, scrape_tool])
writer = Agent(role="Content Writer", goal="Craft narrative", tools=[write_tool])

task1 = Task(description="Research AI trends", agent=researcher)
task2 = Task(description="Write blog post", agent=writer)

crew = Crew(agents=[researcher, writer], tasks=[task1, task2], process="sequential")
result = crew.kickoff()
```

**Process Types:**
- **Sequential**: Tasks execute in order; output of one feeds input to next
- **Hierarchical**: Manager agent reads all tasks, decides order, delegates
- **Custom**: User-defined callback orchestrates execution

**Memory Management**: Agents store short-term (conversation) and long-term (learned patterns) memory. Memory persists across crew runs.

## Common gotchas / interview framings

- **No explicit control flow**: Unlike LangGraph, no explicit state graph. Control flow is implicit in process choice. Hard to trace why Agent B didn't run as expected.
- **Memory bloat**: Long-term memory can grow without bounds. No native pruning—manually manage.
- **Tool assignment ambiguity**: If two agents have the same tool, which one calls it? Depends on task assignment and model behavior—non-deterministic.
- **Handoff latency**: Sequential process waits for full task completion before next agent starts. Parallel execution not native.
- **Interview scenario**: "Design a 5-agent crew for research + writing + review. How do you enforce approval gates? Where do you cache search results between agents?"

## See also
- [[agent-roles]]
- [[crew-orchestration]]
- [[task-based-architecture]]
- [[agent-memory-in-crewai]]
- [[crew-process-types]]
- [[tool-assignment-by-role]]

## Sources
See frontmatter `sources:`.
