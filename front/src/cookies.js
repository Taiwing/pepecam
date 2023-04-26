const getCookies = () =>
  document
  .cookie
  .split(';')
  .map(cookie => cookie.split('='))
  .reduce((acc, [key, value]) => ({
    ...acc,
    [key.trim()]: decodeURIComponent(value.trim()),
  }), {})

const getCookie = (name) => {
  const cookies = getCookies()
  return cookies[name] ? JSON.parse(cookies[name]) : undefined
}

export { getCookies, getCookie }
