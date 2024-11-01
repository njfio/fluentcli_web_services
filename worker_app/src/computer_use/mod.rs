//! Computer use functionality for tool execution and orchestration.
//!
//! This module provides a structured approach to tool execution in a computer
//! environment, following the design patterns from Anthropic's computer-use
//! implementation.

mod base;
mod computer;
#[cfg(test)]
mod tests;
mod tools;

pub use computer::Computer;

/// Create a new Computer instance with default configuration.
pub fn create_computer() -> Computer {
    Computer::new()
}
