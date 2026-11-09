//! Bouwt de template-contexten voor de pagina's. Alle berekeningen die de
//! frontend voorheen in JavaScript deed (sparkline-paden, commit-kalender,
//! taal-kleuren, filter/sorteer-sleutels, URL's voor tag-filters) gebeuren
//! hier server-side; de templates blijven dom.

use crate::icons;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};
use tera::Context;

const MONTH_LABELS: [&str; 12] = [
    "jan", "feb", "mrt", "apr", "mei", "jun", "jul", "aug", "sep", "okt", "nov", "dec",
];
const MONTH_NAMES: [&str; 12] = [
    "januari", "februari", "maart", "april", "mei", "juni", "juli", "augustus", "september",
    "oktober", "november", "december",
];
const HEAT_LEVELS: u32 = 4;

fn get_str<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(|x| x.as_str())
}

fn get_array<'a>(v: &'a Value, key: &str) -> Option<&'a Vec<Value>> {
    v.get(key).and_then(|x| x.as_array())
}

fn get_bool(v: &Value, key: &str) -> bool {
    v.get(key).and_then(|x| x.as_bool()).unwrap_or(false)
}

fn get_u64(v: &Value, key: &str) -> u64 {
    v.get(key).and_then(|x| x.as_u64()).unwrap_or(0)
}

fn project_tags(project: &Value) -> Vec<String> {
    get_array(project, "tags")
        .map(|a| a.iter().filter_map(|t| t.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default()
}

/// Alle programmeertalen van een project (keys van de `languages`-map, bytes
/// per taal) — een project telt mee voor élke taal die het gebruikt.
fn project_languages(project: &Value) -> Vec<String> {
    project
        .get("languages")
        .and_then(|l| l.as_object())
        .map(|m| m.keys().map(|k| k.to_string()).collect())
        .unwrap_or_default()
}

/// Percent-encode a path/query component (RFC 3986 unreserved characters stay).
fn url_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.as_bytes() {
        if b.is_ascii_alphanumeric() || b"-._~".contains(b) {
            out.push(*b as char);
        } else {
            out.push('%');
            out.push_str(&format!("{:02X}", b));
        }
    }
    out
}

/// Builds the catalog URL for a set of active tags (AND-logic) and languages
/// (OR-logic), e.g. `/?tag=frontend&tag=rust&language=Rust`, or `/` for no
/// filters.
fn catalog_url(tags: &[String], languages: &[String]) -> String {
    let mut params: Vec<String> = tags
        .iter()
        .map(|t| format!("tag={}", url_encode(t)))
        .collect();
    params.extend(
        languages
            .iter()
            .map(|l| format!("language={}", url_encode(l))),
    );
    if params.is_empty() {
        "/".to_string()
    } else {
        format!("/?{}", params.join("&"))
    }
}

/// Returns `active` with `tag` toggled (added/removed), preserving order.
fn toggle_tag(active: &[String], tag: &str) -> Vec<String> {
    if active.iter().any(|t| t == tag) {
        active.iter().filter(|t| t.as_str() != tag).cloned().collect()
    } else {
        let mut next = active.to_vec();
        next.push(tag.to_string());
        next
    }
}

#[derive(Serialize)]
struct TagChip {
    label: String,
    href: String,
    active: bool,
    count: Option<u64>,
}

#[derive(Serialize)]
struct TagPill {
    tag: String,
    href: String,
}

#[derive(Serialize)]
struct CheckboxOption {
    value: String,
    /// Vooraangevinkt wanneer de URL dit filter al actief heeft (`?language=…`).
    active: bool,
}

#[derive(Serialize)]
struct Sparkline {
    label: String,
    area: String,
    line: String,
    aria: String,
}

#[derive(Serialize)]
struct CardView {
    slug: String,
    detail_url: String,
    name: String,
    org: String,
    description: String,
    status: String,
    language: Option<String>,
    language_color: Option<String>,
    has_publiccode: bool,
    has_tags: bool,
    tags: Vec<TagChip>,
    sparkline: Option<Sparkline>,
    data_search: String,
    data_tags: String,
    data_status: String,
    data_org: String,
    data_software_type: String,
    data_languages: String,
    data_name: String,
    data_commits: u64,
    data_contributors: u64,
    data_dependencies: u64,
}

#[derive(Serialize)]
struct Avatar {
    login: String,
    avatar_url: String,
}

#[derive(Serialize)]
struct Panel {
    label: String,
    contributors: usize,
    prs: u64,
    avatars: Vec<Avatar>,
}

#[derive(Serialize)]
struct LangSegment {
    language: String,
    color: String,
    width: String,
    title: String,
}

#[derive(Serialize)]
struct LangTag {
    label: String,
    color: String,
    /// Linkt de catalogus, gefilterd op deze taal (`/?language=…`).
    href: String,
    title: String,
}

#[derive(Serialize)]
struct LanguageSection {
    has_any: bool,
    bar_aria: String,
    segments: Vec<LangSegment>,
    tags: Vec<LangTag>,
}

#[derive(Serialize)]
struct WeekCol {
    month_label: Option<String>,
    days: Vec<DayCell>,
}

#[derive(Serialize)]
struct DayCell {
    level: u32,
    title: String,
}

#[derive(Serialize)]
struct CommitGraph {
    available: bool,
    branch: String,
    total: u64,
    aria: String,
    weeks: Vec<WeekCol>,
}

#[derive(Serialize)]
struct DepTab {
    /// Ruwe ecosysteem-sleutel (bijv. `npm`), koppelt tab aan zijn paneel.
    ecosystem: String,
    label: String,
    /// Aantal directe dependencies van deze package manager.
    direct: u64,
    /// Alles wat de SBOM nog meer bevat voor dit ecosysteem.
    transitive: u64,
    /// Aantal unieke OSV-advisories voor dit ecosysteem (rood bolletje).
    advisories: u64,
    vulns_title: String,
    /// Geen directe dependencies bekend → niet aanklikbaar.
    disabled: bool,
    selected: bool,
    title: String,
}

#[derive(Serialize)]
struct DepTopRow {
    name: String,
    width: String,
    text: String,
}

#[derive(Serialize)]
struct DepPanel {
    ecosystem: String,
    heading: String,
    hint: String,
    rows: Vec<DepTopRow>,
}

#[derive(Serialize)]
struct DepsSection {
    available: bool,
    tabs: Vec<DepTab>,
    panels: Vec<DepPanel>,
}

#[derive(Clone, Serialize)]
struct AdvisoryLink {
    id: String,
    url: String,
}

#[derive(Clone, Serialize)]
struct VulnerablePackage {
    name: String,
    advisories: Vec<AdvisoryLink>,
}

#[derive(Serialize)]
struct RuleCheck {
    /// `pass` | `error` | `warning` | `unknown` — bepaalt de stijl van de rij.
    modifier: String,
    icon: String,
    label: String,
    status_text: String,
    vulnerable: Vec<VulnerablePackage>,
    vulnerable_summary: String,
    /// Of de uitklapbare lijst kwetsbare packages getoond moet worden.
    has_vulnerable: bool,
}

#[derive(Serialize)]
struct RulesSection {
    available: bool,
    summary: String,
    checks: Vec<RuleCheck>,
}

#[derive(Serialize)]
struct Contact {
    name: String,
    email: Option<String>,
    affiliation: Option<String>,
}

#[derive(Serialize)]
struct DetailView {
    name: String,
    org: String,
    description: String,
    repo_url: String,
    has_publiccode: bool,
    software_type_label: String,
    software_type_icon: String,
    software_type_unknown: bool,
    license: String,
    has_tags: bool,
    tags: Vec<TagChip>,
    languages: LanguageSection,
    panels: Vec<Panel>,
    commit_graph: CommitGraph,
    dependencies: DepsSection,
    rules: RulesSection,
    has_contacts: bool,
    contacts: Vec<Contact>,
}

/// Looks up a project by slug in the in-memory dataset.
pub fn find_project<'a>(data: &'a Value, slug: &str) -> Option<&'a Value> {
    get_array(data, "projects")?
        .iter()
        .find(|p| get_str(p, "slug") == Some(slug))
}

/// "Data laatst bijgewerkt" line for the footer, e.g. `9 nov 14:23`.
pub fn updated_at(data: &Value) -> Option<String> {
    let raw = get_str(data, "generatedAt")?;
    let dt = DateTime::parse_from_rfc3339(raw).ok()?.with_timezone(&Utc);
    let day: u32 = dt.format("%d").to_string().parse().unwrap_or(0);
    let month: usize = dt.format("%m").to_string().parse().unwrap_or(1);
    Some(format!(
        "{} {} {}",
        day,
        MONTH_LABELS[month - 1],
        dt.format("%H:%M")
    ))
}

fn dutch_date(iso: &str) -> String {
    let parts: Vec<&str> = iso.split('-').collect();
    if parts.len() != 3 {
        return iso.to_string();
    }
    let (Ok(year), Ok(month), Ok(day)) = (
        parts[0].parse::<i32>(),
        parts[1].parse::<usize>(),
        parts[2].parse::<u32>(),
    ) else {
        return iso.to_string();
    };
    format!("{} {} {}", day, MONTH_NAMES[month - 1], year)
}

/// Guard: commit days only exist when GitHub delivered weekly stats.
fn commit_days<'a>(ca: &'a Value) -> Option<&'a Vec<Value>> {
    get_bool(ca, "available")
        .then(|| get_array(ca, "days"))
        .flatten()
        .filter(|d| !d.is_empty())
}

/// Total commits over the project (last 12 months) — also a sort key.
fn total_commits(project: &Value) -> u64 {
    let ca = &project["commitActivity"];
    match commit_days(ca) {
        Some(days) => days.iter().map(|d| get_u64(d, "count")).sum(),
        None => 0,
    }
}

/// All-time unique contributors, union of the three activity buckets.
fn all_time_contributors(project: &Value) -> u64 {
    let mut logins: HashSet<&str> = HashSet::new();
    let activity = &project["activity"];
    for bucket in ["last6Months", "months6to12", "olderThan12Months"] {
        if let Some(contributors) = activity
            .get(bucket)
            .and_then(|b| b.get("contributors"))
            .and_then(|c| c.as_array())
        {
            for c in contributors {
                if let Some(login) = get_str(c, "login") {
                    logins.insert(login);
                }
            }
        }
    }
    logins.len() as u64
}

/// Total dependency count across all package managers, or 0 if unknown.
fn total_dependencies(project: &Value) -> u64 {
    let deps = &project["dependencies"];
    if !get_bool(deps, "available") {
        return 0;
    }
    get_array(deps, "packageManagers")
        .map(|pms| pms.iter().map(|pm| get_u64(pm, "count")).sum())
        .unwrap_or(0)
}

/// Builds the SVG path data for the small commit sparkline on project cards
/// (weekly aggregates over the last 12 months, same math as the old frontend).
fn sparkline(ca: &Value) -> Option<Sparkline> {
    let days = commit_days(ca)?;

    const WIDTH: f64 = 100.0;
    const HEIGHT: f64 = 30.0;

    let weeks: Vec<u64> = days
        .chunks(7)
        .map(|week| week.iter().map(|d| get_u64(d, "count")).sum())
        .collect();
    let total: u64 = weeks.iter().sum();
    let max_count = *weeks.iter().max().unwrap_or(&1).max(&1) as f64;

    let points: Vec<(f64, f64)> = weeks
        .iter()
        .enumerate()
        .map(|(i, &count)| {
            let x = if weeks.len() > 1 {
                (i as f64 / (weeks.len() - 1) as f64) * WIDTH
            } else {
                WIDTH / 2.0
            };
            let y = HEIGHT - (count as f64 / max_count) * (HEIGHT - 2.0) - 1.0;
            (x, y)
        })
        .collect();

    let line = points
        .iter()
        .enumerate()
        .map(|(i, (x, y))| {
            format!(
                "{}{:.1},{:.1}",
                if i == 0 { "M" } else { "L" },
                x,
                y
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    let area = format!(
        "{} L{:.1},{:.1} L{:.1},{:.1} Z",
        line, WIDTH, HEIGHT, 0.0, HEIGHT
    );

    Some(Sparkline {
        label: format!("{} commits (laatste 12 maanden)", total),
        area,
        line,
        aria: format!(
            "Wekelijkse commit-activiteit over de laatste 12 maanden, in totaal {} commits.",
            total
        ),
    })
}

fn card_view(project: &Value, active_tags: &[String], active_languages: &[String]) -> CardView {
    let slug = get_str(project, "slug").unwrap_or_default().to_string();
    let name = get_str(project, "name").unwrap_or_default().to_string();
    let org = get_str(project, "org").unwrap_or_default().to_string();
    let description = get_str(project, "description").unwrap_or_default().to_string();
    let status = get_str(project, "developmentStatus")
        .unwrap_or("onbekend")
        .to_string();
    let software_type = get_str(project, "softwareType").unwrap_or_default().to_string();
    let language = get_str(project, "primaryLanguage").map(|s| s.to_string());
    let tags = project_tags(project);
    let languages = project_languages(project);

    let search = format!(
        "{} {} {}",
        name,
        description,
        tags.join(" ")
    )
    .to_lowercase();

    CardView {
        slug: slug.clone(),
        detail_url: format!("/project/{}", url_encode(&slug)),
        name: name.clone(),
        org: org.clone(),
        description: description.clone(),
        status,
        language_color: language.as_ref().map(|l| icons::language_color(l)),
        language,
        has_publiccode: get_bool(project, "hasPubliccode"),
        has_tags: !tags.is_empty(),
        tags: tags
            .iter()
            .map(|tag| TagChip {
                label: tag.clone(),
                href: catalog_url(&toggle_tag(active_tags, tag), active_languages),
                active: false,
                count: None,
            })
            .collect(),
        sparkline: sparkline(&project["commitActivity"]),
        data_search: search,
        data_tags: serde_json::to_string(&tags).unwrap_or_else(|_| "[]".to_string()),
        data_status: get_str(project, "developmentStatus").unwrap_or_default().to_string(),
        data_org: org,
        data_software_type: software_type,
        data_languages: serde_json::to_string(&languages).unwrap_or_else(|_| "[]".to_string()),
        data_name: name.to_lowercase(),
        data_commits: total_commits(project),
        data_contributors: all_time_contributors(project),
        data_dependencies: total_dependencies(project),
    }
}

/// Matches the server-side `?tag=` pre-filter: a project must have *all*
/// active tags (AND-logic, see FEATURES.md).
fn matches_tags(project: &Value, active_tags: &[String]) -> bool {
    let tags: HashSet<String> = project_tags(project).into_iter().collect();
    active_tags.iter().all(|t| tags.contains(t))
}

/// Matches the server-side `?language=` pre-filter: a project counts for
/// every language it uses, so multiple active languages work as OR (see
/// FEATURES.md).
fn matches_languages(project: &Value, active_languages: &[String]) -> bool {
    if active_languages.is_empty() {
        return true;
    }
    let languages = project_languages(project);
    let known: HashSet<&str> = languages.iter().map(|s| s.as_str()).collect();
    active_languages
        .iter()
        .any(|l| known.contains(l.as_str()))
}

/// Context for the catalog page. `projects` is the full in-memory dataset;
/// `active_tags`/`active_languages` come from the `?tag=`/`?language=` query
/// parameters (server-side pre-filters that also work without JavaScript).
pub fn catalog_context(data: &Value, active_tags: &[String], active_languages: &[String]) -> Context {
    let all: &Vec<Value> = match get_array(data, "projects") {
        Some(p) => p,
        None => return Context::new(),
    };

    // Sidebar options are computed over the full, unfiltered dataset...
    let mut tag_counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut language_counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut statuses: HashSet<&str> = HashSet::new();
    let mut orgs: HashSet<&str> = HashSet::new();
    let mut software_types: HashSet<&str> = HashSet::new();
    for project in all {
        for tag in project_tags(project) {
            *tag_counts.entry(tag).or_insert(0) += 1;
        }
        for language in project_languages(project) {
            *language_counts.entry(language).or_insert(0) += 1;
        }
        if let Some(s) = get_str(project, "developmentStatus") {
            statuses.insert(s);
        }
        if let Some(o) = get_str(project, "org") {
            orgs.insert(o);
        }
        if let Some(st) = get_str(project, "softwareType") {
            software_types.insert(st);
        }
    }

    let mut tags_sorted: Vec<(String, u64)> = tag_counts.into_iter().collect();
    tags_sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    // Language options sorted on how many projects use the language (most
    // used first, then alphabetically) — see FEATURES.md.
    let mut languages_sorted: Vec<(String, u64)> = language_counts.into_iter().collect();
    languages_sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    // ...while the card list is pre-filtered on the active tags and languages.
    let mut cards: Vec<CardView> = all
        .iter()
        .filter(|p| matches_tags(p, active_tags) && matches_languages(p, active_languages))
        .map(|p| card_view(p, active_tags, active_languages))
        .collect();
    cards.sort_by(|a, b| a.data_name.cmp(&b.data_name));

    let mut context = Context::new();
    context.insert("updated_at", &updated_at(data));
    context.insert("cards_empty", &cards.is_empty());
    context.insert("cards", &cards);
    context.insert(
        "tags",
        &tags_sorted
            .iter()
            .map(|(tag, count)| TagChip {
                label: tag.clone(),
                href: catalog_url(&toggle_tag(active_tags, tag), active_languages),
                active: active_tags.iter().any(|t| t == tag),
                count: Some(*count),
            })
            .collect::<Vec<TagChip>>(),
    );
    context.insert(
        "active_tag_pills",
        &active_tags
            .iter()
            .map(|tag| TagPill {
                tag: tag.clone(),
                href: catalog_url(&toggle_tag(active_tags, tag), active_languages),
            })
            .collect::<Vec<TagPill>>(),
    );
    context.insert(
        "statuses",
        &sorted_options(statuses),
    );
    context.insert("orgs", &sorted_options(orgs));
    context.insert("software_types", &sorted_options(software_types));
    context.insert(
        "languages",
        &languages_sorted
            .iter()
            .map(|(language, _count)| CheckboxOption {
                value: language.clone(),
                active: active_languages.iter().any(|l| l == language),
            })
            .collect::<Vec<CheckboxOption>>(),
    );
    context
}

fn sorted_options(values: HashSet<&str>) -> Vec<CheckboxOption> {
    let mut v: Vec<&str> = values.into_iter().collect();
    v.sort();
    v.into_iter()
        .map(|value| CheckboxOption {
            value: value.to_string(),
            active: false,
        })
        .collect()
}

fn ecosystem_label(ecosystem: Option<&str>) -> String {
    match ecosystem {
        None => return "onbekend".to_string(),
        Some(e) => match e {
            "npm" => "npm",
            "pypi" => "pip (PyPI)",
            "cargo" => "Cargo",
            "maven" => "Maven",
            "composer" => "Composer",
            "nuget" => "NuGet",
            "gem" => "RubyGems",
            "golang" => "Go modules",
            "githubactions" => "GitHub Actions",
            "github" => "GitHub (overig)",
            other => other,
        }
        .to_string(),
    }
}

fn language_section(languages: &Value) -> LanguageSection {
    let object = match languages.as_object() {
        Some(o) if !o.is_empty() => o,
        _ => {
            return LanguageSection {
                has_any: false,
                bar_aria: String::new(),
                segments: Vec::new(),
                tags: Vec::new(),
            }
        }
    };

    let total: u64 = object.values().map(|v| v.as_u64().unwrap_or(0)).sum();
    let mut entries: Vec<(&String, u64)> = object
        .iter()
        .map(|(k, v)| (k, v.as_u64().unwrap_or(0)))
        .collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));

    let percent = |bytes: u64| {
        if total > 0 {
            (bytes as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    };

    LanguageSection {
        has_any: true,
        bar_aria: entries
            .iter()
            .map(|(language, bytes)| format!("{} {:.0}%", language, percent(*bytes)))
            .collect::<Vec<_>>()
            .join(", "),
        segments: entries
            .iter()
            .map(|(language, bytes)| LangSegment {
                language: language.to_string(),
                color: icons::language_color(language),
                width: format!("{:.2}", percent(*bytes)),
                title: format!("{} ({:.0}%)", language, percent(*bytes)),
            })
            .collect(),
        tags: entries
            .iter()
            .map(|(language, bytes)| LangTag {
                label: format!("{} ({:.0}%)", language, percent(*bytes)),
                color: icons::language_color(language),
                href: format!("/?language={}", url_encode(language)),
                title: format!("Toon alle projecten in {language}"),
            })
            .collect(),
    }
}

fn panel(bucket: &Value, label: &str) -> Panel {
    let contributors: Vec<Avatar> = bucket
        .get("contributors")
        .and_then(|c| c.as_array())
        .map(|cs| {
            cs.iter()
                .filter_map(|c| {
                    Some(Avatar {
                        login: get_str(c, "login")?.to_string(),
                        avatar_url: get_str(c, "avatarUrl")?.to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    Panel {
        label: label.to_string(),
        contributors: contributors.len(),
        prs: get_u64(bucket, "prsClosed"),
        avatars: contributors,
    }
}

fn commit_graph(ca: &Value) -> CommitGraph {
    let branch = get_str(ca, "branch").unwrap_or_default().to_string();
    let Some(days) = commit_days(ca) else {
        return CommitGraph {
            available: false,
            branch,
            total: 0,
            aria: String::new(),
            weeks: Vec::new(),
        };
    };

    let total: u64 = days.iter().map(|d| get_u64(d, "count")).sum();
    let max_count = days
        .iter()
        .map(|d| get_u64(d, "count"))
        .max()
        .unwrap_or(1)
        .max(1) as f64;

    let mut weeks: Vec<WeekCol> = Vec::new();
    let mut last_month: Option<usize> = None;
    for chunk in days.chunks(7) {
        let first_date = get_str(&chunk[0], "date").unwrap_or_default();
        let month: usize = first_date
            .get(5..7)
            .and_then(|m| m.parse().ok())
            .unwrap_or(0);

        let month_label = if month != 0 && Some(month) != last_month {
            last_month = Some(month);
            Some(MONTH_LABELS[month - 1].to_string())
        } else {
            None
        };

        let cells = chunk
            .iter()
            .map(|day| {
                let count = get_u64(day, "count");
                let level = if count == 0 {
                    0
                } else {
                    ((count as f64 / max_count) * HEAT_LEVELS as f64)
                        .ceil()
                        .min(HEAT_LEVELS as f64) as u32
                };
                DayCell {
                    level,
                    title: format!(
                        "{} commit{} op {}",
                        count,
                        if count == 1 { "" } else { "s" },
                        dutch_date(get_str(day, "date").unwrap_or_default())
                    ),
                }
            })
            .collect();
        weeks.push(WeekCol {
            month_label,
            days: cells,
        });
    }

    CommitGraph {
        available: true,
        branch: branch.clone(),
        total,
        aria: format!(
            "Commit-kalender over de laatste 12 maanden op branch {}, in totaal {} commits.",
            branch, total
        ),
        weeks,
    }
}

/// Bouwt de dependency-sectie: één tab per package manager ("N direct +
/// M transitief", kwetsbare ecosystemen vooraan met een rood bolletje met het
/// aantal unieke OSV-advisories) met daaronder, per package manager, de
/// directe dependencies die de meeste transitieve dependencies binnenhalen —
/// de ingest levert per ecosysteem al de top 20 (zie FEATURES.md).
fn dependencies_section(deps: &Value, compliance: &Value) -> DepsSection {
    let empty = DepsSection {
        available: false,
        tabs: Vec::new(),
        panels: Vec::new(),
    };
    let managers = match get_array(deps, "packageManagers") {
        Some(m) if !m.is_empty() && get_bool(deps, "available") => m,
        _ => return empty,
    };
    let top: &[Value] = get_array(deps, "topDependencies")
        .map(Vec::as_slice)
        .unwrap_or(&[]);

    // Unieke OSV-advisories per ecosysteem, voor de rode bolletjes op de tabs.
    let mut advisory_sets: BTreeMap<&str, HashSet<&str>> = BTreeMap::new();
    if let Some(packages) = get_array(compliance, "vulnerablePackages") {
        for pkg in packages {
            let Some(ecosystem) = get_str(pkg, "ecosystem") else {
                continue;
            };
            if let Some(ids) = get_array(pkg, "advisories") {
                for id in ids.iter().filter_map(|a| a.as_str()) {
                    advisory_sets.entry(ecosystem).or_default().insert(id);
                }
            }
        }
    }

    let mut tabs: Vec<DepTab> = managers
        .iter()
        .map(|pm| {
            let ecosystem = get_str(pm, "ecosystem").unwrap_or_default().to_string();
            let count = get_u64(pm, "count");
            let direct = get_u64(pm, "directCount");
            let advisories = advisory_sets
                .get(ecosystem.as_str())
                .map_or(0, |s| s.len() as u64);
            let has_top = top
                .iter()
                .any(|d| get_str(d, "ecosystem") == Some(ecosystem.as_str()));
            let mut title = format!("{count} packages in totaal");
            if !has_top {
                title.push_str(" — Geen directe dependencies bekend voor deze package manager");
            }
            DepTab {
                ecosystem: ecosystem.clone(),
                label: ecosystem_label(Some(&ecosystem)),
                direct,
                transitive: count.saturating_sub(direct),
                advisories,
                vulns_title: if advisories == 1 {
                    "1 bekende kwetsbaarheid".to_string()
                } else {
                    format!("{advisories} bekende kwetsbaarheden")
                },
                disabled: !has_top,
                selected: false,
                title,
            }
        })
        .collect();
    // Ecosystemen met bekende kwetsbaarheden vooraan (meeste eerst).
    tabs.sort_by(|a, b| {
        b.advisories
            .cmp(&a.advisories)
            .then_with(|| a.ecosystem.cmp(&b.ecosystem))
    });
    // De eerste package manager met directe dependencies is voorgeselecteerd.
    if let Some(first) = tabs.iter().position(|t| !t.disabled) {
        tabs[first].selected = true;
    }

    let mut panels: Vec<DepPanel> = Vec::new();
    for tab in tabs.iter().filter(|t| !t.disabled) {
        let mut rows: Vec<(String, u64)> = top
            .iter()
            .filter(|d| get_str(d, "ecosystem") == Some(tab.ecosystem.as_str()))
            .map(|d| {
                let name = match get_str(d, "version") {
                    Some(version) => {
                        format!("{} {}", get_str(d, "name").unwrap_or_default(), version)
                    }
                    None => get_str(d, "name").unwrap_or_default().to_string(),
                };
                (name, get_u64(d, "transitiveCount"))
            })
            .collect();
        rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        let shown = rows.len();
        let total = if tab.direct > 0 {
            tab.direct as usize
        } else {
            shown
        };
        let hint = if total > shown {
            format!("Top {shown} van {total} directe dependencies, gesorteerd op het aantal dependencies dat ze zelf binnenhalen.")
        } else {
            format!("Alle {total} directe dependencies, gesorteerd op het aantal dependencies dat ze zelf binnenhalen.")
        };

        let max_count = rows.iter().map(|(_, c)| *c).max().unwrap_or(1).max(1) as f64;
        panels.push(DepPanel {
            ecosystem: tab.ecosystem.clone(),
            heading: format!(
                "Directe {}-dependencies met de meeste transitieve dependencies",
                tab.label
            ),
            hint,
            rows: rows
                .iter()
                .map(|(name, count)| DepTopRow {
                    name: name.clone(),
                    width: format!("{:.2}", (*count as f64 / max_count) * 100.0),
                    text: if *count == 1 {
                        "1 transitieve dependency".to_string()
                    } else {
                        format!("{count} transitieve dependencies")
                    },
                })
                .collect(),
        });
    }

    DepsSection {
        available: true,
        tabs,
        panels,
    }
}

/// Bouwt de repositoryregels-sectie uit de `compliance`-data van de ingest:
/// een samenvattingsregel ("N van M regels voldaan · X fouten · Y
/// waarschuwingen") gevolgd door één rij per regel, overtredingen bovenaan —
/// fouten vóór waarschuwingen (zie FEATURES.md). De kwetsbaarheidsregel
/// vouwt open naar de kwetsbare packages met hun OSV-advisorylinks.
fn rules_section(compliance: &Value) -> RulesSection {
    let empty = RulesSection {
        available: false,
        summary: String::new(),
        checks: Vec::new(),
    };
    let checks_json = match get_array(compliance, "checks") {
        Some(c) if !c.is_empty() && get_bool(compliance, "available") => c,
        _ => return empty,
    };

    let vulnerable: Vec<VulnerablePackage> = get_array(compliance, "vulnerablePackages")
        .map(|packages| {
            packages
                .iter()
                .map(|pkg| {
                    let name = match get_str(pkg, "version") {
                        Some(version) => {
                            format!("{} {}", get_str(pkg, "name").unwrap_or_default(), version)
                        }
                        None => get_str(pkg, "name").unwrap_or_default().to_string(),
                    };
                    VulnerablePackage {
                        name,
                        advisories: get_array(pkg, "advisories")
                            .map(|ids| {
                                ids.iter()
                                    .filter_map(|id| id.as_str())
                                    .map(|id| AdvisoryLink {
                                        id: id.to_string(),
                                        url: format!(
                                            "https://osv.dev/vulnerability/{}",
                                            url_encode(id)
                                        ),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default(),
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let mut checks: Vec<RuleCheck> = checks_json
        .iter()
        .map(|check| {
            let severity = get_str(check, "severity").unwrap_or("warning");
            let passed = check.get("passed").and_then(|p| p.as_bool());
            let (modifier, icon, status_text) = match passed {
                Some(true) => ("pass", "✓", "Voldaan"),
                None => ("unknown", "?", "Onbekend"),
                Some(false) if severity == "error" => ("error", "✕", "Fout"),
                Some(false) => ("warning", "!", "Waarschuwing"),
            };
            let is_vulnerability = get_str(check, "id") == Some("VULNERABILITY_IN_DEPENDENCY");
            let vulnerable = if is_vulnerability {
                vulnerable.clone()
            } else {
                Vec::new()
            };
            let vulnerable_summary = if is_vulnerability {
                if vulnerable.len() == 1 {
                    "1 kwetsbaar package".to_string()
                } else {
                    format!("{} kwetsbare packages", vulnerable.len())
                }
            } else {
                String::new()
            };
            RuleCheck {
                modifier: modifier.to_string(),
                icon: icon.to_string(),
                label: get_str(check, "label").unwrap_or_default().to_string(),
                status_text: status_text.to_string(),
                has_vulnerable: is_vulnerability && !vulnerable.is_empty(),
                vulnerable,
                vulnerable_summary,
            }
        })
        .collect();

    // Overtredingen bovenaan: fouten vóór waarschuwingen, dan onbekend, dan
    // voldaan.
    fn rank(check: &RuleCheck) -> u8 {
        match check.modifier.as_str() {
            "error" => 0,
            "warning" => 1,
            "unknown" => 2,
            _ => 3,
        }
    }
    checks.sort_by_key(rank);

    let passed_count = checks.iter().filter(|c| c.modifier == "pass").count();
    let errors = checks.iter().filter(|c| c.modifier == "error").count();
    let warnings = checks.iter().filter(|c| c.modifier == "warning").count();
    let mut summary = format!("{} van {} regels voldaan", passed_count, checks.len());
    let mut parts = Vec::new();
    if errors > 0 {
        parts.push(if errors == 1 {
            "1 fout".to_string()
        } else {
            format!("{errors} fouten")
        });
    }
    if warnings > 0 {
        parts.push(if warnings == 1 {
            "1 waarschuwing".to_string()
        } else {
            format!("{warnings} waarschuwingen")
        });
    }
    if !parts.is_empty() {
        summary.push_str(" · ");
        summary.push_str(&parts.join(" · "));
    }

    RulesSection {
        available: true,
        summary,
        checks,
    }
}

/// Context for a project detail page. Tag chips link back to the catalog
/// filtered on that tag (`/?tag=…`).
pub fn project_context(data: &Value, project: &Value) -> Context {
    let software_type = get_str(project, "softwareType");
    let description = get_str(project, "longDescription")
        .or_else(|| get_str(project, "description"))
        .unwrap_or_default()
        .to_string();

    let activity = &project["activity"];
    let detail = DetailView {
        name: get_str(project, "name").unwrap_or_default().to_string(),
        org: get_str(project, "org").unwrap_or("onbekend").to_string(),
        description,
        repo_url: get_str(project, "repoUrl").unwrap_or_default().to_string(),
        has_publiccode: get_bool(project, "hasPubliccode"),
        software_type_label: software_type.unwrap_or("onbekend").to_string(),
        software_type_icon: icons::software_type_icon(software_type).to_string(),
        software_type_unknown: software_type.is_none(),
        license: get_str(project, "license").unwrap_or("onbekend").to_string(),
        has_tags: !project_tags(project).is_empty(),
        tags: project_tags(project)
            .iter()
            .map(|tag| TagChip {
                label: tag.clone(),
                href: catalog_url(&[tag.clone()], &[]),
                active: false,
                count: None,
            })
            .collect(),
        languages: language_section(&project["languages"]),
        panels: vec![
            panel(&activity["last6Months"], "Laatste 6 maanden"),
            panel(&activity["months6to12"], "6 tot 12 maanden geleden"),
            panel(&activity["olderThan12Months"], "Ouder dan 12 maanden"),
        ],
        commit_graph: commit_graph(&project["commitActivity"]),
        dependencies: dependencies_section(&project["dependencies"], &project["compliance"]),
        rules: rules_section(&project["compliance"]),
        has_contacts: !get_array(project, "contacts").map(|c| c.is_empty()).unwrap_or(true),
        contacts: get_array(project, "contacts")
            .map(|cs| {
                cs.iter()
                    .map(|c| Contact {
                        name: get_str(c, "name").unwrap_or("Onbekend").to_string(),
                        email: get_str(c, "email").map(|s| s.to_string()),
                        affiliation: get_str(c, "affiliation").map(|s| s.to_string()),
                    })
                    .collect()
            })
            .unwrap_or_default(),
    };

    let mut context = Context::new();
    context.insert("updated_at", &updated_at(data));
    context.insert("title", &format!("{} — Open Source Catalogus", detail.name));
    context.insert("p", &detail);
    context
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tera::Tera;

    fn tera() -> Tera {
        Tera::new(&format!(
            "{}/../templates/**/*.html",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("templates konden niet geladen worden")
    }

    fn project(slug: &str, languages: Value) -> Value {
        json!({
            "slug": slug,
            "name": slug,
            "org": "Voorbeeldorganisatie",
            "repoUrl": "https://github.com/voorbeeld/org",
            "description": "Beschrijving",
            "tags": ["demo"],
            "hasPubliccode": true,
            "softwareType": "frontend",
            "developmentStatus": "beta",
            "license": "EUPL-1.2",
            "primaryLanguage": "Rust",
            "languages": languages,
            "contacts": [],
            "activity": {
                "last6Months": { "contributors": [], "prsClosed": 0 },
                "months6to12": { "contributors": [], "prsClosed": 0 },
                "olderThan12Months": { "contributors": [], "prsClosed": 0 }
            },
            "commitActivity": { "available": false },
            "dependencies": {
                "available": true,
                "packageManagers": [
                    { "ecosystem": "npm", "count": 30, "directCount": 12 },
                    { "ecosystem": "cargo", "count": 8, "directCount": 8 }
                ],
                "topDependencies": [
                    { "name": "left-pad", "version": "1.0.0", "ecosystem": "npm", "transitiveCount": 5 },
                    { "name": "serde", "version": "1.0.210", "ecosystem": "cargo", "transitiveCount": 2 }
                ],
                "directDependencyCount": 20
            },
            "compliance": {
                "available": true,
                "checks": [
                    { "id": "VULNERABILITY_IN_DEPENDENCY", "label": "Geen bekende kwetsbaarheden in dependencies", "severity": "warning", "passed": false },
                    { "id": "MISSING_README_FILE", "label": "README.md aanwezig", "severity": "error", "passed": true },
                    { "id": "MISSING_LICENSE_FILE", "label": "LICENSE aanwezig", "severity": "error", "passed": false }
                ],
                "vulnerablePackages": [
                    { "name": "left-pad", "version": "1.0.0", "ecosystem": "npm", "advisories": ["GHSA-1234"] }
                ]
            }
        })
    }

    fn data() -> Value {
        json!({
            "generatedAt": "2026-11-09T10:00:00Z",
            "projects": [
                project("demo", json!({ "Rust": 900, "Shell": 100 })),
                project("pydemo", json!({ "Python": 500 }))
            ]
        })
    }

    fn render(template: &str, context: &Context) -> String {
        tera().render(template, context).expect("renderfout")
    }

    #[test]
    fn catalog_rendert_taalfilter_en_filtert_server_side() {
        let data = data();
        let context = catalog_context(&data, &[], &["Rust".to_string()]);
        let html = render("index.html", &context);

        // Alleen het Rust-project is geresulteerd en de taal staat aangevinkt.
        assert!(html.contains("demo"));
        assert!(!html.contains("pydemo"));
        assert!(html.contains("value=\"Rust\""));
        assert!(html.contains("checked"));
        // Kaarten dragen hun talen als data-attribuut voor de JS-filterlaag.
        assert!(html.contains("data-languages="));
        // Zoekveld en toon-meer-knop zijn server-side aanwezig (verborgen).
        assert!(html.contains("checkbox-filter__search"));
        assert!(html.contains("checkbox-filter__no-matches"));
    }

    #[test]
    fn project_renderst_dependencytabs_en_repositoryregels() {
        let data = data();
        let project = find_project(&data, "demo").expect("project");
        let context = project_context(&data, project);
        let html = render("project.html", &context);

        // Tabs: "12 direct + 18 transitief", npm vooraan (kwetsbaar) en
        // voorgeselecteerd, kwetsbaarheidsbolletje met het aantal advisories.
        assert!(html.contains("12 direct"));
        assert!(html.contains("+ 18 transitief"));
        assert!(html.contains("aria-pressed=\"true\""));
        assert!(html.contains("dependency-row__vulns"));
        assert!(html.contains("1 bekende kwetsbaarheid"));
        // Beide ecosystemen hebben directe dependencies, dus twee panelen.
        assert_eq!(html.matches("dependency-list__top\" data-ecosystem").count(), 2);

        // Repositoryregels: samenvatting en rijen, fouten vóór waarschuwingen.
        assert!(html.contains("1 van 3 regels voldaan"));
        assert!(html.contains("1 fout"));
        assert!(html.contains("1 waarschuwing"));
        assert!(html.contains("repository-rule--warning"));
        assert!(html.contains("repository-rule--error"));
        // Kwetsbaar package met OSV-link onder de kwetsbaarheidsregel. Tera
        // escapet `/` in attributen als `&#x2F;`; de browser decodeert dat.
        assert!(html.contains("left-pad 1.0.0"));
        assert!(html.contains("osv.dev&#x2F;vulnerability&#x2F;GHSA-1234"));
        assert!(html.contains(">GHSA-1234</a"));

        // Taal-labels linken naar de gefilterde catalogus.
        assert!(html.contains("?language=Rust"));
        assert!(html.contains("Toon alle projecten in Rust"));
    }
}

