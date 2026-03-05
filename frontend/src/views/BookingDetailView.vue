<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { bookingsApi, paymentsApi } from '@/api'
import { useToast } from 'vue-toastification'
import type { Booking } from '@/types'

const route = useRoute()
const router = useRouter()
const toast = useToast()

const booking = ref<Booking | null>(null)
const loading = ref(true)
const cancelling = ref(false)
const paying = ref(false)
const showCancelModal = ref(false)
const cancelReason = ref('')

const canCancel = computed(() => {
  if (!booking.value) return false
  return ['ausstehend', 'bestaetigt'].includes(booking.value.status)
})

const canPay = computed(() => {
  if (!booking.value) return false
  return booking.value.zahlungsstatus === 'ausstehend' && booking.value.status !== 'storniert'
})

onMounted(async () => {
  await loadBooking()
})

async function loadBooking() {
  loading.value = true
  try {
    const response = await bookingsApi.get(route.params.id as string)
    booking.value = response.data
  } catch (err: any) {
    if (err.response?.status === 404) {
      toast.error('Buchung nicht gefunden')
      router.push({ name: 'bookings' })
    } else {
      toast.error('Fehler beim Laden der Buchung')
    }
  } finally {
    loading.value = false
  }
}

async function handleCancel() {
  if (!booking.value) return
  
  cancelling.value = true
  try {
    await bookingsApi.cancel(booking.value.id, cancelReason.value || undefined)
    toast.success('Buchung erfolgreich storniert')
    showCancelModal.value = false
    await loadBooking()
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Stornieren')
  } finally {
    cancelling.value = false
  }
}

async function handlePay() {
  if (!booking.value) return
  
  paying.value = true
  try {
    const response = await paymentsApi.initiate({
      buchung_id: booking.value.id,
      methode: 'rechnung'
    })
    
    if (response.data.erfolg) {
      toast.success('Zahlung erfolgreich!')
      await loadBooking()
    } else {
      toast.error(response.data.nachricht)
    }
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler bei der Zahlung')
  } finally {
    paying.value = false
  }
}

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString('de-DE', {
    weekday: 'long',
    day: '2-digit',
    month: 'long',
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

function getPaymentBadgeClass(status: string) {
  switch (status) {
    case 'bezahlt': return 'badge-success'
    case 'ausstehend': return 'badge-warning'
    case 'erstattet': return 'badge-info'
    default: return 'badge-secondary'
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
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <!-- Back button -->
    <router-link :to="{ name: 'bookings' }" class="btn btn-secondary mb-6">
      <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
      Zurück zu meinen Buchungen
    </router-link>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-12">
      <div class="spinner"></div>
    </div>

    <!-- Booking Details -->
    <div v-else-if="booking" class="grid lg:grid-cols-3 gap-8">
      <!-- Main Content -->
      <div class="lg:col-span-2 space-y-6">
        <!-- Header -->
        <div class="card p-6">
          <div class="flex flex-wrap items-start justify-between gap-4 mb-4">
            <div>
              <div class="flex items-center gap-3 mb-2">
                <span class="badge" :class="getStatusBadgeClass(booking.status)">
                  {{ getStatusLabel(booking.status) }}
                </span>
                <span class="badge" :class="getPaymentBadgeClass(booking.zahlungsstatus)">
                  {{ getPaymentStatusLabel(booking.zahlungsstatus) }}
                </span>
              </div>
              <h1 class="text-2xl md:text-3xl font-bold dark:text-gray-100">{{ booking.reise_titel }}</h1>
              <p class="text-gray-600 dark:text-gray-400">{{ booking.reise_ziel }}</p>
            </div>
            <div class="text-right">
              <p class="text-sm text-gray-500 dark:text-gray-400">Buchungsnummer</p>
              <p class="text-xl font-mono font-bold dark:text-gray-100">{{ booking.buchungsnummer }}</p>
            </div>
          </div>
        </div>

        <!-- Trip Details -->
        <div class="card p-6">
          <h2 class="text-xl font-semibold mb-4 dark:text-gray-100">Reisedetails</h2>
          <div class="grid md:grid-cols-2 gap-4">
            <div class="flex items-start">
              <svg class="w-5 h-5 mr-3 text-gray-400 dark:text-gray-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400">Reisedatum</p>
                <p class="font-medium dark:text-gray-100">{{ formatDate(booking.reise_datum || booking.erstellt_am) }}</p>
              </div>
            </div>
            <div class="flex items-start">
              <svg class="w-5 h-5 mr-3 text-gray-400 dark:text-gray-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400">Anzahl Personen</p>
                <p class="font-medium dark:text-gray-100">{{ booking.anzahl_personen }} Person(en)</p>
              </div>
            </div>
            <div class="flex items-start">
              <svg class="w-5 h-5 mr-3 text-gray-400 dark:text-gray-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400">Gebucht am</p>
                <p class="font-medium dark:text-gray-100">{{ formatDate(booking.buchungsdatum) }}</p>
              </div>
            </div>
            <div v-if="booking.zahlungsmethode" class="flex items-start">
              <svg class="w-5 h-5 mr-3 text-gray-400 dark:text-gray-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
              </svg>
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400">Zahlungsmethode</p>
                <p class="font-medium capitalize dark:text-gray-100">{{ booking.zahlungsmethode }}</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Travelers -->
        <div v-if="booking.mitreisende && booking.mitreisende.length > 0" class="card p-6">
          <h2 class="text-xl font-semibold mb-4 dark:text-gray-100">Mitreisende</h2>
          <div class="space-y-3">
            <div
              v-for="(person, index) in booking.mitreisende"
              :key="index"
              class="flex items-center p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg"
            >
              <div class="w-10 h-10 bg-primary-100 dark:bg-primary-900/50 rounded-full flex items-center justify-center mr-3">
                <svg class="w-5 h-5 text-primary-600 dark:text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
              </div>
              <div>
                <p class="font-medium dark:text-gray-100">{{ person.vorname }} {{ person.nachname }}</p>
                <p v-if="person.geburtsdatum" class="text-sm text-gray-500 dark:text-gray-400">
                  Geb.: {{ formatDate(person.geburtsdatum) }}
                </p>
              </div>
            </div>
          </div>
        </div>

        <!-- Notes -->
        <div v-if="booking.bemerkungen" class="card p-6">
          <h2 class="text-xl font-semibold mb-4 dark:text-gray-100">Bemerkungen</h2>
          <p class="text-gray-600 dark:text-gray-400">{{ booking.bemerkungen }}</p>
        </div>
      </div>

      <!-- Sidebar -->
      <div class="lg:col-span-1">
        <div class="card p-6 sticky top-4">
          <!-- Price Summary -->
          <h3 class="font-semibold mb-4 dark:text-gray-100">Preisübersicht</h3>
          <div class="space-y-2 mb-4">
            <div class="flex justify-between">
              <span class="text-gray-600 dark:text-gray-400">{{ booking.anzahl_personen }}x Teilnehmer</span>
              <span class="dark:text-gray-100">{{ formatPrice(booking.gesamtpreis) }}</span>
            </div>
          </div>
          <div class="border-t dark:border-gray-700 pt-4 mb-6">
            <div class="flex justify-between text-xl font-bold">
              <span class="dark:text-gray-100">Gesamt</span>
              <span class="text-primary-600 dark:text-primary-400">{{ formatPrice(booking.gesamtpreis) }}</span>
            </div>
          </div>

          <!-- Actions -->
          <div class="space-y-3">
            <button
              v-if="canPay"
              @click="handlePay"
              :disabled="paying"
              class="w-full btn btn-primary"
            >
              <span v-if="paying" class="spinner mr-2"></span>
              {{ paying ? 'Verarbeite...' : 'Jetzt bezahlen' }}
            </button>

            <router-link
              :to="{ name: 'trip-detail', params: { id: booking.reise_id } }"
              class="w-full btn btn-outline block text-center"
            >
              Reise ansehen
            </router-link>

            <button
              v-if="canCancel"
              @click="showCancelModal = true"
              class="w-full btn btn-danger"
            >
              Buchung stornieren
            </button>
          </div>

          <!-- Help -->
          <div class="mt-6 p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
            <p class="text-sm text-gray-600 dark:text-gray-400">
              <strong class="dark:text-gray-300">Fragen?</strong><br>
              Kontaktieren Sie uns unter<br>
              <a href="mailto:info@sonnenschein-reisen.de" class="text-primary-600 dark:text-primary-400 hover:underline">
                info@sonnenschein-reisen.de
              </a>
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Cancel Modal -->
    <div v-if="showCancelModal" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
      <div class="bg-white dark:bg-gray-800 rounded-xl max-w-md w-full p-6">
        <h3 class="text-xl font-bold mb-4 dark:text-gray-100">Buchung stornieren</h3>
        <p class="text-gray-600 dark:text-gray-400 mb-4">
          Möchten Sie diese Buchung wirklich stornieren? Diese Aktion kann nicht rückgängig gemacht werden.
        </p>
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            Stornierungsgrund (optional)
          </label>
          <textarea
            v-model="cancelReason"
            rows="3"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
            placeholder="Warum möchten Sie stornieren?"
          ></textarea>
        </div>
        <div class="flex gap-3">
          <button @click="showCancelModal = false" class="flex-1 btn btn-secondary">
            Abbrechen
          </button>
          <button
            @click="handleCancel"
            :disabled="cancelling"
            class="flex-1 btn btn-danger"
          >
            <span v-if="cancelling" class="spinner mr-2"></span>
            {{ cancelling ? 'Storniere...' : 'Stornieren' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>