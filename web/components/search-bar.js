const DEBOUNCE_MS = 250;

/**
 * Renders the catalog search field into `container` and calls
 * `onChange(query)` (debounced) whenever the query changes.
 */
export function mountSearchBar(container, { onChange }) {
  const template = document.getElementById("tpl-search-bar");
  const node = template.content.cloneNode(true);
  container.appendChild(node);

  const input = container.querySelector(".search-bar__input");
  let timer = null;

  input.addEventListener("input", () => {
    clearTimeout(timer);
    timer = setTimeout(() => onChange(input.value.trim()), DEBOUNCE_MS);
  });
}
