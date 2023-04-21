// PepeHeader element
class PepeHeader extends HTMLElement {
  constructor() {
    super()
    this.attachShadow({ mode: 'open' })

    const style = document.createElement('style')
    style.textContent = `@import "style/pepe-header.css"`
    const home = document.createElement('a')
    home.href = '/'
    this.shadowRoot.append(style, home)
  }
}

// Register the PepeHeader element
customElements.define('pepe-header', PepeHeader, { extends: 'header' })
