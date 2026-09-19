import DefaultTheme from "vitepress/theme";
import DialectBoxes from "./DialectBoxes.vue";
import DialectPanel from "./DialectPanel.vue";
import DialectSelector from "./DialectSelector.vue";
import ExampleSource from "./ExampleSource.vue";
import "./custom.css";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("DialectBoxes", DialectBoxes);
    app.component("DialectPanel", DialectPanel);
    app.component("DialectSelector", DialectSelector);
    app.component("ExampleSource", ExampleSource);
  },
};
