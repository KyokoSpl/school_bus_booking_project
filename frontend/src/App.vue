<template>
  <div class="min-h-screen flex flex-col">
    <!-- Header -->
    <header class="bg-white dark:bg-gray-800 shadow-sm dark:shadow-gray-900/50 sticky top-0 z-50">
      <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <!-- Logo -->
          <div class="flex items-center">
            <router-link to="/" class="flex items-center space-x-2">
              <img src="/favicon.svg" alt="Logo" class="h-8 w-8" />
              <span class="text-xl font-bold text-primary-600 dark:text-primary-400">Sonnenschein Reisen</span>
            </router-link>
          </div>

          <!-- Desktop Navigation -->
          <div class="hidden md:flex items-center space-x-8">
            <router-link to="/" class="text-gray-700 dark:text-gray-300 hover:text-primary-600 dark:hover:text-primary-400 font-medium">
              Startseite
            </router-link>
            <router-link to="/reisen" class="text-gray-700 dark:text-gray-300 hover:text-primary-600 dark:hover:text-primary-400 font-medium">
              Reisen
            </router-link>
            
            <template v-if="authStore.isAuthenticated">
              <router-link to="/buchungen" class="text-gray-700 dark:text-gray-300 hover:text-primary-600 dark:hover:text-primary-400 font-medium">
                Meine Buchungen
              </router-link>
              <router-link 
                v-if="authStore.isAdmin" 
                to="/admin" 
                class="text-gray-700 dark:text-gray-300 hover:text-primary-600 dark:hover:text-primary-400 font-medium"
              >
                Admin
              </router-link>
              
              <!-- User Menu -->
              <div class="relative">
                <button 
                  @click="showUserMenu = !showUserMenu"
                  class="flex items-center space-x-2 text-gray-700 dark:text-gray-300 hover:text-primary-600 dark:hover:text-primary-400"
                >
                  <div class="w-8 h-8 bg-primary-100 dark:bg-primary-900/50 rounded-full flex items-center justify-center">
                    <span class="text-primary-600 dark:text-primary-400 font-medium">
                      {{ authStore.user?.vorname?.charAt(0) }}{{ authStore.user?.nachname?.charAt(0) }}
                    </span>
                  </div>
                  <span class="font-medium">{{ authStore.user?.vorname }}</span>
                  <ChevronDownIcon class="h-4 w-4" />
                </button>
                
                <div 
                  v-if="showUserMenu"
                  class="absolute right-0 mt-2 w-48 bg-white dark:bg-gray-800 rounded-lg shadow-lg dark:shadow-gray-900/50 py-1 border dark:border-gray-700"
                >
                  <router-link 
                    to="/profil" 
                    class="block px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
                    @click="showUserMenu = false"
                  >
                    Mein Profil
                  </router-link>
                  <button 
                    @click="handleLogout"
                    class="block w-full text-left px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
                  >
                    Abmelden
                  </button>
                </div>
              </div>
            </template>
            
            <template v-else>
              <router-link to="/login" class="text-gray-700 dark:text-gray-300 hover:text-primary-600 dark:hover:text-primary-400 font-medium">
                Anmelden
              </router-link>
              <router-link to="/register" class="btn btn-primary">
                Registrieren
              </router-link>
            </template>

            <!-- Dark Mode Toggle -->
            <button
              @click="themeStore.toggleTheme()"
              class="p-2 rounded-lg text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-700 dark:hover:text-gray-200 transition-colors"
              :title="themeStore.isDark ? 'Zum hellen Modus wechseln' : 'Zum dunklen Modus wechseln'"
            >
              <!-- Sun icon (shown in dark mode) -->
              <svg v-if="themeStore.isDark" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
              <!-- Moon icon (shown in light mode) -->
              <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
              </svg>
            </button>
          </div>

          <!-- Mobile menu button + dark mode toggle -->
          <div class="md:hidden flex items-center space-x-2">
            <!-- Dark Mode Toggle (mobile) -->
            <button
              @click="themeStore.toggleTheme()"
              class="p-2 rounded-lg text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              :title="themeStore.isDark ? 'Zum hellen Modus wechseln' : 'Zum dunklen Modus wechseln'"
            >
              <svg v-if="themeStore.isDark" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
              <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
              </svg>
            </button>

            <button 
              @click="showMobileMenu = !showMobileMenu"
              class="text-gray-700 dark:text-gray-300 hover:text-primary-600 dark:hover:text-primary-400"
            >
              <Bars3Icon v-if="!showMobileMenu" class="h-6 w-6" />
              <XMarkIcon v-else class="h-6 w-6" />
            </button>
          </div>
        </div>

        <!-- Mobile Navigation -->
        <div v-if="showMobileMenu" class="md:hidden pb-4">
          <div class="flex flex-col space-y-2">
            <router-link 
              to="/" 
              class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
              @click="showMobileMenu = false"
            >
              Startseite
            </router-link>
            <router-link 
              to="/reisen" 
              class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
              @click="showMobileMenu = false"
            >
              Reisen
            </router-link>
            
            <template v-if="authStore.isAuthenticated">
              <router-link 
                to="/buchungen" 
                class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
                @click="showMobileMenu = false"
              >
                Meine Buchungen
              </router-link>
              <router-link 
                to="/profil" 
                class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
                @click="showMobileMenu = false"
              >
                Mein Profil
              </router-link>
              <router-link 
                v-if="authStore.isAdmin" 
                to="/admin" 
                class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
                @click="showMobileMenu = false"
              >
                Admin
              </router-link>
              <button 
                @click="handleLogout"
                class="px-4 py-2 text-left text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
              >
                Abmelden
              </button>
            </template>
            
            <template v-else>
              <router-link 
                to="/login" 
                class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
                @click="showMobileMenu = false"
              >
                Anmelden
              </router-link>
              <router-link 
                to="/register" 
                class="px-4 py-2 text-white bg-primary-600 hover:bg-primary-700 rounded"
                @click="showMobileMenu = false"
              >
                Registrieren
              </router-link>
            </template>
          </div>
        </div>
      </nav>
    </header>

    <!-- Main Content -->
    <main class="flex-1">
      <router-view />
    </main>

    <!-- Footer -->
    <footer class="bg-gray-900 dark:bg-gray-950 text-white">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div class="grid grid-cols-1 md:grid-cols-4 gap-8">
          <div>
            <h3 class="text-lg font-bold mb-4">Sonnenschein Reisen</h3>
            <p class="text-gray-400">
              Ihr Partner für unvergessliche Busreisen seit 1990.
            </p>
          </div>
          <div>
            <h4 class="font-semibold mb-4">Schnelllinks</h4>
            <ul class="space-y-2 text-gray-400">
              <li><router-link to="/reisen" class="hover:text-white">Alle Reisen</router-link></li>
              <li><router-link to="/ueber-uns" class="hover:text-white">Über uns</router-link></li>
              <li><router-link to="/kontakt" class="hover:text-white">Kontakt</router-link></li>
            </ul>
          </div>
          <div>
            <h4 class="font-semibold mb-4">Rechtliches</h4>
            <ul class="space-y-2 text-gray-400">
              <li><router-link to="/agb" class="hover:text-white">AGB</router-link></li>
              <li><router-link to="/datenschutz" class="hover:text-white">Datenschutz</router-link></li>
              <li><router-link to="/impressum" class="hover:text-white">Impressum</router-link></li>
            </ul>
          </div>
          <div>
            <h4 class="font-semibold mb-4">Kontakt</h4>
            <ul class="space-y-2 text-gray-400">
              <li>Musterstraße 1</li>
              <li>80331 München</li>
              <li>Tel: +49 89 12345678</li>
              <li>info@sonnenschein-reisen.de</li>
            </ul>
          </div>
        </div>
        <div class="border-t border-gray-800 mt-8 pt-8 text-center text-gray-400">
          <p>&copy; {{ new Date().getFullYear() }} Sonnenschein Reisen GmbH. Alle Rechte vorbehalten.</p>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useThemeStore } from '@/stores/theme'
import { ChevronDownIcon, Bars3Icon, XMarkIcon } from '@heroicons/vue/24/outline'

const router = useRouter()
const authStore = useAuthStore()
const themeStore = useThemeStore()

const showUserMenu = ref(false)
const showMobileMenu = ref(false)

const handleLogout = () => {
  authStore.logout()
  showUserMenu.value = false
  showMobileMenu.value = false
  router.push('/')
}

// Close menus when clicking outside
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (!target.closest('.relative')) {
    showUserMenu.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  // Try to restore auth state from localStorage
  authStore.initializeAuth()
  // Initialize theme (dark/light)
  themeStore.initializeTheme()
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>