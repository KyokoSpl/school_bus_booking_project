// User Types
export interface User {
  id: string
  email: string
  vorname: string
  nachname: string
  telefon?: string
  strasse?: string
  plz?: string
  ort?: string
  land?: string
  geburtsdatum?: string
  rolle: 'kunde' | 'admin'
  email_verifiziert: boolean
  aktiv: boolean
  erstellt_am: string
}

export interface AuthResponse {
  token: string
  token_type: string
  expires_in: number
  user: User
}

export interface LoginRequest {
  email: string
  password: string
}

export interface RegisterRequest {
  email: string
  password: string
  vorname: string
  nachname: string
  telefon?: string
  agb_akzeptiert: boolean
  datenschutz_akzeptiert: boolean
}

export interface UpdateProfileRequest {
  vorname: string
  nachname: string
  telefon?: string
  strasse?: string
  plz?: string
  ort?: string
  land?: string
  geburtsdatum?: string
}

// Trip Types
export interface Trip {
  id: string
  titel: string
  beschreibung: string
  ziel: string
  abfahrtsort: string
  abfahrt_datum: string
  abfahrt_zeit: string
  rueckkehr_datum: string
  rueckkehr_zeit: string
  preis_pro_person: number
  bus_id?: string
  bus_info?: BusInfo
  max_teilnehmer: number
  aktuelle_buchungen: number
  verfuegbare_plaetze: number
  status: string
  bilder: string[]
  highlights: string[]
  inkludiert: string[]
  nicht_inkludiert: string[]
  erstellt_am: string
}

export interface TripListResponse {
  reisen: Trip[]
  gesamt: number
  seite: number
  pro_seite: number
  seiten_gesamt: number
}

export interface TripSearchParams {
  ziel?: string
  abfahrt_von?: string
  abfahrt_bis?: string
  preis_min?: number
  preis_max?: number
  nur_verfuegbar?: boolean
  sortierung?: 'preis_asc' | 'preis_desc' | 'datum_asc' | 'datum_desc'
  seite?: number
  pro_seite?: number
}

export interface CreateTripRequest {
  titel: string
  beschreibung: string
  ziel: string
  abfahrtsort: string
  abfahrt_datum: string
  abfahrt_zeit: string
  rueckkehr_datum: string
  rueckkehr_zeit: string
  preis_pro_person: number
  bus_id?: string
  max_teilnehmer: number
  status?: string
  bilder?: string[]
  highlights?: string[]
  inkludiert?: string[]
  nicht_inkludiert?: string[]
}

// Booking Types
export interface Mitreisender {
  vorname: string
  nachname: string
  geburtsdatum?: string
}

export interface Booking {
  id: string
  buchungsnummer: string
  kunde_id: string
  kunde_name?: string
  kunde_email?: string
  reise_id: string
  reise_titel?: string
  reise_ziel?: string
  reise_datum?: string
  anzahl_personen: number
  gesamtpreis: number
  status: string
  buchungsdatum: string
  mitreisende: Mitreisender[]
  bemerkungen?: string
  zahlungsstatus: string
  zahlungsmethode?: string
  erstellt_am: string
}

export interface BookingListResponse {
  buchungen: Booking[]
  gesamt: number
  seite: number
  pro_seite: number
  seiten_gesamt: number
}

export interface CreateBookingRequest {
  reise_id: string
  anzahl_personen: number
  mitreisende?: Mitreisender[]
  bemerkungen?: string
  zahlungsmethode: string
}

// Bus Types
export interface BusInfo {
  id: string
  kennzeichen: string
  sitzplaetze: number
  ausstattung: string[]
}

export interface Bus {
  id: string
  kennzeichen: string
  bezeichnung: string
  sitzplaetze: number
  ausstattung: string[]
  baujahr?: number
  status: string
  notizen?: string
  erstellt_am: string
}

export interface BusListResponse {
  busse: Bus[]
  gesamt: number
}

export interface CreateBusRequest {
  kennzeichen: string
  bezeichnung: string
  sitzplaetze: number
  ausstattung?: string[]
  baujahr?: number
  status?: string
  notizen?: string
}

// Payment Types
export interface Payment {
  id: string
  buchung_id: string
  betrag: number
  methode: string
  status: string
  transaktions_id?: string
  erstellt_am: string
}

export interface CreditCardInfo {
  kartennummer: string
  inhaber: string
  ablaufdatum: string
  cvv: string
}

export interface InitiatePaymentRequest {
  buchung_id: string
  methode: string
  kreditkarte?: CreditCardInfo
}

export interface PaymentConfirmation {
  erfolg: boolean
  transaktions_id: string
  nachricht: string
  zahlung: Payment
}

// Dashboard Types
export interface DashboardStats {
  benutzer: { gesamt: number }
  reisen: { gesamt: number; aktiv: number }
  buchungen: {
    gesamt: number
    ausstehend: number
    bestaetigt: number
    diesen_monat: number
  }
  umsatz: { bezahlt: number; ausstehend: number }
  busse: { gesamt: number; verfuegbar: number }
  letzte_buchungen: RecentBooking[]
  kommende_reisen: UpcomingTrip[]
}

export interface RecentBooking {
  buchungsnummer: string
  kunde_name: string
  reise_titel: string
  gesamtpreis: number
  status: string
  buchungsdatum: string
}

export interface UpcomingTrip {
  id: string
  titel: string
  ziel: string
  abfahrt_datum: string
  max_teilnehmer: number
  aktuelle_buchungen: number
  verfuegbar: number
}

// API Error
export interface ApiError {
  error: string
  message: string
}
