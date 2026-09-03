import { mountSearchBar } from "./components/search-bar.js";
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

  render();
}

function mountFilters() {
  const filtersContainer = document.getElementById("filters");
  const template = document.getElementById("tpl-filters");
  filtersContainer.appendChild(template.content.cloneNode(true));

  const allTags = [...new Set(state.projects.flatMap((p) => p.tags || []))].sort();
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
    .sort((a, b) => a.name.localeCompare(b.name));

  renderProjectList(document.getElementById("project-list"), filtered, {
    onTagClick: (tag) => toggleTag(tag),
  });
}

init();
