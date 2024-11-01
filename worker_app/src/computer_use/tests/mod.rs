//! Integration tests for computer use functionality.
//!
//! These tests verify that all tools work together correctly and
//! handle real-world scenarios appropriately.

use crate::computer_use::{Computer, ToolResult};
use serde_json::json;
use std::fs;
use tempfile::TempDir;

/// Test scenario combining file operations and command execution.
#[tokio::test]
async fn test_file_and_command_integration() {
    let computer = Computer::new();
    let temp_dir = TempDir::new().unwrap();
    let script_path = temp_dir.path().join("test_script.sh");
    let output_path = temp_dir.path().join("output.txt");

    // Create a shell script using file operations
    let script_result = computer
        .execute_tool(
            "file_operations",
            json!({
                "operation": "write",
                "path": script_path.to_str().unwrap(),
                "content": "#!/bin/sh\necho 'Hello from script' > output.txt"
            }),
        )
        .await
        .unwrap();
    assert!(script_result.success);

    // Make script executable
    let chmod_result = computer
        .execute_tool(
            "execute_command",
            json!({
                "command": format!("chmod +x {}", script_path.to_str().unwrap())
            }),
        )
        .await
        .unwrap();
    assert!(chmod_result.success);

    // Execute the script
    let exec_result = computer
        .execute_tool(
            "execute_command",
            json!({
                "command": format!("cd {} && ./test_script.sh", temp_dir.path().to_str().unwrap())
            }),
        )
        .await
        .unwrap();
    assert!(exec_result.success);

    // Verify output using file operations
    let read_result = computer
        .execute_tool(
            "file_operations",
            json!({
                "operation": "read",
                "path": output_path.to_str().unwrap()
            }),
        )
        .await
        .unwrap();
    assert!(read_result.success);
    assert!(read_result.output.contains("Hello from script"));
}

/// Test scenario combining website inspection and file operations.
#[tokio::test]
async fn test_site_inspection_and_file_operations() {
    let computer = Computer::new();
    let temp_dir = TempDir::new().unwrap();
    let html_path = temp_dir.path().join("test.html");

    // Create a test HTML file
    let html_content = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Test Page</title></head>
        <body>
            <h1>Hello, World!</h1>
            <script>
                console.log('Test log message');
            </script>
        </body>
        </html>
    "#;

    let write_result = computer
        .execute_tool(
            "file_operations",
            json!({
                "operation": "write",
                "path": html_path.to_str().unwrap(),
                "content": html_content
            }),
        )
        .await
        .unwrap();
    assert!(write_result.success);

    // Inspect the local HTML file
    let inspect_result = computer
        .execute_tool(
            "site_inspector",
            json!({
                "url": format!("file://{}", html_path.to_str().unwrap())
            }),
        )
        .await
        .unwrap();
    assert!(inspect_result.success);
    assert!(inspect_result.output.contains("Test Page"));
}

/// Test scenario with user interaction and file operations.
#[tokio::test]
async fn test_user_interaction_and_file_operations() {
    let computer = Computer::new();
    let temp_dir = TempDir::new().unwrap();

    // Ask for file path
    let question_result = computer
        .execute_tool(
            "followup_question",
            json!({
                "question": "Where should I create the file?"
            }),
        )
        .await
        .unwrap();
    assert!(question_result.success);

    // Simulate user response by creating file
    let file_path = temp_dir.path().join("user_file.txt");
    let write_result = computer
        .execute_tool(
            "file_operations",
            json!({
                "operation": "write",
                "path": file_path.to_str().unwrap(),
                "content": "User-specified content"
            }),
        )
        .await
        .unwrap();
    assert!(write_result.success);

    // Verify file exists
    assert!(file_path.exists());
}

/// Test error handling and recovery across tools.
#[tokio::test]
async fn test_error_handling_and_recovery() {
    let computer = Computer::new();
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_file = temp_dir.path().join("nonexistent.txt");

    // Try to read nonexistent file
    let read_result = computer
        .execute_tool(
            "file_operations",
            json!({
                "operation": "read",
                "path": nonexistent_file.to_str().unwrap()
            }),
        )
        .await
        .unwrap();
    assert!(!read_result.success);
    assert!(read_result.error.is_some());

    // Ask user what to do about the error
    let question_result = computer
        .execute_tool(
            "followup_question",
            json!({
                "question": "File not found. Would you like to create it?"
            }),
        )
        .await
        .unwrap();
    assert!(question_result.success);

    // Create the file
    let write_result = computer
        .execute_tool(
            "file_operations",
            json!({
                "operation": "write",
                "path": nonexistent_file.to_str().unwrap(),
                "content": "Recovery content"
            }),
        )
        .await
        .unwrap();
    assert!(write_result.success);

    // Verify recovery
    assert!(nonexistent_file.exists());
}

/// Test concurrent tool execution.
#[tokio::test]
async fn test_concurrent_execution() {
    let computer = Computer::new();
    let temp_dir = TempDir::new().unwrap();

    // Create multiple files concurrently
    let mut handles = vec![];
    for i in 0..5 {
        let computer = computer.clone();
        let dir = temp_dir.path().to_owned();
        handles.push(tokio::spawn(async move {
            let file_path = dir.join(format!("file_{}.txt", i));
            computer
                .execute_tool(
                    "file_operations",
                    json!({
                        "operation": "write",
                        "path": file_path.to_str().unwrap(),
                        "content": format!("Content {}", i)
                    }),
                )
                .await
                .unwrap()
        }));
    }

    // Wait for all operations to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.success);
    }

    // Verify all files were created
    for i in 0..5 {
        assert!(temp_dir.path().join(format!("file_{}.txt", i)).exists());
    }
}

/// Test tool chain execution.
#[tokio::test]
async fn test_tool_chain() {
    let computer = Computer::new();
    let temp_dir = TempDir::new().unwrap();
    let data_file = temp_dir.path().join("data.txt");
    let processed_file = temp_dir.path().join("processed.txt");

    // Chain of operations:
    // 1. Create data file
    // 2. Process with command
    // 3. Save results
    // 4. Verify

    // Step 1: Create data file
    let write_result = computer
        .execute_tool(
            "file_operations",
            json!({
                "operation": "write",
                "path": data_file.to_str().unwrap(),
                "content": "line1\nline2\nline3"
            }),
        )
        .await
        .unwrap();
    assert!(write_result.success);

    // Step 2: Process with command (count lines)
    let process_result = computer
        .execute_tool(
            "execute_command",
            json!({
                "command": format!("wc -l < {}", data_file.to_str().unwrap())
            }),
        )
        .await
        .unwrap();
    assert!(process_result.success);

    // Step 3: Save results
    let save_result = computer
        .execute_tool(
            "file_operations",
            json!({
                "operation": "write",
                "path": processed_file.to_str().unwrap(),
                "content": format!("Line count: {}", process_result.output.trim())
            }),
        )
        .await
        .unwrap();
    assert!(save_result.success);

    // Step 4: Verify results
    let verify_result = computer
        .execute_tool(
            "file_operations",
            json!({
                "operation": "read",
                "path": processed_file.to_str().unwrap()
            }),
        )
        .await
        .unwrap();
    assert!(verify_result.success);
    assert!(verify_result.output.contains("Line count: 3"));
}
