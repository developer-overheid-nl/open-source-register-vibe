import { getLanguageColor } from "./language-colors.js";
import { renderCommitSparkline } from "./commit-sparkline.js";

/**
 * Builds a `<li class="project-card">` node for one project.
 * `onTagClick(tag)` fires when a tag chip inside the card is clicked
 * (the click is stopped from also following the card link).
 */
export function createProjectCard(project, { onTagClick }) {
  const template = document.getElementById("tpl-project-card");
  const node = template.content.cloneNode(true);

  const link = node.querySelector(".project-card__link");
  link.href = `project.html?id=${encodeURIComponent(project.slug)}`;

  node.querySelector(".project-card__name").textContent = project.name;
  node.querySelector(".project-card__org").textContent = project.org || "";
  node.querySelector(".project-card__description").textContent = project.description || "";
  node.querySelector(".project-card__status").textContent = project.developmentStatus || "onbekend";

  const languageEl = node.querySelector(".project-card__language");
  if (project.primaryLanguage) {
    const dot = document.createElement("span");
    dot.className = "project-card__language-dot";
    dot.style.backgroundColor = getLanguageColor(project.primaryLanguage);
    languageEl.appendChild(dot);
    languageEl.appendChild(document.createTextNode(project.primaryLanguage));
  } else {
    languageEl.textContent = "onbekend";
  }

  renderCommitSparkline(node.querySelector(".commit-sparkline"), project.commitActivity);

  const warning = node.querySelector(".project-card__warning");
  warning.hidden = Boolean(project.hasPubliccode);

  const tagsEl = node.querySelector(".project-card__tags");
  const chipTemplate = document.getElementById("tpl-tag-chip");
  for (const tag of project.tags || []) {
    const chipNode = chipTemplate.content.cloneNode(true);
    const button = chipNode.querySelector(".tag-chip");
    button.querySelector(".tag-chip__label").textContent = tag;
    button.addEventListener("click", (event) => {
      event.preventDefault();
      event.stopPropagation();
      onTagClick(tag);
    });
    tagsEl.appendChild(chipNode);
  }

  return node.querySelector(".project-card");
}
