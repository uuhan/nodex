const promise = require('./promise.node')

let p1 = promise.create(true)
p1.then(resolved => {
  console.log(resolved)
}).catch(rejected => {
  console.log(rejected.message)
})

let p2 = promise.create(false)
p2.then(resolved => {
  console.log(resolved)
}).catch(rejected => {
  console.log(rejected.message)
})

console.log("test napi promise")
