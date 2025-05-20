# Agent Programming Guide

This guide explains how to create and program AI agents using our generic function calling system.

## What is an Agent?

An agent is an AI system that can:
1. Perceive its environment through tools
2. Make decisions based on those perceptions
3. Take actions to achieve specific goals
4. Learn from the results of its actions

Our function calling system enables the creation of agents by providing a framework for defining tools, handling their execution, and managing the conversation flow.

## Agent Architecture

```
┌─────────────────────────────────────────────────────────┐
│                         Agent                           │
│                                                         │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │             │    │             │    │             │  │
│  │  Perception │◀───│  Reasoning  │───▶│   Action    │  │
│  │   (Tools)   │    │    (LLM)    │    │   (Tools)   │  │
│  │             │    │             │    │             │  │
│  └─────────────┘    └─────────────┘    └─────────────┘  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Creating an Agent

### 1. Define Agent Goals and Capabilities

Start by defining what your agent should accomplish and what tools it needs:

```rust
pub struct AgentDefinition {
    pub name: String,
    pub description: String,
    pub goals: Vec<String>,
    pub tools: Vec<Tool>,
}

let research_agent = AgentDefinition {
    name: "ResearchAssistant".to_string(),
    description: "An agent that helps with research by searching the web, summarizing content, and organizing information.".to_string(),
    goals: vec![
        "Find relevant information on a given topic".to_string(),
        "Summarize and extract key points".to_string(),
        "Organize information in a structured format".to_string(),
    ],
    tools: vec![
        Tool::new("search_web", "Search the web for information", vec![
            ToolParameter::new("query", ParameterType::String, true),
            ToolParameter::new("num_results", ParameterType::Number, false),
        ]),
        Tool::new("read_webpage", "Read and extract content from a webpage", vec![
            ToolParameter::new("url", ParameterType::String, true),
        ]),
        Tool::new("summarize_text", "Summarize a piece of text", vec![
            ToolParameter::new("text", ParameterType::String, true),
            ToolParameter::new("max_length", ParameterType::Number, false),
        ]),
    ],
};
```

### 2. Implement Tool Executors

Create executors for each tool:

```rust
pub struct WebSearchTool {
    client: reqwest::Client,
    api_key: String,
}

impl WebSearchTool {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
        }
    }
}

#[async_trait]
impl ToolExecutor for WebSearchTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        let query = args["query"].as_str().ok_or(ToolError::InvalidArgument("query".to_string()))?;
        let num_results = args["num_results"].as_u64().unwrap_or(5);
        
        // Perform web search using a search API
        let response = self.client
            .get("https://api.search.com/search")
            .query(&[
                ("q", query),
                ("num", &num_results.to_string()),
                ("key", &self.api_key),
            ])
            .send()
            .await
            .map_err(|e| ToolError::ExecutionError(e.to_string()))?;
            
        let search_results = response.json::<Value>()
            .await
            .map_err(|e| ToolError::DeserializationError(e.to_string()))?;
            
        Ok(search_results)
    }
}

// Implement other tool executors similarly
```

### 3. Create the Agent Manager

The agent manager orchestrates the conversation flow and tool execution:

```rust
pub struct AgentManager {
    definition: AgentDefinition,
    function_calling_manager: FunctionCallingManager,
    conversation_history: Vec<Message>,
    llm_client: Box<dyn LLMClient>,
}

impl AgentManager {
    pub fn new(
        definition: AgentDefinition,
        tool_registry: ToolRegistry,
        llm_provider: LLMProvider,
        llm_client: Box<dyn LLMClient>,
    ) -> Self {
        let function_calling_manager = FunctionCallingManager::new(llm_provider, tool_registry);
        
        Self {
            definition,
            function_calling_manager,
            conversation_history: Vec::new(),
            llm_client,
        }
    }
    
    pub async fn process_message(&mut self, user_message: &str) -> Result<String, AgentError> {
        // Add user message to conversation history
        self.conversation_history.push(Message::user(user_message));
        
        // Prepare LLM request with tools
        let mut request = self.prepare_request(user_message);
        self.function_calling_manager.prepare_request(
            &mut request,
            self.definition.tools.clone(),
            ToolChoice::Auto,
        );
        
        // Send request to LLM provider
        let response = self.llm_client.send_request(request).await?;
        
        // Process the response
        let (assistant_message, tool_calls) = self.process_response(response).await?;
        
        // If there were tool calls, execute them and continue the conversation
        if !tool_calls.is_empty() {
            let tool_results = self.function_calling_manager.handle_response(tool_calls).await?;
            
            // Add tool results to conversation history
            for (tool_id, result) in tool_results.as_object().unwrap()["tool_results"].as_array().unwrap() {
                self.conversation_history.push(Message::tool(
                    result.to_string(),
                    tool_id.as_str().unwrap(),
                ));
            }
            
            // Continue conversation with tool results
            let follow_up_request = self.prepare_follow_up_request();
            let follow_up_response = self.llm_client.send_request(follow_up_request).await?;
            
            let (final_message, _) = self.process_response(follow_up_response).await?;
            self.conversation_history.push(Message::assistant(&final_message));
            
            Ok(final_message)
        } else {
            // No tool calls, just return the assistant message
            self.conversation_history.push(Message::assistant(&assistant_message));
            Ok(assistant_message)
        }
    }
    
    // Helper methods
    fn prepare_request(&self, user_message: &str) -> Value {
        // Create request with system prompt and conversation history
        // ...
    }
    
    fn prepare_follow_up_request(&self) -> Value {
        // Create follow-up request with updated conversation history
        // ...
    }
    
    async fn process_response(&self, response: Value) -> Result<(String, Value), AgentError> {
        // Extract assistant message and tool calls from response
        // ...
    }
}
```

## Agent Programming Patterns

### 1. ReAct Pattern (Reasoning + Acting)

The ReAct pattern involves alternating between reasoning and acting:

```
1. Reason: Analyze the user's request
2. Act: Use a tool to gather information
3. Reason: Interpret the tool's output
4. Act: Use another tool based on the interpretation
5. Reason: Synthesize a final response
```

Implementation:

```rust
// System prompt for ReAct pattern
let system_prompt = format!(
    "You are {}, {}. Follow these steps for each user request:
    1. Think about what information you need
    2. Use the appropriate tool to gather that information
    3. Analyze the results
    4. Use additional tools if needed
    5. Provide a final answer
    
    Available tools:
    {}",
    agent.name,
    agent.description,
    format_tools_for_prompt(&agent.tools)
);
```

### 2. Planning Pattern

The planning pattern involves creating a plan before taking actions:

```
1. Analyze the user's request
2. Create a step-by-step plan
3. Execute each step of the plan
4. Adjust the plan if needed
5. Synthesize a final response
```

Implementation:

```rust
// System prompt for Planning pattern
let system_prompt = format!(
    "You are {}, {}. Follow these steps for each user request:
    1. Analyze what the user is asking for
    2. Create a detailed plan with specific steps
    3. For each step, use the appropriate tool
    4. If a step fails, adjust your plan
    5. Provide a final answer that addresses the user's request
    
    Available tools:
    {}",
    agent.name,
    agent.description,
    format_tools_for_prompt(&agent.tools)
);
```

### 3. Reflection Pattern

The reflection pattern involves evaluating the quality of responses:

```
1. Generate an initial response
2. Evaluate the response against criteria
3. Improve the response if needed
4. Repeat until satisfied
```

Implementation:

```rust
// System prompt for Reflection pattern
let system_prompt = format!(
    "You are {}, {}. Follow these steps for each user request:
    1. Generate an initial response using tools as needed
    2. Evaluate your response: Is it complete? Accurate? Helpful?
    3. If the response needs improvement, use additional tools
    4. Provide your final, improved response
    
    Available tools:
    {}",
    agent.name,
    agent.description,
    format_tools_for_prompt(&agent.tools)
);
```

## Best Practices

1. **Clear Tool Definitions**: Provide detailed descriptions and parameter information
2. **Effective System Prompts**: Guide the agent's behavior with clear instructions
3. **Error Handling**: Implement robust error handling for tool failures
4. **Conversation Management**: Maintain context across multiple turns
5. **Memory Management**: Implement summarization for long conversations
6. **Logging**: Log all agent actions for debugging and improvement

## Example: Research Agent

```rust
// 1. Define the agent
let research_agent = AgentDefinition {
    name: "ResearchAssistant".to_string(),
    description: "An agent that helps with research by searching the web, summarizing content, and organizing information.".to_string(),
    goals: vec![
        "Find relevant information on a given topic".to_string(),
        "Summarize and extract key points".to_string(),
        "Organize information in a structured format".to_string(),
    ],
    tools: vec![
        Tool::new("search_web", "Search the web for information", vec![
            ToolParameter::new("query", ParameterType::String, true),
            ToolParameter::new("num_results", ParameterType::Number, false),
        ]),
        Tool::new("read_webpage", "Read and extract content from a webpage", vec![
            ToolParameter::new("url", ParameterType::String, true),
        ]),
        Tool::new("summarize_text", "Summarize a piece of text", vec![
            ToolParameter::new("text", ParameterType::String, true),
            ToolParameter::new("max_length", ParameterType::Number, false),
        ]),
    ],
};

// 2. Set up tool registry
let mut tool_registry = ToolRegistry::new();
tool_registry.register("search_web", WebSearchTool::new("api_key_here"));
tool_registry.register("read_webpage", WebpageReaderTool::new());
tool_registry.register("summarize_text", TextSummarizerTool::new());

// 3. Create agent manager
let llm_client = OpenAIClient::new("openai_api_key_here");
let agent_manager = AgentManager::new(
    research_agent,
    tool_registry,
    LLMProvider::OpenAI,
    Box::new(llm_client),
);

// 4. Process user message
let response = agent_manager.process_message(
    "Research the latest advancements in quantum computing and summarize the key findings."
).await?;

println!("Agent response: {}", response);
```

## Next Steps

1. Implement the agent manager
2. Create example agents for different use cases
3. Develop testing framework for agents
4. Add monitoring and analytics
5. Implement agent memory and learning
