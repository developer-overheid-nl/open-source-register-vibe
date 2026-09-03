const MONTH_LABELS = [
  "jan", "feb", "mrt", "apr", "mei", "jun", "jul", "aug", "sep", "okt", "nov", "dec",
];
const MONTH_NAMES_FULL = [
  "januari", "februari", "maart", "april", "mei", "juni",
  "juli", "augustus", "september", "oktober", "november", "december",
];
const HEAT_LEVELS = 4;

function formatDutchDate(isoDate) {
  const [year, month, day] = isoDate.split("-").map(Number);
  return `${day} ${MONTH_NAMES_FULL[month - 1]} ${year}`;
}

/**
 * Renders a GitHub-profile-style commit calendar (one square per day, shaded
 * by activity) into `container`, labelled with the branch the data covers.
 */
export function renderCommitGraph(container, commitActivity) {
  container.innerHTML = "";

  if (!commitActivity || !commitActivity.available || !commitActivity.days?.length) {
    const p = document.createElement("p");
    p.className = "commit-graph__empty-state";
    p.textContent = "Geen commit-activiteit beschikbaar voor dit project.";
    container.appendChild(p);
    return;
  }

  const { branch, days } = commitActivity;
  const totalCommits = days.reduce((sum, day) => sum + day.count, 0);
  const maxCount = Math.max(...days.map((day) => day.count), 1);

  const label = document.createElement("p");
  label.className = "commit-graph__label";
  label.append(`${totalCommits} commits in de laatste 12 maanden, op branch `);
  const branchEl = document.createElement("code");
  branchEl.className = "commit-graph__branch";
  branchEl.textContent = branch;
  label.appendChild(branchEl);
  container.appendChild(label);

  const scroller = document.createElement("div");
  scroller.className = "commit-graph__scroller";

  const grid = document.createElement("div");
  grid.className = "commit-graph__grid";
  grid.setAttribute("role", "img");
  grid.setAttribute(
    "aria-label",
    `Commit-kalender over de laatste 12 maanden op branch ${branch}, in totaal ${totalCommits} commits.`
  );

  let lastMonth = null;
  for (let i = 0; i < days.length; i += 7) {
    const week = days.slice(i, i + 7);
    const column = document.createElement("div");
    column.className = "commit-graph__week";

    const [, monthStr] = week[0].date.split("-");
    const month = Number(monthStr) - 1;
    if (month !== lastMonth) {
      const monthLabel = document.createElement("span");
      monthLabel.className = "commit-graph__month-label";
      monthLabel.textContent = MONTH_LABELS[month];
      column.appendChild(monthLabel);
      lastMonth = month;
    }

    for (const day of week) {
      const square = document.createElement("span");
      const level = day.count === 0 ? 0 : Math.min(HEAT_LEVELS, Math.ceil((day.count / maxCount) * HEAT_LEVELS));
      square.className = `commit-graph__day commit-graph__day--level-${level}`;
      square.title = `${day.count} commit${day.count === 1 ? "" : "s"} op ${formatDutchDate(day.date)}`;
      column.appendChild(square);
    }

    grid.appendChild(column);
  }

  scroller.appendChild(grid);
  container.appendChild(scroller);

  const legend = document.createElement("div");
  legend.className = "commit-graph__legend";
  const lessLabel = document.createElement("span");
  lessLabel.textContent = "Minder";
  legend.appendChild(lessLabel);
  for (let level = 0; level <= HEAT_LEVELS; level++) {
    const swatch = document.createElement("span");
    swatch.className = `commit-graph__day commit-graph__day--level-${level}`;
    legend.appendChild(swatch);
  }
  const moreLabel = document.createElement("span");
  moreLabel.textContent = "Meer";
  legend.appendChild(moreLabel);
  container.appendChild(legend);
}
