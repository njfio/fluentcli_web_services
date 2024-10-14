export interface User {
  user_id: string;
  token_version: number;
  // Add other user properties as needed
}

export interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
}

export interface RootState {
  auth: AuthState;
  // Add other module states as needed
  // For example:
  // studio: StudioState;
  // theme: ThemeState;
  // chat: ChatState;
}

// Add other module state interfaces as needed
// For example:
// export interface StudioState { ... }
// export interface ThemeState { ... }
// export interface ChatState { ... }
