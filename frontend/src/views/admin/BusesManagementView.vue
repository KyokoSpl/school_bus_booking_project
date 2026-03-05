<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { adminApi } from '@/api'
import { useToast } from 'vue-toastification'
import type { Bus } from '@/types'

const toast = useToast()

const buses = ref<Bus[]>([])
const loading = ref(true)

const editModal = ref<{ show: boolean; bus: Bus | null; isNew: boolean }>({ show: false, bus: null, isNew: false })
const deleteModal = ref<{ show: boolean; bus: Bus | null }>({ show: false, bus: null })
const saving = ref(false)
const deleting = ref(false)

const editForm = ref({
  kennzeichen: '',
  bezeichnung: '',
  sitzplaetze: 50,
  ausstattung: '',
  baujahr: new Date().getFullYear(),
  status: 'verfuegbar',
  notizen: ''
})

onMounted(() => {
  loadBuses()
})

async function loadBuses() {
  loading.value = true
  try {
    const response = await adminApi.listBuses()
    buses.value = response.data.busse
  } catch (error) {
    toast.error('Fehler beim Laden der Busse')
  } finally {
    loading.value = false
  }
}

function openCreateModal() {
  editForm.value = {
    kennzeichen: '',
    bezeichnung: '',
    sitzplaetze: 50,
    ausstattung: '',
    baujahr: new Date().getFullYear(),
    status: 'verfuegbar',
    notizen: ''
  }
  editModal.value = { show: true, bus: null, isNew: true }
}

function openEditModal(bus: Bus) {
  editForm.value = {
    kennzeichen: bus.kennzeichen,
    bezeichnung: bus.bezeichnung,
    sitzplaetze: bus.sitzplaetze,
    ausstattung: bus.ausstattung?.join(', ') || '',
    baujahr: bus.baujahr || new Date().getFullYear(),
    status: bus.status,
    notizen: bus.notizen || ''
  }
  editModal.value = { show: true, bus, isNew: false }
}

async function handleSave() {
  saving.value = true
  try {
    const data = {
      kennzeichen: editForm.value.kennzeichen,
      bezeichnung: editForm.value.bezeichnung,
      sitzplaetze: editForm.value.sitzplaetze,
      ausstattung: editForm.value.ausstattung.split(',').map(s => s.trim()).filter(s => s),
      baujahr: editForm.value.baujahr,
      status: editForm.value.status,
      notizen: editForm.value.notizen || undefined
    }

    if (editModal.value.isNew) {
      await adminApi.createBus(data)
      toast.success('Bus erstellt')
    } else {
      await adminApi.updateBus(editModal.value.bus!.id, data)
      toast.success('Bus aktualisiert')
    }
    
    editModal.value = { show: false, bus: null, isNew: false }
    loadBuses()
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Speichern')
  } finally {
    saving.value = false
  }
}

async function handleDelete() {
  if (!deleteModal.value.bus) return

  deleting.value = true
  try {
    await adminApi.deleteBus(deleteModal.value.bus.id)
    toast.success('Bus gelöscht')
    deleteModal.value = { show: false, bus: null }
    loadBuses()
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Löschen')
  } finally {
    deleting.value = false
  }
}
</script>

<template>
  <div>
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
      <h2 class="text-2xl font-bold dark:text-gray-100">Busse verwalten</h2>
      <button @click="openCreateModal" class="btn btn-primary">
        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        Neuer Bus
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-12">
      <div class="spinner"></div>
    </div>

    <!-- Grid -->
    <div v-else-if="buses.length === 0" class="card p-8 text-center text-gray-500 dark:text-gray-400">
      Keine Busse vorhanden
    </div>

    <div v-else class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div v-for="bus in buses" :key="bus.id" class="card p-6">
        <div class="flex justify-between items-start mb-4">
          <div>
            <h3 class="text-lg font-semibold dark:text-gray-100">{{ bus.bezeichnung }}</h3>
            <p class="text-gray-500 dark:text-gray-400 font-mono">{{ bus.kennzeichen }}</p>
          </div>
          <span
            class="badge"
            :class="{
              'badge-success': bus.status === 'verfuegbar',
              'badge-warning': bus.status === 'wartung',
              'badge-danger': bus.status === 'defekt'
            }"
          >
            {{ bus.status }}
          </span>
        </div>

        <div class="space-y-2 text-sm mb-4">
          <div class="flex justify-between">
            <span class="text-gray-500 dark:text-gray-400">Sitzplätze:</span>
            <span class="font-medium dark:text-gray-100">{{ bus.sitzplaetze }}</span>
          </div>
          <div v-if="bus.baujahr" class="flex justify-between">
            <span class="text-gray-500 dark:text-gray-400">Baujahr:</span>
            <span class="font-medium dark:text-gray-100">{{ bus.baujahr }}</span>
          </div>
        </div>

        <div v-if="bus.ausstattung && bus.ausstattung.length > 0" class="mb-4">
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-2">Ausstattung:</p>
          <div class="flex flex-wrap gap-1">
            <span v-for="item in bus.ausstattung" :key="item" class="badge badge-info text-xs">
              {{ item }}
            </span>
          </div>
        </div>

        <div v-if="bus.notizen" class="mb-4 text-sm text-gray-600 dark:text-gray-400">
          <p class="text-gray-500 dark:text-gray-400">Notizen:</p>
          <p>{{ bus.notizen }}</p>
        </div>

        <div class="flex gap-2 pt-4 border-t dark:border-gray-700">
          <button @click="openEditModal(bus)" class="flex-1 btn btn-secondary btn-sm">
            Bearbeiten
          </button>
          <button @click="deleteModal = { show: true, bus }" class="btn btn-danger btn-sm">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Edit Modal -->
    <div v-if="editModal.show" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
      <div class="bg-white dark:bg-gray-800 rounded-xl max-w-md w-full p-6">
        <h3 class="text-xl font-bold mb-4 dark:text-gray-100">{{ editModal.isNew ? 'Neuer Bus' : 'Bus bearbeiten' }}</h3>
        
        <form @submit.prevent="handleSave" class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Kennzeichen *</label>
              <input v-model="editForm.kennzeichen" type="text" required class="w-full px-3 py-2 border rounded-lg" placeholder="M-AB 1234" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Sitzplätze *</label>
              <input v-model.number="editForm.sitzplaetze" type="number" min="1" required class="w-full px-3 py-2 border rounded-lg" />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Bezeichnung *</label>
            <input v-model="editForm.bezeichnung" type="text" required class="w-full px-3 py-2 border rounded-lg" placeholder="Mercedes Travego" />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Baujahr</label>
              <input v-model.number="editForm.baujahr" type="number" min="1990" :max="new Date().getFullYear() + 1" class="w-full px-3 py-2 border rounded-lg" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Status</label>
              <select v-model="editForm.status" class="w-full px-3 py-2 border rounded-lg">
                <option value="verfuegbar">Verfügbar</option>
                <option value="wartung">Wartung</option>
                <option value="defekt">Defekt</option>
              </select>
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Ausstattung (kommagetrennt)</label>
            <input v-model="editForm.ausstattung" type="text" class="w-full px-3 py-2 border rounded-lg" placeholder="Klimaanlage, WC, WiFi, USB" />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Notizen</label>
            <textarea v-model="editForm.notizen" rows="2" class="w-full px-3 py-2 border rounded-lg"></textarea>
          </div>

          <div class="flex gap-3 pt-4">
            <button type="button" @click="editModal = { show: false, bus: null, isNew: false }" class="flex-1 btn btn-secondary">
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
        <h3 class="text-xl font-bold mb-4 dark:text-gray-100">Bus löschen</h3>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          Möchten Sie den Bus "{{ deleteModal.bus?.bezeichnung }}" ({{ deleteModal.bus?.kennzeichen }}) wirklich löschen?
        </p>
        <div class="flex gap-3">
          <button @click="deleteModal = { show: false, bus: null }" class="flex-1 btn btn-secondary">
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
