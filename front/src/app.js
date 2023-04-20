// PepePost element
class PepePost extends HTMLElement {
  constructor() {
    super()
    this.attachShadow({ mode: 'open' })

    const style = document.createElement('style')
    style.textContent = '@import "style/pepe-post.css";'
    const title = document.createElement('h2')
    const picture = document.createElement('img')
    this.shadowRoot.append(style, title, picture)
  }

  connectedCallback() {
    const picture_id = this.getAttribute('data-picture-id')
    const creation_ts = this.getAttribute('data-creation-ts')
    const author = this.getAttribute('data-author')

    const title = this.shadowRoot.querySelector('h2')
    const nameSpan = document.createElement('span')
    const dateSpane = document.createElement('span')
    const date = new Date(Number(creation_ts) * 1000)
    nameSpan.textContent = `@${author}`
    dateSpane.textContent = ` at ${date.toLocaleString()}`
    title.append(nameSpan, dateSpane)

    const picture = this.shadowRoot.querySelector('img')
    picture.src = `http://localhost:8080/pictures/${picture_id}.jpg`
    picture.alt = `Picture ${picture_id}`
  }
}

// Register the PepePost element
customElements.define('pepe-post', PepePost)

// Global variables
let index = -1
let finished = false
const count = 50

// Get posts
async function getPepePosts() {
  try {
    index += 1
    const url = `http://localhost:3000/pictures?index=${index}&count=${count}`
    const response = await fetch(url)
    const posts = await response.json()

    if (response.status !== 200 || !posts || posts.length === 0) {
      finished = true
      return
    }

    const feed = document.querySelector('.feed')
    for (const post of posts) {
      const postElement = document.createElement('pepe-post')
      const { picture_id, account_id, creation_ts, author } = post
      postElement.setAttribute('data-picture-id', picture_id)
      postElement.setAttribute('data-account-id', account_id)
      postElement.setAttribute('data-creation-ts', creation_ts)
      postElement.setAttribute('data-author', author)
      feed.appendChild(postElement)
    }
  } catch (error) {
    finished = true
    //TODO: remove this log to "respect" the subject
    console.error(error)
  }
}

// Get posts on scroll
window.addEventListener('scroll', () => {
  if (finished) return

  if (window.innerHeight + window.scrollY >= document.body.offsetHeight * 0.75) {
    getPepePosts()
  }
})

// Get posts on load
window.addEventListener('load', getPepePosts)
