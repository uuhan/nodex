const { sled } = require('./sled.node')
const fs = require('fs')

let db = new sled("./index.db")

fs.readFile('./sled.node', (err, data) => {
  let key = new TextEncoder().encode("sled.node").buffer

  let item = db.insert(
    key,
    Uint8Array.from(data).buffer,
  )
  if (item) {
    console.log("insert: ", item)
  }

  console.log("get: ", db.get(key).byteLength)

  let old = db.remove(key)
  if (old) {
    console.log('remove old:', old);
  }
})

