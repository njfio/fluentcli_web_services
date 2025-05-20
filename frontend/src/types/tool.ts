/**
 * Represents a tool that can be called by an LLM
 */
export interface Tool {
  id: string;
  name: string;
  description: string;
  parameters: {
    type: string;
    properties: Record<string, {
      type: string;
      description?: string;
      enum?: string[];
      format?: string;
      minimum?: number;
      maximum?: number;
      items?: any;
    }>;
    required: string[];
  };
  icon?: string;
  category?: string;
}

/**
 * Represents a tool call from an LLM
 */
export interface ToolCall {
  id: string;
  name: string;
  arguments: any;
}

/**
 * Represents a tool call with status information
 */
export interface ToolCallWithStatus extends ToolCall {
  status: 'pending' | 'running' | 'completed' | 'error';
  result?: any;
  error?: string;
}

/**
 * Represents the result of a tool execution
 */
export interface ToolResult {
  tool_call_id: string;
  result: any;
}

/**
 * Represents a parameter for a tool
 */
export interface ToolParameter {
  name: string;
  description?: string;
  type: string;
  required: boolean;
  enum?: string[];
  format?: string;
  minimum?: number;
  maximum?: number;
  items?: any;
}
