use fluent_web_services::services::{PatternComposer, ReasoningPattern};
use fluent_web_services::services::function_calling::{Tool, ToolParameter, ParameterType};

#[test]
fn system_prompt_combines_patterns() {
    let tools = vec![Tool::new(
        "demo",
        "Demo tool",
        vec![ToolParameter::new(
            "id",
            ParameterType::String { format: None, enum_values: None },
            true,
        )],
    )];
    let composer = PatternComposer::new(vec![ReasoningPattern::Planning, ReasoningPattern::ReAct]);
    let prompt = composer.system_prompt("Agent", "testing", &tools);
    assert!(prompt.contains("detailed plan"));
    assert!(prompt.contains("appropriate tool to gather"));
    assert!(prompt.contains("Demo tool"));
}
