# Datensicherheitskonzept
## IT-Sicherheitsrichtlinien und Technisch-Organisatorische Maßnahmen (TOMs)

**Projekt:** BusBooker Pro - Online-Buchungssystem  
**Verantwortlicher:** Sonnenschein Reisen GmbH  
**Version:** 1.0  
**Stand:** 05.02.2026  
**Klassifizierung:** Vertraulich

---

## Inhaltsverzeichnis

1. [Einleitung](#1-einleitung)
2. [Schutzziele](#2-schutzziele)
3. [Risikoanalyse](#3-risikoanalyse)
4. [Technische Maßnahmen](#4-technische-maßnahmen)
5. [Organisatorische Maßnahmen](#5-organisatorische-maßnahmen)
6. [Zugriffskontrolle](#6-zugriffskontrolle)
7. [Datensicherung und Recovery](#7-datensicherung-und-recovery)
8. [Incident Response](#8-incident-response)
9. [Compliance und Audits](#9-compliance-und-audits)
10. [Anhang: Checklisten](#10-anhang-checklisten)

---

## 1. Einleitung

### 1.1 Zweck des Dokuments
Dieses Datensicherheitskonzept definiert die technischen und organisatorischen Maßnahmen (TOMs) zum Schutz personenbezogener Daten und zur Gewährleistung der IT-Sicherheit des BusBooker Pro Systems gemäß:
- Art. 32 DSGVO (Sicherheit der Verarbeitung)
- § 64 BDSG (Technische und organisatorische Maßnahmen)
- Stand der Technik

### 1.2 Geltungsbereich
- BusBooker Pro Desktop-Anwendung
- Zugehörige Datenbanken (lokal und ggf. Server)
- Schnittstellen zu externen Diensten
- Administrative Zugänge
- Backup-Systeme

### 1.3 Verantwortlichkeiten

| Rolle | Verantwortung |
|-------|---------------|
| Geschäftsführung | Gesamtverantwortung, Ressourcenbereitstellung |
| IT-Leitung | Technische Umsetzung, Systembetrieb |
| Datenschutzbeauftragter | Überwachung, Beratung, Schulung |
| Mitarbeiter | Einhaltung der Richtlinien |

---

## 2. Schutzziele

### 2.1 CIA-Triade

| Schutzziel | Definition | Maßnahmen |
|------------|------------|-----------|
| **Vertraulichkeit** (Confidentiality) | Schutz vor unbefugtem Zugriff | Verschlüsselung, Zugriffskontrollen |
| **Integrität** (Integrity) | Schutz vor unbefugter Änderung | Prüfsummen, Audit-Logs, Validierung |
| **Verfügbarkeit** (Availability) | Sicherstellung der Erreichbarkeit | Redundanz, Backups, Monitoring |

### 2.2 Erweiterte Schutzziele

| Schutzziel | Definition |
|------------|------------|
| **Authentizität** | Echtheit und Glaubwürdigkeit nachweisbar |
| **Verbindlichkeit** | Aktionen sind nachweisbar und nicht abstreitbar |
| **Belastbarkeit** | System widersteht Angriffen und Last |

### 2.3 Schutzbedarfsfeststellung

| Datenkategorie | Vertraulichkeit | Integrität | Verfügbarkeit |
|----------------|-----------------|------------|---------------|
| Kundenstammdaten | Hoch | Hoch | Normal |
| Authentifizierungsdaten | Sehr hoch | Sehr hoch | Hoch |
| Buchungsdaten | Hoch | Hoch | Hoch |
| Zahlungsdaten | Sehr hoch | Sehr hoch | Hoch |
| Systemkonfiguration | Hoch | Sehr hoch | Hoch |
| Log-Daten | Normal | Hoch | Normal |

---

## 3. Risikoanalyse

### 3.1 Bedrohungsszenarien

| ID | Bedrohung | Wahrscheinlichkeit | Auswirkung | Risiko |
|----|-----------|-------------------|------------|--------|
| B-001 | SQL-Injection | Mittel | Sehr hoch | Hoch |
| B-002 | Cross-Site-Scripting (XSS) | Mittel | Hoch | Hoch |
| B-003 | Brute-Force-Angriff | Hoch | Mittel | Hoch |
| B-004 | Datenbankdiebstahl | Niedrig | Sehr hoch | Mittel |
| B-005 | Session-Hijacking | Niedrig | Hoch | Mittel |
| B-006 | Denial of Service | Mittel | Mittel | Mittel |
| B-007 | Social Engineering | Mittel | Hoch | Hoch |
| B-008 | Insider-Bedrohung | Niedrig | Sehr hoch | Mittel |
| B-009 | Malware/Ransomware | Mittel | Sehr hoch | Hoch |
| B-010 | Datenverlust (Hardware) | Niedrig | Hoch | Mittel |

### 3.2 Risikomatrix

```
Auswirkung
    ▲
Sehr│ B-004    B-001    B-009
Hoch│          B-008
    │
Hoch│ B-005    B-002    B-007
    │          B-003
    │
Mit-│          B-006
tel │
    │
Nie-│ B-010
drig│
    └────────────────────────────▶
        Niedrig  Mittel  Hoch
              Wahrscheinlichkeit
```

### 3.3 Risikobehandlung

| Risiko | Strategie | Maßnahmen |
|--------|-----------|-----------|
| B-001, B-002 | Vermeiden | Input-Validierung, Prepared Statements |
| B-003 | Verringern | Rate-Limiting, Account-Sperrung |
| B-004 | Verringern | Verschlüsselung, Zugriffskontrollen |
| B-007 | Verringern | Schulungen, Awareness |
| B-009 | Übertragen/Verringern | Backups, Antivirus, Versicherung |
| B-010 | Übertragen | Backups, redundante Speicherung |

---

## 4. Technische Maßnahmen

### 4.1 Verschlüsselung

#### 4.1.1 Datenübertragung (Data in Transit)
| Aspekt | Maßnahme | Details |
|--------|----------|---------|
| Protokoll | TLS 1.3 | Mindestens TLS 1.2 |
| Zertifikate | X.509 | Von vertrauenswürdiger CA |
| Cipher Suites | Moderne Algorithmen | ECDHE, AES-256-GCM, SHA-384 |
| HSTS | Aktiviert | Max-age: 31536000 |

#### 4.1.2 Datenspeicherung (Data at Rest)
| Datentyp | Verschlüsselung | Algorithmus |
|----------|-----------------|-------------|
| Passwörter | Hash + Salt | Argon2id |
| Sensible Daten | Verschlüsselt | AES-256-GCM |
| Datenbank | Verschlüsselt | SQLCipher / PostgreSQL TDE |
| Backups | Verschlüsselt | AES-256 |

#### 4.1.3 Schlüsselmanagement
- Schlüssel werden getrennt von Daten gespeichert
- Regelmäßige Schlüsselrotation (jährlich)
- Zugriff auf Schlüssel nur für autorisiertes Personal
- Sichere Aufbewahrung von Master-Keys

### 4.2 Authentifizierung

#### 4.2.1 Passwort-Policy
| Anforderung | Wert |
|-------------|------|
| Mindestlänge | 12 Zeichen |
| Komplexität | Groß-/Kleinbuchstaben, Zahlen, Sonderzeichen |
| Maximales Alter | 365 Tage (empfohlen) |
| Passwort-Historie | Letzte 5 Passwörter gesperrt |
| Sperrung | Nach 5 Fehlversuchen für 15 Minuten |

#### 4.2.2 Session-Management
| Aspekt | Maßnahme |
|--------|----------|
| Token-Typ | JWT (JSON Web Token) |
| Lebensdauer | Access Token: 15 min, Refresh Token: 7 Tage |
| Speicherung | Secure, HttpOnly Cookies |
| Invalidierung | Bei Logout, Passwortänderung |
| Schutz | CSRF-Tokens |

#### 4.2.3 Multi-Faktor-Authentifizierung (Optional)
- TOTP (Time-based One-Time Password)
- Unterstützte Apps: Google Authenticator, Authy
- Backup-Codes für Notfälle

### 4.3 Eingabevalidierung

#### 4.3.1 Schutz vor Injection
```rust
// Beispiel: Prepared Statements in Rust
let query = sqlx::query!(
    "SELECT * FROM users WHERE email = $1",
    email  // Parameter wird sicher eingebunden
);
```

#### 4.3.2 Validierungsregeln
| Feldtyp | Validierung |
|---------|-------------|
| E-Mail | RFC 5322 Format, Existenzprüfung |
| Telefon | Nur Ziffern, +, -, () |
| Namen | Nur Buchstaben, Bindestriche, max. 100 Zeichen |
| Datum | ISO 8601 Format |
| Zahlen | Bereichsprüfung, Typ-Check |
| Freitext | HTML-Encoding, Längenbegrenzung |

#### 4.3.3 Output-Encoding
- HTML: `&lt;` `&gt;` `&amp;` `&quot;`
- JavaScript: JSON.stringify() mit Escaping
- SQL: Parametrisierte Abfragen
- URLs: URL-Encoding

### 4.4 Netzwerksicherheit

#### 4.4.1 Firewall-Regeln
| Richtung | Port | Dienst | Erlaubt |
|----------|------|--------|---------|
| Eingehend | 443 | HTTPS | Ja |
| Eingehend | 80 | HTTP | Redirect zu HTTPS |
| Eingehend | 22 | SSH | Nur Admin-IPs |
| Ausgehend | 443 | HTTPS | Ja (APIs) |
| Ausgehend | 587 | SMTP | Ja (E-Mail) |
| Alle anderen | * | * | Blockiert |

#### 4.4.2 Rate-Limiting
| Endpunkt | Limit | Zeitraum |
|----------|-------|----------|
| Login | 5 Versuche | 15 Minuten |
| Registrierung | 3 Anfragen | 1 Stunde |
| API allgemein | 100 Anfragen | 1 Minute |
| Passwort-Reset | 3 Anfragen | 1 Stunde |

### 4.5 Logging und Monitoring

#### 4.5.1 Zu protokollierende Ereignisse
| Ereignis | Log-Level | Aufbewahrung |
|----------|-----------|--------------|
| Login erfolgreich | INFO | 90 Tage |
| Login fehlgeschlagen | WARNING | 90 Tage |
| Passwortänderung | INFO | 1 Jahr |
| Admin-Aktionen | INFO | 1 Jahr |
| Buchungen | INFO | 10 Jahre |
| Systemfehler | ERROR | 90 Tage |
| Sicherheitsvorfälle | CRITICAL | 3 Jahre |

#### 4.5.2 Log-Format (JSON)
```json
{
  "timestamp": "2026-02-05T14:30:00Z",
  "level": "INFO",
  "event": "user_login",
  "user_id": "uuid",
  "ip_address": "192.168.1.1",
  "user_agent": "...",
  "success": true,
  "details": {}
}
```

#### 4.5.3 Alerting
| Ereignis | Schwellwert | Aktion |
|----------|-------------|--------|
| Failed Logins | > 10 pro Minute | E-Mail an Admin |
| CPU-Auslastung | > 90% für 5 Min | Benachrichtigung |
| Speicherplatz | < 10% frei | Benachrichtigung |
| Fehlerrate | > 5% der Requests | Alarm |

---

## 5. Organisatorische Maßnahmen

### 5.1 Richtlinien und Verfahren

#### 5.1.1 Dokumentierte Richtlinien
- [x] IT-Sicherheitsrichtlinie
- [x] Passwort-Richtlinie
- [x] Datenschutzrichtlinie
- [x] Notfallplan
- [x] Backup-Richtlinie
- [x] Incident-Response-Plan

#### 5.1.2 Verfahrensanweisungen
- Mitarbeiter-Onboarding (IT-Sicherheit)
- Mitarbeiter-Offboarding (Zugriffsentzug)
- Umgang mit Sicherheitsvorfällen
- Datenlöschung und -vernichtung
- Schlüsselverwaltung

### 5.2 Mitarbeiterschulung

| Schulungsinhalt | Zielgruppe | Häufigkeit |
|-----------------|------------|------------|
| IT-Sicherheit Basics | Alle | Einstellung + jährlich |
| Phishing-Awareness | Alle | Halbjährlich |
| DSGVO-Grundlagen | Alle | Einstellung + bei Änderungen |
| Admin-Sicherheit | IT-Personal | Einstellung + jährlich |
| Incident Response | IT-Personal | Jährlich + bei Änderungen |

### 5.3 Clean Desk Policy

- Bildschirmsperre bei Verlassen des Arbeitsplatzes (max. 5 Min inaktiv)
- Keine Passworte auf Zetteln
- Vertrauliche Dokumente in verschlossenen Schränken
- Shreddern von nicht mehr benötigten Dokumenten

### 5.4 Physische Sicherheit

| Maßnahme | Beschreibung |
|----------|--------------|
| Zutrittskontrollen | Schlüsselkarten für Serverräume |
| Überwachung | Kamera an kritischen Punkten |
| Umgebungsschutz | Klimaanlage, Rauchmelder, USV |
| Besuchermanagement | Anmeldung, Begleitung |

---

## 6. Zugriffskontrolle

### 6.1 Rollenbasierte Zugriffskontrolle (RBAC)

#### 6.1.1 Definierte Rollen

| Rolle | Beschreibung | Berechtigungen |
|-------|--------------|----------------|
| **Kunde** | Registrierte Benutzer | Eigenes Profil, eigene Buchungen, Reisen ansehen |
| **Support** | Kundenservice-Mitarbeiter | Alle Buchungen lesen, Kundenprofile lesen |
| **Buchhalter** | Buchhaltungsabteilung | Zahlungen, Rechnungen, Berichte |
| **Reise-Manager** | Produktmanagement | Reisen CRUD, Busse CRUD |
| **Admin** | IT-Administrator | Vollzugriff, Benutzerverwaltung |
| **Super-Admin** | System-Administrator | Systemkonfiguration, Audit |

#### 6.1.2 Berechtigungsmatrix

| Ressource | Kunde | Support | Buchhalter | Manager | Admin |
|-----------|-------|---------|------------|---------|-------|
| Eigenes Profil | RW | R | - | - | RW |
| Andere Profile | - | R | - | - | RW |
| Eigene Buchungen | RW | - | - | - | RW |
| Alle Buchungen | - | R | R | R | RW |
| Reisen (Ansicht) | R | R | R | R | R |
| Reisen (Bearbeiten) | - | - | - | RW | RW |
| Zahlungen | - | - | R | - | RW |
| Berichte | - | - | R | R | R |
| Benutzerverwaltung | - | - | - | - | RW |
| Systemeinstellungen | - | - | - | - | RW |

*R = Lesen, W = Schreiben, RW = Lesen und Schreiben*

### 6.2 Prinzipien

#### 6.2.1 Least Privilege (Minimale Rechte)
- Benutzer erhalten nur die minimal notwendigen Berechtigungen
- Berechtigungen werden regelmäßig überprüft (quartalsweise)
- Temporäre Berechtigungen werden automatisch entzogen

#### 6.2.2 Separation of Duties (Funktionstrennung)
- Keine Person hat alleinige Kontrolle über kritische Prozesse
- Vier-Augen-Prinzip bei:
  - Benutzererstellung mit Admin-Rechten
  - Massenlöschungen
  - Konfigurationsänderungen

#### 6.2.3 Need-to-Know
- Zugriff auf Daten nur bei dienstlicher Notwendigkeit
- Dokumentation der Zugriffsberechtigungen

### 6.3 Zugriffsverwaltung

#### 6.3.1 Benutzerlebenszyklus
```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Antrag     │───▶│  Genehmigung│───▶│  Einrichtung│
└─────────────┘    └─────────────┘    └─────────────┘
                                            │
                                            ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Entzug    │◀───│  Review     │◀───│   Nutzung   │
└─────────────┘    └─────────────┘    └─────────────┘
```

#### 6.3.2 Offboarding-Prozess
1. Benachrichtigung durch HR an IT
2. Deaktivierung aller Zugänge am letzten Arbeitstag
3. Passwort-Reset für geteilte Zugänge
4. Archivierung persönlicher Daten
5. Dokumentation

---

## 7. Datensicherung und Recovery

### 7.1 Backup-Strategie

#### 7.1.1 Backup-Typen

| Typ | Häufigkeit | Aufbewahrung | Speicherort |
|-----|------------|--------------|-------------|
| Vollbackup | Wöchentlich (SO) | 12 Wochen | Offsite |
| Inkrementell | Täglich | 4 Wochen | Lokal + Offsite |
| Transaktionslogs | Stündlich | 1 Woche | Lokal |
| Konfiguration | Bei Änderung | 10 Versionen | Versionskontrolle |

#### 7.1.2 3-2-1-Regel
- **3** Kopien der Daten
- **2** verschiedene Medientypen
- **1** Kopie offsite (geografisch getrennt)

### 7.2 Backup-Prozess

#### 7.2.1 Automatisierung
```bash
# Beispiel: Tägliches Backup-Skript
#!/bin/bash
DATE=$(date +%Y%m%d)
BACKUP_DIR="/backup/$DATE"

# Datenbank-Backup
pg_dump -Fc busbooker > "$BACKUP_DIR/db.dump"

# Verschlüsselung
gpg --encrypt --recipient backup@firma.de "$BACKUP_DIR/db.dump"

# Transfer zu Offsite
rsync -avz "$BACKUP_DIR" offsite:/backups/

# Cleanup alte Backups
find /backup -mtime +30 -delete
```

#### 7.2.2 Verschlüsselung
- Backup-Verschlüsselung: AES-256
- Schlüsselspeicherung: Getrennt vom Backup
- Schlüssel-Escrow: Sicherer Tresor

### 7.3 Recovery (Wiederherstellung)

#### 7.3.1 Recovery Time Objective (RTO)

| Szenario | RTO | RPO |
|----------|-----|-----|
| Einzelne Datei | 1 Stunde | 1 Stunde |
| Datenbank-Corruption | 4 Stunden | 1 Stunde |
| Server-Ausfall | 8 Stunden | 24 Stunden |
| Kompletter Datenverlust | 24 Stunden | 24 Stunden |

*RTO = Recovery Time Objective (max. Ausfallzeit)*
*RPO = Recovery Point Objective (max. Datenverlust)*

#### 7.3.2 Recovery-Prozess
1. **Identifikation:** Art und Umfang des Datenverlusts
2. **Entscheidung:** Welches Backup verwenden
3. **Isolation:** Betroffene Systeme isolieren
4. **Wiederherstellung:** Backup einspielen
5. **Validierung:** Integrität prüfen
6. **Freigabe:** System wieder in Betrieb nehmen
7. **Dokumentation:** Vorfall dokumentieren

#### 7.3.3 Backup-Tests
- **Häufigkeit:** Quartalsweise
- **Umfang:** Vollständige Wiederherstellung auf Testsystem
- **Dokumentation:** Testprotokoll mit Dauer und Ergebnis

---

## 8. Incident Response

### 8.1 Incident-Klassifizierung

| Schweregrad | Beschreibung | Reaktionszeit | Beispiele |
|-------------|--------------|---------------|-----------|
| **Kritisch** | Systemausfall, Datenverlust | Sofort (< 15 Min) | Ransomware, Datenleck |
| **Hoch** | Teilausfall, Sicherheitslücke | < 1 Stunde | Erfolgreicher Angriff |
| **Mittel** | Eingeschränkte Funktion | < 4 Stunden | DDoS, Fehlkonfiguration |
| **Niedrig** | Anomalie ohne Auswirkung | < 24 Stunden | Fehlgeschlagene Logins |

### 8.2 Incident-Response-Prozess

```
┌─────────────┐
│ Erkennung   │  Log-Analyse, Monitoring, Meldung
└──────┬──────┘
       ▼
┌─────────────┐
│ Bewertung   │  Klassifizierung, Priorität
└──────┬──────┘
       ▼
┌─────────────┐
│ Eindämmung  │  Isolation, Schadensbegrenzung
└──────┬──────┘
       ▼
┌─────────────┐
│ Beseitigung │  Ursache beheben, Systeme bereinigen
└──────┬──────┘
       ▼
┌─────────────┐
│ Wiederher-  │  Systeme wiederherstellen, Test
│ stellung    │
└──────┬──────┘
       ▼
┌─────────────┐
│ Nachbe-     │  Dokumentation, Lessons Learned
│ reitung     │
└─────────────┘
```

### 8.3 Meldepflichten

#### 8.3.1 DSGVO-Meldepflicht (Art. 33, 34)

| Bedingung | Meldung an | Frist |
|-----------|------------|-------|
| Risiko für Betroffene | Aufsichtsbehörde | 72 Stunden |
| Hohes Risiko | Betroffene Personen | Unverzüglich |

#### 8.3.2 Kontaktdaten

| Ansprechpartner | Kontakt | Erreichbarkeit |
|-----------------|---------|----------------|
| IT-Notfall intern | it-notfall@firma.de | 24/7 |
| Datenschutzbeauftragter | datenschutz@firma.de | Geschäftszeiten |
| Aufsichtsbehörde | [Landes-DSB] | Geschäftszeiten |
| Geschäftsführung | [Kontakt] | Geschäftszeiten |

### 8.4 Incident-Dokumentation

Jeder Sicherheitsvorfall ist zu dokumentieren:

- [ ] Datum und Uhrzeit der Erkennung
- [ ] Art des Vorfalls
- [ ] Betroffene Systeme/Daten
- [ ] Entdeckungsweg
- [ ] Ergriffene Maßnahmen
- [ ] Beteiligte Personen
- [ ] Zeitlicher Ablauf
- [ ] Ursache (Root Cause)
- [ ] Empfehlungen zur Vermeidung

---

## 9. Compliance und Audits

### 9.1 Compliance-Anforderungen

| Anforderung | Beschreibung | Status |
|-------------|--------------|--------|
| DSGVO | Datenschutz-Grundverordnung | ☐ Umgesetzt |
| BDSG | Bundesdatenschutzgesetz | ☐ Umgesetzt |
| PCI DSS | Payment Card Industry (via Dienstleister) | ☐ Umgesetzt |
| AO/HGB | Aufbewahrungspflichten | ☐ Umgesetzt |

### 9.2 Regelmäßige Überprüfungen

| Aktivität | Häufigkeit | Verantwortlich |
|-----------|------------|----------------|
| Sicherheitsüberprüfung | Quartalsweise | IT |
| Berechtigungsreview | Quartalsweise | IT + Fachabteilung |
| Penetrationstest | Jährlich | Externer Dienstleister |
| Backup-Test | Quartalsweise | IT |
| Notfallübung | Jährlich | IT + Geschäftsführung |
| DSGVO-Audit | Jährlich | DSB |

### 9.3 Audit-Checkliste

#### 9.3.1 Technische Prüfpunkte
- [ ] Alle Systeme auf aktuellem Patch-Level
- [ ] Verschlüsselung korrekt konfiguriert
- [ ] Zugriffsrechte aktuell und dokumentiert
- [ ] Logs werden aufgezeichnet und geschützt
- [ ] Backups funktionieren und sind getestet
- [ ] Firewalls korrekt konfiguriert
- [ ] Passwort-Policy wird durchgesetzt

#### 9.3.2 Organisatorische Prüfpunkte
- [ ] Richtlinien aktuell und bekannt
- [ ] Schulungen durchgeführt und dokumentiert
- [ ] Incident-Prozess funktionsfähig
- [ ] Verzeichnis der Verarbeitungstätigkeiten aktuell
- [ ] Auftragsverarbeitungsverträge vorhanden

---

## 10. Anhang: Checklisten

### 10.1 Sicherheits-Checkliste für Entwickler

#### Bei jedem Release:
- [ ] Keine hartcodierten Passwörter/Secrets
- [ ] Alle Eingaben validiert (Whitelist)
- [ ] Prepared Statements für Datenbankzugriffe
- [ ] Output-Encoding implementiert
- [ ] CSRF-Schutz aktiv
- [ ] Sensitive Daten nicht in Logs
- [ ] Dependencies auf Sicherheitslücken geprüft
- [ ] Security-Headers gesetzt
- [ ] Error-Messages ohne sensible Infos

### 10.2 Sicherheits-Checkliste für Administratoren

#### Täglich:
- [ ] Logs auf Anomalien prüfen
- [ ] Backup-Status prüfen
- [ ] System-Monitoring prüfen

#### Wöchentlich:
- [ ] Sicherheitsupdates prüfen und einspielen
- [ ] Fehlgeschlagene Logins analysieren
- [ ] Speicherauslastung prüfen

#### Monatlich:
- [ ] Benutzerkonten überprüfen (inaktive deaktivieren)
- [ ] Zertifikats-Ablaufdaten prüfen
- [ ] Sicherheitsrichtlinien-Compliance prüfen

### 10.3 Sicherheits-Checkliste für Benutzer

- [ ] Starkes, einzigartiges Passwort verwenden
- [ ] Passwort nicht teilen oder aufschreiben
- [ ] Bei Verdacht auf Kompromittierung melden
- [ ] Bildschirmsperre bei Abwesenheit
- [ ] Verdächtige E-Mails melden
- [ ] Keine unbekannten USB-Sticks verwenden

---

## 11. Dokumentenhistorie

| Version | Datum | Autor | Änderungen |
|---------|-------|-------|------------|
| 0.1 | 05.02.2026 | Projektteam | Entwurf |
| 1.0 | 05.02.2026 | Projektteam | Freigabe |

---

## 12. Freigabe

| Rolle | Name | Datum | Unterschrift |
|-------|------|-------|--------------|
| Geschäftsführung | | | |
| IT-Leitung | | | |
| Datenschutzbeauftragter | | | |

---

*Dieses Dokument ist vertraulich und nur für autorisierte Personen bestimmt.*
*Nächste planmäßige Überprüfung: 05.02.2027*
