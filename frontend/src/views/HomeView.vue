<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { tripsApi } from '@/api'
import { useAuthStore } from '@/stores/auth'
import type { Trip } from '@/types'

const router = useRouter()
const authStore = useAuthStore()

const featuredTrips = ref<Trip[]>([])
const searchForm = ref({
  ziel: '',
  datum: ''
})
const loading = ref(true)

onMounted(async () => {
  try {
    const response = await tripsApi.list({ pro_seite: 3, nur_verfuegbar: true })
    featuredTrips.value = response.data.reisen
  } catch (error) {
    console.error('Fehler beim Laden der Reisen:', error)
  } finally {
    loading.value = false
  }
})

function handleSearch() {
  const params: Record<string, string> = {}
  if (searchForm.value.ziel) params.ziel = searchForm.value.ziel
  if (searchForm.value.datum) params.abfahrt_von = searchForm.value.datum
  
  router.push({ name: 'trips', query: params })
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
</script>

<template>
  <div>
    <!-- Hero Section -->
    <section class="relative bg-gradient-to-br from-primary-600 via-primary-700 to-primary-800 text-white">
      <div class="absolute inset-0 bg-black/20"></div>
      <div class="relative container mx-auto px-4 py-20 md:py-32">
        <div class="max-w-3xl">
          <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold mb-6">
            Entdecken Sie unvergessliche Busreisen
          </h1>
          <p class="text-xl md:text-2xl text-white/90 mb-8">
            Mit Sonnenschein Reisen erleben Sie komfortable Tagesausflüge und mehrtägige Reisen 
            zu den schönsten Zielen Deutschlands und Europas.
          </p>

          <!-- Search Box -->
          <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-2xl">
            <form @submit.prevent="handleSearch" class="flex flex-col md:flex-row gap-4">
              <div class="flex-1">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Wohin möchten Sie?</label>
                <input
                  v-model="searchForm.ziel"
                  type="text"
                  placeholder="z.B. Berlin, München, Schwarzwald..."
                  class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 text-gray-900 dark:text-gray-100 dark:bg-gray-700 dark:placeholder-gray-400"
                />
              </div>
              <div class="md:w-48">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Wann?</label>
                <input
                  v-model="searchForm.datum"
                  type="date"
                  class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 text-gray-900 dark:text-gray-100 dark:bg-gray-700"
                />
              </div>
              <div class="md:self-end">
                <button type="submit" class="w-full md:w-auto btn btn-primary py-3 px-8">
                  <svg class="w-5 h-5 mr-2 -ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                  </svg>
                  Suchen
                </button>
              </div>
            </form>
          </div>
        </div>
      </div>
    </section>

    <!-- Features Section -->
    <section class="py-16 bg-gray-50 dark:bg-gray-800/50">
      <div class="container mx-auto px-4">
        <div class="grid md:grid-cols-3 gap-8">
          <div class="text-center p-6">
            <div class="w-16 h-16 mx-auto mb-4 bg-primary-100 dark:bg-primary-900/50 rounded-full flex items-center justify-center">
              <svg class="w-8 h-8 text-primary-600 dark:text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
              </svg>
            </div>
            <h3 class="text-xl font-semibold mb-2 dark:text-gray-100">Sichere Buchung</h3>
            <p class="text-gray-600 dark:text-gray-400">Verschlüsselte Datenübertragung und sichere Zahlungsabwicklung für Ihre Sicherheit.</p>
          </div>
          <div class="text-center p-6">
            <div class="w-16 h-16 mx-auto mb-4 bg-primary-100 dark:bg-primary-900/50 rounded-full flex items-center justify-center">
              <svg class="w-8 h-8 text-primary-600 dark:text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
            </div>
            <h3 class="text-xl font-semibold mb-2 dark:text-gray-100">Flexible Buchung</h3>
            <p class="text-gray-600 dark:text-gray-400">Einfache Online-Buchung rund um die Uhr mit sofortiger Bestätigung per E-Mail.</p>
          </div>
          <div class="text-center p-6">
            <div class="w-16 h-16 mx-auto mb-4 bg-primary-100 dark:bg-primary-900/50 rounded-full flex items-center justify-center">
              <svg class="w-8 h-8 text-primary-600 dark:text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
              </svg>
            </div>
            <h3 class="text-xl font-semibold mb-2 dark:text-gray-100">Persönlicher Service</h3>
            <p class="text-gray-600 dark:text-gray-400">Unser freundliches Team steht Ihnen bei Fragen gerne zur Verfügung.</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Featured Trips -->
    <section class="py-16 dark:bg-gray-900">
      <div class="container mx-auto px-4">
        <div class="text-center mb-12">
          <h2 class="text-3xl md:text-4xl font-bold mb-4 dark:text-gray-100">Beliebte Reisen</h2>
          <p class="text-gray-600 dark:text-gray-400 text-lg max-w-2xl mx-auto">
            Entdecken Sie unsere beliebtesten Reiseziele und buchen Sie Ihren nächsten Ausflug.
          </p>
        </div>

        <div v-if="loading" class="flex justify-center">
          <div class="spinner"></div>
        </div>

        <div v-else-if="featuredTrips.length === 0" class="text-center text-gray-500 dark:text-gray-400">
          <p>Aktuell sind keine Reisen verfügbar.</p>
        </div>

        <div v-else class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
          <div
            v-for="trip in featuredTrips"
            :key="trip.id"
            class="card hover:shadow-xl transition-shadow duration-300 overflow-hidden"
          >
            <div class="h-48 bg-gradient-to-br from-primary-400 to-primary-600 flex items-center justify-center">
              <svg class="w-16 h-16 text-white/50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
            </div>
            <div class="p-6">
              <div class="flex items-center justify-between mb-2">
                <span class="badge badge-primary">{{ trip.ziel }}</span>
                <span class="text-2xl font-bold text-primary-600 dark:text-primary-400">{{ formatPrice(trip.preis_pro_person) }}</span>
              </div>
              <h3 class="text-xl font-semibold mb-2 dark:text-gray-100">{{ trip.titel }}</h3>
              <p class="text-gray-600 dark:text-gray-400 mb-4 line-clamp-2">{{ trip.beschreibung }}</p>
              <div class="flex items-center text-gray-500 dark:text-gray-400 text-sm mb-4">
                <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                </svg>
                {{ formatDate(trip.abfahrt_datum) }}
                <span class="mx-2">•</span>
                <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                </svg>
                {{ trip.verfuegbare_plaetze }} Plätze frei
              </div>
              <router-link :to="{ name: 'trip-detail', params: { id: trip.id } }" class="btn btn-primary w-full">
                Details ansehen
              </router-link>
            </div>
          </div>
        </div>

        <div class="text-center mt-10">
          <router-link :to="{ name: 'trips' }" class="btn btn-outline">
            Alle Reisen anzeigen
            <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
            </svg>
          </router-link>
        </div>
      </div>
    </section>

    <!-- CTA Section – nur für nicht eingeloggte Benutzer -->
    <section v-if="!authStore.isAuthenticated" class="py-16 bg-primary-600 dark:bg-primary-800 text-white">
      <div class="container mx-auto px-4 text-center">
        <h2 class="text-3xl md:text-4xl font-bold mb-4">Bereit für Ihr nächstes Abenteuer?</h2>
        <p class="text-xl text-white/90 mb-8 max-w-2xl mx-auto">
          Registrieren Sie sich jetzt und profitieren Sie von exklusiven Angeboten und einfacher Buchungsverwaltung.
        </p>
        <router-link :to="{ name: 'register' }" class="btn bg-white text-primary-600 hover:bg-gray-100 dark:hover:bg-gray-200 text-lg px-8 py-3">
          Jetzt registrieren
        </router-link>
      </div>
    </section>
  </div>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>