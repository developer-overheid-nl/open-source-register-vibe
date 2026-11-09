//! Één binary die de volledige applicatie draait (zie ARCHITECTURE.md):
//!
//! - een achtergrondthread bevraagt periodiek de GitHub API (`ingest.rs`) en
//!   houdt het resultaat in-memory vast — geen JSON-bestand en geen externe
//!   database;
//! - Axum rendert de pagina's server-side met Tera-templates en serveert de
//!   statische assets (JS/CSS) uit `web/` onder `/static`. Er is geen
//!   JSON-API: alle data verlaat de backend als gerenderde HTML.

mod icons;
mod ingest;
mod view;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use serde_json::Value;
use std::sync::{Arc, RwLock};
use tera::Tera;
use tower_http::services::ServeDir;

const REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(24 * 60 * 60);

#[derive(Clone)]
struct AppState {
    tera: Arc<Tera>,
    /// De laatst opgehaalde dataset; `None` zolang de eerste ingest draait.
    data: Arc<RwLock<Option<Arc<Value>>>>,
}

#[tokio::main]
async fn main() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let tera = Tera::new(&format!("{manifest}/../templates/**/*.html"))
        .expect("templates konden niet geladen worden");

    let state = AppState {
        tera: Arc::new(tera),
        data: Arc::new(RwLock::new(None)),
    };

    // Achtergrond-ingest: bij opstarten eenmalig, daarna elke 24 uur. De
    // blocking HTTP-calls lopen in een gewone OS-thread en blokkeren de
    // tokio-runtime dus nooit.
    let ingest_data = state.data.clone();
    std::thread::spawn(move || loop {
        let data = Arc::new(ingest::collect());
        *ingest_data.write().expect("data-lock vergrendeld") = Some(data);
        std::thread::sleep(REFRESH_INTERVAL);
    });

    let app = Router::new()
        .route("/", get(catalog))
        .route("/project/{slug}", get(project))
        .nest_service("/static", ServeDir::new(format!("{manifest}/../web")))
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("poort niet beschikbaar");
    eprintln!("Server draait op http://localhost:{port}");
    axum::serve(listener, app).await.expect("serverfout");
}

/// Leest herhaalde query-parameters (bijv. `?tag=…` of `?language=…`)
/// meermaals (dedupe, lege waarden genegeerd).
fn active_values(query: &[(String, String)], key: &str) -> Vec<String> {
    let mut values = Vec::new();
    for (k, value) in query {
        if k == key && !value.is_empty() && !values.contains(value) {
            values.push(value.clone());
        }
    }
    values
}

fn current_data(state: &AppState) -> Option<Arc<Value>> {
    state.data.read().ok()?.clone()
}

fn render(state: &AppState, template: &str, context: &tera::Context) -> Result<Html<String>, StatusCode> {
    state
        .tera
        .render(template, context)
        .map(Html)
        .map_err(|e| {
            eprintln!("template {template} renderde niet: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

fn render_loading(state: &AppState) -> Html<String> {
    // Tijdens de eerste ingest is er nog geen data: toon een laadscherm dat
    // zichzelf periodiek herlaadt tot de data binnen is.
    state
        .tera
        .render("loading.html", &tera::Context::new())
        .map(Html)
        .unwrap_or_else(|_| Html("<p>De catalogus-data wordt opgehaald — even geduld.</p>".to_string()))
}

async fn catalog(
    State(state): State<AppState>,
    Query(query): Query<Vec<(String, String)>>,
) -> Result<Html<String>, StatusCode> {
    let Some(data) = current_data(&state) else {
        return Ok(render_loading(&state));
    };
    let active_tags = active_values(&query, "tag");
    let active_languages = active_values(&query, "language");
    let context = view::catalog_context(&data, &active_tags, &active_languages);
    render(&state, "index.html", &context)
}

async fn project(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<(StatusCode, Html<String>), StatusCode> {
    let Some(data) = current_data(&state) else {
        return Ok((StatusCode::OK, render_loading(&state)));
    };
    let Some(project) = view::find_project(&data, &slug) else {
        let mut context = tera::Context::new();
        context.insert("slug", &slug);
        let html = render(&state, "404.html", &context)?;
        return Ok((StatusCode::NOT_FOUND, html));
    };
    let context = view::project_context(&data, project);
    render(&state, "project.html", &context).map(|html| (StatusCode::OK, html))
}
