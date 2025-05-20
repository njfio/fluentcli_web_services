import { axiosInstance } from './apiClient';
import { Agent, CreateAgentRequest, UpdateAgentRequest } from '../types/agent';

/**
 * API client for agent-related operations
 */
export const agentApiClient = {
  /**
   * List all agents
   */
  listAgents: async (): Promise<Agent[]> => {
    const response = await axiosInstance.get('/agents');
    return response.data;
  },
  
  /**
   * Get details for a specific agent
   */
  getAgent: async (id: string): Promise<Agent> => {
    const response = await axiosInstance.get(`/agents/${id}`);
    return response.data;
  },
  
  /**
   * Create a new agent
   */
  createAgent: async (agentData: CreateAgentRequest): Promise<Agent> => {
    const response = await axiosInstance.post('/agents', agentData);
    return response.data;
  },
  
  /**
   * Update an existing agent
   */
  updateAgent: async (id: string, agentData: UpdateAgentRequest): Promise<Agent> => {
    const response = await axiosInstance.put(`/agents/${id}`, agentData);
    return response.data;
  },
  
  /**
   * Delete an agent
   */
  deleteAgent: async (id: string): Promise<void> => {
    await axiosInstance.delete(`/agents/${id}`);
  }
};
