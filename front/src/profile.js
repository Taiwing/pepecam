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

const profileSubmit = async (event) => {
  event.preventDefault()
  const url = 'http://localhost:3000/user'

  try {
    if (form.reportValidity() === false) return
    const response = await submitForm(form, 'PUT', url)
    if (response.ok) {
      alert('Profile updated')
      window.location.href = '/'
    } else {
      const { message, error } = await response.json()
      const errorMessage = message || error || JSON.stringify(response)
      alert(`Error: ${errorMessage}`)
    }
  } catch (error) {
    alert(`Error: ${error}`)
  }
}

const initProfile = () => {
  passwordField.addEventListener('change', validatePassword)
  confirmPasswordField.addEventListener('keyup', validatePassword)
  profileSubmitButton.addEventListener('click', profileSubmit)
}

initProfile()
forbidUnconnected()
window.addEventListener('toggle-connected', forbidUnconnected)
