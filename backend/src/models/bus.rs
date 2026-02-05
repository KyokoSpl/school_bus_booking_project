use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// Bus-Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BusStatus {
    Verfuegbar,
    InWartung,
    AusserBetrieb,
}

impl Default for BusStatus {
    fn default() -> Self {
        BusStatus::Verfuegbar
    }
}

/// Bus-Entität aus der Datenbank
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bus {
    pub id: String,
    pub kennzeichen: String,
    pub bezeichnung: String,
    pub sitzplaetze: i32,
    pub ausstattung: Option<String>,  // JSON-Array als String
    pub baujahr: Option<i32>,
    pub status: String,
    pub notizen: Option<String>,
    pub erstellt_am: DateTime<Utc>,
    pub aktualisiert_am: DateTime<Utc>,
}

/// Bus-Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusResponse {
    pub id: String,
    pub kennzeichen: String,
    pub bezeichnung: String,
    pub sitzplaetze: i32,
    pub ausstattung: Vec<String>,
    pub baujahr: Option<i32>,
    pub status: String,
    pub notizen: Option<String>,
    pub erstellt_am: DateTime<Utc>,
}

impl From<Bus> for BusResponse {
    fn from(bus: Bus) -> Self {
        BusResponse {
            id: bus.id,
            kennzeichen: bus.kennzeichen,
            bezeichnung: bus.bezeichnung,
            sitzplaetze: bus.sitzplaetze,
            ausstattung: bus.ausstattung
                .map(|s| serde_json::from_str(&s).unwrap_or_default())
                .unwrap_or_default(),
            baujahr: bus.baujahr,
            status: bus.status,
            notizen: bus.notizen,
            erstellt_am: bus.erstellt_am,
        }
    }
}

/// Bus erstellen/aktualisieren Request
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateBusRequest {
    #[validate(length(min = 3, max = 20, message = "Kennzeichen muss zwischen 3 und 20 Zeichen lang sein"))]
    pub kennzeichen: String,
    
    #[validate(length(min = 2, max = 100, message = "Bezeichnung muss zwischen 2 und 100 Zeichen lang sein"))]
    pub bezeichnung: String,
    
    #[validate(range(min = 1, max = 100, message = "Sitzplätze müssen zwischen 1 und 100 liegen"))]
    pub sitzplaetze: i32,
    
    pub ausstattung: Option<Vec<String>>,
    pub baujahr: Option<i32>,
    pub status: Option<String>,
    pub notizen: Option<String>,
}

/// Bus-Liste Response
#[derive(Debug, Clone, Serialize)]
pub struct BusListResponse {
    pub busse: Vec<BusResponse>,
    pub gesamt: i64,
}
