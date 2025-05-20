use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

/// Represents a tool that can be called by an LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}

/// Represents a parameter for a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameter_type: ParameterType,
    pub required: bool,
}

/// Represents the type of a parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParameterType {
    #[serde(rename = "string")]
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        enum_values: Option<Vec<String>>,
    },
    #[serde(rename = "number")]
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maximum: Option<f64>,
    },
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "object")]
    Object {
        properties: Vec<ToolParameter>,
    },
    #[serde(rename = "array")]
    Array {
        items: Box<ParameterType>,
    },
}

/// Represents a tool call from an LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

/// Represents the result of a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_call_id: String,
    pub result: Value,
}

/// Represents a message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "role")]
pub enum Message {
    #[serde(rename = "user")]
    User { content: String },
    #[serde(rename = "assistant")]
    Assistant { content: String },
    #[serde(rename = "system")]
    System { content: String },
    #[serde(rename = "tool")]
    ToolResult { tool_call_id: String, content: Value },
}

impl Tool {
    /// Creates a new tool
    pub fn new(name: &str, description: &str, parameters: Vec<ToolParameter>) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            parameters,
        }
    }
    
    /// Converts the tool to a JSON schema
    pub fn to_json_schema(&self) -> Value {
        let properties = self.parameters.iter().map(|param| {
            (param.name.clone(), param.to_json_schema())
        }).collect::<HashMap<_, _>>();
        
        let required = self.parameters.iter()
            .filter(|p| p.required)
            .map(|p| p.name.clone())
            .collect::<Vec<_>>();
            
        serde_json::json!({
            "type": "object",
            "properties": properties,
            "required": required,
        })
    }
}

impl ToolParameter {
    /// Creates a new tool parameter
    pub fn new(name: &str, parameter_type: ParameterType, required: bool) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            parameter_type,
            required,
        }
    }
    
    /// Adds a description to the parameter
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    
    /// Converts the parameter to a JSON schema
    fn to_json_schema(&self) -> Value {
        match &self.parameter_type {
            ParameterType::String { format, enum_values } => {
                let mut schema = serde_json::json!({
                    "type": "string",
                });
                
                if let Some(format) = format {
                    schema["format"] = serde_json::Value::String(format.clone());
                }
                
                if let Some(enum_values) = enum_values {
                    schema["enum"] = serde_json::json!(enum_values);
                }
                
                schema
            },
            ParameterType::Number { minimum, maximum } => {
                let mut schema = serde_json::json!({
                    "type": "number",
                });
                
                if let Some(minimum) = minimum {
                    schema["minimum"] = serde_json::json!(minimum);
                }
                
                if let Some(maximum) = maximum {
                    schema["maximum"] = serde_json::json!(maximum);
                }
                
                schema
            },
            ParameterType::Boolean => {
                serde_json::json!({
                    "type": "boolean",
                })
            },
            ParameterType::Object { properties } => {
                let obj_properties = properties.iter().map(|param| {
                    (param.name.clone(), param.to_json_schema())
                }).collect::<HashMap<_, _>>();
                
                let required = properties.iter()
                    .filter(|p| p.required)
                    .map(|p| p.name.clone())
                    .collect::<Vec<_>>();
                    
                serde_json::json!({
                    "type": "object",
                    "properties": obj_properties,
                    "required": required,
                })
            },
            ParameterType::Array { items } => {
                let item_schema = match items.as_ref() {
                    param_type => {
                        let dummy_param = ToolParameter {
                            name: "item".to_string(),
                            description: None,
                            parameter_type: param_type.clone(),
                            required: false,
                        };
                        dummy_param.to_json_schema()
                    }
                };
                
                serde_json::json!({
                    "type": "array",
                    "items": item_schema,
                })
            },
        }
    }
}

impl Message {
    /// Creates a new user message
    pub fn user(content: &str) -> Self {
        Self::User { content: content.to_string() }
    }
    
    /// Creates a new assistant message
    pub fn assistant(content: &str) -> Self {
        Self::Assistant { content: content.to_string() }
    }
    
    /// Creates a new system message
    pub fn system(content: &str) -> Self {
        Self::System { content: content.to_string() }
    }
    
    /// Creates a new tool result message
    pub fn tool_result(tool_call_id: &str, content: &Value) -> Self {
        Self::ToolResult { 
            tool_call_id: tool_call_id.to_string(), 
            content: content.clone() 
        }
    }
}
