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
  return ECOSYSTEM_LABELS[ecosystem] || ecosystem;
}

/** Renders one row per package manager into `container`, or a fallback message. */
export function renderDependencyList(container, dependencies) {
  container.innerHTML = "";

  if (!dependencies || !dependencies.available || dependencies.packageManagers.length === 0) {
    const p = document.createElement("p");
    p.className = "dependency-list__empty";
    p.textContent = "Geen detecteerbare package manager gevonden voor dit project.";
    container.appendChild(p);
    return;
  }

  const template = document.getElementById("tpl-dependency-row");
  for (const { ecosystem, count } of dependencies.packageManagers) {
    const node = template.content.cloneNode(true);
    node.querySelector(".dependency-row__ecosystem").textContent = ecosystemLabel(ecosystem);
    node.querySelector(".dependency-row__count").textContent =
      count === 1 ? "1 dependency" : `${count} dependencies`;
    container.appendChild(node);
  }
}
