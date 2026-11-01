// @ts-check
import * as slug from "./slug.mjs";

/** @type slug.Buf<"Chars", slug.Char> */
const chars = slug.buf_empty();
slug.greet("world", {
  buf$Chars$tychar$end: chars,
  drop$Chars: (_) => {},
  clone$Chars: (_) => {},
});
console.log(chars.items.join(""));
