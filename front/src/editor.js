import { getCookie, forbidUnconnected } from './utils.js'

// Handle thumbnail click event
const onThumbnailClick = (event) => {
  const post = document.querySelector('pepe-post')
  Object.entries(event.detail).forEach(([key, value]) =>
    post.setAttribute(key, value)
  )
  post.removeAttribute('hidden')
}

const initEditor = async () => {
  // Check if user is connected
  forbidUnconnected()
  window.addEventListener('toggle-connected', forbidUnconnected)

  // Set username for gallery
  const gallery = document.querySelector('pepe-gallery')
  const { username } = getCookie('session')
  gallery.setAttribute('data-username', username)

  // Register thumbnail click event
  window.addEventListener('pepe-thumbnail-click', onThumbnailClick)
}

initEditor()
