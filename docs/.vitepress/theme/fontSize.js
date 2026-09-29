export const SIZE_ORDER = ["small", "medium", "large"];

export const SIZE_LABELS = { small: "Small", medium: "Medium", large: "Large" };

export const SAMPLE_SIZES = { small: "11px", medium: "13px", large: "14px" };

export const TEXT_SIZES = { small: "14px", medium: "16px", large: "18px" };

export const SAMPLE_KEY = "datasources-sample-size";

export const TEXT_KEY = "datasources-text-size";

export const fontSizeBoot = `(() => {
  try {
    const samples = ${JSON.stringify(SAMPLE_SIZES)};
    const texts = ${JSON.stringify(TEXT_SIZES)};
    const sample = localStorage.getItem(${JSON.stringify(SAMPLE_KEY)});
    const text = localStorage.getItem(${JSON.stringify(TEXT_KEY)});
    const root = document.documentElement;
    root.style.setProperty("--docs-sample-size", sample in samples ? samples[sample] : samples.medium);
    root.style.setProperty("--docs-text-size", text in texts ? texts[text] : texts.medium);
  } catch {}
})();`;
