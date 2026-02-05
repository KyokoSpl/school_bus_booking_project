-- Benutzer-Tabelle
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(36) PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    vorname VARCHAR(100) NOT NULL,
    nachname VARCHAR(100) NOT NULL,
    telefon VARCHAR(20),
    strasse VARCHAR(255),
    plz VARCHAR(10),
    ort VARCHAR(100),
    land VARCHAR(100) DEFAULT 'Deutschland',
    geburtsdatum DATE,
    rolle VARCHAR(20) NOT NULL DEFAULT 'kunde',
    email_verifiziert BOOLEAN NOT NULL DEFAULT FALSE,
    aktiv BOOLEAN NOT NULL DEFAULT TRUE,
    verification_token VARCHAR(255),
    password_reset_token VARCHAR(255),
    password_reset_expires TIMESTAMP,
    erstellt_am TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    aktualisiert_am TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_email (email),
    INDEX idx_rolle (rolle)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Busse-Tabelle
CREATE TABLE IF NOT EXISTS buses (
    id VARCHAR(36) PRIMARY KEY,
    kennzeichen VARCHAR(20) NOT NULL UNIQUE,
    bezeichnung VARCHAR(100) NOT NULL,
    sitzplaetze INT NOT NULL,
    ausstattung JSON,
    baujahr INT,
    status VARCHAR(20) NOT NULL DEFAULT 'verfuegbar',
    notizen TEXT,
    erstellt_am TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    aktualisiert_am TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Reisen-Tabelle
CREATE TABLE IF NOT EXISTS trips (
    id VARCHAR(36) PRIMARY KEY,
    titel VARCHAR(255) NOT NULL,
    beschreibung TEXT NOT NULL,
    ziel VARCHAR(255) NOT NULL,
    abfahrtsort VARCHAR(255) NOT NULL,
    abfahrt_datum DATE NOT NULL,
    abfahrt_zeit TIME NOT NULL,
    rueckkehr_datum DATE NOT NULL,
    rueckkehr_zeit TIME NOT NULL,
    preis_pro_person DECIMAL(10,2) NOT NULL,
    bus_id VARCHAR(36),
    max_teilnehmer INT NOT NULL,
    aktuelle_buchungen INT NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'geplant',
    bilder JSON,
    highlights JSON,
    inkludiert JSON,
    nicht_inkludiert JSON,
    erstellt_am TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    aktualisiert_am TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (bus_id) REFERENCES buses(id) ON DELETE SET NULL,
    INDEX idx_ziel (ziel),
    INDEX idx_abfahrt_datum (abfahrt_datum),
    INDEX idx_status (status),
    INDEX idx_preis (preis_pro_person)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Buchungen-Tabelle
CREATE TABLE IF NOT EXISTS bookings (
    id VARCHAR(36) PRIMARY KEY,
    buchungsnummer VARCHAR(20) NOT NULL UNIQUE,
    kunde_id VARCHAR(36) NOT NULL,
    reise_id VARCHAR(36) NOT NULL,
    anzahl_personen INT NOT NULL,
    gesamtpreis DECIMAL(10,2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'ausstehend',
    buchungsdatum TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    mitreisende JSON,
    bemerkungen TEXT,
    zahlungsstatus VARCHAR(20) NOT NULL DEFAULT 'ausstehend',
    zahlungsmethode VARCHAR(50),
    erstellt_am TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    aktualisiert_am TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (kunde_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (reise_id) REFERENCES trips(id) ON DELETE CASCADE,
    INDEX idx_kunde (kunde_id),
    INDEX idx_reise (reise_id),
    INDEX idx_status (status),
    INDEX idx_buchungsnummer (buchungsnummer),
    INDEX idx_buchungsdatum (buchungsdatum)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Zahlungen-Tabelle
CREATE TABLE IF NOT EXISTS payments (
    id VARCHAR(36) PRIMARY KEY,
    buchung_id VARCHAR(36) NOT NULL,
    betrag DECIMAL(10,2) NOT NULL,
    methode VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'ausstehend',
    transaktions_id VARCHAR(255),
    zahlungsdetails JSON,
    erstellt_am TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    aktualisiert_am TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (buchung_id) REFERENCES bookings(id) ON DELETE CASCADE,
    INDEX idx_buchung (buchung_id),
    INDEX idx_status (status),
    INDEX idx_transaktions_id (transaktions_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
