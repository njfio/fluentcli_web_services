use crate::db::DbPool;
use crate::error::AppError;
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use std::sync::Arc;
use lazy_static::lazy_static;
use crate::services::function_calling::tool::registry::ToolRegistry;
use crate::services::function_calling::tool::executor::{ToolExecutor, ToolExecutorBase};
use crate::services::function_calling::tool::error::ToolError;
use crate::services::function_calling::examples::weather_tool::WeatherTool;
use async_trait::async_trait;

// Create a global tool registry
lazy_static! {
    static ref TOOL_REGISTRY: Arc<ToolRegistry> = {
        let registry = Arc::new(ToolRegistry::new());
        registry
    };
}

// Initialize the tool registry with available tools
async fn init_tool_registry() {
    let registry = TOOL_REGISTRY.clone();

    // Only initialize if the registry is empty
    if registry.is_empty().await {
        // Register the weather tool
        let weather_tool = WeatherTool::new();
        registry.register(weather_tool).await;

        // Register the search tool
        let search_tool = SearchTool::new();
        registry.register(search_tool).await;

        // Register the calculator tool
        let calculator_tool = CalculatorTool::new();
        registry.register(calculator_tool).await;

        // Update the tool parameters
        if let Some(mut tool) = registry.get_tool("search_web").await {
            tool.parameters = create_search_tool_parameters();
            registry.update_tool_parameters(&tool.name, tool.parameters).await;
        }

        if let Some(mut tool) = registry.get_tool("calculate").await {
            tool.parameters = create_calculator_tool_parameters();
            registry.update_tool_parameters(&tool.name, tool.parameters).await;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTool {
    pub id: String,
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiToolCall {
    pub id: String,
    pub tool_id: String,
    pub arguments: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiToolResult {
    pub id: String,
    pub tool_id: String,
    pub result: Value,
}

// Search tool implementation
pub struct SearchTool {
    name: String,
    description: String,
}

impl SearchTool {
    pub fn new() -> Self {
        Self {
            name: "search_web".to_string(),
            description: "Search the web for information".to_string(),
        }
    }
}

impl ToolExecutorBase for SearchTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn validate_args(&self, args: &Value) -> Result<(), ToolError> {
        if !args.get("query").is_some() {
            return Err(ToolError::MissingParameter("query".to_string()));
        }

        if let Some(query) = args.get("query") {
            if !query.is_string() || query.as_str().unwrap().trim().is_empty() {
                return Err(ToolError::InvalidArgument("query must be a non-empty string".to_string()));
            }
        }

        Ok(())
    }
}

#[async_trait]
impl ToolExecutor for SearchTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        let query = args["query"].as_str().unwrap();

        // In a real implementation, this would call a search API
        // For now, we'll return realistic but static results
        Ok(serde_json::json!({
            "query": query,
            "results": [
                {
                    "title": format!("Result 1 for '{}'", query),
                    "url": format!("https://example.com/search?q={}", query),
                    "snippet": format!("This is the first search result for '{}'. It contains relevant information about the query.", query)
                },
                {
                    "title": format!("Result 2 for '{}'", query),
                    "url": format!("https://example.org/search?q={}", query),
                    "snippet": format!("This is the second search result for '{}'. It provides additional context and details.", query)
                },
                {
                    "title": format!("Result 3 for '{}'", query),
                    "url": format!("https://search-results.com/search?q={}", query),
                    "snippet": format!("This is the third search result for '{}'. It offers a different perspective on the topic.", query)
                }
            ]
        }))
    }
}

// Calculator tool implementation
pub struct CalculatorTool {
    name: String,
    description: String,
}

impl CalculatorTool {
    pub fn new() -> Self {
        Self {
            name: "calculate".to_string(),
            description: "Perform a calculation".to_string(),
        }
    }

    fn evaluate_expression(&self, expression: &str) -> Result<f64, ToolError> {
        // This is a very simple calculator that only supports basic operations
        // In a real implementation, you would use a proper expression evaluator

        // For simplicity, we'll just parse and evaluate a simple expression
        let expression = expression.trim();

        // Handle simple addition
        if expression.contains('+') {
            let parts: Vec<&str> = expression.split('+').collect();
            if parts.len() != 2 {
                return Err(ToolError::InvalidArgument("Only simple expressions like 'a + b' are supported".to_string()));
            }

            let a = parts[0].trim().parse::<f64>().map_err(|_| {
                ToolError::InvalidArgument(format!("Could not parse '{}' as a number", parts[0]))
            })?;

            let b = parts[1].trim().parse::<f64>().map_err(|_| {
                ToolError::InvalidArgument(format!("Could not parse '{}' as a number", parts[1]))
            })?;

            return Ok(a + b);
        }

        // Handle simple subtraction
        if expression.contains('-') {
            let parts: Vec<&str> = expression.split('-').collect();
            if parts.len() != 2 {
                return Err(ToolError::InvalidArgument("Only simple expressions like 'a - b' are supported".to_string()));
            }

            let a = parts[0].trim().parse::<f64>().map_err(|_| {
                ToolError::InvalidArgument(format!("Could not parse '{}' as a number", parts[0]))
            })?;

            let b = parts[1].trim().parse::<f64>().map_err(|_| {
                ToolError::InvalidArgument(format!("Could not parse '{}' as a number", parts[1]))
            })?;

            return Ok(a - b);
        }

        // Handle simple multiplication
        if expression.contains('*') {
            let parts: Vec<&str> = expression.split('*').collect();
            if parts.len() != 2 {
                return Err(ToolError::InvalidArgument("Only simple expressions like 'a * b' are supported".to_string()));
            }

            let a = parts[0].trim().parse::<f64>().map_err(|_| {
                ToolError::InvalidArgument(format!("Could not parse '{}' as a number", parts[0]))
            })?;

            let b = parts[1].trim().parse::<f64>().map_err(|_| {
                ToolError::InvalidArgument(format!("Could not parse '{}' as a number", parts[1]))
            })?;

            return Ok(a * b);
        }

        // Handle simple division
        if expression.contains('/') {
            let parts: Vec<&str> = expression.split('/').collect();
            if parts.len() != 2 {
                return Err(ToolError::InvalidArgument("Only simple expressions like 'a / b' are supported".to_string()));
            }

            let a = parts[0].trim().parse::<f64>().map_err(|_| {
                ToolError::InvalidArgument(format!("Could not parse '{}' as a number", parts[0]))
            })?;

            let b = parts[1].trim().parse::<f64>().map_err(|_| {
                ToolError::InvalidArgument(format!("Could not parse '{}' as a number", parts[1]))
            })?;

            if b == 0.0 {
                return Err(ToolError::InvalidArgument("Division by zero is not allowed".to_string()));
            }

            return Ok(a / b);
        }

        // If no operation is found, try to parse as a single number
        expression.parse::<f64>().map_err(|_| {
            ToolError::InvalidArgument(format!("Could not parse '{}' as a number or expression", expression))
        })
    }
}

impl ToolExecutorBase for CalculatorTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn validate_args(&self, args: &Value) -> Result<(), ToolError> {
        if !args.get("expression").is_some() {
            return Err(ToolError::MissingParameter("expression".to_string()));
        }

        if let Some(expression) = args.get("expression") {
            if !expression.is_string() || expression.as_str().unwrap().trim().is_empty() {
                return Err(ToolError::InvalidArgument("expression must be a non-empty string".to_string()));
            }
        }

        Ok(())
    }
}

#[async_trait]
impl ToolExecutor for CalculatorTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        let expression = args["expression"].as_str().unwrap();

        let result = self.evaluate_expression(expression)?;

        Ok(serde_json::json!({
            "expression": expression,
            "result": result
        }))
    }
}

// Convert internal Tool to API Tool
fn convert_to_api_tool(tool: &crate::services::function_calling::types::Tool) -> ApiTool {
    // Create a JSON schema for the tool parameters
    let parameters = tool.to_json_schema();

    ApiTool {
        id: tool.name.clone(),
        name: tool.name.clone(),
        description: tool.description.clone(),
        parameters,
    }
}

// Create tool parameters for the search tool
fn create_search_tool_parameters() -> Vec<crate::services::function_calling::types::ToolParameter> {
    use crate::services::function_calling::types::{ToolParameter, ParameterType};

    vec![
        ToolParameter::new("query", ParameterType::String {
            format: None,
            enum_values: None
        }, true)
        .with_description("The search query string"),

        ToolParameter::new("num_results", ParameterType::Number {
            minimum: Some(1.0),
            maximum: Some(10.0)
        }, false)
        .with_description("The number of results to return (default: 3)"),
    ]
}

// Create tool parameters for the calculator tool
fn create_calculator_tool_parameters() -> Vec<crate::services::function_calling::types::ToolParameter> {
    use crate::services::function_calling::types::{ToolParameter, ParameterType};

    vec![
        ToolParameter::new("expression", ParameterType::String {
            format: None,
            enum_values: None
        }, true)
        .with_description("The mathematical expression to evaluate (e.g., '2 + 2', '10 * 5', etc.)"),
    ]
}

pub async fn list_tools(
    _pool: web::Data<DbPool>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Initialize the tool registry if needed
    init_tool_registry().await;

    // Get the list of tools from the registry
    let registry = TOOL_REGISTRY.clone();
    let tools = registry.list_tools().await;

    // Convert to API tools
    let api_tools: Vec<ApiTool> = tools.iter()
        .map(|tool| convert_to_api_tool(tool))
        .collect();

    Ok(HttpResponse::Ok().json(api_tools))
}

pub async fn get_tool(
    _pool: web::Data<DbPool>,
    tool_id: web::Path<String>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Initialize the tool registry if needed
    init_tool_registry().await;

    // Get the tool from the registry
    let registry = TOOL_REGISTRY.clone();
    let tool_id = tool_id.into_inner();

    let tool = registry.get_tool(&tool_id).await
        .ok_or_else(|| AppError::NotFoundError(format!("Tool not found: {}", tool_id)))?;

    // Convert to API tool
    let api_tool = convert_to_api_tool(&tool);

    Ok(HttpResponse::Ok().json(api_tool))
}

pub async fn execute_tool(
    _pool: web::Data<DbPool>,
    tool_call: web::Json<ApiToolCall>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Initialize the tool registry if needed
    init_tool_registry().await;

    let tool_call = tool_call.into_inner();
    let registry = TOOL_REGISTRY.clone();

    // Execute the tool
    let result = registry.execute(&tool_call.tool_id, tool_call.arguments.clone()).await
        .map_err(|e| {
            log::error!("Tool execution error: {}", e);
            AppError::BadRequest(format!("Tool execution failed: {}", e))
        })?;

    let tool_result = ApiToolResult {
        id: Uuid::new_v4().to_string(),
        tool_id: tool_call.tool_id,
        result,
    };

    Ok(HttpResponse::Ok().json(tool_result))
}