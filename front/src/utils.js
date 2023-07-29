import environment from './environment.js'

// This is the front's config object. It contains the front and backend's urls,
// and also values that are set in the .env file (and copied in environment.js).
export const info = {
  _apiPort: environment.API_PUBLIC_PORT,
  superposables_side: environment.SUPERPOSABLES_SIDE,
  pictures_dir: environment.PICTURES_DIR,
  superposables_dir: environment.SUPERPOSABLES_DIR,
  get _url() {
    const { protocol, hostname } = window.location
    return `${protocol}//${hostname}`
  },
  get api() {
    return `${this._url}:${this._apiPort}`
  },
  get front() {
    return `${this._url}:${window.location.port}`
  },
}

export const getCookies = () =>
  document
  .cookie
  .split(';')
  .map(cookie => cookie.split('='))
  .reduce((acc, [key, value]) => {
    if (key && key.trim() && value && value.trim()) {
      acc[key.trim()] = decodeURIComponent(value.trim())
    }
    return acc
  }, {})

export const getCookie = (name) => {
  const cookies = getCookies()
  return cookies[name] ? JSON.parse(cookies[name]) : undefined
}

export const asyncAlert = (message, href) => {
  setTimeout(() => {
    alert(message)
    if (href) window.location.href = href
  }, 1)
}

export const forbidUnconnected = () => {
  if (!getCookie('session')) {
    asyncAlert('Error: You must be connected to access this page', '/')
    return true
  }
  return false
}

export const toggleConnectedEvent = () =>
  new CustomEvent('toggle-connected', {
    bubbles: true,
    composed: true,
    detail: { connected: !!getCookie('session') },
  })

export const createElement = (tag, attributes = {}) => {
  const element = document.createElement(tag)
  Object.entries(attributes).forEach(([key, value]) =>
    element.setAttribute(key, value)
  )
  return element
}

export const submitForm = (formData, method, url) => {
  const data = {}
  for (const [key, value] of formData.entries()) {
    if (key === 'email_notifications') {
      data[key] = value === 'on'
    } else if (key !== 'password-confirm' && value) {
      data[key] = value
    }
  }

  return fetch(url, {
    method,
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify(data),
  })
}

export const capitalize = (string) => {
  return string.charAt(0).toUpperCase() + string.slice(1)
}

export class ApiError extends Error {
  constructor(response) {
    const { status, error, message, method, path } = response
    if (status && error && message && method && path) {
      super(`${status} ${error}: ${message} (${method} ${path})`)
    } else if (response) {
      super(`Unknown error: ${JSON.stringify(response)}`)
    } else {
      super('Unknown error')
    }
    this.name = 'ApiError'
  }
}

export const getSuperposables = async () => {
  try {
    const url = `${info.api}/pictures/superposable`
    const response = await fetch(url)
    const superposables = await response.json()

    if (!response.ok && superposables) {
      throw new ApiError(superposables)
    }

    if (!superposables || !superposables.length) {
      throw new Error('no superposable found')
    }

    return superposables
  } catch (error) {
    alert(`${error.name}: ${error.message}`)
  }
}

// send token to server to confirm email
export const sendToken = async (route) => {
  try {
    const token = window.location.href.split('?token=')[1]
    if (!token) {
      throw new Error('Token not found')
    }
    const url = `${info.api}/user/${route}`
    const response = await fetch(url, {
      method: 'POST',
      credentials: 'include',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token }),
    })

    const data = await response.json()
    if (!response.ok) {
      throw new ApiError(data)
    }

    asyncAlert(`Success: ${data.response}`, '/')
  } catch (error) {
    asyncAlert(`${error.name}: ${error.message}`, '/')
  }
}
