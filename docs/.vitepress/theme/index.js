import DefaultTheme from "vitepress/theme";
import "./index.scss";

import WebGPU from "../components/WebGPU.vue";
import WebGL from "../components/WebGL.vue";

export default {
  ...DefaultTheme,
  enhanceApp({ app }) {
    app.component("web-gpu", WebGPU);
    app.component("web-gl", WebGL);
  },
};
