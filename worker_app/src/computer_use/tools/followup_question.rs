//! Tool for interactive user communication.
//!
//! This module provides functionality to ask follow-up questions to users
//! during tool execution. It's particularly useful when:
//! - Additional information is needed
//! - Clarification is required
//! - Confirmation is needed for potentially destructive operations
//! - User preferences need to be gathered

use crate::computer_use::base::{ToolExecution, ToolInfo, ToolResult};
use async_trait::async_trait;
use serde_json::Value;
use std::any::Any;
use std::error::Error;

/// Tool for asking follow-up questions to users.
///
/// This tool enables interactive communication during tool execution
/// by providing a structured way to ask questions and handle responses.
///
/// # Use Cases
///
/// - Gathering missing information
/// - Confirming destructive operations
/// - Clarifying ambiguous inputs
/// - Collecting user preferences
///
/// # Example Scenarios
///
/// ```text
/// - "What directory should I create the file in?"
/// - "Should I overwrite the existing file?"
/// - "Which format would you prefer: JSON or YAML?"
/// - "Please provide your preferred configuration settings."
/// ```
#[derive(Debug)]
pub struct FollowupQuestion;

impl ToolInfo for FollowupQuestion {
    fn name(&self) -> &'static str {
        "followup_question"
    }

    fn description(&self) -> &'static str {
        "Ask a follow-up question to gather additional information"
    }

    fn parameters(&self) -> Vec<(&'static str, bool)> {
        vec![("question", true)]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait]
impl ToolExecution for FollowupQuestion {
    /// Execute a follow-up question.
    ///
    /// This method formats and presents a question to the user,
    /// preparing it for the LLM to handle the response appropriately.
    ///
    /// # Arguments
    ///
    /// * `params` - JSON object containing:
    ///   - `question`: The question to ask (required)
    ///
    /// # Returns
    ///
    /// Returns a `ToolResult` containing:
    /// - The formatted question
    /// - Success/failure status
    /// - Any error messages
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use serde_json::json;
    /// # use crate::computer_use::base::ToolExecution;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let tool = FollowupQuestion;
    /// let result = tool.execute(json!({
    ///     "question": "What directory should I create the file in?"
    /// })).await?;
    ///
    /// println!("Question asked: {}", result.output);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Best Practices
    ///
    /// When using this tool:
    /// 1. Ask clear, specific questions
    /// 2. Provide context when necessary
    /// 3. Indicate the type of response expected
    /// 4. Handle the response appropriately
    async fn execute(&self, params: Value) -> Result<ToolResult, Box<dyn Error + Send + Sync>> {
        self.validate_params(&params)?;

        let question = params["question"].as_str().unwrap();

        Ok(ToolResult {
            success: true,
            output: format!("Question asked: {}", question),
            error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_followup_question() {
        let tool = FollowupQuestion;
        let result = tool
            .execute(json!({
                "question": "What is your preferred programming language?"
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(result
            .output
            .contains("What is your preferred programming language?"));
    }

    #[tokio::test]
    async fn test_missing_question() {
        let tool = FollowupQuestion;
        let result = tool.validate_params(&json!({}));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("question"));
    }

    #[tokio::test]
    async fn test_empty_question() {
        let tool = FollowupQuestion;
        let result = tool
            .execute(json!({
                "question": ""
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.output.contains("Question asked:"));
    }

    #[tokio::test]
    async fn test_special_characters() {
        let tool = FollowupQuestion;
        let question = "How would you like to handle special characters: @#$%^&*?";
        let result = tool
            .execute(json!({
                "question": question
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.output.contains(question));
    }
}
