#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# BusBooker Pro – Build & Release Script
# Erstellt .deb, .rpm, .exe (via cross-compile)
# und optional einen GitHub Release.
#
# Unterstützte Distros: Debian/Ubuntu, Fedora/RHEL, Arch Linux
# Fehlende Abhängigkeiten werden automatisch installiert.
# ============================================================

# ── Farben ───────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# ── Defaults ─────────────────────────────────────────────────
BUILD_DEB=false
BUILD_RPM=false
BUILD_EXE=false
BUILD_ALL=false
CREATE_RELEASE=false
RELEASE_TAG=""
RELEASE_TITLE=""
RELEASE_NOTES=""
RELEASE_DRAFT=false
RELEASE_PRERELEASE=false
SKIP_FRONTEND=false
VERBOSE=false
DISTRO=""        # arch | debian | fedora
PKG_MANAGER=""   # pacman | apt | dnf

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FRONTEND_DIR="$SCRIPT_DIR/frontend"
TAURI_DIR="$FRONTEND_DIR/src-tauri"
OUTPUT_DIR="$SCRIPT_DIR/release-artifacts"

# ── Versionsinfo aus tauri.conf.json ─────────────────────────
APP_VERSION=$(grep -o '"version": *"[^"]*"' "$TAURI_DIR/tauri.conf.json" | head -1 | cut -d'"' -f4)
APP_NAME=$(grep -o '"productName": *"[^"]*"' "$TAURI_DIR/tauri.conf.json" | head -1 | cut -d'"' -f4)

# ── Hilfe ────────────────────────────────────────────────────
usage() {
    cat <<EOF
${CYAN}╔══════════════════════════════════════════════════════════════╗
║          BusBooker Pro – Build & Release Script              ║
╚══════════════════════════════════════════════════════════════╝${NC}

${YELLOW}Verwendung:${NC}
    ./build-release.sh [OPTIONEN]

${YELLOW}Build-Optionen:${NC}
    --deb               .deb-Paket erstellen (Debian/Ubuntu)
    --rpm               .rpm-Paket erstellen (Fedora/RHEL)
    --exe               .exe erstellen (Windows, Cross-Compile)
    --all               Alle Formate erstellen
    --skip-frontend     Frontend-Build überspringen (nutzt vorherigen)

${YELLOW}Release-Optionen:${NC}
    --release           GitHub Release erstellen
    --tag TAG            Release-Tag (Standard: v\$VERSION)
    --title TITLE        Release-Titel (Standard: "APP_NAME vTAG")
    --notes TEXT         Release-Notes (oder RELEASE_NOTES env var)
    --notes-file FILE    Release-Notes aus Datei lesen
    --draft             Release als Entwurf markieren
    --prerelease        Release als Pre-Release markieren

${YELLOW}Sonstige:${NC}
    --verbose, -v       Ausführliche Ausgabe
    --help, -h          Diese Hilfe anzeigen

${YELLOW}Beispiele:${NC}
    # Nur .deb bauen
    ./build-release.sh --deb

    # Alles bauen und Release erstellen
    ./build-release.sh --all --release --tag v1.0.0

    # Alles bauen, Draft-Release mit Notes
    ./build-release.sh --all --release --draft --notes "Beta-Release"

    # Nur .exe (Cross-Compile von Linux nach Windows)
    ./build-release.sh --exe

EOF
    exit 0
}

# ── Argumente parsen ─────────────────────────────────────────
while [[ $# -gt 0 ]]; do
    case "$1" in
        --deb)          BUILD_DEB=true ;;
        --rpm)          BUILD_RPM=true ;;
        --exe)          BUILD_EXE=true ;;
        --all)          BUILD_ALL=true ;;
        --release)      CREATE_RELEASE=true ;;
        --tag)          RELEASE_TAG="$2"; shift ;;
        --title)        RELEASE_TITLE="$2"; shift ;;
        --notes)        RELEASE_NOTES="$2"; shift ;;
        --notes-file)   RELEASE_NOTES=$(cat "$2"); shift ;;
        --draft)        RELEASE_DRAFT=true ;;
        --prerelease)   RELEASE_PRERELEASE=true ;;
        --skip-frontend) SKIP_FRONTEND=true ;;
        --verbose|-v)   VERBOSE=true ;;
        --help|-h)      usage ;;
        *)
            echo -e "${RED}Unbekannte Option: $1${NC}"
            echo "Nutze --help für Hilfe."
            exit 1
            ;;
    esac
    shift
done

if $BUILD_ALL; then
    BUILD_DEB=true
    BUILD_RPM=true
    BUILD_EXE=true
fi

# Prüfen ob mindestens ein Build-Ziel gewählt wurde
if ! $BUILD_DEB && ! $BUILD_RPM && ! $BUILD_EXE; then
    echo -e "${RED}Fehler: Kein Build-Ziel angegeben.${NC}"
    echo "Nutze --deb, --rpm, --exe oder --all"
    echo "Nutze --help für Hilfe."
    exit 1
fi

# ── Logging ──────────────────────────────────────────────────
log()     { echo -e "${GREEN}[✓]${NC} $1"; }
log_info(){ echo -e "${BLUE}[ℹ]${NC} $1"; }
log_warn(){ echo -e "${YELLOW}[⚠]${NC} $1"; }
log_err() { echo -e "${RED}[✗]${NC} $1"; }
log_step(){ echo -e "\n${CYAN}━━━ $1 ━━━${NC}"; }

verbose() {
    if $VERBOSE; then
        echo -e "${YELLOW}    $1${NC}"
    fi
}

# ── Voraussetzungen prüfen & installieren ────────────────────

# Distro erkennen
detect_distro() {
    log_step "Betriebssystem erkennen"

    if [[ ! -f /etc/os-release ]]; then
        log_err "/etc/os-release nicht gefunden – Distro nicht erkennbar"
        exit 1
    fi

    source /etc/os-release

    case "${ID:-}${ID_LIKE:-}" in
        *arch*)
            DISTRO="arch"
            PKG_MANAGER="pacman"
            ;;
        *debian*|*ubuntu*|*mint*|*pop*)
            DISTRO="debian"
            PKG_MANAGER="apt"
            ;;
        *fedora*|*rhel*|*centos*|*rocky*|*alma*)
            DISTRO="fedora"
            PKG_MANAGER="dnf"
            # Fallback auf yum
            if ! command -v dnf &>/dev/null && command -v yum &>/dev/null; then
                PKG_MANAGER="yum"
            fi
            ;;
        *)
            log_err "Nicht unterstützte Distribution: ${ID:-unbekannt} (${ID_LIKE:-})"
            log_err "Unterstützt: Arch, Debian/Ubuntu, Fedora/RHEL"
            exit 1
            ;;
    esac

    log "Erkannt: ${PRETTY_NAME:-$ID} (Typ: $DISTRO, Paketmanager: $PKG_MANAGER)"
}

# Paket installieren (distro-agnostisch)
pkg_install() {
    local packages=("$@")
    log_info "Installiere: ${packages[*]}"

    case "$PKG_MANAGER" in
        pacman)
            sudo pacman -S --noconfirm --needed "${packages[@]}"
            ;;
        apt)
            sudo apt-get update -qq
            sudo apt-get install -y -qq "${packages[@]}"
            ;;
        dnf|yum)
            sudo "$PKG_MANAGER" install -y "${packages[@]}"
            ;;
    esac
}

# Prüfe ob ein Befehl existiert, wenn nicht installiere das richtige Paket
ensure_cmd() {
    local cmd="$1"
    shift
    # Restliche Argumente: arch_pkg debian_pkg fedora_pkg
    local arch_pkg="${1:-}"
    local debian_pkg="${2:-}"
    local fedora_pkg="${3:-}"

    if command -v "$cmd" &>/dev/null; then
        verbose "$cmd ✓ vorhanden"
        return 0
    fi

    log_warn "$cmd nicht gefunden – wird installiert..."

    local pkg=""
    case "$DISTRO" in
        arch)   pkg="$arch_pkg" ;;
        debian) pkg="$debian_pkg" ;;
        fedora) pkg="$fedora_pkg" ;;
    esac

    if [[ -z "$pkg" ]]; then
        log_err "Kein Paketname für '$cmd' auf $DISTRO bekannt"
        return 1
    fi

    pkg_install $pkg
    return 0
}

# Alle benötigten System-Pakete installieren
install_system_deps() {
    log_step "System-Abhängigkeiten prüfen & installieren"

    # ── Grundlegende Build-Tools ──────────────────────────────
    case "$DISTRO" in
        arch)
            local base_pkgs=()
            for pkg in base-devel curl wget file; do
                if ! pacman -Qi "$pkg" &>/dev/null 2>&1; then
                    base_pkgs+=("$pkg")
                fi
            done
            [[ ${#base_pkgs[@]} -gt 0 ]] && pkg_install "${base_pkgs[@]}" || verbose "Build-Tools ✓"
            ;;
        debian)
            local base_pkgs=()
            for pkg in build-essential curl wget file; do
                if ! dpkg -s "$pkg" &>/dev/null 2>&1; then
                    base_pkgs+=("$pkg")
                fi
            done
            [[ ${#base_pkgs[@]} -gt 0 ]] && pkg_install "${base_pkgs[@]}" || verbose "Build-Tools ✓"
            ;;
        fedora)
            local base_pkgs=()
            for pkg in gcc gcc-c++ make curl wget file; do
                if ! rpm -q "$pkg" &>/dev/null 2>&1; then
                    base_pkgs+=("$pkg")
                fi
            done
            [[ ${#base_pkgs[@]} -gt 0 ]] && pkg_install "${base_pkgs[@]}" || verbose "Build-Tools ✓"
            ;;
    esac

    # ── Tauri Build-Abhängigkeiten (WebKit, GTK, etc.) ────────
    log_info "Tauri-Abhängigkeiten prüfen..."
    case "$DISTRO" in
        arch)
            local tauri_pkgs=()
            for pkg in webkit2gtk-4.1 gtk3 libappindicator-gtk3 librsvg patchelf; do
                if ! pacman -Qi "$pkg" &>/dev/null 2>&1; then
                    tauri_pkgs+=("$pkg")
                fi
            done
            [[ ${#tauri_pkgs[@]} -gt 0 ]] && pkg_install "${tauri_pkgs[@]}" || verbose "Tauri-Deps ✓"
            ;;
        debian)
            local tauri_pkgs=()
            for pkg in libwebkit2gtk-4.1-dev libgtk-3-dev libappindicator3-dev librsvg2-dev patchelf libssl-dev libayatana-appindicator3-dev; do
                if ! dpkg -s "$pkg" &>/dev/null 2>&1; then
                    tauri_pkgs+=("$pkg")
                fi
            done
            [[ ${#tauri_pkgs[@]} -gt 0 ]] && pkg_install "${tauri_pkgs[@]}" || verbose "Tauri-Deps ✓"
            ;;
        fedora)
            local tauri_pkgs=()
            for pkg in webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel patchelf openssl-devel; do
                if ! rpm -q "$pkg" &>/dev/null 2>&1; then
                    tauri_pkgs+=("$pkg")
                fi
            done
            [[ ${#tauri_pkgs[@]} -gt 0 ]] && pkg_install "${tauri_pkgs[@]}" || verbose "Tauri-Deps ✓"
            ;;
    esac

    # ── Node.js ───────────────────────────────────────────────
    if ! command -v node &>/dev/null; then
        log_warn "Node.js nicht gefunden – wird installiert..."
        ensure_cmd node "nodejs npm" "nodejs npm" "nodejs npm"
    else
        verbose "Node.js $(node --version) ✓"
    fi

    if ! command -v npm &>/dev/null; then
        ensure_cmd npm "npm" "npm" "npm"
    fi

    # ── Rust/Cargo ────────────────────────────────────────────
    if ! command -v rustc &>/dev/null || ! command -v cargo &>/dev/null; then
        log_warn "Rust nicht gefunden – wird via rustup installiert..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        source "$HOME/.cargo/env"
        log "Rust $(rustc --version | cut -d' ' -f2) installiert"
    else
        verbose "Rust $(rustc --version | cut -d' ' -f2) ✓"
    fi

    # ── RPM-Build-Tools ───────────────────────────────────────
    if $BUILD_RPM; then
        ensure_cmd rpmbuild "rpm-tools" "rpm" "rpm-build"
    fi

    # ── Windows Cross-Compile ─────────────────────────────────
    if $BUILD_EXE; then
        log_info "Windows Cross-Compile Abhängigkeiten prüfen..."

        # MinGW Compiler
        if ! command -v x86_64-w64-mingw32-gcc &>/dev/null; then
            log_warn "MinGW Cross-Compiler nicht gefunden – wird installiert..."
            case "$DISTRO" in
                arch)   pkg_install mingw-w64-gcc ;;
                debian) pkg_install gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64 ;;
                fedora) pkg_install mingw64-gcc mingw64-gcc-c++ ;;
            esac
        else
            verbose "MinGW ✓"
        fi

        # Rust windows target
        if ! rustup target list --installed 2>/dev/null | grep -q x86_64-pc-windows-gnu; then
            log_info "Installiere Rust Windows-Target..."
            rustup target add x86_64-pc-windows-gnu
        else
            verbose "Rust x86_64-pc-windows-gnu ✓"
        fi
    fi

    # ── GitHub CLI ────────────────────────────────────────────
    if $CREATE_RELEASE; then
        if ! command -v gh &>/dev/null; then
            log_warn "GitHub CLI nicht gefunden – wird installiert..."
            case "$DISTRO" in
                arch)
                    pkg_install github-cli
                    ;;
                debian)
                    # GitHub CLI offizielles Repo
                    if [[ ! -f /etc/apt/sources.list.d/github-cli.list ]] && [[ ! -f /etc/apt/sources.list.d/github-cli.sources ]]; then
                        log_info "Füge GitHub CLI Repository hinzu..."
                        curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg 2>/dev/null
                        sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg
                        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | \
                            sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
                    fi
                    pkg_install gh
                    ;;
                fedora)
                    sudo dnf config-manager --add-repo https://cli.github.com/packages/rpm/gh-cli.repo 2>/dev/null || true
                    pkg_install gh
                    ;;
            esac
        else
            verbose "GitHub CLI $(gh --version | head -1 | awk '{print $3}') ✓"
        fi

        # Auth prüfen
        if ! gh auth status &>/dev/null 2>&1; then
            log_warn "GitHub CLI nicht authentifiziert."
            log_info "Starte Login..."
            gh auth login
        fi
    fi

    # ── Tauri CLI (npm, lokal) ────────────────────────────────
    if ! (cd "$FRONTEND_DIR" && npx tauri --version &>/dev/null 2>&1); then
        log_info "Tauri CLI wird über npm installiert..."
        cd "$FRONTEND_DIR"
        npm install --save-dev @tauri-apps/cli 2>/dev/null || true
    else
        verbose "Tauri CLI $(cd "$FRONTEND_DIR" && npx tauri --version 2>/dev/null) ✓"
    fi

    echo ""
    log "Alle Abhängigkeiten erfüllt"
    log_info "App: ${APP_NAME} v${APP_VERSION}"
    log_info "Distro: ${PRETTY_NAME:-$DISTRO}"
}

# ── Frontend bauen ───────────────────────────────────────────
build_frontend() {
    if $SKIP_FRONTEND; then
        log_info "Frontend-Build übersprungen (--skip-frontend)"
        return
    fi

    log_step "Frontend bauen"
    cd "$FRONTEND_DIR"

    if [[ ! -d "node_modules" ]]; then
        log_info "npm install..."
        npm install
    fi

    log_info "npm run build..."
    npm run build
    log "Frontend erfolgreich gebaut"
}

# ── Output-Verzeichnis vorbereiten ───────────────────────────
prepare_output() {
    mkdir -p "$OUTPUT_DIR"
    log_info "Artefakte werden in $OUTPUT_DIR gesammelt"
}

# ── Tauri Build (Linux-native: deb) ───────────────────────────
build_linux() {
    if ! $BUILD_DEB; then
        return
    fi

    log_step "Linux-Pakete bauen (deb)"
    cd "$FRONTEND_DIR"

    log_info "Baue deb..."
    npx tauri build --bundles deb 2>&1 | if $VERBOSE; then cat; else tail -5; fi

    find "$TAURI_DIR/target/release/bundle/deb" -name "*.deb" -exec cp {} "$OUTPUT_DIR/" \; 2>/dev/null && \
        log "DEB-Paket erstellt" || log_err "DEB-Paket nicht gefunden"
}

# ── RPM bauen ────────────────────────────────────────────────
build_rpm() {
    if ! $BUILD_RPM; then return; fi

    log_step "RPM-Paket bauen"
    cd "$FRONTEND_DIR"

    npx tauri build --bundles rpm 2>&1 | if $VERBOSE; then cat; else tail -5; fi

    find "$TAURI_DIR/target/release/bundle/rpm" -name "*.rpm" -exec cp {} "$OUTPUT_DIR/" \; 2>/dev/null && \
        log "RPM-Paket erstellt" || log_err "RPM-Paket nicht gefunden"
}

# ── Windows .exe (Cross-Compile) ─────────────────────────────
build_windows() {
    if ! $BUILD_EXE; then return; fi

    log_step "Windows .exe bauen (Cross-Compile)"
    cd "$TAURI_DIR"

    # Sicherstellen, dass das Target installiert ist
    if ! rustup target list --installed 2>/dev/null | grep -q "x86_64-pc-windows-gnu"; then
        log_info "Installiere Windows-Target..."
        rustup target add x86_64-pc-windows-gnu
    fi

    # Tauri cross-compile unterstützt --bundles nsis/msi nicht auf Linux.
    # Daher: cargo build für die nackte .exe (ohne Installer-Wrapper).
    log_info "Cross-compile Tauri binary → x86_64-pc-windows-gnu..."

    # MinGW Linker-Konfiguration
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="x86_64-w64-mingw32-gcc"

    cargo build --release --target x86_64-pc-windows-gnu 2>&1 | if $VERBOSE; then cat; else tail -5; fi

    # .exe in Output kopieren
    local found=false
    for pattern in "busbooker-pro.exe" "busbooker_pro.exe"; do
        local exe_path="$TAURI_DIR/target/x86_64-pc-windows-gnu/release/$pattern"
        if [[ -f "$exe_path" ]]; then
            local dest_name="${APP_NAME// /-}_${APP_VERSION}_x86_64-setup.exe"
            cp "$exe_path" "$OUTPUT_DIR/$dest_name"
            log "Windows Binary erstellt → $dest_name"
            found=true
            break
        fi
    done

    if ! $found; then
        # Suche breiter
        local exe
        exe=$(find "$TAURI_DIR/target/x86_64-pc-windows-gnu/release" -maxdepth 1 -name "*.exe" ! -name "*.d" 2>/dev/null | head -1)
        if [[ -n "$exe" ]]; then
            local dest_name="${APP_NAME// /-}_${APP_VERSION}_x86_64-setup.exe"
            cp "$exe" "$OUTPUT_DIR/$dest_name"
            log "Windows Binary erstellt → $dest_name"
        else
            log_err "Keine .exe gefunden"
        fi
    fi
}

# ── Artefakte zusammenfassen ─────────────────────────────────
show_artifacts() {
    log_step "Erstellte Artefakte"

    if [[ ! -d "$OUTPUT_DIR" ]] || [[ -z "$(ls -A "$OUTPUT_DIR" 2>/dev/null)" ]]; then
        log_warn "Keine Artefakte gefunden"
        return
    fi

    echo ""
    local total_size=0
    while IFS= read -r file; do
        local size
        size=$(stat -c%s "$file" 2>/dev/null || stat -f%z "$file" 2>/dev/null || echo 0)
        local human_size
        human_size=$(numfmt --to=iec "$size" 2>/dev/null || echo "${size}B")
        local basename
        basename=$(basename "$file")
        local ext="${basename##*.}"

        case "$ext" in
            deb)      echo -e "    ${GREEN}📦 $basename${NC} ($human_size) [Debian/Ubuntu]" ;;
            rpm)      echo -e "    ${GREEN}📦 $basename${NC} ($human_size) [Fedora/RHEL]" ;;
            exe)      echo -e "    ${GREEN}🪟 $basename${NC} ($human_size) [Windows]" ;;
            *)        echo -e "    ${GREEN}📄 $basename${NC} ($human_size)" ;;
        esac
        total_size=$((total_size + size))
    done < <(find "$OUTPUT_DIR" -type f | sort)

    local total_human
    total_human=$(numfmt --to=iec "$total_size" 2>/dev/null || echo "${total_size}B")
    echo ""
    log_info "Gesamt: $total_human in $OUTPUT_DIR/"
}

# ── GitHub Release erstellen ─────────────────────────────────
create_github_release() {
    if ! $CREATE_RELEASE; then return; fi

    log_step "GitHub Release erstellen"

    # Tag bestimmen
    if [[ -z "$RELEASE_TAG" ]]; then
        RELEASE_TAG="v${APP_VERSION}"
    fi

    # Titel bestimmen
    if [[ -z "$RELEASE_TITLE" ]]; then
        RELEASE_TITLE="${APP_NAME} ${RELEASE_TAG}"
    fi

    # Release Notes
    if [[ -z "$RELEASE_NOTES" ]]; then
        RELEASE_NOTES="## ${APP_NAME} ${RELEASE_TAG}

### Artefakte
| Datei | Plattform |
|-------|-----------|"
        for file in "$OUTPUT_DIR"/*; do
            [[ -f "$file" ]] || continue
            local bn ext platform
            bn=$(basename "$file")
            ext="${bn##*.}"
            case "$ext" in
                deb)      platform="Debian / Ubuntu" ;;
                rpm)      platform="Fedora / RHEL / openSUSE" ;;
                exe)      platform="Windows" ;;
                *)        platform="Sonstige" ;;
            esac
            RELEASE_NOTES+="\n| \`$bn\` | $platform |"
        done
        RELEASE_NOTES+="\n\n---\n*Erstellt am $(date '+%d.%m.%Y %H:%M')*"
    fi

    # gh release Argumente aufbauen
    local gh_args=("release" "create" "$RELEASE_TAG")
    gh_args+=("--title" "$RELEASE_TITLE")
    gh_args+=("--notes" "$(echo -e "$RELEASE_NOTES")")

    if $RELEASE_DRAFT; then
        gh_args+=("--draft")
    fi

    if $RELEASE_PRERELEASE; then
        gh_args+=("--prerelease")
    fi

    # Artefakte anhängen
    for file in "$OUTPUT_DIR"/*; do
        [[ -f "$file" ]] && gh_args+=("$file")
    done

    log_info "Tag: $RELEASE_TAG"
    log_info "Titel: $RELEASE_TITLE"
    $RELEASE_DRAFT && log_info "Modus: Entwurf"
    $RELEASE_PRERELEASE && log_info "Modus: Pre-Release"

    # Release erstellen
    if gh "${gh_args[@]}"; then
        log "GitHub Release erstellt!"
        local release_url
        release_url=$(gh release view "$RELEASE_TAG" --json url -q '.url' 2>/dev/null || echo "")
        if [[ -n "$release_url" ]]; then
            log_info "URL: $release_url"
        fi
    else
        log_err "GitHub Release konnte nicht erstellt werden"
        exit 1
    fi
}

# ── Checksums ────────────────────────────────────────────────
generate_checksums() {
    if [[ -z "$(ls -A "$OUTPUT_DIR" 2>/dev/null)" ]]; then
        return
    fi

    log_info "Checksums generieren..."
    cd "$OUTPUT_DIR"
    sha256sum * > SHA256SUMS.txt 2>/dev/null || shasum -a 256 * > SHA256SUMS.txt 2>/dev/null || true
    if [[ -f "SHA256SUMS.txt" ]]; then
        log "SHA256SUMS.txt erstellt"
    fi
    cd "$SCRIPT_DIR"
}

# ── Hauptprogramm ────────────────────────────────────────────
main() {
    echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║      ${APP_NAME} v${APP_VERSION} – Build & Release              ║${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo ""

    local targets=()
    $BUILD_DEB && targets+=("deb")
    $BUILD_RPM && targets+=("rpm")
    $BUILD_EXE && targets+=("exe")
    log_info "Ziele: ${targets[*]}"
    $CREATE_RELEASE && log_info "GitHub Release: Ja"

    local start_time
    start_time=$(date +%s)

    detect_distro
    install_system_deps
    build_frontend
    prepare_output
    build_linux
    build_rpm
    build_windows
    generate_checksums
    show_artifacts
    create_github_release

    local end_time elapsed
    end_time=$(date +%s)
    elapsed=$((end_time - start_time))
    local mins=$((elapsed / 60))
    local secs=$((elapsed % 60))

    echo ""
    log "Fertig! (${mins}m ${secs}s)"
}

main
