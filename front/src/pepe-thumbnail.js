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
    this.shadowRoot.appendChild(PepeThumbnailTemplate.content.cloneNode(true))
  }

  connectedCallback() {
    const picture = this.shadowRoot.getElementById('thumbnail-picture')
    picture.addEventListener('click', () => this.onClick())
  }

  onClick() {
    //TODO: get every attribute from the picture
    const event = new CustomEvent('pepe-thumbnail-click', {
      bubbles: true,
      detail: { pictureId: this.getAttribute('data-picture-id') }
    })
    this.dispatchEvent(event)
    console.log('Thumbnail clicked!')
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === 'data-picture-id') {
      const picture = this.shadowRoot.querySelector('#thumbnail-picture')
      picture.src = `http://localhost:8080/pictures/${newValue}.jpg`
      picture.alt = `Picture ${newValue}`
    }
  }
}

// Register the PepeThumbnail element
customElements.define('pepe-thumbnail', PepeThumbnail)
