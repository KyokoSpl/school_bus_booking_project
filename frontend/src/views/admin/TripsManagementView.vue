<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { adminApi } from '@/api'
import { useToast } from 'vue-toastification'
import type { Trip } from '@/types'

const router = useRouter()
const toast = useToast()

const trips = ref<Trip[]>([])
const loading = ref(true)
const currentPage = ref(1)
const totalPages = ref(1)
const searchQuery = ref('')
const deleteModal = ref<{ show: boolean; trip: Trip | null }>({ show: false, trip: null })
const deleting = ref(false)

onMounted(() => {
  loadTrips()
})

async function loadTrips() {
  loading.value = true
  try {
    const response = await adminApi.listTrips({ 
      seite: currentPage.value, 
      pro_seite: 10 
    })
    trips.value = response.data.reisen
    totalPages.value = response.data.seiten_gesamt
  } catch (error) {
    toast.error('Fehler beim Laden der Reisen')
  } finally {
    loading.value = false
  }
}

async function handleDelete() {
  if (!deleteModal.value.trip) return
  
  deleting.value = true
  try {
    await adminApi.deleteTrip(deleteModal.value.trip.id)
    toast.success('Reise erfolgreich gelöscht')
    deleteModal.value = { show: false, trip: null }
    loadTrips()
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Löschen')
  } finally {
    deleting.value = false
  }
}

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString('de-DE')
}

function formatPrice(price: number) {
  return new Intl.NumberFormat('de-DE', { style: 'currency', currency: 'EUR' }).format(price)
}

function changePage(page: number) {
  currentPage.value = page
  loadTrips()
}
</script>

<template>
  <div>
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
      <h2 class="text-2xl font-bold">Reisen verwalten</h2>
      <router-link :to="{ name: 'admin-trip-create' }" class="btn btn-primary">
        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        Neue Reise
      </router-link>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-12">
      <div class="spinner"></div>
    </div>

    <!-- Table -->
    <div v-else class="card overflow-hidden">
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Reise</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Datum</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Preis</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Buchungen</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Aktionen</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200">
            <tr v-if="trips.length === 0">
              <td colspan="6" class="px-4 py-8 text-center text-gray-500">
                Keine Reisen vorhanden
              </td>
            </tr>
            <tr v-for="trip in trips" :key="trip.id" class="hover:bg-gray-50">
              <td class="px-4 py-4">
                <div>
                  <p class="font-medium">{{ trip.titel }}</p>
                  <p class="text-sm text-gray-500">{{ trip.ziel }}</p>
                </div>
              </td>
              <td class="px-4 py-4 text-sm">
                {{ formatDate(trip.abfahrt_datum) }}
              </td>
              <td class="px-4 py-4 text-sm font-medium">
                {{ formatPrice(trip.preis_pro_person) }}
              </td>
              <td class="px-4 py-4 text-sm">
                <span :class="trip.verfuegbare_plaetze <= 5 ? 'text-red-600 font-medium' : ''">
                  {{ trip.aktuelle_buchungen }} / {{ trip.max_teilnehmer }}
                </span>
              </td>
              <td class="px-4 py-4">
                <span
                  class="badge text-xs"
                  :class="{
                    'badge-success': trip.status === 'aktiv',
                    'badge-warning': trip.status === 'entwurf',
                    'badge-danger': trip.status === 'abgesagt'
                  }"
                >
                  {{ trip.status }}
                </span>
              </td>
              <td class="px-4 py-4 text-right">
                <div class="flex justify-end gap-2">
                  <router-link
                    :to="{ name: 'admin-trip-edit', params: { id: trip.id } }"
                    class="btn btn-secondary btn-sm"
                  >
                    Bearbeiten
                  </router-link>
                  <button
                    @click="deleteModal = { show: true, trip }"
                    class="btn btn-danger btn-sm"
                  >
                    Löschen
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="px-4 py-3 border-t flex justify-center gap-2">
        <button
          @click="changePage(currentPage - 1)"
          :disabled="currentPage === 1"
          class="btn btn-secondary btn-sm"
          :class="{ 'opacity-50': currentPage === 1 }"
        >
          &larr;
        </button>
        <span class="px-4 py-2 text-sm">{{ currentPage }} / {{ totalPages }}</span>
        <button
          @click="changePage(currentPage + 1)"
          :disabled="currentPage === totalPages"
          class="btn btn-secondary btn-sm"
          :class="{ 'opacity-50': currentPage === totalPages }"
        >
          &rarr;
        </button>
      </div>
    </div>

    <!-- Delete Modal -->
    <div v-if="deleteModal.show" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
      <div class="bg-white rounded-xl max-w-md w-full p-6">
        <h3 class="text-xl font-bold mb-4">Reise löschen</h3>
        <p class="text-gray-600 mb-6">
          Möchten Sie die Reise "{{ deleteModal.trip?.titel }}" wirklich löschen?
          Dies kann nicht rückgängig gemacht werden.
        </p>
        <div class="flex gap-3">
          <button @click="deleteModal = { show: false, trip: null }" class="flex-1 btn btn-secondary">
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
