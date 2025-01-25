

function textToColor(text: string): string {

  // string -> 9 char number // hashing
  let hashNum = 0;
  [...text].forEach(i => {
    hashNum = (hashNum << 5) - hashNum + i.charCodeAt(0)
  })
  hashNum = Math.abs(hashNum + text.length)
  let hash = hashNum.toString().padEnd(6, '0').slice(0, 6)

  return "#" + hash
}




export { textToColor }