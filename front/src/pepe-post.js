import { createElement } from './utils.js'

const PepePostTemplate = document.createElement('template')
PepePostTemplate.innerHTML = `
  <link rel="stylesheet" href="style/pepe-post.css" />
  <link rel="stylesheet" href="style/pepe-icons.css" />

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
    <div id="delete-button-div" class="post-action" hidden>
      <button class="icon" id="delete-button">
        <img id="trash" />
      </button>
    </div>
  </div>

  <div id="post-comments" hidden>
    <div id="post-comments-feed"></div>
    <div id="post-comments-form">
      <input
        id="post-comments-input"
        type="text"
        placeholder="Type a message..."
      />
      <button class="icon" id="send-button" type="submit">
        <img id="send" />
      </button>
    </div>
  </div>
`

// PepePost element
class PepePost extends HTMLElement {
  static get observedAttributes() {
    return [
      'data-picture-id',
      'data-creation-ts',
      'data-author',
      'data-like-count',
      'data-dislike-count',
      'data-comment-count',
      'data-liked',
      'data-disliked',
    ]
  }

  constructor() {
    super()
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.appendChild(PepePostTemplate.content.cloneNode(true))
  }

  connectedCallback() {
    const likeButton = this.shadowRoot.querySelector('#like-button')
    likeButton.addEventListener('click', () => this.like(true))

    const dislikeButton = this.shadowRoot.querySelector('#dislike-button')
    dislikeButton.addEventListener('click', () => this.like(false))

    const commentButton = this.shadowRoot.querySelector('#comment-button')
    commentButton.addEventListener('click', async () => this.toggleFull())

    const sendButton = this.shadowRoot.querySelector('#send-button')
    sendButton.addEventListener('click', async () => this.sendComment())

    if (this.hasAttribute('show-delete-button')) {
      this.shadowRoot.querySelector('#delete-button-div').removeAttribute('hidden')
      const deleteButton = this.shadowRoot.querySelector('#delete-button')
      deleteButton.addEventListener('click', async () => this.deletePost())
    }
  }

  attributeChangedCallback(name, oldValue, newValue) {
    switch (name) {
      case 'data-picture-id':
        const picture = this.shadowRoot.querySelector('#post-picture')
        picture.src = `http://localhost:8080/pictures/${newValue}.jpg`
        picture.alt = `Picture ${newValue}`
        this.full = false
        const comments = this.shadowRoot.querySelector('#post-comments-feed')
        comments.innerHTML = ''
        break
      case 'data-creation-ts':
        const dateSpan = this.shadowRoot.querySelector('#date-span')
        const date = new Date(Number(newValue) * 1000)
        dateSpan.textContent = ` at ${date.toLocaleString()}`
        break
      case 'data-author':
        const authorSpan = this.shadowRoot.querySelector('#author-span')
        authorSpan.textContent = `@${newValue}`
        break
      case 'data-like-count':
        const likeCount = this.shadowRoot.querySelector('#like-count')
        likeCount.textContent = newValue
        break
      case 'data-dislike-count':
        const dislikeCount = this.shadowRoot.querySelector('#dislike-count')
        dislikeCount.textContent = newValue
        break
      case 'data-comment-count':
        const commentCount = this.shadowRoot.querySelector('#comment-count')
        commentCount.textContent = newValue
        break
      case 'data-liked':
        this.liked = newValue === 'true'
        break
      case 'data-disliked':
        this.disliked = newValue === 'true'
        break
    }
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

  createComment(feed, { author, content }) {
    const commentElement = document.createElement('div')
    commentElement.classList.add('comment')
    commentElement.textContent = `@${author}: ${content}`
    feed.appendChild(commentElement)
  }

  async toggleFull() {
    this.full = !this.full
    if (this.full) {
      const comments = await this.getComments()
      const commentsFeed = this.shadowRoot.querySelector('#post-comments-feed')
      commentsFeed.innerHTML = ''
      for (const comment of comments) this.createComment(commentsFeed, comment)
    }
    this.showComments = this.full
  }

  // Update like and dislike counts
  updateLikeCounts(deleteLike, value) {
    const likeCount = this.getAttribute('data-like-count')
    const dislikeCount = this.getAttribute('data-dislike-count')

    // Delete like or dislike
    if (this.liked && (deleteLike || !value)) {
      this.liked = false
      this.setAttribute('data-like-count', Number(likeCount) - 1)
    } else if (this.disliked && (deleteLike || value)) {
      this.disliked = false
      this.setAttribute('data-dislike-count', Number(dislikeCount) - 1)
    }

    // Add like or dislike
    if (!deleteLike && value) {
      this.liked = true
      this.setAttribute('data-like-count', Number(likeCount) + 1)
    } else if (!deleteLike && !value) {
      this.disliked = true
      this.setAttribute('data-dislike-count', Number(dislikeCount) + 1)
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
        throw error || message || JSON.stringify(response)
      }

      this.updateLikeCounts(deleteLike, value)
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
        throw error || message || JSON.stringify(response)
      }

      return response.json()
    } catch (error) {
      alert(`Error: ${error}`)
    }
  }

  // Send comment
  async sendComment() {
    const url = 'http://localhost:3000/picture/comment'
    const picture_id = this.getAttribute('data-picture-id')
    const content = this.shadowRoot.querySelector('#post-comments-input').value
    const payload = { picture_id, comment: content }

    try {
      if (!content) return
      const response = await fetch(url, {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      })

      if (!response.ok) {
        const { message, error } = await response.json()
        throw message || error || JSON.stringify(response)
      }

      this.shadowRoot.querySelector('#post-comments-input').value = ''
      const comment = await response.json()
      const commentsFeed = this.shadowRoot.querySelector('#post-comments-feed')
      this.createComment(commentsFeed, comment)
      const commentCount = this.getAttribute('data-comment-count')
      this.setAttribute('data-comment-count', Number(commentCount) + 1)
    } catch (error) {
      alert(`Error: ${error}`)
    }
  }

  // Delete post
  async deletePost() {
    const picture_id = this.getAttribute('data-picture-id')
    const url = 'http://localhost:3000/picture'

    try {
      const response = await fetch(url, {
        method: 'DELETE',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ picture_id }),
      })

      if (!response.ok) {
        const { message, error } = await response.json()
        throw error || message || JSON.stringify(response)
      }

      //TODO: Remove post from feed with an event
      //this.remove()
    } catch (error) {
      alert(`Error: ${error}`)
    }
  }
}

// Register the PepePost element
customElements.define('pepe-post', PepePost)
