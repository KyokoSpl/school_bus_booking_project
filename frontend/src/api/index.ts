import axios, { type AxiosError, type InternalAxiosRequestConfig, type AxiosResponse } from 'axios'
import type {
  AuthResponse,
  LoginRequest,
  RegisterRequest,
  User,
  UpdateProfileRequest,
  Trip,
  TripListResponse,
  TripSearchParams,
  CreateTripRequest,
  Booking,
  BookingListResponse,
  CreateBookingRequest,
  Bus,
  BusListResponse,
  CreateBusRequest,
  InitiatePaymentRequest,
  PaymentConfirmation,
  DashboardStats
} from '@/types'

// Create axios instance
const api = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json'
  }
})

// Auth interceptor
api.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// Response interceptor for error handling
api.interceptors.response.use(
  (response: AxiosResponse) => response,
  (error: AxiosError) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token')
      localStorage.removeItem('user')
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

// Auth API
export const authApi = {
  register: (data: RegisterRequest) => 
    api.post<{ message: string; user: User }>('/auth/register', data),
  
  login: (data: LoginRequest) => 
    api.post<AuthResponse>('/auth/login', data),
  
  verifyEmail: (token: string) => 
    api.get<{ message: string }>('/auth/verify-email', { params: { token } }),
  
  forgotPassword: (email: string) => 
    api.post<{ message: string }>('/auth/forgot-password', { email }),
  
  resetPassword: (token: string, new_password: string) => 
    api.post<{ message: string }>('/auth/reset-password', { token, new_password }),
  
  getProfile: () => 
    api.get<User>('/user/profile'),
  
  updateProfile: (data: UpdateProfileRequest) => 
    api.put<User>('/user/profile', data),
  
  changePassword: (current_password: string, new_password: string) => 
    api.post<{ message: string }>('/user/change-password', { current_password, new_password })
}

// Trips API
export const tripsApi = {
  list: (params?: TripSearchParams) => 
    api.get<TripListResponse>('/trips', { params }),
  
  search: (params: TripSearchParams) => 
    api.get<TripListResponse>('/trips/search', { params }),
  
  get: (id: string) => 
    api.get<Trip>(`/trips/${id}`)
}

// Bookings API
export const bookingsApi = {
  list: (params?: { seite?: number; pro_seite?: number }) => 
    api.get<BookingListResponse>('/bookings', { params }),
  
  get: (id: string) => 
    api.get<Booking>(`/bookings/${id}`),
  
  create: (data: CreateBookingRequest) => 
    api.post<Booking>('/bookings', data),
  
  cancel: (id: string, grund?: string) => 
    api.post<{ message: string }>(`/bookings/${id}/cancel`, { grund })
}

// Payments API
export const paymentsApi = {
  initiate: (data: InitiatePaymentRequest) => 
    api.post<PaymentConfirmation>('/payments/initiate', data),
  
  get: (id: string) => 
    api.get<PaymentConfirmation>(`/payments/${id}`)
}

// Admin API
export const adminApi = {
  // Dashboard
  getDashboard: () => 
    api.get<DashboardStats>('/admin/dashboard'),
  
  // Trips
  listTrips: (params?: TripSearchParams) => 
    api.get<TripListResponse>('/admin/trips', { params }),
  
  createTrip: (data: CreateTripRequest) => 
    api.post<Trip>('/admin/trips', data),
  
  updateTrip: (id: string, data: CreateTripRequest) => 
    api.put<Trip>(`/admin/trips/${id}`, data),
  
  deleteTrip: (id: string) => 
    api.delete(`/admin/trips/${id}`),
  
  // Bookings
  listBookings: (params?: { 
    kunde_id?: string
    reise_id?: string
    status?: string
    buchungsnummer?: string
    seite?: number
    pro_seite?: number 
  }) => 
    api.get<BookingListResponse>('/admin/bookings', { params }),
  
  getBooking: (id: string) => 
    api.get<Booking>(`/admin/bookings/${id}`),
  
  updateBooking: (id: string, data: { status?: string; zahlungsstatus?: string; bemerkungen?: string }) => 
    api.put<Booking>(`/admin/bookings/${id}`, data),
  
  // Users
  listUsers: (params?: { rolle?: string; suche?: string; aktiv?: boolean; seite?: number; pro_seite?: number }) => 
    api.get<{ benutzer: User[]; gesamt: number; seite: number; pro_seite: number; seiten_gesamt: number }>('/admin/users', { params }),
  
  getUser: (id: string) => 
    api.get<User>(`/admin/users/${id}`),
  
  createUser: (data: Partial<User> & { password?: string }) => 
    api.post<User>('/admin/users', data),
  
  updateUser: (id: string, data: Partial<User> & { password?: string }) => 
    api.put<User>(`/admin/users/${id}`, data),
  
  deleteUser: (id: string) => 
    api.delete(`/admin/users/${id}`),
  
  // Buses
  listBuses: () => 
    api.get<BusListResponse>('/admin/buses'),
  
  getBus: (id: string) => 
    api.get<Bus>(`/admin/buses/${id}`),
  
  createBus: (data: CreateBusRequest) => 
    api.post<Bus>('/admin/buses', data),
  
  updateBus: (id: string, data: CreateBusRequest) => 
    api.put<Bus>(`/admin/buses/${id}`, data),
  
  deleteBus: (id: string) => 
    api.delete(`/admin/buses/${id}`)
}

export default api
