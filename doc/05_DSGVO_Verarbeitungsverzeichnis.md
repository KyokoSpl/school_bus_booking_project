# Verzeichnis von Verarbeitungstätigkeiten
## (Datenspeicherungsplan nach Art. 30 DSGVO)

**Verantwortlicher:**  
Sonnenschein Reisen GmbH  
Sonnenallee 42  
54321 Reisestadt  
Deutschland

**Vertreter des Verantwortlichen:**  
Geschäftsführer/in [Name]

**Datenschutzbeauftragter:**  
[Name]  
datenschutz@sonnenschein-reisen.de

**Datum der Erstellung:** 05.02.2026  
**Letzte Aktualisierung:** 05.02.2026  
**Version:** 1.0

---

## 1. Übersicht der Verarbeitungstätigkeiten

| Nr. | Bezeichnung der Verarbeitungstätigkeit | Risikostufe |
|-----|---------------------------------------|-------------|
| VT-001 | Kundenkontenverwaltung | Mittel |
| VT-002 | Buchungsabwicklung | Mittel |
| VT-003 | Zahlungsabwicklung | Hoch |
| VT-004 | E-Mail-Kommunikation | Niedrig |
| VT-005 | Kundensupport | Niedrig |
| VT-006 | Statistische Auswertungen | Niedrig |
| VT-007 | Sicherheitsprotokollierung | Niedrig |
| VT-008 | Newsletter/Marketing | Mittel |

---

## 2. Detaillierte Verarbeitungstätigkeiten

### VT-001: Kundenkontenverwaltung

| Attribut | Beschreibung |
|----------|--------------|
| **Bezeichnung** | Kundenkontenverwaltung |
| **Verantwortliche Stelle** | Sonnenschein Reisen GmbH |
| **Zuständige Abteilung** | Kundenservice, IT |
| **Zweck der Verarbeitung** | Verwaltung von Benutzerkonten für das Online-Buchungssystem, Ermöglichung von Buchungen und Zugriff auf Buchungshistorie |
| **Rechtsgrundlage** | Art. 6 Abs. 1 lit. b DSGVO (Vertragserfüllung) |
| **Kategorien betroffener Personen** | Kunden (natürliche Personen) |
| **Kategorien personenbezogener Daten** | - Identifikationsdaten (Vorname, Nachname)<br>- Kontaktdaten (E-Mail, Telefon)<br>- Authentifizierungsdaten (Passwort-Hash)<br>- Adressdaten (Straße, PLZ, Ort, Land)<br>- Demografische Daten (Geburtsdatum) |
| **Empfänger/Empfängerkategorien** | - Interne Mitarbeiter (Kundenservice, IT)<br>- Hosting-Anbieter (Auftragsverarbeiter) |
| **Drittlandübermittlung** | Nein (Daten verbleiben in EU/EWR) |
| **Löschfristen** | - Aktive Konten: Unbefristet bis zur Löschung durch Kunden<br>- Inaktive Konten: Löschung nach 3 Jahren Inaktivität<br>- Nach Kündigung: Sofortige Sperrung, Löschung nach Ablauf gesetzlicher Aufbewahrungsfristen |
| **Technische Maßnahmen** | - Passwort-Hashing (Argon2)<br>- Verschlüsselte Speicherung<br>- Zugriffskontrolle<br>- TLS-Verschlüsselung |
| **Organisatorische Maßnahmen** | - Need-to-Know-Prinzip<br>- Schulung der Mitarbeiter<br>- Dokumentierte Prozesse |

---

### VT-002: Buchungsabwicklung

| Attribut | Beschreibung |
|----------|--------------|
| **Bezeichnung** | Buchungsabwicklung |
| **Verantwortliche Stelle** | Sonnenschein Reisen GmbH |
| **Zuständige Abteilung** | Kundenservice, Buchhaltung |
| **Zweck der Verarbeitung** | Durchführung und Verwaltung von Reisebuchungen, Erstellung von Buchungsbestätigungen und Rechnungen |
| **Rechtsgrundlage** | Art. 6 Abs. 1 lit. b DSGVO (Vertragserfüllung)<br>Art. 6 Abs. 1 lit. c DSGVO (rechtliche Verpflichtung - Rechnungsaufbewahrung) |
| **Kategorien betroffener Personen** | - Buchende Kunden<br>- Mitreisende Personen |
| **Kategorien personenbezogener Daten** | - Buchungsdaten (Reise, Datum, Personenzahl)<br>- Mitreisende (Name, Geburtsdatum)<br>- Rechnungsdaten (Adresse, Betrag)<br>- Sonderwünsche/Bemerkungen |
| **Empfänger/Empfängerkategorien** | - Interne Mitarbeiter<br>- Busfahrer (nur Name der Mitreisenden)<br>- Steuerberater (Rechnungsdaten) |
| **Drittlandübermittlung** | Nein |
| **Löschfristen** | - Buchungsdaten: 10 Jahre (steuerliche Aufbewahrungspflicht)<br>- Nach Ablauf: Anonymisierung oder Löschung |
| **Technische Maßnahmen** | - Datenbankverschlüsselung<br>- Backup-Systeme<br>- Audit-Logging |
| **Organisatorische Maßnahmen** | - Vier-Augen-Prinzip bei Stornierungen<br>- Dokumentierte Buchungsprozesse |

---

### VT-003: Zahlungsabwicklung

| Attribut | Beschreibung |
|----------|--------------|
| **Bezeichnung** | Zahlungsabwicklung |
| **Verantwortliche Stelle** | Sonnenschein Reisen GmbH |
| **Zuständige Abteilung** | Buchhaltung |
| **Zweck der Verarbeitung** | Abwicklung von Online-Zahlungen für gebuchte Reisen |
| **Rechtsgrundlage** | Art. 6 Abs. 1 lit. b DSGVO (Vertragserfüllung) |
| **Kategorien betroffener Personen** | Zahlende Kunden |
| **Kategorien personenbezogener Daten** | - Zahlungsmethode<br>- Transaktions-ID<br>- Zahlungsstatus<br>- Betrag<br>**WICHTIG:** Keine vollständigen Kreditkartendaten werden bei uns gespeichert |
| **Empfänger/Empfängerkategorien** | - Zahlungsdienstleister (Stripe, PayPal) - **Auftragsverarbeiter**<br>- Buchhaltung (intern) |
| **Drittlandübermittlung** | Möglich (USA) - Stripe/PayPal<br>Garantien: EU-Commission adequacy decision, Standardvertragsklauseln |
| **Löschfristen** | - Transaktionsdaten: 10 Jahre (steuerliche Pflicht)<br>- Zahlungstoken: Bis zum Widerruf durch Kunden |
| **Technische Maßnahmen** | - PCI DSS-konforme Verarbeitung durch Zahlungsdienstleister<br>- Tokenisierung<br>- Keine Speicherung sensibler Kartendaten |
| **Organisatorische Maßnahmen** | - Auftragsverarbeitungsverträge mit Zahlungsdienstleistern<br>- Regelmäßige Überprüfung der Compliance |

---

### VT-004: E-Mail-Kommunikation

| Attribut | Beschreibung |
|----------|--------------|
| **Bezeichnung** | E-Mail-Kommunikation |
| **Verantwortliche Stelle** | Sonnenschein Reisen GmbH |
| **Zuständige Abteilung** | Kundenservice, IT |
| **Zweck der Verarbeitung** | Versand von Buchungsbestätigungen, Reiseinformationen und systemrelevanten Benachrichtigungen |
| **Rechtsgrundlage** | Art. 6 Abs. 1 lit. b DSGVO (Vertragserfüllung) |
| **Kategorien betroffener Personen** | Kunden mit E-Mail-Adresse |
| **Kategorien personenbezogener Daten** | - E-Mail-Adresse<br>- Name<br>- Buchungsinformationen |
| **Empfänger/Empfängerkategorien** | - E-Mail-Dienstleister (Auftragsverarbeiter) |
| **Drittlandübermittlung** | [Abhängig vom Anbieter - zu prüfen] |
| **Löschfristen** | - Versendete E-Mails: 6 Jahre (Korrespondenz)<br>- E-Mail-Logs: 90 Tage |
| **Technische Maßnahmen** | - TLS-Verschlüsselung<br>- SPF/DKIM/DMARC-Konfiguration |
| **Organisatorische Maßnahmen** | - Auftragsverarbeitungsvertrag mit E-Mail-Dienstleister |

---

### VT-005: Kundensupport

| Attribut | Beschreibung |
|----------|--------------|
| **Bezeichnung** | Kundensupport |
| **Verantwortliche Stelle** | Sonnenschein Reisen GmbH |
| **Zuständige Abteilung** | Kundenservice |
| **Zweck der Verarbeitung** | Bearbeitung von Kundenanfragen, Beschwerden und Supportfällen |
| **Rechtsgrundlage** | Art. 6 Abs. 1 lit. b DSGVO (Vertragserfüllung)<br>Art. 6 Abs. 1 lit. f DSGVO (berechtigtes Interesse) |
| **Kategorien betroffener Personen** | Kunden und Interessenten |
| **Kategorien personenbezogener Daten** | - Kontaktdaten<br>- Anfrage-/Beschwerdeinhalt<br>- Buchungsbezogene Daten<br>- Kommunikationsverlauf |
| **Empfänger/Empfängerkategorien** | - Kundenservice-Mitarbeiter |
| **Drittlandübermittlung** | Nein |
| **Löschfristen** | - Support-Tickets: 3 Jahre nach Abschluss<br>- Bei laufenden Rechtsstreitigkeiten: Bis zum Abschluss + 3 Jahre |
| **Technische Maßnahmen** | - Zugriffsprotokollierung<br>- Verschlüsselung |
| **Organisatorische Maßnahmen** | - Schulung in Datenschutz<br>- Eskalationsprozesse |

---

### VT-006: Statistische Auswertungen

| Attribut | Beschreibung |
|----------|--------------|
| **Bezeichnung** | Statistische Auswertungen |
| **Verantwortliche Stelle** | Sonnenschein Reisen GmbH |
| **Zuständige Abteilung** | Geschäftsführung, Marketing |
| **Zweck der Verarbeitung** | Erstellung von Geschäftsberichten, Analyse des Buchungsverhaltens zur Optimierung des Angebots |
| **Rechtsgrundlage** | Art. 6 Abs. 1 lit. f DSGVO (berechtigtes Interesse) |
| **Kategorien betroffener Personen** | Kunden (anonymisiert/aggregiert) |
| **Kategorien personenbezogener Daten** | Nur aggregierte/anonymisierte Daten:<br>- Buchungsstatistiken<br>- Umsatzzahlen<br>- Beliebte Reiseziele |
| **Empfänger/Empfängerkategorien** | - Geschäftsführung (intern) |
| **Drittlandübermittlung** | Nein |
| **Löschfristen** | Anonymisierte Statistiken: Unbefristet |
| **Technische Maßnahmen** | - Pseudonymisierung<br>- Aggregation<br>- Zugriffsbeschränkung |
| **Organisatorische Maßnahmen** | - Dokumentation der Anonymisierungsmethoden |

---

### VT-007: Sicherheitsprotokollierung

| Attribut | Beschreibung |
|----------|--------------|
| **Bezeichnung** | Sicherheitsprotokollierung |
| **Verantwortliche Stelle** | Sonnenschein Reisen GmbH |
| **Zuständige Abteilung** | IT |
| **Zweck der Verarbeitung** | Gewährleistung der IT-Sicherheit, Erkennung von Angriffen, Fehlerbehebung |
| **Rechtsgrundlage** | Art. 6 Abs. 1 lit. f DSGVO (berechtigtes Interesse - IT-Sicherheit) |
| **Kategorien betroffener Personen** | Alle Nutzer des Systems |
| **Kategorien personenbezogener Daten** | - IP-Adresse<br>- Zugriffszeitpunkt<br>- Angefragte Ressource<br>- Fehlermeldungen (ohne personenbezogene Inhalte) |
| **Empfänger/Empfängerkategorien** | - IT-Abteilung<br>- Ggf. Strafverfolgungsbehörden (bei Cyberangriffen) |
| **Drittlandübermittlung** | Nein |
| **Löschfristen** | - Sicherheitslogs: 90 Tage<br>- Bei Sicherheitsvorfällen: Aufbewahrung bis Klärung |
| **Technische Maßnahmen** | - Zentrale Protokollierung<br>- Zugriffsbeschränkung auf Logs<br>- Integritätsschutz |
| **Organisatorische Maßnahmen** | - Definierte Zuständigkeiten<br>- Incident-Response-Plan |

---

### VT-008: Newsletter/Marketing

| Attribut | Beschreibung |
|----------|--------------|
| **Bezeichnung** | Newsletter und Marketing |
| **Verantwortliche Stelle** | Sonnenschein Reisen GmbH |
| **Zuständige Abteilung** | Marketing |
| **Zweck der Verarbeitung** | Versand von Werbung, Angeboten und Newsletter an interessierte Kunden |
| **Rechtsgrundlage** | Art. 6 Abs. 1 lit. a DSGVO (Einwilligung) |
| **Kategorien betroffener Personen** | Kunden und Interessenten mit Newsletter-Anmeldung |
| **Kategorien personenbezogener Daten** | - E-Mail-Adresse<br>- Name (optional)<br>- Anmeldedatum<br>- Einwilligungsnachweis |
| **Empfänger/Empfängerkategorien** | - E-Mail-Marketing-Dienstleister (Auftragsverarbeiter) |
| **Drittlandübermittlung** | [Abhängig vom Anbieter] |
| **Löschfristen** | - Nach Abmeldung: Sofortige Löschung für Marketingzwecke<br>- Einwilligungsnachweis: 3 Jahre |
| **Technische Maßnahmen** | - Double-Opt-In-Verfahren<br>- Abmeldelink in jeder E-Mail |
| **Organisatorische Maßnahmen** | - Dokumentation der Einwilligungen<br>- Auftragsverarbeitungsvertrag |

---

## 3. Auftragsverarbeiter

### 3.1 Übersicht der Auftragsverarbeiter

| Auftragsverarbeiter | Leistung | Standort | AVV vorhanden |
|---------------------|----------|----------|---------------|
| [Hosting-Anbieter] | Server-Hosting, Backup | Deutschland/EU | Ja |
| Stripe Inc. | Zahlungsabwicklung | USA (EU-Niederlassung) | Ja |
| PayPal (Europe) S.à r.l. | Zahlungsabwicklung | Luxemburg | Ja |
| [E-Mail-Dienstleister] | E-Mail-Versand | Deutschland/EU | Ja |

### 3.2 Inhalte der Auftragsverarbeitungsverträge

Alle AVVs enthalten mindestens:
- [x] Art und Zweck der Verarbeitung
- [x] Art der personenbezogenen Daten
- [x] Kategorien betroffener Personen
- [x] Pflichten und Rechte des Verantwortlichen
- [x] Weisungsgebundenheit
- [x] Vertraulichkeitsverpflichtung
- [x] Technische und organisatorische Maßnahmen
- [x] Unterauftragsverarbeiter-Regelung
- [x] Unterstützungspflichten bei Betroffenenrechten
- [x] Löschung/Rückgabe nach Vertragsende
- [x] Kontrollrechte

---

## 4. Löschkonzept

### 4.1 Übersicht Löschfristen

| Datenkategorie | Regellöschfrist | Rechtsgrundlage |
|----------------|-----------------|-----------------|
| Kundenkonto | 3 Jahre nach letzter Aktivität | Vertragserfüllung |
| Buchungsdaten | 10 Jahre | § 147 AO, § 257 HGB |
| Rechnungen | 10 Jahre | § 147 AO |
| Zahlungsdaten | 10 Jahre | § 147 AO |
| Korrespondenz | 6 Jahre | § 257 HGB |
| Support-Tickets | 3 Jahre | Berechtigtes Interesse |
| Sicherheitslogs | 90 Tage | Berechtigtes Interesse |
| Marketing-Einwilligungen | 3 Jahre nach Widerruf | Nachweispflicht |

### 4.2 Löschprozess

1. **Automatische Löschung:** System prüft regelmäßig (wöchentlich) auf abgelaufene Löschfristen
2. **Manuelle Löschung:** Bei Löschantrag durch Betroffenen
3. **Dokumentation:** Jede Löschung wird protokolliert
4. **Backup-Berücksichtigung:** Auch Daten in Backups werden nach Ablauf gelöscht

---

## 5. Technische und Organisatorische Maßnahmen (TOMs)

### 5.1 Zusammenfassung

| Schutzziel | Maßnahmen |
|------------|-----------|
| **Vertraulichkeit** | - Zugriffskontrolle (RBAC)<br>- Verschlüsselung (AES-256, TLS 1.3)<br>- Pseudonymisierung<br>- VPN für Admin-Zugriff |
| **Integrität** | - Eingabevalidierung<br>- Prüfsummen<br>- Audit-Logging<br>- Code-Reviews |
| **Verfügbarkeit** | - Redundante Systeme<br>- Automatische Backups<br>- Monitoring<br>- Disaster-Recovery-Plan |
| **Belastbarkeit** | - Load-Balancing<br>- DDoS-Schutz<br>- Rate-Limiting |

### 5.2 Detaillierte TOMs

Siehe separates Dokument: **06_Datensicherheitskonzept.md**

---

## 6. Datenschutz-Folgenabschätzung

### 6.1 Prüfung der Notwendigkeit

| Verarbeitungstätigkeit | DSFA erforderlich? | Begründung |
|------------------------|-------------------|------------|
| VT-001 Kundenkontenverwaltung | Nein | Standardverarbeitung |
| VT-002 Buchungsabwicklung | Nein | Standardverarbeitung |
| VT-003 Zahlungsabwicklung | Prüfung | Sensible Finanzdaten, aber Auslagerung an PCI-DSS-konforme Dienstleister |
| VT-004 E-Mail-Kommunikation | Nein | Standardverarbeitung |
| VT-005 Kundensupport | Nein | Standardverarbeitung |
| VT-006 Statistische Auswertungen | Nein | Anonymisierte Daten |
| VT-007 Sicherheitsprotokollierung | Nein | Berechtigtes Interesse, kurze Speicherung |
| VT-008 Newsletter/Marketing | Nein | Einwilligungsbasiert |

### 6.2 Ergebnis
Eine Datenschutz-Folgenabschätzung ist für die aktuellen Verarbeitungstätigkeiten **nicht erforderlich**, da:
- Keine umfangreiche Verarbeitung besonderer Datenkategorien (Art. 9 DSGVO)
- Keine systematische Überwachung öffentlicher Bereiche
- Keine automatisierte Entscheidungsfindung mit Rechtswirkung

Bei Einführung neuer Verarbeitungen ist die Notwendigkeit einer DSFA erneut zu prüfen.

---

## 7. Dokumentenhistorie

| Version | Datum | Bearbeiter | Änderungen |
|---------|-------|------------|------------|
| 1.0 | 05.02.2026 | Projektteam | Ersterstellung |

---

## 8. Nächste Überprüfung

**Geplantes Prüfdatum:** 05.02.2027

**Prüfintervall:** Jährlich oder bei wesentlichen Änderungen der Verarbeitungstätigkeiten

---

*Dieses Verzeichnis wird gemäß Art. 30 DSGVO geführt und der Aufsichtsbehörde auf Anfrage zur Verfügung gestellt.*
