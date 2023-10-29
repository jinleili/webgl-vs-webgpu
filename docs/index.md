---
# https://vitepress.dev/reference/default-theme-home-page
layout: page
sidebar: false
---

 <script type="module">
    var can_resize_canvas = true
    // Called by rust
    window.canvas_resize_completed = function () {
      can_resize_canvas = true;
    }
</script>

<div class="container">
  <div class="gl-container">
    <web-gl />
    <web-gpu />
  </div>
</div>
