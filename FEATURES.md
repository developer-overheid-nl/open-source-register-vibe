# Features

Dit bestand houdt de features van de Open Source Catalogus proof of concept bij,
één user story per feature. Nieuwe features worden hier toegevoegd zodra ze zijn
uitgewerkt.

## Catalogus zoeken

**User story**
Als gebruiker van de catalogus wil ik open source projecten kunnen zoeken op naam
of trefwoord, zodat ik snel een project kan vinden zonder door de volledige lijst
te scrollen.

**Acceptatiecriteria**
- Gegeven de cataloguspagina, is er een zoekveld zichtbaar bovenaan de projectenlijst.
- Wanneer ik een zoekterm typ in het zoekveld, wordt de lijst gefilterd op projecten
  die overeenkomen met die zoekterm (gematcht op projectnaam, en beschrijving/tags
  indien aanwezig).
- Wanneer ik het zoekveld leegmaak, wordt de volledige, ongefilterde lijst weer getoond.
- Wanneer mijn zoekterm geen projecten oplevert, toont de pagina een duidelijke
  "geen resultaten"-melding in plaats van een lege pagina.
- Zoeken is niet hoofdlettergevoelig.

**Notities / open vragen**
- **Besloten (PoC-implementatie):** doorzocht worden naam, beschrijving en tags
  (case-insensitive substring match).
- **Besloten (PoC-implementatie):** live filteren tijdens het typen, met 250ms debounce.

## Filteren op tags

**User story**
Als gebruiker van de catalogus wil ik projecten kunnen filteren op tag (vergelijkbaar
met GitHub's topic-filtering), zodat ik de lijst kan versmallen tot projecten die
relevant zijn voor een technologie, sector of categorie die mij interesseert.

**Acceptatiecriteria**
- Gegeven een project tags heeft, wordt elke tag getoond bij het project in de
  cataloguslijst en op de detailpagina.
- Wanneer ik op een tag klik (in een lijstitem, op een detailpagina, of in een
  tagfilter), wordt de cataloguslijst gefilterd op projecten met die tag.
- Gegeven er één of meer tags actief zijn als filter, worden deze getoond als
  verwijderbare chips/pills, zodat ik kan zien waarop de lijst momenteel gefilterd is.
- Wanneer ik een actief tagfilter verwijder, wordt de lijst bijgewerkt zodat die
  tag niet langer vereist is.
- Wanneer geen enkel project voldoet aan de actieve tagfilter(s), toont de pagina
  een duidelijke "geen resultaten"-melding in plaats van een lege pagina.
- Filteren op tags werkt samen met de zoekbalk — een zoekterm en een actief
  tagfilter versmallen de lijst gezamenlijk, niet los van elkaar.

**Notities / open vragen**
- **Besloten (PoC-implementatie):** meerdere actieve tags werken als EN (zoals
  GitHub's topic-filters) — een project moet aan alle geselecteerde tags voldoen.
- **Besloten (PoC-implementatie):** tags komen uit het `categories`-veld van
  `publiccode.yml` indien aanwezig, anders uit de GitHub-topics van de repository.
- **Besloten (PoC-implementatie):** geen aparte "alle tags"-weergave; tags zijn
  te ontdekken via de tagfilter in de zijbalk en via projectitems/detailpagina's.

## Waarschuwing bij ontbrekende publiccode.yml

**User story**
Als gebruiker van de catalogus wil ik een waarschuwing zien bij het lijstitem van
een project wanneer dat project geen `publiccode.yml` heeft (zie
[ARCHITECTURE.md](./ARCHITECTURE.md) voor de metadatastandaard), zodat ik weet dat
de metadata mogelijk onvolledig of niet geverifieerd is.

**Acceptatiecriteria**
- Gegeven een project geen `publiccode.yml` heeft, toont het item in de
  cataloguslijst een zichtbare waarschuwingsindicator (bijv. een icoon of badge).
- Gegeven een project wél een `publiccode.yml` heeft, wordt er geen waarschuwing
  getoond.
- De waarschuwing is zichtbaar op lijstniveau, zonder dat de detailpagina van het
  project geopend hoeft te worden.
- Op de detail pagina staat een alert met een volledige uitleg-zin:
"⚠ Geen publiccode.yml gevonden voor dit project — onderstaande metadata is mogelijk onvolledig of niet geverifieerd. "

## Package manager & aantal dependencies

**User story**
Als gebruiker van de catalogus wil ik zien welke package manager(s) een project
gebruikt en hoeveel dependencies (inclusief transitieve/indirecte) daarmee
geïnstalleerd worden, zodat ik een beeld krijg van de dependency-omvang van het
project.

**Acceptatiecriteria**
- Gegeven de detailpagina van een project, kan ik zien welke package manager(s)
  het gebruikt (bijv. npm, pip, Maven, Cargo) — een project kan er meerdere gebruiken.
- Gegeven de detailpagina van een project, kan ik het totaal aantal dependencies
  zien dat via elke package manager geïnstalleerd wordt, inclusief transitieve
  (indirecte) dependencies, niet alleen directe/gedeclareerde.
- Gegeven een project geen detecteerbare package manager heeft (bijv. geen
  manifest/lockfile gevonden), toont de detailpagina dit duidelijk in plaats van
  een leeg of misleidend aantal.

**Notities / open vragen**
- **Besloten (PoC-implementatie):** berekend tijdens de ingest-achtergrondtaak
  van de backend, via GitHub's dependency-graph/SBOM-endpoint
  (`/repos/{owner}/{repo}/dependency-graph/sbom`), gegroepeerd per ecosysteem
  (npm, pip, Cargo, GitHub Actions, …).
- **Besloten (PoC-implementatie):** directe en transitieve dependencies worden
  gecombineerd getoond als één totaal per package manager (de SBOM maakt dit
  onderscheid niet expliciet beschikbaar).

## Dependencies per package manager

**User story**
Als gebruiker van de catalogus wil ik op de detailpagina een package manager
kunnen aanklikken en dan alleen de dependencies van die package manager zien,
gesorteerd op het aantal transitieve dependencies, zodat ik per ecosysteem kan
zien welke directe dependencies de meeste andere packages binnenhalen.

**Acceptatiecriteria**
- Gegeven de detailpagina van een project met één of meer package managers, kan
  ik op een package manager klikken.
- Wanneer ik op een package manager klik, toont de lijst alleen de directe
  dependencies van die package manager, gesorteerd van meeste naar minste
  transitieve dependencies.
- Bij het openen van de detailpagina is de eerste package manager al
  geselecteerd; er is geen gecombineerd overzicht over alle package managers.
- Gegeven een package manager waarvoor geen directe dependencies bekend zijn,
  kan ik deze niet aanklikken.
- Bij elke package manager zie ik het aantal directe dependencies, met daarachter
  (minder prominent) het aantal transitieve dependencies, bijv. "12 direct
  + 559 transitief".
- Gegeven een package manager met packages waarin bekende kwetsbaarheden zitten,
  zie ik bij die package manager een rood bolletje met het aantal
  kwetsbaarheden, en staan deze package managers vooraan (meeste eerst).

**Notities / open vragen**
- **Besloten (PoC-implementatie):** de ingest bewaart de top 20 directe
  dependencies *per* package manager (in plaats van de top 20 over alle samen),
  zodat ook kleinere ecosystemen een eigen lijst hebben. Het totale aantal
  directe dependencies per package manager staat in `directCount`.
- **Besloten (PoC-implementatie):** het bolletje telt unieke OSV-advisories
  (GHSA, PYSEC, RUSTSEC, …) uit de repositoryregels, niet strikt CVE-nummers.
  Eén CVE kan onder meerdere advisory-ID's voorkomen en dan dubbel tellen.

## Repositoryregels

**User story**
Als gebruiker van de catalogus wil ik op de detailpagina zien of een project
voldoet aan de repositoryregels van Developer Overheid
([ort-config](https://github.com/developer-overheid-nl/ort-config)), zodat ik
snel kan inschatten hoe goed een project is ingericht voor hergebruik en
samenwerking.

**Acceptatiecriteria**
- Gegeven de detailpagina van een project, zie ik per regel of het project eraan
  voldoet:

  | Regel | Ernst |
  | --- | --- |
  | Geen bekende kwetsbaarheid in een dependency | Waarschuwing |
  | `README.md` aanwezig | Fout |
  | `LICENSE` aanwezig | Fout |
  | `publiccode.yml` of `publiccode.yaml` aanwezig | Fout |
  | `CONTRIBUTING.md` aanwezig | Fout |
  | `SECURITY.md` aanwezig (ook in `.github/` of `docs/`) | Fout |
  | `CODE_OF_CONDUCT.md` aanwezig (ook in `.github/` of `docs/`) | Waarschuwing |
  | `CHANGELOG` of `CHANGELOG.md` aanwezig (ook in `.github/` of `docs/`) | Waarschuwing |
  | Bekende CI-configuratie aanwezig | Waarschuwing |

- Ik zie bovenaan een samenvatting (bijv. "6 van 9 regels voldaan · 2 fouten ·
  1 waarschuwing"); overtredingen staan bovenaan, fouten vóór waarschuwingen.
- Gegeven een project kwetsbare dependencies heeft, kan ik zien welke packages
  dat zijn en via welke advisories (met link naar OSV).
- Gegeven de regels voor een project niet gecontroleerd konden worden, toont de
  detailpagina dit duidelijk in plaats van een misleidende uitslag.

**Notities / open vragen**
- **Besloten (PoC-implementatie):** de regels worden tijdens ingest nagebouwd in
  plaats van ORT zelf te draaien (zie [ARCHITECTURE.md](./ARCHITECTURE.md)).
  Bestandsregels controleren alleen aanwezigheid, niet de inhoud — net als in
  `evaluator.rules.kts`.
- **Besloten (PoC-implementatie):** alleen packages met een exacte versie worden
  op kwetsbaarheden gecontroleerd; bij een project zonder lockfile kan de
  kwetsbaarheidsregel dus te gunstig uitvallen.
- Open vraag: wanneer de ort-config-regels wijzigen, moet de ingest handmatig
  worden bijgewerkt. In het echte product ligt het voor de hand de ORT-runner
  zelf te gebruiken en diens `run.json` in te lezen.
- Open vraag: ook tonen/filteren op de cataloguslijst (bijv. alleen projecten
  zonder fouten)?

## GitHub-link op detailpagina

**User story**
Als gebruiker van de catalogus wil ik op de detailpagina snel de link naar de
GitHub-repository van een project kunnen vinden, zodat ik direct naar de broncode
kan doorklikken.

**Acceptatiecriteria**
- Gegeven de detailpagina van een project, is de link naar de repository direct
  zichtbaar (bijv. bovenaan de pagina), zonder te hoeven scrollen of zoeken.
- Wanneer ik op de link klik, wordt de repository van het project geopend
  (in een nieuw tabblad).
- Gegeven een project niet op GitHub gehost is, toont de detailpagina in plaats
  daarvan de link naar de daadwerkelijke locatie van de broncode.

**Notities / open vragen**
- **Besloten (PoC-implementatie):** een knop rechtsboven op de detailpagina,
  naast de projectnaam, die in een nieuw tabblad opent. Niet herhaald op
  lijstniveau (de kaart zelf linkt al naar de detailpagina).

## Softwaretype

**User story**
Als gebruiker van de catalogus wil ik op de detailpagina van een project kunnen
zien wat voor softwaretype het is, en de cataloguslijst hierop kunnen filteren,
zodat ik weet wat voor soort software het project behelst en snel projecten van
een bepaald softwaretype kan vinden.

**Acceptatiecriteria**
- De overzichtspagina en de detailpagina tonen het softwaretype van het project,
  gebaseerd op het `softwareType`-veld uit `publiccode.yml`
  (zie [ARCHITECTURE.md](./ARCHITECTURE.md)).
- Als een project geen `publiccode.yml` heeft of geen `softwareType` opgeeft, toont
  de detailpagina dit duidelijk in plaats van een leeg of misleidend veld.
- Gegeven de cataloguspagina, kan ik filteren op softwaretype.
- Wanneer ik één of meer softwaretypes selecteer, toont de lijst alleen
  projecten van dat/die type(s).
- Filteren op softwaretype werkt samen met de andere filters (zoekbalk, tags,
  ontwikkelstatus, organisatie) — ze versmallen de lijst gezamenlijk, niet los
  van elkaar.

**Notities / open vragen**
- De toegestane waarden voor `softwareType` staan gedefinieerd in de
  [publiccode.yml schema-documentatie](https://yml.publiccode.tools/v0/schema.core.html)
  (zie ook [ARCHITECTURE.md](./ARCHITECTURE.md)).
- **Besloten (PoC-implementatie):** ruwe schema-waarde 1-op-1 getoond (bijv.
  `standalone/web`), geen vertaling naar Nederlandse labels — ook in het filter.
- **Besloten (PoC-implementatie):** softwaretype is filterbaar via een
  meerkeuzefilter (checkboxes, OF-logica binnen dit filter) in de zijbalk,
  naar hetzelfde patroon als ontwikkelstatus en organisatie. Op de
  cataloguskaart zelf wordt het softwaretype (nog) niet apart getoond — alleen
  op de detailpagina.
- Een project zonder `softwareType` (bijv. zonder `publiccode.yml`) wordt niet
  getoond zodra minstens één softwaretype actief geselecteerd is, conform het
  patroon van het ontwikkelstatus-filter.

## Programmeertalen

**User story**
Als gebruiker van de catalogus wil ik zien welke programmeertalen een project
gebruikt, zodat ik projecten kan vinden op basis van de gebruikte technologie.

**Acceptatiecriteria**
- Gegeven een project, worden de gebruikte programmeertalen getoond op de
  detailpagina.
- Gegeven een project geen detecteerbare programmeertaal heeft, toont de pagina
  dit duidelijk in plaats van een leeg of misleidend veld.
- Het ophalen van programmeertalen moet forge‑agnostisch gebeuren en werkt zowel met Forgejo‑ als met GitLab‑instellingen.

## Filteren op programmeertaal

**User story**
Als gebruiker van de catalogus wil ik de cataloguslijst kunnen filteren op
programmeertaal, zodat ik snel projecten vind die gebouwd zijn in een taal die
aansluit bij de kennis of het technologielandschap van mijn organisatie.

**Acceptatiecriteria**
- Gegeven de cataloguspagina, kan ik filteren op programmeertaal.
- Het filter toont alleen talen die daadwerkelijk voorkomen in de projecten in
  de catalogus.
- Wanneer ik één of meer programmeertalen selecteer, toont de lijst alleen
  projecten die die taal/talen gebruiken.
- Gegeven een project geen detecteerbare programmeertaal heeft, wordt het project
  niet getoond zodra minstens één taal actief geselecteerd is.
- Wanneer geen enkel project voldoet aan de actieve filter(s), toont de pagina
  een duidelijke "geen resultaten"-melding in plaats van een lege pagina.
- Filteren op programmeertaal werkt samen met de andere filters (zoekbalk, tags,
  ontwikkelstatus, organisatie, softwaretype) — ze versmallen de lijst
  gezamenlijk, niet los van elkaar.

**Notities / open vragen**
- **Besloten (PoC-implementatie):** een project telt mee voor elke taal die het
  gebruikt, ongeacht het aandeel in de codebase. Gevolg: onder talen als Shell
  of Dockerfile verschijnen veel projecten, en het filter heeft veel opties.
- **Besloten (PoC-implementatie):** meerdere geselecteerde talen werken als OF,
  naar het patroon van ontwikkelstatus, organisatie en softwaretype.
- **Besloten (PoC-implementatie):** meerkeuzefilter (checkboxes) in de zijbalk,
  gesorteerd op aantal projecten dat de taal gebruikt (meest gebruikt eerst).
  Alleen de eerste 10 talen zijn zichtbaar, met een "Toon meer"/"Toon
  minder"-knop voor de rest (naar het patroon van het tagfilter); een actief
  geselecteerde taal blijft altijd zichtbaar.
- **Besloten (PoC-implementatie):** boven de taal-checkboxes staat een
  type-ahead zoekveld dat de opties live versmalt (niet hoofdlettergevoelig,
  substring-match). Tijdens het zoeken worden alle overeenkomende talen
  getoond, ongeacht de "Toon meer"-limiet; zonder overeenkomsten toont het
  filter "Geen overeenkomende opties.".
- **Besloten (PoC-implementatie):** op de detailpagina zijn de taal-labels
  klikbaar en openen ze de catalogus gefilterd op die taal
  (`index.html?language=…`, naar het patroon van tags). Alle taal-labels zijn
  klikbaar. Op de cataloguskaart zijn talen niet klikbaar.


## Contributor- en PR-activiteit per periode

**User story**
Als gebruiker van de catalogus wil ik op de detailpagina van een project zien
hoeveel contributors er actief waren in de afgelopen 6 maanden, in de periode 6
tot 12 maanden geleden, en in de periode ouder dan 12 maanden, zodat ik een beeld
krijg van hoe actief het project is en of die activiteit toe- of afneemt.

**Acceptatiecriteria**
- Gegeven de detailpagina van een project, zijn er drie aparte vlakken in de UI,
  één per periode:
  - Laatste 6 maanden
  - 6 tot 12 maanden geleden
  - Ouder dan 12 maanden
- Elk vlak toont het exacte aantal contributors dat in die specifieke periode
  heeft bijgedragen, met daarbij hun profielfoto's als overlappende cirkels
  (naar het patroon van GitHub's contributor-avatars).
- Elk vlak toont daarnaast het aantal PR's dat in die specifieke periode is
  gesloten (closed).
- Gegeven een project in een bepaalde periode geen activiteit heeft, toont het
  vlak voor die periode een aantal van 0 in plaats van een leeg of ontbrekend vlak.

**Notities / open vragen**
- **Besloten (PoC-implementatie):** data komt uit de GitHub Pulls-API
  (`state=closed`), periodiek ingelezen tijdens de ingest-achtergrondtaak van
  de backend (niet live).
- **Besloten (PoC-implementatie):** een "contributor" in een periode is de auteur
  van een PR die in die periode gesloten is (commit-auteurs zonder PR en
  reviewers tellen niet apart mee) — dit dekt niet elk mogelijk werkproces, maar
  is een pragmatische, met de GitHub API goed te bepalen definitie.
- **Besloten (PoC-implementatie):** periodegrenzen zijn op dagbasis: 182 dagen
  voor "laatste 6 maanden", 365 dagen voor de 12-maandsgrens.
- **Besloten (PoC-implementatie):** de backend slaat per contributor ook de
  GitHub-login en avatar-URL op (niet alleen het aantal), zodat de
  detailpagina per periode een avatar-stack kan tonen; alle contributors van
  de periode worden getoond (geen afkapping/"+N"-indicator), de rij wraait
  naar een volgende regel binnen het vlak wanneer nodig.
- Om het aantal API-calls te beperken worden per project maximaal 500 gesloten
  PR's (5 pagina's) meegenomen; voor zeer actieve repositories kan de telling in
  de oudste periode daardoor een onderschatting zijn.
- **Besloten (PoC-implementatie):** onder de drie periode-vlakken toont de
  detailpagina ook een commit-kalender: één vierkantje per dag, geschakeerd
  naar activiteit, naar het patroon van de contributiegrafiek op een GitHub-
  profielpagina (bijv. github.com/tomootes) — inclusief maandlabels en een
  "Minder–Meer"-legenda. Kleurintensiteit is relatief aan de drukste dag van
  het project (geen vaste drempelwaarden), zodat zowel kleine als zeer actieve
  projecten een leesbare spreiding tonen.
- **Besloten (PoC-implementatie):** de kalender toont expliciet op welke
  branch hij is gebaseerd (de default branch van de repository — niet elk
  project gebruikt hiervoor `main`; het Amsterdam design-system-project
  gebruikt bijvoorbeeld `develop`). Bron: GitHub's
  `stats/commit_activity`-endpoint (dagelijkse granulariteit), tijdens de
  ingest-achtergrondtaak opgehaald. Is deze data niet beschikbaar (GitHub moet de
  statistiek soms nog berekenen, of de repository is te groot), dan toont de
  detailpagina dit duidelijk in plaats van een lege of misleidende kalender.
- **Besloten (PoC-implementatie):** PR's gesloten door bot-accounts (bijv.
  Dependabot, pre-commit-ci, github-actions[bot]) tellen niet mee in de
  contributor- en PR-tellingen van de drie periode-vlakken, en verschijnen
  niet in de avatar-stacks — dit voorkomt dat geautomatiseerde
  dependency-updates de activiteit van menselijke bijdragers overschaduwen.
  Herkenning gebeurt via GitHub's eigen `type: "Bot"`-veld op de PR-auteur,
  niet via naampatronen. De commit-kalender hieronder komt uit een apart,
  vooraf door GitHub geaggregeerd endpoint (`stats/commit_activity`) en telt
  dus nog wel eventuele bot-commits mee.

## Filteren op ontwikkelstatus

**User story**
Als gebruiker van de catalogus wil ik projecten kunnen filteren op de
`developmentStatus` uit `publiccode.yml`, zodat ik alleen projecten zie met een
ontwikkelfase die voor mij relevant is (bijv. alleen stabiele projecten).

**Acceptatiecriteria**
- Gegeven de cataloguspagina, kan ik filteren op `developmentStatus`.
- Wanneer ik één of meer statussen selecteer, toont de lijst alleen projecten met
  die status(sen).
- Gegeven een project geen `publiccode.yml` heeft of geen `developmentStatus`
  opgeeft, toon dan het project niet in de resultaten.
- Filteren op ontwikkelstatus werkt samen met de andere filters (zoekbalk, tags) —
  ze versmallen de lijst gezamenlijk, niet los van elkaar.

**Notities / open vragen**
- De toegestane waarden voor `developmentStatus` staan gedefinieerd in de
  [publiccode.yml schema-documentatie](https://yml.publiccode.tools/v0/schema.core.html)
  (zie ook [ARCHITECTURE.md](./ARCHITECTURE.md)).
- **Besloten (PoC-implementatie):** meerkeuze via checkboxes (een project heeft
  zelf maar één status, maar je kunt op meerdere statussen tegelijk filteren —
  OF-logica binnen dit filter).
- **Besloten (PoC-implementatie):** een project zonder `developmentStatus` wordt
  niet getoond zodra minstens één status actief geselecteerd is, conform het
  acceptatiecriterium hierboven.

## Organisatie van het project

**User story**
Als gebruiker van de catalogus wil ik zien van welke organisatie een project is
en de lijst hierop kunnen filteren, zodat ik weet wie het project publiceert
en/of onderhoudt, en snel projecten van een specifieke organisatie kan vinden.

**Acceptatiecriteria**
- Gegeven de detailpagina van een project, wordt de organisatie getoond die achter
  het project staat, gebaseerd op de relevante metadata uit `publiccode.yml`
  (zie [ARCHITECTURE.md](./ARCHITECTURE.md)).
- Gegeven een project geen `publiccode.yml` heeft of geen organisatiegegevens
  opgeeft, toont de detailpagina dit duidelijk in plaats van een leeg of
  misleidend veld.
- Gegeven de cataloguspagina, kan ik filteren op organisatie.
- Wanneer ik één of meer organisaties selecteer, toont de lijst alleen projecten
  van die organisatie(s).
- Filteren op organisatie werkt samen met de andere filters (zoekbalk, tags,
  ontwikkelstatus) — ze versmallen de lijst gezamenlijk, niet los van elkaar.

**Notities / open vragen**
- In principe gebruiken we het `organisation/uri`-veld uit `publiccode.yml` voor
  deze feature. Als deze niet beschikbaar is, hardcoden we het in de data.
- **Bevindingen (PoC-implementatie):** geen van de zes gecureerde projecten had
  dit veld ingevuld in hun `publiccode.yml` — de organisatienaam is voor alle zes
  hardcoded in de backend, gebaseerd op de repository-eigenaar.
- **Besloten (PoC-implementatie):** het organisatiefilter is meerkeuze
  (checkboxes, OF-logica binnen dit filter), naar hetzelfde patroon als het
  ontwikkelstatus-filter.

## Laatste releases

**User story**
Als gebruiker van de catalogus wil ik op de detailpagina van een project de
laatste 3 releases zien, zodat ik weet welke versies recent zijn uitgebracht en
hoe actief het project releasematig is.

**Acceptatiecriteria**
- Gegeven de detailpagina van een project, worden de laatste 3 releases van het
  project getoond (bijv. versienummer/tag en releasedatum).
- Gegeven een project geen releases heeft, toont de detailpagina duidelijk dat
  er geen releases zijn, in plaats van een lege of misleidende sectie.

**Notities / open vragen**
- Bron: waarschijnlijk GitHub's releases-API
  (`/repos/{owner}/{repo}/releases`), naar hetzelfde patroon als andere
  GitHub-gebaseerde features in deze catalogus (zie
  [ARCHITECTURE.md](./ARCHITECTURE.md)) — nog niet besloten of dit tijdens de
  ingest-achtergrondtaak wordt opgehaald of live bij het renderen van de
  detailpagina.
- Nog niet besloten: wat te tonen bij een project zonder GitHub-releases maar
  wel met git-tags, en of release-notes/changelog-tekst wordt meegenomen of
  alleen versie + datum.
- Deze feature is nog niet gebouwd.

## Contactinformatie

**User story**
Als gebruiker van de catalogus wil ik de contactinformatie van een project kunnen
zien, zodat ik contact kan opnemen met de mensen die verantwoordelijk zijn voor
het project.

**Acceptatiecriteria**
- Gegeven de detailpagina van een project, wordt de contactinformatie getoond,
  gebaseerd op de relevante metadata uit `publiccode.yml`
  (zie [ARCHITECTURE.md](./ARCHITECTURE.md)).
- Gegeven een project meerdere contacten heeft, worden deze allemaal getoond.
- Gegeven een project geen `publiccode.yml` heeft of geen contactinformatie
  opgeeft, toont de detailpagina dit duidelijk in plaats van een leeg of
  misleidend veld.

**Notities / open vragen**
- **Besloten (PoC-implementatie):** alle beschikbare velden uit
  `maintenance.contacts` worden getoond (naam, e-mailadres, affiliatie) — zie de
  [publiccode.yml schema-documentatie](https://yml.publiccode.tools/v0/schema.core.html).
- Privacyoverwegingen rond het publiekelijk tonen van persoonlijke e-mailadressen
  zijn nog niet besproken. In de zes gecureerde projecten zijn dit doorgaans al
  publieke functionele/team-adressen, maar dit is niet gegarandeerd zodra de
  catalogus met meer/andere projecten wordt gevuld.

