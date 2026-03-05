<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { authApi } from '@/api'
import { useToast } from 'vue-toastification'

const route = useRoute()
const router = useRouter()
const toast = useToast()

const token = ref('')
const password = ref('')
const confirmPassword = ref('')
const loading = ref(false)
const showPassword = ref(false)
const success = ref(false)

const isValid = computed(() => {
  return password.value.length >= 8 && password.value === confirmPassword.value && token.value
})

onMounted(() => {
  token.value = (route.query.token as string) || ''
  
  if (!token.value) {
    toast.error('Ungültiger Link')
    router.push({ name: 'forgot-password' })
  }
})

async function handleSubmit() {
  if (!isValid.value) {
    toast.error('Bitte füllen Sie alle Felder korrekt aus')
    return
  }

  loading.value = true
  try {
    await authApi.resetPassword(token.value, password.value)
    success.value = true
    toast.success('Passwort erfolgreich geändert!')
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Zurücksetzen des Passworts')
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-[70vh] flex items-center justify-center py-12 px-4">
    <div class="max-w-md w-full">
      <!-- Success -->
      <div v-if="success" class="card p-8 text-center">
        <div class="w-20 h-20 mx-auto mb-6 bg-green-100 dark:bg-green-900/50 rounded-full flex items-center justify-center">
          <svg class="w-10 h-10 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </div>
        <h2 class="text-2xl font-bold mb-4 dark:text-gray-100">Passwort geändert!</h2>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          Ihr Passwort wurde erfolgreich zurückgesetzt. Sie können sich jetzt mit Ihrem neuen Passwort anmelden.
        </p>
        <router-link :to="{ name: 'login' }" class="btn btn-primary">
          Zur Anmeldung
        </router-link>
      </div>

      <!-- Form -->
      <template v-else>
        <div class="text-center mb-8">
          <h1 class="text-3xl font-bold dark:text-gray-100">Neues Passwort</h1>
          <p class="text-gray-600 dark:text-gray-400 mt-2">Geben Sie Ihr neues Passwort ein.</p>
        </div>

        <div class="card p-8">
          <form @submit.prevent="handleSubmit" class="space-y-6">
            <div>
              <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Neues Passwort (min. 8 Zeichen)
              </label>
              <div class="relative">
                <input
                  id="password"
                  v-model="password"
                  :type="showPassword ? 'text' : 'password'"
                  required
                  minlength="8"
                  class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400 pr-12"
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

            <div>
              <label for="confirmPassword" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Passwort bestätigen
              </label>
              <input
                id="confirmPassword"
                v-model="confirmPassword"
                type="password"
                required
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
                :class="{ 'border-red-500 dark:border-red-500': confirmPassword && password !== confirmPassword }"
              />
              <p v-if="confirmPassword && password !== confirmPassword" class="text-red-500 dark:text-red-400 text-sm mt-1">
                Die Passwörter stimmen nicht überein
              </p>
            </div>

            <button
              type="submit"
              :disabled="loading || !isValid"
              class="w-full btn btn-primary py-3"
              :class="{ 'opacity-50 cursor-not-allowed': !isValid }"
            >
              <span v-if="loading" class="spinner mr-2"></span>
              {{ loading ? 'Speichern...' : 'Passwort speichern' }}
            </button>
          </form>
        </div>
      </template>
    </div>
  </div>
</template>