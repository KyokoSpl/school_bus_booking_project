<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { adminApi } from '@/api'
import { useToast } from 'vue-toastification'
import type { Trip, Bus, CreateTripRequest } from '@/types'

const route = useRoute()
const router = useRouter()
const toast = useToast()

const isEdit = computed(() => !!route.params.id)
const loading = ref(true)
const saving = ref(false)
const buses = ref<Bus[]>([])

const form = ref<CreateTripRequest>({
  titel: '',
  beschreibung: '',
  ziel: '',
  abfahrtsort: 'ZOB Musterstadt',
  abfahrt_datum: '',
  abfahrt_zeit: '08:00',
  rueckkehr_datum: '',
  rueckkehr_zeit: '18:00',
  preis_pro_person: 0,
  bus_id: undefined,
  max_teilnehmer: 50,
  status: 'entwurf',
  bilder: [],
  highlights: [],
  inkludiert: [],
  nicht_inkludiert: []
})

// Helper for arrays as comma-separated strings
const highlightsText = ref('')
const inkludiertText = ref('')
const nichtInkludiertText = ref('')

onMounted(async () => {
  try {
    // Load buses
    const busResponse = await adminApi.listBuses()
    buses.value = busResponse.data.busse

    // Load trip if editing
    if (isEdit.value) {
      const response = await adminApi.listTrips()
      const trip = response.data.reisen.find((t: Trip) => t.id === route.params.id)
      if (trip) {
        form.value = {
          titel: trip.titel,
          beschreibung: trip.beschreibung,
          ziel: trip.ziel,
          abfahrtsort: trip.abfahrtsort,
          abfahrt_datum: trip.abfahrt_datum,
          abfahrt_zeit: trip.abfahrt_zeit,
          rueckkehr_datum: trip.rueckkehr_datum,
          rueckkehr_zeit: trip.rueckkehr_zeit,
          preis_pro_person: trip.preis_pro_person,
          bus_id: trip.bus_info?.id,
          max_teilnehmer: trip.max_teilnehmer,
          status: trip.status,
          bilder: trip.bilder || [],
          highlights: trip.highlights || [],
          inkludiert: trip.inkludiert || [],
          nicht_inkludiert: trip.nicht_inkludiert || []
        }
        highlightsText.value = form.value.highlights?.join('\n') || ''
        inkludiertText.value = form.value.inkludiert?.join('\n') || ''
        nichtInkludiertText.value = form.value.nicht_inkludiert?.join('\n') || ''
      } else {
        toast.error('Reise nicht gefunden')
        router.push({ name: 'admin-trips' })
      }
    }
  } catch (error) {
    toast.error('Fehler beim Laden')
  } finally {
    loading.value = false
  }
})

async function handleSubmit() {
  // Parse arrays from text
  form.value.highlights = highlightsText.value.split('\n').filter(s => s.trim())
  form.value.inkludiert = inkludiertText.value.split('\n').filter(s => s.trim())
  form.value.nicht_inkludiert = nichtInkludiertText.value.split('\n').filter(s => s.trim())

  saving.value = true
  try {
    if (isEdit.value) {
      await adminApi.updateTrip(route.params.id as string, form.value)
      toast.success('Reise erfolgreich aktualisiert')
    } else {
      await adminApi.createTrip(form.value)
      toast.success('Reise erfolgreich erstellt')
    }
    router.push({ name: 'admin-trips' })
  } catch (err: any) {
    toast.error(err.response?.data?.message || 'Fehler beim Speichern')
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <div class="max-w-3xl">
    <div class="mb-6">
      <router-link :to="{ name: 'admin-trips' }" class="text-primary-600 hover:underline">
        &larr; Zurück zur Übersicht
      </router-link>
    </div>

    <h2 class="text-2xl font-bold mb-6">{{ isEdit ? 'Reise bearbeiten' : 'Neue Reise erstellen' }}</h2>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-12">
      <div class="spinner"></div>
    </div>

    <!-- Form -->
    <form v-else @submit.prevent="handleSubmit" class="space-y-6">
      <div class="card p-6 space-y-4">
        <h3 class="font-semibold border-b pb-2">Grunddaten</h3>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Titel *</label>
          <input v-model="form.titel" type="text" required class="w-full px-3 py-2 border rounded-lg" />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Beschreibung *</label>
          <textarea v-model="form.beschreibung" rows="4" required class="w-full px-3 py-2 border rounded-lg"></textarea>
        </div>

        <div class="grid md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Ziel *</label>
            <input v-model="form.ziel" type="text" required class="w-full px-3 py-2 border rounded-lg" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Abfahrtsort *</label>
            <input v-model="form.abfahrtsort" type="text" required class="w-full px-3 py-2 border rounded-lg" />
          </div>
        </div>
      </div>

      <div class="card p-6 space-y-4">
        <h3 class="font-semibold border-b pb-2">Termine</h3>

        <div class="grid md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Abfahrt Datum *</label>
            <input v-model="form.abfahrt_datum" type="date" required class="w-full px-3 py-2 border rounded-lg" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Abfahrt Zeit *</label>
            <input v-model="form.abfahrt_zeit" type="time" required class="w-full px-3 py-2 border rounded-lg" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Rückkehr Datum *</label>
            <input v-model="form.rueckkehr_datum" type="date" required class="w-full px-3 py-2 border rounded-lg" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Rückkehr Zeit *</label>
            <input v-model="form.rueckkehr_zeit" type="time" required class="w-full px-3 py-2 border rounded-lg" />
          </div>
        </div>
      </div>

      <div class="card p-6 space-y-4">
        <h3 class="font-semibold border-b pb-2">Preis & Kapazität</h3>

        <div class="grid md:grid-cols-3 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Preis pro Person (€) *</label>
            <input v-model.number="form.preis_pro_person" type="number" min="0" step="0.01" required class="w-full px-3 py-2 border rounded-lg" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Max. Teilnehmer *</label>
            <input v-model.number="form.max_teilnehmer" type="number" min="1" required class="w-full px-3 py-2 border rounded-lg" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Bus</label>
            <select v-model="form.bus_id" class="w-full px-3 py-2 border rounded-lg">
              <option :value="undefined">-- Kein Bus zugewiesen --</option>
              <option v-for="bus in buses" :key="bus.id" :value="bus.id">
                {{ bus.bezeichnung }} ({{ bus.sitzplaetze }} Sitze)
              </option>
            </select>
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Status</label>
          <select v-model="form.status" class="w-full md:w-48 px-3 py-2 border rounded-lg">
            <option value="entwurf">Entwurf</option>
            <option value="aktiv">Aktiv</option>
            <option value="abgesagt">Abgesagt</option>
          </select>
        </div>
      </div>

      <div class="card p-6 space-y-4">
        <h3 class="font-semibold border-b pb-2">Details (je Zeile ein Punkt)</h3>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Highlights</label>
          <textarea v-model="highlightsText" rows="4" class="w-full px-3 py-2 border rounded-lg" placeholder="Highlight 1&#10;Highlight 2&#10;Highlight 3"></textarea>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Inkludiert</label>
          <textarea v-model="inkludiertText" rows="4" class="w-full px-3 py-2 border rounded-lg" placeholder="Busfahrt&#10;Reiseleitung&#10;Eintritt"></textarea>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Nicht inkludiert</label>
          <textarea v-model="nichtInkludiertText" rows="4" class="w-full px-3 py-2 border rounded-lg" placeholder="Verpflegung&#10;Trinkgelder"></textarea>
        </div>
      </div>

      <div class="flex gap-4">
        <button type="submit" :disabled="saving" class="btn btn-primary">
          <span v-if="saving" class="spinner mr-2"></span>
          {{ saving ? 'Speichern...' : (isEdit ? 'Änderungen speichern' : 'Reise erstellen') }}
        </button>
        <router-link :to="{ name: 'admin-trips' }" class="btn btn-secondary">
          Abbrechen
        </router-link>
      </div>
    </form>
  </div>
</template>
