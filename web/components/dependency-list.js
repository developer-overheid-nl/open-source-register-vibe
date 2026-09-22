const ECOSYSTEM_LABELS = {
  npm: "npm",
  pypi: "pip (PyPI)",
  cargo: "Cargo",
  maven: "Maven",
  composer: "Composer",
  nuget: "NuGet",
  gem: "RubyGems",
  golang: "Go modules",
  githubactions: "GitHub Actions",
  github: "GitHub (overig)",
};

function ecosystemLabel(ecosystem) {
  if (!ecosystem) return "onbekend";
  return ECOSYSTEM_LABELS[ecosystem] || ecosystem;
}

/**
 * Renders one summary row per package manager, plus a ranked list of the
 * first-level (direct) dependencies that pull in the most transitive
 * dependencies — so dependency "hubs" stand out — into `container`. Falls
 * back to an empty-state message when no dependency data is available.
 */
export function renderDependencyList(container, dependencies) {
  container.innerHTML = "";

  if (!dependencies || !dependencies.available || dependencies.packageManagers.length === 0) {
    const p = document.createElement("p");
    p.className = "dependency-list__empty";
    p.textContent = "Geen detecteerbare package manager gevonden voor dit project.";
    container.appendChild(p);
    return;
  }

  const summary = document.createElement("div");
  summary.className = "dependency-list__summary";
  const rowTemplate = document.getElementById("tpl-dependency-row");
  for (const { ecosystem, count } of dependencies.packageManagers) {
    const node = rowTemplate.content.cloneNode(true);
    node.querySelector(".dependency-row__ecosystem").textContent = ecosystemLabel(ecosystem);
    node.querySelector(".dependency-row__count").textContent =
      count === 1 ? "1 dependency" : `${count} dependencies`;
    summary.appendChild(node);
  }
  container.appendChild(summary);

  const topDependencies = dependencies.topDependencies || [];
  if (topDependencies.length === 0) return;

  const section = document.createElement("div");
  section.className = "dependency-list__top";

  const heading = document.createElement("h3");
  heading.className = "dependency-list__top-heading";
  heading.textContent = "Directe dependencies met de meeste transitieve dependencies";
  section.appendChild(heading);

  const total = dependencies.directDependencyCount ?? topDependencies.length;
  const hint = document.createElement("p");
  hint.className = "dependency-list__top-hint";
  hint.textContent =
    total > topDependencies.length
      ? `Top ${topDependencies.length} van ${total} directe dependencies, gesorteerd op het aantal dependencies dat ze zelf binnenhalen.`
      : `Alle ${total} directe dependencies, gesorteerd op het aantal dependencies dat ze zelf binnenhalen.`;
  section.appendChild(hint);

  const list = document.createElement("ol");
  list.className = "dependency-list__top-rows";

  const maxCount = Math.max(...topDependencies.map((d) => d.transitiveCount), 1);
  const rowTemplate2 = document.getElementById("tpl-dependency-top-row");
  for (const dep of topDependencies) {
    const node = rowTemplate2.content.cloneNode(true);
    node.querySelector(".dependency-top-row__name").textContent = dep.version
      ? `${dep.name} ${dep.version}`
      : dep.name;
    node.querySelector(".dependency-top-row__ecosystem").textContent = ecosystemLabel(dep.ecosystem);
    node.querySelector(".dependency-top-row__bar-fill").style.width =
      `${(dep.transitiveCount / maxCount) * 100}%`;
    node.querySelector(".dependency-top-row__count").textContent =
      dep.transitiveCount === 1 ? "1 transitieve dependency" : `${dep.transitiveCount} transitieve dependencies`;
    list.appendChild(node);
  }
  section.appendChild(list);
  container.appendChild(section);
}
