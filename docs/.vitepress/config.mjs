import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "最糟实践的性能对比",
  description: "不具有太大参考价值",
  base: "/webgl-vs-webgpu/",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [{ text: "WebGL vs WebGPU", link: "/" }],
    socialLinks: [
      { icon: "github", link: "https://github.com/jinleili/webgl-vs-webgpu" },
    ],
  },
});
