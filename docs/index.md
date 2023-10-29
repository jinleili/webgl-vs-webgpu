---
# https://vitepress.dev/reference/default-theme-home-page
layout: home
sidebar: false
---

<script type="module">
    var can_resize_canvas = true
    // Called by rust
    // 写在这 build 时会报错
    // window.canvas_resize_completed = function () {
    //   can_resize_canvas = true;
    // }
</script>

<div class="container">
  <div class="gl-container">
    <web-gl />
    <web-gpu />
  </div>
</div>
