<template>
  <div class="container">
    <div class="header">
      <input
        class="seed"
        type="number"
        placeholder="Seed"
        min="818"
        :value="getSeed()"
        @input="(event) => setSeed(event)"
      />
      <button @click="decode(SelectIndex)">Decode</button>
      <button @click="decode(-1)">Decode All</button>
    </div>

    <div class="body">
      <div
        class="img-box"
        v-for="(img, index) of imageList"
        @click="SelectIndex = index"
        v-bind:class="{ select: SelectIndex === index }"
      >
        <div>{{ img.seed }}</div>
        <img class="encode-img" :src="img.url" alt="img.seed" />
      </div>
    </div>

    <div class="footer">
      <div class="upload-box">
        <span class="upload-text">Upload Image</span>
        <input
          class="upload-input"
          type="file"
          name="image and seed"
          accept=".png"
          @change="uploadFile"
          multiple
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { ImgDecodedStore, Result } from '../stores/decodedImage'
import { useToast } from 'vue-toast-notification'

const ImgDecoded = ImgDecodedStore()
const Toast = useToast()
const SelectIndex = ref(0)

class Image {
  seed: number
  url: string

  constructor(seed: number, url: string) {
    this.seed = seed
    this.url = url
  }

  get convert_backend_struct() {
    return {
      seed: this.seed,
      base64: this.url.split(',')[1],
    }
  }
}

const imageList: Array<Image> = reactive([])

function getSeed(): number {
  return imageList[SelectIndex.value]?.seed ?? 0
}

function setSeed(event: Event) {
  const Target = <HTMLInputElement>event.target
  if (Target === null || imageList.length === 0) return
  const Seed = parseInt(Target.value)
  if (isNaN(Seed)) return

  imageList[SelectIndex.value].seed = Seed
}

function getFileExtension(filename: String) {
  return filename.slice(((filename.lastIndexOf('.') - 1) >>> 0) + 2)
}

async function get_url(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const Reader = new FileReader()
    Reader.onload = (event) => {
      if (event.target === null || event.target.result === null) return
      const Url = event.target.result
      resolve(Url as string)
    }
    Reader.onerror = reject
    Reader.readAsDataURL(file)
  })
}

async function uploadFile(file: Event) {
  const Input = <HTMLInputElement>file.target
  if (Input.files === null) return

  for (const file of Input.files) {
    if (getFileExtension(file.name) !== 'png') continue
    const Seed = parseInt(file.name) || 0
    const Url = await get_url(file)

    imageList.push(new Image(Seed, Url))
  }
}

async function decode(index: number) {
  if (imageList.length === 0) return

  let data = { dataArray: [] as Array<any> }
  if (index === -1) {
    console.log('Decode All');
    
    data.dataArray = imageList.map((img) => img.convert_backend_struct)
  } else {
    console.log(`Decode ${imageList[SelectIndex.value]}`);

    data.dataArray.push(imageList[SelectIndex.value].convert_backend_struct)
  }

  const dataList = (await invoke<Array<Result>>('img_decode_v2', data)).map(
    (data) => new Result(data)
  )

  dataList.forEach((data) => {
    if (data.success) {
      ImgDecoded.increast(data)
    } else {
      Toast.error(data.message)
    }
  })
}
</script>

<style scoped>
.container {
  overflow: auto;
  justify-content: space-between;

  .header {
    height: 40px;
    display: flex;
    column-gap: 1rem;
    justify-content: space-evenly;

    .seed {
      width: 60%;
    }
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
    position: relative;
    display: flex;
    height: 50px;

    .upload-box {
      box-sizing: border-box;
      display: flex;
      position: absolute;
      top: 0;
      left: 0;
      background-color: var(--upload-image-input-background-color);
      width: 100%;
      height: 100%;
      border-radius: 8px;
      align-items: center;
      justify-content: center;
      overflow: hidden;
      border: 3px dashed black;

      .upload-text {
        position: absolute;
        pointer-events: none;
        font-weight: bold;
        font-size: 30px;
      }

      .upload-input {
        cursor: pointer;
        width: 100%;
        height: 100%;
        padding: 0;
        background-color: none;
        opacity: 0;
      }
    }
  }
}
</style>
