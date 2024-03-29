import { info, ApiError, capitalize, getSuperposables } from './utils.js'

const PepeUploadTemplate = document.createElement('template')

PepeUploadTemplate.innerHTML = `
  <link rel="stylesheet" href="style/global.css">
  <link rel="stylesheet" href="style/pepe-upload.css">
  <div id="pepe-upload">
    <video autoplay hidden></video>
    <div id="preview" hidden>
      <canvas id="preview-canvas" alt="Preview" />
      <img id="superposable-img" hidden />
      <canvas id="capture-canvas" hidden />
    </div>
    <div id="toolbar">
      <select>
        <option value="">--Select a superposable--</option>
      </select>
      <label id="import-button" class="button" disabled="">
        Import Picture
        <input
          type="file"
          accept="image/jpeg,image/png"
          capture="environment"
          disabled
        >
      </label>
      <button id="capture-button" disabled>Capture</button>
      <button id="upload-button" disabled>Upload</button>
      <button id="cancel-button" disabled>Cancel</button>
    </div>
  </div>
`

// PepeUpload element
class PepeUpload extends HTMLElement {
  static get observedAttributes() {
    return ['superposable', 'camera']
  }

  static get elements() {
    return {
      superposableImg: '#superposable-img',
      superposableSelect: '#toolbar select',
      importButton: '#import-button',
      importInput: '#import-button input',
      captureButton: '#capture-button',
      uploadButton: '#upload-button',
      cancelButton: '#cancel-button',
      preview: '#preview',
      previewCanvas: '#preview-canvas',
      video: 'video',
      captureCanvas: '#capture-canvas',
    }
  }

  constructor() {
    super()
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.append(PepeUploadTemplate.content.cloneNode(true))

    for (const attribute of this.constructor.observedAttributes) {
      this.__defineGetter__(attribute, () => this.getAttribute(attribute))
      this.__defineSetter__(attribute, (v) => this.setAttribute(attribute, v))
    }
    for (const element in this.constructor.elements) {
      this.__defineGetter__(
        element,
        () => this.shadowRoot.querySelector(this.constructor.elements[element])
      )
    }
    this.superposable = ''
    this.camera = ''
    this.picture = null

    this.importInput.addEventListener('change', () => {
      this.picture = this.importInput.files[0]
      this.importInput.value = ''
      this.showPreview()
    })
    this.superposableImg.addEventListener('load', () => {
      if (this.picture) this.showPreview()
    })
    this.superposableImg.addEventListener('error', () => {
      if (this.picture) this.showPreview()
    })
    this.uploadButton.addEventListener('click', () => this.upload())
    this.cancelButton.addEventListener('click', () => this.cancel())
    this.captureButton.addEventListener('click', () => this.capture())
  }

  async initializeCamera() {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({
        video: true,
        audio: false,
      })
      this.video.srcObject = stream
      this.camera = 'on'
    } catch (_) {
      this.camera = ''
    }
  }

  attributeChangedCallback(name, oldValue, newValue) {
    switch (name) {
      case 'superposable':
        this.disableImportButton(!newValue)
        this.captureButton.disabled = !newValue || this.camera !== 'on'
        if (!newValue) {
          this.uploadButton.disabled = true
          this.cancelButton.disabled = true
          this.superposableSelect.value = ''
          this.superposableImg.src = ''
        } else {
          const { front, superposables_dir } = info
          const picture = `${front}/${superposables_dir}/${newValue}.png`
          this.superposableImg.src = picture
        }
        break
      case 'camera':
        this.video.hidden = newValue !== 'on'
        this.captureButton.disabled = newValue != 'on' || !this.superposable
        break
    }
  }

  disableImportButton(disabled) {
    if (disabled) {
      this.importButton.setAttribute('disabled', '')
    } else {
      this.importButton.removeAttribute('disabled')
    }
    this.importInput.disabled = disabled
  }

  showPreview() {
    if (this.camera) this.camera = 'off'
    const image = new Image()
    image.onload = () => {
      if (image.width < info.superposables_side || image.height < info.superposables_side) {
        this.hidePreview()
        alert('Image too small')
        return
      }
      this.previewCanvas.width = image.width
      this.previewCanvas.height = image.height
      const context = this.previewCanvas.getContext('2d')
      context.drawImage(image, 0, 0)
      if (this.superposable) {
        context.drawImage(
          this.superposableImg,
          0,
          image.height - this.superposableImg.height
        )
      }
      this.preview.hidden = false
      this.uploadButton.disabled = !this.superposable
      this.cancelButton.disabled = false
    }
    image.src = URL.createObjectURL(this.picture)
  }

  hidePreview() {
    if (this.camera) this.camera = 'on'
    this.preview.hidden = true
    this.previewCanvas.getContext('2d').clearRect(0, 0, 0, 0)
    this.uploadButton.disabled = true
    this.cancelButton.disabled = true
  }

  cancel() {
    this.picture = null
    this.hidePreview()
  }

  _reset() {
    this.camera = this.camera ? 'off' : ''
    this.cancel()
    this.superposable = ''
  }

  async upload() {
    try {
      const url = `${info.api}/picture/${this.superposable}`
      const response = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'image/jpeg' },
        credentials: 'include',
        body: this.picture,
      })
      const picture = await response.json()

      if (!response.ok) {
        throw new ApiError(picture)
      }

      const event = new CustomEvent('pepe-upload', {
        bubbles: true,
        composed: true,
        detail: picture,
      })
      this.dispatchEvent(event)

      this._reset()
    } catch (error) {
      alert(`${error.name}: ${error.message}`)
    }
  }

  capture() {
    const context = this.captureCanvas.getContext('2d')
    let width = this.video.videoWidth
    let height = this.video.videoHeight
    const min = Math.min(width, height, info.superposables_side)
    if (min < info.superposables_side) {
      width *= info.superposables_side / min
      height *= info.superposables_side / min
    }
    this.captureCanvas.width = width
    this.captureCanvas.height = height

    context.drawImage(
      this.video,
      0,
      0,
      this.video.videoWidth,
      this.video.videoHeight,
      0,
      0,
      width,
      height
    )

    this.importInput.value = ''
    this.captureCanvas.toBlob((blob) => {
      this.picture = blob
      this.showPreview()
    }, 'image/jpeg')
  }

  connectedCallback() {
    getSuperposables().then((superposables) => {
      for (const superposable of superposables) {
        const option = document.createElement('option')
        option.value = superposable
        option.text = capitalize(superposable)
        this.superposableSelect.append(option)
      }
    })
    this.superposableSelect.addEventListener('input', (event) => {
      const { value } = event.target
      this.superposable = value
    })
    this.initializeCamera()
  }
}

// Register the PepeUpload element
customElements.define('pepe-upload', PepeUpload)
