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

## Voraussetzungen

| Software | Version | Download |
|----------|---------|----------|
| Docker | latest | https://docs.docker.com/get-docker/ |
| Rust | 1.75+ | https://rustup.rs |
| Node.js | 20+ | https://nodejs.org |

### Zusätzliche Abhängigkeiten für die Desktop-App (Tauri)

<details>
<summary><strong>Linux (Debian/Ubuntu)</strong></summary>

```bash
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev \
  librsvg2-dev patchelf libssl-dev libayatana-appindicator3-dev
```
</details>

<details>
<summary><strong>Linux (Arch)</strong></summary>

```bash
sudo pacman -S --needed webkit2gtk-4.1 gtk3 libappindicator-gtk3 librsvg patchelf
```
</details>

<details>
<summary><strong>Linux (Fedora)</strong></summary>

```bash
sudo dnf install webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel \
  librsvg2-devel patchelf openssl-devel
```
</details>

<details>
<summary><strong>Windows</strong></summary>

- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) mit "Desktop-Entwicklung mit C++"
- WebView2 (vorinstalliert auf Windows 10/11)
</details>

---

## Schnellstart (nur Linux)

```bash
# Web-Frontend + Backend starten
./dev.sh

# Desktop-App + Backend starten
./dev.sh --desktop
```

Das Script startet automatisch MariaDB (Docker), Backend (Rust) und Frontend (Vite oder Tauri).

**Weitere Befehle:**

```bash
./dev.sh db        # Nur Datenbank starten
./dev.sh stop      # Docker Container stoppen
./dev.sh reset-db  # Datenbank zurücksetzen (löscht alle Daten!)
./dev.sh logs      # Docker Logs anzeigen
./dev.sh --help    # Alle Optionen anzeigen
```

**URLs nach dem Start:**
- Frontend: http://localhost:5173
- Backend API: http://localhost:8080
- Adminer (DB-GUI): http://localhost:8081

---

## Manuelles Setup

### 1. Datenbank starten

<details>
<summary><strong>Linux</strong></summary>

```bash
docker compose up -d db

# Warte auf Bereitschaft
docker compose logs -f db
```
</details>

<details>
<summary><strong>Windows (PowerShell)</strong></summary>

```powershell
docker compose up -d db

# Warte auf Bereitschaft
docker compose logs -f db
```
</details>

Die Migrationen werden automatisch beim ersten Start ausgeführt.

### 2. Backend starten

<details>
<summary><strong>Linux</strong></summary>

```bash
cd backend

# .env Datei erstellen (nur beim ersten Mal)
cp .env.example .env

# Backend starten
cargo run
```
</details>

<details>
<summary><strong>Windows (PowerShell)</strong></summary>

```powershell
cd backend

# .env Datei erstellen (nur beim ersten Mal)
Copy-Item .env.example .env

# Backend starten
cargo run
```
</details>

Das Backend läuft unter http://localhost:8080

### 3. Frontend starten (Web / Entwicklungsmodus)

<details>
<summary><strong>Linux</strong></summary>

```bash
cd frontend
npm install
npm run dev
```
</details>

<details>
<summary><strong>Windows (PowerShell)</strong></summary>

```powershell
cd frontend
npm install
npm run dev
```
</details>

Das Frontend läuft unter http://localhost:5173

### 4. Desktop-App starten (Tauri / Entwicklungsmodus)

<details>
<summary><strong>Linux</strong></summary>

```bash
cd frontend
npm install
npm run tauri:dev
```
</details>

<details>
<summary><strong>Windows (PowerShell)</strong></summary>

```powershell
cd frontend
npm install
npm run tauri:dev
```
</details>

---

## Installierbare Pakete bauen

### Linux (.deb / .rpm / .exe Cross-Compile)

Das mitgelieferte Build-Script erkennt die Distribution automatisch und installiert fehlende Abhängigkeiten:

```bash
# .deb Paket (Debian/Ubuntu)
./build-release.sh --deb

# .rpm Paket (Fedora/RHEL)
./build-release.sh --rpm

# Windows .exe (Cross-Compile von Linux)
./build-release.sh --exe

# Alle Formate
./build-release.sh --all

# Alle Formate + GitHub Release
./build-release.sh --all --release --tag v1.0.0

# Optionen anzeigen
./build-release.sh --help
```

Artefakte werden in `release-artifacts/` abgelegt.

### Windows (nativ)

```powershell
cd frontend
npm install
npm run tauri:build
```

Dies erstellt einen NSIS-Installer (`.exe`) und ein MSI-Paket unter `frontend/src-tauri/target/release/bundle/`.

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