# Enhanced Function Calling UI

This PR enhances the function calling UI with improved tool display and a new quick tools feature.

## Features

### Enhanced Tool Call Display

- Added copy buttons for arguments, results, and errors
- Improved styling with better visual hierarchy
- Added loading spinner for running tool calls
- Added support for dark mode
- Improved error display

### Enhanced Tool Result Display

- Added support for more result types:
  - Images with captions
  - Tables with pagination info
  - Code with language display and copy button
  - HTML content
  - Links with descriptions
  - Text content
- Improved styling for all result types
- Added dark mode support

### Quick Tools Feature

- Added a quick tools grid to the chat input
- Users can click on tools to insert tool prompts into the input
- Tools are displayed with icons and formatted names
- Improved user experience for tool discovery

## Implementation Details

### ToolCallDisplay.vue

- Added copy to clipboard functionality
- Improved styling with better visual hierarchy
- Added loading spinner for running tool calls
- Added support for dark mode
- Improved error display

### ToolResultDisplay.vue

- Added support for more result types
- Improved type detection logic
- Added formatting for different result types
- Improved styling for all result types
- Added dark mode support

### ChatInput.vue

- Added quick tools grid
- Added tool prompt insertion functionality
- Improved styling for tool selection

## Testing

- Added comprehensive tests for the enhanced ToolResultDisplay component
- Tests cover all supported result types
- Tests verify correct rendering and functionality

## Screenshots

[Add screenshots here]

## How to Test

1. Enable function calling in the chat input
2. Select an agent with tools
3. Click on a quick tool to insert a tool prompt
4. Send a message that would trigger a tool call
5. Verify that the tool call is displayed with the enhanced UI
6. Verify that the tool result is displayed correctly
7. Test copying arguments, results, and errors
8. Test dark mode support

## Checklist

- [x] Enhanced ToolCallDisplay component
- [x] Enhanced ToolResultDisplay component
- [x] Added quick tools feature to ChatInput
- [x] Added comprehensive tests
- [x] Verified dark mode support
- [x] Tested manually
- [x] Code follows project style guidelines
- [x] All tests pass

## Related PRs

- #123: Initial function calling implementation
