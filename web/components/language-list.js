/** Renders a tag per language (sorted by share, largest first) into `container`. */
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

  const template = document.getElementById("tpl-language-tag");
  for (const [language, bytes] of entries) {
    const percentage = total > 0 ? Math.round((bytes / total) * 100) : 0;
    const node = template.content.cloneNode(true);
    node.querySelector(".language-tag").textContent = `${language} (${percentage}%)`;
    container.appendChild(node);
  }
}
