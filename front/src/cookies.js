const getCookies = () =>
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

const getCookie = (name) => {
  const cookies = getCookies()
  return cookies[name] ? JSON.parse(cookies[name]) : undefined
}

export { getCookies, getCookie }
