const tsfn = require('./tsfn.node')

tsfn.create(function (value) {
  console.log(this, value)
})
