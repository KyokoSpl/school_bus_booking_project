#!/bin/bash
set -e

# Farben für Output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

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
    
    cd "$PROJECT_DIR/backend"
    cargo run &
    BACKEND_PID=$!
    
    # Warte kurz und prüfe ob Backend läuft
    sleep 3
    if ! kill -0 $BACKEND_PID 2>/dev/null; then
        echo -e "${RED}✗ Backend konnte nicht gestartet werden${NC}"
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

# Hauptprogramm
main() {
    check_dependencies
    start_database
    setup_backend_env
    setup_frontend
    start_backend
    start_frontend
    
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║              Entwicklungsumgebung bereit!                 ║${NC}"
    echo -e "${GREEN}╠═══════════════════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║  Frontend:  ${BLUE}http://localhost:5173${GREEN}                       ║${NC}"
    echo -e "${GREEN}║  Backend:   ${BLUE}http://localhost:8080${GREEN}                       ║${NC}"
    echo -e "${GREEN}║  Adminer:   ${BLUE}http://localhost:8081${GREEN}                       ║${NC}"
    echo -e "${GREEN}╠═══════════════════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║  Admin-Login:                                             ║${NC}"
    echo -e "${GREEN}║    E-Mail:    admin@sonnenschein-reisen.de                ║${NC}"
    echo -e "${GREEN}║    Passwort:  Admin123!                                   ║${NC}"
    echo -e "${GREEN}╠═══════════════════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║  Drücke ${YELLOW}Ctrl+C${GREEN} zum Beenden                              ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    # Warte auf Benutzer-Interrupt
    wait
}

# Script-Argumente verarbeiten
case "${1:-}" in
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
    *)
        main
        ;;
esac
