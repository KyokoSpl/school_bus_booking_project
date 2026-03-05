<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const authStore = useAuthStore()

const sidebarOpen = ref(false)

const currentSection = computed(() => {
  const path = route.path
  if (path.includes('/admin/reisen')) return 'trips'
  if (path.includes('/admin/buchungen')) return 'bookings'
  if (path.includes('/admin/kunden')) return 'users'
  if (path.includes('/admin/busse')) return 'buses'
  return 'dashboard'
})

const menuItems = [
  { name: 'dashboard', label: 'Dashboard', icon: 'M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6', route: 'admin-dashboard' },
  { name: 'trips', label: 'Reisen', icon: 'M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z M15 11a3 3 0 11-6 0 3 3 0 016 0z', route: 'admin-trips' },
  { name: 'bookings', label: 'Buchungen', icon: 'M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01', route: 'admin-bookings' },
  { name: 'users', label: 'Kunden', icon: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z', route: 'admin-users' },
  { name: 'buses', label: 'Busse', icon: 'M8 7h8a2 2 0 012 2v10a2 2 0 01-2 2H8a2 2 0 01-2-2V9a2 2 0 012-2zM6 21v-2M18 21v-2M9 3v4M15 3v4M9 17h.01M15 17h.01', route: 'admin-buses' }
]
</script>

<template>
  <div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <!-- Mobile sidebar backdrop -->
    <div
      v-if="sidebarOpen"
      @click="sidebarOpen = false"
      class="fixed inset-0 z-40 bg-black/50 lg:hidden"
    ></div>

    <!-- Sidebar -->
    <aside
      :class="[
        'fixed inset-y-0 left-0 z-50 w-64 bg-gray-900 dark:bg-gray-950 text-white transform transition-transform duration-300 lg:translate-x-0',
        sidebarOpen ? 'translate-x-0' : '-translate-x-full'
      ]"
    >
      <div class="flex items-center justify-between px-6 py-4 border-b border-gray-800">
        <router-link :to="{ name: 'admin-dashboard' }" class="text-xl font-bold">
          Admin Panel
        </router-link>
        <button @click="sidebarOpen = false" class="lg:hidden text-gray-400 hover:text-white">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <nav class="px-4 py-6">
        <ul class="space-y-2">
          <li v-for="item in menuItems" :key="item.name">
            <router-link
              :to="{ name: item.route }"
              class="flex items-center px-4 py-3 rounded-lg transition-colors"
              :class="currentSection === item.name 
                ? 'bg-primary-600 text-white' 
                : 'text-gray-300 hover:bg-gray-800 hover:text-white'"
            >
              <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" :d="item.icon" />
              </svg>
              {{ item.label }}
            </router-link>
          </li>
        </ul>
      </nav>

      <div class="absolute bottom-0 left-0 right-0 p-4 border-t border-gray-800">
        <div class="flex items-center mb-4">
          <div class="w-10 h-10 bg-primary-600 rounded-full flex items-center justify-center mr-3">
            <span class="font-bold">{{ authStore.user?.vorname?.charAt(0) }}{{ authStore.user?.nachname?.charAt(0) }}</span>
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium truncate">{{ authStore.user?.vorname }} {{ authStore.user?.nachname }}</p>
            <p class="text-xs text-gray-400 truncate">{{ authStore.user?.email }}</p>
          </div>
        </div>
        <router-link :to="{ name: 'home' }" class="flex items-center text-gray-400 hover:text-white text-sm">
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 17l-5-5m0 0l5-5m-5 5h12" />
          </svg>
          Zurück zur Website
        </router-link>
      </div>
    </aside>

    <!-- Main content -->
    <div class="lg:pl-64">
      <!-- Top bar -->
      <header class="sticky top-0 z-30 bg-white dark:bg-gray-800 border-b dark:border-gray-700 shadow-sm dark:shadow-gray-900/50">
        <div class="flex items-center justify-between px-4 py-3">
          <button @click="sidebarOpen = true" class="lg:hidden text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
          <h1 class="text-xl font-semibold text-gray-800 dark:text-gray-100 lg:pl-0">
            {{ menuItems.find(i => i.name === currentSection)?.label || 'Admin' }}
          </h1>
          <div></div>
        </div>
      </header>

      <!-- Page content -->
      <main class="p-6">
        <router-view />
      </main>
    </div>
  </div>
</template>