
import { ADecoder } from "./utils/ADecoder"



// GLOBALS
declare global {
  interface Window {
    LINKS__GH: string
    LINKS__SRC: string
    LINKS__API: string
    LINKS__ADECODER_ABOUT: string

    APP_VERSION: string

    API_URL: string


    //
    GET_ADECODER: Function
    UNSET_ADECODER: Function
  }
}

window.LINKS__GH = "https://github.com/4Tipsy"
window.LINKS__SRC = ""
window.LINKS__API = ""
window.LINKS__ADECODER_ABOUT = "https://github.com/4Tipsy"

window.APP_VERSION = "v0.1.0"

window.API_URL = "http://localhost:4321"





window.GET_ADECODER = (decodePhrase: string|undefined = undefined) => { return new ADecoder(decodePhrase) }
window.UNSET_ADECODER = () => {
  const i = localStorage.getItem("psw-manager.adecoder_phrase")
  console.log(`UNSET FROM ${i}`)
  localStorage.removeItem("psw-manager.adecoder_phrase")
}