import { submitForm, ApiError, asyncAlert } from './utils.js'

const form = document.querySelector('#reset-form')
const resetTokenField = form.querySelector('input[name="reset_token"]')
const passwordField = form.querySelector('input[name="password"]')
const confirmPasswordField = form.querySelector(
  'input[name="password-confirm"]',
)
const resetSubmitButton = form.querySelector('#reset-submit-button')

const validatePassword = () => {
  if (passwordField.value !== confirmPasswordField.value) {
    confirmPasswordField.setCustomValidity('Passwords must match')
  } else {
    confirmPasswordField.setCustomValidity('')
  }
}

// Submit form
const submitResetForm = async (event) => {
  event.preventDefault()
  const url = `http://${window.location.hostname}:3000/user/reset`

  try {
    if (form.reportValidity() === false) return
    const formData = new FormData(form)
    const response = await submitForm(formData, 'PUT', url)
    const data = await response.json()

    if (!response.ok) {
      throw new ApiError(data)
    }

    asyncAlert(`Success: ${data.message}`, '/')
  } catch (error) {
    asyncAlert(`${error.name}: ${error.message}`, '/')
  }
}

// init reset password
const initResetPassword = () => {
  try {
    // Get token from url
    const token = window.location.href.split('?token=')[1]
    if (!token) {
      throw new Error('Token not found')
    }
    resetTokenField.value = token

    // Event listeners
    passwordField.addEventListener('change', validatePassword)
    confirmPasswordField.addEventListener('keyup', validatePassword)
    resetSubmitButton.addEventListener('click', submitResetForm)
  } catch (error) {
    asyncAlert(`${error.name}: ${error.message}`, '/')
  }
}

initResetPassword()
