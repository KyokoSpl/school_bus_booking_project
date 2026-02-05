-- Admin-Benutzer (Passwort: Admin123!)
-- Hash generiert mit Argon2
INSERT INTO users (id, email, password_hash, vorname, nachname, rolle, email_verifiziert, aktiv)
VALUES (
    'a0000000-0000-0000-0000-000000000001',
    'admin@sonnenschein-reisen.de',
    '$argon2id$v=19$m=19456,t=2,p=1$YWRtaW5zYWx0MTIzNDU$hVwK2l+3HqzU8qKkFGl8Wv8nh8h3Kx0vQ9uE2Y5nF7k',
    'System',
    'Administrator',
    'admin',
    TRUE,
    TRUE
);

-- Beispiel-Busse
INSERT INTO buses (id, kennzeichen, bezeichnung, sitzplaetze, ausstattung, baujahr, status) VALUES
('b0000000-0000-0000-0000-000000000001', 'SR-BUS-001', 'Mercedes Tourismo', 50, '["Klimaanlage", "WC", "WLAN", "Steckdosen", "Kühlschrank"]', 2022, 'verfuegbar'),
('b0000000-0000-0000-0000-000000000002', 'SR-BUS-002', 'Setra ComfortClass', 48, '["Klimaanlage", "WC", "WLAN", "Steckdosen", "TV"]', 2021, 'verfuegbar'),
('b0000000-0000-0000-0000-000000000003', 'SR-BUS-003', 'MAN Lions Coach', 52, '["Klimaanlage", "WC", "WLAN", "Steckdosen"]', 2020, 'verfuegbar');

-- Beispiel-Reisen
INSERT INTO trips (id, titel, beschreibung, ziel, abfahrtsort, abfahrt_datum, abfahrt_zeit, rueckkehr_datum, rueckkehr_zeit, preis_pro_person, bus_id, max_teilnehmer, aktuelle_buchungen, status, highlights, inkludiert, nicht_inkludiert) VALUES
(
    't0000000-0000-0000-0000-000000000001',
    'Paris - Stadt der Liebe',
    'Erleben Sie die romantische Hauptstadt Frankreichs! Besuchen Sie den Eiffelturm, den Louvre und genießen Sie französische Küche in malerischen Cafés. Diese 5-tägige Reise bietet Ihnen unvergessliche Eindrücke der Stadt der Lichter.',
    'Paris, Frankreich',
    'München Hauptbahnhof',
    '2026-04-15',
    '06:00:00',
    '2026-04-19',
    '18:00:00',
    299.00,
    'b0000000-0000-0000-0000-000000000001',
    50,
    0,
    'aktiv',
    '["Eiffelturm-Besuch", "Louvre-Führung", "Seine-Bootsfahrt", "Montmartre-Spaziergang"]',
    '["Busfahrt", "4 Übernachtungen", "Frühstück", "Reiseleitung", "Eintritt Louvre"]',
    '["Mittagessen", "Abendessen", "Optionale Ausflüge"]'
),
(
    't0000000-0000-0000-0000-000000000002',
    'Gardasee Rundfahrt',
    'Entdecken Sie Italiens größten See mit seinen malerischen Dörfern und mediterranem Flair. Von Riva del Garda bis Sirmione - eine Reise voller Genuss und Entspannung.',
    'Gardasee, Italien',
    'München Hauptbahnhof',
    '2026-05-01',
    '07:00:00',
    '2026-05-05',
    '20:00:00',
    249.00,
    'b0000000-0000-0000-0000-000000000002',
    48,
    0,
    'aktiv',
    '["Bootsfahrt auf dem Gardasee", "Besuch von Sirmione", "Weinverkostung", "Riva del Garda"]',
    '["Busfahrt", "4 Übernachtungen", "Halbpension", "Bootsfahrt"]',
    '["Eintritte", "Getränke"]'
),
(
    't0000000-0000-0000-0000-000000000003',
    'Wien - Kaiserliche Pracht',
    'Tauchen Sie ein in die Geschichte der Habsburger! Besuchen Sie Schloss Schönbrunn, die Wiener Staatsoper und genießen Sie Kaffeehauskultur und Sachertorte.',
    'Wien, Österreich',
    'München Hauptbahnhof',
    '2026-03-20',
    '08:00:00',
    '2026-03-23',
    '16:00:00',
    199.00,
    'b0000000-0000-0000-0000-000000000003',
    52,
    0,
    'aktiv',
    '["Schloss Schönbrunn", "Stephansdom", "Wiener Prater", "Kaffeehausbesuch"]',
    '["Busfahrt", "3 Übernachtungen", "Frühstück", "Stadtführung"]',
    '["Eintritte", "Mittagessen", "Abendessen"]'
),
(
    't0000000-0000-0000-0000-000000000004',
    'Amsterdam - Stadt der Grachten',
    'Erleben Sie die niederländische Hauptstadt mit ihren berühmten Grachten, dem Anne-Frank-Haus und dem Van Gogh Museum. Eine Reise für Kunst- und Kulturliebhaber.',
    'Amsterdam, Niederlande',
    'München Hauptbahnhof',
    '2026-06-10',
    '05:00:00',
    '2026-06-14',
    '22:00:00',
    279.00,
    'b0000000-0000-0000-0000-000000000001',
    50,
    0,
    'aktiv',
    '["Grachtenfahrt", "Van Gogh Museum", "Anne-Frank-Haus", "Käsemarkt Gouda"]',
    '["Busfahrt", "4 Übernachtungen", "Frühstück", "Grachtenfahrt"]',
    '["Museumseintritt", "Mittagessen", "Abendessen"]'
),
(
    't0000000-0000-0000-0000-000000000005',
    'Prag - Die Goldene Stadt',
    'Entdecken Sie die tschechische Hauptstadt mit ihrer beeindruckenden Architektur, der Karlsbrücke und der Prager Burg. Geschichte und Moderne vereint.',
    'Prag, Tschechien',
    'München Hauptbahnhof',
    '2026-04-25',
    '07:00:00',
    '2026-04-28',
    '19:00:00',
    169.00,
    'b0000000-0000-0000-0000-000000000002',
    48,
    0,
    'aktiv',
    '["Prager Burg", "Karlsbrücke", "Altstädter Ring", "Moldau-Bootsfahrt"]',
    '["Busfahrt", "3 Übernachtungen", "Frühstück", "Stadtführung"]',
    '["Eintritte", "Bootsfahrt", "Mahlzeiten"]'
);

-- Beispiel-Kunde (Passwort: Kunde123!)
INSERT INTO users (id, email, password_hash, vorname, nachname, telefon, strasse, plz, ort, rolle, email_verifiziert, aktiv)
VALUES (
    'u0000000-0000-0000-0000-000000000001',
    'max.mustermann@example.de',
    '$argon2id$v=19$m=19456,t=2,p=1$a3VuZGVzYWx0MTIzNDU$dGVzdGhhc2hrdW5kZTEyMzQ1Njc4OTAxMjM0NTY3ODkw',
    'Max',
    'Mustermann',
    '+49 170 1234567',
    'Musterstraße 1',
    '80331',
    'München',
    'kunde',
    TRUE,
    TRUE
);
