import { renderProjectDetail, renderProjectNotFound } from "./components/project-detail.js";

async function init() {
  const params = new URLSearchParams(window.location.search);
  const id = params.get("id");

  const response = await fetch("data/projects.json");
  const data = await response.json();
  const project = data.projects.find((p) => p.slug === id);

  const container = document.getElementById("project-detail");

  if (!project) {
    renderProjectNotFound(container);
    return;
  }

  document.title = `${project.name} — Open Source Catalogus`;
  renderProjectDetail(container, project);
}

init();
