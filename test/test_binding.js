const ApiRs = require("../dist/binding.js");
const assert = require("assert");

assert(ApiRs, "The expected function is undefined");

function testBasic()
{
    const result =  ApiRs("hello");
    assert.strictEqual(result, "world", "Unexpected value returned");
}

assert.doesNotThrow(testBasic, undefined, "testBasic threw an expection");

console.log("Tests passed- everything looks OK!");