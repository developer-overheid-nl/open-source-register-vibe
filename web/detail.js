/**
 * Progressive enhancement for the dependency browser on the project detail
 * page. The server already rendered one button per package manager and, per
 * package manager, the ranked list of its direct dependencies — so this
 * script never fetches data; it turns those lists into a tabbed view where
 * only the selected package manager's list is visible. Without JavaScript
 * every list simply stays visible below the buttons.
 */

const container = document.getElementById("dependency-list");

if (container) {
  const buttons = Array.from(container.querySelectorAll("button.dependency-row"));
  const panels = Array.from(container.querySelectorAll(".dependency-list__top"));

  function select(ecosystem) {
    for (const button of buttons) {
      button.setAttribute("aria-pressed", String(button.dataset.ecosystem === ecosystem));
    }
    for (const panel of panels) {
      panel.hidden = panel.dataset.ecosystem !== ecosystem;
    }
  }

  for (const button of buttons) {
    button.addEventListener("click", () => select(button.dataset.ecosystem));
  }

  // The server pre-selects the first package manager with known direct
  // dependencies; fall back to the first clickable button otherwise.
  const selected =
    buttons.find((button) => button.getAttribute("aria-pressed") === "true") ??
    buttons.find((button) => !button.disabled);
  if (selected) {
    select(selected.dataset.ecosystem);
  }
}
