const demo = require("./demo.node")

console.log(demo.utils.func())
console.log(demo.utils.func((value) => {
  console.log(value)
  return "I am from js world."
}))

let obj = new demo.utils.myclass(1)
console.log(demo.utils.instance)
console.log(demo.key1)

let lable = Symbol()
console.log(demo.names({
  1: 1,
  a: 2,
}))

demo.thread((result) => {
  console.log("Get From Thread:", result)
})

console.log('array_index[0]:', demo.buffer_index(Buffer.from([1,2,3,4,5])))
console.log(demo.buffer)

demo.delay(() => {
  console.log("delayed")
})
