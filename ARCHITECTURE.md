# Architectuur

- **Data**: de inhoud van de catalogus bestaat uit een klein aantal echte open
  source overheidsprojecten (geen synthetische/nepdata), samengesteld voor de demo.
  - https://github.com/kiesraad/abacus
  - github.com/Amsterdam/design-system.git
  - https://github.com/SSC-ICT-Innovatie/nl-kat-coordination
  - https://github.com/MinBZK/amt.git
  - https://github.com/nl-design-system/rijkshuisstijl-community
  - https://github.com/developer-overheid-nl/don-site
  - https://github.com/developer-overheid-nl/skills-marketplace
  - https://github.com/ICTU/quality-time.git
  - https://github.com/MinBZK/MijnOverheidZakelijk
  - https://github.com/minvws/gfmodules-nationale-verwijsindex
  - https://github.com/developer-overheid-nl/don-infra.git
  - https://github.com/MinBZK/regelrecht
  - https://github.com/MinBZK/nl-wallet
  - https://github.com/kadaster/generieke-geo-componenten
  - https://github.com/Geonovum/ogc-checker
  - https://github.com/Signalen/frontend
  - https://github.com/RIVM-bioinformatics/Pacini-typing
  - https://github.com/Sudwest-Fryslan/OpenZaakBrug
  - https://github.com/belastingdienst/opr-paas
  - https://github.com/belastingdienst/opr-paas-cli
  - https://github.com/TNO/knowledge-engine
  - https://github.com/nl-design-system/candidate
  - https://github.com/nl-design-system/documentatie
  - https://github.com/Amsterdam/meldingen-frontend
  - https://github.com/Amsterdam/openstad-headless
  - https://github.com/Amsterdam/meldingen
  - https://github.com/GemeenteNijmegen/design-system
  - https://github.com/GemeenteNijmegen/mijn-nijmegen
  - https://github.com/GemeenteNijmegen/mule-infra
  - https://github.com/wigo4it/wigo4it-multitenant
  - https://github.com/kadaster/BRK-bevragen
  - https://github.com/kadaster/WOZ-bevragen
- **Repositoryregels**: projecten worden getoetst aan de regels uit
  [developer-overheid-nl/ort-config](https://github.com/developer-overheid-nl/ort-config)
  (`evaluator.rules.kts`). In de PoC draaien we geen ORT zelf; de ingest bouwt
  dezelfde regels na: bestandsaanwezigheid via de GitHub git-trees-API (root,
  `.github/` en `docs/`), kwetsbaarheden via de
  [OSV batch-API](https://google.github.io/osv.dev/post-v1-querybatch/) op de
  packages uit de SBOM, net als ORT's Advisor. Alleen exacte versies worden
  bevraagd (versiebereiken uit manifests zonder lockfile worden overgeslagen).
- **Metadatastandaard**: projectmetadata volgt de
  [publiccode.yml](https://yml.publiccode.tools/) standaard.
  - Schema: https://yml.publiccode.tools/v0/schema.core.html
- **Vormgeving**: als inspiratie voor de UI gebruiken we de Rijkshuisstijl
  Community componentenbibliotheek (Storybook):
  https://rijkshuisstijl-community.vercel.app/?path=/docs/rijkshuisstijl-community-readme--docs
  - Icoontjes: inspiratie hiervoor zoeken we in de Rijkshuisstijl Community
    icon set:
    https://rijkshuisstijl-community.vercel.app/?path=/story/rhc-templates-default-icon-set--default
  - Vlaggen: inspiratie hiervoor zoeken we in de Rijkshuisstijl Community
    flag icon set:
    https://rijkshuisstijl-community.vercel.app/?path=/story/rhc-templates-flag-icon-set--default
- **Backend**: één Rust-binary draait de volledige applicatie — geen los
  ingest-script en geen statisch JSON-bestand meer (`web/data/projects.json`
  vervalt). Als webframework gebruiken we [Axum](https://github.com/tokio-rs/axum).
  - **Data-ingest** gebeurt als achtergrondtaak binnen dezelfde backend: bij
    opstarten en daarna periodiek bevraagt de backend de GitHub API
    (publiccode.yml, contributors, PR's, talen, dependencies) voor de
    gecureerde projecten. Het resultaat blijft in-memory; een database of
    ander opslagformaat is een mogelijke latere stap.
  - De GitHub-token komt uit de omgeving (`GITHUB_TOKEN`, of `gh auth login`)
    om rate-limiting te voorkomen.
  - Er wordt nooit live vanuit de browser naar GitHub gecalled — al het
    GitHub-verkeer loopt via de backend (voorkomt rate-limit- en
    tokenproblemen).
  - Voorgestelde repo-indeling: `server/` (Rust-crate: Axum-backend, routes en
    ingest-achtergrondtaak), `templates/` (templates/partialen voor pagina's
    en componenten) en `web/` (statische assets: JS-modules en CSS).
- **Frontend-aanpak**: server-side rendering met progressive enhancement in
  vanilla JavaScript (geen framework).
  - De backend rendert alle pagina's en componenten (inclusief zoekbalk,
    filters en sorteerselectie) server-side met een template-engine (bijv.
    [Tera](https://tera.netlify.app/) of
    [Askama](https://askama.readthedocs.io/)). Componentmarkup staat als
    template-partial — naar het patroon van de huidige
    `<template>`-blokken — gevuld door de server in plaats van door de browser.
  - Zonder JavaScript tonen de pagina's de volledige projectenlijst en alle
    detailinformatie. De JS-laag (ES modules, `<script type="module">`)
    verrijkt de gerenderde DOM daarna: live zoeken, filteren en sorteren
    gebeurt client-side op de aanwezige elementen (filterdata staat in
    `data-*`-attributen), niet door data te fetchen.
  - Er is geen JSON-API en de frontend fetcht geen data: alle data verlaat de
    backend als gerenderde HTML.
  - Styling per component in een eigen CSS-bestand, met class-namen geprefixt op
    de componentnaam (bijv. `project-card.css` met `.project-card`,
    `.project-card__tag`) — geen echte scoping (geen Shadow DOM), maar een
    naamgevingsafspraak om stijlconflicten te voorkomen.
- **Pagina's** (gerouteerd en gerenderd door de backend):
  - Catalogus browse-/lijstpagina (`/`) — toont alle projecten.
  - Projectdetailpagina (`/project/{slug}`) — één per project, bereikbaar door
    op een item in de lijst te klikken, met alle informatie over dat project.
