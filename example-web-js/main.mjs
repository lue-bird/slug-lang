// @ts-check
import * as slug from "./slug.mjs";

const rootNode = document.getElementById("root");

let state = slug.initial_state();
let htmls = slug.buf_empty();
let htmlModifiers = slug.buf_empty();
let text = slug.buf_empty();
refreshHtml();

function refreshHtml() {
  const { html } = slug.view_with_explicit_bufs({
    state,
    htmls,
    html_modifiers: htmlModifiers,
    text,
  });
  let slugControlledNode = createHtml(html, htmls, htmlModifiers, text, (event) => {
    state = slug.on_event(event, state);
    // in practice you should do some diff here to preserve dom state
    rootNode?.removeChild(slugControlledNode);
    refreshHtml();
  });
  // alternatively we could call slug.drop$Html
  slug.buf$clear(htmls);
  slug.buf$clear(htmlModifiers);
  slug.buf$clear(text);
  rootNode?.appendChild(slugControlledNode);
}

/** @template View, Event
 * @param {slug.Html<View, Event>} html
 * @param {slug.Buf<View, slug.Html<View, Event>>} htmls
 * @param {slug.Buf<View, slug.Html_modifier<View, Event>>} htmlModifiers
 * @param {slug.Buf<View, slug.Char>} text
 * @param {(_: Event, $: any) => void} onEvent
 * @returns {Node}
 */
function createHtml(html, htmls, htmlModifiers, text, onEvent) {
  if ("text" in html) {
    return document.createTextNode(slugTextToString(html.text, text));
  } else if ("element" in html) {
    const element = document.createElement(html.element[0]);
    for (const modifier of slug.buf$optSpanSlice(htmlModifiers, html.element[1])) {
      if ("attribute" in modifier) {
        element.setAttribute(
          modifier.attribute[0],
          slugTextToString(modifier.attribute[1], text),
        );
      } else if ("event_listener" in modifier) {
        element.addEventListener(modifier.event_listener[0], (event) => {
          if (!event) return;
          const jsonFields = slug.buf_empty();
          const chars = slug.buf_empty();
          const eventJson = jsValueToSlugJson(event, jsonFields, chars);
          const slugEvent = modifier.event_listener[1](eventJson, {
            "buf(View {name [No Yes(span(View char))] value [Null True False Number(f32) String([No Yes(span(View char))]) Object([No Yes(span(View json-object-field(View)))])]})":
              jsonFields,
            "buf(View char)": chars,
          });
          onEvent(slugEvent, {});
        });
      }
    }
    for (const sub of slug.buf$optSpanSlice(htmls, html.element[2])) {
      element.appendChild(createHtml(sub, htmls, htmlModifiers, text, onEvent));
    }
    return element;
  } else {
    throw Error("missing case on html");
  }
}
/** @template View
 * @param {any} jsEvent
 * @param {slug.Buf<View, slug.Json_object_field<View>>} jsonFields
 * @param {slug.Buf<View, slug.Char>} chars
 * @returns {slug.Json<View>}
 */
function jsValueToSlugJson(jsEvent, jsonFields, chars) {
  // must check for null first because for some reason typeof null === "object"
  if (jsEvent === null) {
    return { nullø: undefined };
  } else if (typeof jsEvent === "number") {
    return { number: jsEvent };
  } else if (typeof jsEvent === "object") {
    /** @type slug.Opt<slug.Span<View, slug.Json_object_field<View>>> */
    let fields = { no: undefined };
    for (const fieldName of Object.getOwnPropertyNames(Object.getPrototypeOf(jsEvent))) {
      fields = {
        yes: slug.opt_span_add(
          fields,
          {
            name: { yes: slug.add_str_chars(fieldName, { "buf(Origin char)": chars }) },
            value: jsValueToSlugJson(jsEvent[fieldName], jsonFields, chars),
          },
          { "buf(Origin Item)": jsonFields },
        ),
      };
    }
    return { object: fields };
  } else if (typeof jsEvent == "string") {
    return {
      string:
        jsEvent.length === 0
          ? { no: undefined }
          : { yes: slug.add_str_chars(jsEvent, { "buf(Origin char)": chars }) },
    };
  } else if (typeof jsEvent == "boolean") {
    return jsEvent ? { trueø: undefined } : { falseø: undefined };
  } else {
    return { nullø: undefined };
  }
}
/** @template Chars
 * @param {slug.Text<Chars>} text
 * @param {slug.Buf<Chars, slug.Char>} chars
 * @returns string
 */
function slugTextToString(text, chars) {
  if ("staticø" in text) {
    return text.staticø;
  } else {
    return slug.buf$optSpanSlice(chars, text.dynamic).join("");
  }
}
