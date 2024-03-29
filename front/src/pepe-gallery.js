import { info, capitalize, createElement, getSuperposables } from './utils.js'

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
        Picture ID
        <input type="text" name="picture" placeholder="picture id">
      </label>
      <label class="form-field">
        Username
        <input type="text" name="username" placeholder="username">
      </label>
      <fieldset id="superposables-fieldset" class="form-field">
        <legend>Superposables</legend>
      </fieldset>
      <fieldset class="form-field">
        <legend>Date Range</legend>
        <label class="form-subfield">
          Start
          <input type="date" name="start">
        </label>
        <label class="form-subfield">
          End
          <input type="date" name="end">
        </label>
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

    // Set picture id if picture is in url
    const pictureId = window.location.href.split('?picture=')[1]
    if (pictureId) {
      this.shadowRoot
        .querySelector('#filters-form [name="picture"]').value = pictureId
      this._applyFilters()
    }

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
      // TODO: remove this monkey patch one day (when I'll have time)
      // There is an error when the user is not logged in on the editor's
      // page. I think htat the async window location change interrups the
      // getSuperposables() call which therefore returns undefined.
      if (!superposables) return
      for (const superposable of superposables) {
        const input = document.createElement('input')
        input.type = 'checkbox'
        input.name = superposable
        input.id = `superposable-${superposable}`
        const label = document.createElement('label')
        label.textContent = capitalize(superposable)
        label.prepend(input)
        label.classList.add('checkbox')
        this.shadowRoot.querySelector('#superposables-fieldset').append(label)
        this._superposables.push(superposable)
      }
    })
    if (!this.disabled) this._getPepePosts()
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === 'disabled' && !newValue) this._reset()
    else if (name === 'data-filters' && newValue !== oldValue) {
      this._reset()
    } else if (name === 'data-username') {
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

  // Send event to notify that gallery is ready
  _ready() {
    const event = new CustomEvent('pepe-gallery-ready', {
      bubbles: true,
      composed: true,
    })
    this.dispatchEvent(event)
  }

  // Get posts
  async _getPepePosts() {
    try {
      this._index += 1
      let url = `${info.api}/pictures?index=${this._index}&count=${this.count}`
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
      if (this._index === 0) this._ready()
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
      } else if (key === 'start' || key === 'end') {
        const date = new Date(value)
        const timestamp = Math.floor(date.getTime() / 1000)
        newFilters += `&${key}=${timestamp}`
      } else {
        newFilters += `&${key}=${value}`
      }
    }
    this.filters = newFilters
  }

  getPictureElement(id) {
    return this.shadowRoot.querySelector(`[data-picture-id="${id}"]`)
  }

  appendPicture(picture) {
    const pictureElement = this._newPicture(picture)
    this.shadowRoot.append(pictureElement)
    this.emptyGallery = false
    return this.getPictureElement(picture.picture_id)
  }

  prependPicture(picture) {
    const pictureElement = this._newPicture(picture)
    const filtersDialog = this.shadowRoot.querySelector('#filters-dialog')
    filtersDialog.after(pictureElement)
    this.emptyGallery = false
    return this.getPictureElement(picture.picture_id)
  }

  deletePicture(id) {
    const pictureElement = this.getPictureElement(id)
    pictureElement.remove()
    this.emptyGallery = this.pictureCount === 0
  }
}

// Register the PepeGallery element
customElements.define('pepe-gallery', PepeGallery)
