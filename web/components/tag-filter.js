/**
 * Renders a clickable chip per tag into `listEl`, and removable pills for
 * the currently active tags into `pillsEl`. `isActive(tag)` decides chip
 * state; `onToggle(tag)` is called on any click (chip or pill).
 *
 * Returns `{ update }` — call after the active-tag set changes elsewhere so
 * chip/pill state stays in sync without rebuilding the whole list.
 */
export function mountTagFilter(listEl, pillsEl, { tags, isActive, onToggle }) {
  const chipTemplate = document.getElementById("tpl-tag-chip");
  const pillTemplate = document.getElementById("tpl-tag-pill");

  listEl.innerHTML = "";
  const chipButtons = new Map();

  for (const tag of tags) {
    const node = chipTemplate.content.cloneNode(true);
    const button = node.querySelector(".tag-chip");
    button.querySelector(".tag-chip__label").textContent = tag;
    button.addEventListener("click", () => onToggle(tag));
    listEl.appendChild(node);
    chipButtons.set(tag, listEl.lastElementChild);
  }

  function update() {
    for (const [tag, button] of chipButtons) {
      button.classList.toggle("tag-chip--active", isActive(tag));
      button.setAttribute("aria-pressed", String(isActive(tag)));
    }

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
