/**
 * Renders a checkbox per option into `container` (used for the development-
 * status, organisation, software-type and language filters). `isActive(value)`
 * decides checked state; `onToggle(value)` fires on change.
 *
 * With `visibleLimit`, only the first `visibleLimit` options are shown
 * initially, plus a "toon meer" toggle to reveal the rest — an already-active
 * option always stays visible, even beyond the limit.
 *
 * With `searchPlaceholder`, a type-ahead field is rendered above the options
 * that narrows them as you type (case-insensitive substring match). While a
 * search term is entered, all matching options are shown, regardless of
 * `visibleLimit`.
 */
export function mountCheckboxFilter(
  container,
  { options, isActive, onToggle, visibleLimit, searchPlaceholder }
) {
  const template = document.getElementById("tpl-checkbox-option");
  container.innerHTML = "";
  const optionEls = [];
  let expanded = false;
  let query = "";

  if (searchPlaceholder) {
    const searchInput = document.createElement("input");
    searchInput.type = "search";
    searchInput.className = "checkbox-filter__search";
    searchInput.placeholder = searchPlaceholder;
    searchInput.setAttribute("aria-label", searchPlaceholder);
    searchInput.addEventListener("input", () => {
      query = searchInput.value.trim().toLowerCase();
      applyVisibility();
    });
    container.appendChild(searchInput);
  }

  for (const option of options) {
    const node = template.content.cloneNode(true);
    const checkbox = node.querySelector(".checkbox-filter__checkbox");
    checkbox.checked = isActive(option);
    node.querySelector(".checkbox-filter__label").textContent = option;
    checkbox.addEventListener("change", () => onToggle(option));
    container.appendChild(node);
    optionEls.push(container.lastElementChild);
  }

  const noMatches = document.createElement("p");
  noMatches.className = "checkbox-filter__no-matches";
  noMatches.textContent = "Geen overeenkomende opties.";
  container.appendChild(noMatches);

  const hiddenCount = visibleLimit ? options.length - visibleLimit : 0;
  let toggleButton = null;
  if (hiddenCount > 0) {
    toggleButton = document.createElement("button");
    toggleButton.type = "button";
    toggleButton.className = "checkbox-filter__toggle";
    toggleButton.addEventListener("click", () => {
      expanded = !expanded;
      applyVisibility();
    });
    container.appendChild(toggleButton);
  }

  function applyVisibility() {
    let matchCount = 0;
    options.forEach((option, i) => {
      let visible;
      if (query) {
        visible = option.toLowerCase().includes(query);
      } else {
        visible = expanded || !toggleButton || i < visibleLimit || isActive(option);
      }
      optionEls[i].hidden = !visible;
      if (visible) matchCount++;
    });
    noMatches.hidden = !query || matchCount > 0;
    if (toggleButton) {
      toggleButton.hidden = Boolean(query);
      toggleButton.textContent = expanded ? "Toon minder" : `Toon meer (${hiddenCount})`;
      toggleButton.setAttribute("aria-expanded", String(expanded));
    }
  }

  applyVisibility();
}
