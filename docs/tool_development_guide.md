# Tool Development Guide

This guide explains how to develop tools for our generic function calling system, including best practices and examples.

## What is a Tool?

A tool is a function that an AI agent can call to interact with external systems, retrieve information, or perform actions. Tools have:

1. A name and description
2. A set of parameters with types and descriptions
3. An implementation that executes the tool's functionality

## Tool Definition

Tools are defined using the `Tool` struct:

```rust
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}

pub struct ToolParameter {
    pub name: String,
    pub description: Option<String>,
    pub parameter_type: ParameterType,
    pub required: bool,
}

pub enum ParameterType {
    String,
    Number,
    Boolean,
    Object(Vec<ToolParameter>),
    Array(Box<ParameterType>),
}
```

Example tool definition:

```rust
let weather_tool = Tool {
    name: "get_weather".to_string(),
    description: "Get current weather for a location".to_string(),
    parameters: vec![
        ToolParameter {
            name: "location".to_string(),
            description: Some("City and country, e.g., 'Paris, France'".to_string()),
            parameter_type: ParameterType::String,
            required: true,
        },
        ToolParameter {
            name: "units".to_string(),
            description: Some("Temperature units: 'celsius' or 'fahrenheit'".to_string()),
            parameter_type: ParameterType::String,
            required: false,
        },
    ],
};
```

## Tool Executor Implementation

Tools are implemented by creating a struct that implements the `ToolExecutor` trait:

```rust
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn execute(&self, args: Value) -> Result<Value, ToolError>;
}
```

Example implementation:

```rust
pub struct WeatherTool {
    api_key: String,
    client: reqwest::Client,
}

impl WeatherTool {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl ToolExecutor for WeatherTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        // Extract and validate parameters
        let location = args["location"].as_str()
            .ok_or_else(|| ToolError::InvalidArgument("location must be a string".to_string()))?;
            
        let units = args["units"].as_str().unwrap_or("celsius");
        if units != "celsius" && units != "fahrenheit" {
            return Err(ToolError::InvalidArgument(
                "units must be 'celsius' or 'fahrenheit'".to_string()
            ));
        }
        
        // Call external API
        let url = format!(
            "https://api.weatherapi.com/v1/current.json?key={}&q={}&units={}",
            self.api_key, location, units
        );
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| ToolError::ExternalServiceError(e.to_string()))?;
            
        if !response.status().is_success() {
            return Err(ToolError::ExternalServiceError(format!(
                "Weather API returned status code: {}",
                response.status()
            )));
        }
        
        // Parse and return response
        let weather_data = response.json::<Value>()
            .await
            .map_err(|e| ToolError::DeserializationError(e.to_string()))?;
            
        // Transform to a simplified format
        let result = json!({
            "location": weather_data["location"]["name"],
            "country": weather_data["location"]["country"],
            "temperature": weather_data["current"]["temp_c"],
            "condition": weather_data["current"]["condition"]["text"],
            "humidity": weather_data["current"]["humidity"],
            "wind_speed": weather_data["current"]["wind_kph"],
        });
        
        Ok(result)
    }
}
```

## Tool Categories

### 1. Information Retrieval Tools

These tools fetch information from external sources:

```rust
pub struct WebSearchTool {
    api_key: String,
    client: reqwest::Client,
}

#[async_trait]
impl ToolExecutor for WebSearchTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        let query = args["query"].as_str()
            .ok_or_else(|| ToolError::InvalidArgument("query must be a string".to_string()))?;
        let num_results = args["num_results"].as_u64().unwrap_or(5) as usize;
        
        // Call search API
        // ...
        
        Ok(search_results)
    }
}
```

### 2. Data Processing Tools

These tools transform or analyze data:

```rust
pub struct TextAnalyzerTool;

#[async_trait]
impl ToolExecutor for TextAnalyzerTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        let text = args["text"].as_str()
            .ok_or_else(|| ToolError::InvalidArgument("text must be a string".to_string()))?;
            
        // Analyze text
        let word_count = text.split_whitespace().count();
        let sentence_count = text.split(['.', '!', '?']).filter(|s| !s.trim().is_empty()).count();
        let avg_word_length = text.split_whitespace()
            .map(|word| word.len())
            .sum::<usize>() as f64 / word_count as f64;
            
        Ok(json!({
            "word_count": word_count,
            "sentence_count": sentence_count,
            "average_word_length": avg_word_length,
        }))
    }
}
```

### 3. Action Tools

These tools perform actions in external systems:

```rust
pub struct EmailSenderTool {
    smtp_client: SmtpClient,
}

#[async_trait]
impl ToolExecutor for EmailSenderTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        let to = args["to"].as_str()
            .ok_or_else(|| ToolError::InvalidArgument("to must be a string".to_string()))?;
        let subject = args["subject"].as_str()
            .ok_or_else(|| ToolError::InvalidArgument("subject must be a string".to_string()))?;
        let body = args["body"].as_str()
            .ok_or_else(|| ToolError::InvalidArgument("body must be a string".to_string()))?;
            
        // Send email
        // ...
        
        Ok(json!({
            "status": "sent",
            "message_id": message_id,
            "timestamp": timestamp,
        }))
    }
}
```

## Best Practices

### 1. Parameter Validation

Always validate parameters before using them:

```rust
fn validate_parameters(args: &Value) -> Result<(), ToolError> {
    // Check required parameters
    if !args.get("location").is_some() {
        return Err(ToolError::MissingParameter("location".to_string()));
    }
    
    // Validate parameter types
    if let Some(location) = args.get("location") {
        if !location.is_string() {
            return Err(ToolError::InvalidArgument("location must be a string".to_string()));
        }
    }
    
    // Validate parameter values
    if let Some(units) = args.get("units") {
        if let Some(units_str) = units.as_str() {
            if units_str != "celsius" && units_str != "fahrenheit" {
                return Err(ToolError::InvalidArgument(
                    "units must be 'celsius' or 'fahrenheit'".to_string()
                ));
            }
        }
    }
    
    Ok(())
}
```

### 2. Error Handling

Implement comprehensive error handling:

```rust
pub enum ToolError {
    MissingParameter(String),
    InvalidArgument(String),
    ExternalServiceError(String),
    DeserializationError(String),
    AuthenticationError(String),
    RateLimitExceeded(String),
    PermissionDenied(String),
    InternalError(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingParameter(param) => write!(f, "Missing required parameter: {}", param),
            Self::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            Self::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            Self::DeserializationError(msg) => write!(f, "Failed to deserialize response: {}", msg),
            Self::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            Self::RateLimitExceeded(msg) => write!(f, "Rate limit exceeded: {}", msg),
            Self::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}
```

### 3. Timeouts and Retries

Implement timeouts and retries for external API calls:

```rust
async fn call_with_retry<F, Fut, T, E>(
    f: F,
    max_retries: usize,
    initial_delay: Duration,
) -> Result<T, ToolError>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut delay = initial_delay;
    let mut attempts = 0;
    
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempts += 1;
                if attempts >= max_retries {
                    return Err(ToolError::ExternalServiceError(format!(
                        "Failed after {} attempts: {}", max_retries, e
                    )));
                }
                
                tokio::time::sleep(delay).await;
                delay *= 2; // Exponential backoff
            }
        }
    }
}
```

### 4. Caching

Implement caching for frequently used data:

```rust
pub struct CachedWeatherTool {
    inner: WeatherTool,
    cache: Arc<Mutex<LruCache<String, (Value, Instant)>>>,
    ttl: Duration,
}

impl CachedWeatherTool {
    pub fn new(api_key: &str, cache_size: usize, ttl: Duration) -> Self {
        Self {
            inner: WeatherTool::new(api_key),
            cache: Arc::new(Mutex::new(LruCache::new(cache_size))),
            ttl,
        }
    }
}

#[async_trait]
impl ToolExecutor for CachedWeatherTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        let location = args["location"].as_str()
            .ok_or_else(|| ToolError::InvalidArgument("location must be a string".to_string()))?;
        let units = args["units"].as_str().unwrap_or("celsius");
        
        let cache_key = format!("{}:{}", location, units);
        
        // Check cache
        {
            let mut cache = self.cache.lock().await;
            if let Some((result, timestamp)) = cache.get(&cache_key) {
                if timestamp.elapsed() < self.ttl {
                    return Ok(result.clone());
                }
            }
        }
        
        // Call inner tool
        let result = self.inner.execute(args).await?;
        
        // Update cache
        {
            let mut cache = self.cache.lock().await;
            cache.put(cache_key, (result.clone(), Instant::now()));
        }
        
        Ok(result)
    }
}
```

### 5. Logging

Implement logging for debugging and monitoring:

```rust
#[async_trait]
impl ToolExecutor for LoggingToolExecutor<T> {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        log::info!("Executing tool: {}", self.inner.name());
        log::debug!("Tool arguments: {}", args);
        
        let start = Instant::now();
        let result = self.inner.execute(args).await;
        let duration = start.elapsed();
        
        match &result {
            Ok(value) => {
                log::info!("Tool execution successful, took {:?}", duration);
                log::debug!("Tool result: {}", value);
            }
            Err(e) => {
                log::error!("Tool execution failed after {:?}: {}", duration, e);
            }
        }
        
        result
    }
}
```

## Testing Tools

### 1. Unit Testing

```rust
#[tokio::test]
async fn test_weather_tool() {
    // Create a mock server
    let mock_server = MockServer::start().await;
    
    // Configure the mock
    Mock::given(method("GET"))
        .and(path_regex(r"/v1/current.json"))
        .and(query_param("q", "London"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "location": {
                    "name": "London",
                    "country": "UK"
                },
                "current": {
                    "temp_c": 15.5,
                    "condition": {
                        "text": "Partly cloudy"
                    },
                    "humidity": 65,
                    "wind_kph": 10.5
                }
            }))
        )
        .mount(&mock_server)
        .await;
    
    // Create the tool with the mock server URL
    let weather_tool = WeatherTool::new_with_base_url("fake_api_key", &mock_server.uri());
    
    // Execute the tool
    let args = json!({
        "location": "London",
        "units": "celsius"
    });
    
    let result = weather_tool.execute(args).await.unwrap();
    
    // Verify the result
    assert_eq!(result["location"], "London");
    assert_eq!(result["country"], "UK");
    assert_eq!(result["temperature"], 15.5);
    assert_eq!(result["condition"], "Partly cloudy");
    assert_eq!(result["humidity"], 65);
    assert_eq!(result["wind_kph"], 10.5);
}
```

### 2. Integration Testing

```rust
#[tokio::test]
async fn test_weather_tool_integration() {
    // Use a real API key from environment
    let api_key = std::env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY must be set");
    
    // Create the tool
    let weather_tool = WeatherTool::new(&api_key);
    
    // Execute the tool
    let args = json!({
        "location": "London",
        "units": "celsius"
    });
    
    let result = weather_tool.execute(args).await.unwrap();
    
    // Verify the result structure
    assert!(result.get("location").is_some());
    assert!(result.get("country").is_some());
    assert!(result.get("temperature").is_some());
    assert!(result.get("condition").is_some());
    assert!(result.get("humidity").is_some());
    assert!(result.get("wind_speed").is_some());
}
```

## Example: Complete Tool Implementation

```rust
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

pub struct StockPriceTool {
    api_key: String,
    client: Client,
}

impl StockPriceTool {
    pub fn new(api_key: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
            
        Self {
            api_key: api_key.to_string(),
            client,
        }
    }
    
    pub fn definition() -> Tool {
        Tool {
            name: "get_stock_price".to_string(),
            description: "Get the current stock price for a given symbol".to_string(),
            parameters: vec![
                ToolParameter {
                    name: "symbol".to_string(),
                    description: Some("Stock symbol, e.g., 'AAPL' for Apple".to_string()),
                    parameter_type: ParameterType::String,
                    required: true,
                },
            ],
        }
    }
}

#[async_trait]
impl ToolExecutor for StockPriceTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        // Extract and validate parameters
        let symbol = match args.get("symbol") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            Some(_) => return Err(ToolError::InvalidArgument("symbol must be a string".to_string())),
            None => return Err(ToolError::MissingParameter("symbol".to_string())),
        };
        
        // Call external API with retry
        let url = format!(
            "https://api.marketdata.com/v1/stocks/quote?symbol={}&apikey={}",
            symbol, self.api_key
        );
        
        let response = call_with_retry(
            || async {
                self.client.get(&url)
                    .send()
                    .await
                    .map_err(|e| e.to_string())
            },
            3,
            Duration::from_millis(500),
        ).await?;
        
        if !response.status().is_success() {
            return Err(ToolError::ExternalServiceError(format!(
                "Stock API returned status code: {}",
                response.status()
            )));
        }
        
        // Parse and return response
        let stock_data = response.json::<Value>()
            .await
            .map_err(|e| ToolError::DeserializationError(e.to_string()))?;
            
        // Transform to a simplified format
        let result = json!({
            "symbol": stock_data["symbol"],
            "company_name": stock_data["companyName"],
            "price": stock_data["latestPrice"],
            "change": stock_data["change"],
            "change_percent": stock_data["changePercent"],
            "market_cap": stock_data["marketCap"],
            "pe_ratio": stock_data["peRatio"],
            "timestamp": stock_data["latestUpdate"],
        });
        
        Ok(result)
    }
}
```

## Next Steps

1. Implement the core tool interfaces and traits
2. Create a library of common tools
3. Develop testing utilities for tools
4. Add monitoring and analytics
5. Create documentation for each tool
