import { renderActivityPanels } from "./activity-panel.js";
import { renderCommitGraph } from "./commit-graph.js";
import { renderContactList } from "./contact-list.js";
import { renderDependencyList } from "./dependency-list.js";
import { renderLanguageList } from "./language-list.js";
import { renderSoftwareTypeBadge } from "./software-type-badge.js";

/** Renders the full project detail page into `container`. */
export function renderProjectDetail(container, project) {
  const template = document.getElementById("tpl-project-detail");
  container.innerHTML = "";
  const node = template.content.cloneNode(true);

  node.querySelector(".project-detail__warning-notice").hidden = Boolean(project.hasPubliccode);

  node.querySelector(".project-detail__name").textContent = project.name;
  node.querySelector(".project-detail__org").textContent = project.org || "";

  const repoLink = node.querySelector(".project-detail__repo-link");
  repoLink.href = project.repoUrl;

  node.querySelector(".project-detail__description").textContent =
    project.longDescription || project.description || "";

  renderSoftwareTypeBadge(node.querySelector(".project-detail__software-type"), project.softwareType);
  node.querySelector(".project-detail__license").textContent = project.license || "onbekend";
  node.querySelector(".project-detail__org-fact").textContent = project.org || "onbekend";

  const tagsEl = node.querySelector(".project-detail__tags");
  const chipTemplate = document.getElementById("tpl-tag-chip");
  for (const tag of project.tags || []) {
    const chipNode = chipTemplate.content.cloneNode(true);
    const button = chipNode.querySelector(".tag-chip");
    button.querySelector(".tag-chip__label").textContent = tag;
    button.addEventListener("click", () => {
      window.location.href = `index.html?tag=${encodeURIComponent(tag)}`;
    });
    tagsEl.appendChild(chipNode);
  }

  container.appendChild(node);

  renderLanguageList(document.getElementById("language-list"), project.languages);
  renderActivityPanels(document.getElementById("activity-panels"), project.activity);
  renderCommitGraph(document.getElementById("commit-graph"), project.commitActivity);
  renderDependencyList(document.getElementById("dependency-list"), project.dependencies);
  renderContactList(document.getElementById("contact-list"), project.contacts);
}

export function renderProjectNotFound(container) {
  const template = document.getElementById("tpl-not-found");
  container.innerHTML = "";
  container.appendChild(template.content.cloneNode(true));
}
