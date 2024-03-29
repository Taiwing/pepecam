import {
  info,
  asyncAlert,
  getCookie,
  toggleConnectedEvent,
  submitForm,
  ApiError,
} from './utils.js'

async function dialogSubmit() {
  const id = this.getAttribute('id')
  const url = this.getAttribute('url')
  const form = this.querySelector('form')
  const submit = this.querySelector('button[type="submit"]')

  try {
    if (form.reportValidity() === false) return
    const response = await submitForm(new FormData(form), 'POST', url)
    const data = await response.json()
    if (!response.ok) {
      throw new ApiError(data)
    } else if (id === 'login-dialog') {
      submit.dispatchEvent(toggleConnectedEvent())
    } else if (id === 'signup-dialog' || id === 'reset-dialog') {
      asyncAlert(`Success: ${data.response}`)
    }
  } catch (error) {
    alert(`${error.name}: ${error.message}`)
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
      <label class="form-field">
        username
        <input type="text" name="username" placeholder="username" required>
      </label>
      <label class="form-field">
        password
        <input type="password" name="password" placeholder="password" required>
      </label>
      <div class="form-field" id="forgot-password-container">
        <a href="#" id="forgot-password">forgot password?</a>
      </div>
      <div class="form-field">
        <button type="submit">login</button>
        <button type="reset">cancel</button>
      </div>
    </form>
  </dialog>

  <dialog id="reset-dialog">
    <h3>Reset Password</h3>
    <form id="reset-form" method="dialog" class="form">
      <label class="form-field">
        email
        <input type="email" name="email" placeholder="email" required>
      </label>
      <div class="form-field">
        <button type="submit">send</button>
        <button type="reset">cancel</button>
      </div>
    </form>
  </dialog>

  <dialog id="signup-dialog">
    <h3>Signup</h3>
    <form id="signup-form" method="dialog" class="form">
      <label class="form-field">
        username
        <input type="text" name="username" placeholder="username" required>
      </label>
      <label class="form-field">
        email
        <input type="email" name="email" placeholder="email" required>
      </label>
      <label class="form-field">
        password
        <input type="password" name="password" placeholder="password" required>
      </label>
      <label class="form-field">
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
    this.shadowRoot.append(PepeHeaderTemplate.content.cloneNode(true))

    const loginDialog = this.shadowRoot.querySelector('#login-dialog')
    loginDialog.setAttribute(
      'url',
      `${info.api}/user/login`
    )
    const resetDialog = this.shadowRoot.querySelector('#reset-dialog')
    resetDialog.setAttribute(
      'url',
      `${info.api}/user/reset`,
    )
    const signupDialog = this.shadowRoot.querySelector('#signup-dialog')
    signupDialog.setAttribute(
      'url',
      `${info.api}/user/register`
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
    const resetButton = loginDialog.querySelector('#forgot-password')
    resetButton.addEventListener('click', () => {
      loginDialog.close()
      resetDialog.showModal()
    })
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
    resetDialog
      .querySelector('button[type="submit"]')
      .addEventListener('click', dialogSubmit.bind(resetDialog))
    resetDialog
      .querySelector('button[type="reset"]')
      .addEventListener('click', () => resetDialog.close())
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
      const url = `${info.api}/user/logout`
      const response = await fetch(url, {
        method: 'POST',
        credentials: 'include',
      })

      if (!response.ok) {
        const error = await response.json()
        throw new ApiError(error)
      }
    } catch (error) {
      alert(`${error.name}: ${error.message}`)
    }
  }
}

// Register the PepeHeader element
customElements.define('pepe-header', PepeHeader, { extends: 'header' })
