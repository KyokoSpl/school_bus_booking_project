<script setup lang="ts">
import { ref } from 'vue'
import { authApi } from '@/api'
import { useToast } from 'vue-toastification'

const toast = useToast()

const email = ref('')
const loading = ref(false)
const emailSent = ref(false)

async function handleSubmit() {
  if (!email.value) {
    toast.error('Bitte geben Sie Ihre E-Mail-Adresse ein')
    return
  }

  loading.value = true
  try {
    await authApi.forgotPassword(email.value)
    emailSent.value = true
    toast.success('E-Mail wurde gesendet!')
  } catch (err: any) {
    // Don't reveal if email exists or not
    emailSent.value = true
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-[70vh] flex items-center justify-center py-12 px-4">
    <div class="max-w-md w-full">
      <!-- Email Sent -->
      <div v-if="emailSent" class="card p-8 text-center">
        <div class="w-20 h-20 mx-auto mb-6 bg-primary-100 rounded-full flex items-center justify-center">
          <svg class="w-10 h-10 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
        </div>
        <h2 class="text-2xl font-bold mb-4">E-Mail gesendet</h2>
        <p class="text-gray-600 mb-6">
          Falls ein Konto mit dieser E-Mail-Adresse existiert, haben wir Ihnen eine E-Mail 
          mit Anweisungen zum Zurücksetzen Ihres Passworts gesendet.
        </p>
        <router-link :to="{ name: 'login' }" class="btn btn-primary">
          Zurück zur Anmeldung
        </router-link>
      </div>

      <!-- Form -->
      <template v-else>
        <div class="text-center mb-8">
          <h1 class="text-3xl font-bold">Passwort vergessen?</h1>
          <p class="text-gray-600 mt-2">
            Geben Sie Ihre E-Mail-Adresse ein und wir senden Ihnen einen Link zum Zurücksetzen.
          </p>
        </div>

        <div class="card p-8">
          <form @submit.prevent="handleSubmit" class="space-y-6">
            <div>
              <label for="email" class="block text-sm font-medium text-gray-700 mb-1">
                E-Mail-Adresse
              </label>
              <input
                id="email"
                v-model="email"
                type="email"
                required
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500"
                placeholder="ihre@email.de"
              />
            </div>

            <button
              type="submit"
              :disabled="loading"
              class="w-full btn btn-primary py-3"
            >
              <span v-if="loading" class="spinner mr-2"></span>
              {{ loading ? 'Senden...' : 'Link anfordern' }}
            </button>
          </form>

          <div class="mt-6 text-center">
            <router-link :to="{ name: 'login' }" class="text-primary-600 hover:underline">
              &larr; Zurück zur Anmeldung
            </router-link>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>
