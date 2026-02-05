# Kriterienkatalog
## Ergebnisse der Analysephase

**Projekt:** Online-Buchungssystem "BusBooker Pro"  
**Auftraggeber:** Sonnenschein Reisen GmbH  
**Version:** 1.0  
**Datum:** 05.02.2026

---

## 1. Einleitung

### 1.1 Zweck des Dokuments
Dieser Kriterienkatalog dokumentiert die Ergebnisse der Analysephase und dient als Bewertungsgrundlage für:
- Technologie- und Architekturentscheidungen
- Auswahl von Drittanbieter-Komponenten
- Bewertung von Lösungsalternativen
- Abnahmeprüfung des fertigen Systems

### 1.2 Bewertungssystem
Die Kriterien werden nach folgendem Schema bewertet:

| Gewichtung | Bedeutung |
|------------|-----------|
| ★★★★★ | Kritisch - Must-Have |
| ★★★★☆ | Sehr wichtig |
| ★★★☆☆ | Wichtig |
| ★★☆☆☆ | Wünschenswert |
| ★☆☆☆☆ | Nice-to-Have |

---

## 2. Funktionale Kriterien

### 2.1 Kundenbereich

| ID | Kriterium | Gewichtung | Erfüllt | Bemerkung |
|----|-----------|------------|---------|-----------|
| FK-001 | Benutzerregistrierung mit E-Mail-Validierung | ★★★★★ | ☐ | Pflichtfeld-Validierung erforderlich |
| FK-002 | Sicherer Login mit Passwort-Hashing | ★★★★★ | ☐ | Argon2 oder bcrypt |
| FK-003 | Passwort-Zurücksetzen per E-Mail | ★★★★☆ | ☐ | Token-basiert, max. 24h gültig |
| FK-004 | Profilverwaltung (persönliche Daten) | ★★★★☆ | ☐ | Inklusive Adressverwaltung |
| FK-005 | Reisesuche mit Volltextsuche | ★★★★★ | ☐ | Suche nach Ziel, Beschreibung |
| FK-006 | Filteroptionen (Datum, Preis, Dauer) | ★★★★☆ | ☐ | Kombinierbare Filter |
| FK-007 | Sortieroptionen (Preis, Datum, Beliebtheit) | ★★★☆☆ | ☐ | Auf-/absteigend |
| FK-008 | Detailansicht mit Reisebeschreibung | ★★★★★ | ☐ | Bilder, Leistungen, Preise |
| FK-009 | Buchungsprozess mit Platzwahl | ★★★★★ | ☐ | Maximal 5 Schritte |
| FK-010 | Eingabe Mitreisende-Daten | ★★★★☆ | ☐ | Name, Geburtsdatum |
| FK-011 | Buchungsübersicht/-bestätigung | ★★★★★ | ☐ | Zusammenfassung vor Abschluss |
| FK-012 | Buchungsverlauf einsehen | ★★★★☆ | ☐ | Vergangene und aktuelle Buchungen |
| FK-013 | Buchung stornieren | ★★★☆☆ | ☐ | Mit Stornobedingungen |
| FK-014 | E-Mail-Bestätigung automatisch | ★★★★★ | ☐ | Bei Buchung/Stornierung |
| FK-015 | Reisebewertungen abgeben | ★★☆☆☆ | ☐ | Nach Reisedurchführung |

### 2.2 Administrationsbereich

| ID | Kriterium | Gewichtung | Erfüllt | Bemerkung |
|----|-----------|------------|---------|-----------|
| FA-001 | Dashboard mit Übersicht | ★★★★☆ | ☐ | KPIs, aktuelle Buchungen |
| FA-002 | Reisen anlegen (CRUD) | ★★★★★ | ☐ | Alle Reisedaten |
| FA-003 | Reisen mit Bildern versehen | ★★★☆☆ | ☐ | Mehrere Bilder pro Reise |
| FA-004 | Reisen aktivieren/deaktivieren | ★★★★☆ | ☐ | Ohne Löschung |
| FA-005 | Fahrzeugverwaltung | ★★★★★ | ☐ | Busse, Kapazitäten |
| FA-006 | Bus einer Reise zuweisen | ★★★★★ | ☐ | Mit Platzberechnung |
| FA-007 | Buchungsübersicht | ★★★★★ | ☐ | Filterbar nach Status |
| FA-008 | Buchung manuell bearbeiten | ★★★★☆ | ☐ | Status ändern, Daten korrigieren |
| FA-009 | Buchung manuell anlegen | ★★★☆☆ | ☐ | Für Telefonbuchungen |
| FA-010 | Kundenverwaltung | ★★★★☆ | ☐ | Übersicht, Suche, Details |
| FA-011 | Kundenaccount deaktivieren | ★★★☆☆ | ☐ | DSGVO-konform |
| FA-012 | Preiskonfiguration | ★★★☆☆ | ☐ | Grundpreise, Aufschläge |
| FA-013 | Rabatte und Aktionen | ★★☆☆☆ | ☐ | Prozentual oder fix |
| FA-014 | Berichte und Statistiken | ★★★☆☆ | ☐ | Buchungen, Umsatz |
| FA-015 | Datenexport (CSV/PDF) | ★★☆☆☆ | ☐ | Für Buchhaltung |

### 2.3 Zahlungssystem

| ID | Kriterium | Gewichtung | Erfüllt | Bemerkung |
|----|-----------|------------|---------|-----------|
| FZ-001 | Mindestens ein Online-Zahlungsanbieter | ★★★★★ | ☐ | Stripe oder PayPal |
| FZ-002 | Sichere Zahlungsabwicklung (PCI DSS) | ★★★★★ | ☐ | Externe Abwicklung |
| FZ-003 | Zahlungsstatus nachverfolgen | ★★★★☆ | ☐ | Ausstehend, bezahlt, erstattet |
| FZ-004 | Automatische Rechnungserstellung | ★★★☆☆ | ☐ | PDF-Rechnung per E-Mail |
| FZ-005 | Mehrere Zahlungsmethoden | ★★★☆☆ | ☐ | Kreditkarte, PayPal |
| FZ-006 | Teilzahlung/Anzahlung | ★☆☆☆☆ | ☐ | Optional |
| FZ-007 | Erstattungen abwickeln | ★★★☆☆ | ☐ | Bei Stornierung |

---

## 3. Nicht-funktionale Kriterien

### 3.1 Usability (Benutzerfreundlichkeit)

| ID | Kriterium | Gewichtung | Erfüllt | Messbarkeit/Ziel |
|----|-----------|------------|---------|------------------|
| NU-001 | Intuitive Navigation | ★★★★★ | ☐ | Max. 3 Klicks zum Ziel |
| NU-002 | Konsistentes Design | ★★★★☆ | ☐ | Einheitliche UI-Elemente |
| NU-003 | Fehlertoleranz | ★★★★☆ | ☐ | Klare Fehlermeldungen |
| NU-004 | Responsive Design | ★★★☆☆ | ☐ | Ab 1024px Breite optimiert |
| NU-005 | Barrierefreiheit (WCAG 2.1 AA) | ★★☆☆☆ | ☐ | Optional |
| NU-006 | Deutsche Sprache | ★★★★★ | ☐ | Vollständig lokalisiert |
| NU-007 | Mehrsprachigkeit | ★☆☆☆☆ | ☐ | Englisch als Option |
| NU-008 | Hilfetexte und Tooltips | ★★★☆☆ | ☐ | Bei komplexen Funktionen |
| NU-009 | Tastaturnavigation | ★★☆☆☆ | ☐ | Ohne Maus bedienbar |
| NU-010 | Ladeanimationen | ★★★☆☆ | ☐ | Feedback bei Wartezeiten |

### 3.2 Performance

| ID | Kriterium | Gewichtung | Erfüllt | Messbarkeit/Ziel |
|----|-----------|------------|---------|------------------|
| NP-001 | Startzeit der Anwendung | ★★★★☆ | ☐ | < 5 Sekunden |
| NP-002 | Seitenaufbau | ★★★★★ | ☐ | < 3 Sekunden |
| NP-003 | Suchergebnisse | ★★★★☆ | ☐ | < 2 Sekunden |
| NP-004 | Datenbankabfragen | ★★★★☆ | ☐ | < 500ms |
| NP-005 | Gleichzeitige Benutzer | ★★★★☆ | ☐ | >= 100 |
| NP-006 | Speicherverbrauch | ★★★☆☆ | ☐ | < 500 MB RAM |
| NP-007 | CPU-Auslastung im Leerlauf | ★★★☆☆ | ☐ | < 5% |

### 3.3 Sicherheit

| ID | Kriterium | Gewichtung | Erfüllt | Messbarkeit/Ziel |
|----|-----------|------------|---------|------------------|
| NS-001 | HTTPS/TLS-Verschlüsselung | ★★★★★ | ☐ | TLS 1.2+ |
| NS-002 | Sichere Passwort-Speicherung | ★★★★★ | ☐ | Argon2/bcrypt |
| NS-003 | SQL-Injection-Schutz | ★★★★★ | ☐ | Parameterisierte Abfragen |
| NS-004 | XSS-Schutz | ★★★★★ | ☐ | Input-Sanitization |
| NS-005 | CSRF-Schutz | ★★★★★ | ☐ | Token-basiert |
| NS-006 | Session-Management | ★★★★★ | ☐ | JWT mit Timeout |
| NS-007 | Brute-Force-Schutz | ★★★★☆ | ☐ | Rate-Limiting |
| NS-008 | Zugriffsprotokollierung | ★★★★☆ | ☐ | Audit-Log |
| NS-009 | Datensicherung | ★★★★★ | ☐ | Tägliche Backups |
| NS-010 | DSGVO-Konformität | ★★★★★ | ☐ | Siehe separates Dokument |

### 3.4 Zuverlässigkeit

| ID | Kriterium | Gewichtung | Erfüllt | Messbarkeit/Ziel |
|----|-----------|------------|---------|------------------|
| NR-001 | Verfügbarkeit | ★★★★☆ | ☐ | >= 99% |
| NR-002 | Fehlertoleranz | ★★★★☆ | ☐ | Graceful Degradation |
| NR-003 | Datenkonsistenz | ★★★★★ | ☐ | Transaktionen |
| NR-004 | Recovery-Zeit | ★★★☆☆ | ☐ | < 4 Stunden |
| NR-005 | Automatische Backups | ★★★★★ | ☐ | Täglich, 30 Tage Aufbewahrung |

### 3.5 Wartbarkeit

| ID | Kriterium | Gewichtung | Erfüllt | Messbarkeit/Ziel |
|----|-----------|------------|---------|------------------|
| NW-001 | Modularer Aufbau | ★★★★☆ | ☐ | Getrennte Module |
| NW-002 | Code-Dokumentation | ★★★★☆ | ☐ | JSDoc/rustdoc |
| NW-003 | Test-Abdeckung | ★★★☆☆ | ☐ | >= 80% |
| NW-004 | Logging | ★★★★☆ | ☐ | Verschiedene Level |
| NW-005 | Konfigurierbarkeit | ★★★★☆ | ☐ | Externe Konfiguration |
| NW-006 | Update-Fähigkeit | ★★★☆☆ | ☐ | Auto-Update möglich |

### 3.6 Portabilität

| ID | Kriterium | Gewichtung | Erfüllt | Messbarkeit/Ziel |
|----|-----------|------------|---------|------------------|
| NX-001 | Windows-Unterstützung | ★★★★★ | ☐ | Windows 10+ |
| NX-002 | macOS-Unterstützung | ★★★☆☆ | ☐ | macOS 10.15+ |
| NX-003 | Linux-Unterstützung | ★★★☆☆ | ☐ | Ubuntu 20.04+ |
| NX-004 | Installationspakete | ★★★★☆ | ☐ | MSI, DMG, DEB/AppImage |
| NX-005 | Automatische Updates | ★★☆☆☆ | ☐ | Optional |

---

## 4. Technologiekriterien

### 4.1 Framework-Bewertung: Tauri

| Kriterium | Bewertung | Begründung |
|-----------|-----------|------------|
| Performance | ★★★★★ | Rust-Backend sehr performant |
| Bundle-Größe | ★★★★★ | Deutlich kleiner als Electron |
| Sicherheit | ★★★★★ | Rust Memory Safety |
| Lernkurve | ★★★☆☆ | Rust-Kenntnisse erforderlich |
| Ökosystem | ★★★☆☆ | Wachsend, aber kleiner als Electron |
| Dokumentation | ★★★★☆ | Gute offizielle Docs |
| Community | ★★★☆☆ | Aktiv, aber kleiner |
| Plattform-Support | ★★★★★ | Windows, macOS, Linux |
| **Gesamt** | **★★★★☆** | **Gute Wahl für das Projekt** |

### 4.2 Frontend-Bewertung: Vue.js 3

| Kriterium | Bewertung | Begründung |
|-----------|-----------|------------|
| Performance | ★★★★★ | Virtual DOM, optimiert |
| Lernkurve | ★★★★☆ | Einsteigerfreundlich |
| Ökosystem | ★★★★★ | Umfangreich (Vuetify, PrimeVue) |
| Dokumentation | ★★★★★ | Exzellent |
| TypeScript-Support | ★★★★☆ | Gute Integration |
| Composition API | ★★★★★ | Moderne, flexible Architektur |
| **Gesamt** | **★★★★★** | **Hervorragende Wahl** |

### 4.3 Datenbank-Bewertung

| Kriterium | SQLite | PostgreSQL |
|-----------|--------|------------|
| Performance (lokal) | ★★★★★ | ★★★☆☆ |
| Skalierbarkeit | ★★☆☆☆ | ★★★★★ |
| Einrichtung | ★★★★★ | ★★★☆☆ |
| Features | ★★★☆☆ | ★★★★★ |
| Backup/Recovery | ★★★★☆ | ★★★★★ |
| Multi-User | ★★☆☆☆ | ★★★★★ |
| **Empfehlung** | Lokale Installation | Server-Betrieb |

---

## 5. Bewertung von Drittanbieter-Komponenten

### 5.1 Zahlungsanbieter

| Kriterium | Stripe | PayPal | Mollie |
|-----------|--------|--------|--------|
| Integration | ★★★★★ | ★★★★☆ | ★★★★☆ |
| Gebühren | ★★★☆☆ | ★★★☆☆ | ★★★★☆ |
| Sicherheit | ★★★★★ | ★★★★★ | ★★★★★ |
| Deutschland-Support | ★★★★★ | ★★★★★ | ★★★★★ |
| Dokumentation | ★★★★★ | ★★★★☆ | ★★★★☆ |
| Rust-SDK | ★★★★☆ | ★★☆☆☆ | ★★★☆☆ |
| **Empfehlung** | **Primär** | **Sekundär** | Alternative |

### 5.2 E-Mail-Service

| Kriterium | SMTP (eigen) | SendGrid | Mailjet |
|-----------|--------------|----------|---------|
| Kosten | ★★★★★ | ★★★☆☆ | ★★★☆☆ |
| Zuverlässigkeit | ★★★☆☆ | ★★★★★ | ★★★★★ |
| Einrichtung | ★★★☆☆ | ★★★★★ | ★★★★★ |
| Templates | ★★☆☆☆ | ★★★★★ | ★★★★☆ |
| **Empfehlung** | MVP/Test | **Produktion** | Alternative |

### 5.3 UI-Komponenten-Bibliotheken

| Kriterium | PrimeVue | Vuetify 3 | Naive UI |
|-----------|----------|-----------|----------|
| Komponentenumfang | ★★★★★ | ★★★★★ | ★★★★☆ |
| Design | ★★★★☆ | ★★★★★ | ★★★★☆ |
| Performance | ★★★★☆ | ★★★☆☆ | ★★★★★ |
| TypeScript | ★★★★★ | ★★★★☆ | ★★★★★ |
| Anpassbarkeit | ★★★★★ | ★★★★☆ | ★★★★☆ |
| **Empfehlung** | **Primär** | Alternative | Alternative |

---

## 6. Risikobewertung

### 6.1 Identifizierte Risiken

| ID | Risiko | Wahrscheinlichkeit | Auswirkung | Maßnahme |
|----|--------|-------------------|------------|----------|
| R-001 | Zeitplan-Überschreitung | Mittel | Hoch | Puffer einplanen, Priorisierung |
| R-002 | Technische Komplexität Rust | Mittel | Mittel | Erfahrene Entwickler |
| R-003 | Zahlungsintegration | Niedrig | Hoch | Frühe Integration, Testphase |
| R-004 | Datenmigrationsrisiken | Niedrig | Mittel | Backup vor Migration |
| R-005 | Performance-Probleme | Niedrig | Mittel | Frühes Performance-Testing |
| R-006 | Sicherheitslücken | Niedrig | Sehr hoch | Security-Review, Penetrationstests |
| R-007 | Änderungswünsche | Hoch | Mittel | Klares Scope-Management |

### 6.2 Risiko-Matrix

```
Auswirkung
    ▲
Sehr│         R-006
Hoch│
    │    R-001  R-003
Hoch│    
    │
Mit-│    R-002  R-004  R-007
tel │            R-005
    │
Nie-│
drig│
    └────────────────────────▶
        Niedrig  Mittel  Hoch
              Wahrscheinlichkeit
```

---

## 7. Lösungsalternativen-Bewertung

### 7.1 Architektur-Alternativen

| Variante | Beschreibung | Bewertung | Entscheidung |
|----------|--------------|-----------|--------------|
| A1: Tauri + Vue.js | Desktop-App mit Web-Frontend | ★★★★★ | **Gewählt** |
| A2: Electron + React | JavaScript-basierte Desktop-App | ★★★☆☆ | Verworfen (Ressourcenhungrig) |
| A3: Reine Web-App | Browser-basierte Lösung | ★★★★☆ | Alternative für Phase 2 |
| A4: Native Apps | Separate Entwicklung pro OS | ★★☆☆☆ | Zu aufwändig |

### 7.2 Begründung der Entscheidung for Tauri + Vue.js

**Vorteile:**
1. Performante Desktop-Anwendung mit kleiner Bundle-Größe
2. Sichere Rust-Backend-Logik
3. Modernes Vue.js-Frontend mit großem Ökosystem
4. Cross-Platform ohne Mehraufwand
5. Offline-Fähigkeit möglich

**Nachteile:**
1. Rust-Kenntnisse erforderlich
2. Kleineres Ökosystem als Electron
3. Relativ neues Framework

---

## 8. Abnahmekriterien

### 8.1 Muss-Kriterien für Abnahme

| ID | Kriterium | Akzeptanzkriterium |
|----|-----------|-------------------|
| AK-001 | Benutzerregistrierung funktioniert | Neuer Benutzer kann sich erfolgreich registrieren |
| AK-002 | Login/Logout funktioniert | Benutzer kann sich an- und abmelden |
| AK-003 | Reisesuche funktioniert | Reisen können nach Kriterien gefunden werden |
| AK-004 | Buchung durchführbar | Kompletter Buchungsprozess ohne Fehler |
| AK-005 | E-Mail-Versand funktioniert | Bestätigungsmail wird zugestellt |
| AK-006 | Admin kann Reisen verwalten | CRUD-Operationen funktionieren |
| AK-007 | Admin kann Buchungen einsehen | Buchungsübersicht korrekt |
| AK-008 | Daten werden sicher gespeichert | Passwörter gehasht, HTTPS aktiv |
| AK-009 | System startet ohne Fehler | Installation auf allen OS funktioniert |
| AK-010 | Performance-Anforderungen erfüllt | Seitenaufbau < 3 Sekunden |

### 8.2 Testfälle für Abnahme

| TF-ID | Beschreibung | Erwartetes Ergebnis |
|-------|--------------|---------------------|
| TF-001 | Neuen Benutzer registrieren | Account erstellt, Verifizierungsmail erhalten |
| TF-002 | Mit falschem Passwort einloggen | Fehlermeldung, kein Zugang |
| TF-003 | Reise nach "Paris" suchen | Paris-Reisen werden angezeigt |
| TF-004 | Reise für 2 Personen buchen | Buchung erfolgreich, E-Mail erhalten |
| TF-005 | Buchung stornieren | Stornierung bestätigt |
| TF-006 | Admin: Neue Reise anlegen | Reise erscheint in Suche |
| TF-007 | Admin: Reise bearbeiten | Änderungen gespeichert |
| TF-008 | SQL-Injection-Versuch | Angriff wird abgewehrt |
| TF-009 | 100 User gleichzeitig | System bleibt stabil |
| TF-010 | Installation auf Windows | App startet fehlerfrei |

---

## 9. Checkliste für Projektabschluss

### 9.1 Dokumentation
- [ ] Lastenheft finalisiert und genehmigt
- [ ] Pflichtenheft finalisiert und genehmigt
- [ ] Benutzerhandbuch erstellt
- [ ] Administrationshandbuch erstellt
- [ ] Technische Dokumentation vollständig
- [ ] DSGVO-Dokumentation vollständig

### 9.2 Software
- [ ] Alle Muss-Kriterien implementiert
- [ ] Alle Tests bestanden
- [ ] Installationspakete für alle Plattformen erstellt
- [ ] Quellcode übergeben
- [ ] Lizenzdokumentation vollständig

### 9.3 Übergabe
- [ ] Abnahmeprotokoll unterzeichnet
- [ ] Schulung durchgeführt
- [ ] Support-Kontakt kommuniziert
- [ ] Wartungsvertrag abgeschlossen (optional)

---

## 10. Anhang

### 10.1 Referenzen
- DIN 69901: Projektmanagement
- ISO 25010: Softwarequalität
- OWASP Top 10: Sicherheitsrichtlinien
- DSGVO: Datenschutzanforderungen

### 10.2 Dokumentenhistorie

| Version | Datum | Autor | Änderungen |
|---------|-------|-------|------------|
| 0.1 | 05.02.2026 | Projektteam | Ersterstellung |
| 1.0 | 05.02.2026 | Projektteam | Freigabe |

---

*Dieser Kriterienkatalog dient als Bewertungsgrundlage für alle Projektentscheidungen und die finale Abnahme.*
