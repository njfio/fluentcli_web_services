use std::sync::Arc;
use serde_json::json;
use crate::services::function_calling::{
    Tool, ToolParameter, ParameterType, FunctionCallingManager, ToolChoice
};
use crate::services::function_calling::provider::openai::OpenAIAdapter;
use crate::services::function_calling::tool::registry::ToolRegistry;
use crate::services::function_calling::examples::weather_tool::WeatherTool;

#[tokio::test]
async fn test_openai_adapter_format_tools() {
    // Create a tool
    let weather_tool = Tool::new(
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
    
    // Create the OpenAI adapter
    let adapter = OpenAIAdapter;
    
    // Format the tools
    let formatted_tools = adapter.format_tools(&[weather_tool]);
    
    // Verify the formatted tools
    assert!(formatted_tools.is_array());
    assert_eq!(formatted_tools.as_array().unwrap().len(), 1);
    
    let formatted_tool = &formatted_tools[0];
    assert_eq!(formatted_tool["type"], "function");
    assert_eq!(formatted_tool["function"]["name"], "get_weather");
    assert_eq!(formatted_tool["function"]["description"], "Get the current weather for a location");
    
    let parameters = &formatted_tool["function"]["parameters"];
    assert_eq!(parameters["type"], "object");
    
    let properties = &parameters["properties"];
    assert!(properties.is_object());
    assert!(properties.as_object().unwrap().contains_key("location"));
    assert!(properties.as_object().unwrap().contains_key("units"));
    
    let required = &parameters["required"];
    assert!(required.is_array());
    assert_eq!(required.as_array().unwrap().len(), 1);
    assert_eq!(required[0], "location");
}

#[tokio::test]
async fn test_tool_execution() {
    // Create the tool registry
    let tool_registry = Arc::new(ToolRegistry::new());
    
    // Register the weather tool
    let weather_tool = WeatherTool::new();
    tool_registry.register(weather_tool).await;
    
    // Execute the tool
    let args = json!({
        "location": "Paris, France",
        "units": "celsius"
    });
    
    let result = tool_registry.execute("get_weather", args).await.unwrap();
    
    // Verify the result
    assert!(result.is_object());
    assert!(result.as_object().unwrap().contains_key("location"));
    assert!(result.as_object().unwrap().contains_key("temperature"));
    assert!(result.as_object().unwrap().contains_key("condition"));
    assert!(result.as_object().unwrap().contains_key("humidity"));
    assert!(result.as_object().unwrap().contains_key("wind_speed"));
    assert!(result.as_object().unwrap().contains_key("forecast"));
    
    assert_eq!(result["location"], "Paris, France");
    assert_eq!(result["units"], "celsius");
}

#[tokio::test]
async fn test_function_calling_manager() {
    // Create the tool registry
    let tool_registry = Arc::new(ToolRegistry::new());
    
    // Register the weather tool
    let weather_tool = WeatherTool::new();
    tool_registry.register(weather_tool).await;
    
    // Create the provider adapter
    let provider_adapter = Arc::new(OpenAIAdapter);
    
    // Create the function calling manager
    let function_calling_manager = FunctionCallingManager::new(provider_adapter, tool_registry);
    
    // Define the tools for the request
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
    
    // Prepare the request
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
    
    function_calling_manager.prepare_request(&mut request, &[weather_tool_def], ToolChoice::Auto);
    
    // Verify the request
    assert!(request.as_object().unwrap().contains_key("tools"));
    assert!(request.as_object().unwrap().contains_key("tool_choice"));
    
    // Create a mock response
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
    
    // Handle the response
    let tool_results = function_calling_manager.handle_response(&mock_response).await.unwrap();
    
    // Verify the tool results
    assert_eq!(tool_results.len(), 1);
    assert_eq!(tool_results[0].tool_call_id, "call_abc123");
    assert!(tool_results[0].result.is_object());
    assert_eq!(tool_results[0].result["location"], "Paris, France");
    assert_eq!(tool_results[0].result["units"], "celsius");
}
