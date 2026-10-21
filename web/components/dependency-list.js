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

const TOP_LIMIT = 20;

/** Counts unique OSV advisories per ecosystem across the vulnerable packages. */
function countAdvisories(vulnerablePackages) {
  const byEcosystem = new Map();
  for (const pkg of vulnerablePackages || []) {
    if (!byEcosystem.has(pkg.ecosystem)) byEcosystem.set(pkg.ecosystem, new Set());
    for (const id of pkg.advisories) byEcosystem.get(pkg.ecosystem).add(id);
  }
  return new Map([...byEcosystem].map(([ecosystem, ids]) => [ecosystem, ids.size]));
}

/**
 * Renders one tab per package manager into `container`, plus, for the
 * selected one, a ranked list of its first-level (direct) dependencies that
 * pull in the most transitive dependencies — so dependency "hubs" stand out.
 * The first selectable package manager is selected up front. Package managers
 * with known vulnerabilities get a badge with their advisory count and are
 * listed first. Falls back to an empty-state message when no dependency data
 * is available.
 */
export function renderDependencyList(container, dependencies, vulnerablePackages) {
  container.innerHTML = "";

  if (!dependencies || !dependencies.available || dependencies.packageManagers.length === 0) {
    const p = document.createElement("p");
    p.className = "dependency-list__empty";
    p.textContent = "Geen detecteerbare package manager gevonden voor dit project.";
    container.appendChild(p);
    return;
  }

  const allTop = [...(dependencies.topDependencies || [])].sort(
    (a, b) => b.transitiveCount - a.transitiveCount || a.name.localeCompare(b.name),
  );

  const summary = document.createElement("div");
  summary.className = "dependency-list__summary";
  const section = document.createElement("div");
  section.className = "dependency-list__top";

  const buttons = [];

  const advisoryCounts = countAdvisories(vulnerablePackages);
  const packageManagers = [...dependencies.packageManagers].sort(
    (a, b) => (advisoryCounts.get(b.ecosystem) || 0) - (advisoryCounts.get(a.ecosystem) || 0),
  );

  const rowTemplate = document.getElementById("tpl-dependency-row");
  for (const { ecosystem, count, directCount } of packageManagers) {
    const node = rowTemplate.content.cloneNode(true);
    const button = node.querySelector(".dependency-row");
    node.querySelector(".dependency-row__ecosystem").textContent = ecosystemLabel(ecosystem);
    const advisories = advisoryCounts.get(ecosystem) || 0;
    if (advisories > 0) {
      const badge = node.querySelector(".dependency-row__vulns");
      badge.hidden = false;
      badge.textContent = advisories;
      badge.title = advisories === 1 ? "1 bekende kwetsbaarheid" : `${advisories} bekende kwetsbaarheden`;
      badge.setAttribute("aria-label", badge.title);
    }
    const direct = node.querySelector(".dependency-row__direct");
    const transitive = node.querySelector(".dependency-row__transitive");
    if (directCount == null) {
      // Data from before the ingest tracked direct counts per ecosystem.
      direct.textContent = count === 1 ? "1 dependency" : `${count} dependencies`;
      transitive.remove();
    } else {
      // The SBOM total includes the direct packages, so the remainder is transitive.
      direct.textContent = `${directCount} direct`;
      transitive.textContent = `+ ${Math.max(count - directCount, 0)} transitief`;
      button.title = `${count} packages in totaal`;
    }
    if (!allTop.some((d) => d.ecosystem === ecosystem)) {
      button.disabled = true;
      button.title +=
        (button.title ? " — " : "") + "Geen directe dependencies bekend voor deze package manager";
    }
    button.addEventListener("click", () => select(ecosystem));
    buttons.push({ ecosystem, el: button });
    summary.appendChild(node);
  }
  container.appendChild(summary);

  function select(ecosystem) {
    for (const b of buttons) b.el.setAttribute("aria-pressed", String(b.ecosystem === ecosystem));
    renderTop(section, dependencies, allTop, ecosystem);
  }

  const first = buttons.find((b) => !b.el.disabled);
  if (!first) return;
  container.appendChild(section);
  select(first.ecosystem);
}

function renderTop(section, dependencies, allTop, ecosystem) {
  section.innerHTML = "";

  const shown = allTop.filter((d) => d.ecosystem === ecosystem).slice(0, TOP_LIMIT);
  const pm = dependencies.packageManagers.find((p) => p.ecosystem === ecosystem);
  const total = pm?.directCount ?? shown.length;

  const heading = document.createElement("h3");
  heading.className = "dependency-list__top-heading";
  heading.textContent = `Directe ${ecosystemLabel(ecosystem)}-dependencies met de meeste transitieve dependencies`;
  section.appendChild(heading);

  const hint = document.createElement("p");
  hint.className = "dependency-list__top-hint";
  const scope = total > shown.length
    ? `Top ${shown.length} van ${total} directe dependencies`
    : `Alle ${total} directe dependencies`;
  hint.textContent = `${scope}, gesorteerd op het aantal dependencies dat ze zelf binnenhalen.`;
  section.appendChild(hint);

  const list = document.createElement("ol");
  list.className = "dependency-list__top-rows";

  const maxCount = Math.max(...shown.map((d) => d.transitiveCount), 1);
  const rowTemplate = document.getElementById("tpl-dependency-top-row");
  for (const dep of shown) {
    const node = rowTemplate.content.cloneNode(true);
    node.querySelector(".dependency-top-row__name").textContent = dep.version
      ? `${dep.name} ${dep.version}`
      : dep.name;
    node.querySelector(".dependency-top-row__bar-fill").style.width =
      `${(dep.transitiveCount / maxCount) * 100}%`;
    node.querySelector(".dependency-top-row__count").textContent =
      dep.transitiveCount === 1 ? "1 transitieve dependency" : `${dep.transitiveCount} transitieve dependencies`;
    list.appendChild(node);
  }
  section.appendChild(list);
}
