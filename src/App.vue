<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import LayoutFooter from './components/LayoutFooter.vue'
import UploadFile from './components/UploadFile.vue'
import DecodedImage from './components/DecodedImage.vue'
</script>

<template>
  <div class="container" @contextmenu.prevent>
    <header class="header"></header>

    <div class="body">
      <UploadFile class="input"></UploadFile>
      <DecodedImage class="output"></DecodedImage>
    </div>

    <LayoutFooter class="footer"></LayoutFooter>
  </div>
</template>

<style scoped lang="scss">
.container {
  display: flex;
  height: 100%;
  box-sizing: border-box;
  row-gap: 1rem;

  .header {
    height: 30px;
  }

  .body {
    column-gap: 1rem;
    display: flex;
    overflow-y: hidden;
    flex: 1;

    .input {
      flex: 6;
    }

    .output {
      flex: 4;
    }
  }

  .footer {
    height: 40px;
    background-color: var(--footer-background-color);
  }
}
</style>
