const descriptor = require("./descriptor.node")

console.log(descriptor.obj.myvalue)
console.log(descriptor.obj.mymethod())

descriptor.obj.myaccessor = 200.
console.log(descriptor.obj.myaccessor)
