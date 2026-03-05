<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { bookingsApi } from '@/api'
import type { Booking } from '@/types'

const bookings = ref<Booking[]>([])
const loading = ref(true)
const currentPage = ref(1)
const totalPages = ref(1)

onMounted(() => {
  loadBookings()
})

async function loadBookings() {
  loading.value = true
  try {
    const response = await bookingsApi.list({ seite: currentPage.value, pro_seite: 10 })
    bookings.value = response.data.buchungen
    totalPages.value = response.data.seiten_gesamt
  } catch (error) {
    console.error('Fehler beim Laden der Buchungen:', error)
  } finally {
    loading.value = false
  }
}

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString('de-DE', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  })
}

function formatPrice(price: number) {
  return new Intl.NumberFormat('de-DE', {
    style: 'currency',
    currency: 'EUR'
  }).format(price)
}

function getStatusBadgeClass(status: string) {
  switch (status) {
    case 'bestaetigt': return 'badge-success'
    case 'ausstehend': return 'badge-warning'
    case 'storniert': return 'badge-danger'
    default: return 'badge-info'
  }
}

function getStatusLabel(status: string) {
  switch (status) {
    case 'bestaetigt': return 'Bestätigt'
    case 'ausstehend': return 'Ausstehend'
    case 'storniert': return 'Storniert'
    case 'abgeschlossen': return 'Abgeschlossen'
    default: return status
  }
}

function getPaymentStatusLabel(status: string) {
  switch (status) {
    case 'bezahlt': return 'Bezahlt'
    case 'ausstehend': return 'Ausstehend'
    case 'teilweise_bezahlt': return 'Teilweise bezahlt'
    case 'erstattet': return 'Erstattet'
    default: return status
  }
}

function changePage(page: number) {
  currentPage.value = page
  loadBookings()
}
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <h1 class="text-3xl font-bold mb-8 dark:text-gray-100">Meine Buchungen</h1>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-12">
      <div class="spinner"></div>
    </div>

    <!-- Empty state -->
    <div v-else-if="bookings.length === 0" class="text-center py-12">
      <svg class="w-20 h-20 mx-auto text-gray-400 dark:text-gray-500 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
      </svg>
      <h2 class="text-xl font-semibold mb-2 dark:text-gray-100">Keine Buchungen vorhanden</h2>
      <p class="text-gray-600 dark:text-gray-400 mb-6">Sie haben noch keine Reisen gebucht.</p>
      <router-link :to="{ name: 'trips' }" class="btn btn-primary">
        Reisen entdecken
      </router-link>
    </div>

    <!-- Bookings list -->
    <div v-else class="space-y-4">
      <router-link
        v-for="booking in bookings"
        :key="booking.id"
        :to="{ name: 'booking-detail', params: { id: booking.id } }"
        class="card p-6 block hover:shadow-lg dark:hover:shadow-gray-900/50 transition-shadow"
      >
        <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
          <div class="flex-1">
            <div class="flex items-center gap-3 mb-2">
              <span class="badge" :class="getStatusBadgeClass(booking.status)">
                {{ getStatusLabel(booking.status) }}
              </span>
              <span class="text-gray-500 dark:text-gray-400 text-sm">{{ booking.buchungsnummer }}</span>
            </div>
            <h3 class="text-xl font-semibold dark:text-gray-100">{{ booking.reise_titel }}</h3>
            <p class="text-gray-600 dark:text-gray-400">{{ booking.reise_ziel }}</p>
          </div>
          
          <div class="flex flex-col md:items-end gap-2">
            <div class="text-2xl font-bold text-primary-600 dark:text-primary-400">
              {{ formatPrice(booking.gesamtpreis) }}
            </div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              {{ booking.anzahl_personen }} Person(en)
            </div>
          </div>
        </div>

        <div class="mt-4 pt-4 border-t dark:border-gray-700 flex flex-wrap gap-4 text-sm text-gray-500 dark:text-gray-400">
          <div class="flex items-center">
            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            Reisedatum: {{ formatDate(booking.reise_datum || booking.erstellt_am) }}
          </div>
          <div class="flex items-center">
            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            Zahlung: {{ getPaymentStatusLabel(booking.zahlungsstatus) }}
          </div>
          <div class="flex items-center">
            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            Gebucht am: {{ formatDate(booking.buchungsdatum) }}
          </div>
        </div>
      </router-link>
    </div>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="flex justify-center mt-8 gap-2">
      <button
        @click="changePage(currentPage - 1)"
        :disabled="currentPage === 1"
        class="btn btn-secondary"
        :class="{ 'opacity-50 cursor-not-allowed': currentPage === 1 }"
      >
        &larr; Zurück
      </button>
      <span class="px-4 py-2 text-gray-600 dark:text-gray-400">
        Seite {{ currentPage }} von {{ totalPages }}
      </span>
      <button
        @click="changePage(currentPage + 1)"
        :disabled="currentPage === totalPages"
        class="btn btn-secondary"
        :class="{ 'opacity-50 cursor-not-allowed': currentPage === totalPages }"
      >
        Weiter &rarr;
      </button>
    </div>
  </div>
</template>