import { h } from "vue";
import DefaultTheme from "vitepress/theme";
import DialectBoxes from "./DialectBoxes.vue";
import DialectName from "./DialectName.vue";
import DialectPanel from "./DialectPanel.vue";
import DialectSelector from "./DialectSelector.vue";
import ExampleOutputs from "./ExampleOutputs.vue";
import ExampleSchema from "./ExampleSchema.vue";
import ExampleSource from "./ExampleSource.vue";
import FontSize from "./FontSize.vue";
import SkillPrompt from "./SkillPrompt.vue";
import { loadFontSize } from "./fontSizeState.js";
import "./custom.css";

export default {
  extends: DefaultTheme,
  Layout() {
    return h(DefaultTheme.Layout, null, {
      "nav-bar-content-after": () => h(FontSize),
    });
  },
  enhanceApp({ app }) {
    if (typeof window !== "undefined") loadFontSize();
    app.component("DialectBoxes", DialectBoxes);
    app.component("DialectName", DialectName);
    app.component("DialectPanel", DialectPanel);
    app.component("DialectSelector", DialectSelector);
    app.component("ExampleOutputs", ExampleOutputs);
    app.component("ExampleSchema", ExampleSchema);
    app.component("ExampleSource", ExampleSource);
    app.component("SkillPrompt", SkillPrompt);
  },
};
