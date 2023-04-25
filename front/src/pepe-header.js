// PepeHeader element
class PepeHeader extends HTMLElement {
  constructor() {
    super()
    this.attachShadow({ mode: 'open' })

    const style = document.createElement('style')
    style.textContent = `@import "style/pepe-header.css"`
    const home = document.createElement('a')
    home.href = '/'

    const div = document.createElement('div')
    div.setAttribute('id', 'login-signup')
    const loginButton = document.createElement('button')
    const signupButton = document.createElement('button')
    loginButton.textContent = 'login'
    signupButton.textContent = 'signup'
    div.append(loginButton, signupButton)

    this.shadowRoot.append(style, home, div)
  }
}

// Register the PepeHeader element
customElements.define('pepe-header', PepeHeader, { extends: 'header' })
