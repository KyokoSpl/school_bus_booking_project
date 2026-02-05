# BusBooker Pro - Busreisen-Buchungssystem

Ein modernes Buchungssystem für Busreisen, entwickelt für Sonnenschein Reisen GmbH.

## Technologie-Stack

### Backend
- **Rust** mit **Actix-Web 4** als REST-API-Framework
- **SQLx** für typsichere SQL-Queries mit **MariaDB/MySQL**
- **JWT** für Authentifizierung
- **Argon2** für sicheres Passwort-Hashing
- **lettre** für SMTP E-Mail-Versand

### Frontend
- **Vue.js 3** mit Composition API und TypeScript
- **Vite** als Build-Tool
- **Pinia** für State Management
- **Tailwind CSS** für Styling
- **Vue Router** für Navigation
- **Tauri** für Desktop-App-Kompilierung

## Projektstruktur

```
school_bus_booking_project/
├── backend/              # Rust Backend
│   ├── src/
│   │   ├── handlers/     # API-Endpunkte
│   │   ├── models/       # Datenmodelle
│   │   ├── services/     # Business-Logik
│   │   └── middleware/   # Auth-Middleware
│   └── migrations/       # SQL-Migrationen
├── frontend/             # Vue.js Frontend
│   ├── src/
│   │   ├── api/          # API-Services
│   │   ├── stores/       # Pinia Stores
│   │   ├── views/        # Vue-Komponenten
│   │   │   ├── admin/    # Admin-Bereich
│   │   │   └── legal/    # Rechtliche Seiten
│   │   └── router/       # Vue Router
│   └── src-tauri/        # Tauri Desktop-App
└── doc/                  # Dokumentation
```

## Schnellstart (Empfohlen)

```bash
# Alles automatisch starten
./dev.sh
```

Das Script:
1. ✅ Startet MariaDB in Docker
2. ✅ Wartet auf Datenbank-Bereitschaft
3. ✅ Erstellt `.env` falls nicht vorhanden
4. ✅ Installiert npm packages falls nötig
5. ✅ Startet Backend (Rust)
6. ✅ Startet Frontend (Vite)

**URLs nach dem Start:**
- Frontend: http://localhost:5173
- Backend API: http://localhost:8080
- Adminer (DB-GUI): http://localhost:8081

### Weitere Befehle

```bash
./dev.sh db        # Nur Datenbank starten
./dev.sh stop      # Docker Container stoppen
./dev.sh reset-db  # Datenbank zurücksetzen (löscht alle Daten!)
./dev.sh logs      # Docker Logs anzeigen
```

## Manuelles Setup

### Voraussetzungen

- **Docker** - https://docs.docker.com/get-docker/
- **Rust** (1.75+) - https://rustup.rs
- **Node.js** (20+) - https://nodejs.org
- **Tauri CLI** Voraussetzungen (für Desktop-App):
  - Linux: `sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`
  - macOS: Xcode Command Line Tools
  - Windows: Visual Studio Build Tools, WebView2

### 1. Datenbank starten (Docker)

```bash
docker-compose up -d db

# Warte auf Bereitschaft
docker-compose logs -f db
```

Die Migrationen werden automatisch beim ersten Start ausgeführt.

### 2. Backend einrichten

```bash
cd backend

# .env Datei erstellen
cp .env.example .env
# Bearbeiten Sie .env bei Bedarf

# Backend starten
cargo run
```

Das Backend läuft unter http://localhost:8080

### 3. Frontend einrichten

```bash
cd frontend

# Dependencies installieren
npm install

# Entwicklungsserver starten
npm run dev
```

Das Frontend läuft unter http://localhost:5173

### 4. Desktop-App bauen (optional)

```bash
cd frontend

# Tauri-App im Entwicklungsmodus
npm run tauri:dev

# Produktions-Build erstellen
npm run tauri:build
```

## Standard-Zugangsdaten

Nach dem Ausführen der Seed-Daten:

**Admin:**
- E-Mail: `admin@sonnenschein-reisen.de`
- Passwort: `Admin123!`

**Testkunde:**
- E-Mail: `max.mustermann@test.de`
- Passwort: `Test123!`

## API-Endpunkte

### Öffentlich
- `GET /api/reisen` - Reisen auflisten
- `GET /api/reisen/{id}` - Reise-Details
- `POST /api/auth/register` - Registrierung
- `POST /api/auth/login` - Login

### Authentifiziert (Kunde)
- `GET /api/buchungen` - Eigene Buchungen
- `POST /api/buchungen` - Neue Buchung
- `PUT /api/profil` - Profil bearbeiten

### Admin
- `GET /api/admin/dashboard` - Statistiken
- `GET/POST/PUT/DELETE /api/admin/reisen` - Reisen verwalten
- `GET/PUT /api/admin/buchungen` - Buchungen verwalten
- `GET/POST/PUT/DELETE /api/admin/benutzer` - Benutzer verwalten
- `GET/POST/PUT/DELETE /api/admin/busse` - Busse verwalten

## Umgebungsvariablen

### Backend (.env)

```env
DATABASE_URL=mysql://user:pass@localhost:3306/busbooker
JWT_SECRET=your-secure-secret-key
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=your-email
SMTP_PASS=your-password
SMTP_FROM=noreply@sonnenschein-reisen.de
FRONTEND_URL=http://localhost:5173
```

## Features

### Must-Have (implementiert)
- ✅ Benutzerregistrierung mit E-Mail-Verifizierung
- ✅ Reisen suchen und filtern
- ✅ Detaillierte Reiseinformationen
- ✅ Buchung mit Reiseteilnehmern
- ✅ Buchungsübersicht und -verwaltung
- ✅ E-Mail-Bestätigungen
- ✅ Admin: Reisen CRUD
- ✅ Admin: Buchungsverwaltung
- ✅ Admin: Kundenverwaltung
- ✅ Zahlung (Mock: Kreditkarte, PayPal, Rechnung)

### Kann-Haben (geplant)
- Sitzplatzauswahl
- Echte Zahlungsintegration (Stripe)
- Newsletter-System
- Bewertungen

## Lizenz

Proprietär - Sonnenschein Reisen GmbH