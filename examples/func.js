const func = require("./func.node")

let ret = func.func({a: "value"}, function (n, m) {
  console.log(arguments.length, 'this:', this, n, m)
}, 100, 200)

console.log(ret)
