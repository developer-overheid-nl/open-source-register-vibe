# Open Source Catalogus

Proof of concept van een catalogus voor open source overheidsprojecten. Zie
[AGENTS.md](./AGENTS.md) voor de doelstelling van dit project, [FEATURES.md](./FEATURES.md)
voor de featurespecificaties (user stories en acceptatiecriteria), en
[ARCHITECTURE.md](./ARCHITECTURE.md) voor architectuurdetails.

## Hoe het werkt

De applicatie bestaat uit één Rust-backend (zie
[ARCHITECTURE.md](./ARCHITECTURE.md) voor details):

- De backend bevraagt bij opstarten en daarna periodiek — als achtergrondtaak —
  de GitHub API voor een vaste lijst gecureerde open source overheidsprojecten
  (publiccode.yml, contributors, pull requests, talen, dependencies) en houdt
  het resultaat vast in het geheugen. Een los ingest-script en een statisch
  JSON-bestand zijn er niet meer.
- De backend rendert de catalogus- en projectdetailpagina's server-side en
  serveert zelf de statische assets (JS/CSS); een aparte webserver is niet
  nodig.
- De frontend is een progressive-enhancement-laag in vanilla JavaScript: de
  browser haalt zelf geen data op en calt nooit live naar GitHub — live zoeken,
  filteren en sorteren gebeurt op de server-gerenderde pagina.

## Draaien

```bash
cd server
cargo run
```

Optioneel: zet `GITHUB_TOKEN` (of log in met `gh auth login`) om rate-limiting
van de GitHub API te voorkomen:

```bash
export GITHUB_TOKEN=<jouw-token>
```

De backend ververst de GitHub-data automatisch: bij opstarten en daarna
periodiek. Open het adres dat in de terminal gelogd wordt, bijv.
<http://localhost:3000>.

## TODO
- Add counts for the tags, show popularity.
## Ontwikkelstatus

Dit project heeft de status: **concept** (proof of concept).

## Bijdragen

We verwelkomen bijdragen aan dit project! Bekijk [CONTRIBUTING.md](CONTRIBUTING.md) om te zien hoe je kunt helpen.

## Gedragscode

Dit project hanteert een [gedragscode](CODE_OF_CONDUCT.md). Door bij te dragen aan dit project ga je akkoord met de voorwaarden hiervan.

## Security

Heb je een potentieel securityissue gevonden? Fijn dat je de moeite hebt genomen om hier in te duiken. Hoe je op een veilige manier melding kan maken vind je in [SECURITY.md](SECURITY.md).

## Licentie

Copyright © developer.overheid.nl.
Licensed under the [EUPL-1.2](LICENSE.md).

## Contact

Naam: Tom Ootes\
Email: t.ootes@geonovum.nl\
Organisatie: Geonovum

## Meer informatie

- [publiccode.yml](publiccode.yml) - Metadata volgens de publiccode.yml standaard
