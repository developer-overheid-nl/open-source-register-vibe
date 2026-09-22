import { getLanguageColor } from "./language-colors.js";

/**
 * Renders a GitHub-style language bar (stacked, coloured by share) plus a tag
 * per language (sorted by share, largest first) into `container`.
 */
export function renderLanguageList(container, languages) {
  container.innerHTML = "";

  const entries = Object.entries(languages || {});
  if (entries.length === 0) {
    const p = document.createElement("p");
    p.className = "language-list__empty";
    p.textContent = "Geen detecteerbare programmeertaal gevonden voor dit project.";
    container.appendChild(p);
    return;
  }

  const total = entries.reduce((sum, [, bytes]) => sum + bytes, 0);
  entries.sort(([, a], [, b]) => b - a);

  const bar = document.createElement("div");
  bar.className = "language-list__bar";
  bar.setAttribute("role", "img");
  bar.setAttribute(
    "aria-label",
    entries
      .map(([language, bytes]) => `${language} ${total > 0 ? Math.round((bytes / total) * 100) : 0}%`)
      .join(", ")
  );
  for (const [language, bytes] of entries) {
    const percentage = total > 0 ? (bytes / total) * 100 : 0;
    const segment = document.createElement("span");
    segment.className = "language-list__bar-segment";
    segment.style.width = `${percentage}%`;
    segment.style.backgroundColor = getLanguageColor(language);
    segment.title = `${language} (${Math.round(percentage)}%)`;
    bar.appendChild(segment);
  }
  container.appendChild(bar);

  const tags = document.createElement("div");
  tags.className = "language-list__tags";

  const template = document.getElementById("tpl-language-tag");
  for (const [language, bytes] of entries) {
    const percentage = total > 0 ? Math.round((bytes / total) * 100) : 0;
    const node = template.content.cloneNode(true);
    const tag = node.querySelector(".language-tag");
    const dot = document.createElement("span");
    dot.className = "language-tag__dot";
    dot.style.backgroundColor = getLanguageColor(language);
    tag.appendChild(dot);
    tag.appendChild(document.createTextNode(`${language} (${percentage}%)`));
    tags.appendChild(node);
  }
  container.appendChild(tags);
}
