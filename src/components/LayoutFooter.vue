<template>
  <div class="container">
    <div class="author">
      <span>BeenYan</span>
    </div>

    <div class="Link">
      <span><a href="https://www.pixiv.net/users/37673370" target="_blank">Pixiv</a></span>
      <span> ⭐ </span>
      <span><a href="https://ko-fi.com/beenyan" target="_blank">Ko-Fi</a></span>
      <span> ⭐ </span>
      <span><a href="https://twitter.com/Been_Yan" target="_blank">X (Twitter)</a></span>
    </div>

    <div class="version">
      <span>{{ appVersion }}</span>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { getVersion } from '@tauri-apps/api/app'
import { onBeforeMount, ref } from 'vue'

const appVersion = ref('v0.0.0')

onBeforeMount(async () => {
  appVersion.value = `v${await getVersion()}`
})
</script>

<style lang="scss" scoped>
.container {
  height: 0;
  margin: 0;
  width: 100vw;
  align-self: center;
  background-color: var(--footer-background-color);
  flex-direction: row;
  justify-content: flex-end;
  padding: 20px;
  box-sizing: border-box;
  justify-content: space-between;

  div {
    align-items: center;
  }
}
</style>
