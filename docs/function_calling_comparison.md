# Function Calling API Comparison Across LLM Providers

This document compares function calling capabilities across major LLM providers to inform our generic implementation.

## OpenAI

**API Structure:**
- Uses `tools` array with function definitions
- `tool_choice` parameter controls invocation behavior
- Response includes `tool_calls` array with function name and arguments

**JSON Schema:**
```json
{
  "tools": [{
    "type": "function",
    "function": {
      "name": "get_weather",
      "description": "Get current weather for a location",
      "parameters": {
        "type": "object",
        "properties": {
          "location": {
            "type": "string",
            "description": "City and country"
          }
        },
        "required": ["location"]
      }
    }
  }],
  "tool_choice": "auto"
}
```

**Response Format:**
```json
{
  "choices": [{
    "message": {
      "tool_calls": [{
        "id": "call_abc123",
        "type": "function",
        "function": {
          "name": "get_weather",
          "arguments": "{\"location\":\"San Francisco, CA\"}"
        }
      }]
    }
  }]
}
```

## Anthropic Claude

**API Structure:**
- Uses system prompt to define tools
- Response includes `tool_use` type in content array

**Tool Definition (in system prompt):**
```
You have access to these tools:
<tools>
1. Weather API (parameters: location, unit)
2. Database Query (parameters: sql)
</tools>
Return tool calls in JSON format.
```

**Response Format:**
```json
{
  "content": [{
    "type": "tool_use",
    "id": "toolu_01",
    "name": "Weather API",
    "input": {"location": "Paris", "unit": "Celsius"}
  }]
}
```

## Google Gemini

**API Structure:**
- Uses `tools` array with function declarations
- `tool_config` with function calling mode
- Response includes function call with name and args

**JSON Schema:**
```python
tools = [{
  "functionDeclarations": [{
    "name": "get_weather",
    "parameters": {
      "type": "object",
      "properties": {
        "location": {
          "type": "object",
          "properties": {
            "city": {"type": "string"},
            "state": {"type": "string"}
          }
        },
        "date": {"type": "string", "format": "date"}
      }
    }
  }]
}]

tool_config = types.ToolConfig(
  function_calling_config=types.FunctionCallingConfig(
    mode="ANY",
    allowed_function_names=["get_weather"]
  )
)
```

**Response Format:**
```javascript
const funcCall = response.result.functionCall();
if(funcCall.name === 'get_weather') {
  const args = funcCall.args;
  // Process args
}
```

## Grok

**API Structure:**
- Similar to OpenAI with `tools` array
- `tool_choice` parameter controls invocation
- Response includes tool_calls with ID, name, and arguments

**JSON Schema:**
```json
{
  "tools": [{
    "type": "function",
    "function": {
      "name": "get_weather",
      "parameters": {
        "type": "object",
        "properties": {
          "location": {"type": "string"}
        },
        "required": ["location"]
      }
    }
  }],
  "tool_choice": "auto"
}
```

**Response Format:**
```json
{
  "tool_calls": [{
    "tool_call_id": "call_abc123",
    "name": "get_weather",
    "arguments": {"location": "San Francisco"}
  }]
}
```

## OpenRouter

**API Structure:**
- Compatible with OpenAI format
- Uses `tools` array with function definitions
- `tool_choice` parameter controls invocation
- Can combine with `structured_outputs` for JSON responses

**JSON Schema:**
```json
{
  "tools": [{
    "type": "function",
    "function": {
      "name": "print_text",
      "description": "Send text to printer",
      "parameters": {
        "type": "object",
        "properties": {
          "content": {"type": "string"}
        },
        "required": ["content"]
      }
    }
  }],
  "tool_choice": "required"
}
```

**Response Format:**
```json
{
  "choices": [{
    "message": {
      "tool_calls": [{
        "function": {
          "name": "print_text",
          "arguments": "{\"content\":\"Meeting notes...\"}"
        }
      }]
    }
  }]
}
```

## Key Observations

1. **Schema Commonality**: OpenAI, Grok, and OpenRouter share nearly identical schemas
2. **Invocation Control**: All providers offer some form of function selection control
3. **Response Formats**: Vary significantly between providers
4. **Parameter Validation**: All support JSON Schema for parameter validation
5. **Anthropic Difference**: Uses system prompts instead of structured tool definitions

These differences will inform our generic abstraction layer design to accommodate all providers while maintaining a consistent developer experience.
