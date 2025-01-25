
import CryptoJS from "crypto-js"




class ADecoder {

  private decodePhrase: string


  constructor(decodePhrase: string | undefined = undefined) {



    // override mode
    if (decodePhrase) {
      this.decodePhrase = decodePhrase
      return
    }


    const cachedVal = localStorage.getItem("psw-manager.adecoder_phrase")
    if (cachedVal != null) {
      this.decodePhrase = cachedVal


    } else {
      const input = prompt("ADecoder") || "unset"
      localStorage.setItem("psw-manager.adecoder_phrase", input) // set
      this.decodePhrase = input
    }
  }






  public decode(it: string): string {
    try {
      let v = CryptoJS.DES.decrypt(it, this.decodePhrase).toString( CryptoJS.enc.Utf8 )
      if (v.length == 0) {
        v = "<@error>"
      }
      return v
    } catch(e) {
      console.log("Error while decoding")
      console.error(e)
      return "<@error>"
    }
  }

  public encode(it: string): string {
    return CryptoJS.DES.encrypt(it, this.decodePhrase).toString()
  }


  
}





export { ADecoder }