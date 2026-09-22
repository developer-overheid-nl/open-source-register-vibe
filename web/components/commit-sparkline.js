const DAYS_PER_WEEK = 7;
const SVG_NS = "http://www.w3.org/2000/svg";
const WIDTH = 100;
const HEIGHT = 30;

function aggregateWeeks(days) {
  const weeks = [];
  for (let i = 0; i < days.length; i += DAYS_PER_WEEK) {
    const chunk = days.slice(i, i + DAYS_PER_WEEK);
    weeks.push(chunk.reduce((sum, day) => sum + day.count, 0));
  }
  return weeks;
}

/**
 * Renders a small SVG line graph of weekly commit activity over the last 12
 * months (like GitHub's "graphs/commit-activity" chart, scaled down for use
 * on a compact card) into `container`. Renders nothing if no data is
 * available, so it doesn't add empty-state clutter to every card.
 */
export function renderCommitSparkline(container, commitActivity) {
  container.innerHTML = "";

  if (!commitActivity || !commitActivity.available || !commitActivity.days?.length) {
    return;
  }

  const weeks = aggregateWeeks(commitActivity.days);
  const totalCommits = weeks.reduce((sum, count) => sum + count, 0);
  const maxCount = Math.max(...weeks, 1);

  const points = weeks.map((count, i) => {
    const x = weeks.length > 1 ? (i / (weeks.length - 1)) * WIDTH : WIDTH / 2;
    const y = HEIGHT - (count / maxCount) * (HEIGHT - 2) - 1;
    return [x, y];
  });
  const linePath = points
    .map(([x, y], i) => `${i === 0 ? "M" : "L"}${x.toFixed(1)},${y.toFixed(1)}`)
    .join(" ");
  const areaPath = `${linePath} L${WIDTH},${HEIGHT} L0,${HEIGHT} Z`;

  const label = document.createElement("p");
  label.className = "commit-sparkline__label";
  label.textContent = `${totalCommits} commits (laatste 12 maanden)`;
  container.appendChild(label);

  const svg = document.createElementNS(SVG_NS, "svg");
  svg.setAttribute("viewBox", `0 0 ${WIDTH} ${HEIGHT}`);
  svg.setAttribute("preserveAspectRatio", "none");
  svg.setAttribute("role", "img");
  svg.setAttribute(
    "aria-label",
    `Wekelijkse commit-activiteit over de laatste 12 maanden, in totaal ${totalCommits} commits.`
  );
  svg.classList.add("commit-sparkline__svg");

  const area = document.createElementNS(SVG_NS, "path");
  area.setAttribute("d", areaPath);
  area.classList.add("commit-sparkline__area");
  svg.appendChild(area);

  const line = document.createElementNS(SVG_NS, "path");
  line.setAttribute("d", linePath);
  line.classList.add("commit-sparkline__line");
  svg.appendChild(line);

  container.appendChild(svg);
}
