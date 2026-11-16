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
  switch (html.tag) {
    case "text":
      return document.createTextNode(slugTextToString(html.v, text));
    case "element":
      const element = document.createElement(html.v0);
      for (const modifier of slug.buf$optSpanSlice(htmlModifiers, html.v1)) {
        switch (modifier.tag) {
          case "attribute":
            element.setAttribute(modifier.v0, slugTextToString(modifier.v1, text));
            break;
          case "event_listener":
            element.addEventListener(modifier.v0, (event) => {
              if (!event) return;
              const jsonFields = slug.buf_empty();
              const chars = slug.buf_empty();
              const eventJson = jsValueToSlugJson(event, jsonFields, chars);
              const slugEvent = modifier.v1(eventJson, {
                "buf(View {name [No Yes(span(View char))] value [Null True False Number(f32) String([No Yes(span(View char))]) Object([No Yes(span(View json-object-field(View)))])]})":
                  jsonFields,
                "buf(View char)": chars,
              });
              onEvent(slugEvent, {});
            });
            break;
        }
      }
      for (const sub of slug.buf$optSpanSlice(htmls, html.v2)) {
        element.appendChild(createHtml(sub, htmls, htmlModifiers, text, onEvent));
      }
      return element;
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
    return { tag: "nullø" };
  } else if (typeof jsEvent === "number") {
    return { tag: "number", v: jsEvent };
  } else if (typeof jsEvent === "object") {
    /** @type slug.Opt<slug.Span<View, slug.Json_object_field<View>>> */
    let fields = { tag: "no" };
    for (const fieldName of Object.getOwnPropertyNames(Object.getPrototypeOf(jsEvent))) {
      fields = {
        tag: "yes",
        v: slug.opt_span_add(
          fields,
          {
            name: {
              tag: "yes",
              v: slug.add_str_chars(fieldName, { "buf(Origin char)": chars }),
            },
            value: jsValueToSlugJson(jsEvent[fieldName], jsonFields, chars),
          },
          { "buf(Origin Item)": jsonFields },
        ),
      };
    }
    return { tag: "object", v: fields };
  } else if (typeof jsEvent == "string") {
    return {
      tag: "string",
      v:
        jsEvent.length === 0
          ? { tag: "no" }
          : { tag: "yes", v: slug.add_str_chars(jsEvent, { "buf(Origin char)": chars }) },
    };
  } else if (typeof jsEvent == "boolean") {
    return jsEvent ? { tag: "trueø" } : { tag: "falseø" };
  } else {
    return { tag: "nullø" };
  }
}
/** @template Chars
 * @param {slug.Text<Chars>} text
 * @param {slug.Buf<Chars, slug.Char>} chars
 * @returns string
 */
function slugTextToString(text, chars) {
  switch (text.tag) {
    case "staticø":
      return text.v;
    case "dynamic":
      return slug.buf$optSpanSlice(chars, text.v).join("");
  }
}
