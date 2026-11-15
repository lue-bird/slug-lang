/** @typedef {number} P32 */
/** @typedef {number} U32 */
/** @typedef {number} I32 */
/** @typedef {number} F32 */
/** @typedef {string} Char
 * Assumed to contain exactly one codepoint
 */
/** @typedef {string} Str
 * Assumed to contain at least one codepoint and at most 2^32-1 bytes
 */
/** @template $Yes @typedef {{ yes: $Yes } | { no: void }} Opt */
/** @typedef {{ less: void } | { equal: void } | { greater: void }} Order */
/** @template $Origin, $Item @typedef {{ items: $Item[], counts: ArrayBuffer } & { readonly origin?: $Origin }} Buf
 * .counts holds Uint16s
 */
/** @template $Item @typedef {$Item[]} Unset_slice
 * Assumed to contain an empty array (with spare capacity)
 */
/** @template $Origin, $Item
 * @typedef {U32 & { readonly origin?: $Origin, readonly item?: $Item }} Slot
 */
/** @template $Origin, $Item
 * @typedef {{ start: U32, length: U32 } & { readonly origin?: $Origin, readonly item?: $Item }} Span
 */

const U16$MAX = 65535;
const I32$MIN = -2147483648;
const I32$MAX = 2147483647;
const U32$MAX = 4294967296;
const F32$MAX = 3.40282347e38;
const F32$MIN = -F32$MAX;
const F32$MAX_DIGITS = 55;
/** @param {F32} n @returns {string} */
export function f32$to_string(n) {
  // This is piles of hacks. If you know something better, please tell me!

  // digit handled separately because BigInt.toString does not emit -0
  let f32_string = n < 0 ? "-" : "";
  // all functions on Number use exponent notation for >10^21
  f32_string += BigInt(Math.trunc(Math.abs(n))).toString();
  if (!Number.isInteger(n)) {
    // calculate fraction such that there are at least F32_MAX_DIGITS.
    // Technically toFixed doesn't promise support for >20 digits
    // but all environments I've tested do actually support this up to 100
    f32_string += n.toFixed(F32$MAX_DIGITS - f32_string.length + 1).slice(1); // drop the 0 in 0.
    while (f32_string.endsWith("0")) {
      f32_string = f32_string.slice(0, -1);
    }
  }
  return f32_string;
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @returns {Uint16Array}
 */
export function buf$counts_view(buf) {
  return new Uint16Array(buf.counts);
}
/** @template Origin, Item
 * @param {Buf<Origin, Item>} buf
 * @param {Opt<Span<Origin, Item>>} span
 */
export function buf$optSpanSlice(buf, span) {
  if ("no" in span) {
    return [];
  } else {
    return buf$spanSlice(buf, span.yes);
  }
}
/** @template Origin, Item
 * @param {Buf<Origin, Item>} buf
 * @param {Span<Origin, Item>} span
 */
export function buf$spanSlice(buf, span) {
  return buf.items.slice(span.start, span.start + span.length);
}

/** @param {Order} _ @returns {void} */
export function order$drop(_) {}
/** @param {Order} _ @returns {void} */
export function order$clone(_) {}
/** @template $Yes @param {Opt<$Yes>} opt @param {{ dropYes: (_: $Yes) => void }} $ opt @returns {void} */
export function opt$drop(opt, $) {
  if ("yes" in opt) {
    $.dropYes(opt.yes);
  }
}
/** @template $Yes @param {Opt<$Yes>} opt @param {{ cloneYes: (_: $Yes) => void }} $ opt @returns {void} */
export function opt$clone(opt, $) {
  if ("yes" in opt) {
    $.cloneYes(opt.yes);
  }
}

/** @param {P32} p @returns {U32} */
export function p32_to_u32(p) {
  return p;
}
/** @param {P32} p @param {U32} u mul @returns {P32} */
export function p32_mul_clamp(p, u) {
  return Math.min(U32$MAX, p * u);
}
/** @param {P32} left @param{P32} right @returns {Order} */
export function p32_order(left, right) {
  return left < right
    ? { less: undefined }
    : left > right
      ? { greater: undefined }
      : { equal: undefined };
}
/** @param {P32} p @param{U32} u add @returns {P32} */
export function p32_add_clamp(p, u) {
  return Math.min(U32$MAX, p + u);
}
/** @param {U32} i @returns {Opt<P32>} */
export function u32_to_p32(i) {
  return i === 0 ? { no: undefined } : { yes: i };
}
/** @param {U32} u @returns {F32} */
export function u32_round_to_nearest_f32_else_even(u) {
  return Math.fround(u);
}
/** @param {U32} u @returns {P32} */
export function u32_successor_clamp(u) {
  return Math.min(U32$MAX, u + 1);
}
/** @param {U32} a @param {U32} b add @returns {U32} */
export function u32_add_clamp(a, b) {
  return Math.min(U32$MAX, a + b);
}
/** @param {U32} u @param {I32} i @returns {U32} */
export function u32_add_i32_clamp(u, i) {
  return Math.max(0, Math.min(U32$MAX, u + i));
}
/** @param {U32} a @param {U32} b @returns {U32} */
export function u32_mul_clamp(a, b) {
  return Math.min(U32$MAX, a * b);
}
/** @param {U32} a @param {U32} b @returns {U32} */
export function u32_mul_wrap(a, b) {
  /// >>> 0 undoes the conversion from unsigned to signed
  // (step 4 in https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.imul)
  return Math.imul(a, b) >>> 0;
}
/** @param {U32} base @param {P32} exponent @returns {U32} */
export function u32_pow_clamp(base, exponent) {
  return Math.min(U32$MAX, Math.pow(base, exponent));
}
/** @param {U32} left @param {U32} right sides @returns {Order} */
export function u32_order(left, right) {
  return left < right
    ? { less: undefined }
    : left > right
      ? { greater: undefined }
      : { equal: undefined };
}
/** @param {U32} u @returns {P32} */
export function u32_to_i32_clamp(u) {
  return Math.min(I32$MAX, u);
}

/** @param {I32} i @returns {Opt<U32>} */
export function i32_to_u32(i) {
  return i < 0 ? { no: undefined } : { yes: i };
}
/** @param {I32} i @returns {F32} */
export function i32_round_to_nearest_f32_else_even(i) {
  return Math.fround(i);
}
/** @param {I32} i @returns {U32} */
export function i32_abs_to_u32(i) {
  return Math.abs(i);
}
/** @param {I32} i @returns {U32} */
export function i32_negate_clamp(i) {
  return Math.max(I32$MIN, -i);
}
/** @param {I32} left @param {I32} right @returns {Order} */
export function i32_order(left, right) {
  return left < right
    ? { less: undefined }
    : left > right
      ? { greater: undefined }
      : { equal: undefined };
}
/** @param {I32} a @param {I32} b add @returns {I32} */
export function i32_add_clamp(a, b) {
  return Math.max(I32$MIN, Math.min(I32$MAX, a + b));
}
/** @param {I32} a @param {I32} b @returns {I32} */
export function i32_mul_clamp(a, b) {
  return Math.max(I32$MIN, Math.min(I32$MAX, a * b));
}
/** @param {I32} base @param {P32} exponent @returns {I32} */
export function i32_pow_clamp(base, exponent) {
  return Math.max(I32$MIN, Math.min(I32$MAX, Math.pow(base, exponent)));
}
/** @returns {F32} */
export function f32_pi() {
  return Math.PI;
}
/** @param {F32} a @param {F32} b @returns {F32} */
export function f32_add_clamp(a, b) {
  return a + b;
}
/** @param {F32} a @param {F32} b @returns {F32} */
export function f32_mul_clamp(a, b) {
  return Math.max(Number.MIN_VALUE, Math.min(Number.MAX_VALUE, a * b));
}
/** @param {F32} n @param {F32} by @returns {F32} */
export function f32_div_clamp(n, by) {
  return by === 0 ? 0 : Math.max(F32$MIN, Math.min(F32$MAX, Math.fround(n / by)));
}
/** @param {F32} n @returns {Opt<F32>} */
export function f32_square_root(n) {
  return n < 0.0 ? { no: undefined } : { yes: Math.sqrt(n) };
}
/** @param {F32} base @param {I32} exponent @returns {Opt<F32>} */
export function f32_pow_i32(base, exponent) {
  return f32_pow(base, exponent);
}
/** @param {F32} base @param {F32} exponent @returns {Opt<F32>} */
export function f32_pow(base, exponent) {
  const power = Math.pow(base, exponent);
  return Number.isFinite(power) && power >= F32$MIN && power <= F32$MAX
    ? { yes: Math.fround(power) }
    : { no: undefined };
}
/** @param {F32} n @returns {F32} */
export function f32_exp(n) {
  return Math.min(Number.MAX_VALUE, Math.exp(n));
}
/** @param {F32} n @returns {Opt<F32>} */
export function f32_ln(n) {
  if (n <= 0) {
    return { no: undefined };
  } else {
    const ln_result = Math.log(n);
    return Number.isFinite(ln_result)
      ? { yes: Math.fround(ln_result) }
      : { no: undefined };
  }
}
/** @param {F32} n @returns {F32} */
export function f32_sin(n) {
  return Math.fround(Math.sin(n));
}
/** @param {F32} n @returns {F32} */
export function f32_tan(n) {
  return Math.fround(Math.tan(n));
}
/** @param {F32} n @returns {F32} */
export function f32_cos(n) {
  return Math.fround(Math.cos(n));
}
/** @param {F32} n @returns {F32} */
export function f32_atan(n) {
  return Math.fround(Math.atan(n));
}
/** @param {F32} n @returns {F32} */
export function f32_abs(n) {
  return Math.abs(n);
}
/** @param {F32} n @returns {F32} */
export function f32_negate(n) {
  return -n;
}
/** @param {F32} n @returns {F32} */
export function f32_round_down(n) {
  return Math.floor(n);
}
/** @param {F32} n @returns {F32} */
export function f32_round_nearest_else_even(n) {
  const int_part_is_even = Math.trunc(n) % 2 === 0;
  return n < 0 === int_part_is_even
    ? // negative even or positive odd, e.g.
      // -12.5 -> -12
      // 1.5 -> 2
      Math.round(n)
    : // positive even or negative odd, e.g.
      // -1.5 -> -2
      // 12.5 -> 12
      -Math.round(-n);
}
/** @param {F32} n @returns {F32} */
export function f32_round_up(n) {
  return Math.ceil(n);
}
/** @param {F32} n @returns {I32} */
export function f32_round_up_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, f32_round_up(n)));
}
/** @param {F32} n @returns {I32} */
export function f32_round_nearest_else_even_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, f32_round_nearest_else_even(n)));
}
/** @param {F32} n @returns {F32} */
export function f32_round_nearest_else_away_from_0(n) {
  return Math.sign(n) * Math.round(Math.abs(n));
}
/** @param {F32} n @returns {I32} */
export function f32_round_nearest_else_away_from_0_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, f32_round_nearest_else_away_from_0(n)));
}
/** @param {F32} n @returns {F32} */
export function f32_round_away_from_0(n) {
  return n < 0 ? Math.floor(n) : Math.ceil(n);
}
/** @param {F32} n @returns {I32} */
export function f32_round_away_from_0_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, f32_round_away_from_0(n)));
}
/** @param {F32} n @returns {I32} */
export function f32_round_down_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, Math.floor(n)));
}
/** @param {F32} n @returns {F32} */
export function f32_round_toward_0(n) {
  return Math.trunc(n);
}
/** @param {F32} n @returns {I32} */
export function f32_round_toward_0_to_i32_clamp(n) {
  return Math.max(I32$MIN, Math.min(I32$MAX, Math.trunc(n)));
}
/** @param {F32} left @param {F32} right sides @returns {Order} */
export function f32_order(left, right) {
  return left < right
    ? { less: undefined }
    : left > right
      ? { greater: undefined }
      : { equal: undefined };
}

/** @param {Char} char @returns {U32} */
export function char_to_u32(char) {
  return char.charCodeAt(0);
}

/** @param {Str} str @returns {P32} */
export function str_char_count(str) {
  let count = 0;
  for (const _ of str) {
    count++;
  }
  return count;
}
/** @param {Str} str @returns {P32} */
export function str_utf8_length(str) {
  // probably wasteful but couldn't find something native other than this
  // new TextEncoder().encode(str).length
  return new Blob([str]).size;
}
/** @param {Str} str @returns {[Char, Opt<Str>]} */
export function str_start(str) {
  const first = str.charAt(0);
  return [
    first,
    str.length === first.length ? { no: undefined } : { yes: str.slice(first.length) },
  ];
}
/** @param {Str} str @returns {[Opt<Str>, Char]} */
export function str_end(str) {
  const lastOneOrTwo = str.slice(-2);
  return lastOneOrTwo.charAt(0).length === 2
    ? // last codepoint consists of 2 code units
      [str.length === 2 ? { no: undefined } : { yes: str.slice(0, -2) }, lastOneOrTwo]
    : // last codepoint consists of 1 code unit
      [str.length === 1 ? { no: undefined } : { yes: str.slice(0, -1) }, str.slice(-1)];
}

/** @template $Result @param {never} never @returns {$Result} */
export function choice_empty_to(never) {
  return never;
}

/** @template $Origin, $Item
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, dropItem?: (_: $Item) => void }} $
 * @param {Slot<$Origin, $Item>} slot @returns {U32} */
export function slot_index(slot, $) {
  slot$drop($["buf(Origin Item)"], slot, $.dropItem);
  return slot;
}
/** @template $Origin, $Item @param {Slot<$Origin, $Item>} slot @returns {Span<$Origin, $Item>} */
export function slot_to_span(slot) {
  return { start: slot, length: 1 };
}
/** @template $Origin, $Item
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, dropItem?: (_: $Item) => void }} $
 * @param {Span<$Origin, $Item>} span @returns {U32} */
export function span_start_index(span, $) {
  span$drop($["buf(Origin Item)"], span, $.dropItem);
  return span.start;
}
/** @template $Origin, $Item
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, dropItem?: (_: $Item) => void }} $
 * @param {Span<$Origin, $Item>} span @returns {P32} */
export function span_length(span, $) {
  span$drop($["buf(Origin Item)"], span, $.dropItem);
  return span.length;
}
/** @template $Origin, $Item
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, dropItem?: (_: $Item) => void }} $
 * @param {Opt<Span<$Origin, $Item>>} span @returns {U32} */
export function opt_span_length(span, $) {
  return "no" in span ? 0 : span_length(span.yes, $);
}
/** @template $Origin, $Item @param {Span<$Origin, $Item>} span @returns {[Slot<$Origin, $Item>, Opt<Span<$Origin, $Item>>]} */
export function span_start(span) {
  return [
    span.start,
    span.length >= 2
      ? { yes: { start: span.start + 1, length: span.length - 1 } }
      : { no: undefined },
  ];
}
/** @template $Origin, $Item @param {Span<$Origin, $Item>} span @returns {[Opt<Span<$Origin, $Item>>, Slot<$Origin, $Item>]} */
export function span_end(span) {
  return [
    span.length >= 2
      ? { yes: { start: span.start, length: span.length - 1 } }
      : { no: undefined },
    span.start + span.length - 1,
  ];
}
/** @template $Origin, $Item
 * @param {Span<$Origin, $Item>} span
 * @param{P32} length
 * @returns {[Span<$Origin, $Item>, Opt<Span<$Origin, $Item>>]} */
export function span_start_of_length_positive(span, length) {
  return [
    { start: span.start, length: Math.min(span.length, length) },
    span.length <= length
      ? { no: undefined }
      : {
          yes: {
            start: span.start + length,
            length: span.length - length,
          },
        },
  ];
}
/** @template $Origin, $Item
 * @param {Span<$Origin, $Item>} span
 * @param{P32} length
 * @returns {[Opt<Span<$Origin, $Item>>, Span<$Origin, $Item>]} */
export function span_end_of_length_positive(span, length) {
  return [
    span.length <= length
      ? { no: undefined }
      : {
          yes: {
            start: span.start,
            length: span.length - length,
          },
        },
    {
      start: span.start + Math.max(0, span.length - length),
      length: length,
    },
  ];
}
/** @template $Origin, $Item, $State
 * @param {Span<$Origin, $Item>} span
 * @param{{ up: void } | { down: void }} direction
 * @param{$State} state
 * @param{(state: $State, slot: Slot<$Origin, $Item>) => $State} step
 * @returns {$State} */
export function span_step(span, direction, state, step) {
  if ("up" in direction) {
    for (let i = span.start; i < span.start + span.length; i++) {
      state = step(state, i);
    }
  } else {
    for (let i = span.start + span.length - 1; i >= span.start; i--) {
      state = step(state, i);
    }
  }
  return state;
}
/** @template $Origin, $Item, $State
 * @param {Opt<Span<$Origin, $Item>>} span
 * @param{{ up: void } | { down: void }} direction
 * @param{$State} state
 * @param{(state: $State, slot: Slot<$Origin, $Item>) => $State} step
 * @returns {$State} */
export function opt_span_step(span, direction, state, step) {
  return "no" in span ? state : span_step(span.yes, direction, state, step);
}
/** @template $Done, $Going, $Origin, $Item
 * @param {Span<$Origin, $Item>} span
 * @param{{ up: void } | { down: void }} direction
 * @param {$Going} state
 * @param {(state: $Going, slot: Slot<$Origin, $Item>) => { going: $Going } | { done: $Done }} step
 * @returns {{ going: $Going } | { done: [$Done, Opt<Span<$Origin, $Item>>] }} */
export function span_step_while(span, direction, state, step) {
  if ("up" in direction) {
    for (let i = span.start; i < span.start + span.length; i++) {
      const step_result = step(state, i);
      if ("done" in step_result) {
        const rest_length = span.start + span.length - 1 - i;
        return {
          done: [
            step_result.done,
            rest_length >= 1
              ? { yes: { start: i + 1, length: rest_length } }
              : { no: undefined },
          ],
        };
      }
      state = step_result.going;
    }
  } else {
    for (let i = span.start + span.length - 1; i >= span.start; i--) {
      const step_result = step(state, i);
      if ("done" in step_result) {
        const rest_length = i - span.start;
        return {
          done: [
            step_result.done,
            rest_length >= 1
              ? { yes: { start: 0, length: rest_length } }
              : { no: undefined },
          ],
        };
      }
      state = step_result.going;
    }
  }
  return { going: state };
}
/** @template $Done, $Going, $Origin, $Item
 * @param {Opt<Span<$Origin, $Item>>} span
 * @param{{ up: void } | { down: void }} direction
 * @param {$Going} state
 * @param {(state: $Going, slot: Slot<$Origin, $Item>) => { going: $Going } | { done: $Done }} step
 * @returns {{ going: $Going } | { done: [$Done, Opt<Span<$Origin, $Item>>] }} */
export function opt_span_step_while(span, direction, state, step) {
  if ("no" in span) return { going: state };
  return span_step_while(span.yes, direction, state, step);
}

/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Slot<$Origin, $Item>} slot
 * @returns {void} */
export function slot$clone(buf, slot) {
  buf$counts_view(buf)[slot] = Math.min(U16$MAX, buf$counts_view(buf)[slot] + 1);
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Span<$Origin, $Item>} span
 * @returns {void} */
export function span$clone(buf, span) {
  for (let i = span.start; i < span.start + span.length; i++) {
    slot$clone(buf, i);
  }
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Slot<$Origin, $Item>} slot
 * @param {((item: $Item) => void) | undefined} dropItem
 * @returns {void} */
export function slot$drop(buf, slot, dropItem) {
  if (buf$counts_view(buf)[slot] === 1) {
    if (slot + 1 < buf.items.length) {
      const item = /** @type $Item */ (buf.items.pop());
      while (buf$counts_view(buf)[buf.items.length - 1] === 0) {
        buf.items.length -= 1;
      }
      buf.counts.resize(buf.items.length); // optional
      dropItem?.(item);
    } else {
      dropItem?.(buf.items[slot]);
    }
  } else {
    buf$counts_view(buf)[slot] =
      buf$counts_view(buf)[slot] === U16$MAX ? U16$MAX : buf$counts_view(buf)[slot] - 1;
  }
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{Span<$Origin, $Item>} span
 * @param {((item: $Item) => void) | undefined} dropItem
 * @returns {void} */
export function span$drop(buf, span, dropItem) {
  for (let i = span.start; i < span.start + span.length; i++) {
    if (buf$counts_view(buf)[i] === 1) {
      dropItem?.(buf.items[i]);
      buf$counts_view(buf)[i] = 0;
    } else {
      buf$counts_view(buf)[i] =
        buf$counts_view(buf)[i] == U16$MAX ? U16$MAX : buf$counts_view(buf)[i] - 1;
    }
  }
  if (span.start + span.length === buf.items.length) {
    while (buf$counts_view(buf)[buf.items.length - 1] === 0) {
      buf.items.length -= 1;
    }
    // optional
    buf.counts.resize(buf.items.length);
  }
}
/** @template $Item, $Origin @returns {Buf<$Origin, $Item>} */
export function buf_empty() {
  // is maxByteLength: U32$MAX okay? mdn only recommends < U32$MAX/4
  // If not, what are the alternatives? number[]?
  return { items: [], counts: new ArrayBuffer(0, { maxByteLength: U32$MAX }) };
}
/** @template $Item, $Origin @param {Unset_slice<$Item>} slice @returns {Buf<$Origin, $Item>} */
export function buf_reuse(slice) {
  return { items: slice, counts: new ArrayBuffer(0, { maxByteLength: U32$MAX }) };
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} _ @returns {void} */
export function buf_pre_allocation_rid(_) {}
/** @template $Item, $Origin
 * @param {Span<$Origin, $Item>} span
 * @param {(item: $Item) => $Item} item_alter
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @returns {void} */
export function span_alter(span, item_alter, $) {
  for (let i = span.start; i < span.start + span.length; i++) {
    $["buf(Origin Item)"].items[i] = item_alter(item(i, $));
  }
}
/** @template $Item, $Origin
 * @param {Opt<Span<$Origin, $Item>>} span
 * @param {(item: $Item) => $Item} item_alter
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @returns {void} */
export function opt_span_alter(span, item_alter, $) {
  if ("yes" in span) {
    span_alter(span.yes, item_alter, $);
  }
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{U32} length
 * @returns {void} */
export function buf_pre_allocate_at_least(length, buf) {
  buf.counts.resize(buf.items.length + length * Uint16Array.BYTES_PER_ELEMENT);
  buf.items.length += length;
  buf.items.length -= length;
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{U32} length
 * @returns {{ success: void } | { out_of_memory: void }} */
export function buf_try_pre_allocate_at_least(buf, length) {
  buf_pre_allocate_at_least(length, buf);
  return { success: undefined };
}
/** @template $Item, $Origin
 * @param {$Item} new_item
 * @returns {Slot<$Origin, $Item>}
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} $ */
export function add(new_item, $) {
  const new_index = $["buf(Origin Item)"].items.length;
  $["buf(Origin Item)"].items.push(new_item);
  $["buf(Origin Item)"].counts.resize($["buf(Origin Item)"].items.length);
  buf$counts_view($["buf(Origin Item)"])[$["buf(Origin Item)"].items.length - 1] = 1;
  if ($["buf(Origin Item)"].items.length > U32$MAX)
    throw Error(
      "Array length " +
        $["buf(Origin Item)"].items.length +
        " not representable as a u32",
    );
  return new_index;
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} $
 * @param {$Item} new_item
 * @returns {Slot<$Origin, $Item>} */
export function insert(new_item, $) {
  const existing_unset_index = buf$counts_view($["buf(Origin Item)"]).findIndex(
    (count) => count === 0,
  );
  if (existing_unset_index >= 0) {
    $["buf(Origin Item)"].items[existing_unset_index] = new_item;
    buf$counts_view($["buf(Origin Item)"])[existing_unset_index] = 1;
    return existing_unset_index;
  } else {
    return add(new_item, $);
  }
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} $
 * @param {U32} index
 * @param {$Item} new_item
 * @returns {Slot<$Origin, $Item>} */
export function set_or_insert(index, new_item, $) {
  if (buf$counts_view($["buf(Origin Item)"])[index] === 0) {
    $["buf(Origin Item)"].items[index] = new_item;
    buf$counts_view($["buf(Origin Item)"])[index] = 1;
    return index;
  }
  return insert(new_item, $);
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param {Span<$Origin, $Item>} span
 * @param {$Item} new_item
 * @returns {Span<$Origin, $Item>} */
export function span_add(span, new_item, $) {
  span_move_to_end(span, $);
  add(new_item, $);
  return {
    start: $["buf(Origin Item)"].items.length - 1 - span.length,
    length: span.length + 1,
  };
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param {Opt<Span<$Origin, $Item>>} span
 * @param {$Item} new_item
 * @returns {Span<$Origin, $Item>} */
export function opt_span_add(span, new_item, $) {
  if ("no" in span) {
    const new_index = add(new_item, $);
    return slot_to_span(new_index);
  }
  return span_add(span.yes, new_item, $);
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param {Slot<$Origin, $Item>} slot
 * @returns {$Item} */
export function item(slot, $) {
  if (buf$counts_view($["buf(Origin Item)"])[slot] === 1) {
    if (slot + 1 < $["buf(Origin Item)"].items.length) {
      buf$counts_view($["buf(Origin Item)"])[slot] = 0;
      return $["buf(Origin Item)"].items[slot];
    } else {
      const item = /** @type $Item */ ($["buf(Origin Item)"].items.pop());
      while (
        buf$counts_view($["buf(Origin Item)"])[$["buf(Origin Item)"].items.length - 1] ===
        0
      ) {
        $["buf(Origin Item)"].items.length -= 1;
      }
      $["buf(Origin Item)"].counts.resize($["buf(Origin Item)"].items.length); // optional
      return item;
    }
  } else {
    buf$counts_view($["buf(Origin Item)"])[slot] =
      buf$counts_view($["buf(Origin Item)"])[slot] === U16$MAX
        ? U16$MAX
        : buf$counts_view($["buf(Origin Item)"])[slot] - 1;
    const item = $["buf(Origin Item)"].items[slot];
    $.cloneItem?.(item);
    return item;
  }
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} $
 * @param {Slot<$Origin, $Item>} slot
 * @param {$Item} new_item
 * @returns {$Item} */
export function replace(slot, new_item, $) {
  const old_item = $["buf(Origin Item)"].items[slot];
  $["buf(Origin Item)"].items[slot] = new_item;
  return old_item;
}
/** @template $In, $Item, $Origin, $Out
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} $
 * @param {Slot<$Origin, $Item>} slot
 * @param {$In} in_
 * @param {(inø: $In, item:$Item) => [$Out, $Item]} step
 * @returns {$Out} */
export function item_step(slot, in_, step, $) {
  const item = $["buf(Origin Item)"].items[slot];
  const [stepped_out, stepped_item] = step(in_, item);
  $["buf(Origin Item)"].items[slot] = stepped_item;
  return stepped_out;
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} $
 * @param {Slot<$Origin, $Item>} slot_a
 * @param {Slot<$Origin, $Item>} slot_b
 * @returns {void} */
export function swap(slot_a, slot_b, $) {
  const a_item = $["buf(Origin Item)"].items[slot_a];
  $["buf(Origin Item)"].items[slot_a] = $["buf(Origin Item)"].items[slot_b];
  $["buf(Origin Item)"].items[slot_b] = a_item;
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param {Span<$Origin, $Item>} span
 * @returns {Span<$Origin, $Item>} */
export function span_move_to_end(span, $) {
  if (span.start + span.length === $["buf(Origin Item)"].items.length) return span;

  for (let i = span.start; i < span.start + span.length; i++) {
    $["buf(Origin Item)"].items.push(item(i, $));
    $["buf(Origin Item)"].counts.resize($["buf(Origin Item)"].items.length);
    buf$counts_view($["buf(Origin Item)"])[$["buf(Origin Item)"].items.length - 1] = 1;
  }
  if ($["buf(Origin Item)"].items.length > U32$MAX)
    throw Error(
      "Array length " +
        $["buf(Origin Item)"].items.length +
        " not representable as a u32",
    );
  return { start: $["buf(Origin Item)"].items.length - span.length, length: span.length };
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param {Opt<Span<$Origin, $Item>>} span
 * @returns {Opt<Span<$Origin, $Item>>} */
export function opt_span_move_to_end(span, $) {
  if ("no" in span) return { no: undefined };
  return { yes: span_move_to_end(span.yes, $) };
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param {Span<$Origin, $Item>} span
 * @returns {Span<$Origin, $Item>} */
export function span_move_to_unset(span, $) {
  if (span.start + span.length < $["buf(Origin Item)"].items.length) return span;

  let unset_length = 0;
  for (let i = 0; i < $["buf(Origin Item)"].items.length; i++) {
    if (buf$counts_view($["buf(Origin Item)"])[i] === 0) {
      unset_length++;
      if (unset_length === span.length) {
        const unset_start = i - span.length + 1;
        for (let unset_i = 0; unset_i < span.length; unset_i++) {
          $["buf(Origin Item)"].items[unset_start + unset_i] = item(
            span.start + unset_i,
            $,
          );
        }
        $["buf(Origin Item)"].items.length -= span.length;
        $["buf(Origin Item)"].counts.resize($["buf(Origin Item)"].items.length); // optional
        while (
          buf$counts_view($["buf(Origin Item)"])[
            $["buf(Origin Item)"].items.length - 1
          ] === 0
        ) {
          $["buf(Origin Item)"].items.length -= 1;
        }
        return { start: unset_start, length: span.length };
      }
    } else {
      unset_length = 0;
    }
  }
  return span;
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param {Opt<Span<$Origin, $Item>>} span
 * @returns {Opt<Span<$Origin, $Item>>} */
export function opt_span_move_to_unset(span, $) {
  if ("no" in span) return span;
  return { yes: span_move_to_unset(span.yes, $) };
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} $
 * @param {Span<$Origin, $Item>} span
 * @returns {void} */
export function span_reverse(span, $) {
  // TODO check if non-allocating version is faster (probably is)
  const slice = $["buf(Origin Item)"].items.slice(span.start, span.start + span.length);
  slice.reverse();
  $["buf(Origin Item)"].items.splice(span.start, span.length, ...slice);
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} $
 * @param {Opt<Span<$Origin, $Item>>} span
 * @returns {void} */
export function opt_span_reverse(span, $) {
  if ("no" in span) return;
  span_reverse(span.yes, $);
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} $
 * @param{Span<$Origin, $Item>} span
 * @param{(left: $Item, right: $Item) => Order} item_order
 * @returns {Span<$Origin, $Item>} */
export function span_sort(span, item_order, $) {
  const span_indexes = Array.from(span, (_, i) => span.start + i);
  span_indexes.sort((left, right) => {
    const order = item_order(
      $["buf(Origin Item)"].items[left],
      $["buf(Origin Item)"].items[right],
    );
    return "less" in order ? -1 : "greater" in order ? 1 : 0;
  });
  span_indexes.forEach((buf_index_sorted, old_offset_within_span) => {
    if (span.start + old_offset_within_span < buf_index_sorted) {
      // swap
      const old = $["buf(Origin Item)"].items[span.start + old_offset_within_span];
      $["buf(Origin Item)"].items[span.start + old_offset_within_span] =
        $["buf(Origin Item)"].items[buf_index_sorted];
      $["buf(Origin Item)"].items[buf_index_sorted] = old;
    }
  });
  return span;
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item> }} $
 * @param{Opt<Span<$Origin, $Item>>} span
 * @param{(left: $Item, right: $Item) => Order} item_order
 * @returns {Opt<Span<$Origin, $Item>>} */
export function opt_span_sort(span, item_order, $) {
  if ("yes" in span) {
    span_sort(span.yes, item_order, $);
  }
  return span;
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param{Span<$Origin, $Item>} start
 * @param{Span<$Origin, $Item>} end
 * @returns {Span<$Origin, $Item>} */
export function span_add_own_span(start, end, $) {
  if (start.start + start.length === end.start) {
    return { start: start.start, length: start.length + end.length };
  }
  span_move_to_end(start, $);
  // move end span to end after the start items
  span_move_to_end(end, $);
  return {
    start: $["buf(Origin Item)"].items.length - start.length - end.length,
    length: start.length + end.length,
  };
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param{Span<$Origin, $Item>} start
 * @param{Opt<Span<$Origin, $Item>>} end
 * @returns {Span<$Origin, $Item>} */
export function span_add_own_opt_span(start, end, $) {
  if ("no" in end) {
    return start;
  }
  return span_add_own_span(start, end.yes, $);
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param{Opt<Span<$Origin, $Item>>} start
 * @param{Span<$Origin, $Item>} end
 * @returns {Span<$Origin, $Item>} */
export function opt_span_add_own_span(start, end, $) {
  if ("no" in start) {
    return end;
  }
  return span_add_own_span(start.yes, end, $);
}
/** @template $Item, $Origin
 * @param {{ "buf(Origin Item)": Buf<$Origin, $Item>, cloneItem?: (_: $Item) => void }} $
 * @param{Opt<Span<$Origin, $Item>>} start
 * @param{Opt<Span<$Origin, $Item>>} end
 * @returns {Opt<Span<$Origin, $Item>>} */
export function opt_span_add_own_opt_span(start, end, $) {
  if ("no" in start) return end;
  if ("no" in end) return start;
  return { yes: span_add_own_span(start.yes, end.yes, $) };
}
/** @template $Origin
 * @param {{ "buf(Origin char)": Buf<$Origin, Char> }} $
 * @param {Str} new_str
 * @returns {Span<$Origin, Char>} */
export function add_str_chars(new_str, $) {
  const new_start = $["buf(Origin char)"].items.length;
  for (const new_char of new_str) {
    $["buf(Origin char)"].items.push(new_char);
    $["buf(Origin char)"].counts.resize($["buf(Origin char)"].items.length);
    buf$counts_view($["buf(Origin char)"])[$["buf(Origin char)"].items.length - 1] = 1;
  }
  if ($["buf(Origin char)"].items.length > U32$MAX)
    throw Error(
      "Array length " +
        $["buf(Origin char)"].items.length +
        " not representable as a u32",
    );
  return {
    start: new_start,
    length: $["buf(Origin char)"].items.length - new_start,
  };
}
/** @template $Origin
 * @param {{ "buf(Origin char)": Buf<$Origin, Char> }} $
 * @param {Span<$Origin, Char>} span
 * @param {Str} new_str
 * @returns {Span<$Origin, Char>} */
export function span_add_str_chars(span, new_str, $) {
  span_move_to_end(span, {
    "buf(Origin Item)": $["buf(Origin char)"],
    cloneItem: (_) => {},
  });
  const new_start = $["buf(Origin char)"].items.length;
  for (const new_char of new_str) {
    $["buf(Origin char)"].items.push(new_char);
    $["buf(Origin char)"].counts.resize($["buf(Origin char)"].items.length);
    buf$counts_view($["buf(Origin char)"])[$["buf(Origin char)"].items.length - 1] = 1;
  }
  if ($["buf(Origin char)"].items.length > U32$MAX)
    throw Error(
      "Array length " +
        $["buf(Origin char)"].items.length +
        " not representable as a u32",
    );
  return {
    start: new_start - span.length,
    length: span.length + ($["buf(Origin char)"].items.length - new_start),
  };
}
/** @template $Origin
 * @param {{ "buf(Origin char)": Buf<$Origin, Char> }} $
 * @param {Opt<Span<$Origin, Char>>} span
 * @param {Str} new_str
 * @returns {Span<$Origin, Char>} */
export function opt_span_add_str_chars(span, new_str, $) {
  if ("no" in span) {
    return add_str_chars(new_str, $);
  }
  return span_add_str_chars(span.yes, new_str, $);
}
/** @template $Origin
 * @param {{ "buf(Origin char)": Buf<$Origin, Char> }} $
 * @param {Span<$Origin, Char>} span
 * @param {U32} n
 * @returns {Span<$Origin, Char>} */
export function span_add_u32_chars(span, n, $) {
  return span_add_str_chars(span, n.toString(), $);
}
/** @template $Origin
 * @param {{ "buf(Origin char)": Buf<$Origin, Char> }} $
 * @param {Opt<Span<$Origin, Char>>} span
 * @param {U32} n
 * @returns {Span<$Origin, Char>} */
export function opt_span_add_u32_chars(span, n, $) {
  return opt_span_add_str_chars(span, n.toString(), $);
}
/** @template $Origin
 * @param {{ "buf(Origin char)": Buf<$Origin, Char> }} $
 * @param {Span<$Origin, Char>} span
 * @param {I32} n
 * @returns {Span<$Origin, Char>} */
export function span_add_i32_chars(span, n, $) {
  return span_add_str_chars(span, n.toString(), $);
}
/** @template $Origin
 * @param {{ "buf(Origin char)": Buf<$Origin, Char> }} $
 * @param {Opt<Span<$Origin, Char>>} span
 * @param {I32} n
 * @returns {Span<$Origin, Char>} */
export function opt_span_add_i32_chars(span, n, $) {
  return opt_span_add_str_chars(span, n.toString(), $);
}
/** @template $Origin
 * @param {{ "buf(Origin char)": Buf<$Origin, Char> }} $
 * @param {Span<$Origin, Char>} span
 * @param {F32} n
 * @returns {Span<$Origin, Char>} */
export function span_add_f32_chars(span, n, $) {
  return span_add_str_chars(span, f32$to_string(n), $);
}
/** @template $Origin
 * @param {{ "buf(Origin char)": Buf<$Origin, Char> }} $
 * @param {Opt<Span<$Origin, Char>>} span
 * @param {F32} n
 * @returns {Span<$Origin, Char>} */
export function opt_span_add_f32_chars(span, n, $) {
  return opt_span_add_str_chars(span, f32$to_string(n), $);
}
/** @template $Item, $Origin @param {Buf<$Origin, $Item>} _ @returns {Unset_slice<$Item>} */
export function buf_to_unset(_) {
  // preferably we would reuse the allocation but since Bufs can be shared
  // and we can't determine if it's the only instance left we can't do this.
  // (technically we could add a ref count to Buf but this seems annoying for FFI for example)
  return [];
}

/** @template $Item @param {U32} length @returns {Unset_slice<$Item>} */
export function unset_slice_allocate_length(length) {
  const array = Array(length);
  array.length = 0;
  return array;
}
/** @template $Item, $New_item @param {Unset_slice<$Item>} unset_slice @returns {Unset_slice<$New_item>} */
export function unset_slice_cast_or_rid_and_allocate(unset_slice) {
  // for once equal type sizes come in clutch
  return /** @type Unset_slice<any> */ (unset_slice);
}
