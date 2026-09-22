/**
 * Simple line icons (24x24, stroke-only, Rijkshuisstijl-style: thin strokes,
 * round joins, no fill) for each `softwareType` value from the publiccode.yml
 * schema (https://yml.publiccode.tools). Keyed on the raw schema value, since
 * the catalogus shows that value 1-op-1 without translation (see FEATURES.md).
 */
const SOFTWARE_TYPE_ICONS = {
  "standalone/mobile": `
    <rect x="7" y="2" width="10" height="20" rx="2"/>
    <line x1="11" y1="18" x2="13" y2="18"/>
  `,
  "standalone/iot": `
    <rect x="7" y="7" width="10" height="10" rx="1"/>
    <circle cx="12" cy="12" r="2"/>
    <line x1="12" y1="2" x2="12" y2="7"/>
    <line x1="12" y1="17" x2="12" y2="22"/>
    <line x1="2" y1="12" x2="7" y2="12"/>
    <line x1="17" y1="12" x2="22" y2="12"/>
  `,
  "standalone/desktop": `
    <rect x="3" y="4" width="18" height="12" rx="1"/>
    <line x1="8" y1="20" x2="16" y2="20"/>
    <line x1="12" y1="16" x2="12" y2="20"/>
  `,
  "standalone/web": `
    <circle cx="12" cy="12" r="9"/>
    <line x1="3" y1="12" x2="21" y2="12"/>
    <path d="M12 3c3 4 3 14 0 18"/>
    <path d="M12 3c-3 4-3 14 0 18"/>
  `,
  "standalone/backend": `
    <rect x="4" y="4" width="16" height="7" rx="1"/>
    <rect x="4" y="13" width="16" height="7" rx="1"/>
    <circle cx="8" cy="7.5" r="0.9" fill="currentColor" stroke="none"/>
    <circle cx="8" cy="16.5" r="0.9" fill="currentColor" stroke="none"/>
  `,
  "standalone/other": `
    <path d="M12 3l8 4.5v9L12 21l-8-4.5v-9L12 3z"/>
    <path d="M12 12v9"/>
    <path d="M4 7.5l8 4.5 8-4.5"/>
  `,
  addon: `
    <path d="M9 4.2h3.2a1 1 0 0 1 1 1v1.6a1.7 1.7 0 1 0 0 3.4v3.6a1 1 0 0 1-1 1h-1.6a1.7 1.7 0 1 1-3.4 0H5.2a1 1 0 0 1-1-1V11a1.7 1.7 0 1 0 0-3.4V5.2a1 1 0 0 1 1-1H9z"/>
  `,
  library: `
    <path d="M4 5a2 2 0 0 1 2-2h5v18H6a2 2 0 0 1-2-2V5z"/>
    <path d="M20 5a2 2 0 0 0-2-2h-5v18h5a2 2 0 0 0 2-2V5z"/>
  `,
  configurationFiles: `
    <line x1="4" y1="6" x2="20" y2="6"/>
    <circle cx="9" cy="6" r="2" fill="var(--color-bg)"/>
    <line x1="4" y1="12" x2="20" y2="12"/>
    <circle cx="15" cy="12" r="2" fill="var(--color-bg)"/>
    <line x1="4" y1="18" x2="20" y2="18"/>
    <circle cx="7" cy="18" r="2" fill="var(--color-bg)"/>
  `,
};

const DEFAULT_ICON = `
  <circle cx="12" cy="12" r="9"/>
  <path d="M9.5 9a2.5 2.5 0 1 1 3.5 2.3c-.8.4-1.5 1-1.5 2.2"/>
  <line x1="12" y1="17" x2="12" y2="17.01"/>
`;

/** Returns the inner SVG markup (paths/shapes, no <svg> wrapper) for `softwareType`. */
export function getSoftwareTypeIcon(softwareType) {
  return SOFTWARE_TYPE_ICONS[softwareType] || DEFAULT_ICON;
}
