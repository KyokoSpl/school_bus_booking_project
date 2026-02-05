use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// Reise-Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TripStatus {
    Geplant,
    Aktiv,
    Ausgebucht,
    Abgeschlossen,
    Storniert,
}

impl Default for TripStatus {
    fn default() -> Self {
        TripStatus::Geplant
    }
}

/// Reise-Entität aus der Datenbank
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Trip {
    pub id: String,
    pub titel: String,
    pub beschreibung: String,
    pub ziel: String,
    pub abfahrtsort: String,
    pub abfahrt_datum: NaiveDate,
    pub abfahrt_zeit: NaiveTime,
    pub rueckkehr_datum: NaiveDate,
    pub rueckkehr_zeit: NaiveTime,
    pub preis_pro_person: f64,
    pub bus_id: Option<String>,
    pub max_teilnehmer: i32,
    pub aktuelle_buchungen: i32,
    pub status: String,
    pub bilder: Option<String>,  // JSON-Array als String
    pub highlights: Option<String>,
    pub inkludiert: Option<String>,
    pub nicht_inkludiert: Option<String>,
    pub erstellt_am: DateTime<Utc>,
    pub aktualisiert_am: DateTime<Utc>,
}

impl Trip {
    pub fn verfuegbare_plaetze(&self) -> i32 {
        self.max_teilnehmer - self.aktuelle_buchungen
    }
    
    pub fn ist_verfuegbar(&self) -> bool {
        self.verfuegbare_plaetze() > 0 && self.status == "aktiv"
    }
}

/// Reise-Response mit zusätzlichen berechneten Feldern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripResponse {
    pub id: String,
    pub titel: String,
    pub beschreibung: String,
    pub ziel: String,
    pub abfahrtsort: String,
    pub abfahrt_datum: NaiveDate,
    pub abfahrt_zeit: NaiveTime,
    pub rueckkehr_datum: NaiveDate,
    pub rueckkehr_zeit: NaiveTime,
    pub preis_pro_person: f64,
    pub bus_id: Option<String>,
    pub bus_info: Option<BusInfo>,
    pub max_teilnehmer: i32,
    pub aktuelle_buchungen: i32,
    pub verfuegbare_plaetze: i32,
    pub status: String,
    pub bilder: Vec<String>,
    pub highlights: Vec<String>,
    pub inkludiert: Vec<String>,
    pub nicht_inkludiert: Vec<String>,
    pub erstellt_am: DateTime<Utc>,
}

/// Bus-Kurzinfo für Reise-Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusInfo {
    pub id: String,
    pub kennzeichen: String,
    pub sitzplaetze: i32,
    pub ausstattung: Vec<String>,
}

impl From<Trip> for TripResponse {
    fn from(trip: Trip) -> Self {
        TripResponse {
            id: trip.id,
            titel: trip.titel,
            beschreibung: trip.beschreibung,
            ziel: trip.ziel,
            abfahrtsort: trip.abfahrtsort,
            abfahrt_datum: trip.abfahrt_datum,
            abfahrt_zeit: trip.abfahrt_zeit,
            rueckkehr_datum: trip.rueckkehr_datum,
            rueckkehr_zeit: trip.rueckkehr_zeit,
            preis_pro_person: trip.preis_pro_person,
            bus_id: trip.bus_id,
            bus_info: None,
            max_teilnehmer: trip.max_teilnehmer,
            aktuelle_buchungen: trip.aktuelle_buchungen,
            verfuegbare_plaetze: trip.max_teilnehmer - trip.aktuelle_buchungen,
            status: trip.status,
            bilder: trip.bilder
                .map(|s| serde_json::from_str(&s).unwrap_or_default())
                .unwrap_or_default(),
            highlights: trip.highlights
                .map(|s| serde_json::from_str(&s).unwrap_or_default())
                .unwrap_or_default(),
            inkludiert: trip.inkludiert
                .map(|s| serde_json::from_str(&s).unwrap_or_default())
                .unwrap_or_default(),
            nicht_inkludiert: trip.nicht_inkludiert
                .map(|s| serde_json::from_str(&s).unwrap_or_default())
                .unwrap_or_default(),
            erstellt_am: trip.erstellt_am,
        }
    }
}

/// Reise erstellen/aktualisieren Request
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTripRequest {
    #[validate(length(min = 3, max = 255, message = "Titel muss zwischen 3 und 255 Zeichen lang sein"))]
    pub titel: String,
    
    #[validate(length(min = 10, message = "Beschreibung muss mindestens 10 Zeichen lang sein"))]
    pub beschreibung: String,
    
    #[validate(length(min = 2, message = "Ziel ist erforderlich"))]
    pub ziel: String,
    
    #[validate(length(min = 2, message = "Abfahrtsort ist erforderlich"))]
    pub abfahrtsort: String,
    
    pub abfahrt_datum: NaiveDate,
    pub abfahrt_zeit: NaiveTime,
    pub rueckkehr_datum: NaiveDate,
    pub rueckkehr_zeit: NaiveTime,
    
    #[validate(range(min = 1.0, message = "Preis muss positiv sein"))]
    pub preis_pro_person: f64,
    
    pub bus_id: Option<String>,
    
    #[validate(range(min = 1, message = "Mindestens 1 Teilnehmer erforderlich"))]
    pub max_teilnehmer: i32,
    
    pub status: Option<String>,
    pub bilder: Option<Vec<String>>,
    pub highlights: Option<Vec<String>>,
    pub inkludiert: Option<Vec<String>>,
    pub nicht_inkludiert: Option<Vec<String>>,
}

/// Reise-Suchfilter
#[derive(Debug, Clone, Deserialize, Default)]
pub struct TripSearchParams {
    pub ziel: Option<String>,
    pub abfahrt_von: Option<NaiveDate>,
    pub abfahrt_bis: Option<NaiveDate>,
    pub preis_min: Option<f64>,
    pub preis_max: Option<f64>,
    pub nur_verfuegbar: Option<bool>,
    pub sortierung: Option<String>,  // preis_asc, preis_desc, datum_asc, datum_desc
    pub seite: Option<u32>,
    pub pro_seite: Option<u32>,
}

/// Paginierte Reise-Liste
#[derive(Debug, Clone, Serialize)]
pub struct TripListResponse {
    pub reisen: Vec<TripResponse>,
    pub gesamt: i64,
    pub seite: u32,
    pub pro_seite: u32,
    pub seiten_gesamt: u32,
}
