export const parseToc = (text) => {
  const title = text.match(/^title:\s*(.+)$/m)?.[1]?.trim() ?? "Datasources";
  const pages = [...text.matchAll(/^\s+- path:\s+(\S+)((?:\n[ \t]+\S+:.*)*)/gm)].map(([, path, rest]) => {
    const field = (name) => rest.match(new RegExp(`^[ \\t]+${name}:\\s+(.+)$`, "m"))?.[1]?.trim();
    return {
      path,
      title: field("title") ?? path,
      group: field("group"),
      hash: field("hash"),
      parent: field("parent"),
      collapsed: field("collapsed") === "true",
    };
  });
  return { title, pages };
};

export const uniquePages = (pages) => {
  const seen = new Set();
  return pages.filter(({ path }) => (seen.has(path) ? false : (seen.add(path), true)));
};

export const sidebarTitle = (title) =>
  title
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1 $2");

export const nestSidebar = (pages, toItem) => {
  const kids = new Map();
  for (const page of pages) {
    if (!page.parent) continue;
    kids.set(page.parent, [...(kids.get(page.parent) ?? []), page]);
  }
  const nest = (page) => {
    const items = page.parent === page.title ? undefined : kids.get(page.title);
    const item = toItem(page);
    return items
      ? { ...item, collapsed: page.collapsed === true, items: items.map(nest) }
      : item;
  };
  return pages.filter((page) => !page.parent).map(nest);
};
