# Collaborative Agentic Process Plan

This document proposes a high-level design for adding collaborative agentic workflows to FluentCLI Web Services. The goal is to allow multiple LLM-driven agents to coordinate on tasks using the existing chat, LLM provider and job infrastructure.

## Overview

FluentCLI already supports conversations and streaming chat with different LLM providers. The new agent system will build on top of these features and introduce orchestration components for agents that can work together toward a goal.

## Key Concepts

- **Agent Session** – A container for one collaborative run. Each session links to a conversation and tracks participating agents.
- **Agent** – An autonomous unit backed by an LLM provider and optional tools. Agents exchange messages within the session conversation.
- **Agent Manager** – Service responsible for stepping agents, maintaining state and invoking LLMService for message generation.

## Database Additions

1. `agent_sessions` table
   - `id` (UUID primary key)
   - `conversation_id` (foreign key to `conversations`)
   - `created_at`, `updated_at`
2. `agents` table
   - `id` (UUID primary key)
   - `session_id` (foreign key to `agent_sessions`)
   - `user_llm_config_id` – selects provider/model via existing user LLM configs
   - `role_name` – descriptive name such as "planner" or "critic"
   - `tool_config` – JSON blob describing available tools

Corresponding Rust models would follow the style used in existing models such as `LLMProvider`【F:src/models/llm_provider.rs†L1-L31】.

## Services and APIs

### AgentService

A new service module will coordinate agent sessions. Functions include:

- `create_session(user_id, title)` – creates a conversation and session record.
- `add_agent(session_id, user_llm_config_id, role_name, tool_config)`.
- `run_step(session_id)` – for each agent, gather recent conversation messages and call `LLMService::llm_stream_chat` to generate the agent's next message.
- `end_session(session_id)` – finalize records.

Implementation can leverage the existing streaming chat logic starting at line 102 of `LLMService`【F:src/services/llm_service.rs†L102-L178】.

### HTTP Routes

New Actix Web handlers will expose REST endpoints:

- `POST /agents/sessions` – start a collaborative session.
- `POST /agents/{session_id}/agents` – add an agent to a session.
- `POST /agents/{session_id}/step` – execute one iteration of all agents.
- `DELETE /agents/{session_id}` – terminate the session.

Routes should be registered similarly to current chat endpoints described in `api_documentation.md`【F:api_documentation.md†L1-L30】.

## Agent Workflow

1. A client creates a session and adds agents specifying provider configs.
2. Each step collects the conversation history for context and calls the provider through `LLMService`.
3. Generated messages are stored via `MessageService::create_message_with_attachments` to handle any images or files【F:src/services/chat/message_service.rs†L18-L75】.
4. The manager evaluates stopping conditions (goal reached or max rounds) and ends the session.

The process can be run synchronously via HTTP requests or asynchronously using a dedicated worker similar to `worker_app/src/main.rs`【F:worker_app/src/main.rs†L1-L22】【F:worker_app/src/main.rs†L23-L22】 (which demonstrates running tasks with Actix).

## Future Enhancements

- **Tool Invocation** – allow agents to trigger FluentCLI jobs using the existing job service.
- **Scheduling** – integrate timers or event triggers for autonomous execution loops.
- **Front‑End UI** – extend the Tauri/Vue interface to visualize agent interactions in real time.

## Conclusion

By introducing agent sessions, agent records and an AgentService, FluentCLI Web Services can support multi‑agent collaboration while reusing the current chat and LLM provider infrastructure. This foundation enables experimentation with advanced workflows such as planning, code generation and iterative improvement among multiple specialized agents.

