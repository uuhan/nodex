const demo = require("./demo.node")

console.log(demo.utils.func())
demo.utils.func((value) => {
  console.log(value)
})

console.log(demo.key1)
