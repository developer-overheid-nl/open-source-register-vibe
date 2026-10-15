use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::process::Command;

/// (owner, repo, hardcoded organisation name — publiccode.yml files in this
/// curated set don't carry an `organisation` field, see ARCHITECTURE.md).
const REPOS: &[(&str, &str, &str)] = &[
    ("kiesraad", "abacus", "Kiesraad"),
    ("Amsterdam", "design-system", "Gemeente Amsterdam"),
    (
        "SSC-ICT-Innovatie",
        "nl-kat-coordination",
        "SSC-ICT Innovatie",
    ),
    (
        "MinBZK",
        "amt",
        "Ministerie van Binnenlandse Zaken en Koninkrijksrelaties",
    ),
    (
        "nl-design-system",
        "rijkshuisstijl-community",
        "NL Design System (community)",
    ),
    (
        "developer-overheid-nl",
        "don-site",
        "Developer overheid NL",
    ),
    (
        "developer-overheid-nl",
        "skills-marketplace",
        "Developer overheid NL",
    ),
    ("ICTU", "quality-time", "ICTU"),
    (
        "MinBZK",
        "MijnOverheidZakelijk",
        "Ministerie van Binnenlandse Zaken en Koninkrijksrelaties",
    ),
    (
        "minvws",
        "gfmodules-nationale-verwijsindex",
        "Ministerie van Volksgezondheid, Welzijn en Sport",
    ),
    (
        "developer-overheid-nl",
        "don-infra",
        "Developer overheid NL",
    ),
    (
        "MinBZK",
        "regelrecht",
        "Ministerie van Binnenlandse Zaken en Koninkrijksrelaties",
    ),
    (
        "MinBZK",
        "nl-wallet",
        "Ministerie van Binnenlandse Zaken en Koninkrijksrelaties",
    ),
    ("kadaster", "generieke-geo-componenten", "Kadaster"),
    ("Geonovum", "ogc-checker", "Geonovum"),
    ("Signalen", "frontend", "Signalen"),
    ("RIVM-bioinformatics", "Pacini-typing", "RIVM"),
    (
        "Sudwest-Fryslan",
        "OpenZaakBrug",
        "Gemeente Súdwest-Fryslân",
    ),
    ("belastingdienst", "opr-paas", "Belastingdienst"),
    ("belastingdienst", "opr-paas-cli", "Belastingdienst"),
    (
        "nl-design-system",
        "candidate",
        "NL Design System (community)",
    ),
    (
        "nl-design-system",
        "documentatie",
        "NL Design System (community)",
    ),
    ("wigo4it", "nl-app", "Wigo4it"),
    ("wigo4it", "techradar", "Wigo4it"),
    ("wigo4it", "kwartaalmeeting", "Wigo4it"),
    ("wigo4it", "knowledge-sharing-jp", "Wigo4it"),
    ("wigo4it", "kerstbingo", "Wigo4it"),
    ("wigo4it", "duurzaamheidsradar", "Wigo4it"),
    ("wigo4it", "DevStream", "Wigo4it"),
    ("wigo4it", "github-action-azure-devops-pipeline-runner", "Wigo4it"),
    ("wigo4it", "module-tf-azure-aks", "Wigo4it"),
    ("wigo4it", "platform-infra-talos-endpoints", "Wigo4it"),
    ("wigo4it", "wigo4it-code-conventions-mcp", "Wigo4it"),
    ("wigo4it", "wigo4it-multitenant", "Wigo4it"),
    ("wigo4it", "gitops-core-infra", "Wigo4it"),
    ("wigo4it", "POC-Socrates-PDF-Vergelijker", "Wigo4it"),
    ("wigo4it", "wigo4it-brandbook", "Wigo4it"),
    ("wigo4it", "renovate-config", "Wigo4it"),
    ("wigo4it", "azure-jwks-publisher", "Wigo4it"),
    ("wigo4it", "modules-tf-leafcloud", "Wigo4it"),
    ("wigo4it", "w4u-api-test-client", "Wigo4it"),
    ("wigo4it", "jwks-publisher-w4", "Wigo4it"),
    ("kadaster", "imbag", "Kadaster"),
    ("kadaster", "imkad", "Kadaster"),
    ("kadaster", "imbrt", "Kadaster"),
    ("kadaster", "top10nl", "Kadaster"),
    ("kadaster", "top50nl", "Kadaster"),
    ("kadaster", "top250nl", "Kadaster"),
    ("kadaster", "brt-algemeen", "Kadaster"),
    ("kadaster", "klic-win", "Kadaster"),
    ("kadaster", "top100nl", "Kadaster"),
    ("kadaster", "top500nl", "Kadaster"),
    ("kadaster", "vgl", "Kadaster"),
    ("kadaster", "top1000nl", "Kadaster"),
    ("kadaster", "metagegevens-voorbeelden", "Kadaster"),
    ("kadaster", "BRK-bevragen", "Kadaster"),
    ("kadaster", "bag-leveranciers", "Kadaster"),
    ("kadaster", "WOZ-bevragen", "Kadaster"),
    ("kadaster", "eurogeographics-erm-validator", "Kadaster"),
    ("kadaster", "ladm-composer", "Kadaster"),
];

#[derive(Debug, Deserialize, Default)]
struct PubliccodeYml {
    #[serde(default)]
    name: Option<String>,
    #[serde(rename = "softwareType", default)]
    software_type: Option<String>,
    #[serde(rename = "developmentStatus", default)]
    development_status: Option<String>,
    #[serde(default)]
    categories: Vec<String>,
    #[serde(default)]
    description: BTreeMap<String, PubliccodeDescription>,
    #[serde(default)]
    legal: Option<PubliccodeLegal>,
    #[serde(default)]
    maintenance: Option<PubliccodeMaintenance>,
}

#[derive(Debug, Deserialize, Default)]
struct PubliccodeDescription {
    #[serde(rename = "shortDescription", default)]
    short_description: Option<String>,
    #[serde(rename = "longDescription", default)]
    long_description: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct PubliccodeLegal {
    #[serde(default)]
    license: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct PubliccodeMaintenance {
    #[serde(default)]
    contacts: Vec<PubliccodeContact>,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct PubliccodeContact {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    affiliation: Option<String>,
}

fn github_token() -> Option<String> {
    if let Ok(t) = std::env::var("GITHUB_TOKEN") {
        let t = t.trim().to_string();
        if !t.is_empty() {
            return Some(t);
        }
    }
    let out = Command::new("gh").args(["auth", "token"]).output().ok()?;
    if out.status.success() {
        let t = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !t.is_empty() {
            return Some(t);
        }
    }
    None
}

/// GETs a GitHub API URL. Some orgs (e.g. nl-design-system) reject classic
/// personal access tokens outright, so a 401/403 is retried unauthenticated
/// before giving up.
fn api_get(
    client: &reqwest::blocking::Client,
    token: &Option<String>,
    url: &str,
) -> Result<(u16, Value), String> {
    let send = |with_auth: bool| -> Result<reqwest::blocking::Response, String> {
        let mut req = client
            .get(url)
            .header("User-Agent", "open-source-register-vibe-ingest")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");
        if with_auth {
            if let Some(t) = token {
                req = req.header("Authorization", format!("Bearer {t}"));
            }
        }
        req.send().map_err(|e| e.to_string())
    };

    let mut resp = send(true)?;
    if matches!(resp.status().as_u16(), 401 | 403) && token.is_some() {
        resp = send(false)?;
    }
    let status = resp.status().as_u16();
    if status == 404 {
        return Ok((404, Value::Null));
    }
    let body: Value = resp.json().unwrap_or(Value::Null);
    Ok((status, body))
}

fn fetch_publiccode(client: &reqwest::blocking::Client, owner: &str, repo: &str, branch: &str) -> Option<PubliccodeYml> {
    let url = format!("https://raw.githubusercontent.com/{owner}/{repo}/{branch}/publiccode.yml");
    let resp = client
        .get(&url)
        .header("User-Agent", "open-source-register-vibe-ingest")
        .send()
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let text = resp.text().ok()?;
    serde_yaml::from_str(&text).ok()
}

struct ActivityBucket {
    /// login -> avatar URL, deduplicated per contributor.
    contributors: BTreeMap<String, String>,
    prs_closed: u32,
}

impl ActivityBucket {
    fn new() -> Self {
        ActivityBucket {
            contributors: BTreeMap::new(),
            prs_closed: 0,
        }
    }
    fn to_json(&self) -> Value {
        let contributors: Vec<Value> = self
            .contributors
            .iter()
            .map(|(login, avatar_url)| json!({ "login": login, "avatarUrl": avatar_url }))
            .collect();
        json!({
            "count": self.contributors.len(),
            "contributors": contributors,
            "prsClosed": self.prs_closed,
        })
    }
}

fn fetch_activity(client: &reqwest::blocking::Client, token: &Option<String>, owner: &str, repo: &str) -> Value {
    let now = Utc::now();
    let six_months_ago = now - Duration::days(182);
    let twelve_months_ago = now - Duration::days(365);

    let mut last6 = ActivityBucket::new();
    let mut m6to12 = ActivityBucket::new();
    let mut older = ActivityBucket::new();

    for page in 1..=5u32 {
        let url = format!(
            "https://api.github.com/repos/{owner}/{repo}/pulls?state=closed&per_page=100&page={page}&sort=updated&direction=desc"
        );
        let (status, body) = match api_get(client, token, &url) {
            Ok(v) => v,
            Err(_) => break,
        };
        if status != 200 {
            break;
        }
        let items = match body.as_array() {
            Some(a) if !a.is_empty() => a,
            _ => break,
        };
        for pr in items {
            let closed_at = pr.get("closed_at").and_then(|v| v.as_str());
            let user = pr.get("user");
            let author = user.and_then(|u| u.get("login")).and_then(|l| l.as_str());
            let avatar_url = user
                .and_then(|u| u.get("avatar_url"))
                .and_then(|a| a.as_str());
            let is_bot = user.and_then(|u| u.get("type")).and_then(|t| t.as_str()) == Some("Bot");
            let (Some(closed_at), Some(author), Some(avatar_url)) = (closed_at, author, avatar_url)
            else {
                continue;
            };
            if is_bot {
                continue;
            }
            let Ok(closed_at) = DateTime::parse_from_rfc3339(closed_at) else {
                continue;
            };
            let closed_at = closed_at.with_timezone(&Utc);
            let bucket = if closed_at >= six_months_ago {
                &mut last6
            } else if closed_at >= twelve_months_ago {
                &mut m6to12
            } else {
                &mut older
            };
            bucket.prs_closed += 1;
            bucket
                .contributors
                .insert(author.to_string(), avatar_url.to_string());
        }
    }

    json!({
        "last6Months": last6.to_json(),
        "months6to12": m6to12.to_json(),
        "olderThan12Months": older.to_json(),
    })
}

/// Fetches GitHub's weekly commit-activity stats (the data behind the green
/// bar graph on a repo's Insights > Commits page) for the repository's
/// default branch. GitHub computes these stats asynchronously on first
/// request (returning 202 while it does), so this retries a few times.
fn fetch_commit_activity(
    client: &reqwest::blocking::Client,
    token: &Option<String>,
    owner: &str,
    repo: &str,
    default_branch: &str,
) -> Value {
    let url = format!("https://api.github.com/repos/{owner}/{repo}/stats/commit_activity");
    let unavailable = || json!({ "available": false, "branch": default_branch, "days": [] });

    let mut status = 0u16;
    let mut body = Value::Null;
    for attempt in 0..4u64 {
        match api_get(client, token, &url) {
            Ok((s, b)) => {
                status = s;
                body = b;
                if s != 202 {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_secs(2 + attempt));
            }
            Err(_) => break,
        }
    }

    if status != 200 {
        return unavailable();
    }
    let Some(weeks) = body.as_array() else {
        return unavailable();
    };

    // Flatten into one entry per day (each week's `days` is Sun..Sat), so the
    // frontend can render a GitHub-profile-style calendar heatmap of squares.
    let mut days_json: Vec<Value> = Vec::new();
    for week in weeks {
        let (Some(week_ts), Some(day_counts)) = (
            week.get("week").and_then(|v| v.as_i64()),
            week.get("days").and_then(|v| v.as_array()),
        ) else {
            continue;
        };
        let Some(week_start) = DateTime::<Utc>::from_timestamp(week_ts, 0) else {
            continue;
        };
        for (offset, count) in day_counts.iter().enumerate() {
            let Some(count) = count.as_u64() else { continue };
            let date = (week_start + Duration::days(offset as i64))
                .format("%Y-%m-%d")
                .to_string();
            days_json.push(json!({ "date": date, "count": count }));
        }
    }

    json!({ "available": true, "branch": default_branch, "days": days_json })
}

fn purl_ecosystem(purl: &str) -> Option<String> {
    let rest = purl.strip_prefix("pkg:")?;
    let ecosystem = rest.split('/').next()?;
    Some(ecosystem.to_string())
}

const TOP_DEPENDENCIES_LIMIT: usize = 20;

struct SbomPackage {
    name: String,
    version: Option<String>,
    ecosystem: Option<String>,
}

fn empty_dependencies() -> Value {
    json!({
        "available": false,
        "packageManagers": [],
        "topDependencies": [],
        "directDependencyCount": 0,
    })
}

/// Fetches the repo's SBOM and returns per-ecosystem totals plus, for each
/// first-level (direct) dependency, how many other packages it transitively
/// pulls in — computed by walking the SBOM's `DEPENDS_ON` graph from the
/// package the repo's document `DESCRIBES`, so heavy first-level deps stand
/// out. Only the top `TOP_DEPENDENCIES_LIMIT` (by transitive count) are
/// returned; `directDependencyCount` carries the true total.
fn fetch_dependencies(client: &reqwest::blocking::Client, token: &Option<String>, owner: &str, repo: &str) -> Value {
    let url = format!("https://api.github.com/repos/{owner}/{repo}/dependency-graph/sbom");
    let (status, body) = match api_get(client, token, &url) {
        Ok(v) => v,
        Err(_) => return empty_dependencies(),
    };
    if status != 200 {
        return empty_dependencies();
    }

    let sbom = body.get("sbom").cloned().unwrap_or(Value::Null);
    let packages = sbom
        .get("packages")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let relationships = sbom
        .get("relationships")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let doc_id = sbom
        .get("SPDXID")
        .and_then(|v| v.as_str())
        .unwrap_or("SPDXRef-DOCUMENT");

    let mut pkg_by_id: BTreeMap<String, SbomPackage> = BTreeMap::new();
    let mut ecosystem_counts: BTreeMap<String, u32> = BTreeMap::new();

    for pkg in &packages {
        let id = match pkg.get("SPDXID").and_then(|v| v.as_str()) {
            Some(s) => s.to_string(),
            None => continue,
        };
        let name = pkg
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(&id)
            .to_string();
        let version = pkg
            .get("versionInfo")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let ecosystem = pkg
            .get("externalRefs")
            .and_then(|v| v.as_array())
            .and_then(|refs| {
                refs.iter().find_map(|r| {
                    if r.get("referenceCategory").and_then(|v| v.as_str()) != Some("PACKAGE-MANAGER") {
                        return None;
                    }
                    r.get("referenceLocator")
                        .and_then(|v| v.as_str())
                        .and_then(purl_ecosystem)
                })
            });
        if let Some(eco) = &ecosystem {
            *ecosystem_counts.entry(eco.clone()).or_insert(0) += 1;
        }
        pkg_by_id.insert(id, SbomPackage { name, version, ecosystem });
    }

    if pkg_by_id.is_empty() {
        return empty_dependencies();
    }

    let package_managers: Vec<Value> = ecosystem_counts
        .into_iter()
        .map(|(ecosystem, count)| json!({ "ecosystem": ecosystem, "count": count }))
        .collect();

    // pkg id -> ids it directly DEPENDS_ON (root included, keyed by doc_id's DESCRIBES target).
    let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut root_id: Option<String> = None;
    for rel in &relationships {
        let from = rel.get("spdxElementId").and_then(|v| v.as_str());
        let to = rel.get("relatedSpdxElement").and_then(|v| v.as_str());
        let rtype = rel.get("relationshipType").and_then(|v| v.as_str());
        match (from, to, rtype) {
            (Some(f), Some(t), Some("DESCRIBES")) if f == doc_id => {
                root_id = Some(t.to_string());
            }
            (Some(f), Some(t), Some("DEPENDS_ON")) => {
                children.entry(f.to_string()).or_default().push(t.to_string());
            }
            _ => {}
        }
    }

    let Some(root_id) = root_id else {
        return json!({
            "available": true,
            "packageManagers": package_managers,
            "topDependencies": [],
            "directDependencyCount": 0,
        });
    };

    let direct_ids = children.get(&root_id).cloned().unwrap_or_default();

    let mut top: Vec<(&String, u32)> = direct_ids
        .iter()
        .map(|id| {
            // BFS over package->package DEPENDS_ON edges (not the root's own
            // edges) to count everything this direct dependency pulls in.
            let mut visited: HashSet<&str> = HashSet::new();
            visited.insert(id.as_str());
            let mut queue: VecDeque<&str> = VecDeque::new();
            queue.push_back(id.as_str());
            while let Some(current) = queue.pop_front() {
                if let Some(kids) = children.get(current) {
                    for kid in kids {
                        if visited.insert(kid.as_str()) {
                            queue.push_back(kid.as_str());
                        }
                    }
                }
            }
            (id, (visited.len() - 1) as u32)
        })
        .collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));

    let top_dependencies: Vec<Value> = top
        .into_iter()
        .take(TOP_DEPENDENCIES_LIMIT)
        .filter_map(|(id, transitive_count)| {
            pkg_by_id.get(id).map(|p| {
                json!({
                    "name": p.name,
                    "version": p.version,
                    "ecosystem": p.ecosystem,
                    "transitiveCount": transitive_count,
                })
            })
        })
        .collect();

    json!({
        "available": true,
        "packageManagers": package_managers,
        "topDependencies": top_dependencies,
        "directDependencyCount": direct_ids.len(),
    })
}

fn main() {
    let token = github_token();
    if token.is_none() {
        eprintln!("warning: no GitHub token found (GITHUB_TOKEN unset, `gh auth token` unavailable) — continuing unauthenticated, expect low rate limits");
    }
    let client = reqwest::blocking::Client::new();

    let mut projects = Vec::new();

    for &(owner, repo, org_fallback) in REPOS {
        eprintln!("fetching {owner}/{repo}...");

        let repo_url = format!("https://api.github.com/repos/{owner}/{repo}");
        let (status, repo_info) = api_get(&client, &token, &repo_url).expect("repo info request failed");
        if status != 200 {
            eprintln!("  skipping {owner}/{repo}: repo info returned HTTP {status}");
            continue;
        }

        let default_branch = repo_info
            .get("default_branch")
            .and_then(|v| v.as_str())
            .unwrap_or("main")
            .to_string();
        let github_description = repo_info
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let github_topics: Vec<String> = repo_info
            .get("topics")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|t| t.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let github_license = repo_info
            .pointer("/license/spdx_id")
            .and_then(|v| v.as_str())
            .filter(|s| *s != "NOASSERTION")
            .map(|s| s.to_string());
        let default_html_url = format!("https://github.com/{owner}/{repo}");
        let repo_html_url = repo_info
            .get("html_url")
            .and_then(|v| v.as_str())
            .unwrap_or(&default_html_url)
            .to_string();

        let publiccode = fetch_publiccode(&client, owner, repo, &default_branch);
        let has_publiccode = publiccode.is_some();

        let github_name = repo_info
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(repo)
            .to_string();
        let name = publiccode
            .as_ref()
            .and_then(|p| p.name.clone())
            .filter(|n| !n.trim().is_empty())
            .unwrap_or(github_name);

        let (short_description, long_description) = publiccode
            .as_ref()
            .and_then(|p| p.description.get("nl").or_else(|| p.description.get("en")))
            .map(|d| (d.short_description.clone(), d.long_description.clone()))
            .unwrap_or((None, None));

        let description = short_description.or(github_description).unwrap_or_default();

        let tags: Vec<String> = publiccode
            .as_ref()
            .map(|p| p.categories.clone())
            .filter(|c| !c.is_empty())
            .unwrap_or(github_topics);

        let software_type = publiccode.as_ref().and_then(|p| p.software_type.clone());
        let development_status = publiccode.as_ref().and_then(|p| p.development_status.clone());
        let license = publiccode
            .as_ref()
            .and_then(|p| p.legal.as_ref())
            .and_then(|l| l.license.clone())
            .or(github_license);
        let contacts: Vec<PubliccodeContact> = publiccode
            .as_ref()
            .and_then(|p| p.maintenance.as_ref())
            .map(|m| m.contacts.clone())
            .unwrap_or_default();

        let languages_url = format!("https://api.github.com/repos/{owner}/{repo}/languages");
        let (_, languages) = api_get(&client, &token, &languages_url).unwrap_or((0, Value::Null));
        let languages = if languages.is_object() { languages } else { json!({}) };
        let primary_language = languages
            .as_object()
            .and_then(|m| m.iter().max_by_key(|(_, v)| v.as_u64().unwrap_or(0)))
            .map(|(k, _)| k.clone());

        let activity = fetch_activity(&client, &token, owner, repo);
        let dependencies = fetch_dependencies(&client, &token, owner, repo);
        let commit_activity =
            fetch_commit_activity(&client, &token, owner, repo, &default_branch);

        let contacts_json: Vec<Value> = contacts
            .iter()
            .map(|c| {
                json!({
                    "name": c.name,
                    "email": c.email,
                    "affiliation": c.affiliation,
                })
            })
            .collect();

        projects.push(json!({
            "slug": repo,
            "name": name,
            "org": org_fallback,
            "repoUrl": repo_html_url,
            "description": description,
            "longDescription": long_description,
            "tags": tags,
            "hasPubliccode": has_publiccode,
            "softwareType": software_type,
            "developmentStatus": development_status,
            "license": license,
            "primaryLanguage": primary_language,
            "languages": languages,
            "contacts": contacts_json,
            "activity": activity,
            "commitActivity": commit_activity,
            "dependencies": dependencies,
        }));
    }

    let output = json!({
        "generatedAt": Utc::now().to_rfc3339(),
        "projects": projects,
    });

    let out_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../web/data/projects.json");
    std::fs::write(&out_path, serde_json::to_string_pretty(&output).unwrap())
        .expect("failed to write data/projects.json");
    eprintln!("wrote {}", out_path.display());
}
