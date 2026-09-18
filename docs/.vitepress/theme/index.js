import DefaultTheme from "vitepress/theme";
import DialectBoxes from "./DialectBoxes.vue";
import "./custom.css";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("DialectBoxes", DialectBoxes);
  },
};
