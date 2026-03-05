<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { adminApi } from '@/api'
import { useToast } from 'vue-toastification'
import type { User } from '@/types'

const toast = useToast()

const users = ref<User[]>([])
const loading = ref(true)
const currentPage = ref(1)
const totalPages = ref(1)

const searchQuery = ref('')
const roleFilter = ref('')

const editModal = ref<{ show: boolean; user: User | null; isNew: boolean }>({ show: false, user: null, isNew: false })
const deleteModal = ref<{ show: boolean; user: User | null }>({ show: false, user: null })
const saving = ref(false)
const deleting = ref(false)

const editForm = ref({
  vorname: '',
  nachname: '',
  email: '',
  telefon: '',
  rolle: 'kunde',
  aktiv: true,
  password: ''
})

onMounted(() => {
  loadUsers()
})

async function loadUsers() {
  loading.value = true
  try {
    const params: Record<string, string | number | boolean> = {
      seite: currentPage.value,
      pro_seite: 15
    }
    if (searchQuery.value) params.suche = searchQuery.value
    if (roleFilter.value) params.rolle = roleFilter.value

    const response = await adminApi.listUsers(params)
    users.value = response.data.benutzer
    totalPages.value = response.data.seiten_gesamt
  } catch (error) {
    toast.error('Fehler beim Laden der Benutzer')
  } finally {
    loading.value = false
  }
}

function openCreateModal() {
  editForm.value = {
    vorname: '',
    nachname: '',
    email: '',
    telefon: '',
    rolle: 'kunde',
    aktiv: true,
    password: ''
  }
  editModal.value = { show: true, user: null, isNew: true }
}

function openEditModal(user: User) {
  editForm.value = {
    vorname: user.vorname,
    nachname: user.nachname,
    email: user.email,
    telefon: user.telefon || '',
    rolle: user.rolle,
    aktiv: user.aktiv,
    password: ''
  }
  editModal.value = { show: true, user, isNew: false }
}

async function handleSave() {
  saving.value = true
  try {
    const data: any = {
      vorname: editForm.value.vorname,
      nachname: editForm.value.nachname,
      email: editForm.value.email,
      telefon: editForm.value.telefon || undefined,
      rolle: editForm.value.rolle,
      aktiv: editForm.value.aktiv
    }
    
    if (editForm.value.password) {
      data.password = editForm.value.password
    }

    if (editModal.value.isNew) {
      await adminApi.createUser(data)
      toast.success('Benutzer erstellt')
    } else {
      await adminApi.updateUser(editModal.value.user!.id, data)
      toast.success('Benutzer aktualisiert')
    }
    
    editModal.value = { show: false, user: null, isNew: false }
    loadUsers()
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Speichern')
  } finally {
    saving.value = false
  }
}

async function handleDelete() {
  if (!deleteModal.value.user) return

  deleting.value = true
  try {
    await adminApi.deleteUser(deleteModal.value.user.id)
    toast.success('Benutzer gelöscht')
    deleteModal.value = { show: false, user: null }
    loadUsers()
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Löschen')
  } finally {
    deleting.value = false
  }
}

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString('de-DE')
}

function changePage(page: number) {
  currentPage.value = page
  loadUsers()
}
</script>

<template>
  <div>
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
      <h2 class="text-2xl font-bold dark:text-gray-100">Kunden verwalten</h2>
      <button @click="openCreateModal" class="btn btn-primary">
        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        Neuer Benutzer
      </button>
    </div>

    <!-- Filters -->
    <div class="card p-4 mb-6 flex flex-wrap gap-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Suche</label>
        <input
          v-model="searchQuery"
          @keyup.enter="loadUsers"
          type="text"
          placeholder="Name oder E-Mail..."
          class="px-3 py-2 border rounded-lg"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Rolle</label>
        <select v-model="roleFilter" @change="loadUsers" class="px-3 py-2 border rounded-lg">
          <option value="">Alle</option>
          <option value="kunde">Kunde</option>
          <option value="admin">Admin</option>
        </select>
      </div>
      <div class="flex items-end">
        <button @click="loadUsers" class="btn btn-secondary">Suchen</button>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-12">
      <div class="spinner"></div>
    </div>

    <!-- Table -->
    <div v-else class="card overflow-hidden">
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead class="bg-gray-50 dark:bg-gray-800">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Name</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">E-Mail</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Rolle</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Status</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Registriert</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Aktionen</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
            <tr v-if="users.length === 0">
              <td colspan="6" class="px-4 py-8 text-center text-gray-500 dark:text-gray-400">Keine Benutzer gefunden</td>
            </tr>
            <tr v-for="user in users" :key="user.id" class="hover:bg-gray-50 dark:hover:bg-gray-700/50">
              <td class="px-4 py-3">
                <p class="font-medium dark:text-gray-100">{{ user.vorname }} {{ user.nachname }}</p>
              </td>
              <td class="px-4 py-3 text-sm dark:text-gray-300">
                {{ user.email }}
                <span v-if="!user.email_verifiziert" class="text-yellow-600 dark:text-yellow-400 text-xs ml-1">(nicht verifiziert)</span>
              </td>
              <td class="px-4 py-3">
                <span class="badge" :class="user.rolle === 'admin' ? 'badge-primary' : 'badge-secondary'">
                  {{ user.rolle }}
                </span>
              </td>
              <td class="px-4 py-3">
                <span class="badge" :class="user.aktiv ? 'badge-success' : 'badge-danger'">
                  {{ user.aktiv ? 'Aktiv' : 'Inaktiv' }}
                </span>
              </td>
              <td class="px-4 py-3 text-sm dark:text-gray-300">{{ formatDate(user.erstellt_am) }}</td>
              <td class="px-4 py-3 text-right">
                <div class="flex justify-end gap-2">
                  <button @click="openEditModal(user)" class="btn btn-secondary btn-sm">
                    Bearbeiten
                  </button>
                  <button @click="deleteModal = { show: true, user }" class="btn btn-danger btn-sm">
                    Löschen
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="px-4 py-3 border-t dark:border-gray-700 flex justify-center gap-2">
        <button @click="changePage(currentPage - 1)" :disabled="currentPage === 1" class="btn btn-secondary btn-sm">
          &larr;
        </button>
        <span class="px-4 py-2 text-sm dark:text-gray-400">{{ currentPage }} / {{ totalPages }}</span>
        <button @click="changePage(currentPage + 1)" :disabled="currentPage === totalPages" class="btn btn-secondary btn-sm">
          &rarr;
        </button>
      </div>
    </div>

    <!-- Edit Modal -->
    <div v-if="editModal.show" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
      <div class="bg-white dark:bg-gray-800 rounded-xl max-w-md w-full p-6">
        <h3 class="text-xl font-bold mb-4 dark:text-gray-100">{{ editModal.isNew ? 'Neuer Benutzer' : 'Benutzer bearbeiten' }}</h3>
        
        <form @submit.prevent="handleSave" class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Vorname *</label>
              <input v-model="editForm.vorname" type="text" required class="w-full px-3 py-2 border rounded-lg" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nachname *</label>
              <input v-model="editForm.nachname" type="text" required class="w-full px-3 py-2 border rounded-lg" />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">E-Mail *</label>
            <input v-model="editForm.email" type="email" required class="w-full px-3 py-2 border rounded-lg" />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Telefon</label>
            <input v-model="editForm.telefon" type="tel" class="w-full px-3 py-2 border rounded-lg" />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              {{ editModal.isNew ? 'Passwort *' : 'Neues Passwort (leer lassen = unverändert)' }}
            </label>
            <input v-model="editForm.password" type="password" :required="editModal.isNew" class="w-full px-3 py-2 border rounded-lg" />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Rolle</label>
              <select v-model="editForm.rolle" class="w-full px-3 py-2 border rounded-lg">
                <option value="kunde">Kunde</option>
                <option value="admin">Admin</option>
              </select>
            </div>
            <div class="flex items-end pb-2">
              <label class="flex items-center cursor-pointer">
                <input v-model="editForm.aktiv" type="checkbox" class="w-4 h-4 text-primary-600 rounded" />
                <span class="ml-2 text-sm dark:text-gray-300">Aktiv</span>
              </label>
            </div>
          </div>

          <div class="flex gap-3 pt-4">
            <button type="button" @click="editModal = { show: false, user: null, isNew: false }" class="flex-1 btn btn-secondary">
              Abbrechen
            </button>
            <button type="submit" :disabled="saving" class="flex-1 btn btn-primary">
              <span v-if="saving" class="spinner mr-2"></span>
              {{ saving ? 'Speichern...' : 'Speichern' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Delete Modal -->
    <div v-if="deleteModal.show" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
      <div class="bg-white dark:bg-gray-800 rounded-xl max-w-md w-full p-6">
        <h3 class="text-xl font-bold mb-4 dark:text-gray-100">Benutzer löschen</h3>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          Möchten Sie "{{ deleteModal.user?.vorname }} {{ deleteModal.user?.nachname }}" wirklich löschen?
        </p>
        <div class="flex gap-3">
          <button @click="deleteModal = { show: false, user: null }" class="flex-1 btn btn-secondary">
            Abbrechen
          </button>
          <button @click="handleDelete" :disabled="deleting" class="flex-1 btn btn-danger">
            <span v-if="deleting" class="spinner mr-2"></span>
            {{ deleting ? 'Löschen...' : 'Löschen' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
