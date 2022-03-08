const demo = require("./demo.node")

console.log(demo.utils.func(() => {}))
console.log(demo.utils.func((value) => {
  console.log(value)
  return "I am from js world."
}))

try {
  let obj = new demo.utils.myclass()
  console.log(demo.utils.instance)
  console.log(demo.key1)
} catch (e) {
  console.error("new myclass: ", e)
}

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

try {
  demo.throw_error()
} catch (e) {
  console.error('throw_error:', e)
}
