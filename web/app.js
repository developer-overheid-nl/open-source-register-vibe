/**
 * Progressive enhancement for the catalog page. The server already rendered
 * the complete page — filters, sort select, project cards with data-*
 * attributes, empty state — so this script never fetches data; it only wires
 * the interactive behavior (live search, filtering, sorting) onto that DOM.
 * Tag chips/pills are real links (`/?tag=…`) that also work without JS; here
 * we intercept their clicks to filter instantly without a page load. The
 * language filter follows the same pattern: the server pre-checks and
 * pre-filters on `?language=…`; this script wires the live filtering plus the
 * type-ahead search and "toon meer" toggle over the rendered checkboxes.
 */

const DEBOUNCE_MS = 250;
const VISIBLE_TAG_LIMIT = 15;
const VISIBLE_LANGUAGE_LIMIT = 10;

const listEl = document.getElementById("project-list-cards");
const emptyState = document.getElementById("empty-state");
const tagListEl = document.getElementById("tag-filter-list");
const pillsEl = document.getElementById("active-tag-pills");

const cards = Array.from((listEl ?? document).querySelectorAll("li.project-card")).map((el) => ({
  el,
  search: el.dataset.search ?? "",
  tags: JSON.parse(el.dataset.tags || "[]"),
  status: el.dataset.status || "",
  org: el.dataset.org || "",
  type: el.dataset.softwareType || "",
  languages: JSON.parse(el.dataset.languages || "[]"),
  name: el.dataset.name || "",
  commits: Number(el.dataset.commits || 0),
  contributors: Number(el.dataset.contributors || 0),
  dependencies: Number(el.dataset.dependencies || 0),
}));

const params = new URLSearchParams(window.location.search);

const state = {
  query: "",
  tags: new Set(params.getAll("tag").filter(Boolean)),
  statuses: new Set(),
  orgs: new Set(),
  softwareTypes: new Set(),
  languages: new Set(params.getAll("language").filter(Boolean)),
  sort: "name-asc",
};

const compareName = (a, b) => a.name.localeCompare(b.name);
const SORTERS = {
  "name-asc": compareName,
  "commits-desc": (a, b) => b.commits - a.commits || compareName(a, b),
  "commits-asc": (a, b) => a.commits - a.commits || compareName(a, b),
  "contributors-desc": (a, b) => b.contributors - a.contributors || compareName(a, b),
  "contributors-asc": (a, b) => a.contributors - b.contributors || compareName(a, b),
  "dependencies-desc": (a, b) => b.dependencies - a.dependencies || compareName(a, b),
  "dependencies-asc": (a, b) => a.dependencies - b.dependencies || compareName(a, b),
};

function matches(card) {
  if (state.query && !card.search.includes(state.query)) return false;
  if (state.tags.size > 0 && ![...state.tags].every((tag) => card.tags.includes(tag))) return false;
  if (state.statuses.size > 0 && !state.statuses.has(card.status)) return false;
  if (state.orgs.size > 0 && !state.orgs.has(card.org)) return false;
  if (state.softwareTypes.size > 0 && !state.softwareTypes.has(card.type)) return false;
  if (state.languages.size > 0 && !card.languages.some((language) => state.languages.has(language))) {
    return false;
  }
  return true;
}

function apply() {
  let visible = 0;
  for (const card of cards) {
    const ok = matches(card);
    card.el.hidden = !ok;
    if (ok) visible += 1;
  }
  if (listEl) {
    [...cards].sort(SORTERS[state.sort]).forEach((card) => listEl.appendChild(card.el));
  }
  emptyState.hidden = visible > 0;
}

/** Catalog URL for a set of active tags and languages — mirrors the server. */
function catalogUrl(tags, languages) {
  const params = [];
  for (const tag of tags) params.push(`tag=${encodeURIComponent(tag)}`);
  for (const language of languages) params.push(`language=${encodeURIComponent(language)}`);
  return params.length === 0 ? "/" : `/?${params.join("&")}`;
}

function createPill(tag) {
  const pill = document.createElement("a");
  pill.className = "tag-pill";
  pill.dataset.tag = tag;
  const next = new Set(state.tags);
  next.delete(tag);
  pill.href = catalogUrl(next, state.languages);

  const label = document.createElement("span");
  label.className = "tag-pill__label";
  label.textContent = tag;
  const remove = document.createElement("span");
  remove.className = "tag-pill__remove";
  remove.setAttribute("aria-hidden", "true");
  remove.textContent = "×";
  pill.append(label, remove);
  return pill;
}

/** Keeps chip state/hrefs, pills and the URL in sync with `state.tags`. */
function syncTagUi() {
  if (tagListEl) {
    for (const chip of tagListEl.querySelectorAll("a.tag-chip")) {
      const tag = chip.dataset.tag;
      const active = state.tags.has(tag);
      chip.classList.toggle("tag-chip--active", active);
      const next = new Set(state.tags);
      active ? next.delete(tag) : next.add(tag);
      chip.href = catalogUrl(next, state.languages);
      chip.hidden = !tagsExpanded && chipIndex(chip) >= VISIBLE_TAG_LIMIT && !active;
    }
  }
  updateToggleButton();
  pillsEl.innerHTML = "";
  for (const tag of state.tags) pillsEl.appendChild(createPill(tag));
  history.replaceState(null, "", catalogUrl(state.tags, state.languages));
}

const tagChips = tagListEl ? Array.from(tagListEl.querySelectorAll("a.tag-chip")) : [];
const tagChipIndex = new Map(tagChips.map((chip, i) => [chip, i]));
const chipIndex = (chip) => tagChipIndex.get(chip) ?? 0;
let tagsExpanded = false;
let toggleButton = null;

function updateToggleButton() {
  if (tagChips.length <= VISIBLE_TAG_LIMIT) return;
  if (!toggleButton) {
    toggleButton = document.createElement("button");
    toggleButton.type = "button";
    toggleButton.className = "tag-filter__toggle";
    toggleButton.addEventListener("click", () => {
      tagsExpanded = !tagsExpanded;
      syncTagUi();
    });
    tagListEl.appendChild(toggleButton);
  }
  const hiddenCount = tagChips.length - VISIBLE_TAG_LIMIT;
  toggleButton.textContent = tagsExpanded ? "Toon minder" : `Toon meer (${hiddenCount})`;
}

function toggleTag(tag) {
  if (state.tags.has(tag)) {
    state.tags.delete(tag);
  } else {
    state.tags.add(tag);
  }
  syncTagUi();
  apply();
}

function onTagClick(event) {
  const chip = event.target.closest("a.tag-chip, a.tag-pill");
  if (!chip) return;
  event.preventDefault();
  toggleTag(chip.dataset.tag);
}
tagListEl?.addEventListener("click", onTagClick);
pillsEl?.addEventListener("click", onTagClick);
document.getElementById("project-list")?.addEventListener("click", onTagClick);

const searchInput = document.getElementById("search-bar-input");
let debounceTimer = null;
searchInput?.addEventListener("input", () => {
  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(() => {
    state.query = searchInput.value.trim().toLowerCase();
    apply();
  }, DEBOUNCE_MS);
});

document.getElementById("sort-select-input")?.addEventListener("change", (event) => {
  state.sort = event.target.value;
  apply();
});

function wireCheckboxGroup(id, set) {
  document.getElementById(id)?.addEventListener("change", (event) => {
    const checkbox = event.target.closest("input.checkbox-filter__checkbox");
    if (!checkbox) return;
    if (checkbox.checked) {
      set.add(checkbox.value);
    } else {
      set.delete(checkbox.value);
    }
    apply();
  });
}
wireCheckboxGroup("status-filter-options", state.statuses);
wireCheckboxGroup("org-filter-options", state.orgs);
wireCheckboxGroup("software-type-filter-options", state.softwareTypes);
wireCheckboxGroup("language-filter-options", state.languages);

/**
 * Enhances the server-rendered language filter with the type-ahead search and
 * the "toon meer" toggle: without JavaScript every language checkbox is
 * simply visible (and `?language=…` links keep working); with it, only the
 * first 10 languages show until expanded, an active language always stays
 * visible, and the search field narrows the options live.
 */
function wireLanguageFilter() {
  const container = document.getElementById("language-filter-options");
  if (!container) return;

  const searchInput = container.querySelector(".checkbox-filter__search");
  const noMatches = container.querySelector(".checkbox-filter__no-matches");
  const toggleButton = container.querySelector(".checkbox-filter__toggle");
  const options = Array.from(container.querySelectorAll("label.checkbox-filter__option"));
  const optionsText = options.map((option) =>
    (option.querySelector(".checkbox-filter__label")?.textContent ?? "").trim().toLowerCase()
  );
  const optionActive = (i) =>
    options[i].querySelector("input.checkbox-filter__checkbox")?.checked ?? false;

  const hiddenCount = options.length - VISIBLE_LANGUAGE_LIMIT;
  let expanded = false;
  let query = "";

  const applyVisibility = () => {
    let matchCount = 0;
    options.forEach((option, i) => {
      const visible = query
        ? optionsText[i].includes(query)
        : expanded || i < VISIBLE_LANGUAGE_LIMIT || optionActive(i);
      option.hidden = !visible;
      if (visible) matchCount += 1;
    });
    if (noMatches) {
      noMatches.hidden = !query || matchCount > 0;
    }
    if (toggleButton) {
      toggleButton.hidden = Boolean(query) || hiddenCount <= 0;
      toggleButton.textContent = expanded ? "Toon minder" : `Toon meer (${hiddenCount})`;
      toggleButton.setAttribute("aria-expanded", String(expanded));
    }
  };

  searchInput?.removeAttribute("hidden");
  searchInput?.addEventListener("input", () => {
    query = searchInput.value.trim().toLowerCase();
    applyVisibility();
  });
  toggleButton?.addEventListener("click", () => {
    expanded = !expanded;
    applyVisibility();
  });
  // Re-apply after every toggle: an activated language beyond the limit must
  // stay visible, even when the list is collapsed again.
  container.addEventListener("change", applyVisibility);

  applyVisibility();
}
wireLanguageFilter();

syncTagUi();
apply();
