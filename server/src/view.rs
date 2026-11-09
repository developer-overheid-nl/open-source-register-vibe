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

/// Builds the catalog URL for a set of active tags (AND-logic), e.g.
/// `/?tag=frontend&tag=rust`, or `/` for no tags.
fn tags_url(tags: &[String]) -> String {
    if tags.is_empty() {
        return "/".to_string();
    }
    let params: Vec<String> = tags
        .iter()
        .map(|t| format!("tag={}", url_encode(t)))
        .collect();
    format!("/?{}", params.join("&"))
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
struct DepRow {
    label: String,
    text: String,
}

#[derive(Serialize)]
struct DepTopRow {
    name: String,
    ecosystem_label: String,
    width: String,
    text: String,
}

#[derive(Serialize)]
struct DepsSection {
    available: bool,
    rows: Vec<DepRow>,
    has_top: bool,
    hint: String,
    top_rows: Vec<DepTopRow>,
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

fn card_view(project: &Value, active_tags: &[String]) -> CardView {
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
                href: tags_url(&toggle_tag(active_tags, tag)),
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

/// Context for the catalog page. `projects` is the full in-memory dataset;
/// `active_tags` comes from the `?tag=` query parameters.
pub fn catalog_context(data: &Value, active_tags: &[String]) -> Context {
    let all: &Vec<Value> = match get_array(data, "projects") {
        Some(p) => p,
        None => return Context::new(),
    };

    // Sidebar options are computed over the full, unfiltered dataset...
    let mut tag_counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut statuses: HashSet<&str> = HashSet::new();
    let mut orgs: HashSet<&str> = HashSet::new();
    let mut software_types: HashSet<&str> = HashSet::new();
    for project in all {
        for tag in project_tags(project) {
            *tag_counts.entry(tag).or_insert(0) += 1;
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

    // ...while the card list is pre-filtered on the active tags.
    let mut cards: Vec<CardView> = all
        .iter()
        .filter(|p| matches_tags(p, active_tags))
        .map(|p| card_view(p, active_tags))
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
                href: tags_url(&toggle_tag(active_tags, tag)),
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
                href: tags_url(&toggle_tag(active_tags, tag)),
            })
            .collect::<Vec<TagPill>>(),
    );
    context.insert(
        "statuses",
        &sorted_options(statuses),
    );
    context.insert("orgs", &sorted_options(orgs));
    context.insert("software_types", &sorted_options(software_types));
    context
}

fn sorted_options(values: HashSet<&str>) -> Vec<CheckboxOption> {
    let mut v: Vec<&str> = values.into_iter().collect();
    v.sort();
    v.into_iter()
        .map(|value| CheckboxOption {
            value: value.to_string(),
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

fn dependencies_section(deps: &Value) -> DepsSection {
    let managers = get_array(deps, "packageManagers");
    if !get_bool(deps, "available") || managers.map(|m| m.is_empty()).unwrap_or(true) {
        return DepsSection {
            available: false,
            rows: Vec::new(),
            has_top: false,
            hint: String::new(),
            top_rows: Vec::new(),
        };
    }

    let rows: Vec<DepRow> = managers
        .unwrap_or(&Vec::new())
        .iter()
        .map(|pm| {
            let count = get_u64(pm, "count");
            DepRow {
                label: ecosystem_label(get_str(pm, "ecosystem")),
                text: if count == 1 {
                    "1 dependency".to_string()
                } else {
                    format!("{} dependencies", count)
                },
            }
        })
        .collect();

    let top: &[Value] = get_array(deps, "topDependencies").map(Vec::as_slice).unwrap_or(&[]);
    if top.is_empty() {
        return DepsSection {
            available: true,
            rows,
            has_top: false,
            hint: String::new(),
            top_rows: Vec::new(),
        };
    }

    let direct_count = get_u64(deps, "directDependencyCount") as usize;
    let total = if direct_count > 0 { direct_count } else { top.len() };
    let hint = if total > top.len() {
        format!(
            "Top {} van {} directe dependencies, gesorteerd op het aantal dependencies dat ze zelf binnenhalen.",
            top.len(),
            total
        )
    } else {
        format!(
            "Alle {} directe dependencies, gesorteerd op het aantal dependencies dat ze zelf binnenhalen.",
            total
        )
    };

    let max_count = top
        .iter()
        .map(|d| get_u64(d, "transitiveCount"))
        .max()
        .unwrap_or(1)
        .max(1) as f64;

    let top_rows: Vec<DepTopRow> = top
        .iter()
        .map(|dep| {
            let count = get_u64(dep, "transitiveCount");
            let name = match get_str(dep, "version") {
                Some(version) => format!("{} {}", get_str(dep, "name").unwrap_or_default(), version),
                None => get_str(dep, "name").unwrap_or_default().to_string(),
            };
            DepTopRow {
                name,
                ecosystem_label: ecosystem_label(get_str(dep, "ecosystem")),
                width: format!("{:.2}", (count as f64 / max_count) * 100.0),
                text: if count == 1 {
                    "1 transitieve dependency".to_string()
                } else {
                    format!("{} transitieve dependencies", count)
                },
            }
        })
        .collect();

    DepsSection {
        available: true,
        rows,
        has_top: true,
        hint,
        top_rows,
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
                href: tags_url(&[tag.clone()]),
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
        dependencies: dependencies_section(&project["dependencies"]),
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
