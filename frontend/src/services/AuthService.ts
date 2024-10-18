import { axiosInstance } from './apiClient';
import store from '../store';

interface LoginResponse {
  token: string;
  user: any; // Define a proper type based on your user structure
}

interface UserData {
  user_id: string;
  // Add other user properties as needed
}

const AuthService = {
  async login(username: string, password: string): Promise<LoginResponse> {
    console.log('Attempting login for user:', username);
    const response = await axiosInstance.post('/users/login', { username, password });
    console.log('Login response:', response.data);
    store.dispatch('login', { user: response.data.user });
    this.setToken(response.data.token);
    return response.data;
  },

  async validateToken(token: string): Promise<UserData> {
    console.log('Validating token');
    try {
      const response = await axiosInstance.get('/users/validate-token', {
        headers: { Authorization: `Bearer ${token}` }
      });
      console.log('Token validation response:', response.data);

      // Ensure the user data is properly formatted
      const userData: UserData = {
        user_id: response.data.user_id || response.data.id,
        // Add other user properties as needed
      };

      console.log('Formatted user data:', userData);

      // Set the user data in the store
      store.dispatch('login', { user: userData });
      return userData;
    } catch (error) {
      console.error('Token validation error:', error);
      throw new Error('Invalid token');
    }
  },

  logout() {
    console.log('Logging out');
    localStorage.removeItem('token');
    delete axiosInstance.defaults.headers.common['Authorization'];
    store.dispatch('logout');
    window.location.href = '/login';
  },

  setToken(token: string) {
    console.log('Setting token');
    localStorage.setItem('token', token);
    axiosInstance.defaults.headers.common['Authorization'] = `Bearer ${token}`;
  },

  getToken(): string | null {
    const token = localStorage.getItem('token');
    console.log('Getting token:', token ? 'Token exists' : 'No token found');
    return token;
  },

  removeToken(): void {
    console.log('Removing token');
    localStorage.removeItem('token');
  },

  async initializeAuthState(): Promise<void> {
    console.log('Initializing auth state');
    const token = this.getToken();
    if (token) {
      console.log('Token found, validating');
      try {
        const userData = await this.validateToken(token);
        console.log('User data retrieved:', userData);
        // The user data is now set in the store by validateToken
      } catch (error) {
        console.error('Error initializing auth state:', error);
        this.removeToken();
        store.dispatch('logout');
      }
    } else {
      console.log('No token found, user is not authenticated');
      store.dispatch('logout');
    }
  },
};

export default AuthService;
