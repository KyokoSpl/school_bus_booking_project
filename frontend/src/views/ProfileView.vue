<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { authApi } from '@/api'
import { useToast } from 'vue-toastification'

const authStore = useAuthStore()
const toast = useToast()

const activeTab = ref<'profile' | 'password'>('profile')
const savingProfile = ref(false)
const savingPassword = ref(false)

const profileForm = ref({
  vorname: '',
  nachname: '',
  telefon: '',
  strasse: '',
  plz: '',
  ort: '',
  land: 'Deutschland',
  geburtsdatum: ''
})

const passwordForm = ref({
  current_password: '',
  new_password: '',
  confirm_password: ''
})

onMounted(() => {
  if (authStore.user) {
    profileForm.value = {
      vorname: authStore.user.vorname || '',
      nachname: authStore.user.nachname || '',
      telefon: authStore.user.telefon || '',
      strasse: authStore.user.strasse || '',
      plz: authStore.user.plz || '',
      ort: authStore.user.ort || '',
      land: authStore.user.land || 'Deutschland',
      geburtsdatum: authStore.user.geburtsdatum || ''
    }
  }
})

async function handleSaveProfile() {
  savingProfile.value = true
  try {
    const response = await authApi.updateProfile(profileForm.value)
    authStore.updateUser(response.data)
    toast.success('Profil erfolgreich gespeichert')
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Speichern')
  } finally {
    savingProfile.value = false
  }
}

async function handleChangePassword() {
  if (passwordForm.value.new_password !== passwordForm.value.confirm_password) {
    toast.error('Die Passwörter stimmen nicht überein')
    return
  }

  if (passwordForm.value.new_password.length < 8) {
    toast.error('Das Passwort muss mindestens 8 Zeichen haben')
    return
  }

  savingPassword.value = true
  try {
    await authApi.changePassword(
      passwordForm.value.current_password,
      passwordForm.value.new_password
    )
    toast.success('Passwort erfolgreich geändert')
    passwordForm.value = {
      current_password: '',
      new_password: '',
      confirm_password: ''
    }
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Ändern des Passworts')
  } finally {
    savingPassword.value = false
  }
}
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <h1 class="text-3xl font-bold mb-8 dark:text-gray-100">Mein Profil</h1>

    <!-- Tabs -->
    <div class="flex gap-2 mb-6 border-b dark:border-gray-700">
      <button
        @click="activeTab = 'profile'"
        class="px-4 py-2 font-medium border-b-2 transition-colors"
        :class="activeTab === 'profile' 
          ? 'border-primary-600 text-primary-600 dark:border-primary-400 dark:text-primary-400' 
          : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'"
      >
        Persönliche Daten
      </button>
      <button
        @click="activeTab = 'password'"
        class="px-4 py-2 font-medium border-b-2 transition-colors"
        :class="activeTab === 'password' 
          ? 'border-primary-600 text-primary-600 dark:border-primary-400 dark:text-primary-400' 
          : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'"
      >
        Passwort ändern
      </button>
    </div>

    <!-- Profile Form -->
    <div v-if="activeTab === 'profile'" class="max-w-2xl">
      <div class="card p-6">
        <form @submit.prevent="handleSaveProfile" class="space-y-6">
          <div class="grid md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Vorname *</label>
              <input
                v-model="profileForm.vorname"
                type="text"
                required
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nachname *</label>
              <input
                v-model="profileForm.nachname"
                type="text"
                required
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">E-Mail</label>
            <input
              :value="authStore.user?.email"
              type="email"
              disabled
              class="w-full px-4 py-3 border border-gray-200 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-800 text-gray-500 dark:text-gray-400"
            />
            <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">E-Mail-Adresse kann nicht geändert werden</p>
          </div>

          <div class="grid md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Telefon</label>
              <input
                v-model="profileForm.telefon"
                type="tel"
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
                placeholder="+49 123 456789"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Geburtsdatum</label>
              <input
                v-model="profileForm.geburtsdatum"
                type="date"
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Straße und Hausnummer</label>
            <input
              v-model="profileForm.strasse"
              type="text"
              class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
              placeholder="Musterstraße 123"
            />
          </div>

          <div class="grid md:grid-cols-3 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">PLZ</label>
              <input
                v-model="profileForm.plz"
                type="text"
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
                placeholder="12345"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Ort</label>
              <input
                v-model="profileForm.ort"
                type="text"
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
                placeholder="Musterstadt"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Land</label>
              <input
                v-model="profileForm.land"
                type="text"
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
              />
            </div>
          </div>

          <div class="pt-4">
            <button type="submit" :disabled="savingProfile" class="btn btn-primary">
              <span v-if="savingProfile" class="spinner mr-2"></span>
              {{ savingProfile ? 'Speichern...' : 'Änderungen speichern' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Password Form -->
    <div v-if="activeTab === 'password'" class="max-w-md">
      <div class="card p-6">
        <form @submit.prevent="handleChangePassword" class="space-y-6">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Aktuelles Passwort *</label>
            <input
              v-model="passwordForm.current_password"
              type="password"
              required
              class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Neues Passwort * (min. 8 Zeichen)</label>
            <input
              v-model="passwordForm.new_password"
              type="password"
              required
              minlength="8"
              class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Neues Passwort bestätigen *</label>
            <input
              v-model="passwordForm.confirm_password"
              type="password"
              required
              class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
              :class="{ 'border-red-500 dark:border-red-500': passwordForm.confirm_password && passwordForm.new_password !== passwordForm.confirm_password }"
            />
            <p v-if="passwordForm.confirm_password && passwordForm.new_password !== passwordForm.confirm_password" class="text-red-500 dark:text-red-400 text-sm mt-1">
              Die Passwörter stimmen nicht überein
            </p>
          </div>

          <div class="pt-4">
            <button type="submit" :disabled="savingPassword" class="btn btn-primary">
              <span v-if="savingPassword" class="spinner mr-2"></span>
              {{ savingPassword ? 'Ändern...' : 'Passwort ändern' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>