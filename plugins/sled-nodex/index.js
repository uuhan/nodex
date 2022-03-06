const { sled } = require('./sled.node')
const fs = require('fs')

let db = new sled("./index.db")

fs.readFile('./sled.node', (err, data) => {
  console.log(
    db.insert(
      new TextEncoder().encode("sled.node").buffer,
      Uint8Array.from(data).buffer,
    )
  )
})

