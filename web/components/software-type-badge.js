import { getSoftwareTypeIcon } from "./software-type-icons.js";

const SVG_NS = "http://www.w3.org/2000/svg";

/**
 * Renders a badge (icon + raw softwareType value) into `container`. Shows a
 * muted "onbekend" badge with a generic icon when `softwareType` is missing.
 */
export function renderSoftwareTypeBadge(container, softwareType) {
  container.innerHTML = "";

  const badge = document.createElement("span");
  badge.className = "software-type-badge";
  if (!softwareType) {
    badge.classList.add("software-type-badge--unknown");
  }

  const svg = document.createElementNS(SVG_NS, "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("aria-hidden", "true");
  svg.classList.add("software-type-badge__icon");
  svg.innerHTML = getSoftwareTypeIcon(softwareType);
  badge.appendChild(svg);

  const label = document.createElement("span");
  label.className = "software-type-badge__label";
  label.textContent = softwareType || "onbekend";
  badge.appendChild(label);

  container.appendChild(badge);
}
