// PepePost element
class PepePost extends HTMLElement {
  constructor() {
    super()
    this.attachShadow({ mode: 'open' })

    const title = document.createElement('h2')
    const picture = document.createElement('img')
    this.shadowRoot.appendChild(title)
    this.shadowRoot.appendChild(picture)
  }

  connectedCallback() {
    const picture_id = this.getAttribute('data-picture-id')
    const creation_ts = this.getAttribute('data-creation-ts')
    const author = this.getAttribute('data-author')

    const title = this.shadowRoot.querySelector('h2')
    const picture = this.shadowRoot.querySelector('img')

    const date = new Date(Number(creation_ts) * 1000)
    title.textContent = `${author} at ${date.toLocaleString()}`
    picture.src = `http://localhost:8080/picture/${picture_id}.jpg`
    picture.alt = `Picture ${picture_id}`
  }
}

// Register the PepePost element
customElements.define('pepe-post', PepePost)

// Global variables
let index = 0
const count = 50

// Get posts
async function getPepePosts() {
  try {
    const url = `http://localhost:3000/pictures?index=${index}&count=${count}`
    const response = await fetch(url)

    index += 1
    const posts = await response.json()

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
    console.error(error)
  }
}

// Get posts on scroll
window.addEventListener('scroll', () => {
  if (window.innerHeight + window.scrollY >= document.body.offsetHeight) {
    getPepePosts()
  }
})

// Get posts on load
window.addEventListener('load', getPepePosts)
