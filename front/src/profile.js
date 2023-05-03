import { forbidUnconnected, submitForm } from './utils.js'

const form = document.querySelector('#profile-form')
const passwordField = form.querySelector('input[name="password"]')
const confirmPasswordField = form.querySelector(
  'input[name="password-confirm"]',
)
const profileSubmitButton = form.querySelector('#profile-submit-button')

const validatePassword = () => {
  if (passwordField.value !== confirmPasswordField.value) {
    confirmPasswordField.setCustomValidity('Passwords must match')
  } else {
    confirmPasswordField.setCustomValidity('')
  }
}

let user

const profileSubmit = async (event) => {
  event.preventDefault()
  const url = 'http://localhost:3000/user'

  try {
    if (!user || form.reportValidity() === false) return
    const formData = new FormData(form)
    const notifications = formData.get('email_notifications') === 'on'
    if (notifications === user.email_notifications) {
      formData.delete('email_notifications')
    } else if (!notifications) {
      formData.set('email_notifications', 'off')
    }
    const response = await submitForm(formData, 'PUT', url)
    if (!response.ok) {
      const { message, error } = await response.json()
      throw message || error || JSON.stringify(response)
    }
    alert('Profile updated')
    window.location.href = '/'
  } catch (error) {
    alert(`Error: ${error}`)
  }
}

const getUser = async () => {
  const url = 'http://localhost:3000/user'
  const response = await fetch(url, { credentials: 'include' })
  if (!response.ok) {
    const { message, error } = await response.json()
    const errorMessage = message || error || JSON.stringify(response)
    throw errorMessage
  }
  return response.json()
}

const initProfile = async () => {
  // Event listeners
  passwordField.addEventListener('change', validatePassword)
  confirmPasswordField.addEventListener('keyup', validatePassword)
  profileSubmitButton.addEventListener('click', profileSubmit)

  // Check if user is connected
  forbidUnconnected()
  window.addEventListener('toggle-connected', forbidUnconnected)

  // Set form values
  try {
    user = await getUser()
    const { username, email, email_notifications } = user
    form
      .querySelector('input[name="username"]')
      .setAttribute('placeholder', username)
    form.querySelector('input[name="email"]').setAttribute('placeholder', email)
    form
      .querySelector('input[name="email_notifications"]')
      .checked = email_notifications
  } catch (error) {
    alert(`Error: ${error}`)
  }
}

initProfile()
