import { mountSearchBar } from "./components/search-bar.js";
import { mountSortSelect } from "./components/sort-select.js";
import { mountTagFilter } from "./components/tag-filter.js";
import { mountCheckboxFilter } from "./components/checkbox-filter.js";
import { renderProjectList } from "./components/project-list.js";

const state = {
  projects: [],
  query: "",
  activeTags: new Set(),
  activeStatuses: new Set(),
  activeOrgs: new Set(),
  activeSoftwareTypes: new Set(),
  sort: "name-asc",
};

let tagFilterHandle = null;

async function init() {
  const response = await fetch("data/projects.json");
  const data = await response.json();
  state.projects = data.projects;

  const params = new URLSearchParams(window.location.search);
  const tagFromUrl = params.get("tag");
  if (tagFromUrl) {
    state.activeTags.add(tagFromUrl);
  }

  mountFilters();

  mountSearchBar(document.getElementById("search-bar"), {
    onChange: (query) => {
      state.query = query.toLowerCase();
      render();
    },
  });

  mountSortSelect(document.getElementById("sort-select"), {
    value: state.sort,
    onChange: (sort) => {
      state.sort = sort;
      render();
    },
  });

  render();
}

function mountFilters() {
  const filtersContainer = document.getElementById("filters");
  const template = document.getElementById("tpl-filters");
  filtersContainer.appendChild(template.content.cloneNode(true));

  const tagCounts = {};
  for (const project of state.projects) {
    for (const tag of project.tags || []) {
      tagCounts[tag] = (tagCounts[tag] || 0) + 1;
    }
  }
  const allTags = Object.keys(tagCounts).sort(
    (a, b) => tagCounts[b] - tagCounts[a] || a.localeCompare(b)
  );
  const allStatuses = [
    ...new Set(state.projects.map((p) => p.developmentStatus).filter(Boolean)),
  ].sort();
  const allOrgs = [...new Set(state.projects.map((p) => p.org).filter(Boolean))].sort();
  const allSoftwareTypes = [
    ...new Set(state.projects.map((p) => p.softwareType).filter(Boolean)),
  ].sort();

  tagFilterHandle = mountTagFilter(
    document.getElementById("tag-filter-list"),
    document.getElementById("active-tag-pills"),
    {
      tags: allTags,
      counts: tagCounts,
      isActive: (tag) => state.activeTags.has(tag),
      onToggle: (tag) => toggleTag(tag),
    }
  );

  mountCheckboxFilter(document.getElementById("status-filter-options"), {
    options: allStatuses,
    isActive: (status) => state.activeStatuses.has(status),
    onToggle: (status) => {
      if (state.activeStatuses.has(status)) {
        state.activeStatuses.delete(status);
      } else {
        state.activeStatuses.add(status);
      }
      render();
    },
  });

  mountCheckboxFilter(document.getElementById("org-filter-options"), {
    options: allOrgs,
    isActive: (org) => state.activeOrgs.has(org),
    onToggle: (org) => {
      if (state.activeOrgs.has(org)) {
        state.activeOrgs.delete(org);
      } else {
        state.activeOrgs.add(org);
      }
      render();
    },
  });

  mountCheckboxFilter(document.getElementById("software-type-filter-options"), {
    options: allSoftwareTypes,
    isActive: (softwareType) => state.activeSoftwareTypes.has(softwareType),
    onToggle: (softwareType) => {
      if (state.activeSoftwareTypes.has(softwareType)) {
        state.activeSoftwareTypes.delete(softwareType);
      } else {
        state.activeSoftwareTypes.add(softwareType);
      }
      render();
    },
  });
}

function toggleTag(tag) {
  if (state.activeTags.has(tag)) {
    state.activeTags.delete(tag);
  } else {
    state.activeTags.add(tag);
  }
  render();
}

function matchesQuery(project) {
  if (!state.query) return true;
  const haystack = [project.name, project.description, ...(project.tags || [])]
    .join(" ")
    .toLowerCase();
  return haystack.includes(state.query);
}

function matchesTags(project) {
  if (state.activeTags.size === 0) return true;
  const projectTags = new Set(project.tags || []);
  return [...state.activeTags].every((tag) => projectTags.has(tag));
}

function matchesStatus(project) {
  if (state.activeStatuses.size === 0) return true;
  return state.activeStatuses.has(project.developmentStatus);
}

function matchesOrg(project) {
  if (state.activeOrgs.size === 0) return true;
  return state.activeOrgs.has(project.org);
}

function matchesSoftwareType(project) {
  if (state.activeSoftwareTypes.size === 0) return true;
  return state.activeSoftwareTypes.has(project.softwareType);
}

/** Total commits over the last 12 months, or 0 if no commit activity is known. */
function totalCommits(project) {
  const days = project.commitActivity?.available ? project.commitActivity.days : null;
  if (!days) return 0;
  return days.reduce((sum, day) => sum + day.count, 0);
}

const SORTERS = {
  "name-asc": (a, b) => a.name.localeCompare(b.name),
  "commits-desc": (a, b) => totalCommits(b) - totalCommits(a) || a.name.localeCompare(b.name),
  "commits-asc": (a, b) => totalCommits(a) - totalCommits(b) || a.name.localeCompare(b.name),
};

function render() {
  tagFilterHandle?.update();

  const filtered = state.projects
    .filter(
      (project) =>
        matchesQuery(project) &&
        matchesTags(project) &&
        matchesStatus(project) &&
        matchesOrg(project) &&
        matchesSoftwareType(project)
    )
    .sort(SORTERS[state.sort] || SORTERS["name-asc"]);

  renderProjectList(document.getElementById("project-list"), filtered, {
    onTagClick: (tag) => toggleTag(tag),
  });
}

init();
