# Lastenheft
## Online-Buchungssystem für Busreisen

**Auftraggeber:** Sonnenschein Reisen GmbH  
**Erstellt am:** 05.02.2026  
**Version:** 1.0  
**Dokumentstatus:** Freigegeben

---

## 1. Einleitung

### 1.1 Ausgangssituation
Die Sonnenschein Reisen GmbH ist ein mittelständisches Busunternehmen mit Sitz in Deutschland. Das Unternehmen bietet Busreisen im In- und Ausland an und möchte seinen Kunden eine moderne, digitale Buchungsmöglichkeit bieten. Bisher erfolgen Buchungen überwiegend telefonisch oder per E-Mail, was einen hohen administrativen Aufwand verursacht.

### 1.2 Ziel des Projekts
Implementierung eines webbasierten Buchungssystems, das Kunden ermöglicht, Busreisen online zu suchen, auszuwählen und zu buchen. Das System soll als Desktop-Anwendung mit Weboberfläche realisiert werden.

---

## 2. Beschreibung des Ist-Zustands

### 2.1 Aktuelle Geschäftsprozesse
- Buchungsanfragen erfolgen telefonisch oder per E-Mail
- Manuelle Erfassung von Kundendaten in Excel-Tabellen
- Keine automatisierte Verfügbarkeitsprüfung
- Rechnungsstellung erfolgt manuell
- Keine Online-Präsenz für Buchungen

### 2.2 Problemstellung
- Hoher manueller Verwaltungsaufwand
- Fehleranfälligkeit bei der Dateneingabe
- Begrenzte Erreichbarkeit (nur während Geschäftszeiten)
- Keine Übersicht für Kunden über verfügbare Reisen
- Verlust potenzieller Kunden durch fehlende Online-Präsenz

---

## 3. Beschreibung des Soll-Zustands

### 3.1 Funktionale Anforderungen

#### 3.1.1 Kundenfunktionen
| ID | Anforderung | Priorität |
|----|-------------|-----------|
| LA-001 | Kunden können sich registrieren und ein Benutzerkonto anlegen | Muss |
| LA-002 | Kunden können verfügbare Reisen nach Datum, Ziel und Preis suchen | Muss |
| LA-003 | Kunden können detaillierte Reiseinformationen einsehen | Muss |
| LA-004 | Kunden können Plätze für Reisen buchen | Muss |
| LA-005 | Kunden können ihre Buchungen einsehen und verwalten | Muss |
| LA-006 | Kunden erhalten automatische Buchungsbestätigungen per E-Mail | Muss |
| LA-007 | Kunden können Buchungen stornieren | Soll |
| LA-008 | Kunden können Bewertungen für durchgeführte Reisen abgeben | Kann |

#### 3.1.2 Administrationsfunktionen
| ID     | Anforderung                                                   | Priorität |
| ------ | ------------------------------------------------------------- | --------- |
| LA-009 | Administratoren können Reisen anlegen, bearbeiten und löschen | Muss      |
| LA-011 | Administratoren können Buchungen einsehen und bearbeiten      | Muss      |
| LA-012 | Administratoren können Kundendaten verwalten                  | Muss      |
| LA-013 | Administratoren können Berichte und Statistiken abrufen       | Kann      |
| LA-014 | Administratoren können Preise und Rabatte konfigurieren       | Soll      |

#### 3.1.3 Zahlungsfunktionen
| ID     | Anforderung                                              | Priorität |
| ------ | -------------------------------------------------------- | --------- |
| LA-015 | Integration von mindestens einer Zahlungsquelle (mocked) | Muss      |
| LA-016 | Zahlung per Rechnung (vereinfacht dargestellt mit email) | Soll      |
| LA-017 | Unterstützung von PayPal                                 | Soll      |
| LA-018 | Möglichkeit zur Zahlung per Rechnung                     | Kann      |

### 3.2 Nicht-funktionale Anforderungen

#### 3.2.1 Benutzerfreundlichkeit
| ID | Anforderung | Priorität |
|----|-------------|-----------|
| LA-019 | Intuitive, benutzerfreundliche Oberfläche | Muss |
| LA-020 | Responsive Design für mobile Endgeräte | Soll |
| LA-021 | Barrierefreiheit nach WCAG 2.1 Level AA | Kann |
| LA-022 | Deutsche Sprachunterstützung | Muss |
| LA-023 | Mehrsprachigkeit (Englisch) | Kann |

#### 3.2.2 Performance
| ID | Anforderung | Priorität |
|----|-------------|-----------|
| LA-024 | Ladezeit der Hauptseite unter 3 Sekunden | Muss |
| LA-025 | System muss mindestens 100 gleichzeitige Benutzer unterstützen | Muss |
| LA-026 | Verfügbarkeit von mindestens 99% | Soll |

#### 3.2.3 Sicherheit
| ID | Anforderung | Priorität |
|----|-------------|-----------|
| LA-027 | Verschlüsselte Datenübertragung (HTTPS/TLS) | Muss |
| LA-028 | Sichere Speicherung von Passwörtern (Hashing) | Muss |
| LA-029 | DSGVO-konforme Datenverarbeitung | Muss |
| LA-030 | Regelmäßige Sicherheits-Backups | Muss |
| LA-031 | Schutz vor SQL-Injection und XSS | Muss |

---

## 4. Technische Rahmenbedingungen

### 4.1 Technologievorgaben
- **Framework:** Tauri (Rust-basiert)
- **Frontend:** Vue.js
- **Datenbank:** SQLite oder Mysql
- **Deployment:** Cross-Platform Desktop-Anwendung

### 4.2 Schnittstellen
- E-Mail-Versand (SMTP)
- Zahlungsanbieter-API (z.B. Stripe, PayPal)
- Optional: Kartendienst für Routenanzeige
- (Zahlungsanbieter nur mocked, keine reale anbindung)

### 4.3 Systemumgebung
- Betriebssysteme: Windows 10+, macOS 10.15+, Linux
- Bildschirmauflösung: mindestens 1280x720 Pixel

---

## 5. Lieferumfang

**Hinweis:** Der Lieferumfang ist auf ein MVP (Minimum Viable Product) für max. 100 Arbeitsstunden eines Junior-Entwicklers ausgelegt.

### 5.1 Software
- Installierbare Desktop-Anwendung (primär Windows)
- Integrierte Administrationsoberfläche
- SQLite-Datenbank mit Beispieldaten

### 5.2 Dokumentation
- Kompaktes Benutzerhandbuch
- README mit technischer Übersicht
- Installationsanleitung

### 5.3 Einweisung
- Kurze Remote-Einweisung (ca. 1 Stunde, inklusive)

---

## 6. Projektrahmen

### 6.1 Zeitplan
| Phase | Dauer | Meilenstein |
|-------|-------|-------------|
| Analyse/Planung | 2-3 Tage | Konzept, DB-Design |
| Entwicklung | ca. 3 Wochen | Funktionsfähiges MVP |
| Test & Abnahme | 2-3 Tage | Abnahme durch Auftraggeber |

**Gesamtlaufzeit:** 1 Monat  
**Maximaler Aufwand:** 100 Arbeitsstunden (1 Junior-Entwickler)

### 6.2 Budget
- Budgetrahmen: ca. 4.500 € - 5.500 € (netto)
- Detaillierte Kostenaufstellung im kaufmännischen Angebot

---

## 7. Abnahmekriterien

### 7.1 Funktionale Abnahme
- Alle "Muss"-Anforderungen sind vollständig umgesetzt
- Erfolgreicher Durchlauf der Basis-Testfälle

### 7.2 Qualitätskriterien
- Keine kritischen Fehler (Severity 1)
- System ist benutzbar und stabil
- Basis-Dokumentation vollständig

### 7.3 MVP-Einschränkungen
Folgende Features sind im MVP **nicht** enthalten und können in Phase 2 ergänzt werden:
- Echte Zahlungsintegration (nur Mock/Simulation)
- Stornierungsfunktion
- Passwort-Zurücksetzen per E-Mail
- Erweiterte Statistiken und Berichte
- Fahrzeugverwaltung
- Multi-Plattform-Builds (macOS, Linux)

---

## 8. Mitwirkungspflichten des Auftraggebers

- Bereitstellung erforderlicher Informationen und Daten
- Benennung eines Ansprechpartners
- Zeitnahe Rückmeldungen und Freigaben
- Bereitstellung von Testdaten
- Teilnahme an Abstimmungsterminen
- Bereitstellung von Testumgebungen

---

## 9. Glossar

| Begriff | Definition |
|---------|------------|
| Tauri | Framework zur Erstellung von Desktop-Anwendungen mit Web-Technologien |
| Vue.js | JavaScript-Framework für Benutzeroberflächen |
| DSGVO | Datenschutz-Grundverordnung |
| API | Application Programming Interface (Programmierschnittstelle) |
| MVP | Minimum Viable Product (minimal funktionsfähiges Produkt) |

---

## 10. Anhang

### 10.1 Dokumentenhistorie
| Version | Datum | Autor | Änderungen |
|---------|-------|-------|------------|
| 1.0 | 05.02.2026 | Projektteam | Erstversion |

### 10.2 Unterschriften

| Rolle | Name | Datum | Unterschrift |
|-------|------|-------|--------------|
| Auftraggeber | | | |
| Auftragnehmer | | | |

---

*Dieses Lastenheft dient als Grundlage für die weitere Projektplanung und das Pflichtenheft.*
