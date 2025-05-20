use async_trait::async_trait;
use serde_json::{json, Value};
use crate::services::function_calling::tool::executor::ToolExecutor;
use crate::services::function_calling::tool::error::ToolError;

/// A simple weather tool for demonstration purposes
pub struct WeatherTool {
    name: String,
    description: String,
}

impl WeatherTool {
    /// Creates a new weather tool
    pub fn new() -> Self {
        Self {
            name: "get_weather".to_string(),
            description: "Get the current weather for a location".to_string(),
        }
    }
    
    /// Validates the location parameter
    fn validate_location(&self, location: Option<&str>) -> Result<(), ToolError> {
        match location {
            Some(loc) if !loc.trim().is_empty() => Ok(()),
            _ => Err(ToolError::InvalidArgument("location must be a non-empty string".to_string())),
        }
    }
    
    /// Validates the units parameter
    fn validate_units(&self, units: Option<&str>) -> Result<(), ToolError> {
        match units {
            Some(u) if u == "celsius" || u == "fahrenheit" => Ok(()),
            Some(u) => Err(ToolError::InvalidArgument(format!(
                "units must be 'celsius' or 'fahrenheit', got '{}'", u
            ))),
            None => Ok(()),
        }
    }
}

#[async_trait]
impl ToolExecutor for WeatherTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        // Extract and validate parameters
        let location = args["location"].as_str();
        let units = args["units"].as_str().unwrap_or("celsius");
        
        self.validate_location(location)?;
        self.validate_units(Some(units))?;
        
        let location = location.unwrap();
        
        // In a real implementation, this would call a weather API
        // For demonstration, we'll return mock data
        let temperature = if units == "celsius" { 22.5 } else { 72.5 };
        
        Ok(json!({
            "location": location,
            "temperature": temperature,
            "units": units,
            "condition": "Partly Cloudy",
            "humidity": 65,
            "wind_speed": 10,
            "forecast": [
                {
                    "day": "Today",
                    "condition": "Partly Cloudy",
                    "high": temperature + 2.0,
                    "low": temperature - 5.0
                },
                {
                    "day": "Tomorrow",
                    "condition": "Sunny",
                    "high": temperature + 4.0,
                    "low": temperature - 3.0
                }
            ]
        }))
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn validate_args(&self, args: &Value) -> Result<(), ToolError> {
        // Check required parameters
        if !args.get("location").is_some() {
            return Err(ToolError::MissingParameter("location".to_string()));
        }
        
        // Validate parameter types and values
        self.validate_location(args["location"].as_str())?;
        
        if let Some(units) = args.get("units") {
            self.validate_units(units.as_str())?;
        }
        
        Ok(())
    }
}
