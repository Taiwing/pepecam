import { createElement } from './utils.js'

const PepePostTemplate = document.createElement('template')
PepePostTemplate.innerHTML = `
  <style>
    @import "style/pepe-post.css";
    @import "style/pepe-icons.css";
  </style>

  <h2>
    <span id="author-span"></span>
    <span id="date-span"></span>
  </h2>

  <img id="post-picture" />

  <div id="action-bar">
    <div class="post-action">
      <button class="icon" id="like-button">
        <img id="thumbs-up" />
      </button>
      <span id="like-count"></span>
    </div>
    <div class="post-action">
      <button class="icon" id="dislike-button">
        <img id="thumbs-down" />
      </button>
      <span id="dislike-count"></span>
    </div>
    <div class="post-action">
      <button class="icon" id="comment-button">
        <img id="comment" />
      </button>
      <span id="comment-count"></span>
    </div>
  </div>

  <div id="post-comments" hidden>
    <div id="post-comments-feed"></div>
    <form id="post-comments-form">
      <input id="post-comments-input" type="text" placeholder="Type a message..." />
      <button class="icon" id="send-button" type="submit">
        <img id="send" />
      </button>
    </form>
  </div>
`

// PepePost element
class PepePost extends HTMLElement {
  constructor() {
    super()
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.appendChild(PepePostTemplate.content.cloneNode(true))
  }

  connectedCallback() {
    const picture_id = this.getAttribute('data-picture-id')
    const creation_ts = this.getAttribute('data-creation-ts')
    const author = this.getAttribute('data-author')
    const like_count = this.getAttribute('data-like-count')
    const dislike_count = this.getAttribute('data-dislike-count')
    const comment_count = this.getAttribute('data-comment-count')
    const liked = this.getAttribute('data-liked')
    const disliked = this.getAttribute('data-disliked')

    const authorSpan = this.shadowRoot.querySelector('#author-span')
    authorSpan.textContent = `@${author}`

    const dateSpan = this.shadowRoot.querySelector('#date-span')
    const date = new Date(Number(creation_ts) * 1000)
    dateSpan.textContent = ` at ${date.toLocaleString()}`

    const picture = this.shadowRoot.querySelector('#post-picture')
    picture.src = `http://localhost:8080/pictures/${picture_id}.jpg`
    picture.alt = `Picture ${picture_id}`

    const likeCount = this.shadowRoot.querySelector('#like-count')
    likeCount.textContent = like_count

    const dislikeCount = this.shadowRoot.querySelector('#dislike-count')
    dislikeCount.textContent = dislike_count

    const commentCount = this.shadowRoot.querySelector('#comment-count')
    commentCount.textContent = comment_count

    const likeButton = this.shadowRoot.querySelector('#like-button')
    likeButton.addEventListener('click', () => this.like(true))

    const dislikeButton = this.shadowRoot.querySelector('#dislike-button')
    dislikeButton.addEventListener('click', () => this.like(false))

    const commentButton = this.shadowRoot.querySelector('#comment-button')
    commentButton.addEventListener('click', async () => this.toggleFull())

    this.liked = liked === 'true'
    this.disliked = disliked === 'true'
  }

  set liked(value) {
    const thumbsUp = this.shadowRoot.querySelector('#thumbs-up')
    if (value) {
      thumbsUp.setAttribute('filled', '')
    } else {
      thumbsUp.removeAttribute('filled')
    }
  }

  get liked() {
    return this.shadowRoot.querySelector('#thumbs-up').hasAttribute('filled')
  }

  set disliked(value) {
    const thumbsDown = this.shadowRoot.querySelector('#thumbs-down')
    if (value) {
      thumbsDown.setAttribute('filled', '')
    } else {
      thumbsDown.removeAttribute('filled')
    }
  }

  get disliked() {
    return this.shadowRoot.querySelector('#thumbs-down').hasAttribute('filled')
  }

  set full(value) {
    if (value) {
      this.setAttribute('full', '')
    } else {
      this.removeAttribute('full')
    }
  }

  get full() {
    return this.hasAttribute('full')
  }

  set showComments(value) {
    const postComments = this.shadowRoot.querySelector('#post-comments')
    const commentIcon = this.shadowRoot.querySelector('#comment')
    if (value) {
      postComments.removeAttribute('hidden')
      commentIcon.setAttribute('filled', '')
    } else {
      postComments.setAttribute('hidden', '')
      commentIcon.removeAttribute('filled')
    }
  }

  get showComments() {
    return this.shadowRoot.querySelector('#post-comments').hasAttribute('hidden')
  }

  async toggleFull() {
    this.full = !this.full
    if (this.full) {
      const comments = await this.getComments()
      const commentsFeed = this.shadowRoot.querySelector('#post-comments-feed')
      commentsFeed.innerHTML = ''
      for (const comment of comments) {
        const commentElement = document.createElement('div')
        commentElement.classList.add('comment')
        commentElement.textContent = `@${comment.author}: ${comment.content}`
        commentsFeed.appendChild(commentElement)
      }
    }
    this.showComments = this.full
  }

  // Update like and dislike counts
  updateCounts(deleteLike, value) {
    const likeCount = this.shadowRoot.querySelector('#like-count')
    const dislikeCount = this.shadowRoot.querySelector('#dislike-count')

    // Delete like or dislike
    if (this.liked && (deleteLike || !value)) {
      this.liked = false
      likeCount.textContent = Number(likeCount.textContent) - 1
    } else if (this.disliked && (deleteLike || value)) {
      this.disliked = false
      dislikeCount.textContent = Number(dislikeCount.textContent) - 1
    }

    // Add like or dislike
    if (!deleteLike && value) {
      this.liked = true
      likeCount.textContent = Number(likeCount.textContent) + 1
    } else if (!deleteLike && !value) {
      this.disliked = true
      dislikeCount.textContent = Number(dislikeCount.textContent) + 1
    }
  }

  // Like or dislike post
  async like(value) {
    const deleteLike = (this.liked && value) || (this.disliked && !value)
    const picture_id = this.getAttribute('data-picture-id')
    const payload = { picture_id }
    if (!deleteLike) payload.like = value
    const url = 'http://localhost:3000/picture/like'

    try {
      const response = await fetch(url, {
        method: deleteLike ? 'DELETE' : 'PUT',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      })

      if (!response.ok) {
        const { message, error } = await response.json()
        const errorMessage = error || message || JSON.stringify(response)
        alert(`Error: ${errorMessage}`)
        return
      }

      this.updateCounts(deleteLike, value)
    } catch (error) {
      alert(`Error: ${error}`)
    }
  }

  // Get comments
  async getComments() {
    const picture_id = this.getAttribute('data-picture-id')
    const url = `http://localhost:3000/picture/comments?picture=${picture_id}`

    try {
      const response = await fetch(url, { method: 'GET' })

      if (!response.ok) {
        const { message, error } = await response.json()
        const errorMessage = error || message || JSON.stringify(response)
        alert(`Error: ${errorMessage}`)
        return
      }

      return response.json()
    } catch (error) {
      alert(`Error: ${error}`)
    }
  }
}

// Register the PepePost element
customElements.define('pepe-post', PepePost)

const PepeGalleryTemplate = document.createElement('template')
PepeGalleryTemplate.innerHTML = `
  <style>@import "style/pepe-gallery.css"</style>
`

// PepeGallery element
class PepeGallery extends HTMLElement {
  count = 10 // Number of posts to get

  constructor() {
    super()
    this._index = -1
    this._finished = false
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.appendChild(PepeGalleryTemplate.content.cloneNode(true))

    // Get posts on scroll
    window.addEventListener('scroll', this._onScroll.bind(this))

    // Handle login/logout
    window.addEventListener(
      'toggle-connected',
      this._onToggleConnected.bind(this),
    )
  }

  _onScroll() {
    if (this._finished) return

    if (
      window.innerHeight + window.scrollY >= document.body.offsetHeight * 0.75
    ) {
      this.getPepePosts()
    }
  }

  _onToggleConnected() {
    this._index = -1
    this._finished = false
    this.shadowRoot.innerHTML = ''
    this.shadowRoot.appendChild(PepeGalleryTemplate.content.cloneNode(true))
    this.getPepePosts()
  }

  // Get posts
  async getPepePosts() {
    try {
      this._index += 1
      const url =
        `http://localhost:3000/pictures?index=${this._index}&count=${this.count}`
      const response = await fetch(url, { method: 'GET', credentials: 'include' })
      const posts = await response.json()

      if (response.status !== 200 || !posts || posts.length === 0) {
        this._finished = true
        return
      }

      for (const post of posts) {
        const {
          picture_id,
          account_id,
          creation_ts,
          author,
          like_count,
          dislike_count,
          comment_count,
          liked,
          disliked,
        } = post
        const attributes = {
          'data-picture-id': picture_id,
          'data-account-id': account_id,
          'data-creation-ts': creation_ts,
          'data-author': author,
          'data-like-count': like_count,
          'data-dislike-count': dislike_count,
          'data-comment-count': comment_count,
          'data-liked': liked,
          'data-disliked': disliked,
        }
        this.shadowRoot.appendChild(createElement('pepe-post', attributes))
      }
    } catch (error) {
      this._finished = true
      //TODO: remove this log to "respect" the subject
      console.error(error)
    }
  }

  connectedCallback() {
    this.getPepePosts()
  }
}

// Register the PepeHeader element
customElements.define('pepe-gallery', PepeGallery, { extends: 'main' })
