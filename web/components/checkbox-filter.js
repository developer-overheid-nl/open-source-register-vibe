/**
 * Renders a checkbox per option into `container` (used for both the
 * development-status and organisation filters). `isActive(value)` decides
 * checked state; `onToggle(value)` fires on change.
 */
export function mountCheckboxFilter(container, { options, isActive, onToggle }) {
  const template = document.getElementById("tpl-checkbox-option");
  container.innerHTML = "";

  for (const option of options) {
    const node = template.content.cloneNode(true);
    const checkbox = node.querySelector(".checkbox-filter__checkbox");
    checkbox.checked = isActive(option);
    node.querySelector(".checkbox-filter__label").textContent = option;
    checkbox.addEventListener("change", () => onToggle(option));
    container.appendChild(node);
  }
}
