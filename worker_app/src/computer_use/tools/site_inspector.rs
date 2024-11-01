//! Tool for website inspection and analysis.
//!
//! This module provides functionality to:
//! - Navigate to web pages
//! - Capture screenshots
//! - Collect console logs
//! - Monitor page state
//!
//! It uses headless Chrome via chromiumoxide for browser automation.

use crate::computer_use::base::{ToolExecution, ToolInfo, ToolResult};
use async_trait::async_trait;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::target::CreateTargetParams;
use chromiumoxide::page::ScreenshotParams;
use futures::StreamExt;
use serde_json::Value;
use std::any::Any;
use std::error::Error;

/// Tool for inspecting websites and capturing their state.
///
/// This tool provides capabilities to:
/// - Load web pages
/// - Capture full-page screenshots
/// - Monitor console output
/// - Track page load state
#[derive(Debug)]
pub struct SiteInspector;

impl ToolInfo for SiteInspector {
    fn name(&self) -> &'static str {
        "site_inspector"
    }

    fn description(&self) -> &'static str {
        "Inspect websites by capturing screenshots and console logs"
    }

    fn parameters(&self) -> Vec<(&'static str, bool)> {
        vec![("url", true)]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait]
impl ToolExecution for SiteInspector {
    /// Execute website inspection.
    ///
    /// This method:
    /// 1. Launches a headless browser
    /// 2. Navigates to the specified URL
    /// 3. Captures a screenshot
    /// 4. Collects console logs
    /// 5. Returns the inspection results
    ///
    /// # Arguments
    ///
    /// * `params` - JSON object containing:
    ///   - `url`: The URL to inspect (required)
    ///
    /// # Returns
    ///
    /// Returns a `ToolResult` containing:
    /// - Screenshot path
    /// - Console logs
    /// - Any errors encountered
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use serde_json::json;
    /// # use crate::computer_use::base::ToolExecution;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let inspector = SiteInspector;
    /// let result = inspector.execute(json!({
    ///     "url": "https://example.com"
    /// })).await?;
    ///
    /// println!("Inspection results: {}", result.output);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The browser fails to launch
    /// - Navigation fails
    /// - Screenshot capture fails
    /// - File system operations fail
    async fn execute(&self, params: Value) -> Result<ToolResult, Box<dyn Error + Send + Sync>> {
        self.validate_params(&params)?;

        let url = params["url"].as_str().unwrap();

        // Configure and launch browser with reasonable defaults
        let (mut browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .no_sandbox()
                .window_size(1920, 1080)
                .build()?,
        )
        .await?;

        // Spawn browser handler to manage browser events
        tokio::spawn(async move {
            while let Some(h) = handler.next().await {
                if h.is_err() {
                    break;
                }
            }
        });

        // Create new page with explicit params
        let target_params = CreateTargetParams::new("about:blank");
        let page = browser.new_page(target_params).await?;

        // Navigate to URL and wait for load
        page.goto(url).await?;
        page.wait_for_navigation().await?;

        // Capture full page screenshot
        let screenshot = page.screenshot(ScreenshotParams::default()).await?;

        // Save screenshot to temporary location
        let temp_dir = std::env::temp_dir();
        let screenshot_path = temp_dir.join("screenshot.png");
        std::fs::write(&screenshot_path, &screenshot)?;

        // Collect console logs using JavaScript injection
        let js_result = page
            .evaluate(
                r#"
            (function() {
                return window.consoleMessages || ['No console messages captured'];
            })()
        "#,
            )
            .await?;

        // Parse console logs from JavaScript result
        let console_logs = match js_result.value() {
            Some(value) => match value {
                Value::String(s) => s.to_string(),
                Value::Array(arr) => arr
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
                _ => "No console logs available".to_string(),
            },
            None => "No console logs available".to_string(),
        };

        // Format inspection results
        let result = format!(
            "Site Inspection Results:\n\n\
            URL: {}\n\
            Screenshot saved to: {}\n\n\
            Console Logs:\n{}",
            url,
            screenshot_path.display(),
            console_logs
        );

        // Clean up browser instance
        browser.close().await?;

        Ok(ToolResult {
            success: true,
            output: result,
            error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_site_inspection() {
        let tool = SiteInspector;
        let result = tool
            .execute(json!({
                "url": "https://example.com"
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.output.contains("Site Inspection Results"));
        assert!(result.output.contains("https://example.com"));
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let tool = SiteInspector;
        let result = tool
            .execute(json!({
                "url": "invalid-url"
            }))
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_parameter_validation() {
        let tool = SiteInspector;
        let result = tool.validate_params(&json!({}));
        assert!(result.is_err());
    }
}
