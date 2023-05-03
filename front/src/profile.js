import { forbidUnconnected } from './utils.js'

forbidUnconnected()
window.addEventListener('toggle-connected', forbidUnconnected)
