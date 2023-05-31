import { createElement } from './utils.js'

const PepeGalleryTemplate = document.createElement('template')
PepeGalleryTemplate.innerHTML = `
  <link rel="stylesheet" href="style/pepe-gallery.css">
  <div id="empty-gallery" hidden>
    <p>There is no post yet</p>
    <img src="pictures/superposables/sad.png" alt="Pepe sad">
  </div>
`

// PepeGallery element
class PepeGallery extends HTMLElement {
  count = 10 // Number of posts to get

  static get observedAttributes() {
    return ['data-username']
  }

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
      this._reset.bind(this),
    )
  }

  _onScroll(event) {
    if (this._finished) return

    const { scrollTop, scrollHeight } = event.target
    if (scrollTop >= scrollHeight * 0.75) this.getPepePosts()
  }

  _reset() {
    this._index = -1
    this._finished = false
    this.shadowRoot.innerHTML = ''
    this.shadowRoot.appendChild(PepeGalleryTemplate.content.cloneNode(true))
    this.getPepePosts()
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === 'data-username') this._reset()
  }

  get thumbnail() {
    return this.hasAttribute('thumbnail')
  }

  get username() {
    return this.getAttribute('data-username')
  }

  // Get posts
  async getPepePosts() {
    try {
      this._index += 1
      //TODO: This is a dirty hack so that the gallery is empty when needed.
      // The root cause of the problem is that this method is called twice
      // when loading the editor page, once when the gallery is connected
      // and once when data-username property is set. Fix this.
      const index = this._index
      let url =
        `http://localhost:3000/pictures?index=${this._index}&count=${this.count}`
      if (this.username) url += `&username=${this.username}`
      const response = await fetch(url, { method: 'GET', credentials: 'include' })
      const posts = await response.json()

      if (response.status !== 200 || !posts || posts.length === 0) {
        this._finished = true
        //TODO: replace index by this._index when root cause is fixed
        if (index === 0) {
          this.shadowRoot
            .getElementById('empty-gallery')
            .removeAttribute('hidden')
        }
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
