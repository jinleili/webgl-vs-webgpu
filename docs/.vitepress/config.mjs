import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "WebGL 与 WebGPU 性能对比",
  description: "不具有太大参考价值",
  base: "/webgl-vs-webgpu/",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "WebGL vs WebGPU", link: "/" },
      { text: "代码说明", link: "/code" },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/jinleili/webgl-vs-webgpu" },
    ],
  },
});
