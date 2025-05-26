
import { type User } from "../types/User"
import { type TypedPswRecord, type RawPswRecord } from "../types/PswRecord"
import { ADecoder } from "./ADecoder"



function downloadRecords(user: User, records: Array<TypedPswRecord|RawPswRecord>, recordsSize: number) {

  let recordsSizePretty = (recordsSize / 1000).toFixed(2) + "KB"
  let adecoder = new ADecoder()


  let decodedRecords = records.map((r) => {
    if (r._record_type == 'TYPED') {
      r.account_name = adecoder.decode(r.account_name)
      r.encoded_login = adecoder.decode(r.encoded_login)
      r.encoded_password = adecoder.decode(r.encoded_password)
    }
    if (r._record_type == 'RAW') {
      r.raw_content = adecoder.decode(r.raw_content)
    }
    return r
  })



  let text = 
`
// All ${user.user_id}\`s records (${recordsSizePretty})
// Decoded via CryptoJS.DES, ADecoder.decodePhrase = "${localStorage.getItem('psw-manager.adecoder_phrase')}"

${JSON.stringify(decodedRecords, null, 2)}
`



  // trigger download
  let a = document.createElement('a')
  a.download = 'records.json5' // file name
  a.href = URL.createObjectURL(new Blob([text]))
  a.style.display = 'none'
  a.click()
  a.remove()
}




export { downloadRecords }