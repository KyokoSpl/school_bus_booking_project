<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useToast } from 'vue-toastification'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()
const toast = useToast()

const email = ref('')
const password = ref('')
const rememberMe = ref(false)
const loading = ref(false)
const showPassword = ref(false)

const redirectPath = computed(() => {
  return (route.query.redirect as string) || '/'
})

async function handleLogin() {
  if (!email.value || !password.value) {
    toast.error('Bitte füllen Sie alle Felder aus')
    return
  }

  loading.value = true
  try {
    await authStore.login({
      email: email.value,
      password: password.value
    })
    
    toast.success('Erfolgreich angemeldet!')
    router.push(redirectPath.value)
  } catch (err: any) {
    const message = err.response?.data?.message || 'Anmeldung fehlgeschlagen'
    toast.error(message)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-[80vh] flex items-center justify-center py-12 px-4">
    <div class="max-w-md w-full">
      <div class="text-center mb-8">
        <h1 class="text-3xl font-bold dark:text-gray-100">Willkommen zurück</h1>
        <p class="text-gray-600 dark:text-gray-400 mt-2">Melden Sie sich an, um Ihre Buchungen zu verwalten</p>
      </div>

      <div class="card p-8">
        <form @submit.prevent="handleLogin" class="space-y-6">
          <div>
            <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              E-Mail-Adresse
            </label>
            <input
              id="email"
              v-model="email"
              type="email"
              autocomplete="email"
              required
              class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
              placeholder="ihre@email.de"
            />
          </div>

          <div>
            <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Passwort
            </label>
            <div class="relative">
              <input
                id="password"
                v-model="password"
                :type="showPassword ? 'text' : 'password'"
                autocomplete="current-password"
                required
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400 pr-12"
                placeholder="Ihr Passwort"
              />
              <button
                type="button"
                @click="showPassword = !showPassword"
                class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              >
                <svg v-if="!showPassword" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                </svg>
                <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                </svg>
              </button>
            </div>
          </div>

          <div class="flex items-center justify-between">
            <label class="flex items-center cursor-pointer">
              <input
                v-model="rememberMe"
                type="checkbox"
                class="w-4 h-4 text-primary-600 border-gray-300 dark:border-gray-600 rounded focus:ring-primary-500 dark:bg-gray-700"
              />
              <span class="ml-2 text-sm text-gray-600 dark:text-gray-400">Angemeldet bleiben</span>
            </label>

            <router-link :to="{ name: 'forgot-password' }" class="text-sm text-primary-600 dark:text-primary-400 hover:underline">
              Passwort vergessen?
            </router-link>
          </div>

          <button
            type="submit"
            :disabled="loading"
            class="w-full btn btn-primary py-3 text-lg"
          >
            <span v-if="loading" class="spinner mr-2"></span>
            {{ loading ? 'Anmelden...' : 'Anmelden' }}
          </button>
        </form>

        <div class="mt-6 text-center">
          <p class="text-gray-600 dark:text-gray-400">
            Noch kein Konto?
            <router-link :to="{ name: 'register' }" class="text-primary-600 dark:text-primary-400 hover:underline font-medium">
              Jetzt registrieren
            </router-link>
          </p>
        </div>
      </div>
    </div>
  </div>
</template>