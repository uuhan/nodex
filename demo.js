const demo = require("./demo.node")

console.log(demo.utils.func())
console.log(demo.utils.func((value) => {
  console.log(value)
  return "I am from js world."
}))

console.log(new demo.utils.myclass(1))

console.log(demo.key1)
