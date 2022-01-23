const promise = require('./promise.node')

let p = promise.create()

p.then(resolved => {
  console.log(resolved)
})

console.log("test napi promise")
