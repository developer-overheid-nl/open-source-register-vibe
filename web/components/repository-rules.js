const RULESET_URL = "https://github.com/developer-overheid-nl/ort-config";

// Failed errors first, then failed warnings, then unknown, then passed.
function rank(check) {
  if (check.passed === false) return check.severity === "error" ? 0 : 1;
  if (check.passed == null) return 2;
  return 3;
}

function statusText(check) {
  if (check.passed === true) return "Voldaan";
  if (check.passed == null) return "Onbekend";
  return check.severity === "error" ? "Fout" : "Waarschuwing";
}

function statusModifier(check) {
  if (check.passed === true) return "pass";
  if (check.passed == null) return "unknown";
  return check.severity === "error" ? "error" : "warning";
}

const ICONS = { pass: "✓", error: "✕", warning: "!", unknown: "?" };

/**
 * Renders the outcome of the developer-overheid-nl/ort-config repository
 * rules into `container`: a one-line summary, then one row per rule with
 * violations on top. The vulnerability rule expands to the affected packages
 * and their OSV advisories.
 */
export function renderRepositoryRules(container, compliance) {
  container.innerHTML = "";

  if (!compliance || !compliance.available || compliance.checks.length === 0) {
    const p = document.createElement("p");
    p.className = "repository-rules__empty";
    p.textContent = "Repositoryregels konden voor dit project niet worden gecontroleerd.";
    container.appendChild(p);
    return;
  }

  const checks = [...compliance.checks].sort((a, b) => rank(a) - rank(b));
  const passed = checks.filter((c) => c.passed === true).length;
  const errors = checks.filter((c) => c.passed === false && c.severity === "error").length;
  const warnings = checks.filter((c) => c.passed === false && c.severity !== "error").length;

  const summary = document.createElement("p");
  summary.className = "repository-rules__summary";
  const strong = document.createElement("strong");
  strong.textContent = `${passed} van ${checks.length} regels voldaan`;
  summary.appendChild(strong);
  const parts = [];
  if (errors) parts.push(errors === 1 ? "1 fout" : `${errors} fouten`);
  if (warnings) parts.push(warnings === 1 ? "1 waarschuwing" : `${warnings} waarschuwingen`);
  if (parts.length) summary.append(` · ${parts.join(" · ")}`);
  container.appendChild(summary);

  const list = document.createElement("ul");
  list.className = "repository-rules__list";
  const template = document.getElementById("tpl-repository-rule");
  for (const check of checks) {
    const node = template.content.cloneNode(true);
    const row = node.querySelector(".repository-rule");
    const modifier = statusModifier(check);
    row.classList.add(`repository-rule--${modifier}`);
    node.querySelector(".repository-rule__icon").textContent = ICONS[modifier];
    node.querySelector(".repository-rule__label").textContent = check.label;
    node.querySelector(".repository-rule__status").textContent = statusText(check);

    const vulnerable = compliance.vulnerablePackages || [];
    if (check.id === "VULNERABILITY_IN_DEPENDENCY" && vulnerable.length > 0) {
      row.appendChild(renderVulnerablePackages(vulnerable));
    }
    list.appendChild(node);
  }
  container.appendChild(list);

  const source = document.createElement("p");
  source.className = "repository-rules__source";
  const link = document.createElement("a");
  link.href = RULESET_URL;
  link.target = "_blank";
  link.rel = "noopener";
  link.textContent = "developer-overheid-nl/ort-config";
  source.append("Regels uit ", link, ". Bestanden worden alleen op aanwezigheid gecontroleerd, niet op inhoud.");
  container.appendChild(source);
}

function renderVulnerablePackages(packages) {
  const details = document.createElement("details");
  details.className = "repository-rule__details";
  const summary = document.createElement("summary");
  summary.textContent =
    packages.length === 1 ? "1 kwetsbaar package" : `${packages.length} kwetsbare packages`;
  details.appendChild(summary);

  const list = document.createElement("ul");
  list.className = "repository-rule__packages";
  for (const pkg of packages) {
    const item = document.createElement("li");
    const name = document.createElement("span");
    name.className = "repository-rule__package";
    name.textContent = `${pkg.name} ${pkg.version}`;
    item.appendChild(name);
    pkg.advisories.forEach((id, i) => {
      item.append(i === 0 ? " — " : ", ");
      const a = document.createElement("a");
      a.href = `https://osv.dev/vulnerability/${encodeURIComponent(id)}`;
      a.target = "_blank";
      a.rel = "noopener";
      a.textContent = id;
      item.appendChild(a);
    });
    list.appendChild(item);
  }
  details.appendChild(list);
  return details;
}
