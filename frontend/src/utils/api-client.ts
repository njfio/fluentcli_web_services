import axios from 'axios'
import AuthService from '../services/AuthService'

const apiClient = axios.create({
    baseURL: import.meta.env.VITE_API_URL || 'http://localhost:8000',
    headers: {
        'Content-Type': 'application/json'
    }
})

// Add auth token to requests
apiClient.interceptors.request.use(config => {
    const token = AuthService.getToken()
    if (token) {
        config.headers.Authorization = `Bearer ${token}`
    }
    return config
})

// Handle 401 responses
apiClient.interceptors.response.use(
    response => response,
    error => {
        if (error.response?.status === 401) {
            AuthService.removeToken()
            window.location.href = '/login'
        }
        return Promise.reject(error)
    }
)

export { apiClient }
