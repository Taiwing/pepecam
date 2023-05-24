import { forbidUnconnected } from './utils.js'

// Handle thumbnail click event
const onThumbnailClick = (event) => {
  const post = document.querySelector('pepe-post')
  Object.entries(event.detail).forEach(([key, value]) =>
    post.setAttribute(key, value)
  )
  post.removeAttribute('hidden')
}

const initUpload = async () => {
  // Check if user is connected
  //forbidUnconnected()
  //window.addEventListener('toggle-connected', forbidUnconnected)

  // Register thumbnail click event
  window.addEventListener('pepe-thumbnail-click', onThumbnailClick)
}

initUpload()
