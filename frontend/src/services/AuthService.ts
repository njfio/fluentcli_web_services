import axios from 'axios';

const API_URL = 'http://localhost:8000'; // Update with your backend URL

export default {
  async login(username: string, password: string): Promise<string> {
    const response = await axios.post(`${API_URL}/users/login`, { username, password });
    return response.data.token;
  },

  logout() {
    localStorage.removeItem('token');
  },

  getToken(): string | null {
    return localStorage.getItem('token');
  }
};