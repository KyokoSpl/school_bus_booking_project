@echo off
setlocal enabledelayedexpansion
chcp 65001 >nul 2>&1
title BusBooker Pro - Development Environment

:: ============================================================
:: BusBooker Pro – Windows Development Script
:: Startet MariaDB (Docker), Backend (Rust) und Frontend (Vite)
:: ============================================================

:: Ins Projektverzeichnis wechseln (wichtig bei "Als Admin ausfuehren")
cd /d "%~dp0"

echo ╔═══════════════════════════════════════════════════════════╗
echo ║         BusBooker Pro - Development Environment          ║
echo ╚═══════════════════════════════════════════════════════════╝
echo.

:: ── Argumente ───────────────────────────────────────────────
if "%~1"=="--help"   goto :usage
if "%~1"=="-h"       goto :usage
if "%~1"=="stop"     goto :stop
if "%~1"=="reset-db" goto :reset_db
if "%~1"=="db"       goto :db_only

:: ── Hauptprogramm ───────────────────────────────────────────
call :check_deps
if errorlevel 1 goto :eof

call :start_database
if errorlevel 1 goto :eof

call :setup_backend_env
call :setup_frontend
call :start_all

pause
goto :eof

:: ── Hilfefunktion ───────────────────────────────────────────
:usage
echo Verwendung: dev.bat [OPTION]
echo.
echo   (keine)        Web-Frontend + Backend starten
echo   db             Nur Datenbank starten
echo   stop           Docker Container stoppen
echo   reset-db       Datenbank zuruecksetzen (Daten loeschen)
echo   --help         Diese Hilfe anzeigen
pause
goto :eof

:: ── Abhängigkeiten prüfen ───────────────────────────────────
:check_deps
echo [*] Pruefe Abhaengigkeiten...

:: Docker
where docker >nul 2>&1
if errorlevel 1 (
    echo [!] Docker nicht gefunden.
    echo     Installieren mit: winget install Docker.DockerDesktop
    echo     Oder: https://docs.docker.com/get-docker/
    echo.
    set /p "INSTALL_DOCKER=Docker jetzt mit winget installieren? [j/N]: "
    if /i "!INSTALL_DOCKER!"=="j" (
        winget install Docker.DockerDesktop
        echo [!] Bitte Docker Desktop starten und dieses Script erneut ausfuehren.
        exit /b 1
    )
    exit /b 1
)
echo [OK] Docker

:: Cargo/Rust
where cargo >nul 2>&1
if errorlevel 1 (
    echo [!] Rust/Cargo nicht gefunden.
    echo     Installieren mit: winget install Rustlang.Rustup
    echo     Dann: rustup default stable
    echo.
    set /p "INSTALL_RUST=Rust jetzt mit winget installieren? [j/N]: "
    if /i "!INSTALL_RUST!"=="j" (
        winget install Rustlang.Rustup
        echo [!] Bitte ein neues CMD-Fenster oeffnen und dieses Script erneut ausfuehren.
        exit /b 1
    )
    exit /b 1
)
for /f "tokens=2" %%v in ('rustc --version 2^>nul') do echo [OK] Rust %%v

:: Node.js/npm
where npm >nul 2>&1
if errorlevel 1 (
    echo [!] Node.js/npm nicht gefunden.
    echo     Installieren mit: winget install OpenJS.NodeJS.LTS
    echo.
    set /p "INSTALL_NODE=Node.js jetzt mit winget installieren? [j/N]: "
    if /i "!INSTALL_NODE!"=="j" (
        winget install OpenJS.NodeJS.LTS
        echo [!] Bitte ein neues CMD-Fenster oeffnen und dieses Script erneut ausfuehren.
        exit /b 1
    )
    exit /b 1
)
for /f "delims=" %%v in ('node --version 2^>nul') do echo [OK] Node.js %%v

echo [OK] Alle Abhaengigkeiten vorhanden
echo.
exit /b 0

:: ── Datenbank starten ───────────────────────────────────────
:start_database
echo [*] Starte MariaDB Docker Container...
docker compose up -d db
if errorlevel 1 (
    echo [!] Docker Compose fehlgeschlagen. Laeuft Docker Desktop?
    exit /b 1
)

echo [*] Warte auf Datenbank-Bereitschaft...
set ATTEMPTS=0
:db_wait_loop
if !ATTEMPTS! geq 30 (
    echo [!] Datenbank-Timeout - konnte nicht gestartet werden
    exit /b 1
)
set /a ATTEMPTS+=1

docker compose exec -T db mysqladmin ping -h localhost -u busbooker -pbusbooker_dev_password --silent >nul 2>&1
if errorlevel 1 (
    echo     Versuch !ATTEMPTS!/30...
    timeout /t 2 /nobreak >nul
    goto :db_wait_loop
)

echo [OK] Datenbank ist bereit!
exit /b 0

:: ── Backend .env erstellen ──────────────────────────────────
:setup_backend_env
if exist "%~dp0backend\.env" (
    echo [OK] backend\.env existiert bereits
    exit /b 0
)

echo [*] Erstelle backend\.env...
(
echo DATABASE_URL=mysql://busbooker:busbooker_dev_password@localhost:3307/busbooker
echo JWT_SECRET=dev_secret_change_in_production_min_32_chars_long
echo HOST=127.0.0.1
echo PORT=8080
echo SMTP_HOST=localhost
echo SMTP_PORT=1025
echo SMTP_USERNAME=
echo SMTP_PASSWORD=
echo SMTP_FROM_EMAIL=noreply@sonnenschein-reisen.de
echo SMTP_FROM_NAME=BusBooker
echo FRONTEND_URL=http://localhost:5173
echo RUST_LOG=info,sqlx=warn,actix_web=info
) > "%~dp0backend\.env"
echo [OK] .env erstellt
exit /b 0

:: ── Frontend Dependencies ───────────────────────────────────
:setup_frontend
echo.
echo [*] Pruefe Frontend Dependencies...
if exist "%~dp0frontend\node_modules" (
    echo [OK] node_modules vorhanden
    exit /b 0
)
echo [*] Installiere npm packages...
pushd "%~dp0frontend"
npm install
popd
exit /b 0

:: ── Backend + Frontend starten ──────────────────────────────
:start_all
echo.
echo ╔═══════════════════════════════════════════════════════════╗
echo ║  Starte Backend und Frontend in separaten Fenstern...    ║
echo ╚═══════════════════════════════════════════════════════════╝
echo.

:: Backend in neuem Fenster starten
start "BusBooker Backend (Rust)" cmd /k "cd /d "%~dp0backend" && echo [*] Starte Backend (Rust/Actix-Web)... && echo [*] Erster Build kann einige Minuten dauern... && cargo run"

:: Warten bis Backend bereit ist
echo [*] Warte auf Backend-Start (Port 8080)...
set ATTEMPTS=0
:backend_wait_loop
if !ATTEMPTS! geq 90 (
    echo [!] Backend-Timeout. Pruefe das Backend-Fenster auf Fehler.
    goto :start_frontend_anyway
)
set /a ATTEMPTS+=1

netstat -an | findstr "127.0.0.1:8080" | findstr "LISTENING" >nul 2>&1
if errorlevel 1 (
    if !ATTEMPTS! == 1 echo     (Erster Build dauert laenger - bitte Geduld...)
    timeout /t 2 /nobreak >nul
    goto :backend_wait_loop
)
echo [OK] Backend gestartet!

:start_frontend_anyway
:: Frontend in neuem Fenster starten
start "BusBooker Frontend (Vite)" cmd /k "cd /d "%~dp0frontend" && echo [*] Starte Frontend (Vue.js/Vite)... && npm run dev"

:: Kurz warten, dann Info anzeigen
timeout /t 3 /nobreak >nul

echo.
echo ╔═══════════════════════════════════════════════════════════╗
echo ║              Entwicklungsumgebung bereit!                 ║
echo ╠═══════════════════════════════════════════════════════════╣
echo ║  Frontend:  http://localhost:5173                        ║
echo ║  Backend:   http://localhost:8080                        ║
echo ║  Adminer:   http://localhost:8081                        ║
echo ╠═══════════════════════════════════════════════════════════╣
echo ║  Admin-Login:                                             ║
echo ║    E-Mail:    admin@sonnenschein-reisen.de                ║
echo ║    Passwort:  Admin123!                                   ║
echo ║                                                           ║
echo ║  Kunde-Login:                                             ║
echo ║    E-Mail:    max.mustermann@example.de                   ║
echo ║    Passwort:  Kunde123!                                   ║
echo ╠═══════════════════════════════════════════════════════════╣
echo ║  Backend/Frontend laufen in separaten Fenstern.          ║
echo ║  Zum Beenden: Fenster schliessen oder Ctrl+C druecken.   ║
echo ║  Datenbank stoppen: dev.bat stop                         ║
echo ╚═══════════════════════════════════════════════════════════╝
echo.
goto :eof

:: ── Nur Datenbank starten ───────────────────────────────────
:db_only
call :check_deps
if errorlevel 1 goto :eof
call :start_database
if errorlevel 1 goto :eof
echo.
echo [OK] Datenbank laeuft. Adminer: http://localhost:8081
pause
goto :eof

:: ── Docker stoppen ──────────────────────────────────────────
:stop
echo [*] Stoppe Docker Container...
docker compose down
echo [OK] Docker Container gestoppt
pause
goto :eof

:: ── Datenbank zurücksetzen ──────────────────────────────────
:reset_db
echo [*] Setze Datenbank zurueck...
docker compose down -v
echo [OK] Datenbank-Volume geloescht. Starte neu mit: dev.bat
pause
goto :eof
