import { capitalize } from './utils.js'

const PepeUploadTemplate = document.createElement('template')
/*
    <div id="pepe-upload-dropzone">
      <p>Drop files here</p>
    </div>
*/
PepeUploadTemplate.innerHTML = `
  <link rel="stylesheet" href="style/global.css">
  <link rel="stylesheet" href="style/pepe-upload.css">
  <div id="pepe-upload">
    <div id="pepe-upload-capture" hidden>
      <video id="pepe-upload-capture-video" autoplay></video>
    </div>
    <div id="pepe-upload-preview" hidden>
      <canvas id="pepe-upload-preview-canvas"></canvas>
    </div>
    <div id="pepe-upload-toolbar">
      <select id="pepe-upload-toolbar-select">
        <option value="">--Select a superposable--</option>
      </select>
      <label class="button">
        Import Picture
        <input type="file">
      </label>
      <button id="pepe-capture-button" disabled>Capture</button>
      <button id="pepe-upload-button" disabled>Upload</button>
      <button id="pepe-cancel-button" disabled>Cancel</button>
    </div>
  </div>
`

// PepeUpload element
class PepeUpload extends HTMLElement {
  constructor () {
    super()
    this.attachShadow({ mode: 'open' })
    this.shadowRoot.appendChild(PepeUploadTemplate.content.cloneNode(true))
  }

  async getSuperposables () {
    try {
      const url = 'http://localhost:3000/pictures/superposable'
      const response = await fetch(url)
      const superposables = await response.json()

      if (response.status !== 200 || !superposables || !superposables.length) {
        //TODO: think about the appropriate error handling
        throw new Error('Error fetching superposables')
      }

      const select = this.shadowRoot
        .querySelector('#pepe-upload-toolbar-select')

      for (const superposable of superposables) {
        const option = document.createElement('option')
        option.value = superposable
        option.text = capitalize(superposable)
        select.appendChild(option)
      }
    } catch (error) {
      console.error(error)
    }
  }

  connectedCallback () {
    this.getSuperposables()
  }
}

// Register the PepeUpload element
customElements.define('pepe-upload', PepeUpload)
