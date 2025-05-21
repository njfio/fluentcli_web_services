/**
 * Represents an agent that can use tools
 */
export interface Agent {
  id: string;
  name: string;
  description: string;
  tools: string[]; // Array of tool IDs
  icon?: string;
  system_prompt?: string;
  reasoning_patterns?: string[];
  created_at: string;
  updated_at: string;
}

/**
 * Request to create a new agent
 */
export interface CreateAgentRequest {
  name: string;
  description: string;
  tools: string[];
  icon?: string;
  system_prompt?: string;
  reasoning_patterns?: string[];
}

/**
 * Request to update an existing agent
 */
export interface UpdateAgentRequest {
  name?: string;
  description?: string;
  tools?: string[];
  icon?: string;
  system_prompt?: string;
  reasoning_patterns?: string[];
}
