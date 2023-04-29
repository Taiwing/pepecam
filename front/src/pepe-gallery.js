import { createElement } from './utils.js'

const PepePostTemplate = document.createElement('template')
PepePostTemplate.innerHTML = `
  <style>@import "style/pepe-post.css"</style>
  <h2>
    <span id='author-span'></span>
    <span id='date-span'></span>
  </h2>
  <img />
`

// PepePost element
class PepePost extends HTMLElement {
  constructor() {
    super()
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.appendChild(PepePostTemplate.content.cloneNode(true))
  }

  connectedCallback() {
    const picture_id = this.getAttribute('data-picture-id')
    const creation_ts = this.getAttribute('data-creation-ts')
    const author = this.getAttribute('data-author')

    const authorSpan = this.shadowRoot.querySelector('#author-span')
    const dateSpan = this.shadowRoot.querySelector('#date-span')
    authorSpan.textContent = `@${author}`
    const date = new Date(Number(creation_ts) * 1000)
    dateSpan.textContent = ` at ${date.toLocaleString()}`

    const picture = this.shadowRoot.querySelector('img')
    picture.src = `http://localhost:8080/pictures/${picture_id}.jpg`
    picture.alt = `Picture ${picture_id}`
  }
}

// Register the PepePost element
customElements.define('pepe-post', PepePost)

// PepeGallery element
class PepeGallery extends HTMLElement {
  count = 10 // Number of posts to get

  constructor() {
    super()
    this.attachShadow({ mode: 'open' })
    this.index = -1
    this.finished = false

    const style = document.createElement('style')
    style.textContent = '@import "style/pepe-gallery.css";'
    const gallery = document.createElement('div')
    gallery.classList.add('gallery')
    this.shadowRoot.append(style, gallery)

    // Get posts on scroll
    window.addEventListener('scroll', () => {
      if (this.finished) return

      if (window.innerHeight + window.scrollY >= document.body.offsetHeight * 0.75) {
        this.getPepePosts()
      }
    })
  }

  // Get posts
  async getPepePosts() {
    try {
      this.index += 1
      const url =
        `http://localhost:3000/pictures?index=${this.index}&count=${this.count}`
      const response = await fetch(url)
      const posts = await response.json()

      if (response.status !== 200 || !posts || posts.length === 0) {
        this.finished = true
        return
      }

      const gallery = this.shadowRoot.querySelector('.gallery')
      for (const post of posts) {
        const { picture_id, account_id, creation_ts, author } = post
        const attributes = {
          'data-picture-id': picture_id,
          'data-account-id': account_id,
          'data-creation-ts': creation_ts,
          'data-author': author,
        }
        gallery.appendChild(createElement('pepe-post', attributes))
      }
    } catch (error) {
      this.finished = true
      //TODO: remove this log to "respect" the subject
      console.error(error)
    }
  }

  connectedCallback() {
    this.getPepePosts()
  }
}

// Register the PepeHeader element
customElements.define('pepe-gallery', PepeGallery, { extends: 'main' })
