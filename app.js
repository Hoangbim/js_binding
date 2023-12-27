const addon = require("./index.node");

const s = "hello world, today is webnesday!";
console.log("split sentence:", addon.split_and_sha256(s));
