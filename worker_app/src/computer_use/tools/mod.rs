//! Tool implementations for computer use functionality.
//!
//! This module provides implementations of various tools that can be executed
//! by the computer system. Each tool implements the `ToolInfo` and `ToolExecution`
//! traits and provides specific functionality.
//!
//! # Available Tools
//!
//! - `ExecuteCommand`: Execute system commands
//! - `FileOperations`: File system operations
//! - `SiteInspector`: Website inspection
//! - `FollowupQuestion`: User interaction
//!
//! # Example
//!
//! ```no_run
//! use crate::computer_use::tools::ExecuteCommand;
//! use crate::computer_use::base::{ToolInfo, ToolExecution};
//!
//! let tool = ExecuteCommand;
//! assert_eq!(tool.name(), "execute_command");
//! ```

mod execute_command;
mod file_operations;
mod followup_question;
mod site_inspector;

pub use execute_command::ExecuteCommand;
pub use file_operations::FileOperations;
pub use followup_question::FollowupQuestion;
pub use site_inspector::SiteInspector;

use crate::computer_use::base::ToolCollection;

/// Create a new tool collection with all available tools.
///
/// This function instantiates and registers all available tools
/// in a new ToolCollection.
///
/// # Returns
/// * `ToolCollection` - Collection containing all available tools
pub fn create_tool_collection() -> ToolCollection {
    let mut collection = ToolCollection::new();
    collection.add_tool(ExecuteCommand);
    collection.add_tool(FileOperations);
    collection.add_tool(SiteInspector);
    collection.add_tool(FollowupQuestion);
    collection
}
