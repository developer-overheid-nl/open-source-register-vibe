# Bijdragen aan Open Source Catalogus

Allereerst bedankt dat je wilt bijdragen aan dit project! Zonder jouw input
wordt dit nooit een beter open source project.

## Code of Conduct

Dit project hanteert een [Code of Conduct](CODE_OF_CONDUCT.md). Door bij te
dragen aan dit project ga je akkoord met de voorwaarden hiervan.

## Hoe kan je bijdragen?

Er zijn verschillende manieren om bij te dragen aan dit project:

### Meld een bug

Heb je een bug gevonden? Maak dan een [issue](https://github.com/developer-overheid-nl/open-source-register-vibe/issues/new) aan met:

- Een duidelijke en beschrijvende titel
- Stappen om de bug te reproduceren
- Verwacht gedrag vs. daadwerkelijk gedrag
- Screenshots of foutmeldingen (indien van toepassing)
- Je omgeving (OS, browserversie, etc.)

### Features voorstellen

Heb je een idee voor een nieuwe feature? Open een issue met:

- Een duidelijke beschrijving van de feature
- Waarom deze feature waardevol zou zijn
- Eventuele voorbeelden of mockups

### Documentatie verbeteren

Documentatie kan altijd beter! Pull requests voor verbeteringen aan de
documentatie zijn zeer welkom.

### Code bijdragen

Wil je code bijdragen? Volg dan onderstaand proces.

## Ontwikkelproces

1. Fork de repository en maak een branch aan vanaf `main`.
2. Lees [FEATURES.md](FEATURES.md) en [ARCHITECTURE.md](ARCHITECTURE.md) voor de
   featurespecificaties en de architectuurafspraken (Rust-backend met
   server-side rendering, progressive enhancement in vanilla JavaScript, één
   bestand per component, CSS-classes geprefixt op de componentnaam).
3. Controleer Rust-wijzigingen met `cargo fmt` en `cargo clippy`.
4. Bekijk wijzigingen lokaal door de backend te draaien
   (`cd server && cargo run`); de GitHub-data wordt bij het opstarten en
   daarna periodiek automatisch opgehaald.
5. Voeg een regel toe aan [CHANGELOG.md](CHANGELOG.md) onder `Unreleased`.
6. Open een pull request met een duidelijke beschrijving van de wijziging.

## Licentie

Door bij te dragen aan dit project ga je ermee akkoord dat je bijdragen worden
gelicenseerd onder de [EUPL-1.2](LICENSE.md) licentie. Dit betekent dat:

- Je het auteursrecht behoudt op je bijdragen
- Je bijdragen beschikbaar komen onder dezelfde open source licentie
- De code gebruikt mag worden door andere overheidsorganisaties

## Erkenning

Bijdragers worden vermeld in [CHANGELOG.md](CHANGELOG.md) bij elke release.

---

Bedankt voor je bijdrage!

_Dit project wordt onderhouden door developer.overheid.nl en de open source
community._
