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
        <div class="box-title">
          <div>{{ img.seed }}</div>
          <div class="close" @click="close(img.src_path)">X</div>
        </div>
        <img
          class="encode-img"
          :src="img.src_path"
          :alt="img.seed.toString()"
        />
      </div>
    </div>

    <div class="footer">
      <div class="upload-box">
        <span class="upload-text">Upload Image</span>
        <button
          class="upload-input"
          name="image and seed"
          @click="uploadFile"
        ></button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { DialogFilter, OpenDialogOptions, open } from '@tauri-apps/api/dialog'
import { ImgDecodedStore, Result, Image } from '../stores/decodedImage'
import { useToast } from 'vue-toast-notification'

const ImgDecoded = ImgDecodedStore()
const Toast = useToast()
const SelectIndex = ref(0)

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

async function uploadFile() {
  const filters: DialogFilter[] = [
    {
      name: 'Encode Image',
      extensions: ['png'],
    },
  ]

  const OPTIONS = {
    title: 'Select Encode Image',
    filters: filters,
    multiple: true,
  } as OpenDialogOptions
  const FILES = await open(OPTIONS)
  if (FILES === null) return

  for (const FILE_PATH of FILES) {
    if (getFileExtension(FILE_PATH) !== 'png') continue
    const SEED = await invoke<number>('img_seed_extract', { path: FILE_PATH })
    imageList.push(new Image(SEED, FILE_PATH))
  }
}

async function decode(index: number) {
  if (imageList.length === 0) return

  let data = {
    dataArray: index === -1 ? imageList : [imageList[SelectIndex.value]],
  }
  const dataList = (await invoke<Array<Result>>('img_decode_v2', data)).map(
    (data) => new Result(data)
  )

  dataList.forEach((data) => {
    if (data.success) {
      ImgDecoded.increast(new Image(data.seed, data.message))
    } else {
      Toast.error(data.message)
    }
  })
}

async function close(src_path: string) {
  const INDEX = imageList.findIndex((img) => img.src_path === src_path)
  if (INDEX === -1) return
  imageList.splice(INDEX, 1)
}

listen('tauri://file-drop', async (event) => {
  for (const FILE_PATH of event.payload as Array<string>) {
    if (getFileExtension(FILE_PATH) !== 'png') continue
    const SEED = await invoke<number>('img_seed_extract', { path: FILE_PATH })
    imageList.push(new Image(SEED, FILE_PATH))
  }
})
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

      .box-title {
        position: relative;

        .close {
          position: absolute;
          top: 0;
          right: 0;
          width: 24px;
          height: 22px;
          border-radius: 50%;
          transition: 200ms;
          line-height: 24px;
          color: rgb(174, 0, 174);

          &:hover {
            background-color: rgba(0, 255, 255, 0.2);
          }
        }
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
