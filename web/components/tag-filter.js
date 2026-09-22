const VISIBLE_LIMIT = 15;

/**
 * Renders a clickable chip per tag (with its project count, most popular
 * first) into `listEl`, and removable pills for the currently active tags
 * into `pillsEl`. Only the top `VISIBLE_LIMIT` tags are shown initially,
 * plus a "toon meer" toggle to reveal the rest — an already-active tag
 * always stays visible, even beyond the limit. `isActive(tag)` decides chip
 * state; `onToggle(tag)` is called on any click (chip or pill).
 *
 * Returns `{ update }` — call after the active-tag set changes elsewhere so
 * chip/pill state stays in sync without rebuilding the whole list.
 */
export function mountTagFilter(listEl, pillsEl, { tags, counts, isActive, onToggle }) {
  const chipTemplate = document.getElementById("tpl-tag-chip");
  const pillTemplate = document.getElementById("tpl-tag-pill");

  listEl.innerHTML = "";
  const chipButtons = new Map();
  let expanded = false;

  for (const tag of tags) {
    const node = chipTemplate.content.cloneNode(true);
    const button = node.querySelector(".tag-chip");
    button.querySelector(".tag-chip__label").textContent = tag;
    button.querySelector(".tag-chip__count").textContent = counts?.[tag] ?? "";
    button.addEventListener("click", () => onToggle(tag));
    listEl.appendChild(node);
    chipButtons.set(tag, listEl.lastElementChild);
  }

  let toggleButton = null;
  const hiddenCount = tags.length - VISIBLE_LIMIT;
  if (hiddenCount > 0) {
    toggleButton = document.createElement("button");
    toggleButton.type = "button";
    toggleButton.className = "tag-filter__toggle";
    toggleButton.addEventListener("click", () => {
      expanded = !expanded;
      applyVisibility();
    });
    listEl.appendChild(toggleButton);
  }

  function applyVisibility() {
    tags.forEach((tag, i) => {
      chipButtons.get(tag).hidden = !expanded && i >= VISIBLE_LIMIT && !isActive(tag);
    });
    if (toggleButton) {
      toggleButton.textContent = expanded ? "Toon minder" : `Toon meer (${hiddenCount})`;
    }
  }

  function update() {
    for (const [tag, button] of chipButtons) {
      button.classList.toggle("tag-chip--active", isActive(tag));
      button.setAttribute("aria-pressed", String(isActive(tag)));
    }
    applyVisibility();

    pillsEl.innerHTML = "";
    for (const tag of tags) {
      if (!isActive(tag)) continue;
      const node = pillTemplate.content.cloneNode(true);
      node.querySelector(".tag-pill__label").textContent = tag;
      node.querySelector(".tag-pill").addEventListener("click", () => onToggle(tag));
      pillsEl.appendChild(node);
    }
  }

  update();
  return { update };
}
