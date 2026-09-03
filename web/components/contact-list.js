/** Renders one card per contact into `container`, or a fallback message. */
export function renderContactList(container, contacts) {
  container.innerHTML = "";

  if (!contacts || contacts.length === 0) {
    const p = document.createElement("p");
    p.className = "contact-list__empty";
    p.textContent = "Geen contactinformatie beschikbaar.";
    container.appendChild(p);
    return;
  }

  const template = document.getElementById("tpl-contact");
  for (const contact of contacts) {
    const node = template.content.cloneNode(true);
    node.querySelector(".contact-card__name").textContent = contact.name || "Onbekend";

    const emailEl = node.querySelector(".contact-card__email");
    if (contact.email) {
      const link = document.createElement("a");
      link.href = `mailto:${contact.email}`;
      link.textContent = contact.email;
      emailEl.appendChild(link);
    } else {
      emailEl.remove();
    }

    const affiliationEl = node.querySelector(".contact-card__affiliation");
    if (contact.affiliation) {
      affiliationEl.textContent = contact.affiliation;
    } else {
      affiliationEl.remove();
    }

    container.appendChild(node);
  }
}
