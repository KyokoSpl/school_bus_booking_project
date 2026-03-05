#!/bin/bash
set -e

# Farben für Output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DESKTOP_MODE=false

echo -e "${BLUE}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         BusBooker Pro - Development Environment          ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Funktion: Cleanup bei Beenden
cleanup() {
    echo ""
    echo -e "${YELLOW}Beende alle Prozesse...${NC}"
    kill $BACKEND_PID 2>/dev/null || true
    kill $FRONTEND_PID 2>/dev/null || true
    echo -e "${GREEN}Auf Wiedersehen!${NC}"
    exit 0
}

trap cleanup SIGINT SIGTERM

# Prüfe Abhängigkeiten
check_dependencies() {
    echo -e "${YELLOW}Prüfe Abhängigkeiten...${NC}"
    
    if ! command -v docker &> /dev/null; then
        echo -e "${RED}✗ Docker nicht gefunden. Bitte installieren: https://docs.docker.com/get-docker/${NC}"
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
        echo -e "${RED}✗ Docker Compose nicht gefunden.${NC}"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}✗ Rust/Cargo nicht gefunden. Bitte installieren: https://rustup.rs${NC}"
        exit 1
    fi
    
    if ! command -v npm &> /dev/null; then
        echo -e "${RED}✗ Node.js/npm nicht gefunden. Bitte installieren: https://nodejs.org${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ Alle Abhängigkeiten vorhanden${NC}"
}

# Docker Compose Kommando ermitteln
get_docker_compose_cmd() {
    if docker compose version &> /dev/null; then
        echo "docker compose"
    else
        echo "docker-compose"
    fi
}

# Starte Datenbank
start_database() {
    echo ""
    echo -e "${YELLOW}Starte MariaDB Docker Container...${NC}"
    
    COMPOSE_CMD=$(get_docker_compose_cmd)
    cd "$PROJECT_DIR"
    
    $COMPOSE_CMD up -d db
    
    echo -e "${YELLOW}Warte auf Datenbank-Bereitschaft...${NC}"
    
    # Warte auf Healthcheck
    local max_attempts=30
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if $COMPOSE_CMD exec -T db mysqladmin ping -h localhost -u busbooker -pbusbooker_dev_password --silent 2>/dev/null; then
            echo -e "${GREEN}✓ Datenbank ist bereit!${NC}"
            return 0
        fi
        echo -e "  Versuch $attempt/$max_attempts..."
        sleep 2
        ((attempt++))
    done
    
    echo -e "${RED}✗ Datenbank konnte nicht gestartet werden${NC}"
    exit 1
}

# Backend .env erstellen falls nicht vorhanden
setup_backend_env() {
    if [ ! -f "$PROJECT_DIR/backend/.env" ]; then
        echo -e "${YELLOW}Erstelle backend/.env...${NC}"
        cat > "$PROJECT_DIR/backend/.env" << 'EOF'
# Datenbank
DATABASE_URL=mysql://busbooker:busbooker_dev_password@localhost:3306/busbooker

# JWT
JWT_SECRET=dev_secret_change_in_production_min_32_chars_long

# Server
HOST=127.0.0.1
PORT=8080

# SMTP (für Entwicklung - verwende z.B. Mailhog oder echten SMTP)
SMTP_HOST=localhost
SMTP_PORT=1025
SMTP_USER=
SMTP_PASS=
SMTP_FROM=noreply@sonnenschein-reisen.de

# Frontend URL (für E-Mail Links)
FRONTEND_URL=http://localhost:5173

# Logging
RUST_LOG=info,sqlx=warn,actix_web=info
EOF
        echo -e "${GREEN}✓ .env erstellt${NC}"
    else
        echo -e "${GREEN}✓ backend/.env existiert bereits${NC}"
    fi
}

# Frontend Dependencies installieren
setup_frontend() {
    echo ""
    echo -e "${YELLOW}Prüfe Frontend Dependencies...${NC}"
    
    cd "$PROJECT_DIR/frontend"
    
    if [ ! -d "node_modules" ]; then
        echo -e "${YELLOW}Installiere npm packages...${NC}"
        npm install
    else
        echo -e "${GREEN}✓ node_modules vorhanden${NC}"
    fi
}

# Backend starten
start_backend() {
    echo ""
    echo -e "${YELLOW}Starte Backend (Rust/Actix-Web)...${NC}"
    
    # Beende vorherige Backend-Instanzen auf Port 8080
    local existing_pid
    existing_pid=$(lsof -ti :8080 2>/dev/null || true)
    if [ -n "$existing_pid" ]; then
        echo -e "${YELLOW}Beende vorherige Instanz auf Port 8080 (PID: $existing_pid)...${NC}"
        kill -9 $existing_pid 2>/dev/null || true
        sleep 1
    fi
    
    cd "$PROJECT_DIR/backend"
    cargo run &
    BACKEND_PID=$!
    
    # Warte bis Backend auf Port 8080 lauscht (max 120 Sekunden)
    echo -e "${YELLOW}Warte auf Backend-Start...${NC}"
    local max_attempts=60
    local attempt=1
    while [ $attempt -le $max_attempts ]; do
        if ! kill -0 $BACKEND_PID 2>/dev/null; then
            echo -e "${RED}✗ Backend-Prozess wurde unerwartet beendet${NC}"
            exit 1
        fi
        if ss -tlnp 2>/dev/null | grep -q ':8080'; then
            break
        fi
        sleep 2
        ((attempt++))
    done
    
    if [ $attempt -gt $max_attempts ]; then
        echo -e "${RED}✗ Backend Timeout - konnte nicht gestartet werden${NC}"
        kill $BACKEND_PID 2>/dev/null || true
        exit 1
    fi
    
    echo -e "${GREEN}✓ Backend gestartet (PID: $BACKEND_PID)${NC}"
}

# Frontend starten
start_frontend() {
    echo ""
    echo -e "${YELLOW}Starte Frontend (Vue.js/Vite)...${NC}"
    
    cd "$PROJECT_DIR/frontend"
    npm run dev &
    FRONTEND_PID=$!
    
    sleep 2
    echo -e "${GREEN}✓ Frontend gestartet (PID: $FRONTEND_PID)${NC}"
}

# Desktop-App starten (Tauri)
start_desktop() {
    echo ""
    echo -e "${YELLOW}Starte Desktop-App (Tauri)...${NC}"
    
    cd "$PROJECT_DIR/frontend"
    npm run tauri:dev &
    FRONTEND_PID=$!
    
    sleep 3
    echo -e "${GREEN}✓ Desktop-App gestartet (PID: $FRONTEND_PID)${NC}"
}

# Hauptprogramm
main() {
    check_dependencies
    start_database
    setup_backend_env
    setup_frontend
    start_backend

    if $DESKTOP_MODE; then
        start_desktop
    else
        start_frontend
    fi
    
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║              Entwicklungsumgebung bereit!                 ║${NC}"
    echo -e "${GREEN}╠═══════════════════════════════════════════════════════════╣${NC}"
    if $DESKTOP_MODE; then
    echo -e "${GREEN}║  Modus:     ${BLUE}Desktop (Tauri)${GREEN}                             ║${NC}"
    else
    echo -e "${GREEN}║  Frontend:  ${BLUE}http://localhost:5173${GREEN}                       ║${NC}"
    fi
    echo -e "${GREEN}║  Backend:   ${BLUE}http://localhost:8080${GREEN}                       ║${NC}"
    echo -e "${GREEN}║  Adminer:   ${BLUE}http://localhost:8081${GREEN}                       ║${NC}"
    echo -e "${GREEN}╠═══════════════════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║  Admin-Login:                                             ║${NC}"
    echo -e "${GREEN}║    E-Mail:    admin@sonnenschein-reisen.de                ║${NC}"
    echo -e "${GREEN}║    Passwort:  Admin123!                                   ║${NC}"
    echo -e "${GREEN}║                                                           ║${NC}"
    echo -e "${GREEN}║  Kunde-Login:                                             ║${NC}"
    echo -e "${GREEN}║    E-Mail:    max.mustermann@example.de                   ║${NC}"
    echo -e "${GREEN}║    Passwort:  Kunde123!                                   ║${NC}"
    echo -e "${GREEN}╠═══════════════════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║  Drücke ${YELLOW}Ctrl+C${GREEN} zum Beenden                              ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    # Warte auf Benutzer-Interrupt
    wait
}

# Script-Argumente verarbeiten
case "${1:-}" in
    --desktop|desktop)
        DESKTOP_MODE=true
        main
        ;;
    db|database)
        check_dependencies
        start_database
        echo -e "${GREEN}Datenbank läuft. Adminer: http://localhost:8081${NC}"
        ;;
    stop)
        COMPOSE_CMD=$(get_docker_compose_cmd)
        cd "$PROJECT_DIR"
        $COMPOSE_CMD down
        echo -e "${GREEN}Docker Container gestoppt${NC}"
        ;;
    reset-db)
        COMPOSE_CMD=$(get_docker_compose_cmd)
        cd "$PROJECT_DIR"
        $COMPOSE_CMD down -v
        echo -e "${YELLOW}Datenbank-Volume gelöscht. Starte neu mit: ./dev.sh${NC}"
        ;;
    logs)
        COMPOSE_CMD=$(get_docker_compose_cmd)
        cd "$PROJECT_DIR"
        $COMPOSE_CMD logs -f
        ;;
    --help|-h)
        echo -e "${BLUE}Verwendung:${NC} ./dev.sh [OPTION]"
        echo ""
        echo -e "  ${GREEN}(keine)${NC}        Web-Frontend starten (Vite)"
        echo -e "  ${GREEN}--desktop${NC}      Desktop-App starten (Tauri)"
        echo -e "  ${GREEN}db${NC}             Nur Datenbank starten"
        echo -e "  ${GREEN}stop${NC}           Docker Container stoppen"
        echo -e "  ${GREEN}reset-db${NC}       Datenbank zurücksetzen"
        echo -e "  ${GREEN}logs${NC}           Docker Logs anzeigen"
        echo -e "  ${GREEN}--help${NC}         Diese Hilfe anzeigen"
        ;;
    *)
        main
        ;;
esac
