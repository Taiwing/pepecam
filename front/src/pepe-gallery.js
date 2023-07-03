import { capitalize, createElement, getSuperposables } from './utils.js'

const PepeGalleryTemplate = document.createElement('template')
PepeGalleryTemplate.innerHTML = `
  <link rel="stylesheet" href="style/global.css">
  <link rel="stylesheet" href="style/pepe-gallery.css">

  <div class="user-actions">
    <button id="filters-button">filters</button>
  </div>

  <div id="empty-gallery" hidden>
    <p>There is no post yet</p>
    <img src="pictures/superposables/sad.png" alt="Pepe sad">
  </div>

  <dialog id="filters-dialog">
    <h3>Filters</h3>
    <form id="filters-form" method="dialog" class="form">
      <label class="form-field">
        username
        <input type="text" name="username" placeholder="username">
      </label>
      <fieldset id="superposables-fieldset" class="form-field">
        <legend>Superposables</legend>
      </fieldset>
      <div class="form-field">
        <button type="submit">apply</button>
        <button type="reset">reset</button>
        <button type="button">close</button>
      </div>
    </form>
  </dialog>
`

// PepeGallery element
class PepeGallery extends HTMLElement {
  count = 10 // Number of posts to get

  static get observedAttributes() {
    return ['disabled', 'data-filters', 'data-username']
  }

  constructor() {
    super()
    this._index = -1
    this._finished = false
    this._superposables = []
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.append(PepeGalleryTemplate.content.cloneNode(true))

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

    // Handle filters dialog
    const filtersDialog = this.shadowRoot.querySelector('#filters-dialog')
    filtersDialog
      .querySelector('button[type="reset"]')
      .addEventListener('click', this._resetFilters.bind(this))
    filtersDialog
      .querySelector('button[type="submit"]')
      .addEventListener('click', this._applyFilters.bind(this))
    filtersDialog
      .querySelector('button[type="button"]')
      .addEventListener('click', () => filtersDialog.close())
    const filtersButton = this.shadowRoot.querySelector('#filters-button')
    filtersButton.addEventListener('click', () => filtersDialog.showModal())
  }

  connectedCallback() {
    getSuperposables().then(superposables => {
      for (const superposable of superposables) {
        const input = document.createElement('input')
        input.type = 'checkbox'
        input.name = superposable
        input.id = `superposable-${superposable}`
        const label = document.createElement('label')
        label.textContent = capitalize(superposable)
        label.prepend(input)
        this.shadowRoot.querySelector('#superposables-fieldset').append(label)
        this._superposables.push(superposable)
      }
    })
    if (!this.disabled) this._getPepePosts()
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === 'disabled' && !newValue) this._reset()
    else if (name === 'data-filters' && newValue !== oldValue) this._reset()
    else if (name === 'data-username') {
      const usernameField = this.shadowRoot.querySelector('#filters-form [name="username"]')
      usernameField.value = ''
      usernameField.placeholder = newValue
      usernameField.disabled = true
    }
  }

  get thumbnail() {
    return this.hasAttribute('thumbnail')
  }

  get username() {
    return this.getAttribute('data-username')
  }

  get pictureCount() {
    if (this.thumbnail) {
      return this.shadowRoot.querySelectorAll('pepe-thumbnail').length
    } else {
      return this.shadowRoot.querySelectorAll('pepe-post').length
    }
  }

  set emptyGallery(value) {
    if (value) {
      this.shadowRoot.getElementById('empty-gallery').removeAttribute('hidden')
    } else {
      this.shadowRoot.getElementById('empty-gallery').setAttribute('hidden', '')
    }
  }

  get filters() {
    return this.getAttribute('data-filters')
  }

  set filters(value) {
    this.setAttribute('data-filters', value)
  }

  _onScroll(event) {
    if (this._finished) return

    const { scrollTop, scrollHeight } = event.target
    if (scrollTop >= scrollHeight * 0.75) this._getPepePosts()
  }

  _reset() {
    this._index = -1
    this._finished = false
    const selector = this.thumbnail ? 'pepe-thumbnail' : 'pepe-post'
    this.shadowRoot.querySelectorAll(selector).forEach(element => {
      element.remove()
    })
    this.emptyGallery = this.pictureCount === 0
    this._getPepePosts()
  }

  _getPictureElement(id) {
    return this.shadowRoot.querySelector(`[data-picture-id="${id}"]`)
  }

  _getSuperposableElement(superposable) {
    return this.shadowRoot.querySelector(`#superposable-${superposable}`)
  }

  _newPicture(picture) {
    const {
      picture_id,
      account_id,
      superposable,
      creation_ts,
      author,
      like_count,
      dislike_count,
      comment_count,
      liked,
      disliked,
    } = picture

    const attributes = {
      'data-picture-id': picture_id,
      'data-account-id': account_id,
      'data-superposable': superposable,
      'data-creation-ts': creation_ts,
      'data-author': author,
      'data-like-count': like_count,
      'data-dislike-count': dislike_count,
      'data-comment-count': comment_count,
      'data-liked': liked,
      'data-disliked': disliked,
    }

    if (this.thumbnail) {
      return createElement('pepe-thumbnail', attributes)
    } else {
      return createElement('pepe-post', attributes)
    }
  }

  // Get posts
  async _getPepePosts() {
    try {
      this._index += 1
      const { hostname } = window.location
      let url =
        `http://${hostname}:3000/pictures?index=${this._index}&count=${this.count}`
      if (this.username) url += `&username=${this.username}`
      if (this.filters) url += this.filters
      const response = await fetch(url, { method: 'GET', credentials: 'include' })
      const posts = await response.json()

      if (response.status !== 200 || !posts || posts.length === 0) {
        this._finished = true
        if (this._index === 0) this.emptyGallery = true
        return
      }

      for (const post of posts) this.appendPicture(post)
    } catch (error) {
      this._finished = true
      alert(`${error.name}: ${error.message}`)
    }
  }

  _resetFilters() {
    const filtersForm = this.shadowRoot.querySelector('#filters-form')
    filtersForm.reset()
    this._applyFilters()
  }

  _applyFilters() {
    const filtersForm = this.shadowRoot.querySelector('#filters-form')
    const formData = new FormData(filtersForm)
    let newFilters = ''
    for (const [key, value] of formData.entries()) {
      if (!value) {
        continue
      } else if (this._superposables.includes(key)) {
        newFilters += `&superposable=${key}`
      } else {
        newFilters += `&${key}=${value}`
      }
    }
    this.filters = newFilters
  }

  appendPicture(picture) {
    const pictureElement = this._newPicture(picture)
    this.shadowRoot.append(pictureElement)
    this.emptyGallery = false
    return this._getPictureElement(picture.picture_id)
  }

  prependPicture(picture) {
    const pictureElement = this._newPicture(picture)
    this.shadowRoot.prepend(pictureElement)
    this.emptyGallery = false
    return this._getPictureElement(picture.picture_id)
  }

  deletePicture(id) {
    const pictureElement = this._getPictureElement(id)
    pictureElement.remove()
    this.emptyGallery = this.pictureCount === 0
  }
}

// Register the PepeGallery element
customElements.define('pepe-gallery', PepeGallery)
