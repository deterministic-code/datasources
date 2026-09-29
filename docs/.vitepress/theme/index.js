import { h } from "vue";
import DefaultTheme from "vitepress/theme";
import DialectBoxes from "./DialectBoxes.vue";
import DialectName from "./DialectName.vue";
import DialectPanel from "./DialectPanel.vue";
import DialectSelector from "./DialectSelector.vue";
import ExampleOutputs from "./ExampleOutputs.vue";
import ExampleSource from "./ExampleSource.vue";
import FontSize from "./FontSize.vue";
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
    app.component("ExampleSource", ExampleSource);
  },
};
