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
    };
  });
  return { title, pages };
};

export const uniquePages = (pages) => {
  const seen = new Set();
  return pages.filter(({ path }) => (seen.has(path) ? false : (seen.add(path), true)));
};
