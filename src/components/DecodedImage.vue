<template>
  <div class="container">
    <div class="header">
      <button @click="EasterEggs">Decode v2</button>
    </div>

    <div class="body">
      <div
        class="img-box"
        v-for="(img, index) of ImgDecoded.imgList"
        @click="SelectIndex = index"
        v-bind:class="{ select: SelectIndex === index }"
      >
        <div>{{ img.seed }}</div>
        <img class="encode-img" :src="img.message" alt="img.seed" />
      </div>
    </div>

    <div class="footer">
      <input class="path" type="text" v-model.trim="SavePath" />
      <button @click="SelectSavePath">Folder</button>
      <button @click="SaveImg">Save</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { appDataDir } from '@tauri-apps/api/path'
import { ImgDecodedStore } from '../stores/decodedImage'
import { open } from '@tauri-apps/api/dialog'
import { useToast } from 'vue-toast-notification'

const ImgDecoded = ImgDecodedStore()
const SelectIndex = ref(0)
const SavePath = ref('')
const Toast = useToast()
const Count = ref(0)

onMounted(async () => {
  SavePath.value = await appDataDir()
})

function EasterEggs() {
  if (++Count.value < 10) return
  Toast.success('Found Easter eggs')
  Count.value = 0
}

async function SelectSavePath() {
  // Open a selection dialog for image files
  const Path = await open({ directory: true })
  if (Path === null) return
  SavePath.value = Path as string
}

async function SaveImg() {
  const resultList = await ImgDecoded.saveImage(SavePath.value)

  for (const result of resultList) {
    if (result.success) {
      Toast.success(result.message)
    } else {
      Toast.error(result.message)
    }
  }
}
</script>

<style scoped>
.container {
  justify-content: space-between;

  .header {
    height: 40px;
    display: flex;
    column-gap: 1rem;
    justify-content: space-evenly;
  }

  .body {
    display: flex;
    border-radius: 8px;
    padding: 1rem;
    flex-direction: row;
    background-color: var(--footer-background-color);
    justify-content: space-evenly;
    row-gap: 10px;
    flex: 1;
    column-gap: 10px;
    flex-wrap: wrap;
    overflow-y: auto;

    .img-box {
      cursor: pointer;
      padding: 5px 10px;
      background-color: var(--seed-correct-background-color);
      border-radius: 5px;
      max-height: 300px;

      &.select {
        background-color: var(--image-select-background-color);
      }

      .encode-img {
        border: 1px solid black;
        max-height: 25vh;
        border-radius: 5px;
      }
    }
  }

  .footer {
    display: flex;
    column-gap: 1rem;
    height: 50px;

    .path {
      flex: 1;
    }
  }
}
</style>
