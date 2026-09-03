const DEFAULT_MAX_AVATARS = 10;

/**
 * Renders an overlapping row of circular contributor avatars into
 * `container` (GitHub-style), capped at `max` with a "+N" overflow
 * indicator for the rest.
 */
export function renderAvatarStack(container, contributors, max = DEFAULT_MAX_AVATARS) {
  container.innerHTML = "";
  if (!contributors || contributors.length === 0) return;

  const visible = contributors.slice(0, max);
  const overflow = contributors.length - visible.length;

  for (const { login, avatarUrl } of visible) {
    const img = document.createElement("img");
    img.className = "avatar-stack__avatar";
    img.src = avatarUrl;
    img.alt = login;
    img.title = login;
    img.loading = "lazy";
    container.appendChild(img);
  }

  if (overflow > 0) {
    const more = document.createElement("span");
    more.className = "avatar-stack__avatar avatar-stack__overflow";
    more.textContent = `+${overflow}`;
    more.title = `${overflow} meer`;
    container.appendChild(more);
  }
}
