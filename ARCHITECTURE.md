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
  - https://github.com/wigo4it/nl-app
  - https://github.com/wigo4it/techradar
  - https://github.com/wigo4it/kwartaalmeeting
  - https://github.com/wigo4it/knowledge-sharing-jp
  - https://github.com/wigo4it/kerstbingo
  - https://github.com/wigo4it/duurzaamheidsradar
  - https://github.com/wigo4it/DevStream
  - https://github.com/wigo4it/github-action-azure-devops-pipeline-runner
  - https://github.com/wigo4it/module-tf-azure-aks
  - https://github.com/wigo4it/platform-infra-talos-endpoints
  - https://github.com/wigo4it/wigo4it-code-conventions-mcp
  - https://github.com/wigo4it/wigo4it-multitenant
  - https://github.com/wigo4it/gitops-core-infra
  - https://github.com/wigo4it/POC-Socrates-PDF-Vergelijker
  - https://github.com/wigo4it/wigo4it-brandbook
  - https://github.com/wigo4it/renovate-config
  - https://github.com/wigo4it/azure-jwks-publisher
  - https://github.com/wigo4it/modules-tf-leafcloud
  - https://github.com/wigo4it/w4u-api-test-client
  - https://github.com/wigo4it/jwks-publisher-w4
  - https://github.com/kadaster/imbag
  - https://github.com/kadaster/imkad
  - https://github.com/kadaster/imbrt
  - https://github.com/kadaster/top10nl
  - https://github.com/kadaster/top50nl
  - https://github.com/kadaster/top250nl
  - https://github.com/kadaster/brt-algemeen
  - https://github.com/kadaster/klic-win
  - https://github.com/kadaster/top100nl
  - https://github.com/kadaster/top500nl
  - https://github.com/kadaster/vgl
  - https://github.com/kadaster/top1000nl
  - https://github.com/kadaster/metagegevens-voorbeelden
  - https://github.com/kadaster/BRK-bevragen
  - https://github.com/kadaster/bag-leveranciers
  - https://github.com/kadaster/WOZ-bevragen
  - https://github.com/kadaster/eurogeographics-erm-validator
  - https://github.com/kadaster/ladm-composer
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
