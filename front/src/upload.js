import { forbidUnconnected } from './utils.js'

const initUpload = async () => {
  // Check if user is connected
  forbidUnconnected()
  window.addEventListener('toggle-connected', forbidUnconnected)
}

initUpload()
