import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('@/views/HomeView.vue')
    },
    {
      path: '/reisen',
      name: 'trips',
      component: () => import('@/views/TripsView.vue')
    },
    {
      path: '/reisen/:id',
      name: 'trip-detail',
      component: () => import('@/views/TripDetailView.vue')
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('@/views/LoginView.vue'),
      meta: { guest: true }
    },
    {
      path: '/register',
      name: 'register',
      component: () => import('@/views/RegisterView.vue'),
      meta: { guest: true }
    },
    {
      path: '/verify-email',
      name: 'verify-email',
      component: () => import('@/views/VerifyEmailView.vue')
    },
    {
      path: '/forgot-password',
      name: 'forgot-password',
      component: () => import('@/views/ForgotPasswordView.vue'),
      meta: { guest: true }
    },
    {
      path: '/reset-password',
      name: 'reset-password',
      component: () => import('@/views/ResetPasswordView.vue'),
      meta: { guest: true }
    },
    {
      path: '/buchungen',
      name: 'bookings',
      component: () => import('@/views/BookingsView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/buchungen/:id',
      name: 'booking-detail',
      component: () => import('@/views/BookingDetailView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/buchen/:tripId',
      name: 'book-trip',
      component: () => import('@/views/BookTripView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/profil',
      name: 'profile',
      component: () => import('@/views/ProfileView.vue'),
      meta: { requiresAuth: true }
    },
    // Admin routes
    {
      path: '/admin',
      name: 'admin',
      component: () => import('@/views/admin/AdminLayout.vue'),
      meta: { requiresAuth: true, requiresAdmin: true },
      children: [
        {
          path: '',
          name: 'admin-dashboard',
          component: () => import('@/views/admin/DashboardView.vue')
        },
        {
          path: 'reisen',
          name: 'admin-trips',
          component: () => import('@/views/admin/TripsManagementView.vue')
        },
        {
          path: 'reisen/neu',
          name: 'admin-trip-create',
          component: () => import('@/views/admin/TripFormView.vue')
        },
        {
          path: 'reisen/:id',
          name: 'admin-trip-edit',
          component: () => import('@/views/admin/TripFormView.vue')
        },
        {
          path: 'buchungen',
          name: 'admin-bookings',
          component: () => import('@/views/admin/BookingsManagementView.vue')
        },
        {
          path: 'kunden',
          name: 'admin-users',
          component: () => import('@/views/admin/UsersManagementView.vue')
        },
        {
          path: 'busse',
          name: 'admin-buses',
          component: () => import('@/views/admin/BusesManagementView.vue')
        }
      ]
    },
    // Legal pages
    {
      path: '/agb',
      name: 'terms',
      component: () => import('@/views/legal/TermsView.vue')
    },
    {
      path: '/datenschutz',
      name: 'privacy',
      component: () => import('@/views/legal/PrivacyView.vue')
    },
    {
      path: '/impressum',
      name: 'imprint',
      component: () => import('@/views/legal/ImprintView.vue')
    },
    // 404
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: () => import('@/views/NotFoundView.vue')
    }
  ],
  scrollBehavior(_to, _from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    } else {
      return { top: 0 }
    }
  }
})

// Navigation guards
router.beforeEach((to, _from, next) => {
  const authStore = useAuthStore()

  // Check if auth is initialized (from localStorage)
  if (!authStore.token) {
    authStore.initializeAuth()
  }

  const requiresAuth = to.matched.some(record => record.meta.requiresAuth)
  const requiresAdmin = to.matched.some(record => record.meta.requiresAdmin)
  const isGuestOnly = to.matched.some(record => record.meta.guest)

  if (requiresAuth && !authStore.isAuthenticated) {
    next({ name: 'login', query: { redirect: to.fullPath } })
  } else if (requiresAdmin && !authStore.isAdmin) {
    next({ name: 'home' })
  } else if (isGuestOnly && authStore.isAuthenticated) {
    next({ name: 'home' })
  } else {
    next()
  }
})

export default router
