# Pflichtenheft
## Online-Buchungssystem für Busreisen - "BusBooker Pro"

**Auftraggeber:** Sonnenschein Reisen GmbH  
**Auftragnehmer:** [Ihr Ausbildungsbetrieb]  
**Erstellt am:** 05.02.2026  
**Version:** 1.0  
**Dokumentstatus:** Entwurf

---

## 1. Zielbestimmung

### 1.1 Musskriterien
Das System **muss** folgende Funktionen erfüllen:

#### Kundenbereich
- [ ] Benutzerregistrierung
- [ ] Sichere Anmeldung (Login/Logout)
- [ ] Passwort-Zurücksetzen-Funktion
- [ ] Reisesuche mit Filtern (Datum, Ziel, Preis)
- [ ] Detailansicht für einzelne Reisen
- [ ] Buchungsprozess mit Platzauswahl
- [ ] Persönlicher Buchungsverlauf
- [ ] Automatische E-Mail-Bestätigung

#### Administrationsbereich
- [ ] Reiseverwaltung (CRUD-Operationen)
- [ ] Kundenverwaltung
- [ ] Dashboard mit Übersichtsinformationen

#### Technische Anforderungen
- [ ] HTTPS-Verschlüsselung
- [ ] DSGVO-konforme Datenspeicherung
- [ ] Passwort-Hashing (bcrypt/Argon2)
- [ ] Input-Validierung gegen Injection-Angriffe

### 1.2 Sollkriterien
Das System **soll** folgende Funktionen erfüllen:

- [ ] Buchungsstornierung durch Kunden
- [ ] Responsive Design für mobile Geräte
- [ ] Statistiken und Berichte für Administratoren
- [ ] Preiskonfiguration und Rabattsystem
- [ ] PayPal-Integration (moked)
- [ ] Kreditkartenzahlung (Stripe)
- [ ] Systemverfügbarkeit von 99%

### 1.3 Kannkriterien
Das System **kann** folgende Funktionen erfüllen:

- [ ] Kundenbewertungen für Reisen
- [ ] Mehrsprachigkeit (Englisch)
- [ ] Barrierefreiheit WCAG 2.1 AA
- [ ] Zahlung per Rechnung (mocked)
- [ ] Push-Benachrichtigungen
- [ ] Integration von Kartendiensten
Adminfunktionen
- [ ] Fahrzeugverwaltung mit Kapazitäten
- [ ] Buchungsübersicht und -verwaltung

### 1.4 Abgrenzungskriterien
Das System wird **nicht** umfassen:

- Eigene mobile Apps (iOS/Android)
- Flottenmanagement-System
- Personalplanung
- Buchhaltungssystem (nur Export-Schnittstellen)
- Live-Tracking von Bussen

---

## 2. Produkteinsatz

### 2.1 Anwendungsbereiche
- Online-Verkauf von Busreisen
- Kundenselbstverwaltung
- Interne Verwaltung von Reisen und Buchungen
- Kundenkommunikation

### 2.2 Zielgruppen

| Zielgruppe | Beschreibung | Technische Kenntnisse |
|------------|--------------|----------------------|
| Endkunden | Privatpersonen, Reiseinteressenten | Basis-PC/Internet-Kenntnisse |
| Sachbearbeiter | Mitarbeiter im Kundenservice | Office-Anwendungen |
| Administratoren | IT-Personal, Geschäftsführung | Erweiterte IT-Kenntnisse |

### 2.3 Betriebsbedingungen
- **Betriebszeit:** 24/7 (automatisierter Betrieb)
- **Wartungsfenster:** Sonntags 02:00-06:00 Uhr
- **Betreuung:** Während Geschäftszeiten (Mo-Fr 08:00-18:00)

---

## 3. Produktumgebung

### 3.1 Software

#### Entwicklung
| Komponente | Technologie | Version |
|------------|-------------|---------|
| Framework | Tauri | 2.x |
| Backend | Rust | 1.75+ |
| Frontend | Vue.js | 3.x |
| Build-Tool | Vite | 5.x |
| State-Management | Pinia | 2.x |
| UI-Komponenten | PrimeVue / Vuetify | aktuell |
| CSS-Framework | Tailwind CSS | 3.x |

#### Datenbank
| Typ | Technologie | Verwendung |
|-----|-------------|------------|
| Lokal | SQLite | Desktop-Installation |
| Server | PostgreSQL 15+ | Zentrale Datenhaltung |

#### Zusätzliche Bibliotheken
- **Authentifizierung:** jsonwebtoken (JWT)
- **Passwort-Hashing:** argon2
- **E-Mail:** lettre (Rust)
- **HTTP-Client:** reqwest
- **Validierung:** validator

### 3.2 Hardware (Mindestanforderungen)

#### Client-System
| Komponente | Anforderung |
|------------|-------------|
| Prozessor | Dual-Core 2.0 GHz |
| RAM | 4 GB |
| Festplatte | 500 MB frei |
| Bildschirm | 1280x720 Pixel |
| Netzwerk | Internetverbindung |

#### Server (bei zentraler Installation)
| Komponente | Anforderung |
|------------|-------------|
| Prozessor | Quad-Core 2.5 GHz |
| RAM | 8 GB |
| Festplatte | 50 GB SSD |
| Netzwerk | 100 Mbit/s |

### 3.3 Orgware
- Aktuelle Betriebssysteme (Windows 10+, macOS 10.15+, Linux)
- Gültige Lizenzen für Drittanbieter-Software
- Zugang zu E-Mail-Server (SMTP)
- Verträge mit Zahlungsanbietern (Stripe/PayPal)

---

## 4. Produktfunktionen

### 4.1 Anwendungsfalldiagramm (Use-Case-Übersicht)

```
┌─────────────────────────────────────────────────────────────────┐
│                    BusBooker Pro System                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────┐         ┌─────────────────┐                      │
│  │  Kunde   │────────▶│  Registrieren   │                      │
│  └──────────┘         └─────────────────┘                      │
│       │               ┌─────────────────┐                      │
│       ├──────────────▶│    Anmelden     │                      │
│       │               └─────────────────┘                      │
│       │               ┌─────────────────┐                      │
│       ├──────────────▶│  Reisen suchen  │                      │
│       │               └─────────────────┘                      │
│       │               ┌─────────────────┐                      │
│       ├──────────────▶│  Reise buchen   │                      │
│       │               └─────────────────┘                      │
│       │               ┌─────────────────┐                      │
│       └──────────────▶│ Buchung ansehen │                      │
│                       └─────────────────┘                      │
│                                                                 │
│  ┌──────────┐         ┌─────────────────┐                      │
│  │  Admin   │────────▶│ Reisen verwalten│                      │
│  └──────────┘         └─────────────────┘                      │
│       │               ┌─────────────────┐                      │
│       ├──────────────▶│ Busse verwalten │                      │
│       │               └─────────────────┘                      │
│       │               ┌─────────────────┐                      │
│       ├──────────────▶│Buchungen verw.  │                      │
│       │               └─────────────────┘                      │
│       │               ┌─────────────────┐                      │
│       └──────────────▶│ Berichte        │                      │
│                       └─────────────────┘                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 4.2 Detaillierte Funktionsbeschreibungen

#### F001: Benutzerregistrierung
| Attribut | Beschreibung |
|----------|--------------|
| **ID** | F001 |
| **Name** | Benutzerregistrierung |
| **Akteur** | Kunde |
| **Vorbedingung** | Keine |
| **Nachbedingung** | Benutzerkonto angelegt, Verifizierungs-E-Mail gesendet |
| **Ablauf** | 1. Kunde öffnet Registrierungsseite<br>2. Eingabe: Vorname, Nachname, E-Mail, Passwort<br>3. Zustimmung zu AGB und Datenschutz<br>4. System validiert Eingaben<br>5. System erstellt Konto (inaktiv)<br>6. System sendet Verifizierungs-E-Mail<br>7. Kunde klickt Verifizierungslink<br>8. Konto wird aktiviert |
| **Fehlerfälle** | - E-Mail bereits registriert<br>- Passwort zu schwach<br>- Ungültige Eingaben |

#### F002: Reisesuche
| Attribut | Beschreibung |
|----------|--------------|
| **ID** | F002 |
| **Name** | Reisesuche |
| **Akteur** | Kunde (auch ohne Anmeldung) |
| **Vorbedingung** | Keine |
| **Nachbedingung** | Liste passender Reisen angezeigt |
| **Ablauf** | 1. Kunde öffnet Suchseite<br>2. Eingabe Suchkriterien (Ziel, Datum, etc.)<br>3. System sucht passende Reisen<br>4. Ergebnisse werden sortiert angezeigt<br>5. Kunde kann Filter anpassen |
| **Filter** | - Reiseziel (Freitext/Auswahl)<br>- Datum (von-bis)<br>- Preisbereich<br>- Verfügbare Plätze<br>- Reisedauer |

#### F003: Buchungsprozess
| Attribut | Beschreibung |
|----------|--------------|
| **ID** | F003 |
| **Name** | Buchungsprozess |
| **Akteur** | Registrierter Kunde |
| **Vorbedingung** | Kunde angemeldet, Reise ausgewählt |
| **Nachbedingung** | Buchung erstellt, Bestätigung gesendet |
| **Ablauf** | 1. Kunde wählt Reise aus<br>2. Kunde wählt Anzahl Plätze<br>3. Eingabe Mitreisende-Daten<br>4. Auswahl Zahlungsmethode<br>5. Zusammenfassung anzeigen<br>6. Kunde bestätigt Buchung<br>7. Zahlung wird verarbeitet<br>8. Buchungsbestätigung per E-Mail |

#### F004: Reiseverwaltung (Admin)
| Attribut | Beschreibung |
|----------|--------------|
| **ID** | F004 |
| **Name** | Reiseverwaltung |
| **Akteur** | Administrator |
| **Vorbedingung** | Admin angemeldet |
| **Funktionen** | - Neue Reise anlegen<br>- Reise bearbeiten<br>- Reise löschen/deaktivieren<br>- Bus zuweisen<br>- Preise festlegen<br>- Bilder hochladen |

---

## 5. Produktdaten

### 5.1 Datenmodell (ER-Diagramm vereinfacht)

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│    Kunde     │     │   Buchung    │     │    Reise     │
├──────────────┤     ├──────────────┤     ├──────────────┤
│ id           │◄────│ kunde_id     │     │ id           │
│ email        │     │ id           │────▶│ ziel         │
│ passwort_hash│     │ reise_id     │     │ beschreibung │
│ vorname      │     │ status       │     │ abfahrt_datum│
│ nachname     │     │ anzahl_plätze│     │ rückkehr     │
│ telefon      │     │ gesamtpreis  │     │ preis        │
│ adresse      │     │ buchungsdatum│     │ bus_id       │
│ erstellt_am  │     │ zahlungsstatus     │ kapazität    │
└──────────────┘     └──────────────┘     └──────────────┘
                                                 │
                                                 ▼
┌──────────────┐                          ┌──────────────┐
│   Zahlung    │                          │     Bus      │
├──────────────┤                          ├──────────────┤
│ id           │                          │ id           │
│ buchung_id   │                          │ kennzeichen  │
│ betrag       │                          │ sitzplätze   │
│ methode      │                          │ ausstattung  │
│ status       │                          │ baujahr      │
│ transaktions_│                          │ status       │
│ id           │                          └──────────────┘
└──────────────┘
```

### 5.2 Datenfelder im Detail

#### D001: Kunde
| Feld | Typ | Pflicht | Beschreibung |
|------|-----|---------|--------------|
| id | UUID | Ja | Eindeutige Kennung |
| email | VARCHAR(255) | Ja | E-Mail-Adresse (einmalig) |
| passwort_hash | VARCHAR(255) | Ja | Gehashtes Passwort |
| vorname | VARCHAR(100) | Ja | Vorname |
| nachname | VARCHAR(100) | Ja | Nachname |
| telefon | VARCHAR(20) | Nein | Telefonnummer |
| strasse | VARCHAR(255) | Nein | Straße und Hausnummer |
| plz | VARCHAR(10) | Nein | Postleitzahl |
| ort | VARCHAR(100) | Nein | Wohnort |
| land | VARCHAR(100) | Nein | Land |
| geburtsdatum | DATE | Nein | Geburtsdatum |
| erstellt_am | TIMESTAMP | Ja | Registrierungsdatum |
| aktualisiert_am | TIMESTAMP | Ja | Letzte Änderung |
| email_verifiziert | BOOLEAN | Ja | Verifizierungsstatus |
| aktiv | BOOLEAN | Ja | Kontostatus |

#### D002: Reise
| Feld | Typ | Pflicht | Beschreibung |
|------|-----|---------|--------------|
| id | UUID | Ja | Eindeutige Kennung |
| titel | VARCHAR(255) | Ja | Reisetitel |
| beschreibung | TEXT | Ja | Detaillierte Beschreibung |
| ziel | VARCHAR(255) | Ja | Reiseziel |
| abfahrtsort | VARCHAR(255) | Ja | Startpunkt |
| abfahrt_datum | TIMESTAMP | Ja | Abfahrtszeitpunkt |
| rueckkehr_datum | TIMESTAMP | Ja | Rückkehrzeitpunkt |
| preis_pro_person | DECIMAL(10,2) | Ja | Grundpreis |
| bus_id | UUID | Ja | Zugewiesener Bus |
| max_teilnehmer | INTEGER | Ja | Maximale Teilnehmerzahl |
| aktuelle_buchungen | INTEGER | Ja | Gebuchte Plätze |
| status | ENUM | Ja | geplant/aktiv/abgeschlossen/storniert |
| bilder | JSON | Nein | Bilderpfade |

#### D003: Buchung
| Feld | Typ | Pflicht | Beschreibung |
|------|-----|---------|--------------|
| id | UUID | Ja | Eindeutige Kennung |
| buchungsnummer | VARCHAR(20) | Ja | Lesbare Buchungsnummer |
| kunde_id | UUID | Ja | Referenz auf Kunde |
| reise_id | UUID | Ja | Referenz auf Reise |
| anzahl_personen | INTEGER | Ja | Gebuchte Plätze |
| gesamtpreis | DECIMAL(10,2) | Ja | Gesamtbetrag |
| status | ENUM | Ja | ausstehend/bestätigt/storniert |
| buchungsdatum | TIMESTAMP | Ja | Buchungszeitpunkt |
| mitreisende | JSON | Nein | Daten der Mitreisenden |
| bemerkungen | TEXT | Nein | Kundenbemerkungen |

---

## 6. Produktleistungen

### 6.1 Performance-Anforderungen

| ID | Anforderung | Messkriterium |
|----|-------------|---------------|
| L001 | Seitenaufbau | < 3 Sekunden |
| L002 | Suchergebnisse | < 2 Sekunden |
| L003 | Buchungsabschluss | < 5 Sekunden |
| L004 | Gleichzeitige Benutzer | >= 100 |
| L005 | Datenbankabfragen | < 500ms |

### 6.2 Kapazitätsanforderungen

| Ressource | Kapazität |
|-----------|-----------|
| Benutzerkonten | Unbegrenzt |
| Reisen | Unbegrenzt |
| Buchungen pro Tag | > 1.000 |
| Datenbankgröße | > 10 GB |
| Dateispeicher | > 5 GB |

---

## 7. Benutzeroberfläche

### 7.1 Dialogstruktur

```
┌─────────────────────────────────────────────────────────────┐
│                      STARTSEITE                             │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │
│  │ Reisen  │  │ Über uns│  │ Kontakt │  │ Login   │        │
│  └────┬────┘  └─────────┘  └─────────┘  └────┬────┘        │
│       │                                       │             │
│       ▼                                       ▼             │
│  ┌─────────┐                            ┌─────────┐        │
│  │ Suche   │                            │Register │        │
│  └────┬────┘                            └─────────┘        │
│       │                                                     │
│       ▼                                                     │
│  ┌─────────┐                                               │
│  │Ergebnis │                                               │
│  │ liste   │                                               │
│  └────┬────┘                                               │
│       │                                                     │
│       ▼                                                     │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐                │
│  │ Detail  │───▶│ Buchen  │───▶│Bestätig.│                │
│  └─────────┘    └─────────┘    └─────────┘                │
└─────────────────────────────────────────────────────────────┘
```

### 7.2 Bildschirmmasken (Wireframes)

#### Startseite
```
┌────────────────────────────────────────────────────────────┐
│ [LOGO] Sonnenschein Reisen    Reisen | Über uns | ○ Login │
├────────────────────────────────────────────────────────────┤
│                                                            │
│            ═══ Entdecken Sie unsere Reisen ═══            │
│                                                            │
│  ┌────────────────────────────────────────────────────┐   │
│  │  Wohin?  [________________▼]                       │   │
│  │  Wann?   [__/__/____] - [__/__/____]              │   │
│  │  Preis?  [____€] - [____€]                        │   │
│  │                                   [🔍 Suchen]     │   │
│  └────────────────────────────────────────────────────┘   │
│                                                            │
│  ══════════ Beliebte Reiseziele ══════════                │
│                                                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │   Bild   │  │   Bild   │  │   Bild   │               │
│  │  Paris   │  │  Rom     │  │  Wien    │               │
│  │  ab 299€ │  │  ab 349€ │  │  ab 199€ │               │
│  └──────────┘  └──────────┘  └──────────┘               │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

#### Buchungsübersicht (Kunde)
```
┌────────────────────────────────────────────────────────────┐
│ [LOGO]    Reisen | Meine Buchungen | ○ Max Mustermann ▼   │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  Meine Buchungen                                          │
│  ════════════════                                         │
│                                                            │
│  ┌─────────────────────────────────────────────────────┐  │
│  │ Buchung #2026-0001                     [Bestätigt]  │  │
│  │ Paris – Stadt der Liebe                             │  │
│  │ 15.03.2026 - 20.03.2026                             │  │
│  │ 2 Personen | 598,00 €                               │  │
│  │                              [Details] [Stornieren] │  │
│  └─────────────────────────────────────────────────────┘  │
│                                                            │
│  ┌─────────────────────────────────────────────────────┐  │
│  │ Buchung #2026-0002                     [Ausstehend] │  │
│  │ Gardasee Rundfahrt                                  │  │
│  │ 01.05.2026 - 05.05.2026                             │  │
│  │ 1 Person | 249,00 €                                 │  │
│  │                              [Details] [Bezahlen]   │  │
│  └─────────────────────────────────────────────────────┘  │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

### 7.3 Design-Richtlinien

| Aspekt | Spezifikation |
|--------|---------------|
| Primärfarbe | #2563EB (Blau) |
| Sekundärfarbe | #F59E0B (Orange) |
| Schriftart | Inter, system-ui |
| Schriftgröße | 16px Basis |
| Border-Radius | 8px |
| Abstände | 8px Grid-System |

---

## 8. Qualitätsanforderungen

### 8.1 Qualitätsmerkmale nach ISO 25010

| Merkmal | Priorität | Beschreibung |
|---------|-----------|--------------|
| Funktionalität | Sehr hoch | Alle Kernfunktionen implementiert |
| Zuverlässigkeit | Hoch | 99% Verfügbarkeit |
| Benutzbarkeit | Hoch | Intuitive Bedienung ohne Schulung |
| Effizienz | Mittel | Schnelle Reaktionszeiten |
| Wartbarkeit | Mittel | Modularer Aufbau |
| Portabilität | Mittel | Cross-Platform-Fähigkeit |
| Sicherheit | Sehr hoch | DSGVO-Konformität, Verschlüsselung |

### 8.2 Testanforderungen

| Testart | Umfang | Verantwortlich |
|---------|--------|----------------|
| Unit-Tests | 80% Code-Abdeckung | Entwicklung |
| Integrationstests | Alle Schnittstellen | Entwicklung |
| Systemtests | Alle Use-Cases | QA |
| Abnahmetests | Alle Muss-Kriterien | Auftraggeber |
| Lasttests | 100 gleichzeitige User | QA |
| Sicherheitstests | OWASP Top 10 | Security |

---

## 9. Technische Architektur

### 9.1 Systemarchitektur

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Desktop-App                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   Vue.js Frontend                    │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐            │   │
│  │  │ Views   │  │ Compo-  │  │  Store  │            │   │
│  │  │         │  │ nents   │  │ (Pinia) │            │   │
│  │  └─────────┘  └─────────┘  └─────────┘            │   │
│  └────────────────────────┬────────────────────────────┘   │
│                           │ Tauri Commands                  │
│                           ▼                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   Rust Backend                       │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐            │   │
│  │  │Commands │  │ Services│  │ Models  │            │   │
│  │  │ (API)   │  │         │  │         │            │   │
│  │  └─────────┘  └─────────┘  └─────────┘            │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐            │   │
│  │  │Database │  │ E-Mail  │  │ Payment │            │   │
│  │  │ Layer   │  │ Service │  │ Service │            │   │
│  │  └─────────┘  └─────────┘  └─────────┘            │   │
│  └────────────────────────┬────────────────────────────┘   │
│                           │                                 │
│                           ▼                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              SQLite / PostgreSQL DB                  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 9.2 Verzeichnisstruktur

```
busbooker-pro/
├── src/                      # Vue.js Frontend
│   ├── assets/               # Statische Dateien
│   ├── components/           # Wiederverwendbare Komponenten
│   ├── views/                # Seitenkomponenten
│   ├── stores/               # Pinia Stores
│   ├── router/               # Vue Router
│   ├── services/             # API-Services
│   └── utils/                # Hilfsfunktionen
├── src-tauri/                # Rust Backend
│   ├── src/
│   │   ├── commands/         # Tauri Commands
│   │   ├── models/           # Datenmodelle
│   │   ├── services/         # Business-Logik
│   │   ├── db/               # Datenbankzugriff
│   │   └── main.rs           # Einstiegspunkt
│   ├── Cargo.toml            # Rust Dependencies
│   └── tauri.conf.json       # Tauri Konfiguration
├── tests/                    # Tests
├── docs/                     # Dokumentation
└── package.json              # Node.js Dependencies
```

---

## 10. Entwicklungsdokumentation

### 10.1 Entwicklungswerkzeuge

| Werkzeug | Verwendung |
|----------|------------|
| VS Code / RustRover | IDE |
| Git / GitHub | Versionskontrolle |
| npm / cargo | Paketverwaltung |
| Vitest | Frontend-Tests |
| Cargo test | Backend-Tests |
| ESLint / Clippy | Code-Qualität |

### 10.2 Coding Guidelines

- **Rust:** Rust Style Guide, Clippy-Warnungen beheben
- **Vue.js:** Vue.js Style Guide, Composition API bevorzugt
- **TypeScript:** Strict Mode aktiviert
- **Kommentare:** Englisch, JSDoc/rustdoc
- **Commits:** Conventional Commits

---

## 11. Projektplan (MVP - 100 Stunden)

**Hinweis:** Dieser Projektplan ist auf 100 Arbeitsstunden eines Junior-Entwicklers ausgelegt. Der Fokus liegt auf den Kernfunktionen (MVP).

### 11.1 Meilensteine

| Phase | Zeitraum | Stunden | Meilenstein |
|-------|----------|---------|-------------|
| Planung | Tag 1-2 | 8 Std. | M1: Konzept & DB-Design |
| Entwicklung Kern | Woche 1-2 | 50 Std. | M2: Basis-Funktionen |
| Entwicklung Features | Woche 2-3 | 28 Std. | M3: MVP-Complete |
| Test & Abnahme | Woche 4 | 8 Std. | M4: Go-Live |
| Dokumentation | Parallel | 6 Std. | Inkl. in Lieferung |
| **Gesamt** | **4 Wochen** | **100 Std.** | |

### 11.2 Detaillierter Zeitplan

```
Planung (8 Std.)
├── Anforderungsabstimmung (2 Std.)
├── Datenbankdesign (3 Std.)
└── UI-Konzept/Wireframes (3 Std.)

Entwicklung Kern (50 Std.)
├── Projektsetup Tauri + Vue.js (4 Std.)
├── Datenbankanbindung SQLite (4 Std.)
├── Registrierung & Login (8 Std.)
├── Reiseverwaltung Admin CRUD (10 Std.)
├── Reiseübersicht & Details (8 Std.)
├── Reisesuche mit Filtern (6 Std.)
└── Buchungsprozess (10 Std.)

Entwicklung Features (28 Std.)
├── Buchungsübersicht Kunde (6 Std.)
├── Admin-Dashboard (8 Std.)
├── Buchungsverwaltung Admin (6 Std.)
├── E-Mail-Bestätigung (4 Std.)
└── Zahlungssimulation Mock (4 Std.)

Test & Abnahme (8 Std.)
├── Funktionstests (4 Std.)
├── Fehlerbehebung (3 Std.)
└── Abnahme mit Auftraggeber (1 Std.)

Dokumentation (6 Std.)
├── Benutzerhandbuch kompakt (3 Std.)
├── README/Technische Doku (2 Std.)
└── Installationsanleitung (1 Std.)
```

---

## 12. Glossar

| Begriff | Erklärung |
|---------|-----------|
| Tauri | Cross-Platform Framework für Desktop-Apps mit Web-Frontend und Rust-Backend |
| Vue.js | Progressives JavaScript-Framework für Benutzeroberflächen |
| Pinia | State-Management-Bibliothek für Vue.js |
| JWT | JSON Web Token für Authentifizierung |
| CRUD | Create, Read, Update, Delete - Grundoperationen |
| DSGVO | Datenschutz-Grundverordnung |
| SSO | Single Sign-On |
| API | Application Programming Interface |

---

## 13. Anhang

### 13.1 Dokumentenhistorie

| Version | Datum | Autor | Änderungen |
|---------|-------|-------|------------|
| 0.1 | 05.02.2026 | Projektteam | Ersterstellung |
| 1.0 | 05.02.2026 | Projektteam | Freigabe zur Prüfung |

### 13.2 Referenzierte Dokumente

- Lastenheft v1.0
- DSGVO-Datenschutzkonzept
- Angebot

### 13.3 Abnahme

| Rolle | Name | Datum | Unterschrift |
|-------|------|-------|--------------|
| Auftraggeber | | | |
| Auftragnehmer | | | |
| Projektleiter | | | |

---

*Dieses Pflichtenheft definiert die verbindliche technische Umsetzung basierend auf dem Lastenheft.*
