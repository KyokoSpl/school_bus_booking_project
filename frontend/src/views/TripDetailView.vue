<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { tripsApi } from '@/api'
import { useAuthStore } from '@/stores/auth'
import type { Trip } from '@/types'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

const trip = ref<Trip | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)

const isBookable = computed(() => {
  return trip.value && trip.value.verfuegbare_plaetze > 0 && trip.value.status === 'aktiv'
})

onMounted(async () => {
  await loadTrip()
})

async function loadTrip() {
  loading.value = true
  error.value = null
  
  try {
    const response = await tripsApi.get(route.params.id as string)
    trip.value = response.data
  } catch (err: any) {
    if (err.response?.status === 404) {
      error.value = 'Reise nicht gefunden'
    } else {
      error.value = 'Fehler beim Laden der Reise'
    }
  } finally {
    loading.value = false
  }
}

function handleBook() {
  if (!authStore.isAuthenticated) {
    router.push({ name: 'login', query: { redirect: route.fullPath } })
    return
  }
  
  router.push({ name: 'book-trip', params: { tripId: trip.value!.id } })
}

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString('de-DE', {
    weekday: 'long',
    day: '2-digit',
    month: 'long',
    year: 'numeric'
  })
}

function formatTime(timeString: string) {
  return timeString.substring(0, 5) + ' Uhr'
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
    <!-- Back button -->
    <button @click="router.back()" class="btn btn-secondary mb-6">
      <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
      Zurück
    </button>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-12">
      <div class="spinner"></div>
    </div>

    <!-- Error -->
    <div v-else-if="error" class="text-center py-12">
      <svg class="w-16 h-16 mx-auto text-red-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
      </svg>
      <h3 class="text-xl font-semibold mb-2">{{ error }}</h3>
      <router-link :to="{ name: 'trips' }" class="btn btn-primary mt-4">
        Zu allen Reisen
      </router-link>
    </div>

    <!-- Trip Content -->
    <div v-else-if="trip" class="grid lg:grid-cols-3 gap-8">
      <!-- Main Content -->
      <div class="lg:col-span-2 space-y-6">
        <!-- Hero Image -->
        <div class="card overflow-hidden">
          <div class="h-64 md:h-96 bg-gradient-to-br from-primary-400 to-primary-600 flex items-center justify-center relative">
            <svg class="w-24 h-24 text-white/30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <div class="absolute bottom-4 left-4">
              <span class="badge badge-primary text-lg font-semibold px-4 py-2">{{ trip.ziel }}</span>
            </div>
          </div>
        </div>

        <!-- Title & Description -->
        <div class="card p-6">
          <h1 class="text-3xl md:text-4xl font-bold mb-4">{{ trip.titel }}</h1>
          <p class="text-gray-600 text-lg leading-relaxed">{{ trip.beschreibung }}</p>
        </div>

        <!-- Highlights -->
        <div v-if="trip.highlights && trip.highlights.length > 0" class="card p-6">
          <h2 class="text-xl font-semibold mb-4 flex items-center">
            <svg class="w-6 h-6 mr-2 text-yellow-500" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
            </svg>
            Highlights
          </h2>
          <ul class="grid md:grid-cols-2 gap-3">
            <li v-for="(highlight, index) in trip.highlights" :key="index" class="flex items-start">
              <svg class="w-5 h-5 mr-2 text-green-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              <span>{{ highlight }}</span>
            </li>
          </ul>
        </div>

        <!-- Included / Not Included -->
        <div class="grid md:grid-cols-2 gap-6">
          <div v-if="trip.inkludiert && trip.inkludiert.length > 0" class="card p-6">
            <h2 class="text-xl font-semibold mb-4 text-green-600 flex items-center">
              <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              Inklusive
            </h2>
            <ul class="space-y-2">
              <li v-for="(item, index) in trip.inkludiert" :key="index" class="flex items-start">
                <svg class="w-5 h-5 mr-2 text-green-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
                <span>{{ item }}</span>
              </li>
            </ul>
          </div>

          <div v-if="trip.nicht_inkludiert && trip.nicht_inkludiert.length > 0" class="card p-6">
            <h2 class="text-xl font-semibold mb-4 text-red-600 flex items-center">
              <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              Nicht inklusive
            </h2>
            <ul class="space-y-2">
              <li v-for="(item, index) in trip.nicht_inkludiert" :key="index" class="flex items-start">
                <svg class="w-5 h-5 mr-2 text-red-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
                <span>{{ item }}</span>
              </li>
            </ul>
          </div>
        </div>

        <!-- Bus Info -->
        <div v-if="trip.bus_info" class="card p-6">
          <h2 class="text-xl font-semibold mb-4 flex items-center">
            <svg class="w-6 h-6 mr-2 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h8a2 2 0 012 2v10a2 2 0 01-2 2H8a2 2 0 01-2-2V9a2 2 0 012-2zM6 21v-2M18 21v-2M9 3v4M15 3v4M9 17h.01M15 17h.01" />
            </svg>
            Unser Reisebus
          </h2>
          <div class="flex flex-wrap gap-2">
            <span v-for="(ausstattung, index) in trip.bus_info.ausstattung" :key="index" class="badge badge-info">
              {{ ausstattung }}
            </span>
          </div>
        </div>
      </div>

      <!-- Sidebar -->
      <div class="lg:col-span-1">
        <div class="card p-6 sticky top-4">
          <!-- Price -->
          <div class="text-center mb-6 pb-6 border-b">
            <p class="text-gray-500 text-sm">Preis pro Person</p>
            <p class="text-4xl font-bold text-primary-600">{{ formatPrice(trip.preis_pro_person) }}</p>
          </div>

          <!-- Trip Details -->
          <div class="space-y-4 mb-6">
            <div class="flex items-start">
              <svg class="w-5 h-5 mr-3 text-gray-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              <div>
                <p class="text-sm text-gray-500">Abfahrt</p>
                <p class="font-medium">{{ formatDate(trip.abfahrt_datum) }}</p>
                <p class="text-sm text-gray-600">{{ formatTime(trip.abfahrt_zeit) }}</p>
              </div>
            </div>

            <div class="flex items-start">
              <svg class="w-5 h-5 mr-3 text-gray-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              <div>
                <p class="text-sm text-gray-500">Rückkehr</p>
                <p class="font-medium">{{ formatDate(trip.rueckkehr_datum) }}</p>
                <p class="text-sm text-gray-600">{{ formatTime(trip.rueckkehr_zeit) }}</p>
              </div>
            </div>

            <div class="flex items-start">
              <svg class="w-5 h-5 mr-3 text-gray-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
              </svg>
              <div>
                <p class="text-sm text-gray-500">Abfahrtsort</p>
                <p class="font-medium">{{ trip.abfahrtsort }}</p>
              </div>
            </div>

            <div class="flex items-start">
              <svg class="w-5 h-5 mr-3 text-gray-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              <div>
                <p class="text-sm text-gray-500">Verfügbare Plätze</p>
                <p class="font-medium">
                  <span :class="trip.verfuegbare_plaetze <= 5 ? 'text-red-600' : 'text-green-600'">
                    {{ trip.verfuegbare_plaetze }}
                  </span>
                  von {{ trip.max_teilnehmer }}
                </p>
              </div>
            </div>
          </div>

          <!-- Book Button -->
          <button
            v-if="isBookable"
            @click="handleBook"
            class="btn btn-primary w-full py-3 text-lg"
          >
            Jetzt buchen
          </button>
          
          <div v-else-if="trip.verfuegbare_plaetze === 0" class="text-center">
            <span class="badge bg-gray-800 text-white text-lg py-2 px-4">Ausgebucht</span>
            <p class="text-gray-500 text-sm mt-2">Diese Reise ist leider ausgebucht.</p>
          </div>

          <div v-else class="text-center">
            <span class="badge bg-gray-500 text-white text-lg py-2 px-4">Nicht verfügbar</span>
          </div>

          <!-- Login hint -->
          <p v-if="isBookable && !authStore.isAuthenticated" class="text-center text-sm text-gray-500 mt-3">
            <router-link :to="{ name: 'login', query: { redirect: route.fullPath } }" class="text-primary-600 hover:underline">
              Anmelden
            </router-link>
            oder
            <router-link :to="{ name: 'register' }" class="text-primary-600 hover:underline">
              registrieren
            </router-link>
            um zu buchen
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
