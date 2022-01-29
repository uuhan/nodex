const descriptor = require("./descriptor.node")

console.log(descriptor.obj.myvalue)
console.log(descriptor.obj.mymethod())

descriptor.obj.myaccessor = 400.
console.log(descriptor.obj.myaccessor)
