import { getCookie, forbidUnconnected } from './utils.js'

// Handle thumbnail click event
const onThumbnailClick = (event) => {
  const post = document.querySelector('pepe-post')
  Object.entries(event.detail).forEach(([key, value]) =>
    post.setAttribute(key, value)
  )
  post.removeAttribute('hidden')
}

const onPostClose = () => {
  const post = document.querySelector('pepe-post')
  post.setAttribute('hidden', '')
}

const onPostDelete = (event) => {
  onPostClose()
  const { picture_id } = event.detail
  const gallery = document.querySelector('pepe-gallery')
  const selector = `pepe-thumbnail[data-picture-id="${picture_id}"]`
  const thumbnail = gallery.shadowRoot.querySelector(selector)
  thumbnail.remove()
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

  //TODO: implement close button for pepe posts
  // Register post close event
  //window.addEventListener('pepe-post-close', onPostClose)

  // Register post delete event
  window.addEventListener('pepe-post-delete', onPostDelete)
}

initEditor()
