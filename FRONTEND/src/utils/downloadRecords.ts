
import { type User } from "../types/User"
import { type PswRecord } from "../types/PswRecord"




function downloadRecords(user: User, records: PswRecord[], recordsSize: number) {

  let recordsSizePretty = (recordsSize / 1000).toFixed(2)

  let text = 
`
All records (${recordsSizePretty} KB) are encoded via CryptoJS.DES method
Decode em by yourself somehow ;)



@@@ USER:
\`\`\`
${JSON.stringify(user, null, 2)}
\`\`\`



@@@ RECORDS:
\`\`\`
${JSON.stringify(records, null, 2)}
\`\`\`
`



  // trigger download
  let a = document.createElement('a')
  a.download = 'records.txt' // file name
  a.href = URL.createObjectURL(new Blob([text]))
  a.style.display = 'none'
  a.click()
  a.remove()
}




export { downloadRecords }