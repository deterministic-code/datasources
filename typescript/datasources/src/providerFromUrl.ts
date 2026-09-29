export const providerFromUrl = (url: string): string | undefined =>
  url.match(/^([a-zA-Z][a-zA-Z0-9+.-]*):/)?.[1]?.toLowerCase();
