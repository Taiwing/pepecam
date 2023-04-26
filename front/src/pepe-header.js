import { getCookie } from './cookies.js'

const formField = (name, type, label, placeholder) => {
  const labelElement = document.createElement('label')
  labelElement.setAttribute('for', name)
  labelElement.textContent = label || name
  labelElement.classList.add('form-field')
  const input = document.createElement('input')
  input.setAttribute('type', type)
  input.setAttribute('name', name)
  input.setAttribute('placeholder', placeholder || name)
  input.setAttribute('required', '')
  labelElement.append(input)
  return labelElement
}

const capitalize = (str) => str[0].toUpperCase() + str.slice(1)

const postForm = (form, action) => {
  const formData = new FormData(form)

  const data = {}
  for (const [key, value] of formData.entries()) {
    if (key !== 'password-confirm') {
      data[key] = value
    }
  }

  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  return fetch(action, {
    method: 'POST',
    headers,
    credentials: 'include',
    body: JSON.stringify(data),
  })
}

const buildFormDialog = (formName, action, formFields, toggleConnected) => {
    const dialog = document.createElement('dialog')
    dialog.setAttribute('id', `${formName}-dialog`)

    const title = document.createElement('h3')
    title.textContent = capitalize(formName)

    const form = document.createElement('form')
    form.setAttribute('id', `${formName}-form`)
    form.setAttribute('method', 'dialog')
    form.classList.add('form')

    const forms = []
    for (const field of formFields) {
      const { name, type, label, placeholder } = field
      const fieldElement = formField(name, type, label, placeholder)
      fieldElement.setAttribute('id', `${formName}-${name}`)
      forms.push(fieldElement)
    }

    const buttons = document.createElement('div')
    buttons.classList.add('form-field')
    const submit = document.createElement('button')
    submit.setAttribute('type', 'submit')
    submit.textContent = formName
    submit.addEventListener('click', async () => {
      try {
        if (form.reportValidity() === false) return
        const response = await postForm(form, action)
        if (response.ok) {
          const message = await response.json()
          alert(`Success: ${JSON.stringify(message)}`) //TEMP
          toggleConnected()
        } else {
          const { message, error } = await response.json()
          const errorMessage = message || error || JSON.stringify(response)
          alert(`Error: ${errorMessage}`)
        }
      } catch (error) {
        alert(`Error: ${error}`)
      }
    })
    const cancel = document.createElement('button')
    cancel.setAttribute('type', 'reset')
    cancel.textContent = 'cancel'
    cancel.addEventListener('click', () => dialog.close())
    buttons.append(submit, cancel)

    form.append(...forms, buttons)
    dialog.append(title, form)
    return dialog
}

const loginFields = [
  { name: 'username', type: 'text' },
  { name: 'password', type: 'password' },
]

const signupFields = [
  { name: 'username', type: 'text' },
  { name: 'email', type: 'email' },
  { name: 'password', type: 'password' },
  {
    name: 'password-confirm',
    type: 'password',
    label: 'confirm',
    placeholder: 'password',
  },
]

// PepeHeader element
class PepeHeader extends HTMLElement {
  constructor() {
    super()
    this.attachShadow({ mode: 'open' })

    const style = document.createElement('style')
    style.textContent = `@import "style/pepe-header.css"`
    const home = document.createElement('a')
    home.href = '/'

    const unconnected = document.createElement('div')
    unconnected.setAttribute('id', 'unconnected')
    unconnected.classList.add('user-actions')

    const connected = document.createElement('div')
    connected.setAttribute('id', 'connected')
    connected.classList.add('user-actions')

    const toggleConnected = () => {
      if (getCookie('session')) {
        unconnected.style.display = 'none'
        connected.style.display = 'flex'
      } else {
        connected.style.display = 'none'
        unconnected.style.display = 'flex'
      }
    }

    const loginDialog = buildFormDialog(
      'login',
      'http://localhost:3000/user/login',
      loginFields,
      toggleConnected,
    )
    const signupDialog = buildFormDialog(
      'signup',
      'http://localhost:3000/user/register',
      signupFields,
      toggleConnected,
    )
    const passwordField = signupDialog
      .querySelector('#signup-password')
      .querySelector('input')
    const confirmPasswordField = signupDialog
      .querySelector('#signup-password-confirm')
      .querySelector('input')
    const validatePassword = () => {
      if (passwordField.value !== confirmPasswordField.value) {
        confirmPasswordField.setCustomValidity('Passwords must match')
      } else {
        confirmPasswordField.setCustomValidity('')
      }
    }
    passwordField.addEventListener('change', validatePassword)
    confirmPasswordField.addEventListener('keyup', validatePassword)

    const logoutButton = document.createElement('button')
    logoutButton.textContent = 'logout'
    logoutButton.addEventListener('click', async () => {
      try {
        const response = await fetch('http://localhost:3000/user/logout', {
          method: 'POST',
          credentials: 'include',
        })

        if (response.ok) {
          const message = await response.json()
          alert(`Success: ${JSON.stringify(message)}`) //TEMP
        } else {
          const { message, error } = await response.json()
          const errorMessage = message || error || JSON.stringify(response)
          alert(`Error: ${errorMessage}`)
        }
      } catch (error) {
        alert(`Error: ${error}`)
      }

      toggleConnected()
    })

    const loginButton = document.createElement('button')
    loginButton.textContent = 'login'
    loginButton.addEventListener('click', () => loginDialog.showModal())

    const signupButton = document.createElement('button')
    signupButton.textContent = 'signup'
    signupButton.addEventListener('click', () => signupDialog.showModal())

    connected.append(logoutButton)
    unconnected.append(loginButton, signupButton)
    toggleConnected()

    this.shadowRoot.append(
      style,
      home,
      loginDialog,
      signupDialog,
      connected,
      unconnected,
    )
  }
}

// Register the PepeHeader element
customElements.define('pepe-header', PepeHeader, { extends: 'header' })
