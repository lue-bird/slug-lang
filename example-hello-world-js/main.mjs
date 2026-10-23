// @ts-check

import * as slug from "./slug.mjs"

const chars = [];
let _ = slug.greet(chars, "world");
console.log(chars.join(""));
