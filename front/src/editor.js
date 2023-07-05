import { getCookie, forbidUnconnected } from './utils.js'

// Switch between upload and editor
const toggleUploadEditor = () => {
  const upload = document.querySelector('#editor-upload')
  upload.toggleAttribute('hidden')
  const editor = document.querySelector('#editor-editor')
  editor.toggleAttribute('hidden')
}

// Handle thumbnail click event
const onThumbnailClick = (event) => {
  // Set post attributes
  const post = document.querySelector('pepe-post')
  Object.entries(event.detail).forEach(([key, value]) =>
    post.setAttribute(key, value)
  )
  const editor = document.querySelector('#editor-editor')
  if (editor.hasAttribute('hidden')) toggleUploadEditor()

  // Scroll to left
  const container = document.querySelector('.editor')
  container.scrollLeft = 0
}

// Handle post delete event
const onPostDelete = (event) => {
  const { picture_id } = event.detail
  const gallery = document.querySelector('pepe-gallery')
  gallery.deletePicture(picture_id)
  toggleUploadEditor()
}

// Handle editor upload event
const onPepeUpload = (event) => {
  const { detail } = event
  const gallery = document.querySelector('pepe-gallery')
  const thumbnail = gallery.prependPicture(detail)
  thumbnail.onClick()
}

const initEditor = async () => {
  // Check if user is connected
  if (forbidUnconnected()) return
  window.addEventListener('toggle-connected', forbidUnconnected)

  // Set event listener if picture id is provided (one-time event)
  const pictureId = window.location.href.split('?picture=')[1]
  if (pictureId) {
    window.addEventListener('pepe-gallery-ready', (event) => {
      event.target.getPictureElement(pictureId).onClick()
    }, { once: true })
  }

  // Set username for gallery
  const gallery = document.querySelector('pepe-gallery')
  const { username } = getCookie('session')
  gallery.setAttribute('data-username', username)
  gallery.removeAttribute('disabled')

  // Register thumbnail click event
  window.addEventListener('pepe-thumbnail-click', onThumbnailClick)

  // Register editor close event
  const close = document.querySelector('#editor-editor-close')
  close.addEventListener('click', toggleUploadEditor)

  // Register post delete event
  window.addEventListener('pepe-post-delete', onPostDelete)

  // Register editor upload event
  window.addEventListener('pepe-upload', onPepeUpload)
}

initEditor()
