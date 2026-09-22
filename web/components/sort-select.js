/**
 * Renders the catalog sort dropdown into `container`, pre-selected to
 * `value`, and calls `onChange(value)` whenever the user picks a different
 * sort order.
 */
export function mountSortSelect(container, { value, onChange }) {
  const template = document.getElementById("tpl-sort-select");
  const node = template.content.cloneNode(true);
  container.appendChild(node);

  const select = container.querySelector(".sort-select__input");
  select.value = value;
  select.addEventListener("change", () => onChange(select.value));
}
