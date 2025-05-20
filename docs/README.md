# Generic Function Calling System Documentation

This directory contains comprehensive documentation for implementing a generic function calling system that works across multiple LLM providers.

## Overview

The generic function calling system provides a unified interface for defining tools/functions and handling their invocation across different LLM providers, including OpenAI, Anthropic Claude, Google Gemini, Grok, and OpenRouter. The system abstracts away provider-specific implementation details while maintaining the full capabilities of each provider.

## Documentation Index

### Core Concepts

1. [Function Calling Comparison](function_calling_comparison.md) - Comparison of function calling capabilities across LLM providers
2. [Generic Function Calling Design](generic_function_calling_design.md) - Architecture and implementation details for the generic system

### Implementation Guides

1. [Rust Implementation Guide](rust_implementation_guide.md) - Detailed guide for implementing the system in Rust
2. [Implementation Roadmap](implementation_roadmap.md) - Phased implementation plan with timelines and milestones

### Integration and Extension

1. [MCP Integration Guide](mcp_integration_guide.md) - Guide for integrating with the Mendable Control Protocol
2. [Tool Development Guide](tool_development_guide.md) - Guide for developing tools for the function calling system
3. [Agent Programming Guide](agent_programming_guide.md) - Guide for creating and programming AI agents

## Key Features

- **Provider Abstraction**: Unified interface for multiple LLM providers
- **Tool Definition**: Consistent schema for defining tools and parameters
- **Tool Execution**: Framework for registering and executing tools
- **Error Handling**: Comprehensive error handling for various failure modes
- **MCP Integration**: Support for remote tool execution via MCP
- **Agent Framework**: System for creating and managing AI agents

## Getting Started

To get started with the generic function calling system:

1. Read the [Generic Function Calling Design](generic_function_calling_design.md) document to understand the system architecture
2. Follow the [Rust Implementation Guide](rust_implementation_guide.md) to implement the core components
3. Use the [Tool Development Guide](tool_development_guide.md) to create tools for your specific use cases
4. Refer to the [Agent Programming Guide](agent_programming_guide.md) to build agents that use your tools

## Implementation Strategy

The implementation is divided into six phases:

1. **Core Infrastructure**: Basic data structures, provider adapters, and tool execution framework
2. **Provider Coverage**: Support for all major LLM providers
3. **Advanced Features**: Streaming, parallel execution, and caching
4. **MCP Integration**: Remote tool execution via MCP
5. **Agent Framework**: Agent definition, management, and patterns
6. **Tool Library**: Collection of ready-to-use tools

See the [Implementation Roadmap](implementation_roadmap.md) for a detailed timeline.

## Architecture

The system follows a modular architecture:

```
┌─────────────────┐     ┌───────────────────┐     ┌─────────────────┐
│                 │     │                   │     │                 │
│  Application    │────▶│  Generic Function │────▶│  LLM Provider   │
│                 │     │  Calling System   │     │  (OpenAI, etc.) │
│                 │◀────│                   │◀────│                 │
└─────────────────┘     └───────────────────┘     └─────────────────┘
                                 │
                                 ▼
                        ┌─────────────────┐
                        │                 │
                        │  Tool Registry  │
                        │                 │
                        └───────┬─────────┘
                                │
                    ┌───────────┼───────────┐
                    │           │           │
            ┌───────▼───┐ ┌─────▼─────┐ ┌───▼───────┐
            │           │ │           │ │           │
            │ Local     │ │ MCP       │ │ External  │
            │ Tools     │ │ Client    │ │ Services  │
            │           │ │           │ │           │
            └───────────┘ └───────────┘ └───────────┘
```

## Contributing

Contributions to the generic function calling system are welcome! Please follow these steps:

1. Read the relevant documentation to understand the system
2. Follow the coding standards and best practices
3. Write tests for your changes
4. Submit a pull request with a clear description of your changes

## License

This project is licensed under the MIT License - see the LICENSE file for details.
