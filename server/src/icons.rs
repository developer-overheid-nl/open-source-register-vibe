//! Statische iconen en kleuren voor server-side rendering, 1-op-1 geport van
//! de vroegere frontend-bestanden `software-type-icons.js` en
//! `language-colors.js`.

/// Simple line icons (24x24, stroke-only, Rijkshuisstijl-style: thin strokes,
/// round joins, no fill) for each `softwareType` value from the publiccode.yml
/// schema (https://yml.publiccode.tools). Keyed on the raw schema value, since
/// the catalogus shows that value 1-op-1 without translation (see FEATURES.md).
pub fn software_type_icon(software_type: Option<&str>) -> &'static str {
    match software_type {
        Some("standalone/mobile") => {
            "<rect x=\"7\" y=\"2\" width=\"10\" height=\"20\" rx=\"2\"/><line x1=\"11\" y1=\"18\" x2=\"13\" y2=\"18\"/>"
        }
        Some("standalone/iot") => {
            "<rect x=\"7\" y=\"7\" width=\"10\" height=\"10\" rx=\"1\"/><circle cx=\"12\" cy=\"12\" r=\"2\"/><line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"7\"/><line x1=\"12\" y1=\"17\" x2=\"12\" y2=\"22\"/><line x1=\"2\" y1=\"12\" x2=\"7\" y2=\"12\"/><line x1=\"17\" y1=\"12\" x2=\"22\" y2=\"12\"/>"
        }
        Some("standalone/desktop") => {
            "<rect x=\"3\" y=\"4\" width=\"18\" height=\"12\" rx=\"1\"/><line x1=\"8\" y1=\"20\" x2=\"16\" y2=\"20\"/><line x1=\"12\" y1=\"16\" x2=\"12\" y2=\"20\"/>"
        }
        Some("standalone/web") => {
            "<circle cx=\"12\" cy=\"12\" r=\"9\"/><line x1=\"3\" y1=\"12\" x2=\"21\" y2=\"12\"/><path d=\"M12 3c3 4 3 14 0 18\"/><path d=\"M12 3c-3 4-3 14 0 18\"/>"
        }
        Some("standalone/backend") => {
            "<rect x=\"4\" y=\"4\" width=\"16\" height=\"7\" rx=\"1\"/><rect x=\"4\" y=\"13\" width=\"16\" height=\"7\" rx=\"1\"/><circle cx=\"8\" cy=\"7.5\" r=\"0.9\" fill=\"currentColor\" stroke=\"none\"/><circle cx=\"8\" cy=\"16.5\" r=\"0.9\" fill=\"currentColor\" stroke=\"none\"/>"
        }
        Some("standalone/other") => {
            "<path d=\"M12 3l8 4.5v9L12 21l-8-4.5v-9L12 3z\"/><path d=\"M12 12v9\"/><path d=\"M4 7.5l8 4.5 8-4.5\"/>"
        }
        Some("addon") => {
            "<path d=\"M9 4.2h3.2a1 1 0 0 1 1 1v1.6a1.7 1.7 0 1 0 0 3.4v3.6a1 1 0 0 1-1 1h-1.6a1.7 1.7 0 1 1-3.4 0H5.2a1 1 0 0 1-1-1V11a1.7 1.7 0 1 0 0-3.4V5.2a1 1 0 0 1 1-1H9z\"/>"
        }
        Some("library") => {
            "<path d=\"M4 5a2 2 0 0 1 2-2h5v18H6a2 2 0 0 1-2-2V5z\"/><path d=\"M20 5a2 2 0 0 0-2-2h-5v18h5a2 2 0 0 0 2-2V5z\"/>"
        }
        Some("configurationFiles") => {
            "<line x1=\"4\" y1=\"6\" x2=\"20\" y2=\"6\"/><circle cx=\"9\" cy=\"6\" r=\"2\" fill=\"var(--color-bg)\"/><line x1=\"4\" y1=\"12\" x2=\"20\" y2=\"12\"/><circle cx=\"15\" cy=\"12\" r=\"2\" fill=\"var(--color-bg)\"/><line x1=\"4\" y1=\"18\" x2=\"20\" y2=\"18\"/><circle cx=\"7\" cy=\"18\" r=\"2\" fill=\"var(--color-bg)\"/>"
        }
        _ => {
            "<circle cx=\"12\" cy=\"12\" r=\"9\"/><path d=\"M9.5 9a2.5 2.5 0 1 1 3.5 2.3c-.8.4-1.5 1-1.5 2.2\"/><line x1=\"12\" y1=\"17\" x2=\"12\" y2=\"17.01\"/>"
        }
    }
}

/// Approximate GitHub Linguist colours for common programming languages.
/// Anything not in this list falls back to a colour derived deterministically
/// from the language name, so it's still stable and distinct.
pub fn language_color(language: &str) -> String {
    let known = match language {
        "JavaScript" => Some("#f1e05a"),
        "TypeScript" => Some("#3178c6"),
        "Python" => Some("#3572A5"),
        "Java" => Some("#b07219"),
        "Kotlin" => Some("#A97BFF"),
        "C" => Some("#555555"),
        "C++" => Some("#f34b7d"),
        "C#" => Some("#178600"),
        "Go" => Some("#00ADD8"),
        "Rust" => Some("#dea584"),
        "Ruby" => Some("#701516"),
        "PHP" => Some("#4F5D95"),
        "Swift" => Some("#F05138"),
        "Dart" => Some("#00B4AB"),
        "HTML" => Some("#e34c26"),
        "CSS" => Some("#563d7c"),
        "SCSS" => Some("#c6538c"),
        "Less" => Some("#1d365d"),
        "Shell" => Some("#89e051"),
        "PowerShell" => Some("#012456"),
        "Vue" => Some("#41b883"),
        "Svelte" => Some("#ff3e00"),
        "Scala" => Some("#c22d40"),
        "Elixir" => Some("#6e4a7e"),
        "Erlang" => Some("#B83998"),
        "Haskell" => Some("#5e5086"),
        "Lua" => Some("#000080"),
        "Perl" => Some("#0298c3"),
        "R" => Some("#198CE7"),
        "Objective-C" => Some("#438eff"),
        "Dockerfile" => Some("#384d54"),
        "Makefile" => Some("#427819"),
        "YAML" => Some("#cb171e"),
        "Markdown" => Some("#083fa1"),
        "Nix" => Some("#7e7eff"),
        "Zig" => Some("#ec915c"),
        "Julia" => Some("#a270ba"),
        "Clojure" => Some("#db5855"),
        "Elm" => Some("#60B5CC"),
        "F#" => Some("#b845fc"),
        "OCaml" => Some("#3be133"),
        "Groovy" => Some("#4298b8"),
        "Assembly" => Some("#6E4C13"),
        "Vim Script" | "Vim script" => Some("#199f4b"),
        "TeX" => Some("#3D6117"),
        "Jupyter Notebook" => Some("#DA5B0B"),
        "CoffeeScript" => Some("#244776"),
        "Solidity" => Some("#AA6746"),
        "WebAssembly" => Some("#04133b"),
        _ => None,
    };
    if let Some(hex) = known {
        return hex.to_string();
    }

    // Deterministically derive a stable HSL colour for an unknown language
    // (same hash as the old frontend: h = h*31 + charCode, mod 360).
    let mut hash: u32 = 0;
    for c in language.chars() {
        hash = hash.wrapping_mul(31).wrapping_add(c as u32);
    }
    format!("hsl({}, 55%, 45%)", hash % 360)
}
