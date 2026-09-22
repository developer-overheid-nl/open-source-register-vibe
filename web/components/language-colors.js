/**
 * Approximate GitHub Linguist colours for common programming languages, so the
 * catalogus can show the same colour-coded dots/bars people recognise from a
 * GitHub repo page. Anything not in this list falls back to a colour derived
 * deterministically from the language name, so it's still stable and distinct.
 */
const LANGUAGE_COLORS = {
  "JavaScript": "#f1e05a",
  "TypeScript": "#3178c6",
  "Python": "#3572A5",
  "Java": "#b07219",
  "Kotlin": "#A97BFF",
  "C": "#555555",
  "C++": "#f34b7d",
  "C#": "#178600",
  "Go": "#00ADD8",
  "Rust": "#dea584",
  "Ruby": "#701516",
  "PHP": "#4F5D95",
  "Swift": "#F05138",
  "Dart": "#00B4AB",
  "HTML": "#e34c26",
  "CSS": "#563d7c",
  "SCSS": "#c6538c",
  "Less": "#1d365d",
  "Shell": "#89e051",
  "PowerShell": "#012456",
  "Vue": "#41b883",
  "Svelte": "#ff3e00",
  "Scala": "#c22d40",
  "Elixir": "#6e4a7e",
  "Erlang": "#B83998",
  "Haskell": "#5e5086",
  "Lua": "#000080",
  "Perl": "#0298c3",
  "R": "#198CE7",
  "Objective-C": "#438eff",
  "Dockerfile": "#384d54",
  "Makefile": "#427819",
  "YAML": "#cb171e",
  "Markdown": "#083fa1",
  "Nix": "#7e7eff",
  "Zig": "#ec915c",
  "Julia": "#a270ba",
  "Clojure": "#db5855",
  "Elm": "#60B5CC",
  "F#": "#b845fc",
  "OCaml": "#3be133",
  "Groovy": "#4298b8",
  "Assembly": "#6E4C13",
  "Vim Script": "#199f4b",
  "TeX": "#3D6117",
  "Jupyter Notebook": "#DA5B0B",
  "CoffeeScript": "#244776",
  "Solidity": "#AA6746",
  "WebAssembly": "#04133b",
  "Vim script": "#199f4b",
};

/** Deterministically derives a stable HSL colour for a language not in the known list. */
function fallbackColor(language) {
  let hash = 0;
  for (let i = 0; i < language.length; i++) {
    hash = (hash * 31 + language.charCodeAt(i)) >>> 0;
  }
  const hue = hash % 360;
  return `hsl(${hue}, 55%, 45%)`;
}

/** Returns a GitHub-style colour (hex or hsl) for a given language name. */
export function getLanguageColor(language) {
  if (!language) return fallbackColor("onbekend");
  return LANGUAGE_COLORS[language] || fallbackColor(language);
}
