use crate::services::function_calling::types::Tool;

/// Available reasoning patterns for agents
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReasoningPattern {
    /// Plan tasks before acting
    Planning,
    /// Interleave reasoning and acting
    ReAct,
    /// Evaluate previous actions and self-correct
    Reflection,
    /// Formalized tool usage
    ToolUse,
}

/// Utility for composing multiple reasoning patterns together
#[derive(Debug, Clone)]
pub struct PatternComposer {
    pub patterns: Vec<ReasoningPattern>,
}

impl PatternComposer {
    /// Create a new composer from a list of patterns
    pub fn new(patterns: Vec<ReasoningPattern>) -> Self {
        Self { patterns }
    }

    /// Build a system prompt describing the combined patterns
    pub fn system_prompt(
        &self,
        agent_name: &str,
        agent_description: &str,
        tools: &[Tool],
    ) -> String {
        let mut sections = Vec::new();
        for pattern in &self.patterns {
            let text = match pattern {
                ReasoningPattern::Planning => PLANNING_TEXT,
                ReasoningPattern::ReAct => REACT_TEXT,
                ReasoningPattern::Reflection => REFLECTION_TEXT,
                ReasoningPattern::ToolUse => TOOLUSE_TEXT,
            };
            sections.push(text);
        }
        let tools_formatted = format_tools_for_prompt(tools);
        format!(
            "You are {}, {}.\n{}\nAvailable tools:\n{}",
            agent_name,
            agent_description,
            sections.join("\n"),
            tools_formatted
        )
    }
}

/// Format a list of tools for inclusion in a system prompt
pub fn format_tools_for_prompt(tools: &[Tool]) -> String {
    tools
        .iter()
        .map(|t| format!("- {}: {}", t.name, t.description))
        .collect::<Vec<_>>()
        .join("\n")
}

const REACT_TEXT: &str = "Follow these steps for each user request:\n1. Think about what information you need\n2. Use the appropriate tool to gather that information\n3. Analyze the results\n4. Use additional tools if needed\n5. Provide a final answer";

const PLANNING_TEXT: &str = "Follow these steps for each user request:\n1. Analyze what the user is asking for\n2. Create a detailed plan with specific steps\n3. For each step, use the appropriate tool\n4. If a step fails, adjust your plan\n5. Provide a final answer that addresses the user's request";

const REFLECTION_TEXT: &str = "Follow these steps for each user request:\n1. Generate an initial response using tools as needed\n2. Evaluate your response: Is it complete? Accurate? Helpful?\n3. If the response needs improvement, use additional tools\n4. Provide your final, improved response";

const TOOLUSE_TEXT: &str = "When invoking tools ensure you supply the required parameters and handle any errors gracefully.";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::function_calling::{Tool, ToolParameter, ParameterType};

    #[test]
    fn compose_prompts() {
        let tools = vec![Tool::new(
            "demo",
            "A demo tool",
            vec![ToolParameter::new(
                "id",
                ParameterType::String { format: None, enum_values: None },
                true,
            )],
        )];
        let composer = PatternComposer::new(vec![ReasoningPattern::Planning, ReasoningPattern::ReAct]);
        let prompt = composer.system_prompt("Tester", "does tests", &tools);
        assert!(prompt.contains("detailed plan"));
        assert!(prompt.contains("appropriate tool to gather"));
        assert!(prompt.contains("demo tool"));
    }
}

