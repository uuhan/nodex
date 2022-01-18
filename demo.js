const demo = require("./demo.node")

console.log(demo.utils.func())
console.log(demo.utils.func((value) => {
  console.log(value)
  return "I am from js world."
}))

console.log(demo.key1)
