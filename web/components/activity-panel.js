import { renderAvatarStack } from "./avatar-stack.js";

const PERIODS = [
  { key: "last6Months", label: "Laatste 6 maanden" },
  { key: "months6to12", label: "6 tot 12 maanden geleden" },
  { key: "olderThan12Months", label: "Ouder dan 12 maanden" },
];

/** Renders the three contributor/PR activity panels into `container`. */
export function renderActivityPanels(container, activity) {
  const template = document.getElementById("tpl-activity-panel");
  container.innerHTML = "";

  for (const { key, label } of PERIODS) {
    const period = activity?.[key] || { count: 0, contributors: [], prsClosed: 0 };
    const node = template.content.cloneNode(true);
    node.querySelector(".activity-panel__label").textContent = label;
    node.querySelector(".activity-panel__contributors").textContent = period.count;
    node.querySelector(".activity-panel__prs").textContent = period.prsClosed;
    renderAvatarStack(node.querySelector(".activity-panel__avatars"), period.contributors, Infinity);
    container.appendChild(node);
  }
}
