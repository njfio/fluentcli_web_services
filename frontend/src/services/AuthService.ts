import { axiosInstance } from './apiClient';

interface LoginResponse {
  token: string;
  user: any; // Define a proper type based on your user structure
}

const AuthService = {
  async login(username: string, password: string): Promise<LoginResponse> {
    const response = await axiosInstance.post('/users/login', { username, password });
    return response.data;
  },

  async validateToken(token: string): Promise<any> {
    try {
      const response = await axiosInstance.get('/users/validate-token', {
        headers: { Authorization: `Bearer ${token}` }
      });
      return response.data;
    } catch (error) {
      throw new Error('Invalid token');
    }
  },

  logout() {
    localStorage.removeItem('token');
    delete axiosInstance.defaults.headers.common['Authorization'];
    window.location.href = '/login';
  },

  setToken(token: string) {
    localStorage.setItem('token', token);
    axiosInstance.defaults.headers.common['Authorization'] = `Bearer ${token}`;
  },

  getToken(): string | null {
    return localStorage.getItem('token');
  },

  removeToken(): void {
    localStorage.removeItem('token');
  },
};

export default AuthService;