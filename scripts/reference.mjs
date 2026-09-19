const splitAt = (text, regex) => {
  const matches = [...text.matchAll(regex)];
  return matches.map((match, i) => ({
    title: match[1].trim(),
    body: text.slice(match.index + match[0].length, i + 1 < matches.length ? matches[i + 1].index : text.length).trim(),
  }));
};

const parseMember = (body) => {
  let text = body.trim();
  let definition = "";
  text = text.replace(/^```[a-zA-Z0-9_+-]+[^\n]*\r?\n[\s\S]*?^```/m, (fence) => {
    definition = fence.trim();
    return "";
  });
  let returns = "";
  text = text.replace(/^\*\*Returns\*\*[ \t]+(.+)$/m, (_, value) => {
    returns = value.trim();
    return "";
  });
  const params = [];
  text = text.replace(/^- +(.+)$/gm, (_, item) => {
    params.push(item.trim());
    return "";
  });
  return { description: text.replace(/\n{3,}/g, "\n\n").trim(), definition, params, returns };
};

const parseParam = (item) => {
  const match = item.match(/^`([^`]+)`(?:[ \t]*—[ \t]*|[ \t]+-[ \t]+)(.+)$/);
  return match ? { name: match[1], description: match[2] } : { name: item, description: "" };
};

export const parseClasses = (body) =>
  splitAt(body, /^####[ \t]+(.+?)[ \t]*$/gm).map(({ title, body: raw }) => {
    let rest = raw;
    let definition = "";
    rest = rest.replace(/^```[\s\S]*?^```/m, (fence) => {
      definition = fence.trim();
      return "";
    });
    const marks = [...rest.matchAll(/^\*\*(Constructor|Properties|Methods)\*\*[ \t]*$/gm)];
    const groups = marks.map((mark, i) => {
      const chunk = rest.slice(mark.index + mark[0].length, i + 1 < marks.length ? marks[i + 1].index : rest.length);
      return {
        name: mark[1],
        members: splitAt(chunk, /^#####[ \t]+(.+?)[ \t]*$/gm).map(({ title: name, body }) => ({
          name,
          ...parseMember(body),
        })),
      };
    });
    return { name: title, definition, groups };
  });

const token = (value) =>
  value
    .toLowerCase()
    .replace(/[^\w]+/g, "-")
    .replace(/-+/g, "-")
    .replace(/^-|-$/g, "");

export const slug = (lang, className, member) =>
  [lang, token(className), member && token(member)].filter(Boolean).join("-");

const prefixHashes = (lang, markdown) =>
  markdown.replace(/\]\(([^)#]*?)#([^)]+)\)/g, (full, href, hash) =>
    href.startsWith("http") || hash.startsWith(`${lang}-`) ? full : `](${href}#${lang}-${hash})`,
  );

const summaryOf = (description) => (description.split(/\n\n/)[0] ?? "").replace(/\n/g, " ");

const renderIndex = (lang, cls) =>
  cls.groups
    .filter((group) => group.members.length)
    .map((group) => {
      const rows = group.members
        .map((member) => `| [${member.name}](#${slug(lang, cls.name, member.name)}) | ${summaryOf(member.description)} |`)
        .join("\n");
      return `**${group.name}**\n\n| Name | Description |\n| --- | --- |\n${rows}`;
    })
    .join("\n\n");

const renderDetail = (lang, cls, member) => {
  const id = slug(lang, cls.name, member.name);
  const definition = member.definition ? `\n\n**Definition**\n\n${member.definition}` : "";
  const params = member.params.length
    ? `\n\n**Parameters**\n\n| Name | Description |\n| --- | --- |\n${member.params
        .map((item) => {
          const { name, description } = parseParam(item);
          return `| \`${name}\` | ${description} |`;
        })
        .join("\n")}`
    : "";
  const returns = member.returns ? `\n\n**Returns**\n\n${member.returns}` : "";
  return `##### ${member.name} {#${id}}\n\n${member.description}${definition}${params}${returns}`;
};

export const renderClassReference = (body, lang) => {
  const rendered = parseClasses(body)
    .map((cls) => {
      const details = cls.groups
        .flatMap((group) => group.members.map((member) => renderDetail(lang, cls, member)))
        .join("\n\n");
      return [`#### ${cls.name} {#${slug(lang, cls.name)}}`, cls.definition, renderIndex(lang, cls), details]
        .filter(Boolean)
        .join("\n\n");
    })
    .join("\n\n");
  return prefixHashes(lang, rendered);
};
