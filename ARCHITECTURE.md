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
- **Frontend-aanpak**: vanilla JavaScript, geen framework.
  - ES modules (`<script type="module">`); elk component krijgt zijn eigen
    `.js`-bestand dat een render/mount-functie exporteert, geïmporteerd in een
    hoofdbestand (bijv. `app.js`).
  - Markup per component staat als `<template>`-element in `index.html` (met een
    `id`); de component-JS haalt dit op via `document.getElementById`, kloont het
    (`cloneNode`) en vult het met data.
  - Styling per component in een eigen CSS-bestand, met class-namen geprefixt op
    de componentnaam (bijv. `project-card.css` met `.project-card`,
    `.project-card__tag`) — geen echte scoping (geen Shadow DOM), maar een
    naamgevingsafspraak om stijlconflicten te voorkomen.
- **Backend / data-ingest**: geen live backend-server. Een los ingest-script in
  Rust bevraagt periodiek/handmatig de GitHub API (publiccode.yml, contributors,
  PR's, talen, dependencies) voor de gecureerde projecten en schrijft het
  resultaat weg naar een statisch JSON-bestand (bijv. `data/projects.json`). De
  vanilla JS-frontend fetcht alleen dat statische bestand; er wordt nooit live
  naar GitHub gecalled vanuit de browser (voorkomt rate-limit- en tokenproblemen).
- **Pagina's**:
  - Catalogus browse-/lijstpagina — toont alle projecten.
  - Projectdetailpagina — één per project, bereikbaar door op een item in de lijst
    te klikken, met alle informatie over dat project.
