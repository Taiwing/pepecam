import { createElement } from './utils.js'

const PepeGalleryTemplate = document.createElement('template')
PepeGalleryTemplate.innerHTML = `
  <link rel="stylesheet" href="style/pepe-gallery.css">
`

// PepeGallery element
class PepeGallery extends HTMLElement {
  count = 10 // Number of posts to get

  constructor() {
    super()
    this._index = -1
    this._finished = false
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.appendChild(PepeGalleryTemplate.content.cloneNode(true))

    // Get posts on scroll
    this.shadowRoot
      .host
      .parentElement
      .addEventListener('scroll', this._onScroll.bind(this))

    // Handle login/logout
    window.addEventListener(
      'toggle-connected',
      this._onToggleConnected.bind(this),
    )
  }

  _onScroll(event) {
    if (this._finished) return

    const { scrollTop, scrollHeight } = event.target
    if (scrollTop >= scrollHeight * 0.75) this.getPepePosts()
  }

  _onToggleConnected() {
    this._index = -1
    this._finished = false
    this.shadowRoot.innerHTML = ''
    this.shadowRoot.appendChild(PepeGalleryTemplate.content.cloneNode(true))
    this.getPepePosts()
  }

  get thumbnail() {
    return this.hasAttribute('thumbnail')
  }

  // Get posts
  async getPepePosts() {
    try {
      this._index += 1
      const url =
        `http://localhost:3000/pictures?index=${this._index}&count=${this.count}`
      const response = await fetch(url, { method: 'GET', credentials: 'include' })
      const posts = await response.json()

      if (response.status !== 200 || !posts || posts.length === 0) {
        this._finished = true
        return
      }

      for (const post of posts) {
        const {
          picture_id,
          account_id,
          creation_ts,
          author,
          like_count,
          dislike_count,
          comment_count,
          liked,
          disliked,
        } = post
        const attributes = {
          'data-picture-id': picture_id,
          'data-account-id': account_id,
          'data-creation-ts': creation_ts,
          'data-author': author,
          'data-like-count': like_count,
          'data-dislike-count': dislike_count,
          'data-comment-count': comment_count,
          'data-liked': liked,
          'data-disliked': disliked,
        }
        if (this.thumbnail) {
          this.shadowRoot.appendChild(createElement('pepe-thumbnail', attributes))
        } else {
          this.shadowRoot.appendChild(createElement('pepe-post', attributes))
        }
      }
    } catch (error) {
      this._finished = true
      //TODO: remove this log to "respect" the subject
      console.error(error)
    }
  }

  connectedCallback() {
    this.getPepePosts()
  }
}

// Register the PepeGallery element
customElements.define('pepe-gallery', PepeGallery)
