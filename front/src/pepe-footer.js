const PepeFooterTemplate = document.createElement('template')
PepeFooterTemplate.innerHTML = `
  <link rel="stylesheet" href="style/pepe-footer.css" />
  <nav class="nav-menu">
    <a href="/index.html">Gallery</a>
    <a href="/editor.html">Editor</a>
    <a href="/profile.html">Profile</a>
  </nav>
  <p class="footer-text">
    ©
    <span id="years"></span>
    <span id="author"></span>
    --
    <a id="license"></a>
  </p>
`

// PepeFooter element
class PepeFooter extends HTMLElement {
  author = 'Yoann FOREAU'
  license = 'GNU General Public License v3'
  licenseUrl = 'https://www.gnu.org/licenses/gpl-3.0.html#license-text'
  startYear = 2022

  get currentYear() {
    return new Date().getFullYear()
  }

  get years() {
    return this.startYear === this.currentYear
      ? this.startYear
      : `${this.startYear}-${this.currentYear}`
  }

  constructor() {
    super()
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.append(PepeFooterTemplate.content.cloneNode(true))

    this.shadowRoot.getElementById('years').innerText = this.years
    this.shadowRoot.getElementById('author').innerText = this.author
    this.shadowRoot.getElementById('license').innerText = this.license
    this.shadowRoot.getElementById('license').href = this.licenseUrl
  }
}

customElements.define('pepe-footer', PepeFooter, { extends: 'footer' })
