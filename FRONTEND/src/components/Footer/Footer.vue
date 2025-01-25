

<script setup lang="ts">

  const appVersion = window.APP_VERSION

  const links = {
    gh: window.LINKS__GH,
    src: window.LINKS__SRC,
    api: window.LINKS__API,
  }

</script>




<template>
  
  <footer class="footer-wrapper">
    <div class="footer">
      Made by <a :href="links.gh" class="_gh">@4Tipsy</a>. Current client version: "{{appVersion}}" | <a :href="links.src">SRC</a> <a :href="links.api">API</a>
    </div>
  </footer>

</template>




<style scoped lang="scss">

  .footer-wrapper {
    display: flex;
    justify-content: center;

    font-size: 16px;
    color: var(--text-color-1);
  }

  .footer {
    width: var(--content-width);
    padding: 10px 0;


    > a {
      color: inherit;
    }
  }

  ._gh {
    color: var(--text-color-2) !important;
  }

</style>
