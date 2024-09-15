import apiClient from './apiClient';

interface LoginResponse {
  token: string;
  user: any; // Define a proper type based on your user structure
}

const AuthService = {
  async login(username: string, password: string): Promise<LoginResponse> {
    const response = await apiClient.post('/users/login', { username, password });
    return response.data;
  },

  logout() {
    localStorage.removeItem('token');
    delete apiClient.defaults.headers.common['Authorization'];
    window.location.href = '/login';
  },

  setToken(token: string) {
    localStorage.setItem('token', token);
    apiClient.defaults.headers.common['Authorization'] = `Bearer ${token}`;
  },

  getToken(): string | null {
    return localStorage.getItem('token');
  },
};

export default AuthService;