import { Message, Conversation, UserLLMConfig, LLMProvider } from './modules/chat';
import { ChatUIState } from './modules/chatUI';
import { ToolState } from './modules/tool';
import { AgentState } from './modules/agent';

export interface User {
  user_id: string;
  token_version: number;
}

export interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
}

export interface ChatUIOptions {
  isExpanded: boolean;
  isSidebarOpen: boolean;
}

export interface ChatState extends ChatUIOptions {
  conversations: Conversation[];
  currentConversation: Conversation | null;
  messages: Message[];
  userLLMConfigs: UserLLMConfig[];
  llmProviders: LLMProvider[];
  error: string | null;
}

export interface RootState {
  auth: AuthState;
  chat: ChatUIOptions;
  chatUI: ChatUIState;
  tool: ToolState;
  agent: AgentState;
}

export interface State extends RootState {
  chat: ChatState;
}
