<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { adminApi } from '@/api'
import type { DashboardStats } from '@/types'

const stats = ref<DashboardStats | null>(null)
const loading = ref(true)

onMounted(async () => {
  try {
    const response = await adminApi.getDashboard()
    stats.value = response.data
  } catch (error) {
    console.error('Fehler beim Laden des Dashboards:', error)
  } finally {
    loading.value = false
  }
})

function formatPrice(price: number) {
  return new Intl.NumberFormat('de-DE', {
    style: 'currency',
    currency: 'EUR'
  }).format(price)
}

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString('de-DE', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  })
}
</script>

<template>
  <div>
    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-12">
      <div class="spinner"></div>
    </div>

    <template v-else-if="stats">
      <!-- Stats Cards -->
      <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <div class="card p-6">
          <div class="flex items-center">
            <div class="w-12 h-12 bg-blue-100 dark:bg-blue-900/50 rounded-lg flex items-center justify-center mr-4">
              <svg class="w-6 h-6 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
            </div>
            <div>
              <p class="text-2xl font-bold dark:text-gray-100">{{ stats.benutzer.gesamt }}</p>
              <p class="text-gray-500 dark:text-gray-400 text-sm">Kunden</p>
            </div>
          </div>
        </div>

        <div class="card p-6">
          <div class="flex items-center">
            <div class="w-12 h-12 bg-green-100 dark:bg-green-900/50 rounded-lg flex items-center justify-center mr-4">
              <svg class="w-6 h-6 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
              </svg>
            </div>
            <div>
              <p class="text-2xl font-bold dark:text-gray-100">{{ stats.reisen.aktiv }}</p>
              <p class="text-gray-500 dark:text-gray-400 text-sm">Aktive Reisen</p>
            </div>
          </div>
        </div>

        <div class="card p-6">
          <div class="flex items-center">
            <div class="w-12 h-12 bg-yellow-100 dark:bg-yellow-900/50 rounded-lg flex items-center justify-center mr-4">
              <svg class="w-6 h-6 text-yellow-600 dark:text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
            </div>
            <div>
              <p class="text-2xl font-bold dark:text-gray-100">{{ stats.buchungen.diesen_monat }}</p>
              <p class="text-gray-500 dark:text-gray-400 text-sm">Buchungen (Monat)</p>
            </div>
          </div>
        </div>

        <div class="card p-6">
          <div class="flex items-center">
            <div class="w-12 h-12 bg-purple-100 dark:bg-purple-900/50 rounded-lg flex items-center justify-center mr-4">
              <svg class="w-6 h-6 text-purple-600 dark:text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <p class="text-2xl font-bold dark:text-gray-100">{{ formatPrice(stats.umsatz.bezahlt) }}</p>
              <p class="text-gray-500 dark:text-gray-400 text-sm">Umsatz bezahlt</p>
            </div>
          </div>
        </div>
      </div>

      <div class="grid lg:grid-cols-2 gap-6">
        <!-- Recent Bookings -->
        <div class="card">
          <div class="p-4 border-b dark:border-gray-700">
            <h2 class="font-semibold dark:text-gray-100">Letzte Buchungen</h2>
          </div>
          <div class="divide-y dark:divide-gray-700">
            <div v-if="stats.letzte_buchungen.length === 0" class="p-4 text-gray-500 dark:text-gray-400 text-center">
              Keine Buchungen vorhanden
            </div>
            <div
              v-for="booking in stats.letzte_buchungen"
              :key="booking.buchungsnummer"
              class="p-4 flex items-center justify-between"
            >
              <div>
                <p class="font-medium dark:text-gray-100">{{ booking.kunde_name }}</p>
                <p class="text-sm text-gray-500 dark:text-gray-400">{{ booking.reise_titel }}</p>
                <p class="text-xs text-gray-400 dark:text-gray-500">{{ booking.buchungsnummer }}</p>
              </div>
              <div class="text-right">
                <p class="font-bold text-primary-600 dark:text-primary-400">{{ formatPrice(booking.gesamtpreis) }}</p>
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
              </div>
            </div>
          </div>
          <div class="p-4 border-t dark:border-gray-700">
            <router-link :to="{ name: 'admin-bookings' }" class="text-primary-600 dark:text-primary-400 hover:underline text-sm">
              Alle Buchungen ansehen &rarr;
            </router-link>
          </div>
        </div>

        <!-- Upcoming Trips -->
        <div class="card">
          <div class="p-4 border-b dark:border-gray-700">
            <h2 class="font-semibold dark:text-gray-100">Kommende Reisen</h2>
          </div>
          <div class="divide-y dark:divide-gray-700">
            <div v-if="stats.kommende_reisen.length === 0" class="p-4 text-gray-500 dark:text-gray-400 text-center">
              Keine kommenden Reisen
            </div>
            <div
              v-for="trip in stats.kommende_reisen"
              :key="trip.id"
              class="p-4 flex items-center justify-between"
            >
              <div>
                <p class="font-medium dark:text-gray-100">{{ trip.titel }}</p>
                <p class="text-sm text-gray-500 dark:text-gray-400">{{ trip.ziel }}</p>
                <p class="text-xs text-gray-400 dark:text-gray-500">{{ formatDate(trip.abfahrt_datum) }}</p>
              </div>
              <div class="text-right">
                <p class="text-lg font-bold dark:text-gray-100">
                  {{ trip.aktuelle_buchungen }}/{{ trip.max_teilnehmer }}
                </p>
                <p class="text-sm" :class="trip.verfuegbar <= 5 ? 'text-red-500 dark:text-red-400' : 'text-gray-500 dark:text-gray-400'">
                  {{ trip.verfuegbar }} frei
                </p>
              </div>
            </div>
          </div>
          <div class="p-4 border-t dark:border-gray-700">
            <router-link :to="{ name: 'admin-trips' }" class="text-primary-600 dark:text-primary-400 hover:underline text-sm">
              Alle Reisen verwalten &rarr;
            </router-link>
          </div>
        </div>
      </div>

      <!-- Quick Stats -->
      <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mt-6">
        <div class="card p-4 text-center">
          <p class="text-3xl font-bold text-green-600 dark:text-green-400">{{ stats.buchungen.bestaetigt }}</p>
          <p class="text-sm text-gray-500 dark:text-gray-400">Bestätigte Buchungen</p>
        </div>
        <div class="card p-4 text-center">
          <p class="text-3xl font-bold text-yellow-600 dark:text-yellow-400">{{ stats.buchungen.ausstehend }}</p>
          <p class="text-sm text-gray-500 dark:text-gray-400">Ausstehende Buchungen</p>
        </div>
        <div class="card p-4 text-center">
          <p class="text-3xl font-bold text-blue-600 dark:text-blue-400">{{ stats.busse.gesamt }}</p>
          <p class="text-sm text-gray-500 dark:text-gray-400">Busse gesamt</p>
        </div>
        <div class="card p-4 text-center">
          <p class="text-3xl font-bold text-purple-600 dark:text-purple-400">{{ formatPrice(stats.umsatz.ausstehend) }}</p>
          <p class="text-sm text-gray-500 dark:text-gray-400">Offene Zahlungen</p>
        </div>
      </div>
    </template>
  </div>
</template>