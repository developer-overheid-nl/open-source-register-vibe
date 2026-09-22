# Open Source Catalogus

Proof of concept van een catalogus voor open source overheidsprojecten. Zie
[AGENTS.md](./AGENTS.md) voor de doelstelling van dit project, [FEATURES.md](./FEATURES.md)
voor de featurespecificaties (user stories en acceptatiecriteria), en
[ARCHITECTURE.md](./ARCHITECTURE.md) voor architectuurdetails.

## Hoe het werkt

Het project bestaat uit twee delen:

- **`ingest/`** — een Rust-commandline-tool die de GitHub API bevraagt voor een
  vaste lijst gecureerde open source overheidsprojecten (publiccode.yml,
  contributors, pull requests, talen, dependencies) en het resultaat wegschrijft
  naar `web/data/projects.json`.
- **`web/`** — een statische frontend in vanilla JavaScript (ES modules, geen
  framework) die uitsluitend dat statische JSON-bestand uitleest. Er wordt nooit
  live vanuit de browser naar GitHub gecalled.

## Data verversen

```bash
cd ingest
cargo run
```

Optioneel: zet `GITHUB_TOKEN` (of log in met `gh auth login`) om rate-limiting
van de GitHub API te voorkomen:

```bash
export GITHUB_TOKEN=<jouw-token>
```

Dit schrijft `web/data/projects.json` opnieuw weg.

## Frontend draaien

De frontend is statisch, maar moet via een webserver bekeken worden (niet als
`file://`, vanwege de `fetch()`-call naar `data/projects.json`):

```bash
cd web
python3 -m http.server 8000
```

Open vervolgens <http://localhost:8000>.

## TODO
- Add counts for the tags, show popularity.