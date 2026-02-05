<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { tripsApi } from '@/api'
import type { Trip, TripSearchParams } from '@/types'

const route = useRoute()
const router = useRouter()

const trips = ref<Trip[]>([])
const loading = ref(true)
const totalPages = ref(1)
const currentPage = ref(1)
const totalTrips = ref(0)

const filters = ref<TripSearchParams>({
  ziel: '',
  abfahrt_von: '',
  abfahrt_bis: '',
  preis_min: undefined,
  preis_max: undefined,
  nur_verfuegbar: true,
  sortierung: 'datum_asc',
  seite: 1,
  pro_seite: 9
})

const showFilters = ref(false)

onMounted(() => {
  // Parse URL params
  if (route.query.ziel) filters.value.ziel = route.query.ziel as string
  if (route.query.abfahrt_von) filters.value.abfahrt_von = route.query.abfahrt_von as string
  
  loadTrips()
})

watch(() => route.query, () => {
  if (route.query.ziel) filters.value.ziel = route.query.ziel as string
  if (route.query.abfahrt_von) filters.value.abfahrt_von = route.query.abfahrt_von as string
  loadTrips()
}, { deep: true })

async function loadTrips() {
  loading.value = true
  try {
    const params: TripSearchParams = {
      ...filters.value,
      seite: currentPage.value
    }
    
    // Remove empty values
    Object.keys(params).forEach(key => {
      const k = key as keyof TripSearchParams
      if (params[k] === '' || params[k] === undefined) {
        delete params[k]
      }
    })

    const response = await tripsApi.list(params)
    trips.value = response.data.reisen
    totalPages.value = response.data.seiten_gesamt
    totalTrips.value = response.data.gesamt
  } catch (error) {
    console.error('Fehler beim Laden der Reisen:', error)
  } finally {
    loading.value = false
  }
}

function applyFilters() {
  currentPage.value = 1
  loadTrips()
  
  // Update URL
  const query: Record<string, string> = {}
  if (filters.value.ziel) query.ziel = filters.value.ziel
  if (filters.value.abfahrt_von) query.abfahrt_von = filters.value.abfahrt_von
  router.replace({ query })
}

function resetFilters() {
  filters.value = {
    ziel: '',
    abfahrt_von: '',
    abfahrt_bis: '',
    preis_min: undefined,
    preis_max: undefined,
    nur_verfuegbar: true,
    sortierung: 'datum_asc',
    seite: 1,
    pro_seite: 9
  }
  currentPage.value = 1
  router.replace({ query: {} })
  loadTrips()
}

function changePage(page: number) {
  currentPage.value = page
  loadTrips()
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString('de-DE', {
    weekday: 'short',
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
  <div class="container mx-auto px-4 py-8">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-3xl md:text-4xl font-bold mb-2">Unsere Reisen</h1>
      <p class="text-gray-600">Finden Sie Ihre perfekte Busreise</p>
    </div>

    <!-- Filters -->
    <div class="card mb-8">
      <div class="p-4 border-b flex items-center justify-between">
        <h2 class="font-semibold">Filter & Suche</h2>
        <button @click="showFilters = !showFilters" class="md:hidden btn btn-secondary btn-sm">
          {{ showFilters ? 'Verbergen' : 'Anzeigen' }}
        </button>
      </div>
      
      <div :class="['p-4', { 'hidden md:block': !showFilters }]">
        <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-4 mb-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Ziel</label>
            <input
              v-model="filters.ziel"
              type="text"
              placeholder="z.B. Berlin"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Abfahrt ab</label>
            <input
              v-model="filters.abfahrt_von"
              type="date"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Abfahrt bis</label>
            <input
              v-model="filters.abfahrt_bis"
              type="date"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Sortierung</label>
            <select
              v-model="filters.sortierung"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500"
            >
              <option value="datum_asc">Datum aufsteigend</option>
              <option value="datum_desc">Datum absteigend</option>
              <option value="preis_asc">Preis aufsteigend</option>
              <option value="preis_desc">Preis absteigend</option>
            </select>
          </div>
        </div>
        
        <div class="grid md:grid-cols-4 gap-4 mb-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Preis min (€)</label>
            <input
              v-model.number="filters.preis_min"
              type="number"
              min="0"
              placeholder="0"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Preis max (€)</label>
            <input
              v-model.number="filters.preis_max"
              type="number"
              min="0"
              placeholder="500"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500"
            />
          </div>
          <div class="flex items-end">
            <label class="flex items-center cursor-pointer">
              <input
                v-model="filters.nur_verfuegbar"
                type="checkbox"
                class="w-4 h-4 text-primary-600 border-gray-300 rounded focus:ring-primary-500"
              />
              <span class="ml-2 text-sm text-gray-700">Nur verfügbare Reisen</span>
            </label>
          </div>
        </div>

        <div class="flex gap-2">
          <button @click="applyFilters" class="btn btn-primary">
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            Suchen
          </button>
          <button @click="resetFilters" class="btn btn-secondary">
            Zurücksetzen
          </button>
        </div>
      </div>
    </div>

    <!-- Results info -->
    <div class="mb-4 text-gray-600">
      <span v-if="!loading">{{ totalTrips }} Reise(n) gefunden</span>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-12">
      <div class="spinner"></div>
    </div>

    <!-- No results -->
    <div v-else-if="trips.length === 0" class="text-center py-12">
      <svg class="w-16 h-16 mx-auto text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <h3 class="text-xl font-semibold mb-2">Keine Reisen gefunden</h3>
      <p class="text-gray-600 mb-4">Versuchen Sie es mit anderen Suchkriterien.</p>
      <button @click="resetFilters" class="btn btn-primary">Filter zurücksetzen</button>
    </div>

    <!-- Trip Grid -->
    <div v-else class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
      <router-link
        v-for="trip in trips"
        :key="trip.id"
        :to="{ name: 'trip-detail', params: { id: trip.id } }"
        class="card hover:shadow-xl transition-all duration-300 overflow-hidden group"
      >
        <div class="h-48 bg-gradient-to-br from-primary-400 to-primary-600 relative">
          <div class="absolute inset-0 flex items-center justify-center">
            <svg class="w-16 h-16 text-white/30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
          </div>
          <div class="absolute top-4 right-4">
            <span class="badge badge-warning text-sm font-bold">{{ formatPrice(trip.preis_pro_person) }}</span>
          </div>
          <div v-if="trip.verfuegbare_plaetze <= 5 && trip.verfuegbare_plaetze > 0" class="absolute top-4 left-4">
            <span class="badge bg-red-500 text-white">Nur noch {{ trip.verfuegbare_plaetze }} Plätze!</span>
          </div>
          <div v-if="trip.verfuegbare_plaetze === 0" class="absolute top-4 left-4">
            <span class="badge bg-gray-800 text-white">Ausgebucht</span>
          </div>
        </div>
        
        <div class="p-5">
          <div class="flex items-center gap-2 text-sm text-gray-500 mb-2">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            {{ formatDate(trip.abfahrt_datum) }}
          </div>
          
          <h3 class="text-xl font-semibold mb-2 group-hover:text-primary-600 transition-colors">{{ trip.titel }}</h3>
          
          <div class="flex items-center text-gray-600 text-sm mb-3">
            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
            </svg>
            {{ trip.ziel }}
          </div>
          
          <p class="text-gray-600 text-sm mb-4 line-clamp-2">{{ trip.beschreibung }}</p>
          
          <div class="flex items-center justify-between text-sm">
            <span class="text-gray-500">
              <svg class="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              {{ trip.verfuegbare_plaetze }} / {{ trip.max_teilnehmer }} frei
            </span>
            <span class="text-primary-600 font-medium group-hover:underline">Details &rarr;</span>
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
      
      <template v-for="page in totalPages" :key="page">
        <button
          v-if="page === 1 || page === totalPages || Math.abs(page - currentPage) <= 1"
          @click="changePage(page)"
          class="btn"
          :class="page === currentPage ? 'btn-primary' : 'btn-secondary'"
        >
          {{ page }}
        </button>
        <span v-else-if="Math.abs(page - currentPage) === 2" class="px-2 py-2">...</span>
      </template>
      
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

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
