import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL;

const apiClient = axios.create({
  baseURL: API_URL,
});

export default {
  async login(username: string, password: string): Promise<{ token: string; user: any }> {
    const response = await apiClient.post('/users/login', { username, password });
    return response.data;
  },
  logout() {
    localStorage.removeItem('token');
    delete apiClient.defaults.headers.common['Authorization'];
  },
  setToken(token: string) {
    localStorage.setItem('token', token);
    apiClient.defaults.headers.common['Authorization'] = `Bearer ${token}`;
  },
  getToken(): string | null {
    return localStorage.getItem('token');
  },
};