<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { adminApi } from '@/api'
import { useToast } from 'vue-toastification'
import type { Booking } from '@/types'

const toast = useToast()

const bookings = ref<Booking[]>([])
const loading = ref(true)
const currentPage = ref(1)
const totalPages = ref(1)

const statusFilter = ref('')
const searchQuery = ref('')

const detailModal = ref<{ show: boolean; booking: Booking | null }>({ show: false, booking: null })
const updating = ref(false)

onMounted(() => {
  loadBookings()
})

async function loadBookings() {
  loading.value = true
  try {
    const params: Record<string, string | number> = {
      seite: currentPage.value,
      pro_seite: 15
    }
    if (statusFilter.value) params.status = statusFilter.value
    if (searchQuery.value) params.buchungsnummer = searchQuery.value

    const response = await adminApi.listBookings(params)
    bookings.value = response.data.buchungen
    totalPages.value = response.data.seiten_gesamt
  } catch (error) {
    toast.error('Fehler beim Laden der Buchungen')
  } finally {
    loading.value = false
  }
}

async function updateBooking(field: 'status' | 'zahlungsstatus', value: string) {
  if (!detailModal.value.booking) return

  updating.value = true
  try {
    const data = field === 'status' ? { status: value } : { zahlungsstatus: value }
    await adminApi.updateBooking(detailModal.value.booking.id, data)
    toast.success('Buchung aktualisiert')
    loadBookings()
    
    // Update modal data
    if (detailModal.value.booking) {
      if (field === 'status') {
        detailModal.value.booking.status = value
      } else {
        detailModal.value.booking.zahlungsstatus = value
      }
    }
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Aktualisieren')
  } finally {
    updating.value = false
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
  loadBookings()
}
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold mb-6">Buchungen verwalten</h2>

    <!-- Filters -->
    <div class="card p-4 mb-6 flex flex-wrap gap-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Buchungsnummer</label>
        <input
          v-model="searchQuery"
          @keyup.enter="loadBookings"
          type="text"
          placeholder="Suchen..."
          class="px-3 py-2 border rounded-lg"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Status</label>
        <select v-model="statusFilter" @change="loadBookings" class="px-3 py-2 border rounded-lg">
          <option value="">Alle</option>
          <option value="ausstehend">Ausstehend</option>
          <option value="bestaetigt">Bestätigt</option>
          <option value="storniert">Storniert</option>
        </select>
      </div>
      <div class="flex items-end">
        <button @click="loadBookings" class="btn btn-secondary">
          <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          Aktualisieren
        </button>
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
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Buchungsnr.</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Kunde</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Reise</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Personen</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Preis</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Zahlung</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Aktionen</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200">
            <tr v-if="bookings.length === 0">
              <td colspan="8" class="px-4 py-8 text-center text-gray-500">Keine Buchungen gefunden</td>
            </tr>
            <tr v-for="booking in bookings" :key="booking.id" class="hover:bg-gray-50">
              <td class="px-4 py-3 font-mono text-sm">{{ booking.buchungsnummer }}</td>
              <td class="px-4 py-3 text-sm">
                <p>{{ booking.kunde_name }}</p>
                <p class="text-gray-500 text-xs">{{ booking.kunde_email }}</p>
              </td>
              <td class="px-4 py-3 text-sm">{{ booking.reise_titel }}</td>
              <td class="px-4 py-3 text-sm">{{ booking.anzahl_personen }}</td>
              <td class="px-4 py-3 text-sm font-medium">{{ formatPrice(booking.gesamtpreis) }}</td>
              <td class="px-4 py-3">
                <span
                  class="badge text-xs"
                  :class="{
                    'badge-success': booking.status === 'bestaetigt',
                    'badge-warning': booking.status === 'ausstehend',
                    'badge-danger': booking.status === 'storniert'
                  }"
                >
                  {{ booking.status }}
                </span>
              </td>
              <td class="px-4 py-3">
                <span
                  class="badge text-xs"
                  :class="{
                    'badge-success': booking.zahlungsstatus === 'bezahlt',
                    'badge-warning': booking.zahlungsstatus === 'ausstehend',
                    'badge-info': booking.zahlungsstatus === 'erstattet'
                  }"
                >
                  {{ booking.zahlungsstatus }}
                </span>
              </td>
              <td class="px-4 py-3 text-right">
                <button
                  @click="detailModal = { show: true, booking }"
                  class="btn btn-secondary btn-sm"
                >
                  Details
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="px-4 py-3 border-t flex justify-center gap-2">
        <button @click="changePage(currentPage - 1)" :disabled="currentPage === 1" class="btn btn-secondary btn-sm">
          &larr;
        </button>
        <span class="px-4 py-2 text-sm">{{ currentPage }} / {{ totalPages }}</span>
        <button @click="changePage(currentPage + 1)" :disabled="currentPage === totalPages" class="btn btn-secondary btn-sm">
          &rarr;
        </button>
      </div>
    </div>

    <!-- Detail Modal -->
    <div v-if="detailModal.show" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
      <div class="bg-white rounded-xl max-w-lg w-full p-6 max-h-[90vh] overflow-y-auto">
        <div class="flex justify-between items-start mb-4">
          <h3 class="text-xl font-bold">Buchung {{ detailModal.booking?.buchungsnummer }}</h3>
          <button @click="detailModal = { show: false, booking: null }" class="text-gray-500 hover:text-gray-700">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div v-if="detailModal.booking" class="space-y-4">
          <div class="grid grid-cols-2 gap-4 text-sm">
            <div>
              <p class="text-gray-500">Kunde</p>
              <p class="font-medium">{{ detailModal.booking.kunde_name }}</p>
              <p class="text-gray-600">{{ detailModal.booking.kunde_email }}</p>
            </div>
            <div>
              <p class="text-gray-500">Reise</p>
              <p class="font-medium">{{ detailModal.booking.reise_titel }}</p>
              <p class="text-gray-600">{{ detailModal.booking.reise_ziel }}</p>
            </div>
            <div>
              <p class="text-gray-500">Personen</p>
              <p class="font-medium">{{ detailModal.booking.anzahl_personen }}</p>
            </div>
            <div>
              <p class="text-gray-500">Gesamtpreis</p>
              <p class="font-medium">{{ formatPrice(detailModal.booking.gesamtpreis) }}</p>
            </div>
            <div>
              <p class="text-gray-500">Buchungsdatum</p>
              <p class="font-medium">{{ formatDate(detailModal.booking.buchungsdatum) }}</p>
            </div>
            <div>
              <p class="text-gray-500">Zahlungsmethode</p>
              <p class="font-medium capitalize">{{ detailModal.booking.zahlungsmethode || '-' }}</p>
            </div>
          </div>

          <div v-if="detailModal.booking.bemerkungen" class="text-sm">
            <p class="text-gray-500">Bemerkungen</p>
            <p class="font-medium">{{ detailModal.booking.bemerkungen }}</p>
          </div>

          <div class="border-t pt-4 space-y-3">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Buchungsstatus</label>
              <select
                :value="detailModal.booking.status"
                @change="updateBooking('status', ($event.target as HTMLSelectElement).value)"
                :disabled="updating"
                class="w-full px-3 py-2 border rounded-lg"
              >
                <option value="ausstehend">Ausstehend</option>
                <option value="bestaetigt">Bestätigt</option>
                <option value="storniert">Storniert</option>
                <option value="abgeschlossen">Abgeschlossen</option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Zahlungsstatus</label>
              <select
                :value="detailModal.booking.zahlungsstatus"
                @change="updateBooking('zahlungsstatus', ($event.target as HTMLSelectElement).value)"
                :disabled="updating"
                class="w-full px-3 py-2 border rounded-lg"
              >
                <option value="ausstehend">Ausstehend</option>
                <option value="bezahlt">Bezahlt</option>
                <option value="teilweise_bezahlt">Teilweise bezahlt</option>
                <option value="erstattet">Erstattet</option>
              </select>
            </div>
          </div>

          <div class="pt-4">
            <button @click="detailModal = { show: false, booking: null }" class="btn btn-secondary w-full">
              Schließen
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
