// @ts-check

import * as slug from "./slug.mjs";
/** @type slug.Char[] */
const chars = [];
let _ = slug.greet(chars, "world");
console.log(chars.join(""));
