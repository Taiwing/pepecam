import { info } from './utils.js'

const PepeThumbnailTemplate = document.createElement('template')
PepeThumbnailTemplate.innerHTML = `
  <style>
    #thumbnail-picture {
      width: 100%;
      border-radius: 10px;
    }
  </style>
  <img id="thumbnail-picture" />
`

class PepeThumbnail extends HTMLElement {
  static get observedAttributes() {
    return ['data-picture-id']
  }

  constructor() {
    super()
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.append(PepeThumbnailTemplate.content.cloneNode(true))
  }

  connectedCallback() {
    const picture = this.shadowRoot.getElementById('thumbnail-picture')
    picture.addEventListener('click', () => this.onClick())
  }

  onClick() {
    const detail = {
      'data-picture-id': this.getAttribute('data-picture-id'),
      'data-account-id': this.getAttribute('data-account-id'),
      'data-superposable': this.getAttribute('data-superposable'),
      'data-creation-ts': this.getAttribute('data-creation-ts'),
      'data-author': this.getAttribute('data-author'),
      'data-like-count': this.getAttribute('data-like-count'),
      'data-dislike-count': this.getAttribute('data-dislike-count'),
      'data-comment-count': this.getAttribute('data-comment-count'),
      'data-liked': this.getAttribute('data-liked'),
      'data-disliked': this.getAttribute('data-disliked'),
    }
    const event = new CustomEvent('pepe-thumbnail-click', {
      bubbles: true,
      composed: true,
      detail,
    })
    this.dispatchEvent(event)
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === 'data-picture-id') {
      const picture = this.shadowRoot.querySelector('#thumbnail-picture')
      picture.src = `${info.front}/${info.pictures_dir}/${newValue}.jpg`
      picture.alt = `Picture ${newValue}`
    }
  }
}

// Register the PepeThumbnail element
customElements.define('pepe-thumbnail', PepeThumbnail)
