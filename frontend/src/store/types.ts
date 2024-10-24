import { Message, Conversation, UserLLMConfig, LLMProvider } from './modules/chat';
import { ChatUIState } from './modules/chatUI';

export interface User {
  user_id: string;
  username: string;
  email: string;
}

export interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
}

export interface ChatState {
  conversations: Conversation[];
  currentConversation: Conversation | null;
  messages: Message[];
  userLLMConfigs: UserLLMConfig[];
  llmProviders: LLMProvider[];
  error: string | null;
}

export interface RootState {
  auth: AuthState;
  chat: {
    isExpanded: boolean;
    isSidebarOpen: boolean;
  };
  studio?: any;
  theme?: any;
  chatUI: ChatUIState;
}

export interface State extends RootState {
  chat: ChatState;
}
