const demo = require("./demo.node")

console.log(demo.utils.func())
console.log(demo.utils.func(() => {
  console.log("demo callback")
}))

console.log(demo.key1)
