import { invoke } from '@tauri-apps/api/tauri'
import { defineStore } from 'pinia'

class SaveStruct {
  filename: string
  base64: string

  constructor(data: Result, path: string) {
    const LastCahr = path[path.length - 1]
    if (LastCahr !== '\\') path += '\\'

    this.filename = `${path}${data.seed}.png`
    this.base64 = data.message.split(',')[1]
  }
}

export class Result {
  success!: boolean
  message!: string
  seed!: number

  constructor(data: Result) {
    Object.assign(this, data)
    if (this.message.length >= 100) {
      this.message = 'data:image/png;base64,' + this.message
    }
  }
}

export const ImgDecodedStore = defineStore('imgDecoded', {
  state: () => {
    return { imgList: [] as Array<Result> }
  },
  actions: {
    increast(data: Result) {
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
