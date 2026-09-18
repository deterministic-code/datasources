import DefaultTheme from "vitepress/theme";
import DialectBoxes from "./DialectBoxes.vue";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("DialectBoxes", DialectBoxes);
  },
};
