/**
 * @file slug grammar for tree-sitter
 * @author lue-bird
 * @license Unlicense
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "slug",
  extras: ($) => [/\s/],
  rules: {
    source_file: ($) => repeat($.project_item),
    project_item: ($) => choice($.comment, $.project_fn, $.type_alias),
    type_alias: ($) =>
      seq(
        $.keyword_ty,
        $.type_name,
        optional(seq("(", repeat($.type_variable), ")")),
        repeat($.comment),
        $.type,
      ),
    keyword_ty: ($) => "ty",
    project_fn: ($) =>
      seq(
        $.keyword_fn,
        $.callable_name,
        optional(seq("<", repeat($.type_variable), ">")),
        "(",
        repeat($.pattern_typed),
        ")",
        $.type,
        // repeat($.comment) are already included in expression
        $.expression,
      ),

    expression: ($) =>
      prec.left(
        seq(
          choice(
            $.expression_commented,
            $.expression_number,
            $.expression_text,
            $.expression_variable,
            $.expression_call,
            $.expression_variant,
            $.expression_tuple,
            $.expression_record,
            $.expression_fn,
            $.expression_if,
            $.expression_deconstruct,
          ),
          repeat(seq($.key_symbol_chain, $.expression_call)),
        ),
      ),
    key_symbol_chain: ($) => ".",
    expression_commented: ($) =>
      seq(
        // semantically repeat1($.comment) but for simplicity the syntactic equivalent
        $.comment,
        $.expression,
      ),
    expression_number: ($) => seq($.number, $.type),
    expression_text: ($) => seq($.string, $.type),
    expression_variant: ($) =>
      seq(
        $.variant_name,
        optional(seq("<", $.type, ">")),
        optional(seq("(", repeat($.expression), ")")),
      ),
    expression_variable: ($) => $.callable_name,
    expression_call: ($) =>
      seq(
        $.callable_name,
        optional(seq("<", repeat($.type), ">")),
        "(",
        repeat($.expression),
        ")",
      ),
    expression_deconstruct: ($) =>
      seq($.key_symbol_equals, $.pattern_untyped, $.expression, $.expression),
    key_symbol_equals: ($) => "=",
    expression_if: ($) =>
      seq($.keyword_if, $.expression, "{", repeat($.expression_if_case), "}"),
    keyword_if: ($) => "if",
    expression_if_case: ($) => seq($.pattern_untyped, $.expression),
    expression_record: ($) => seq("{", repeat($.expression_record_field), "}"),
    expression_tuple: ($) =>
      seq($.key_symbol_tuple, optional(seq("(", repeat($.expression), ")"))),
    expression_record_field: ($) => seq($.field_name, $.expression),
    expression_fn: ($) =>
      seq($.keyword_fn, "(", repeat($.pattern_typed), ")", $.expression),

    pattern_typed: ($) =>
      choice(
        $.pattern_variable_typed,
        $.pattern_ignore_typed,
        $.pattern_variant_typed,
        $.pattern_tuple_typed,
        $.pattern_record_typed,
      ),
    pattern_untyped: ($) =>
      choice(
        $.pattern_variable_untyped,
        $.pattern_ignore_untyped,
        $.pattern_variant_untyped,
        $.pattern_tuple_untyped,
        $.pattern_record_untyped,
      ),
    pattern_variable_typed: ($) => seq($.pattern_variable_untyped, $.type),
    pattern_variable_untyped: ($) => $.lower_name,
    pattern_ignore_typed: ($) => seq($.pattern_ignore_untyped, $.type),
    pattern_ignore_untyped: ($) => "_",
    pattern_variant_typed: ($) =>
      seq($.variant_name, optional(seq("(", repeat($.pattern_typed), ")"))),
    pattern_variant_untyped: ($) =>
      seq($.variant_name, optional(seq("(", repeat($.pattern_untyped), ")"))),
    pattern_tuple_typed: ($) =>
      seq($.key_symbol_tuple, optional(seq("(", repeat($.pattern_typed), ")"))),
    pattern_tuple_untyped: ($) =>
      seq($.key_symbol_tuple, optional(seq("(", repeat($.pattern_untyped), ")"))),
    pattern_record_typed: ($) => seq("{", repeat($.pattern_field_typed), "}"),
    pattern_field_typed: ($) => seq($.field_name, $.pattern_typed),
    pattern_record_untyped: ($) => seq("{", repeat($.pattern_field_untyped), "}"),
    pattern_field_untyped: ($) => seq($.field_name, $.pattern_untyped),

    type: ($) =>
      choice(
        $.type_variable,
        $.type_fn,
        $.type_name,
        $.type_construct_with_arguments,
        $.type_tuple,
        $.type_record,
        $.type_choice,
      ),
    type_fn: ($) => seq($.keyword_fn, "(", repeat($.type), ")", $.type),
    type_construct_with_arguments: ($) => seq($.type_name, "(", repeat($.type), ")"),
    type_choice: ($) => seq("[", repeat($.type_choice_variant), "]"),
    type_choice_variant: ($) =>
      seq($.variant_name, optional(seq("(", repeat($.type), ")"))),
    type_tuple: ($) => seq($.key_symbol_tuple, optional(seq("(", repeat($.type), ")"))),
    type_record: ($) => seq("{", repeat($.type_record_field), "}"),
    type_record_field: ($) => seq($.field_name, $.type),
    type_name: ($) => $.lower_name,

    string: ($) => $.string_quoted,
    string_quoted: ($) => seq('"', repeat(choice("\\\\", '\\"', /[^"]/)), '"'),
    variant_name: ($) => $.upper_name,
    field_name: ($) => $.lower_name,
    type_variable: ($) => $.upper_name,
    callable_name: ($) => $.lower_name,

    keyword_fn: ($) => "fn",
    key_symbol_tuple: ($) => "*",

    number: ($) => /-?\+?\d+\.?\d*/,
    upper_name: ($) => /[A-Z][a-zA-Z0-9-]*/,
    lower_name: ($) => /[a-z][a-zA-Z0-9-]*/,
    comment: ($) => /#[^\n]*\n/,
  },
});
