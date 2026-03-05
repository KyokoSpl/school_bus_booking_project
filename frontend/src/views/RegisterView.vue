<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useToast } from 'vue-toastification'

const authStore = useAuthStore()
const toast = useToast()

const form = ref({
  vorname: '',
  nachname: '',
  email: '',
  password: '',
  passwordConfirm: '',
  telefon: '',
  agb_akzeptiert: false,
  datenschutz_akzeptiert: false
})

const loading = ref(false)
const showPassword = ref(false)
const registrationComplete = ref(false)

const passwordStrength = computed(() => {
  const password = form.value.password
  if (!password) return { score: 0, label: '', color: '' }
  
  let score = 0
  if (password.length >= 8) score++
  if (password.length >= 12) score++
  if (/[a-z]/.test(password) && /[A-Z]/.test(password)) score++
  if (/\d/.test(password)) score++
  if (/[^a-zA-Z0-9]/.test(password)) score++
  
  const levels = [
    { label: 'Sehr schwach', color: 'bg-red-500' },
    { label: 'Schwach', color: 'bg-orange-500' },
    { label: 'Mittel', color: 'bg-yellow-500' },
    { label: 'Stark', color: 'bg-lime-500' },
    { label: 'Sehr stark', color: 'bg-green-500' }
  ]
  
  return { score, ...levels[Math.min(score, 4)] }
})

const isFormValid = computed(() => {
  return (
    form.value.vorname &&
    form.value.nachname &&
    form.value.email &&
    form.value.password &&
    form.value.password === form.value.passwordConfirm &&
    form.value.password.length >= 8 &&
    form.value.agb_akzeptiert &&
    form.value.datenschutz_akzeptiert
  )
})

async function handleRegister() {
  if (!isFormValid.value) {
    toast.error('Bitte füllen Sie alle Pflichtfelder korrekt aus')
    return
  }

  loading.value = true
  try {
    await authStore.register({
      vorname: form.value.vorname,
      nachname: form.value.nachname,
      email: form.value.email,
      password: form.value.password,
      telefon: form.value.telefon || undefined,
      agb_akzeptiert: form.value.agb_akzeptiert,
      datenschutz_akzeptiert: form.value.datenschutz_akzeptiert
    })
    
    registrationComplete.value = true
    toast.success('Registrierung erfolgreich!')
  } catch (err: any) {
    const message = err.response?.data?.message || 'Registrierung fehlgeschlagen'
    toast.error(message)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-[80vh] flex items-center justify-center py-12 px-4">
    <div class="max-w-lg w-full">
      <!-- Success Message -->
      <div v-if="registrationComplete" class="text-center">
        <div class="card p-8">
          <div class="w-20 h-20 mx-auto mb-6 bg-green-100 dark:bg-green-900/50 rounded-full flex items-center justify-center">
            <svg class="w-10 h-10 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </div>
          <h2 class="text-2xl font-bold mb-4 dark:text-gray-100">Registrierung erfolgreich!</h2>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            Wir haben Ihnen eine E-Mail gesendet. Bitte bestätigen Sie Ihre E-Mail-Adresse, 
            um Ihr Konto zu aktivieren.
          </p>
          <router-link :to="{ name: 'login' }" class="btn btn-primary">
            Zur Anmeldung
          </router-link>
        </div>
      </div>

      <!-- Registration Form -->
      <template v-else>
        <div class="text-center mb-8">
          <h1 class="text-3xl font-bold dark:text-gray-100">Konto erstellen</h1>
          <p class="text-gray-600 dark:text-gray-400 mt-2">Registrieren Sie sich für Ihr persönliches Buchungsportal</p>
        </div>

        <div class="card p-8">
          <form @submit.prevent="handleRegister" class="space-y-5">
            <div class="grid md:grid-cols-2 gap-4">
              <div>
                <label for="vorname" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Vorname *
                </label>
                <input
                  id="vorname"
                  v-model="form.vorname"
                  type="text"
                  required
                  class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
                  placeholder="Max"
                />
              </div>
              <div>
                <label for="nachname" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Nachname *
                </label>
                <input
                  id="nachname"
                  v-model="form.nachname"
                  type="text"
                  required
                  class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
                  placeholder="Mustermann"
                />
              </div>
            </div>

            <div>
              <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                E-Mail-Adresse *
              </label>
              <input
                id="email"
                v-model="form.email"
                type="email"
                required
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
                placeholder="max.mustermann@email.de"
              />
            </div>

            <div>
              <label for="telefon" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Telefon (optional)
              </label>
              <input
                id="telefon"
                v-model="form.telefon"
                type="tel"
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
                placeholder="+49 123 456789"
              />
            </div>

            <div>
              <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Passwort * (min. 8 Zeichen)
              </label>
              <div class="relative">
                <input
                  id="password"
                  v-model="form.password"
                  :type="showPassword ? 'text' : 'password'"
                  required
                  minlength="8"
                  class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400 pr-12"
                  placeholder="Mindestens 8 Zeichen"
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
              <!-- Password strength indicator -->
              <div v-if="form.password" class="mt-2">
                <div class="flex gap-1 h-1">
                  <div
                    v-for="i in 5"
                    :key="i"
                    class="flex-1 rounded"
                    :class="i <= passwordStrength.score ? passwordStrength.color : 'bg-gray-200 dark:bg-gray-600'"
                  ></div>
                </div>
                <p class="text-xs mt-1" :class="passwordStrength.score >= 3 ? 'text-green-600 dark:text-green-400' : 'text-gray-500 dark:text-gray-400'">
                  {{ passwordStrength.label }}
                </p>
              </div>
            </div>

            <div>
              <label for="passwordConfirm" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Passwort bestätigen *
              </label>
              <input
                id="passwordConfirm"
                v-model="form.passwordConfirm"
                type="password"
                required
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
                :class="{ 'border-red-500 dark:border-red-500': form.passwordConfirm && form.password !== form.passwordConfirm }"
                placeholder="Passwort wiederholen"
              />
              <p v-if="form.passwordConfirm && form.password !== form.passwordConfirm" class="text-red-500 dark:text-red-400 text-sm mt-1">
                Die Passwörter stimmen nicht überein
              </p>
            </div>

            <div class="space-y-3 pt-2">
              <label class="flex items-start cursor-pointer">
                <input
                  v-model="form.agb_akzeptiert"
                  type="checkbox"
                  required
                  class="w-4 h-4 text-primary-600 border-gray-300 dark:border-gray-600 rounded focus:ring-primary-500 dark:bg-gray-700 mt-1"
                />
                <span class="ml-2 text-sm text-gray-600 dark:text-gray-400">
                  Ich akzeptiere die
                  <router-link :to="{ name: 'terms' }" target="_blank" class="text-primary-600 dark:text-primary-400 hover:underline">
                    AGB
                  </router-link>
                  *
                </span>
              </label>

              <label class="flex items-start cursor-pointer">
                <input
                  v-model="form.datenschutz_akzeptiert"
                  type="checkbox"
                  required
                  class="w-4 h-4 text-primary-600 border-gray-300 dark:border-gray-600 rounded focus:ring-primary-500 dark:bg-gray-700 mt-1"
                />
                <span class="ml-2 text-sm text-gray-600 dark:text-gray-400">
                  Ich habe die
                  <router-link :to="{ name: 'privacy' }" target="_blank" class="text-primary-600 dark:text-primary-400 hover:underline">
                    Datenschutzerklärung
                  </router-link>
                  gelesen und akzeptiere sie *
                </span>
              </label>
            </div>

            <button
              type="submit"
              :disabled="loading || !isFormValid"
              class="w-full btn btn-primary py-3 text-lg"
              :class="{ 'opacity-50 cursor-not-allowed': !isFormValid }"
            >
              <span v-if="loading" class="spinner mr-2"></span>
              {{ loading ? 'Registrieren...' : 'Registrieren' }}
            </button>
          </form>

          <div class="mt-6 text-center">
            <p class="text-gray-600 dark:text-gray-400">
              Bereits registriert?
              <router-link :to="{ name: 'login' }" class="text-primary-600 dark:text-primary-400 hover:underline font-medium">
                Jetzt anmelden
              </router-link>
            </p>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>