import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ToastPlugin from 'vue-toast-notification'

import 'vue-toast-notification/dist/theme-sugar.css'
import './styles.scss'
import './color.scss'
import App from './App.vue'

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(ToastPlugin)
app.mount('#app')
