<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { authApi } from '@/api'
import { useToast } from 'vue-toastification'

const route = useRoute()
const router = useRouter()
const toast = useToast()

const loading = ref(true)
const success = ref(false)
const error = ref<string | null>(null)

onMounted(async () => {
  const token = route.query.token as string
  
  if (!token) {
    error.value = 'Ungültiger Verifizierungslink'
    loading.value = false
    return
  }

  try {
    await authApi.verifyEmail(token)
    success.value = true
    toast.success('E-Mail erfolgreich verifiziert!')
  } catch (err: any) {
    error.value = err.response?.data?.message || 'Verifizierung fehlgeschlagen'
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="min-h-[70vh] flex items-center justify-center py-12 px-4">
    <div class="max-w-md w-full text-center">
      <!-- Loading -->
      <div v-if="loading" class="card p-8">
        <div class="spinner mx-auto mb-4"></div>
        <p class="text-gray-600">E-Mail wird verifiziert...</p>
      </div>

      <!-- Success -->
      <div v-else-if="success" class="card p-8">
        <div class="w-20 h-20 mx-auto mb-6 bg-green-100 rounded-full flex items-center justify-center">
          <svg class="w-10 h-10 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </div>
        <h2 class="text-2xl font-bold mb-4">E-Mail verifiziert!</h2>
        <p class="text-gray-600 mb-6">
          Ihre E-Mail-Adresse wurde erfolgreich bestätigt. Sie können sich jetzt anmelden.
        </p>
        <router-link :to="{ name: 'login' }" class="btn btn-primary">
          Zur Anmeldung
        </router-link>
      </div>

      <!-- Error -->
      <div v-else class="card p-8">
        <div class="w-20 h-20 mx-auto mb-6 bg-red-100 rounded-full flex items-center justify-center">
          <svg class="w-10 h-10 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </div>
        <h2 class="text-2xl font-bold mb-4">Verifizierung fehlgeschlagen</h2>
        <p class="text-gray-600 mb-6">{{ error }}</p>
        <router-link :to="{ name: 'login' }" class="btn btn-primary">
          Zur Anmeldung
        </router-link>
      </div>
    </div>
  </div>
</template>
