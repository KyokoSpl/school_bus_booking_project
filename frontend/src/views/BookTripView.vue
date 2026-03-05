<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { tripsApi, bookingsApi, paymentsApi } from '@/api'
import { useAuthStore } from '@/stores/auth'
import { useToast } from 'vue-toastification'
import type { Trip, Mitreisender, Booking } from '@/types'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()
const toast = useToast()

const trip = ref<Trip | null>(null)
const loading = ref(true)
const submitting = ref(false)
const step = ref(1)
const createdBooking = ref<Booking | null>(null)

const form = ref({
  anzahl_personen: 1,
  mitreisende: [] as Mitreisender[],
  bemerkungen: '',
  zahlungsmethode: 'rechnung'
})

// Add main booker as first traveler
const allTravelers = computed(() => {
  const mainTraveler: Mitreisender = {
    vorname: authStore.user?.vorname || '',
    nachname: authStore.user?.nachname || '',
    geburtsdatum: authStore.user?.geburtsdatum || ''
  }
  return [mainTraveler, ...form.value.mitreisende]
})

const totalPrice = computed(() => {
  if (!trip.value) return 0
  return trip.value.preis_pro_person * form.value.anzahl_personen
})

const maxPersons = computed(() => {
  if (!trip.value) return 1
  return Math.min(trip.value.verfuegbare_plaetze, 10)
})

onMounted(async () => {
  await loadTrip()
})

// Watch person count to adjust travelers array
watch(() => form.value.anzahl_personen, (newCount) => {
  const additionalCount = newCount - 1
  while (form.value.mitreisende.length < additionalCount) {
    form.value.mitreisende.push({ vorname: '', nachname: '', geburtsdatum: '' })
  }
  while (form.value.mitreisende.length > additionalCount) {
    form.value.mitreisende.pop()
  }
})

async function loadTrip() {
  loading.value = true
  try {
    const response = await tripsApi.get(route.params.tripId as string)
    trip.value = response.data
    
    if (trip.value.verfuegbare_plaetze === 0) {
      toast.error('Diese Reise ist leider ausgebucht')
      router.push({ name: 'trip-detail', params: { id: trip.value.id } })
    }
  } catch (err: any) {
    toast.error('Reise konnte nicht geladen werden')
    router.push({ name: 'trips' })
  } finally {
    loading.value = false
  }
}

function nextStep() {
  if (step.value === 1) {
    // Validate travelers
    const valid = form.value.mitreisende.every(t => t.vorname && t.nachname)
    if (!valid) {
      toast.error('Bitte füllen Sie alle Pflichtfelder für die Mitreisenden aus')
      return
    }
  }
  step.value++
}

function prevStep() {
  step.value--
}

async function handleSubmit() {
  if (!trip.value) return

  submitting.value = true
  try {
    // Create booking
    const response = await bookingsApi.create({
      reise_id: trip.value.id,
      anzahl_personen: form.value.anzahl_personen,
      mitreisende: form.value.mitreisende.length > 0 ? form.value.mitreisende : undefined,
      bemerkungen: form.value.bemerkungen || undefined,
      zahlungsmethode: form.value.zahlungsmethode
    })
    
    createdBooking.value = response.data

    // Process payment if not invoice
    if (form.value.zahlungsmethode !== 'rechnung') {
      await paymentsApi.initiate({
        buchung_id: response.data.id,
        methode: form.value.zahlungsmethode
      })
    }

    step.value = 4 // Success step
    toast.success('Buchung erfolgreich erstellt!')
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Buchung fehlgeschlagen')
  } finally {
    submitting.value = false
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
    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-12">
      <div class="spinner"></div>
    </div>

    <template v-else-if="trip">
      <!-- Progress Steps -->
      <div class="max-w-3xl mx-auto mb-8">
        <div class="flex items-start">
          <!-- Step 1 -->
          <div class="flex flex-col items-center">
            <div
              class="w-10 h-10 rounded-full flex items-center justify-center font-bold"
              :class="step >= 1 ? 'bg-primary-600 text-white' : 'bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400'"
            >
              <svg v-if="step > 1" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              <span v-else>1</span>
            </div>
            <span class="mt-2 text-sm" :class="step >= 1 ? 'text-primary-600 dark:text-primary-400 font-medium' : 'text-gray-500 dark:text-gray-400'">Teilnehmer</span>
          </div>
          <!-- Line 1-2 -->
          <div class="flex-1 h-1 mt-5 mx-2" :class="step > 1 ? 'bg-primary-600' : 'bg-gray-200 dark:bg-gray-700'"></div>
          <!-- Step 2 -->
          <div class="flex flex-col items-center">
            <div
              class="w-10 h-10 rounded-full flex items-center justify-center font-bold"
              :class="step >= 2 ? 'bg-primary-600 text-white' : 'bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400'"
            >
              <svg v-if="step > 2" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              <span v-else>2</span>
            </div>
            <span class="mt-2 text-sm" :class="step >= 2 ? 'text-primary-600 dark:text-primary-400 font-medium' : 'text-gray-500 dark:text-gray-400'">Zahlung</span>
          </div>
          <!-- Line 2-3 -->
          <div class="flex-1 h-1 mt-5 mx-2" :class="step > 2 ? 'bg-primary-600' : 'bg-gray-200 dark:bg-gray-700'"></div>
          <!-- Step 3 -->
          <div class="flex flex-col items-center">
            <div
              class="w-10 h-10 rounded-full flex items-center justify-center font-bold"
              :class="step >= 3 ? 'bg-primary-600 text-white' : 'bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400'"
            >
              <svg v-if="step > 3" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              <span v-else>3</span>
            </div>
            <span class="mt-2 text-sm" :class="step >= 3 ? 'text-primary-600 dark:text-primary-400 font-medium' : 'text-gray-500 dark:text-gray-400'">Bestätigung</span>
          </div>
        </div>
      </div>

      <div class="grid lg:grid-cols-3 gap-8">
        <!-- Main Form -->
        <div :class="step === 4 ? 'lg:col-span-3' : 'lg:col-span-2'">
          <!-- Step 1: Travelers -->
          <div v-if="step === 1" class="card p-6">
            <h2 class="text-2xl font-bold mb-6 dark:text-gray-100">Teilnehmer</h2>

            <div class="mb-6">
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Anzahl Personen</label>
              <select
                v-model="form.anzahl_personen"
                class="w-full md:w-48 px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
              >
                <option v-for="n in maxPersons" :key="n" :value="n">{{ n }} Person(en)</option>
              </select>
            </div>

            <!-- Main Booker -->
            <div class="mb-6 p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
              <h3 class="font-semibold mb-3 dark:text-gray-100">Hauptbucher (Sie)</h3>
              <div class="grid md:grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm text-gray-600 dark:text-gray-400">Vorname</label>
                  <p class="font-medium dark:text-gray-100">{{ authStore.user?.vorname }}</p>
                </div>
                <div>
                  <label class="block text-sm text-gray-600 dark:text-gray-400">Nachname</label>
                  <p class="font-medium dark:text-gray-100">{{ authStore.user?.nachname }}</p>
                </div>
                <div>
                  <label class="block text-sm text-gray-600 dark:text-gray-400">E-Mail</label>
                  <p class="font-medium dark:text-gray-100">{{ authStore.user?.email }}</p>
                </div>
              </div>
            </div>

            <!-- Additional Travelers -->
            <div v-if="form.mitreisende.length > 0" class="space-y-4">
              <h3 class="font-semibold dark:text-gray-100">Mitreisende</h3>
              <div
                v-for="(traveler, index) in form.mitreisende"
                :key="index"
                class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg"
              >
                <h4 class="font-medium mb-3 dark:text-gray-100">Person {{ index + 2 }}</h4>
                <div class="grid md:grid-cols-3 gap-4">
                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Vorname *</label>
                    <input
                      v-model="traveler.vorname"
                      type="text"
                      required
                      class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
                    />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nachname *</label>
                    <input
                      v-model="traveler.nachname"
                      type="text"
                      required
                      class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
                    />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Geburtsdatum</label>
                    <input
                      v-model="traveler.geburtsdatum"
                      type="date"
                      class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100"
                    />
                  </div>
                </div>
              </div>
            </div>

            <!-- Notes -->
            <div class="mt-6">
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Bemerkungen (optional)</label>
              <textarea
                v-model="form.bemerkungen"
                rows="3"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-100 dark:placeholder-gray-400"
                placeholder="Besondere Wünsche oder Hinweise..."
              ></textarea>
            </div>

            <div class="mt-6 flex justify-end">
              <button @click="nextStep" class="btn btn-primary">
                Weiter zur Zahlung
                <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Step 2: Payment -->
          <div v-if="step === 2" class="card p-6">
            <h2 class="text-2xl font-bold mb-6 dark:text-gray-100">Zahlungsmethode</h2>

            <div class="space-y-4">
              <label class="flex items-start p-4 border dark:border-gray-700 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
                :class="{ 'border-primary-500 bg-primary-50 dark:bg-primary-900/30 dark:border-primary-500': form.zahlungsmethode === 'rechnung' }"
              >
                <input
                  v-model="form.zahlungsmethode"
                  type="radio"
                  value="rechnung"
                  class="mt-1 text-primary-600"
                />
                <div class="ml-3">
                  <p class="font-semibold dark:text-gray-100">Rechnung</p>
                  <p class="text-sm text-gray-600 dark:text-gray-400">Zahlung nach Erhalt der Rechnung per Überweisung</p>
                </div>
              </label>

              <label class="flex items-start p-4 border dark:border-gray-700 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
                :class="{ 'border-primary-500 bg-primary-50 dark:bg-primary-900/30 dark:border-primary-500': form.zahlungsmethode === 'kreditkarte' }"
              >
                <input
                  v-model="form.zahlungsmethode"
                  type="radio"
                  value="kreditkarte"
                  class="mt-1 text-primary-600"
                />
                <div class="ml-3">
                  <p class="font-semibold dark:text-gray-100">Kreditkarte</p>
                  <p class="text-sm text-gray-600 dark:text-gray-400">Sichere Sofortzahlung mit Visa, Mastercard oder AmEx (simuliert)</p>
                </div>
              </label>

              <label class="flex items-start p-4 border dark:border-gray-700 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
                :class="{ 'border-primary-500 bg-primary-50 dark:bg-primary-900/30 dark:border-primary-500': form.zahlungsmethode === 'paypal' }"
              >
                <input
                  v-model="form.zahlungsmethode"
                  type="radio"
                  value="paypal"
                  class="mt-1 text-primary-600"
                />
                <div class="ml-3">
                  <p class="font-semibold dark:text-gray-100">PayPal</p>
                  <p class="text-sm text-gray-600 dark:text-gray-400">Schnelle Zahlung über Ihr PayPal-Konto (simuliert)</p>
                </div>
              </label>
            </div>

            <div class="mt-6 flex justify-between">
              <button @click="prevStep" class="btn btn-secondary">
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                </svg>
                Zurück
              </button>
              <button @click="nextStep" class="btn btn-primary">
                Weiter zur Zusammenfassung
                <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Step 3: Confirmation -->
          <div v-if="step === 3" class="card p-6">
            <h2 class="text-2xl font-bold mb-6 dark:text-gray-100">Buchung bestätigen</h2>

            <div class="space-y-6">
              <!-- Trip Info -->
              <div class="p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
                <h3 class="font-semibold mb-2 dark:text-gray-100">Reise</h3>
                <p class="text-lg font-medium dark:text-gray-100">{{ trip.titel }}</p>
                <p class="text-gray-600 dark:text-gray-400">{{ trip.ziel }}</p>
                <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">{{ formatDate(trip.abfahrt_datum) }}, {{ formatTime(trip.abfahrt_zeit) }}</p>
              </div>

              <!-- Travelers -->
              <div>
                <h3 class="font-semibold mb-2 dark:text-gray-100">Teilnehmer</h3>
                <ul class="space-y-1">
                  <li v-for="(traveler, index) in allTravelers" :key="index" class="text-gray-600 dark:text-gray-400">
                    {{ traveler.vorname }} {{ traveler.nachname }}
                  </li>
                </ul>
              </div>

              <!-- Payment Method -->
              <div>
                <h3 class="font-semibold mb-2 dark:text-gray-100">Zahlungsmethode</h3>
                <p class="text-gray-600 dark:text-gray-400 capitalize">{{ form.zahlungsmethode }}</p>
              </div>

              <!-- Notes -->
              <div v-if="form.bemerkungen">
                <h3 class="font-semibold mb-2 dark:text-gray-100">Bemerkungen</h3>
                <p class="text-gray-600 dark:text-gray-400">{{ form.bemerkungen }}</p>
              </div>

              <!-- Agreement -->
              <div class="p-4 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg">
                <p class="text-sm text-yellow-800 dark:text-yellow-300">
                  Mit der Buchung akzeptieren Sie unsere
                  <router-link :to="{ name: 'terms' }" target="_blank" class="underline">AGB</router-link>
                  und
                  <router-link :to="{ name: 'privacy' }" target="_blank" class="underline">Datenschutzerklärung</router-link>.
                </p>
              </div>
            </div>

            <div class="mt-6 flex justify-between">
              <button @click="prevStep" class="btn btn-secondary">
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                </svg>
                Zurück
              </button>
              <button @click="handleSubmit" :disabled="submitting" class="btn btn-primary">
                <span v-if="submitting" class="spinner mr-2"></span>
                {{ submitting ? 'Buchung wird erstellt...' : 'Jetzt verbindlich buchen' }}
              </button>
            </div>
          </div>

          <!-- Step 4: Success -->
          <div v-if="step === 4" class="card p-8 text-center">
            <div class="w-20 h-20 mx-auto mb-6 bg-green-100 dark:bg-green-900/50 rounded-full flex items-center justify-center">
              <svg class="w-10 h-10 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <h2 class="text-2xl font-bold mb-4 dark:text-gray-100">Buchung erfolgreich!</h2>
            <p class="text-gray-600 dark:text-gray-400 mb-2">
              Ihre Buchungsnummer: <strong class="font-mono text-lg dark:text-gray-100">{{ createdBooking?.buchungsnummer }}</strong>
            </p>
            <p class="text-gray-600 dark:text-gray-400 mb-6">
              Eine Bestätigung wurde an <strong class="dark:text-gray-100">{{ authStore.user?.email }}</strong> gesendet.
            </p>
            <div class="flex flex-col sm:flex-row gap-3 justify-center">
              <router-link :to="{ name: 'booking-detail', params: { id: createdBooking?.id } }" class="btn btn-primary">
                Buchung ansehen
              </router-link>
              <router-link :to="{ name: 'bookings' }" class="btn btn-secondary">
                Alle Buchungen
              </router-link>
            </div>
          </div>
        </div>

        <!-- Sidebar -->
        <div v-if="step < 4" class="lg:col-span-1">
          <div class="card p-6 sticky top-4">
            <h3 class="font-semibold mb-4 dark:text-gray-100">Zusammenfassung</h3>

            <!-- Trip -->
            <div class="mb-4 pb-4 border-b dark:border-gray-700">
              <p class="font-medium dark:text-gray-100">{{ trip.titel }}</p>
              <p class="text-sm text-gray-600 dark:text-gray-400">{{ trip.ziel }}</p>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">{{ formatDate(trip.abfahrt_datum) }}</p>
            </div>

            <!-- Price -->
            <div class="space-y-2 mb-4">
              <div class="flex justify-between text-sm">
                <span class="text-gray-600 dark:text-gray-400">{{ form.anzahl_personen }}x Teilnehmer</span>
                <span class="dark:text-gray-100">{{ formatPrice(trip.preis_pro_person * form.anzahl_personen) }}</span>
              </div>
            </div>

            <div class="border-t dark:border-gray-700 pt-4">
              <div class="flex justify-between text-xl font-bold">
                <span class="dark:text-gray-100">Gesamt</span>
                <span class="text-primary-600 dark:text-primary-400">{{ formatPrice(totalPrice) }}</span>
              </div>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">inkl. MwSt.</p>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
