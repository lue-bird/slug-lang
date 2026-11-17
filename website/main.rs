#![allow(non_upper_case_globals)]

use gen_lsp_types as lsp_types;
use sauron::web_sys;
use slug_compile as slug;

fn main() {
    let selected_example: Example = web_sys::window()
        .and_then(|window| window.location().search().ok())
        .and_then(|search| {
            let example_name = search.trim_start_matches("?example=");
            example_infos
                .into_iter()
                .find(|(_, example_info)| example_info.name.replace(' ', "-") == example_name)
                .map(|(example, _)| example)
        })
        .unwrap_or(Example::HelloWorld);
    sauron::Program::mount_to_body(State {
        text_area_content: example_source(selected_example).to_string(),
        selected_example,
        slug_core_declarations_html_static: slug_core_declarations_html(),
    });
}

struct State {
    text_area_content: String,
    selected_example: Example,
    // there must be a better way to cache this...
    slug_core_declarations_html_static: sauron::Node<Event>,
}
enum Event {
    None,
    TextAreaContentChanged(String),
    ExampleSelected(Example),
}
impl sauron::Application for State {
    type MSG = Event;
    fn init(&mut self) -> sauron::Cmd<Self::MSG> {
        sauron::Cmd::none()
    }
    fn update(&mut self, event: Event) -> sauron::Cmd<Self::MSG> {
        match event {
            Event::None => sauron::Cmd::none(),
            Event::TextAreaContentChanged(new_text_area_content) => {
                self.text_area_content = new_text_area_content;
                sauron::Cmd::none()
            }
            Event::ExampleSelected(selected_example) => {
                self.selected_example = selected_example;
                self.text_area_content = example_source(selected_example).to_string();
                if let Some(window) = web_sys::window()
                    && let Ok(history) = window.history()
                {
                    let _ = history.push_state_with_url(
                        &web_sys::wasm_bindgen::JsValue::NULL,
                        "",
                        Some(&format!(
                            "{}?example={}",
                            window
                                .location()
                                .pathname()
                                .unwrap_or_else(|_| String::new()),
                            example_name(selected_example).replace(' ', "-")
                        )),
                    );
                }
                sauron::Cmd::none()
            }
        }
    }

    fn view(&self) -> sauron::Node<Self::MSG> {
        sauron::main(
            [],
            [
                sauron::h2(
                    [html_style("white-space: pre-line;")],
                    [html_text(
                        "small programming language which uses
indexes bound to specific array buffers
instead of references: slug",
                    )],
                ),
                html_link_to("https://codeberg.org/lue-bird/slug-lang", "source code"),
                html_text(". Try an example: "),
                sauron::p(
                    [sauron::id("example-select"), html_style("display: inline")],
                    example_infos
                        .into_iter()
                        .map(|(example_kind, example_info)| {
                            sauron::button(
                                [sauron::on_click(move |_| {
                                    Event::ExampleSelected(example_kind)
                                })],
                                [html_text(example_info.name)],
                            )
                        }),
                ),
                linebreak_html(),
                playground_html(self.selected_example, &self.text_area_content),
                installation_html(),
                usage_html(),
                self.slug_core_declarations_html_static.clone(),
            ],
        )
    }
}
fn installation_html<Event>() -> sauron::Node<Event> {
    sauron::section(
        [sauron::html::attributes::skip(true)],
        [
            sub_heading_html("install"),
            sauron::ol(
                [],
                [
                    html_link_to("https://rust-lang.org/tools/install/", "install rust"),
                    sauron::code(
                        [],
                        [html_text(
                            "cargo install --git https://codeberg.org/lue-bird/slug-lang slug",
                        )],
                    ),
                ]
                .map(|item| sauron::li([], [item])),
            ),
        ],
    )
}
fn usage_html<Event>() -> sauron::Node<Event> {
    sauron::section(
        [sauron::html::attributes::skip(true)],
        [
            sub_heading_html("use"),
            sauron::ul(
                [],
                [
                    html_link_to(
                        "https://codeberg.org/lue-bird/slug-lang#editor-setups",
                        "lsp setups and extensions",
                    ),
                    html_link_to(
                        "https://codeberg.org/lue-bird/slug-lang",
                        "more examples of e.g. compiling to zig or rust",
                    ),
                ]
                .map(|item| sauron::li([], [item])),
            ),
        ],
    )
}
fn playground_html(selected_example: Example, text_area_content: &str) -> sauron::Node<Event> {
    // stacked on top but still allow filling height:
    // https://stackoverflow.com/a/51949049
    // Originally I was doing position:absolute for the text area
    // and field-sizing: content and as a fallback for firefox height: line count * factor
    // but this ended up an infuriating mess since height:..em was rendering at different lengths in gecko.
    // I also tried display:flex but this didn't consistently ignore the textarea space and also didn't fill its height
    let mut text_area_stack = sauron::div([html_style("display: grid;")], []);
    let cursor_text = "world";
    let cursor_offset = example_source(selected_example)
        .find(cursor_text)
        .unwrap_or(0) as u32;
    let interactive_text_area = sauron::textarea(
        [
            sauron::id("playground_text_area"),
            sauron::attr("autocorrect", "off"),
            sauron::spellcheck("false"),
            sauron::autofocus("true"),
            sauron::name("playground"),
            html_style(
                r#"grid-column: 1;
grid-row: 1;
height: 100%;
width: 100%;
background: none;
color: transparent;
border: none;
line-height: inherit;
resize: none;
overflow: hidden;
font-family: "Liga NovaMono", monospace, sans-serif;
font-size: medium;
resize: none;
caret-color: white;
position: relative;
top: 0.335em;
left: -0.1em"#,
            ),
            sauron::value(text_area_content.to_string()),
            sauron::attr("selectionStart", cursor_offset),
            sauron::attr("selectionEnd", cursor_offset + cursor_text.len() as u32),
            sauron::on_focus({
                |event: sauron::FocusEvent| {
                    web_sys::console::log_1(&"on focus".into());
                    if let Some(event_target) = event.target() {
                        // hacky:
                        // We want to attach an event listener which does not repond back to sauron
                        // because sauron otherwise eats the event returned by on_input sometimes on key press.
                        // Best would dprobably creating a web component or something but I can' be bothered
                        let text_area_object: web_sys::HtmlTextAreaElement =
                            web_sys::HtmlTextAreaElement::from(
                                web_sys::wasm_bindgen::JsValue::from(event_target),
                            );
                        // stop pressing tab from blurring and jumping to the next element on the page
                        // The current implementation uses the obsolete execCommand.
                        // A more future-proof method may be manually editing the text area
                        // and dispatching a new InputEvent from it.
                        // https://kubyshkin.name/posts/insert-text-into-textarea-at-cursor-position/
                        text_area_object.set_onkeydown(Some(
                            &sauron::js_sys::Function::new_with_args(
                                "event",
                                "
if (event.key === 'Tab') {
    event.preventDefault();
    // ↓ is an obsolete API but it works I at least
    window.document.execCommand('insertText', false, '    ')
}
                ",
                            ),
                        ));
                    };
                    Event::None
                }
            }),
            sauron::on_input(move |event: sauron::InputEvent| {
                web_sys::console::log_1(&"on input".into());
                Event::TextAreaContentChanged(event.value())
            }),
        ],
        [],
    );
    _ = text_area_stack.add_children([interactive_text_area]);

    let syntax_project = slug::parse_project(text_area_content);
    let mut errors = Vec::new();
    let checked_project = slug::syntax_project_check(&mut errors, &syntax_project);
    errors.sort_by_key(|error| error.range.start);
    let compiled_project = slug::checked_project_to_js(&checked_project, &syntax_project);
    let mut highlights = slug::HighlightState {
        tokens: Vec::with_capacity(text_area_content.len() / 2),
        previous_token_start: lsp_types::Position {
            line: 0,
            character: 0,
        },
    };
    slug::project_highlight(&mut highlights, &syntax_project);
    _ = text_area_stack.add_children([sauron::div(
        [
            sauron::attr("aria-hidden", "true"),
            html_style(
                "grid-column: 1; grid-row: 1; z-index: 1; pointer-events: none; user-select: none;",
            ),
        ],
        [highlighted_error_ranges_to_html(
            text_area_content,
            errors.iter().map(|error| error.range),
        )],
    )]);
    _ = text_area_stack.add_children([sauron::div(
        [
            sauron::attr("aria-hidden", "true"),
            html_style(
                "grid-column: 1; grid-row: 1; z-index: 1; pointer-events: none; user-select: none;",
            ),
        ],
        [highlighted_slug_source_to_html(
            text_area_content,
            &mut highlights.tokens.into_iter(),
        )],
    )]);
    let mut full = sauron::div([], [text_area_stack]);
    let mut evaluated_variables_html = sauron::ul([html_style(r#"list-style-type: "↪ ""#)], []);
    for (project_fn_name, _project_fn) in
        checked_project
            .checked_project_fns
            .iter()
            .filter(|(fn_name, project_fn)| {
                !slug::core_fns.contains_key(*fn_name) && project_fn.parameter_types.is_empty()
            })
    {
        // would be faster if we only use one return for all results.
        // If performance is bad, do that instead
        let to_evaluate = format!(
            "{}\n\nreturn {}({{}});",
            compiled_project.replace("export ", ""),
            slug::name_to_lowercase_js(project_fn_name),
        );
        let mut evaluated_variable_html = sauron::li(
            [],
            [
                sauron::code([], [sauron::text(project_fn_name)]),
                html_text(" is "),
            ],
        );
        let function_to_evaluate = web_sys::js_sys::Function::new_no_args(&to_evaluate);
        let evaluated = function_to_evaluate.call(&web_sys::wasm_bindgen::JsValue::NULL, ());
        match evaluated {
            Ok(evaluated) => {
                let mut result_as_slug = String::new();
                slug_value_as_js_value_print(&mut result_as_slug, &evaluated);
                _ = evaluated_variable_html
                    .add_children([sauron::code([], [sauron::text(result_as_slug)])]);
            }
            Err(error) => {
                _ = evaluated_variable_html.add_children([sauron::text(
                    match web_sys::wasm_bindgen::JsCast::dyn_ref::<web_sys::js_sys::Error>(&error) {
                        Some(error) => format!("error: {:?}", error.message()),
                        None => format!("error: {:?}", error),
                    },
                )]);
            }
        }
        _ = evaluated_variables_html.add_children([evaluated_variable_html]);
    }
    _ = full.add_children([evaluated_variables_html]);
    let errors_html = sauron::ul(
        [html_style(r#"list-style-type: "⚠︎ ""#)],
        errors.iter().map(|error| {
            sauron::li(
                [],
                [sauron::p(
                    [html_style("white-space: pre-wrap;")],
                    [sauron::text(format!(
                        "line {} char {}: {}",
                        error.range.start.line, error.range.start.character, error.message
                    ))],
                )],
            )
        }),
    );
    _ = full.add_children([
        errors_html,
        sauron::p(
            [html_style("font: inherit; white-space: pre-line;")],
            [
                html_text("💡 "),
                html_text(example_explainer(selected_example)),
            ],
        ),
    ]);
    full
}
fn slug_value_as_js_value_print(formatted: &mut String, js_value: &web_sys::wasm_bindgen::JsValue) {
    use std::fmt::Write as _;
    if let Some(number) = js_value.as_f64() {
        let _ = write!(formatted, "{}", number);
    } else if let Some(str) = js_value.as_string() {
        let _ = write!(formatted, "{:?}", str);
    } else if js_value.is_function() {
        formatted.push_str("..fn..");
    } else if js_value.is_undefined() {
        formatted.push('*');
    } else if js_value.is_array()
        && let array = web_sys::js_sys::Array::from(js_value)
        && let mut elements = array.iter()
        && let Some(element0) = elements.next()
    {
        formatted.push_str("*(");
        slug_value_as_js_value_print(formatted, &element0);
        for element in elements {
            formatted.push(' ');
            slug_value_as_js_value_print(formatted, &element);
        }
        formatted.push(')');
    } else if let Some(object) = web_sys::wasm_bindgen::JsCast::dyn_ref(js_value)
        && let Ok(entries) =
            web_sys::js_sys::Object::entries_typed::<web_sys::wasm_bindgen::JsValue>(object)
        && let mut entries_iterator = entries.iter()
        && let Some(entry0) = entries_iterator.next()
    {
        let get_field_value = |field_name: &str| -> Option<web_sys::wasm_bindgen::JsValue> {
            if let Some((_, tag)) = entries
                .iter()
                .map(|entry| entry.into_tuple())
                .find(|(key, _)| key == field_name)
            {
                Some(tag)
            } else {
                None
            }
        };
        if let Some(tag) = get_field_value("tag")
            && let Some(mut tag) = tag.as_string()
        {
            tag[0..=0].make_ascii_uppercase();
            formatted.push_str(&tag.replace("_", "-"));
            if let Some(value) = get_field_value("v") {
                formatted.push('(');
                slug_value_as_js_value_print(formatted, &value);
                formatted.push(')');
            } else if let Some(value0) = get_field_value("v0") {
                formatted.push('(');
                slug_value_as_js_value_print(formatted, &value0);
                let mut i = 1;
                while let Some(value_i) = get_field_value(&format!("v{i}")) {
                    formatted.push(' ');
                    slug_value_as_js_value_print(formatted, &value_i);
                    i += 1;
                }
                formatted.push(')');
            }
        } else if let Some(items) = get_field_value("items")
            && let Some(_) = get_field_value("counts")
            && items.is_array()
        {
            let array = web_sys::js_sys::Array::from(&items);
            let mut elements = array.iter();
            if let Some(element0) = elements.next() {
                if array.iter().all(|element| element.is_string()) {
                    formatted.push_str("[...");
                    let _ = write!(formatted, "{:?}", array.join(""));
                    formatted.push(']');
                } else {
                    formatted.push('[');
                    slug_value_as_js_value_print(formatted, &element0);
                    for element in elements {
                        formatted.push(' ');
                        slug_value_as_js_value_print(formatted, &element);
                    }
                    formatted.push(']');
                }
            } else {
                formatted.push_str("buf-empty");
            }
        } else {
            formatted.push('{');
            formatted.push_str(&ToString::to_string(&entry0.get0()).replace("_", "-"));
            formatted.push(' ');
            slug_value_as_js_value_print(formatted, &entry0.get1());
            for entry in entries_iterator {
                formatted.push(' ');
                formatted.push_str(&ToString::to_string(&entry.get0()).replace("_", "-"));
                formatted.push(' ');
                slug_value_as_js_value_print(formatted, &entry.get1());
            }
            formatted.push('}');
        }
    } else {
        let _ = write!(formatted, "{:?}", js_value);
    }
}
fn slug_core_declarations_html<Event>() -> sauron::Node<Event> {
    let mut section = sauron::section(
        [
            // is there some "from_arc/memoize/..." primitive in sauron to prevent deep clones on every render?
            // Not even .render_to_string().leak() + sauron::raw_html works because it would parse the html into a tree
            sauron::html::attributes::skip(true),
            sauron::key("slug_core_declarations_html"),
        ],
        [sub_heading_html("core declarations")],
    );
    let mut type_aliases_sorted = slug::core_type_aliases.iter().collect::<Vec<_>>();
    type_aliases_sorted.sort_unstable_by_key(|(name, _)| *name);
    _ = section.add_children(type_aliases_sorted.into_iter().map(
        |(core_choice_type_name, core_choice_type_info)| {
            slug_type_alias_to_html(core_choice_type_name, core_choice_type_info)
        },
    ));
    let mut project_fns_sorted = slug::core_fns.iter().collect::<Vec<(&slug::Name, _)>>();
    project_fns_sorted.sort_unstable_by_key(|(name, _)| *name);
    _ = section.add_children(project_fns_sorted.into_iter().map(
        |(core_variable_name, core_variable_info)| {
            slug_project_fn_to_html(core_variable_name, core_variable_info)
        },
    ));
    section
}
fn slug_project_source_to_html<Event>(project_source: &str) -> sauron::Node<Event> {
    let syntax_project = slug::parse_project(project_source);
    let mut highlights = slug::HighlightState {
        tokens: Vec::with_capacity(project_source.len() / 2),
        previous_token_start: lsp_types::Position {
            line: 0,
            character: 0,
        },
    };
    slug::project_highlight(&mut highlights, &syntax_project);
    highlighted_slug_source_to_html(project_source, &mut highlights.tokens.into_iter())
}
fn slug_type_alias_to_html<Event>(
    name: &slug::Name,
    type_alias: &slug::CheckedTypeAlias,
) -> sauron::Node<Event> {
    let mut section_html = sauron::section([], [documentation_heading_html(name)]);
    let mut formatted = String::new();
    slug::checked_type_alias_format(&mut formatted, name, type_alias);
    _ = section_html.add_children([slug_project_source_to_html(&formatted)]);
    if let Some(documentation) = &type_alias.documentation {
        _ = section_html.add_children([slug_documentation_markdown_to_html(documentation)]);
    }
    section_html
}
fn slug_project_fn_to_html<Event>(
    name: &slug::Name,
    project_fn: &slug::CheckedProjectFn,
) -> sauron::Node<Event> {
    let mut section_html = sauron::section([], [documentation_heading_html(name)]);
    let mut formatted = String::new();
    slug::checked_project_fn_format(&mut formatted, name, project_fn);
    let mut parse_state = slug::ParseState {
        source: &formatted,
        offset_utf8: 0,
        position: lsp_types::Position {
            line: 0,
            character: 0,
        },
    };
    if let Some(project_fn_signature) = slug::parse_project_fn_signature(&mut parse_state) {
        let mut highlights = slug::HighlightState {
            tokens: Vec::with_capacity(formatted.len() / 2),
            previous_token_start: lsp_types::Position {
                line: 0,
                character: 0,
            },
        };
        slug::project_fn_signature_highlight(&mut highlights, &project_fn_signature);
        _ = section_html.add_children([highlighted_slug_source_to_html(
            &formatted,
            highlights.tokens.into_iter(),
        )]);
    }
    if let Some(documentation) = &project_fn.documentation {
        _ = section_html.add_children([slug_documentation_markdown_to_html(documentation)]);
    }
    section_html
}
fn documentation_heading_html<Event>(name: &str) -> sauron::Node<Event> {
    sauron::h4([], [html_link_to_self(name)])
}
fn html_link_to_self<Event>(name: &str) -> sauron::Node<Event> {
    let id = name.replace(" ", "-");
    sauron::a(
        [sauron::href(format!("#{id}")), sauron::id(id)],
        [sauron::text(format!("#{name}"))],
    )
}
fn sub_heading_html<Event>(name: &str) -> sauron::Node<Event> {
    sauron::h3([], [html_link_to_self(name)])
}
/// assumes error_ranges are sorted by start
fn highlighted_error_ranges_to_html<Event>(
    source: &str,
    error_ranges: impl Iterator<Item = lsp_types::Range>,
) -> sauron::Node<Event> {
    let mut html = sauron::pre(
        [html_style(
            r#"line-height: inherit; font-size: medium; font-family: "Liga NovaMono", monospace, sans-serif; margin-top: 0.5em; color: transparent;"#,
        )],
        [],
    );
    // could be optimized by not computing the offsets twice for example
    let mut previous_end: lsp_types::Position = lsp_types::Position::default();
    for error_range in error_ranges {
        if error_range.start.lt(&previous_end) {
            continue;
        }
        _ = html.add_children([
            sauron::code(
                [],
                [sauron::text(
                    source
                        .get(slug::str_lsp_range_to_utf8_range(
                            source,
                            lsp_types::Range {
                                start: previous_end,
                                end: error_range.start,
                            },
                        ))
                        .unwrap_or(""),
                )],
            ),
            sauron::code(
                [html_style("background-color: rgba(255, 0, 0, 0.32)")],
                [sauron::text(
                    source
                        .get(slug::str_lsp_range_to_utf8_range(source, error_range))
                        .unwrap_or(""),
                )],
            ),
        ]);
        previous_end = error_range.end;
    }
    html
}
fn highlighted_slug_source_to_html<Event>(
    source: &str,
    mut highlights: impl Iterator<Item = lsp_types::SemanticToken>,
) -> sauron::Node<Event> {
    let mut html = sauron::pre(
        [html_style(
            r#"line-height: inherit; font-size: medium; font-family: "Liga NovaMono", monospace, sans-serif; margin-top: 0.5em"#,
        )],
        [],
    );
    let mut previous_token_start = lsp_types::Position {
        line: 0,
        character: 0,
    };
    // can be optimized
    let mut maybe_next_highlight = highlights.next();
    for (source_line_index, source_line) in source.lines().enumerate() {
        let mut current_offset_in_line: usize = 0;
        while let Some(highlight) = maybe_next_highlight
            && let highlight_start = (lsp_types::Position {
                line: previous_token_start.line + highlight.delta_line,
                character: if highlight.delta_line == 0 {
                    previous_token_start.character + highlight.delta_start
                } else {
                    highlight.delta_start
                },
            })
            && source_line_index as u32 == highlight_start.line
            && let highlight_range = (lsp_types::Range {
                start: highlight_start,
                end: lsp_types::Position {
                    line: highlight_start.line,
                    character: highlight_start.character + highlight.length,
                },
            })
        {
            let highlight_start_offset_in_line =
                utf16_offset_to_utf8_in(source_line, highlight_range.start.character as usize);
            let highlight_end_offset_in_line =
                utf16_offset_to_utf8_in(source_line, highlight_range.end.character as usize);

            _ = html.add_children([
                sauron::code(
                    [],
                    [sauron::text(
                        &source_line[current_offset_in_line..highlight_start_offset_in_line],
                    )],
                ),
                sauron::code(
                    [html_style(format!(
                        "color: {}",
                        slug_syntax_highlight_kind_to_css_color(
                            &slug::token_types[highlight.token_type as usize]
                        )
                    ))],
                    [sauron::text(
                        &source_line[highlight_start_offset_in_line..highlight_end_offset_in_line],
                    )],
                ),
            ]);

            current_offset_in_line = highlight_end_offset_in_line;
            previous_token_start = highlight_start;
            maybe_next_highlight = highlights.next();
        }
        _ = html.add_children([
            sauron::code([], [sauron::text(&source_line[current_offset_in_line..])]),
            sauron::code([], [html_text("\n")]),
        ]);
    }
    html
}
fn slug_syntax_highlight_kind_to_css_color(kind: &lsp_types::SemanticTokenTypes) -> &'static str {
    match kind {
        lsp_types::SemanticTokenTypes::Type => "rgb(0,255,255)",
        lsp_types::SemanticTokenTypes::TypeParameter
        | lsp_types::SemanticTokenTypes::Variable
        | lsp_types::SemanticTokenTypes::Parameter => "rgb(130,140,255)",
        lsp_types::SemanticTokenTypes::EnumMember => "rgb(120,235,30)",
        lsp_types::SemanticTokenTypes::Property => "rgb(255, 145, 0)",
        lsp_types::SemanticTokenTypes::Function => "rgb(255, 225, 140)",
        lsp_types::SemanticTokenTypes::Comment => "rgb(140,140,140)",
        lsp_types::SemanticTokenTypes::String | lsp_types::SemanticTokenTypes::Number => {
            "rgb(225,105,240)"
        }
        lsp_types::SemanticTokenTypes::Keyword => "rgb(255,60,100)",
        _ => "white",
    }
}
fn slug_documentation_markdown_to_html<Event>(
    slug_documentation_markdown: &str,
) -> sauron::Node<Event> {
    let mut html = sauron::p([], []);
    let mut maybe_current_code_block_start_line_index: Option<usize> = None;
    for (slug_documentation_markdown_line_index, slug_documentation_markdown_line) in
        slug_documentation_markdown.lines().enumerate()
    {
        match slug_documentation_markdown_line {
            "```" | "```slug" => match maybe_current_code_block_start_line_index {
                None => {
                    maybe_current_code_block_start_line_index =
                        Some(slug_documentation_markdown_line_index);
                }
                Some(current_code_block_start_line_index) => {
                    maybe_current_code_block_start_line_index = None;
                    _ = html.add_children([slug_project_source_to_html(
                        &slug_documentation_markdown
                            .lines()
                            .skip(current_code_block_start_line_index + 1)
                            .take(
                                slug_documentation_markdown_line_index
                                    - current_code_block_start_line_index
                                    - 1,
                            )
                            .collect::<Vec<&str>>()
                            .join("\n"),
                    )]);
                }
            },
            "" => {
                if maybe_current_code_block_start_line_index.is_none() {
                    _ = html.add_children([linebreak_html()]);
                }
            }
            _ => {
                if maybe_current_code_block_start_line_index.is_none() {
                    // insert space before because otherwise if the previous line ends in
                    // punctuation like , the text in the next line would be attached directly after it
                    _ = html.add_children([
                        html_text(" "),
                        sauron::text(slug_documentation_markdown_line),
                    ]);
                }
            }
        }
    }
    html
}

#[derive(Copy, Clone)]
struct ExampleInfo {
    source: &'static str,
    explainer: &'static str,
    name: &'static str,
}
fn example_source(example: Example) -> &'static str {
    example_info(example).source
}
fn example_explainer(example: Example) -> &'static str {
    example_info(example).explainer
}
const fn example_name(example: Example) -> &'static str {
    example_info(example).name
}
#[derive(Copy, Clone)]
enum Example {
    HelloWorld,
    Variable,
    Numbers,
    Text,
    FunctionCall,
    Types,
    Function,
    Group,
    Choice,
    Match,
    Vec,
    Comment,
    Extras,
}
static example_infos: [(Example, ExampleInfo); 13] = {
    const fn entry(example: Example) -> (Example, ExampleInfo) {
        (example, example_info(example))
    }
    [
        entry(Example::HelloWorld),
        entry(Example::Variable),
        entry(Example::Numbers),
        entry(Example::Text),
        entry(Example::FunctionCall),
        entry(Example::Types),
        entry(Example::Function),
        entry(Example::Match),
        entry(Example::Group),
        entry(Example::Choice),
        entry(Example::Vec),
        entry(Example::Comment),
        entry(Example::Extras),
    ]
};
const fn example_info(example: Example) -> ExampleInfo {
    match example {
        Example::HelloWorld => ExampleInfo {
            name:"hello world",
            source: r#"
fn greet<Chars>(name str) span(Chars char)
    add-str-chars<Chars>("Hello, "str)
    .span-add-str-chars(name)
    .span-add("!"char)

# using the greet function
fn hi() { chars buf(* char) span span(* char) }
    = chars buf-empty<* char>()
    # try replacing "world" with your name ^~^
    { span greet<*>("world"str) chars chars }
"#,
            explainer: "What a mouthful!
We declare a greet() function which takes a static name string (and an implicit buffer to add the message to).
We then append the name along with other strings to buffer to form a message span in the buffer.
For more details, click through the examples above and try changing things.",
        },
        Example::Variable => ExampleInfo {
            name: "declare a function",
            source: r#"
fn your-project-function-name() str
    "Yahallo there, cute slug"str
"#,
            explainer: "Add a new function to your project by choosing a lowercse name consisting of a-z, A-Z, 0-9 or - after `fn`,
followed by its typed parameter patterns in parens, the result type, followed by the resulting expression (for example \":)\"str).
slug also has \"core\" functions like i32-add-clamp that any project can reference.
To see the full list, scroll down or search the site for #some-name-to-search-for.",
        },
        Example::Numbers => ExampleInfo {
            name: "numbers",
            source: r#"
fn number-with-a-decimal-point() f32 -2.7f32
fn f32-not-ending-in-decimal-point() f32 4f32

fn signed-integer() i32 -2i32
fn signed-integer-zero() i32 0i32

fn unsigned-integer() u32 2u32
fn unsigned-integer-zero() u32 0u32

fn positive-integer() p32 2p32
"#,
            explainer: "some basic types of numbers:
- f32: floating point number; can have a decimal point and can have a sign
- i32: integer; whole number; can have a sign but no decimal point
- u32: unsigned integer: number without a sign or decimal point.
  Used mainly for indexes, counting and similar
- p32: positive integer: non-zero number without a sign, decimal point.
  Used mainly for lengths, counts and similar",
        },
        Example::Text => ExampleInfo {
            name: "text",
            source: r#"
fn single-character() char "a"char
fn escaped-quote() char "\'" char
fn escaped-backslash() char "\\" char
fn escaped-tab() char "\t"char
fn escaped-linebreak() char "\n"char
fn by-code-point-hex() char "\(1F648)"char

fn cat() str "₍^. .^₎⟆"str
fn escaped-double-quote() str "\"hello\""str
fn strings-have-at-least-1-char() *(char opt(str))
    str-start("Hello"str)
"#,
            explainer: "text is wrapped in \"...\" and has a type after it (either char or str).
String buffers, dynamic string slices and static string slices use entirely different types, str refers to static memory.
",
        },
        Example::FunctionCall => ExampleInfo {
            name: "function call",
            source: r#"
fn regular-call() f32
    f32-add-clamp(1.2f32 2.3f32)

fn nested-call() f32
    f32-mul-clamp(7f32 regular-call())

fn chained-call() f32
    regular-call()
    .f32-mul-clamp(7f32)
    .f32-mul-clamp(2f32)
"#,
            explainer: "Function names are followed by (, then space-separated arguments, then ).
You can also \"pipe\" the first argument from before the function with a ., like first-argument.function(arguments-from-second-on)

Here, chained-call() first evaluates regular-call, then multiplies this result by 7, then multiplies that result by 2",
        },
        Example::Types => ExampleInfo {
            name: "declare a type",
            source: r#"
ty string-alias str
ty stringify(It)
    fn(It) str

# * is tuple syntax, explained later in section "group"
ty pair-alias(A B)
    *(A B)

fn using-the-alias(arguments pair-alias(i32 i32)) i32
    # = is assignment syntax, explained later in section "query"
    = *(a b) arguments
    i32-mul-clamp(a b)
"#,
            explainer: "Every parameter variable/ignore and project fn result needs an explicit type.
Variable type names start with an uppercase letter.
They represent a type that is filled in when constructed; for example `stringify(u32)` fills in the `It` variable with `u32`.

You can also create short name aliases for types you often use using type, type name, space-separated variables, result type.
These will become especially useful for record and choice types
",
        },
        Example::Function => ExampleInfo {
            name: "function",
            source: r#"
fn i32-subtract(from i32 minus i32) i32
    i32-add-clamp(from i32-negate-clamp(minus))

fn use-int-subtract() i32
    i32-subtract(3i32 4i32)

fn same-in-same-out(in In) In
    in

fn use-same-in-same-out() str
    same-in-same-out("oo ee oo"str)

fn function-returning-a-function() fn(i32 i32) i32
    fn(a i32 b i32) i32-add-clamp(a b)

fn call-function-value() i32
    = returned-function function-returning-a-function()
    returned-function(63i32 6i32)

fn dot-call-syntax-sugar-retturning-16() i32
    1i32
    .i32-add-clamp(3i32)
    .i32-mul-clamp(4i32)
"#,
            explainer: "As seen in the last example, function declarations can also have type parameters if only the result type uses them.
Each is specified in angled brackets <...>. When calling, each type argument is aso wrapped in angled brackets <...>.

To within a function create a function value (of type fn(...) ...),
type fn followed by typed parameter patterns in parens followed by the result.
The simplest parameter pattern is a variable name or underscore followed by its type.
See the \"query\" section for other kinds of patterns.

These local functions are not \"closures\", meaning they can NOT reference local variables introduced outside; only the parameter variables can be accessed.
This may seem surprising! But it's actually pretty nice to know what kind of inputs (and outputs) a function has!
Functions in slug:
- always take up very little space and dealing with them never interacts with the heap
- can be trivially duplicated and scrapped
- are easy and safe to pass to other threads and languages
To actually pass in variables from the surounding context, accept it as a parameter to that function"
        },
        Example::Match => ExampleInfo {
            // TODO show nested pattern matching
            name: "query",
            source: r#"
fn with-intermediate-local-variable() f32
    = intermediate f32-mul-clamp(3.14f32 2.07f32)
    f32-mul-clamp(intermediate intermediate)

fn opt-p32-to-u32(maybe opt(p32)) u32
    if maybe {
        No 0u32
        Yes(p32) p32-to-u32(p32)
    }
"#,
            explainer: "To decide what to do based on the shape of some value, put an if before the value, then one or more cases in curly braces.
A case consists of a (possibly nested) pattern without types followed by the result.
(You may know this from other languages as `switch`/`match`/`case`.)

In the simple case, you can assign some value to a new variable name using `= pattern value`, most commonly `= variable-name value`.
(You may know this from other languages as `let` or `const` binding.)
But you can also decompose it into its fields or even match on its variants in multiple cases.
A query is checked for missing cases so you don't forget some shape.
Works for \"record\", \"tuple\" and \"choice\" types whose examples show how to pattern match them."
        },
        Example::Group => ExampleInfo {
            name: "group",
            source: r#"
ty empty-tuple
    *

fn multiple-shapes-of-data-bundled-together()
    {weight f32 color color position position}
    {
        weight 1.0f32
        color {r 255u32 g 100u32 b 40u32}
        position {x 0.0f32 y 0.0f32}
    }

ty color {r u32 g u32 b u32}
ty position {x f32 y f32}

fn default-config() config {
    line-separator "\r\n"str
    item-separator ";"str
    version 2p32
}

ty config {
    line-separator str
    item-separator str
    version p32
}

ty position-as-a-tuple *(f32 f32)

fn example-position-as-a-tuple() position-as-a-tuple
    *(10.0f32 0f32)

ty vector {x f32 y f32}

fn example-vector() vector
    {x 2.0f32 y 3.0f32}

fn vector-length(vector vector) opt(f32)
    = {x x y y} vector
    f32-square-root(f32-add-clamp(
        f32-mul-clamp(x x)
        f32-mul-clamp(y y)
    ))

fn use-vector-length() opt(f32)
    vector-length(example-vector())
"#,
            explainer: "Passing some infos which are connected to each other as separate arguments is inconvenient and error-prone.
A \"record\" gives the individual values a field name and combines them into one value
(other languages usually call this struct(ure) or data object).
When you end up passing this record a bunch, it's probably a good idea to make a type alias for it.

Remember this weird * that you've seen?
It's just an empty tuple! You may be used to seeing this as `()` or `void` in other languages.
Tuples can be used instead of records when there are no reasonable field names available
or you need extra convenience because these are typed a lot.
I would recommend avoiding tuples with multiple values whenever you can.",
        },
        Example::Choice => ExampleInfo {
            name: "choice",
            source: r#"
ty bool [
    True
    False
]

fn bool-order(a bool b bool) order
    if *(a b) {
        *(False True) Less<order>
        *(True True) Equal<order>
        *(False False) Equal<order>
        *(True False) Greater<order>
    }

ty type(Types) [
    Variable(str)
    Construct(str span(Types type(Types)))
    Tuple(span(Types type(Types)))
    Function({
        inputs span(Types type(Types))
        output slot(Types type(Types))
    })
]

fn type-collect-variables(type type(Types) variables opt(span(Variables str)))
    opt(span(Variables str))
    if type {
        Variable(variable)
            Yes<[No]>(variables.opt-span-add(variable))
        Tuple(parts)
            type-span-collect-variables(parts variables)
        Construct(_ parts)
            type-span-collect-variables(parts variables)
        Function({inputs inputs output output})
            type-span-collect-variables(
                inputs
                type-collect-variables(item(output) variables)
            )
    }

fn type-span-collect-variables(
    type-span span(Types type(Types))
    variables opt(span(Variables str))
)
    opt(span(Variables str))
    span-step(
        type-span
        Up<[Down]>
        variables
        fn(
            variables-so-far opt(span(Variables str))
            type slot(Types type(Types))
        )
            type-collect-variables(item(type) variables-so-far)
    )
"#,
            explainer: "Some info can come in multiple shapes (variants).
For example there could be an error or a value, nothing or something, different state per page etc.
To construct a variant, choose an uppercase name, then a type in angle brackets <>, then optionally its values in parens.
The choice type in angle brackets does not need to include a variant with the name you're currently constructing.
(In other languages, this is typically done with object hierarchies or a kind enum + union of value types.)

The most common choice type in slug is `opt(_)`, the optional type which is `[No Yes(Value)]`.
In this example we also met `order` and `[Up Down]`.",
        },
        Example::Vec => ExampleInfo {
            name: "buf",
            source: r#"
fn number-buffer-sum() u32
    # even ignored bufs will be reachable until the end of the scope
    = _ buf-empty<* u32>
    = slot add<*>(1234u32)
    = span slot-to-span(slot)
    = extended-span span-add-example-u32s(span)
    u32s-sum(extended-span)

fn span-add-example-u32s(span span(Origin u32)) span(Origin u32)
    span
    .span-add(2u32)
    .span-add(3u32)
    .span-add(4u32)

fn u32s-sum(span span(Origin u32)) u32
    span-step(
        span
        # walk from start to end
        Up<[Down]>
        # start with sum 0
        0u32
        # how to update the sum for each slot in the span
        fn(so-far u32 slot slot(Origin u32))
            u32-add-clamp(so-far item(slot))
    )
"#,
            explainer: "A buf (short for array buffer) holds a variable amount of items of the same item type on the heap.
To refer to slices or specific items inside the heap, one might think a simple pointer is enough but...
If the buf has fully occupied its allotted space on the heap and a new item gets added,
the buf must move its items to a larger space, invalidating your pointers.

To combat this, you could move all slices and specific items to each their own little space on the heap.
This works well! But it can be slower as related memory is more fragmented and thus harder to find.

So, indexes and ranges? Yes! If we address the problems
- we don't know which buf to search in. Indexes are easy to confuse, leading to bugs
- because we cannot guarantee you've chosen the right buf,
  we need to bounds-check on every access etc. to be memory safe.
  The cost is small but it's a bit sad

Slug's solution is quite simple:
Check that only one buf with the same item type and tag exists at any given time
and that slots and spans don't outlive the buf.
- `buf(Origin Item)`: the resizable array on the heap, which can be tagged with an Origin
  to differentiate bufs with the same item type but different purposes
- `slot(Origin Item)` an index into the `buf` with the same origin and item type
- `span(Origin)` a range (start index + length) into the `buf` with the same origin and item type

More examples and explanations of this pattern can be found in the
[readme](https://codeberg.org/lue-bird/slug-lang).
The `buf` operation docs are also useful.",
        },
        Example::Comment => ExampleInfo {
            name: "comment",
            source: r#"
#shegang

fn Documented-after-the-type . : .
# fn documentation.
# All comments can span
# multiple lines.
=
    .

ty string
# type alias documentation
    str

fn Nice . : . =
    # in front of any expression
    .
"#,
            explainer: "Comments can be put in front of expressions or at the file level.
Documentation comments can be put after the type of a ty or fn declaration.",
        },
        Example::Extras => ExampleInfo {
            name: "extras",
            source: r#"
fn Make-it-3d xy .x f32 .y f32 : .x f32 .y f32 .z f32 =
    .. xy .z 0 f32
"#,
            explainer: "slug has special syntax sugar for combining records and fields (\"spreading\" its fields into the record).
Between, before or after the existing fields and spreads, add .. followed by a variable or any other expression.

This feature is not strictly necessary but it can make builders that carry e.g. a buf and span at every step a bit nicer.",
        }
    }
}

// //

fn html_text<Event>(content: &'static str) -> sauron::Node<Event> {
    sauron::Node::Leaf(sauron::vdom::Leaf::Text(std::borrow::Cow::Borrowed(
        content,
    )))
}
fn html_link_to<Event>(resource: &str, name: &'static str) -> sauron::Node<Event> {
    sauron::a([sauron::href(resource.to_string())], [html_text(name)])
}
fn linebreak_html<Event>() -> sauron::Node<Event> {
    sauron::br([], [])
}
fn html_style<Event>(value: impl Into<sauron::Value>) -> sauron::Attribute<Event> {
    sauron::attr("style", value)
}

fn utf16_offset_to_utf8_in(source: &str, utf16_offset: usize) -> usize {
    let mut utf8_length: usize = 0;
    let mut so_far_length_utf16: usize = 0;
    'traversing_utf16_length: for char in source.chars() {
        if so_far_length_utf16 >= utf16_offset {
            break 'traversing_utf16_length;
        }
        utf8_length += char.len_utf8();
        so_far_length_utf16 += char.len_utf16();
    }
    utf8_length
    // below does not work for string containing 2-part UTF-16 characters
    // source
    //     .encode_utf16()
    //     .take(utf16_offset)
    //     .map(|utf16_char| {
    //         char::decode_utf16([utf16_char])
    //             .map(|r| r.map(char::len_utf8).unwrap_or(0))
    //             .sum::<usize>()
    //     })
    //     .sum()
}
