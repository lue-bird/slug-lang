// @ts-check

import * as slug from "./slug.mjs";
/** @type slug.Buf<"Chars", slug.Char> */
const chars = slug.buf_empty();
slug.greet("world", { buf$Chars$tychar: chars });
console.log(chars.items.join(""));
