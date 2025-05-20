import { axiosInstance } from './apiClient';
import { Tool, ToolCall, ToolResult } from '../types/tool';

/**
 * API client for tool-related operations
 */
export const toolApiClient = {
  /**
   * List all available tools
   */
  listTools: async (): Promise<Tool[]> => {
    const response = await axiosInstance.get('/function-calling/tools');
    return response.data;
  },
  
  /**
   * Get details for a specific tool
   */
  getTool: async (id: string): Promise<Tool> => {
    const response = await axiosInstance.get(`/function-calling/tools/${id}`);
    return response.data;
  },
  
  /**
   * Execute a tool call
   */
  executeToolCall: async (toolCall: ToolCall): Promise<ToolResult> => {
    const response = await axiosInstance.post('/function-calling/execute', toolCall);
    return response.data;
  }
};
