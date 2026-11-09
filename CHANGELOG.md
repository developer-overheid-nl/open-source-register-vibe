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

### Changed

- Architectuur: de losse Rust-ingest-CLI en het statische
  `web/data/projects.json` vervallen. Er komt één Rust-backend (Axum) die de
  GitHub-data periodiek als achtergrondtaak in-memory ophaalt, de pagina's
  server-side rendert en de statische assets serveert; de vanilla JS-frontend
  wordt een progressive-enhancement-laag op de gerenderde pagina's. De
  documentatie is alvast bijgewerkt; de code volgt nog.
