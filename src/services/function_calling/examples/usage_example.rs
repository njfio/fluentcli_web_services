use std::sync::Arc;
use serde_json::json;
use crate::services::function_calling::{
    Tool, ToolParameter, ParameterType, FunctionCallingManager, ToolChoice
};
use crate::services::function_calling::provider::openai::OpenAIAdapter;
use crate::services::function_calling::tool::registry::ToolRegistry;
use crate::services::function_calling::examples::weather_tool::WeatherTool;

/// Example of how to use the function calling system
pub async fn run_example(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create the tool registry
    let tool_registry = Arc::new(ToolRegistry::new());

    // 2. Register tools
    let weather_tool = WeatherTool::new();
    tool_registry.register(weather_tool).await;

    // 3. Create the provider adapter
    let provider_adapter = Arc::new(OpenAIAdapter);

    // 4. Create the function calling manager
    let function_calling_manager = FunctionCallingManager::new(provider_adapter, tool_registry);

    // 5. Define the tools for the request
    let weather_tool_def = Tool::new(
        "get_weather",
        "Get the current weather for a location",
        vec![
            ToolParameter::new(
                "location",
                ParameterType::String {
                    format: None,
                    enum_values: None,
                },
                true,
            ).with_description("City and country, e.g., 'Paris, France'"),
            ToolParameter::new(
                "units",
                ParameterType::String {
                    format: None,
                    enum_values: Some(vec!["celsius".to_string(), "fahrenheit".to_string()]),
                },
                false,
            ).with_description("Temperature units: 'celsius' or 'fahrenheit'"),
        ],
    );

    // 6. Prepare the request
    let mut request = json!({
        "model": "gpt-4",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant that can provide weather information."
            },
            {
                "role": "user",
                "content": "What's the weather like in Paris today?"
            }
        ],
    });

    function_calling_manager.prepare_request(&mut request, &[weather_tool_def.clone()], ToolChoice::Auto);

    println!("Prepared request: {}", serde_json::to_string_pretty(&request)?);

    // 7. In a real application, you would send this request to the OpenAI API
    // For demonstration, we'll create a mock response
    let mock_response = json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1677858242,
        "model": "gpt-4",
        "choices": [
            {
                "message": {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [
                        {
                            "id": "call_abc123",
                            "type": "function",
                            "function": {
                                "name": "get_weather",
                                "arguments": "{\"location\":\"Paris, France\",\"units\":\"celsius\"}"
                            }
                        }
                    ]
                },
                "finish_reason": "tool_calls"
            }
        ]
    });

    // 8. Handle the response
    let tool_results = function_calling_manager.handle_response(&mock_response).await?;

    println!("Tool results: {}", serde_json::to_string_pretty(&tool_results)?);

    // 9. In a real application, you would send these results back to the LLM
    // For demonstration, we'll create a mock follow-up request
    let mut follow_up_request = json!({
        "model": "gpt-4",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant that can provide weather information."
            },
            {
                "role": "user",
                "content": "What's the weather like in Paris today?"
            },
            {
                "role": "assistant",
                "content": null,
                "tool_calls": [
                    {
                        "id": "call_abc123",
                        "type": "function",
                        "function": {
                            "name": "get_weather",
                            "arguments": "{\"location\":\"Paris, France\",\"units\":\"celsius\"}"
                        }
                    }
                ]
            },
            {
                "role": "tool",
                "tool_call_id": "call_abc123",
                "content": serde_json::to_string(&tool_results[0].result)?
            }
        ],
    });

    function_calling_manager.prepare_request(&mut follow_up_request, &[weather_tool_def], ToolChoice::Auto);

    println!("Follow-up request: {}", serde_json::to_string_pretty(&follow_up_request)?);

    // 10. In a real application, you would send this request to the OpenAI API
    // For demonstration, we'll create a mock final response
    let mock_final_response = json!({
        "id": "chatcmpl-456",
        "object": "chat.completion",
        "created": 1677858242,
        "model": "gpt-4",
        "choices": [
            {
                "message": {
                    "role": "assistant",
                    "content": "The weather in Paris today is partly cloudy with a temperature of 22.5°C. The humidity is at 65% with a wind speed of 10 km/h. For today, you can expect a high of 24.5°C and a low of 17.5°C. Tomorrow will be sunny with a high of 26.5°C and a low of 19.5°C."
                },
                "finish_reason": "stop"
            }
        ]
    });

    // 11. Extract the final response
    let final_response = mock_final_response["choices"][0]["message"]["content"].as_str().unwrap();

    println!("Final response: {}", final_response);

    Ok(())
}
