import { ApiError, asyncAlert } from './utils.js'

// confirm email address
const confirmEmail = async () => {
  try {
    const token = window.location.href.split('?token=')[1]
    if (!token) {
      throw new Error('Token not found')
    }
    const url = `http://${window.location.hostname}:3000/user/confirm`
    const response = await fetch(url, {
      method: 'POST',
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

confirmEmail()
