import { invoke, convertFileSrc } from '@tauri-apps/api/tauri'
import { defineStore } from 'pinia'

export class Image {
  seed: number
  file_path: string
  src_path: string

  constructor(seed: number, file_path: string) {
    this.seed = seed
    this.file_path = file_path
    this.src_path = convertFileSrc(file_path)
  }
}

class SaveStruct {
  filename: string
  file_path: string
  dir_path: string

  constructor(data: Image, path: string) {
    this.filename = `${data.seed}.png`
    this.file_path = data.file_path
    this.dir_path = path;
  }
}

export class Result {
  success!: boolean
  message!: string
  seed!: number

  constructor(data: Result) {
    Object.assign(this, data)
  }
}

export const ImgDecodedStore = defineStore('imgDecoded', {
  state: () => {
    return { imgList: [] as Array<Image> }
  },
  actions: {
    increast(data: Image) {
      if (this.imgList.some((img) => img.seed === data.seed)) return

      this.imgList.push(data)
    },

    async saveImage(path: string): Promise<Result[]> {
      const SaveList = this.imgList.map((data) => new SaveStruct(data, path))
      const data = { dataArray: SaveList }
      return invoke('save_image', data)
    },
  },
})
