# Frontend Component Design for Function Calling

This document details the design of Vue components needed to implement function calling in the frontend application.

## Component Architecture

The function calling UI will be integrated into the existing chat interface with new components to handle tool calls and agent configuration.

```
┌─────────────────────────────────────────────────────────────┐
│                         ChatView                            │
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────┐  │
│  │             │    │             │    │                 │  │
│  │   Sidebar   │    │  ChatArea   │    │  AgentSidebar   │  │
│  │             │    │             │    │                 │  │
│  └─────────────┘    └─────────────┘    └─────────────────┘  │
│                            │                                 │
│                     ┌──────┴───────┐                         │
│                     │              │                         │
│                     │  ChatInput   │                         │
│                     │              │                         │
│                     └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
```

### New Components

1. **ToolCallDisplay**: Renders a tool call with its arguments and results
2. **ToolResultDisplay**: Renders the result of a tool call
3. **AgentSidebar**: Configures and manages agents and their tools
4. **AgentConfigPanel**: Configures an agent's settings and tools
5. **ToolSelector**: Selects tools for an agent to use

### Modified Components

1. **ChatArea**: Updated to display tool calls and results
2. **ChatInput**: Updated to support agent selection and tool configuration
3. **ChatLogic**: Extended to handle function calling flow

## Component Specifications

### 1. ToolCallDisplay Component

**Purpose**: Display a tool call with its arguments and status.

**Props**:
- `toolCall`: Object containing tool call details
  - `id`: String - Unique identifier for the tool call
  - `name`: String - Name of the tool
  - `arguments`: Object - Arguments passed to the tool
  - `status`: String - Status of the tool call (pending, running, completed, error)
  - `result`: Object (optional) - Result of the tool call
  - `error`: String (optional) - Error message if the tool call failed

**Events**:
- `retry`: Emitted when the user wants to retry a failed tool call

**Template**:
```html
<div class="tool-call" :class="{ 'has-error': toolCall.status === 'error' }">
  <div class="tool-call-header">
    <div class="tool-icon">
      <icon :name="getToolIcon(toolCall.name)" />
    </div>
    <div class="tool-info">
      <div class="tool-name">{{ toolCall.name }}</div>
      <div class="tool-status" :class="statusClass">{{ statusText }}</div>
    </div>
    <div class="tool-actions">
      <button v-if="toolCall.status === 'error'" @click="$emit('retry')" class="retry-button">
        <icon name="refresh" />
        Retry
      </button>
    </div>
  </div>
  <div class="tool-call-arguments">
    <code-block :code="formattedArguments" language="json" />
  </div>
  <div v-if="toolCall.result" class="tool-call-result">
    <tool-result-display :result="toolCall.result" />
  </div>
  <div v-if="toolCall.error" class="tool-call-error">
    <div class="error-message">{{ toolCall.error }}</div>
  </div>
</div>
```

### 2. ToolResultDisplay Component

**Purpose**: Display the result of a tool call in an appropriate format.

**Props**:
- `result`: Object or primitive value - The result of the tool call

**Template**:
```html
<div class="tool-result">
  <div v-if="isImage" class="image-result">
    <img :src="result.url" :alt="result.alt || 'Tool result'" />
  </div>
  <div v-else-if="isTable" class="table-result">
    <table>
      <thead>
        <tr>
          <th v-for="header in tableHeaders" :key="header">{{ header }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(row, index) in tableRows" :key="index">
          <td v-for="header in tableHeaders" :key="header">{{ row[header] }}</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div v-else-if="isCode" class="code-result">
    <code-block :code="result.code" :language="result.language" />
  </div>
  <div v-else class="json-result">
    <code-block :code="formattedResult" language="json" />
  </div>
</div>
```

### 3. AgentSidebar Component

**Purpose**: Manage agents and their configurations.

**Props**:
- `agents`: Array of agent objects
- `selectedAgentId`: String - ID of the currently selected agent

**Events**:
- `select-agent`: Emitted when an agent is selected
- `create-agent`: Emitted when a new agent is created
- `update-agent`: Emitted when an agent is updated
- `delete-agent`: Emitted when an agent is deleted

**Template**:
```html
<div class="agent-sidebar">
  <div class="sidebar-header">
    <h2>Agents</h2>
    <button @click="showCreateAgentPanel = true" class="create-button">
      <icon name="plus" />
      New Agent
    </button>
  </div>
  <div class="agent-list">
    <div
      v-for="agent in agents"
      :key="agent.id"
      class="agent-item"
      :class="{ 'selected': agent.id === selectedAgentId }"
      @click="$emit('select-agent', agent.id)"
    >
      <div class="agent-icon">
        <icon :name="agent.icon || 'robot'" />
      </div>
      <div class="agent-info">
        <div class="agent-name">{{ agent.name }}</div>
        <div class="agent-description">{{ agent.description }}</div>
      </div>
      <div class="agent-actions">
        <button @click.stop="editAgent(agent)" class="edit-button">
          <icon name="edit" />
        </button>
        <button @click.stop="confirmDeleteAgent(agent)" class="delete-button">
          <icon name="trash" />
        </button>
      </div>
    </div>
  </div>
  <agent-config-panel
    v-if="showCreateAgentPanel"
    @save="createAgent"
    @cancel="showCreateAgentPanel = false"
  />
  <agent-config-panel
    v-if="showEditAgentPanel"
    :agent="editingAgent"
    @save="updateAgent"
    @cancel="showEditAgentPanel = false"
  />
  <confirm-dialog
    v-if="showDeleteConfirmation"
    :title="`Delete ${deletingAgent?.name}?`"
    :message="'This action cannot be undone.'"
    @confirm="deleteAgent"
    @cancel="showDeleteConfirmation = false"
  />
</div>
```

### 4. AgentConfigPanel Component

**Purpose**: Configure an agent's settings and tools.

**Props**:
- `agent`: Object (optional) - Agent to edit, if not provided, a new agent will be created

**Events**:
- `save`: Emitted when the agent configuration is saved
- `cancel`: Emitted when the configuration is cancelled

**Template**:
```html
<div class="agent-config-panel">
  <div class="panel-header">
    <h3>{{ agent ? 'Edit Agent' : 'Create Agent' }}</h3>
    <button @click="$emit('cancel')" class="close-button">
      <icon name="x" />
    </button>
  </div>
  <div class="panel-body">
    <div class="form-group">
      <label for="agent-name">Name</label>
      <input id="agent-name" v-model="agentName" type="text" />
    </div>
    <div class="form-group">
      <label for="agent-description">Description</label>
      <textarea id="agent-description" v-model="agentDescription"></textarea>
    </div>
    <div class="form-group">
      <label for="agent-icon">Icon</label>
      <select id="agent-icon" v-model="agentIcon">
        <option value="robot">Robot</option>
        <option value="brain">Brain</option>
        <option value="search">Search</option>
        <option value="code">Code</option>
        <option value="database">Database</option>
      </select>
    </div>
    <div class="form-group">
      <label>Tools</label>
      <tool-selector v-model="selectedTools" />
    </div>
  </div>
  <div class="panel-footer">
    <button @click="saveAgent" :disabled="!isValid" class="save-button">
      Save
    </button>
    <button @click="$emit('cancel')" class="cancel-button">
      Cancel
    </button>
  </div>
</div>
```

### 5. ToolSelector Component

**Purpose**: Select tools for an agent to use.

**Props**:
- `modelValue`: Array of selected tool IDs
- `availableTools`: Array of available tools (optional)

**Events**:
- `update:modelValue`: Emitted when the selection changes

**Template**:
```html
<div class="tool-selector">
  <div v-if="loading" class="loading-indicator">
    Loading tools...
  </div>
  <div v-else-if="error" class="error-message">
    {{ error }}
  </div>
  <div v-else class="tool-list">
    <div v-for="tool in tools" :key="tool.id" class="tool-item">
      <input
        type="checkbox"
        :id="`tool-${tool.id}`"
        :value="tool.id"
        v-model="selectedToolIds"
        @change="updateModelValue"
      />
      <label :for="`tool-${tool.id}`" class="tool-label">
        <div class="tool-icon">
          <icon :name="tool.icon || 'tool'" />
        </div>
        <div class="tool-info">
          <div class="tool-name">{{ tool.name }}</div>
          <div class="tool-description">{{ tool.description }}</div>
        </div>
      </label>
    </div>
  </div>
</div>
```

## Modified Component Specifications

### 1. ChatArea Component (Modified)

**Changes**:
- Add support for rendering tool calls and results
- Update message rendering to handle different message types

**Template Changes**:
```html
<!-- Add to the assistant message rendering -->
<div v-if="message.tool_calls && message.tool_calls.length > 0" class="tool-calls">
  <div v-for="toolCall in message.tool_calls" :key="toolCall.id" class="tool-call-container">
    <tool-call-display
      :tool-call="toolCall"
      @retry="retryToolCall(message.id, toolCall.id)"
    />
  </div>
</div>
```

### 2. ChatInput Component (Modified)

**Changes**:
- Add agent selection dropdown
- Add toggle for enabling/disabling function calling

**Template Changes**:
```html
<!-- Add to the chat input component -->
<div class="input-options">
  <div class="agent-selector" v-if="agents.length > 0">
    <select v-model="selectedAgentId">
      <option value="">No Agent</option>
      <option v-for="agent in agents" :key="agent.id" :value="agent.id">
        {{ agent.name }}
      </option>
    </select>
  </div>
  <div class="function-calling-toggle">
    <input
      type="checkbox"
      id="enable-function-calling"
      v-model="enableFunctionCalling"
    />
    <label for="enable-function-calling">Enable Tools</label>
  </div>
</div>
```

### 3. ChatLogic (Modified)

**Changes**:
- Add support for function calling
- Handle tool calls and results
- Manage agent state

**Code Changes**:
```typescript
// Add to ChatLogic.ts
const agents = ref<Agent[]>([]);
const selectedAgentId = ref<string>('');
const enableFunctionCalling = ref<boolean>(false);

const loadAgents = async () => {
  try {
    const response = await apiClient.listAgents();
    agents.value = response.data;
  } catch (error) {
    console.error('Error loading agents:', error);
  }
};

const sendMessageWithFunctionCalling = async () => {
  // Implementation details...
};

const retryToolCall = async (messageId: string, toolCallId: string) => {
  // Implementation details...
};

// Update the sendMessage function
const sendMessage = async () => {
  if (enableFunctionCalling.value && selectedAgentId.value) {
    await sendMessageWithFunctionCalling();
  } else {
    await sendRegularMessage();
  }
};
```

## Styling Guidelines

1. **Tool Calls**:
   - Use a distinct background color to differentiate tool calls from regular messages
   - Use icons to indicate tool type and status
   - Use consistent spacing and padding

2. **Agent UI**:
   - Use a sidebar layout for agent management
   - Use cards for agent selection
   - Use consistent form styling for configuration

3. **Color Scheme**:
   - Use the application's existing color palette
   - Use status colors consistently (green for success, red for error, etc.)
   - Ensure sufficient contrast for accessibility

## Responsive Design

1. **Mobile View**:
   - Stack components vertically on small screens
   - Collapse agent sidebar into a modal on small screens
   - Ensure touch targets are large enough for mobile use

2. **Tablet View**:
   - Use a responsive layout that adapts to medium-sized screens
   - Consider a collapsible sidebar for agent management

3. **Desktop View**:
   - Use the full layout with sidebars and main content area
   - Provide keyboard shortcuts for power users
