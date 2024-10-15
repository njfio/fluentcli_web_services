export interface User {
  user_id: string;
  token_version: number;
  // Add other user properties as needed
}

export interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
}

export interface ChatState {
  isExpanded: boolean;
  isSidebarOpen: boolean;
  // Add other chat-related state properties as needed
}

export interface RootState {
  auth: AuthState;
  chat: ChatState;
  // Add other module states as needed
  // For example:
  // studio: StudioState;
  // theme: ThemeState;
}

// Add other module state interfaces as needed
// For example:
// export interface StudioState { ... }
// export interface ThemeState { ... }
