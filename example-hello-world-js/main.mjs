// @ts-check
import * as slug from "./slug.mjs";

/** @type slug.Buf<"Greeting", slug.Char> */
const greetingChars = slug.buf_empty();
slug.greet("world", {
  "buf(Chars char)": greetingChars,
});
console.log(greetingChars.items.join(""));
