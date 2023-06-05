import { getCookie, toggleConnectedEvent, submitForm } from './utils.js'

async function dialogSubmit() {
  const url = this.getAttribute('url')
  const form = this.querySelector('form')
  const submit = this.querySelector('button[type="submit"]')

  try {
    if (form.reportValidity() === false) return
    const response = await submitForm(new FormData(form), 'POST', url)
    if (!response.ok) {
      const { message, error } = await response.json()
      throw message || error || JSON.stringify(response)
    }
    submit.dispatchEvent(toggleConnectedEvent())
  } catch (error) {
    alert(`Error: ${error}`)
  }
}

const PepeHeaderTemplate = document.createElement('template')
PepeHeaderTemplate.innerHTML = `
  <link rel="stylesheet" href="style/global.css" />
  <link rel="stylesheet" href="style/pepe-header.css" />
  <a id="app-name" href="/"></a>
  <nav class="nav-menu">
    <a href="/index.html">Gallery</a>
    <a href="/editor.html">Editor</a>
    <a href="/profile.html">Profile</a>
  </nav>
  <div id="unconnected" class="user-actions" hidden>
    <button id="login">login</button>
    <button id="signup">signup</button>
  </div>
  <div id="connected" class="user-actions">
    <button id="logout">logout</button>
  </div>

  <dialog id="login-dialog">
    <h3>Login</h3>
    <form id="login-form" method="dialog" class="form">
      <label for="username" class="form-field">
        username
        <input type="text" name="username" placeholder="username" required>
      </label>
      <label for="password" class="form-field">
        password
        <input type="password" name="password" placeholder="password" required>
      </label>
      <div class="form-field">
        <button type="submit">login</button>
        <button type="reset">cancel</button>
      </div>
    </form>
  </dialog>

  <dialog id="signup-dialog">
    <h3>Signup</h3>
    <form id="signup-form" method="dialog" class="form">
      <label for="username" class="form-field">
        username
        <input type="text" name="username" placeholder="username" required>
      </label>
      <label for="email" class="form-field">
        email
        <input type="email" name="email" placeholder="email" required>
      </label>
      <label for="password" class="form-field">
        password
        <input type="password" name="password" placeholder="password" required>
      </label>
      <label for="password-confirm" class="form-field">
        confirm
        <input
          type="password"
          name="password-confirm"
          placeholder="password"
          required
        >
      </label>
      <div class="form-field">
        <button type="submit">signup</button>
        <button type="reset">cancel</button>
      </div>
    </form>
  </dialog>
`

// PepeHeader element
class PepeHeader extends HTMLElement {
  constructor() {
    super()
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.appendChild(PepeHeaderTemplate.content.cloneNode(true))

    const loginDialog = this.shadowRoot.querySelector('#login-dialog')
    loginDialog.setAttribute(
      'url',
      `http://${window.location.hostname}:3000/user/login`
    )
    const signupDialog = this.shadowRoot.querySelector('#signup-dialog')
    signupDialog.setAttribute(
      'url',
      `http://${window.location.hostname}:3000/user/register`
    )

    const passwordField = signupDialog.querySelector('input[name="password"]')
    const confirmPasswordField = signupDialog.querySelector(
      'input[name="password-confirm"]',
    )
    const validatePassword = () => {
      if (passwordField.value !== confirmPasswordField.value) {
        confirmPasswordField.setCustomValidity('Passwords must match')
      } else {
        confirmPasswordField.setCustomValidity('')
      }
    }
    passwordField.addEventListener('change', validatePassword)
    confirmPasswordField.addEventListener('keyup', validatePassword)

    const loginButton = this.shadowRoot.querySelector('#login')
    loginButton.addEventListener('click', () => loginDialog.showModal())
    const signupButton = this.shadowRoot.querySelector('#signup')
    signupButton.addEventListener('click', () => signupDialog.showModal())
    const logoutButton = this.shadowRoot.querySelector('#logout')
    logoutButton.addEventListener('click', async () => {
      await this.logout()
      logoutButton.dispatchEvent(toggleConnectedEvent())
    })

    loginDialog
      .querySelector('button[type="submit"]')
      .addEventListener('click', dialogSubmit.bind(loginDialog))
    loginDialog
      .querySelector('button[type="reset"]')
      .addEventListener('click', () => loginDialog.close())
    signupDialog
      .querySelector('button[type="submit"]')
      .addEventListener('click', dialogSubmit.bind(signupDialog))
    signupDialog
      .querySelector('button[type="reset"]')
      .addEventListener('click', () => signupDialog.close())

    window.addEventListener(
      'toggle-connected',
      this._onToggleConnected.bind(this)
    )
    this._onToggleConnected({ detail: { connected: !!getCookie('session') } })
  }

  _onToggleConnected({ detail: { connected } }) {
    const unconnectedElement = this.shadowRoot.getElementById('unconnected')
    const connectedElement = this.shadowRoot.getElementById('connected')

    if (connected) {
      connectedElement.removeAttribute('hidden')
      unconnectedElement.setAttribute('hidden', '')
    } else {
      unconnectedElement.removeAttribute('hidden')
      connectedElement.setAttribute('hidden', '')
    }
  }

  async logout() {
    try {
      const url = `http://${window.location.hostname}:3000/user/logout`
      const response = await fetch(url, {
        method: 'POST',
        credentials: 'include',
      })

      if (!response.ok) {
        const { message, error } = await response.json()
        throw message || error || JSON.stringify(response)
      }
    } catch (error) {
      alert(`Error: ${error}`)
    }
  }
}

// Register the PepeHeader element
customElements.define('pepe-header', PepeHeader, { extends: 'header' })
