# Changelog

Alle belangrijke wijzigingen aan dit project worden gedocumenteerd in dit
bestand.

Het formaat is gebaseerd op
[Keep a Changelog](https://keepachangelog.com/nl/1.0.0/), en dit project volgt
[Semantic Versioning](https://semver.org/lang/nl/).

## [Unreleased]

### Added

- Rust-ingest-tool die publiccode.yml, contributors, pull requests, talen en
  dependencies van gecureerde projecten ophaalt via de GitHub API.
- Statische catalogus- en projectdetailpagina in vanilla JavaScript.
- Sorteren van dependencies.
- Open source-bestanden: EUPL-1.2-licentie, publiccode.yml, SECURITY.md,
  CONTRIBUTING.md, CODE_OF_CONDUCT.md en REUSE-compliance.
- Rust-backend met Axum (`server/`): haalt de GitHub-data bij opstarten en
  daarna elke 24 uur in een achtergrondthread op, houdt het resultaat in het
  geheugen en rendert de catalogus- en projectdetailpagina's server-side met
  Tera-templates (`templates/`). Inclusief laadscherm tijdens de eerste
  ingest en een "data laatst bijgewerkt"-vermelding in de footer.
- Filteren op programmeertaal op de cataloguspagina: meerkeuze-checkboxes
  (met type-ahead zoekveld en "Toon meer"-knop) in de zijbalk, werkend samen
  met de bestaande filters; de server filtert vooraf op `?language=`-params,
  zodat de links ook zonder JavaScript werken.
- Dependencies per package manager op de detailpagina: per package manager
  een tab ("N direct + M transitief", rode badge met het aantal unieke
  OSV-advisories, kwetsbare ecosystemen vooraan) met de directe dependencies
  die de meeste transitieve dependencies binnenhalen. De ingest bewaart de
  top 20 per ecosysteem met het echte aantal directe dependencies
  (`directCount`).
- Repositoryregels (naar
  [developer-overheid-nl/ort-config](https://github.com/developer-overheid-nl/ort-config)):
  de ingest controleert tijdens het ophalen aanwezigheid van communityfiles
  (root, `.github/`, `docs/`) en CI-configuratie plus bekende kwetsbaarheden
  via de OSV batch-API; de detailpagina toont een samenvatting en per regel
  of het project eraan voldoet, met uitklapbare kwetsbare packages en
  OSV-links.
- De taal-tags op de detailpagina linken naar de catalogus gefilterd op die
  taal (`/?language=…`).
- Render-tests voor de catalogus- en detailpagina (fixturedata, geen
  netwerkverkeer).

### Changed

- Architectuur: de losse Rust-ingest-CLI en het statische
  `web/data/projects.json` vervallen; één Rust-backend draait nu de volledige
  applicatie (zie ARCHITECTURE.md).
- De frontend-JS (`web/app.js`) is nu uitsluitend progressive enhancement op
  de server-gerenderde DOM (live zoeken, filteren en sorteren via
  `data-*`-attributen). Tag-chips en -pills zijn echte links (`/?tag=…`) en
  alle pagina's tonen ook zonder JavaScript de volledige inhoud; meerkeuze
  op tags werkt ook zonder JS (de server filtert op herhaalde `?tag=`-params).
- De dependency-sectie op de detailpagina toont per package manager een
  eigen toplijst (voorheen één gecombineerde top 20) en is interactief:
  `web/detail.js` schakelt tussen de package managers (zonder JavaScript
  blijven alle lijsten zichtbaar).

### Removed

- De losse ingest-CLI en het statische `web/data/projects.json`.
- De JSON-gebaseerde frontend: `web/index.html`, `web/project.html`,
  `web/detail.js` en alle component-JS die data client-side renderde.
- De GitHub Pages-workflow (`.github/workflows/pages.yml`): deze publiceerde
  de statische site in `web/`, die door de backend-migratie is vervallen.
