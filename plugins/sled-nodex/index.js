const { sled } = require('./sled.node')
const fs = require('fs')

let db = new sled("./index.db")

fs.readFile('./sled.node', (err, data) => {
  let key = new TextEncoder().encode("sled.node").buffer
  console.log(
    db.insert(
      key,
      Uint8Array.from(data).buffer,
    )
  )
  console.log("got: ", db.get(key).byteLength)
})

