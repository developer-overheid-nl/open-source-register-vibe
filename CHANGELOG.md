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

### Changed

- Architectuur: de losse Rust-ingest-CLI en het statische
  `web/data/projects.json` vervallen; één Rust-backend draait nu de volledige
  applicatie (zie ARCHITECTURE.md).
- De frontend-JS (`web/app.js`) is nu uitsluitend progressive enhancement op
  de server-gerenderde DOM (live zoeken, filteren en sorteren via
  `data-*`-attributen). Tag-chips en -pills zijn echte links (`/?tag=…`) en
  alle pagina's tonen ook zonder JavaScript de volledige inhoud; meerkeuze
  op tags werkt ook zonder JS (de server filtert op herhaalde `?tag=`-params).

### Removed

- De losse ingest-CLI en het statische `web/data/projects.json`.
- De JSON-gebaseerde frontend: `web/index.html`, `web/project.html`,
  `web/detail.js` en alle component-JS die data client-side renderde.
