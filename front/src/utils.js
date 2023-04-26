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

export const toggleConnectedEvent = () =>
  new CustomEvent('toggle-connected', {
    bubbles: true,
    composed: true,
    detail: { connected: !!getCookie('session') },
  })
