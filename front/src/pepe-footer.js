const PepeFooterTemplate = document.createElement('template')
PepeFooterTemplate.innerHTML = `
  <link rel="stylesheet" href="style/pepe-footer.css" />
  <nav class="nav-menu">
    <a href="/index.html">Gallery</a>
    <a href="/editor.html">Editor</a>
    <a href="/profile.html">Profile</a>
  </nav>
`

// PepeFooter element
class PepeFooter extends HTMLElement {
  constructor() {
    super()
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.appendChild(PepeFooterTemplate.content.cloneNode(true))
  }
}

customElements.define('pepe-footer', PepeFooter, { extends: 'footer' })
