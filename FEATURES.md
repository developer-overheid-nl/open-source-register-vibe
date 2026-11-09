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

**Notities / open vragen**
- **Besloten (PoC-implementatie):** badge met tekst "⚠ metadata onvolledig" op de
  kaart; de detailpagina herhaalt de waarschuwing met een volledige uitleg-zin.

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

**Notities / open vragen**
- **Besloten (PoC-implementatie):** bron is GitHub's languages-API (bytes per
  taal); detailpagina toont alle talen met hun percentage van de codebase.
- **Besloten:** sorteren van de cataloguslijst op programmeertaal is uit scope
  voor deze PoC. Op lijstniveau wordt alleen de primaire taal getoond, in de
  vaste (alfabetische) volgorde van de lijst.

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

