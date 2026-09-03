import { createProjectCard } from "./project-card.js";
import { renderEmptyState } from "./empty-state.js";

/**
 * Renders `projects` as a list of project cards into `container`, or the
 * empty state if the list is empty. `onTagClick(tag)` is forwarded from
 * each card's tag chips.
 */
export function renderProjectList(container, projects, { onTagClick }) {
  container.innerHTML = "";

  if (projects.length === 0) {
    renderEmptyState(container);
    return;
  }

  const list = document.createElement("ul");
  list.className = "project-list";
  for (const project of projects) {
    list.appendChild(createProjectCard(project, { onTagClick }));
  }
  container.appendChild(list);
}
