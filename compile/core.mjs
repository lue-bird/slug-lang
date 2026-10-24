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
/** @template $In, $Out @typedef {(_: $In) => $Out} Fn
 * TODO inline
 */
/** @template $Yes @typedef {{ yes: $Yes } | { no: void }} Opt */
/** @typedef {{ less: void } | { equal: void } | { greater: void }} Order */
/** @type{unique symbol}
 * Originally this was simply set to null but this prevented values
 * passed in from js (which can be null) to be handled correctly.
 */
const SYMBOL$UNSET = Symbol();
/** @template $Origin, $Item @typedef {(typeof SYMBOL$UNSET | $Item)[] & { readonly origin?: $Origin }} Buf
 * TODO change to { items: $Item[], counts: TypedU16Array }
 */
/** @template $Item @typedef {(typeof SYMBOL$UNSET | $Item)[]} Unset_slice
 * Assumed to contain an empty array (with spare capacity)
 */
/** @template $Origin, $Item
 * @typedef {U32 & { readonly origin?: $Origin, readonly item?: $Item }} Slot
 */
/** @template $Origin, $Item
 * @typedef {{ start: U32, length: U32 } & { readonly origin?: $Origin, readonly item?: $Item }} Span
 */

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
 * @param {Span<$Origin, $Item>} span
 * @returns {$Item[]}
 */
export function buf$span_slice(buf, span) {
  return /** @type $Item[] */ (buf.slice(span.start, span.start + span.length));
}

/** @param {P32} p @returns {U32} */
export function p32_to_u32(p) {
  return p;
}
/** @param {{ p: P32, u: U32, }} mul @returns {P32} */
export function p32_mul_clamp(mul) {
  return Math.min(U32$MAX, mul.p * mul.u);
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
/** @param {{ a: U32; b: U32 }} add @returns {U32} */
export function u32_add_clamp(add) {
  return Math.min(U32$MAX, add.a + add.b);
}
/** @param {{ u: U32, i: I32, }} add @returns {U32} */
export function u32_add_i32_clamp(add) {
  return Math.max(0, Math.min(U32$MAX, add.u + add.i));
}
/** @param {{ a: U32, b: U32, }} mul @returns {U32} */
export function u32_mul_clamp(mul) {
  return Math.min(U32$MAX, mul.a * mul.b);
}
/** @param {{ a: U32, b: U32, }} mul @returns {U32} */
export function u32_mul_wrap(mul) {
  /// >>> 0 undoes the conversion from unsigned to signed
  // (step 4 in https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.imul)
  return Math.imul(mul.a, mul.b) >>> 0;
}
/** @param {{ base: U32, exponent: P32, }} pow @returns {U32} */
export function u32_pow_clamp(pow) {
  return Math.min(U32$MAX, Math.pow(pow.base, pow.exponent));
}
/** @param {{ left: U32, right: U32, }} sides @returns {Order} */
export function u32_order(sides) {
  return sides.left < sides.right
    ? { less: undefined }
    : sides.left > sides.right
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
/** @param {{ left: I32, right: I32, }} sides @returns {Order} */
export function i32_order(sides) {
  return sides.left < sides.right
    ? { less: undefined }
    : sides.left > sides.right
      ? { greater: undefined }
      : { equal: undefined };
}
/** @param {{ a: I32; b: I32 }} add @returns {I32} */
export function i32_add_clamp(add) {
  return Math.max(I32$MIN, Math.min(I32$MAX, add.a + add.b));
}
/** @param {{ a: I32, b: I32, }} mul @returns {I32} */
export function i32_mul_clamp(mul) {
  return Math.max(I32$MIN, Math.min(I32$MAX, mul.a * mul.b));
}
/** @param {{ base: I32, exponent: P32, }} power @returns {I32} */
export function i32_pow_clamp(power) {
  return Math.max(I32$MIN, Math.min(I32$MAX, Math.pow(power.base, power.exponent)));
}
/** @returns {F32} */
export function f32_pi() {
  return Math.PI;
}
/** @param {{ a: F32; b: F32 }} add @returns {F32} */
export function f32_add_clamp(add) {
  return add.a + add.b;
}
/** @param {{ a: F32, b: F32, }} mul @returns {F32} */
export function f32_mul_clamp(mul) {
  return Math.max(Number.MIN_VALUE, Math.min(Number.MAX_VALUE, mul.a * mul.b));
}
/** @param {{ n: F32, by: F32, }} div @returns {F32} */
export function f32_div_clamp(div) {
  return div.by === 0
    ? 0
    : Math.max(F32$MIN, Math.min(F32$MAX, Math.fround(div.n / div.by)));
}
/** @param {F32} n @returns {Opt<F32>} */
export function f32_square_root(n) {
  return n < 0.0 ? { no: undefined } : { yes: Math.sqrt(n) };
}
/** @param {{ base: F32, exponent: I32, }} pow @returns {Opt<F32>} */
export function f32_pow_i32(pow) {
  return f32_pow(pow);
}
/** @param {{ base: F32, exponent: F32, }} pow @returns {Opt<F32>} */
export function f32_pow(pow) {
  const power = Math.pow(pow.base, pow.exponent);
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
/** @param {{ left: F32, right: F32, }} sides @returns {Order} */
export function f32_order(sides) {
  return sides.left < sides.right
    ? { less: undefined }
    : sides.left > sides.right
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
/** @param {Str} str @returns {{ start: Char, after: Opt<Str>, }} */
export function str_start(str) {
  const first = str.charAt(0);
  return {
    start: first,
    after:
      str.length === first.length ? { no: undefined } : { yes: str.slice(first.length) },
  };
}
/** @param {Str} str @returns {{ end: Char, before: Opt<Str>, }} */
export function str_end(str) {
  const lastOneOrTwo = str.slice(-2);
  return lastOneOrTwo.charAt(0).length === 2
    ? // last codepoint consists of 2 code units
      {
        end: lastOneOrTwo,
        before: str.length === 2 ? { no: undefined } : { yes: str.slice(0, -2) },
      }
    : // last codepoint consists of 1 code unit
      {
        end: str.slice(-1),
        before: str.length === 1 ? { no: undefined } : { yes: str.slice(0, -1) },
      };
}

/** @template $Yes @param {$Yes} yes @returns {Opt<$Yes>} */
export function opt_yes(yes) {
  return { yes: yes };
}

/** @template $Result @param {never} never @returns {$Result} */
export function choice_empty_to(never) {
  return never;
}

/** @template $In, $Out @param {{ fn: Fn<$In, $Out>, inø: $In, }} call @returns {$Out} */
export function call(call) {
  return call.fn(call.inø);
}

/** @template $Origin, $Item @param {Slot<$Origin, $Item>} slot @returns {{ slot: Slot<$Origin, $Item>, index: U32 }} */
export function slot_index(slot) {
  return { slot: slot, index: slot };
}
/** @template $Origin, $Item @param {Slot<$Origin, $Item>} slot @returns {Span<$Origin, $Item>} */
export function slot_to_span(slot) {
  return { start: slot, length: 1 };
}
/** @template $Origin, $Item @param {Span<$Origin, $Item>} span @returns {{ span: Span<$Origin, $Item>, index: U32, }} */
export function span_start_index(span) {
  return { span: span, index: span.start };
}
/** @template $Origin, $Item @param {Span<$Origin, $Item>} span @returns {{ span: Span<$Origin, $Item>, length: P32, }} */
export function span_length(span) {
  return { span: span, length: span.length };
}
/** @template $Origin, $Item @param {Opt<Span<$Origin, $Item>>} span @returns {{ span: Opt<Span<$Origin, $Item>>, length: U32, }} */
export function opt_span_length(span) {
  return { span: span, length: "no" in span ? 0 : span.yes.length };
}
/** @template $Origin, $Item @param {Span<$Origin, $Item>} span @returns {{ start: Slot<$Origin, $Item>, after: Opt<Span<$Origin, $Item>>, }} */
export function span_start(span) {
  return {
    start: span.start,
    after:
      span.length >= 2
        ? { yes: { start: span.start + 1, length: span.length - 1 } }
        : { no: undefined },
  };
}
/** @template $Origin, $Item @param {Span<$Origin, $Item>} span @returns {{ end: Slot<$Origin, $Item>, before: Opt<Span<$Origin, $Item>>, }} */
export function span_end(span) {
  return {
    end: span.start + span.length - 1,
    before:
      span.length >= 2
        ? { yes: { start: span.start, length: span.length - 1 } }
        : { no: undefined },
  };
}
/** @template $Origin, $Item @param {{ span: Span<$Origin, $Item>, length: P32, }} take @returns {{ start: Span<$Origin, $Item>, after: Opt<Span<$Origin, $Item>>, }} */
export function span_start_of_length_positive(take) {
  return {
    start: { start: take.span.start, length: Math.min(take.span.length, take.length) },
    after:
      take.span.length <= take.length
        ? { no: undefined }
        : {
            yes: {
              start: take.span.start + take.length,
              length: take.span.length - take.length,
            },
          },
  };
}
/** @template $Origin, $Item @param {{ span: Span<$Origin, $Item>, length: P32, }} take @returns {{ end: Span<$Origin, $Item>, before: Opt<Span<$Origin, $Item>>, }} */
export function span_end_of_length_positive(take) {
  return {
    end: {
      start: take.span.start + Math.max(0, take.span.length - take.length),
      length: take.length,
    },
    before:
      take.span.length <= take.length
        ? { no: undefined }
        : {
            yes: {
              start: take.span.start,
              length: take.span.length - take.length,
            },
          },
  };
}
/** @template $Origin, $Item, $State
 * @param {Span<$Origin, $Item>} span
 * @param{{ up: void } | { down: void }} direction
 * @param{$State} state
 * @param{Fn<{ slot: Slot<$Origin, $Item>, state: $State, }, $State>} step
 * @returns {$State} */
export function span_step(span, direction, state, step) {
  if ("up" in direction) {
    for (let i = span.start; i < span.start + span.length; i++) {
      state = step({ state: state, slot: i });
    }
  } else {
    for (let i = span.start + span.length - 1; i >= span.start; i--) {
      state = step({ state: state, slot: i });
    }
  }
  return state;
}
/** @template $Origin, $Item, $State
 * @param {Opt<Span<$Origin, $Item>>} span
 * @param{{ up: void } | { down: void }} direction
 * @param{$State} state
 * @param{Fn<{ slot: Slot<$Origin, $Item>, state: $State, }, $State>} step
 * @returns {$State} */
export function opt_span_step(span, direction, state, step) {
  return "no" in span ? state : span_step(span.yes, direction, state, step);
}
/** @template $Done, $Going, $Origin, $Item
 * @param {{
 *     span: Span<$Origin, $Item>,
 *     direction: { up: void } | { down: void },
 *     state: $Going,
 *     step: Fn<{ slot: Slot<$Origin, $Item>, state: $Going, }, { going: $Going } | { done: $Done }>,
 * }} step
 * @returns {{ going: $Going } | { done: { done: $Done, rest: Opt<Span<$Origin, $Item>>} }} */
export function span_step_while(step) {
  let state = step.state;
  if ("up" in step.direction) {
    for (let i = step.span.start; i < step.span.start + step.span.length; i++) {
      const step_result = step.step({ state: state, slot: i });
      if ("done" in step_result) {
        const rest_length = step.span.start + step.span.length - 1 - i;
        return {
          done: {
            done: step_result.done,
            rest:
              rest_length >= 1
                ? { yes: { start: i + 1, length: rest_length } }
                : { no: undefined },
          },
        };
      }
      state = step_result.going;
    }
  } else {
    for (let i = step.span.start + step.span.length - 1; i >= step.span.start; i--) {
      const step_result = step.step({ state: state, slot: i });
      if ("done" in step_result) {
        const rest_length = i - step.span.start;
        return {
          done: {
            done: step_result.done,
            rest:
              rest_length >= 1
                ? { yes: { start: 0, length: rest_length } }
                : { no: undefined },
          },
        };
      }
      state = step_result.going;
    }
  }
  return { going: state };
}
/** @template $Done, $Going, $Origin, $Item
 * @param {{
 *     span: Opt<Span<$Origin, $Item>>,
 *     direction: { up: void } | { down: void },
 *     state: $Going,
 *     step: Fn<{ slot: Slot<$Origin, $Item>, state: $Going, }, { going: $Going } | { done: $Done }>,
 * }} step
 * @returns {{ going: $Going } | { done: { done: $Done, rest: Opt<Span<$Origin, $Item>>} }} */
export function opt_span_step_while(step) {
  if ("no" in step.span) return { going: step.state };
  return span_step_while({
    span: step.span.yes,
    direction: step.direction,
    state: step.state,
    step: step.step,
  });
}

/** @template $Item, $Origin @returns {Buf<$Origin, $Item>} */
export function buf_empty() {
  return [];
}
/** @template $Item, $Origin @param {Buf<$Origin, $Item>} buf @returns {Buf<$Origin, $Item>} */
export function buf_pre_allocation_rid(buf) {
  return buf;
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{Span<$Origin, $Item>} span
 * @param {Fn<$Item, void>} item_rid
 * @returns {void} */
export function buf_span_rid(buf, span, item_rid) {
  if (span.start + span.length === buf.length) {
    for (let i = span.start; i < span.start + span.length; i++) {
      item_rid(/** @type $Item */ (buf[i]));
    }
    buf.length -= span.length;
    while (buf[buf.length - 1] === SYMBOL$UNSET) {
      buf.length -= 1;
    }
  } else {
    for (let i = span.start; i < span.start + span.length; i++) {
      item_rid(/** @type $Item */ (buf[i]));
      buf[i] = SYMBOL$UNSET;
    }
  }
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Span<$Origin, $Item>} span
 * @param {Fn<$Item, $Item>} item_alter
 * @returns {void} */
export function buf_span_alter(buf, span, item_alter) {
  for (let i = span.start; i < span.start + span.length; i++) {
    buf[i] = item_alter(/** @type $Item */ (buf[i]));
  }
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Opt<Span<$Origin, $Item>>} span
 * @param {Fn<$Item, $Item>} item_alter
 * @returns {void} */
export function buf_opt_span_alter(buf, span, item_alter) {
  if ("yes" in span) {
    buf_span_alter(buf, span.yes, item_alter);
  }
}
/** @template $Item, $Origin, $State
 * @param {Buf<$Origin, $Item>} buf
 * @param{Span<$Origin, $Item>} span
 * @param{{ up: void } | { down: void }} direction
 * @param{$State} state
 * @param{Fn<{ item: $Item, state: $State, }, [$State, $Item]>} step
 * @returns {$State} */
export function buf_span_step(buf, span, direction, state, step) {
  if ("up" in direction) {
    for (let i = span.start; i < span.start + span.length; i++) {
      const [stepped_state, stepped_item] = step({
        state: state,
        item: /** @type $Item */ (buf[i]),
      });
      buf[i] = stepped_item;
      state = stepped_state;
    }
  } else {
    for (let i = span.start + span.length - 1; i >= span.start; i--) {
      const [stepped_state, stepped_item] = step({
        state: state,
        item: /** @type $Item */ (buf[i]),
      });
      buf[i] = stepped_item;
      state = stepped_state;
    }
  }
  return state;
}
/** @template $Item, $Origin, $State
 * @param {Buf<$Origin, $Item>} buf
 * @param{Opt<Span<$Origin, $Item>>} span
 * @param{{ up: void } | { down: void }} direction
 * @param{$State} state
 * @param{Fn<{ item: $Item, state: $State, }, [$State, $Item]>} step
 * @returns {$State} */
export function buf_opt_span_step(buf, span, direction, state, step) {
  return "yes" in span ? buf_span_step(buf, span.yes, direction, state, step) : state;
}
/** @template $Item, $Origin, $State, $Done
 * @param {Buf<$Origin, $Item>} buf
 * @param{Span<$Origin, $Item>} span
 * @param{{ up: void } | { down: void }} direction
 * @param{$State} state
 * @param{Fn<{ item: $Item, state: $State, }, { item: $Item, state: { going: $State} | { done: $Done }, }>} step
 * @returns {{ going: $State} | { done: $Done }} */
export function buf_span_step_while(buf, span, direction, state, step) {
  if ("up" in direction) {
    for (let i = span.start; i < span.start + span.length; i++) {
      const stepped = step({ state: state, item: /** @type $Item */ (buf[i]) });
      buf[i] = stepped.item;
      if ("done" in stepped.state) {
        return stepped.state;
      }
      state = stepped.state.going;
    }
  } else {
    for (let i = span.start + span.length - 1; i >= span.start; i--) {
      const stepped = step({ state: state, item: /** @type $Item */ (buf[i]) });
      buf[i] = stepped.item;
      if ("done" in stepped.state) {
        return stepped.state;
      }
      state = stepped.state.going;
    }
  }
  return { going: state };
}
/** @template $Item, $Origin, $State, $Done
 * @param {Buf<$Origin, $Item>} buf
 * @param{Opt<Span<$Origin, $Item>>} span
 * @param{{ up: void } | { down: void }} direction
 * @param{$State} state
 * @param{Fn<{ item: $Item, state: $State, }, { item: $Item, state: { going: $State} | { done: $Done }, }>} step
 * @returns {{ going: $State} | { done: $Done }} */
export function buf_opt_span_step_while(buf, span, direction, state, step) {
  if ("yes" in span) {
    return buf_span_step_while(buf, span.yes, direction, state, step);
  }
  return { going: state };
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{U32} length
 * @returns {void} */
export function buf_pre_allocate_at_least(buf, length) {
  buf.length += length;
  buf.length -= length;
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{U32} length
 * @returns {{ success: void } | { out_of_memory: void }} */
export function buf_try_pre_allocate_at_least(buf, length) {
  buf_pre_allocate_at_least(buf, length);
  return { success: undefined };
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {$Item} new_item
 * @returns {Slot<$Origin, $Item>} */
export function buf_add(buf, new_item) {
  const new_index = buf.length;
  buf.push(new_item);
  if (buf.length > U32$MAX)
    throw Error("Array length " + buf.length + " not representable as a u32");
  return new_index;
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {$Item} new_item
 * @returns {Slot<$Origin, $Item>} */
export function buf_insert(buf, new_item) {
  const existing_unset_index = buf.findIndex((item) => item === SYMBOL$UNSET);
  if (existing_unset_index >= 0) {
    buf[existing_unset_index] = new_item;
    return existing_unset_index;
  } else {
    return buf_add(buf, new_item);
  }
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {U32} index
 * @param {$Item} new_item
 * @returns {Slot<$Origin, $Item>} */
export function buf_set_insert(buf, index, new_item) {
  if (buf[index] === SYMBOL$UNSET) {
    buf[index] = new_item;
    return index;
  }
  return buf_insert(buf, new_item);
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Span<$Origin, $Item>} span
 * @param {$Item} new_item
 * @returns {Span<$Origin, $Item>} */
export function buf_span_add(buf, span, new_item) {
  if (span.start + span.length < buf.length) {
    // move span to end
    for (let i = span.start; i < span.start + span.length; i++) {
      buf.push(buf[i]);
      buf[i] = SYMBOL$UNSET;
    }
  }
  buf.push(new_item);
  if (buf.length > U32$MAX)
    throw Error("Array length " + buf.length + " not representable as a u32");
  return { start: buf.length - 1 - span.length, length: span.length + 1 };
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Opt<Span<$Origin, $Item>>} span
 * @param {$Item} new_item
 * @returns {Span<$Origin, $Item>} */
export function buf_opt_span_add(buf, span, new_item) {
  if ("no" in span) {
    const new_index = buf.length;
    buf.push(new_item);
    if (buf.length > U32$MAX)
      throw Error("Array length " + buf.length + " not representable as a u32");
    return slot_to_span(new_index);
  }
  return buf_span_add(buf, span.yes, new_item);
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Slot<$Origin, $Item>} slot
 * @returns {$Item} */
export function buf_remove(buf, slot) {
  if (slot + 1 < buf.length) {
    const item = /** @type {$Item} */ (buf[slot]);
    buf[slot] = SYMBOL$UNSET;
    return item;
  } else {
    const item = /** @type {$Item} */ (buf.pop());
    while (buf[buf.length - 1] === SYMBOL$UNSET) {
      buf.length -= 1;
    }
    return item;
  }
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Slot<$Origin, $Item>} slot
 * @param {$Item} new_item
 * @returns {$Item} */
export function buf_replace(buf, slot, new_item) {
  const old_item = /** @type {$Item} */ (buf[slot]);
  buf[slot] = new_item;
  return old_item;
}
/** @template $In, $Item, $Origin, $Out
 * @param {Buf<$Origin, $Item>} buf
 * @param {Slot<$Origin, $Item>} slot
 * @param {$In} in_
 * @param {Fn<{inø: $In, item:$Item}, {item: $Item, out:$Out}>} step
 * @returns {$Out} */
export function buf_item_step(buf, slot, in_, step) {
  const item = /** @type {$Item} */ (buf[slot]);
  const stepped = step({ inø: in_, item: item });
  buf[slot] = stepped.item;
  return stepped.out;
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Slot<$Origin, $Item>} slot_a
 * @param {Slot<$Origin, $Item>} slot_b
 * @returns {void} */
export function buf_swap(buf, slot_a, slot_b) {
  const a_item = buf[slot_a];
  buf[slot_a] = buf[slot_b];
  buf[slot_b] = a_item;
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Span<$Origin, $Item>} span
 * @returns {Span<$Origin, $Item>} */
export function buf_span_move_to_end(buf, span) {
  if (span.start + span.length < buf.length) {
    // move span to end
    for (let i = span.start; i < span.start + span.length; i++) {
      buf.push(buf[i]);
      buf[i] = SYMBOL$UNSET;
    }
    if (buf.length > U32$MAX)
      throw Error("Array length " + buf.length + " not representable as a u32");
  }
  return { start: buf.length - span.length, length: span.length };
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Opt<Span<$Origin, $Item>>} span
 * @returns {Opt<Span<$Origin, $Item>>} */
export function buf_opt_span_move_to_end(buf, span) {
  if ("no" in span) return { no: undefined };
  return { yes: buf_span_move_to_end(buf, span.yes) };
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Span<$Origin, $Item>} span
 * @returns {Span<$Origin, $Item>} */
export function buf_span_move_to_unset(buf, span) {
  if (span.start + span.length < buf.length) return span;
  let unset_length = 0;
  for (let i = 0; i < buf.length; i++) {
    if (buf[i] === SYMBOL$UNSET) {
      unset_length++;
      if (unset_length === span.length) {
        const unset_start = i - span.length + 1;
        for (let unset_i = 0; unset_i < span.length; unset_i++) {
          buf[unset_start + unset_i] = buf[span.start + unset_i];
        }
        buf.length -= span.length;
        while (buf[buf.length - 1] === SYMBOL$UNSET) {
          buf.length -= 1;
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
 * @param {Buf<$Origin, $Item>} buf
 * @param {Opt<Span<$Origin, $Item>>} span
 * @returns {Opt<Span<$Origin, $Item>>} */
export function buf_opt_span_move_to_unset(buf, span) {
  if ("no" in span) return span;
  return { yes: buf_span_move_to_unset(buf, span.yes) };
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Span<$Origin, $Item>} span
 * @returns {void} */
export function buf_span_reverse(buf, span) {
  const slice = buf.slice(span.start, span.start + span.length);
  slice.reverse();
  buf.splice(span.start, span.length, ...slice);
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param {Opt<Span<$Origin, $Item>>} span
 * @returns {void} */
export function buf_opt_span_reverse(buf, span) {
  if ("no" in span) return;
  buf_span_reverse(buf, span.yes);
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{Span<$Origin, $Item>} span
 * @param{Fn<{ left: $Item, right: $Item }, Order>} item_order
 * @returns {Span<$Origin, $Item>} */
export function buf_span_sort(buf, span, item_order) {
  const span_indexes = Array.from(span, (_, i) => span.start + i);
  span_indexes.sort((left, right) => {
    const order = item_order({
      left: /** @type $Item */ (buf[left]),
      right: /** @type $Item */ (buf[right]),
    });
    return "less" in order ? -1 : "greater" in order ? 1 : 0;
  });
  span_indexes.forEach((buf_index_sorted, old_offset_within_span) => {
    if (span.start + old_offset_within_span < buf_index_sorted) {
      // swap
      const old = buf[span.start + old_offset_within_span];
      buf[span.start + old_offset_within_span] = buf[buf_index_sorted];
      buf[buf_index_sorted] = old;
    }
  });
  return span;
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{Opt<Span<$Origin, $Item>>} span
 * @param{Fn<{ left: $Item, right: $Item }, Order>} item_order
 * @returns {Opt<Span<$Origin, $Item>>} */
export function buf_opt_span_sort(buf, span, item_order) {
  if ("yes" in span) {
    buf_span_sort(buf, span.yes, item_order);
  }
  return span;
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{Span<$Origin, $Item>} start
 * @param{Span<$Origin, $Item>} end
 * @returns {Span<$Origin, $Item>} */
export function buf_span_add_own_span(buf, start, end) {
  if (start.start + start.length === end.start) {
    return { start: start.start, length: start.length + end.length };
  }
  if (start.start + start.length < buf.length) {
    // move start span to end
    for (let i = start.start; i < start.start + start.length; i++) {
      buf.push(buf[i]);
      buf[i] = SYMBOL$UNSET;
    }
  }
  // move end span to end after the start items
  for (let i = end.start; i < end.start + end.length; i++) {
    buf.push(buf[i]);
    buf[i] = SYMBOL$UNSET;
  }
  return {
    start: buf.length - start.length - end.length,
    length: start.length + end.length,
  };
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{Span<$Origin, $Item>} start
 * @param{Opt<Span<$Origin, $Item>>} end
 * @returns {Span<$Origin, $Item>} */
export function buf_span_add_own_opt_span(buf, start, end) {
  if ("no" in end) {
    return start;
  }
  return buf_span_add_own_span(buf, start, end.yes);
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{Opt<Span<$Origin, $Item>>} start
 * @param{Span<$Origin, $Item>} end
 * @returns {Span<$Origin, $Item>} */
export function buf_opt_span_add_own_span(buf, start, end) {
  if ("no" in start) {
    return end;
  }
  return buf_span_add_own_span(buf, start.yes, end);
}
/** @template $Item, $Origin
 * @param {Buf<$Origin, $Item>} buf
 * @param{Opt<Span<$Origin, $Item>>} start
 * @param{Opt<Span<$Origin, $Item>>} end
 * @returns {Opt<Span<$Origin, $Item>>} */
export function buf_opt_span_add_own_opt_span(buf, start, end) {
  if ("no" in start) {
    return end;
  }
  if ("no" in end) {
    return start;
  }
  return { yes: buf_span_add_own_span(buf, start.yes, end.yes) };
}
/** @template $Origin
 * @param {Buf<$Origin, Char>} buf
 * @param {Str} new_str
 * @returns {Span<$Origin, Char>} */
export function buf_add_str_chars(buf, new_str) {
  const new_start = buf.length;
  for (const new_char of new_str) {
    buf.push(new_char);
  }
  if (buf.length > U32$MAX)
    throw Error("Array length " + buf.length + " not representable as a u32");
  return {
    start: new_start,
    length: buf.length - new_start,
  };
}
/** @template $Origin
 * @param {Buf<$Origin, Char>} buf
 * @param {Span<$Origin, Char>} span
 * @param {Str} new_str
 * @returns {Span<$Origin, Char>} */
export function buf_span_add_str_chars(buf, span, new_str) {
  if (span.start + span.length < buf.length) {
    // move span to end
    for (let i = span.start; i < span.start + span.length; i++) {
      buf.push(buf[i]);
      buf[i] = SYMBOL$UNSET;
    }
  }
  const new_start = buf.length;
  for (const new_char of new_str) {
    buf.push(new_char);
  }
  if (buf.length > U32$MAX)
    throw Error("Array length " + buf.length + " not representable as a u32");
  return {
    start: new_start - span.length,
    length: span.length + (buf.length - new_start),
  };
}
/** @template $Origin
 * @param {Buf<$Origin, Char>} buf
 * @param {Opt<Span<$Origin, Char>>} span
 * @param {Str} new_str
 * @returns {Span<$Origin, Char>} */
export function buf_opt_span_add_str_chars(buf, span, new_str) {
  if ("no" in span) {
    return buf_add_str_chars(buf, new_str);
  }
  return buf_span_add_str_chars(buf, span.yes, new_str);
}
/** @template $Origin
 * @param {Buf<$Origin, Char>} buf
 * @param {Span<$Origin, Char>} span
 * @param {U32} n
 * @returns {Span<$Origin, Char>} */
export function buf_span_add_u32_chars(buf, span, n) {
  return buf_span_add_str_chars(buf, span, n.toString());
}
/** @template $Origin
 * @param {Buf<$Origin, Char>} buf
 * @param {Opt<Span<$Origin, Char>>} span
 * @param {U32} n
 * @returns {Span<$Origin, Char>} */
export function buf_opt_span_add_u32_chars(buf, span, n) {
  return buf_opt_span_add_str_chars(buf, span, n.toString());
}
/** @template $Origin
 * @param {Buf<$Origin, Char>} buf
 * @param {Span<$Origin, Char>} span
 * @param {I32} n
 * @returns {Span<$Origin, Char>} */
export function buf_span_add_i32_chars(buf, span, n) {
  return buf_span_add_str_chars(buf, span, n.toString());
}
/** @template $Origin
 * @param {Buf<$Origin, Char>} buf
 * @param {Opt<Span<$Origin, Char>>} span
 * @param {I32} n
 * @returns {Span<$Origin, Char>} */
export function buf_opt_span_add_i32_chars(buf, span, n) {
  return buf_opt_span_add_str_chars(buf, span, n.toString());
}
/** @template $Origin
 * @param {Buf<$Origin, Char>} buf
 * @param {Span<$Origin, Char>} span
 * @param {F32} n
 * @returns {Span<$Origin, Char>} */
export function buf_span_add_f32_chars(buf, span, n) {
  return buf_span_add_str_chars(buf, span, f32$to_string(n));
}
/** @template $Origin
 * @param {Buf<$Origin, Char>} buf
 * @param {Opt<Span<$Origin, Char>>} span
 * @param {F32} n
 * @returns {Span<$Origin, Char>} */
export function buf_opt_span_add_f32_chars(buf, span, n) {
  return buf_opt_span_add_str_chars(buf, span, f32$to_string(n));
}
/** @template $Item, $Origin @param {Buf<$Origin, $Item>} buf @returns {Unset_slice<$Item>} */
export function buf_to_unset(buf) {
  buf.length = 0;
  return buf;
}
/** @template $Item, $Origin @param {Unset_slice<$Item>} slice @returns {Buf<$Origin, $Item>} */
export function buf_reuse(slice) {
  return slice;
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
  return /** @type ($New_item | typeof SYMBOL$UNSET)[] */ (unset_slice);
}
