export function renderEmptyState(container) {
  const template = document.getElementById("tpl-empty-state");
  container.innerHTML = "";
  container.appendChild(template.content.cloneNode(true));
}
