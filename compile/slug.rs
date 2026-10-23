#![allow(non_upper_case_globals)]

use gen_lsp_types as lsp_types;
pub mod core;

pub type Name = byteyarn::Yarn;
#[derive(Clone, Copy, Debug)]
pub struct WithStartPosition<Value> {
    pub value: Value,
    pub start: lsp_types::Position,
}

#[derive(Debug)]
pub struct SyntaxProject<Expressions, Patterns> {
    pub items: Vec<SyntaxProjectItem<Expressions, Patterns>>,
}

#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    reason = "::Fn is the largest but most variants in practice are ::Fn anyway"
)]
pub enum SyntaxProjectItem<Expressions, Patterns> {
    TypeAlias {
        ty_keyword_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        parameters: Option<SyntaxTyParameters>,
        documentation: Option<SyntaxComments>,
        type_: Option<SyntaxType>,
    },
    Fn {
        fn_keyword_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        type_parameters: Option<SyntaxAngledTypeParameters>,
        open_paren_start: Option<lsp_types::Position>,
        parameters: Vec<SyntaxPattern<Patterns>>,
        closed_paren_start: Option<lsp_types::Position>,
        result_type: Option<SyntaxType>,
        documentation: Option<SyntaxComments>,
        result: Option<SyntaxExpression<Expressions, Patterns>>,
    },
    Comments(SyntaxComments),
    Unrecognized {
        range: lsp_types::Range,
        source: Box<str>,
    },
}
#[derive(Clone, Debug)]
pub struct SyntaxComments {
    pub line0: WithStartPosition<Box<str>>,
    pub line1_up: Vec<WithStartPosition<Box<str>>>,
}
#[derive(Clone, Debug)]
pub struct SyntaxAngledTypeParameters {
    pub open_angle_start: lsp_types::Position,
    pub names: Vec<WithStartPosition<Name>>,
    pub closed_angle_start: Option<lsp_types::Position>,
}
#[derive(Clone, Debug)]
pub struct SyntaxTyParameters {
    pub open_paren_start: lsp_types::Position,
    pub parameters: Vec<WithStartPosition<Name>>,
    pub closed_paren_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub enum SyntaxType {
    Variable(WithStartPosition<Name>),
    Origin {
        hat_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
    },
    Fn {
        fn_keyword_start: lsp_types::Position,
        open_paren_start: Option<lsp_types::Position>,
        parameters: Vec<SyntaxType>,
        closed_paren_start: Option<lsp_types::Position>,
        result: Option<Box<SyntaxType>>,
    },
    ConstructWithArguments {
        name: WithStartPosition<Name>,
        arguments: Option<SyntaxTypeConstructParenthesizedArguments>,
    },
    Tuple {
        star_start: lsp_types::Position,
        open_paren_start: Option<lsp_types::Position>,
        parts: Vec<SyntaxType>,
        closed_paren_start: Option<lsp_types::Position>,
    },
    Record {
        open_curly_start: lsp_types::Position,
        fields: Vec<SyntaxRecordField<SyntaxType>>,
        closed_curly_start: Option<lsp_types::Position>,
    },
    Choice {
        open_bracket_start: lsp_types::Position,
        variants: Vec<SyntaxTypeVariant>,
        closed_bracket_start: Option<lsp_types::Position>,
    },
}
#[derive(Debug)]
pub struct SyntaxTypeVariant {
    pub name: WithStartPosition<Name>,
    pub value: Option<SyntaxType>,
}
#[derive(Debug)]
pub struct SyntaxTypeConstructParenthesizedArguments {
    pub open_paren_start: lsp_types::Position,
    pub arguments: Vec<SyntaxType>,
    pub closed_paren_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub enum SyntaxPattern<Patterns> {
    Variable {
        name: WithStartPosition<Name>,
        type_: Option<SyntaxType>,
    },
    Ignored {
        underscore_start: lsp_types::Position,
        type_: Option<SyntaxType>,
    },
    Variant {
        open_bracket_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        value: Option<core::Slot<Patterns>>,
        closed_bracket_start: Option<lsp_types::Position>,
    },
    Tuple {
        star_start: lsp_types::Position,
        open_paren_start: Option<lsp_types::Position>,
        parts: Vec<SyntaxPattern<Patterns>>,
        closed_paren_start: Option<lsp_types::Position>,
    },
    Record {
        open_curly_start: lsp_types::Position,
        fields: Vec<SyntaxRecordField<SyntaxPattern<Patterns>>>,
        closed_curly_start: Option<lsp_types::Position>,
    },
}
#[derive(Debug)]
pub struct SyntaxAngledTypeArgument {
    pub open_angle_start: lsp_types::Position,
    pub type_: Option<SyntaxType>,
    pub closed_angle_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub struct SyntaxAngledTypeArguments {
    pub open_angle_start: lsp_types::Position,
    pub types: Vec<SyntaxType>,
    pub closed_angle_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub enum SyntaxExpression<Expressions, Patterns> {
    Number {
        value: WithStartPosition<Box<str>>,
        type_: Option<SyntaxType>,
    },
    Text {
        open_quote_start: lsp_types::Position,
        content: Box<str>,
        content_end: lsp_types::Position, // consider storing the content length
        closed_quote_exists: bool,
        type_: Option<SyntaxType>,
    },
    Variable(WithStartPosition<Name>),
    Call {
        first_argument_dot: Option<SyntaxExpressionArgumentDot<core::Slot<Expressions>>>,
        name: WithStartPosition<Name>,
        type_arguments: Option<SyntaxAngledTypeArguments>,
        open_paren_start: Option<lsp_types::Position>,
        arguments_rest: Vec<SyntaxExpression<Expressions, Patterns>>,
        closed_paren_start: Option<lsp_types::Position>,
    },
    Variant {
        open_bracket_start: lsp_types::Position,
        name: Option<WithStartPosition<Name>>,
        type_: Option<SyntaxAngledTypeArgument>,
        value: Option<core::Slot<Expressions>>,
        closed_bracket_start: Option<lsp_types::Position>,
    },
    Fn {
        fn_keyword_start: lsp_types::Position,
        open_paren_start: Option<lsp_types::Position>,
        parameters: Vec<SyntaxPattern<Patterns>>,
        closed_paren_start: Option<lsp_types::Position>,
        result: Option<core::Slot<Expressions>>,
    },
    Tuple {
        star_start: lsp_types::Position,
        open_paren_start: Option<lsp_types::Position>,
        parts: Vec<SyntaxExpression<Expressions, Patterns>>,
        closed_paren_start: Option<lsp_types::Position>,
    },
    Record {
        open_curly_start: lsp_types::Position,
        fields: Vec<SyntaxRecordField<SyntaxExpression<Expressions, Patterns>>>,
        closed_curly_start: Option<lsp_types::Position>,
    },
    Commented {
        comments: SyntaxComments,
        expression: Option<core::Slot<Expressions>>,
    },
    Destruct {
        equals_start: lsp_types::Position,
        pattern: Option<SyntaxPattern<Patterns>>,
        queried: Option<core::Slot<Expressions>>,
        result: Option<core::Slot<Expressions>>,
    },
    If {
        if_keyword_start: lsp_types::Position,
        queried: Option<core::Slot<Expressions>>,
        open_curly_start: Option<lsp_types::Position>,
        cases: Vec<SyntaxExpressionIfCase<Expressions, Patterns>>,
        closed_curly_start: Option<lsp_types::Position>,
    },
}
#[derive(Debug)]
pub struct SyntaxExpressionArgumentDot<Argument> {
    pub argument: Argument,
    pub dot_start: lsp_types::Position,
}
#[derive(Debug)]
pub struct SyntaxExpressionIfCase<Expressions, Patterns> {
    pub pattern: SyntaxPattern<Patterns>,
    pub result: Option<SyntaxExpression<Expressions, Patterns>>,
}
#[derive(Debug)]
pub struct SyntaxRecordField<Value> {
    pub name: WithStartPosition<Name>,
    pub value: Option<Value>,
}

pub fn name_end(name: WithStartPosition<&Name>) -> lsp_types::Position {
    position_add_characters(name.start, name.value.len() as u32)
}
pub fn name_range(name: WithStartPosition<&Name>) -> lsp_types::Range {
    lsp_types::Range {
        start: name.start,
        end: name_end(name),
    }
}
pub fn project_item_range<Expressions, Patterns>(
    item: &SyntaxProjectItem<Expressions, Patterns>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: project_item_start(item),
        end: project_item_end(item, expressions, patterns),
    }
}
pub fn project_item_start<Expressions, Patterns>(
    item: &SyntaxProjectItem<Expressions, Patterns>,
) -> lsp_types::Position {
    match item {
        SyntaxProjectItem::TypeAlias {
            ty_keyword_start,
            name: _,
            parameters: _,
            documentation: _,
            type_: _,
        } => *ty_keyword_start,
        SyntaxProjectItem::Fn {
            fn_keyword_start, ..
        } => *fn_keyword_start,
        SyntaxProjectItem::Comments(syntax_comments) => syntax_comments.line0.start,
        SyntaxProjectItem::Unrecognized { range, source: _ } => range.start,
    }
}
pub fn project_item_end<Expressions, Patterns>(
    item: &SyntaxProjectItem<Expressions, Patterns>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> lsp_types::Position {
    match item {
        SyntaxProjectItem::TypeAlias {
            ty_keyword_start,
            name,
            parameters,
            documentation,
            type_,
        } => type_
            .as_ref()
            .map(|type_| type_end(type_))
            .or_else(|| documentation.as_ref().map(|doc| comments_end(doc)))
            .or_else(|| parameters.as_ref().map(|params| ty_parameters_end(params)))
            .or_else(|| {
                name.as_ref()
                    .map(|name| name_end(with_start_position_as_ref(name)))
            })
            .unwrap_or(*ty_keyword_start),
        SyntaxProjectItem::Fn {
            fn_keyword_start,
            name,
            type_parameters,
            open_paren_start,
            parameters,
            closed_paren_start,
            result_type,
            documentation,
            result,
        } => result
            .as_ref()
            .map(|result| expression_end(result, expressions, patterns))
            .or_else(|| documentation.as_ref().map(|doc| comments_end(doc)))
            .or_else(|| {
                result_type
                    .as_ref()
                    .map(|result_type| type_end(result_type))
            })
            .or(*closed_paren_start)
            .or_else(|| {
                parameters
                    .last()
                    .map(|parameter| pattern_end(parameter, patterns))
            })
            .or(*open_paren_start)
            .or_else(|| {
                type_parameters
                    .as_ref()
                    .map(|type_parameters| angled_type_parameters_end(type_parameters))
            })
            .or_else(|| {
                name.as_ref()
                    .map(|name| name_end(with_start_position_as_ref(name)))
            })
            .unwrap_or(*fn_keyword_start),
        SyntaxProjectItem::Comments(syntax_comments) => comments_end(syntax_comments),
        SyntaxProjectItem::Unrecognized { range, source: _ } => range.end,
    }
}
pub fn ty_parameters_end(ty_parameters: &SyntaxTyParameters) -> lsp_types::Position {
    ty_parameters
        .closed_paren_start
        .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
        .or_else(|| {
            ty_parameters
                .parameters
                .last()
                .map(|last| name_end(with_start_position_as_ref(last)))
        })
        .unwrap_or_else(|| symbol_end(ty_parameters.open_paren_start, "("))
}
pub fn angled_type_parameters_end(
    angled_type_parameters: &SyntaxAngledTypeParameters,
) -> lsp_types::Position {
    angled_type_parameters
        .closed_angle_start
        .map(|closed_angle_start| symbol_end(closed_angle_start, ">"))
        .or_else(|| {
            angled_type_parameters
                .names
                .last()
                .map(|name| name_end(with_start_position_as_ref(name)))
        })
        .unwrap_or(symbol_end(angled_type_parameters.open_angle_start, "<"))
}
pub fn type_range(type_: &SyntaxType) -> lsp_types::Range {
    lsp_types::Range {
        start: type_start(type_),
        end: type_end(type_),
    }
}
pub fn type_start(type_: &SyntaxType) -> lsp_types::Position {
    match type_ {
        SyntaxType::Variable(name) => name.start,
        SyntaxType::Origin { hat_start, name: _ } => *hat_start,
        SyntaxType::Fn {
            fn_keyword_start, ..
        } => *fn_keyword_start,
        SyntaxType::ConstructWithArguments { name, .. } => name.start,
        SyntaxType::Tuple { star_start, .. } => *star_start,
        SyntaxType::Record {
            open_curly_start, ..
        } => *open_curly_start,
        SyntaxType::Choice {
            open_bracket_start, ..
        } => *open_bracket_start,
    }
}
pub fn type_end(type_: &SyntaxType) -> lsp_types::Position {
    match type_ {
        SyntaxType::Variable(name) => name_end(with_start_position_as_ref(name)),
        SyntaxType::Origin { hat_start, name } => match name {
            Some(name) => name_end(with_start_position_as_ref(name)),
            None => symbol_end(*hat_start, "^"),
        },
        SyntaxType::Fn {
            fn_keyword_start,
            open_paren_start,
            parameters,
            closed_paren_start,
            result,
        } => result
            .as_ref()
            .map(|result| type_end(result))
            .or_else(|| open_paren_start.map(|open_paren_start| symbol_end(open_paren_start, "(")))
            .or_else(|| parameters.last().map(|parameter| type_end(parameter)))
            .or_else(|| {
                closed_paren_start.map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
            })
            .unwrap_or_else(|| symbol_end(*fn_keyword_start, "fn")),
        SyntaxType::ConstructWithArguments { name, arguments } => match arguments {
            None => name_end(with_start_position_as_ref(name)),
            Some(SyntaxTypeConstructParenthesizedArguments {
                open_paren_start,
                arguments,
                closed_paren_start,
            }) => closed_paren_start
                .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
                .or_else(|| {
                    arguments
                        .last()
                        .map(|last_argument| type_end(last_argument))
                })
                .unwrap_or_else(|| symbol_end(*open_paren_start, "(")),
        },
        SyntaxType::Tuple {
            star_start,
            open_paren_start,
            parts,
            closed_paren_start,
        } => closed_paren_start
            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
            .or_else(|| parts.last().map(|part| type_end(part)))
            .or_else(|| open_paren_start.map(|open_paren_start| symbol_end(open_paren_start, "(")))
            .unwrap_or_else(|| symbol_end(*star_start, "*")),
        SyntaxType::Record {
            open_curly_start,
            fields,
            closed_curly_start,
        } => closed_curly_start
            .map(|closed_curly_start| symbol_end(closed_curly_start, "}"))
            .or_else(|| {
                fields.last().map(|last_field| {
                    last_field
                        .value
                        .as_ref()
                        .map(|value| type_end(value))
                        .unwrap_or_else(|| name_end(with_start_position_as_ref(&last_field.name)))
                })
            })
            .unwrap_or_else(|| symbol_end(*open_curly_start, "{")),
        SyntaxType::Choice {
            open_bracket_start: open_curly_start,
            variants,
            closed_bracket_start: closed_curly_start,
        } => closed_curly_start
            .map(|closed_curly_start| symbol_end(closed_curly_start, "}"))
            .or_else(|| {
                variants.last().map(|last_variant| {
                    last_variant
                        .value
                        .as_ref()
                        .map(|value| type_end(value))
                        .unwrap_or_else(|| name_end(with_start_position_as_ref(&last_variant.name)))
                })
            })
            .unwrap_or_else(|| symbol_end(*open_curly_start, "{")),
    }
}

pub fn pattern_range<Patterns>(
    pattern: &SyntaxPattern<Patterns>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: pattern_start(pattern),
        end: pattern_end(pattern, patterns),
    }
}
pub fn pattern_start<Patterns>(pattern: &SyntaxPattern<Patterns>) -> lsp_types::Position {
    match pattern {
        SyntaxPattern::Variable { name, type_: _ } => name.start,
        SyntaxPattern::Ignored {
            underscore_start,
            type_: _,
        } => *underscore_start,
        SyntaxPattern::Variant {
            open_bracket_start, ..
        } => *open_bracket_start,
        SyntaxPattern::Tuple { star_start, .. } => *star_start,
        SyntaxPattern::Record {
            open_curly_start, ..
        } => *open_curly_start,
    }
}
pub fn pattern_end<Patterns>(
    pattern: &SyntaxPattern<Patterns>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> lsp_types::Position {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => type_
            .as_ref()
            .map(|type_| type_end(type_))
            .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
        SyntaxPattern::Ignored {
            underscore_start,
            type_,
        } => type_
            .as_ref()
            .map(|type_| type_end(type_))
            .unwrap_or_else(|| symbol_end(*underscore_start, "_")),
        SyntaxPattern::Variant {
            open_bracket_start,
            name,
            value,
            closed_bracket_start,
        } => closed_bracket_start
            .map(|closed_bracket_start| symbol_end(closed_bracket_start, "]"))
            .or_else(|| {
                value
                    .as_ref()
                    .map(|value| pattern_end(patterns.item(value), patterns))
            })
            .or_else(|| {
                name.as_ref()
                    .map(|name| name_end(with_start_position_as_ref(name)))
            })
            .unwrap_or_else(|| symbol_end(*open_bracket_start, "[")),
        SyntaxPattern::Tuple {
            star_start,
            open_paren_start,
            parts,
            closed_paren_start,
        } => closed_paren_start
            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
            .or_else(|| parts.last().map(|part| pattern_end(part, patterns)))
            .or_else(|| open_paren_start.map(|open_paren_start| symbol_end(open_paren_start, "(")))
            .unwrap_or_else(|| symbol_end(*star_start, "*")),
        SyntaxPattern::Record {
            open_curly_start,
            fields,
            closed_curly_start,
        } => closed_curly_start
            .map(|closed_curly_start| symbol_end(closed_curly_start, "}"))
            .or_else(|| {
                fields.last().map(|last| {
                    last.value
                        .as_ref()
                        .map(|last_value| pattern_end(last_value, patterns))
                        .unwrap_or_else(|| name_end(with_start_position_as_ref(&last.name)))
                })
            })
            .unwrap_or_else(|| symbol_end(*open_curly_start, "{")),
    }
}
pub fn angled_type_argument_range(type_argument: &SyntaxAngledTypeArgument) -> lsp_types::Range {
    lsp_types::Range {
        start: type_argument.open_angle_start,
        end: angled_type_argument_end(type_argument),
    }
}
pub fn angled_type_argument_end(type_arguments: &SyntaxAngledTypeArgument) -> lsp_types::Position {
    type_arguments
        .closed_angle_start
        .map(|closed_brace_start| symbol_end(closed_brace_start, ">"))
        .or_else(|| {
            type_arguments
                .type_
                .as_ref()
                .map(|last_type| type_end(last_type))
        })
        .unwrap_or_else(|| symbol_end(type_arguments.open_angle_start, "<"))
}
pub fn angled_type_arguments_end(
    type_arguments: &SyntaxAngledTypeArguments,
) -> lsp_types::Position {
    type_arguments
        .closed_angle_start
        .map(|closed_brace_start| symbol_end(closed_brace_start, ">"))
        .or_else(|| {
            type_arguments
                .types
                .last()
                .map(|last_type| type_end(last_type))
        })
        .unwrap_or_else(|| symbol_end(type_arguments.open_angle_start, "<"))
}
pub fn expression_range<Expressions, Patterns>(
    expression: &SyntaxExpression<Expressions, Patterns>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: expression_start(expression),
        end: expression_end(expression, expressions, patterns),
    }
}
pub fn expression_start<Expressions, Patterns>(
    expression: &SyntaxExpression<Expressions, Patterns>,
) -> lsp_types::Position {
    match expression {
        SyntaxExpression::Number { value, type_: _ } => value.start,
        SyntaxExpression::Text {
            open_quote_start,
            content: _,
            content_end: _,
            closed_quote_exists: _,
            type_: _,
        } => *open_quote_start,
        SyntaxExpression::Variable(name) => name.start,
        SyntaxExpression::Call { name, .. } => name.start,
        SyntaxExpression::Variant {
            open_bracket_start, ..
        } => *open_bracket_start,
        SyntaxExpression::Fn {
            fn_keyword_start, ..
        } => *fn_keyword_start,
        SyntaxExpression::Tuple { star_start, .. } => *star_start,
        SyntaxExpression::Record {
            open_curly_start, ..
        } => *open_curly_start,
        SyntaxExpression::Commented {
            comments,
            expression: _,
        } => comments.line0.start,
        SyntaxExpression::Destruct { equals_start, .. } => *equals_start,
        SyntaxExpression::If {
            if_keyword_start, ..
        } => *if_keyword_start,
    }
}
pub fn expression_end<Expressions, Patterns>(
    expression: &SyntaxExpression<Expressions, Patterns>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> lsp_types::Position {
    match expression {
        SyntaxExpression::Number { value, type_ } => type_
            .as_ref()
            .map(|type_| type_end(type_))
            .unwrap_or_else(|| position_add_characters(value.start, value.value.len() as u32)),
        SyntaxExpression::Text {
            open_quote_start: _,
            content: _,
            content_end,
            closed_quote_exists,
            type_,
        } => type_
            .as_ref()
            .map(|type_| type_end(type_))
            .unwrap_or_else(|| {
                if *closed_quote_exists {
                    symbol_end(*content_end, "\"")
                } else {
                    *content_end
                }
            }),
        SyntaxExpression::Variable(name) => name_end(with_start_position_as_ref(name)),
        SyntaxExpression::Call {
            first_argument_dot: _,
            name,
            type_arguments,
            open_paren_start,
            arguments_rest,
            closed_paren_start,
        } => closed_paren_start
            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
            .or_else(|| {
                arguments_rest
                    .last()
                    .map(|argument| expression_end(argument, expressions, patterns))
            })
            .or_else(|| open_paren_start.map(|open_paren_start| symbol_end(open_paren_start, "(")))
            .or_else(|| {
                type_arguments
                    .as_ref()
                    .map(|type_arguments| angled_type_arguments_end(type_arguments))
            })
            .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
        SyntaxExpression::Variant {
            open_bracket_start,
            type_,
            name,
            value,
            closed_bracket_start,
        } => closed_bracket_start
            .map(|closed_bracket_start| symbol_end(closed_bracket_start, "]"))
            .or_else(|| {
                value
                    .as_ref()
                    .map(|value| expression_end(expressions.item(value), expressions, patterns))
            })
            .or_else(|| {
                name.as_ref()
                    .map(|name| name_end(with_start_position_as_ref(name)))
            })
            .or_else(|| type_.as_ref().map(|type_| angled_type_argument_end(type_)))
            .unwrap_or(symbol_end(*open_bracket_start, "[")),
        SyntaxExpression::Fn {
            fn_keyword_start,
            open_paren_start,
            parameters,
            closed_paren_start,
            result,
        } => result
            .as_ref()
            .map(|result_slot| expression_end(expressions.item(result_slot), expressions, patterns))
            .or_else(|| {
                closed_paren_start.map(|closed_bracket_start| symbol_end(closed_bracket_start, ")"))
            })
            .or_else(|| {
                parameters
                    .last()
                    .map(|parameter| pattern_end(parameter, patterns))
            })
            .or_else(|| open_paren_start.map(|open_paren_start| symbol_end(open_paren_start, "(")))
            .unwrap_or_else(|| symbol_end(*fn_keyword_start, "fn")),
        SyntaxExpression::Tuple {
            star_start,
            open_paren_start,
            parts,
            closed_paren_start,
        } => closed_paren_start
            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
            .or_else(|| {
                parts
                    .last()
                    .map(|part| expression_end(part, expressions, patterns))
            })
            .or_else(|| open_paren_start.map(|open_paren_start| symbol_end(open_paren_start, ")")))
            .unwrap_or_else(|| symbol_end(*star_start, "*")),
        SyntaxExpression::Record {
            open_curly_start,
            fields,
            closed_curly_start,
        } => closed_curly_start
            .map(|closed_curly_start| symbol_end(closed_curly_start, "}"))
            .or_else(|| {
                fields.last().map(|last_field| {
                    last_field
                        .value
                        .as_ref()
                        .map(|value| expression_end(value, expressions, patterns))
                        .unwrap_or_else(|| name_end(with_start_position_as_ref(&last_field.name)))
                })
            })
            .unwrap_or_else(|| symbol_end(*open_curly_start, "{")),
        SyntaxExpression::Commented {
            comments,
            expression,
        } => expression
            .as_ref()
            .map(|inner| expression_end(expressions.item(inner), expressions, patterns))
            .unwrap_or_else(|| comments_end(comments)),
        SyntaxExpression::Destruct {
            equals_start,
            pattern,
            queried,
            result,
        } => result
            .as_ref()
            .map(|result| expression_end(expressions.item(result), expressions, patterns))
            .or_else(|| {
                queried
                    .as_ref()
                    .map(|queried| expression_end(expressions.item(queried), expressions, patterns))
            })
            .or_else(|| {
                pattern
                    .as_ref()
                    .map(|pattern| pattern_end(pattern, patterns))
            })
            .unwrap_or_else(|| symbol_end(*equals_start, "=")),
        SyntaxExpression::If {
            if_keyword_start,
            queried,
            open_curly_start,
            cases,
            closed_curly_start,
        } => closed_curly_start
            .map(|closed_curly_start| symbol_end(closed_curly_start, "}"))
            .or_else(|| {
                cases.last().map(|last_case| {
                    last_case
                        .result
                        .as_ref()
                        .map(|result| expression_end(result, expressions, patterns))
                        .unwrap_or_else(|| pattern_end(&last_case.pattern, patterns))
                })
            })
            .or_else(|| open_curly_start.map(|open_curly_start| symbol_end(open_curly_start, "{")))
            .or_else(|| {
                queried
                    .as_ref()
                    .map(|queried| expression_end(expressions.item(queried), expressions, patterns))
            })
            .unwrap_or_else(|| symbol_end(*if_keyword_start, "if")),
    }
}
fn comments_end(comments: &SyntaxComments) -> lsp_types::Position {
    let last_line = comments.line1_up.last().unwrap_or(&comments.line0);
    position_add_characters(
        last_line.start,
        last_line.value.encode_utf16().count() as u32,
    )
}

pub struct ParseState<'a> {
    pub source: &'a str,
    pub offset_utf8: usize,
    pub position: lsp_types::Position,
}

fn parse_linebreak(state: &mut ParseState) -> bool {
    // see EOL in https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocuments
    if state.source[state.offset_utf8..].starts_with("\n") {
        state.offset_utf8 += 1;
        state.position.line += 1;
        state.position.character = 0;
        true
    } else if state.source[state.offset_utf8..].starts_with("\r\n") {
        state.offset_utf8 += 2;
        state.position.line += 1;
        state.position.character = 0;
        true
    } else if state.source[state.offset_utf8..].starts_with("\r") {
        state.offset_utf8 += 1;
        state.position.line += 1;
        state.position.character = 0;
        true
    } else {
        false
    }
}
fn str_starts_with_linebreak(str: &str) -> bool {
    // \r allowed because both \r and \r\n are counted as linebreak
    // see EOL in https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocuments
    str.starts_with("\n") || str.starts_with("\r")
}
/// prefer using after `parse_line_break` or similar failed
fn parse_any_guaranteed_non_linebreak_char(state: &mut ParseState) -> bool {
    match state.source[state.offset_utf8..].chars().next() {
        None => false,
        Some(parsed_char) => {
            state.offset_utf8 += parsed_char.len_utf8();
            state.position.character += parsed_char.len_utf16() as u32;
            true
        }
    }
}
fn parse_any_char(state: &mut ParseState) -> bool {
    // can probably be optimized
    parse_linebreak(state) || parse_any_guaranteed_non_linebreak_char(state)
}
/// symbol cannot contain non-utf8 characters or \n
fn parse_symbol(state: &mut ParseState, symbol: &str) -> bool {
    if state.source[state.offset_utf8..].starts_with(symbol) {
        state.offset_utf8 += symbol.len();
        state.position.character += symbol.len() as u32;
        true
    } else {
        false
    }
}
/// symbol cannot contain non-utf8 characters or \n
fn parse_symbol_as<A>(state: &mut ParseState, symbol: &'static str, result: A) -> Option<A> {
    if parse_symbol(state, symbol) {
        Some(result)
    } else {
        None
    }
}
/// symbol cannot contain non-utf8 characters or \n
fn parse_symbol_as_start(state: &mut ParseState, symbol: &str) -> Option<lsp_types::Position> {
    let start_position: lsp_types::Position = state.position;
    if parse_symbol(state, symbol) {
        Some(start_position)
    } else {
        None
    }
}
/// given condition must not succeed on linebreak
fn parse_same_line_char_if(state: &mut ParseState, char_is_valid: impl Fn(char) -> bool) -> bool {
    if let Some(next_char) = state.source[state.offset_utf8..].chars().next()
        && char_is_valid(next_char)
    {
        state.offset_utf8 += next_char.len_utf8();
        state.position.character += next_char.len_utf16() as u32;
        true
    } else {
        false
    }
}
/// given condition must not succeed on linebreak
fn parse_same_line_while(state: &mut ParseState, char_is_valid: impl Fn(char) -> bool) {
    let consumed_chars_iterator = state.source[state.offset_utf8..]
        .chars()
        .take_while(|&c| char_is_valid(c));
    let consumed_length_utf8: usize = consumed_chars_iterator.clone().map(char::len_utf8).sum();
    let consumed_length_utf16: usize = consumed_chars_iterator.map(char::len_utf16).sum();
    state.offset_utf8 += consumed_length_utf8;
    state.position.character += consumed_length_utf16 as u32;
}
fn parse_unsigned_integer_base10(state: &mut ParseState) -> bool {
    if parse_symbol(state, "0") {
        true
    } else if parse_same_line_char_if(state, |c| ('1'..='9').contains(&c)) {
        parse_same_line_while(state, |c| c.is_ascii_digit());
        true
    } else {
        false
    }
}

fn parse_before_next_linebreak_or_end_as_str<'a>(state: &mut ParseState<'a>) -> &'a str {
    let content: &str = state.source[state.offset_utf8..]
        .lines()
        .next()
        .unwrap_or("");
    state.offset_utf8 += content.len();
    state.position.character += content.encode_utf16().count() as u32;
    content
}

/// a valid slug symbol that must be followed by a character that could not be part of an slug identifier
fn parse_slug_keyword_as_start(
    state: &mut ParseState,
    symbol: &str,
) -> Option<lsp_types::Position> {
    if state.source[state.offset_utf8..].starts_with(symbol)
        && !(state.source[(state.offset_utf8 + symbol.len())..]
            .starts_with(|c: char| c.is_ascii_alphanumeric() || c == '-'))
    {
        let start_position: lsp_types::Position = state.position;
        state.offset_utf8 += symbol.len();
        state.position.character += symbol.len() as u32;
        Some(start_position)
    } else {
        None
    }
}
fn parse_slug_whitespace(state: &mut ParseState) {
    while parse_linebreak(state) || parse_same_line_char_if(state, char::is_whitespace) {}
}
fn parse_slug_comments(state: &mut ParseState) -> Option<SyntaxComments> {
    let Some(line0) = parse_slug_comment(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut line1_up = Vec::new();
    while let Some(next_comment_line) = parse_slug_comment(state) {
        line1_up.push(next_comment_line);
        parse_slug_whitespace(state);
    }
    Some(SyntaxComments {
        line0: line0,
        line1_up: line1_up,
    })
}
fn parse_slug_comment(state: &mut ParseState) -> Option<WithStartPosition<Box<str>>> {
    let Some(hashtag_start) = parse_symbol_as_start(state, "#") else {
        return None;
    };
    Some(WithStartPosition {
        start: hashtag_start,
        value: Box::from(parse_before_next_linebreak_or_end_as_str(state)),
    })
}
fn is_slug_lowercase_name_start(name_start: char) -> bool {
    name_start.is_ascii_lowercase()
}
fn parse_slug_lowercase_name(state: &mut ParseState) -> Option<Name> {
    let mut chars_from_offset: std::str::Chars = state.source[state.offset_utf8..].chars();
    if let Some(first_char) = chars_from_offset.next()
        && is_slug_lowercase_name_start(first_char)
    {
        let parsed_length: usize = first_char.len_utf8()
            + chars_from_offset
                .take_while(|&c| c.is_ascii_alphanumeric() || c == '-')
                .map(char::len_utf8)
                .sum::<usize>();
        let end_offset_utf8: usize = state.offset_utf8 + parsed_length;
        let parsed_str: &str = &state.source[state.offset_utf8..end_offset_utf8];
        state.offset_utf8 = end_offset_utf8;
        state.position.character += parsed_str.encode_utf16().count() as u32;
        Some(Name::from_fmt(format_args!("{}", parsed_str)))
    } else {
        None
    }
}
fn parse_slug_lowercase_name_with_start(state: &mut ParseState) -> Option<WithStartPosition<Name>> {
    let start_position: lsp_types::Position = state.position;
    if start_position.character == 0 {
        // disambiguate from project-level fn/ty keywords
        return None;
    }
    parse_slug_lowercase_name(state).map(|name| WithStartPosition {
        start: start_position,
        value: name,
    })
}

fn parse_slug_uppercase_name(state: &mut ParseState) -> Option<Name> {
    let mut chars_from_offset = state.source[state.offset_utf8..].chars();
    if let Some(first_char) = chars_from_offset.next()
        && first_char.is_ascii_uppercase()
    {
        let parsed_length: usize = first_char.len_utf8()
            + chars_from_offset
                .take_while(|&c| c.is_ascii_alphanumeric() || c == '-')
                .map(char::len_utf8)
                .sum::<usize>();
        let end_offset_utf8: usize = state.offset_utf8 + parsed_length;
        let parsed_str: &str = &state.source[state.offset_utf8..end_offset_utf8];
        state.offset_utf8 = end_offset_utf8;
        state.position.character += parsed_str.encode_utf16().count() as u32;
        Some(Name::from_fmt(format_args!("{}", parsed_str)))
    } else {
        None
    }
}
fn parse_slug_uppercase_name_with_start(state: &mut ParseState) -> Option<WithStartPosition<Name>> {
    let start_position: lsp_types::Position = state.position;
    parse_slug_uppercase_name(state).map(|name| WithStartPosition {
        start: start_position,
        value: name,
    })
}

pub fn parse_project<Expressions, Patterns>(
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,

    project_source: &str,
) -> SyntaxProject<Expressions, Patterns> {
    // entirely unscientific lower estimates c:
    let mut items = Vec::with_capacity(project_source.len() / 24);
    expressions.pre_allocate_at_least_usize(project_source.len() / 4);
    patterns.pre_allocate_at_least_usize(project_source.len() / 6);

    let mut state = ParseState {
        source: project_source,
        offset_utf8: 0,
        position: lsp_types::Position {
            line: 0,
            character: 0,
        },
    };
    let mut last_parsed_was_valid = true;
    let mut last_parsed_valid_end_offset_utf8 = state.offset_utf8;
    let mut last_parsed_valid_end_position = state.position;
    parse_slug_whitespace(&mut state);
    'parsing_items: loop {
        match parse_project_item(&mut state, expressions, patterns) {
            None => {
                if !parse_any_char(&mut state) {
                    if !last_parsed_was_valid {
                        items.push(SyntaxProjectItem::Unrecognized {
                            range: lsp_types::Range {
                                start: last_parsed_valid_end_position,
                                end: state.position,
                            },
                            source: Box::from(&project_source[last_parsed_valid_end_offset_utf8..]),
                        });
                    }
                    break 'parsing_items;
                }
                last_parsed_was_valid = false;
            }
            Some(item) => {
                if !last_parsed_was_valid {
                    items.push(SyntaxProjectItem::Unrecognized {
                        range: lsp_types::Range {
                            start: last_parsed_valid_end_position,
                            end: state.position,
                        },
                        source: Box::from(&project_source[last_parsed_valid_end_offset_utf8..]),
                    });
                }
                items.push(item);
                last_parsed_was_valid = true;
                parse_slug_whitespace(&mut state);
                last_parsed_valid_end_offset_utf8 = state.offset_utf8;
                last_parsed_valid_end_position = state.position;
            }
        }
    }
    SyntaxProject { items }
}

fn parse_project_item<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxProjectItem<Expressions, Patterns>> {
    parse_project_fn(state, expressions, patterns)
        .or_else(|| parse_project_ty(state))
        .or_else(|| parse_slug_comments(state).map(SyntaxProjectItem::Comments))
}
fn parse_project_ty<Expressions, Patterns>(
    state: &mut ParseState,
) -> Option<SyntaxProjectItem<Expressions, Patterns>> {
    let Some(ty_keyword_start) = parse_symbol_as_start(state, "ty") else {
        return None;
    };
    parse_slug_whitespace(state);
    let (name, parameters) = match parse_slug_lowercase_name_with_start(state) {
        Some(name) => (Some(name), None),
        None => {
            let name = parse_slug_uppercase_name_with_start(state);
            parse_slug_whitespace(state);
            let parameters = parse_ty_parameters(state);
            (name, parameters)
        }
    };
    parse_slug_whitespace(state);
    let documentation = parse_slug_comments(state);
    parse_slug_whitespace(state);
    let type_ = parse_type(state);
    Some(SyntaxProjectItem::TypeAlias {
        ty_keyword_start,
        name: name,
        parameters: parameters,
        documentation: documentation,
        type_: type_,
    })
}
fn parse_ty_parameters(state: &mut ParseState) -> Option<SyntaxTyParameters> {
    let Some(open_paren_start) = parse_symbol_as_start(state, "(") else {
        return None;
    };
    let mut parameters = Vec::new();
    while let Some(parameter) = parse_slug_lowercase_name_with_start(state) {
        parameters.push(parameter);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxTyParameters {
        open_paren_start,
        parameters,
        closed_paren_start,
    })
}
fn parse_angled_type_parameters(state: &mut ParseState) -> Option<SyntaxAngledTypeParameters> {
    let Some(open_angle_start) = parse_symbol_as_start(state, "<") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut names = Vec::with_capacity(1);
    while let Some(name) = parse_slug_lowercase_name_with_start(state) {
        names.push(name);
        parse_slug_whitespace(state);
    }
    let closed_angle_start = parse_symbol_as_start(state, ">");
    Some(SyntaxAngledTypeParameters {
        open_angle_start,
        names,
        closed_angle_start,
    })
}
fn parse_project_fn<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxProjectItem<Expressions, Patterns>> {
    let Some(fn_keyword_start) = parse_symbol_as_start(state, "fn") else {
        return None;
    };
    parse_slug_whitespace(state);
    let name = parse_slug_uppercase_name_with_start(state);
    parse_slug_whitespace(state);
    let type_parameters = parse_angled_type_parameters(state);
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    parse_slug_whitespace(state);
    let mut parameters = Vec::with_capacity(2);
    while let Some(parameter) = parse_parameter_pattern(state, patterns) {
        parameters.push(parameter);
        parse_slug_whitespace(state);
    }
    parse_slug_whitespace(state);
    parse_slug_whitespace(state);
    let closed_paren_start = parse_symbol_as_start(state, ")");
    parse_slug_whitespace(state);
    let result_type = parse_type(state);
    parse_slug_whitespace(state);
    let documentation = parse_slug_comments(state);
    parse_slug_whitespace(state);
    let result = parse_expression(state, expressions, patterns);
    Some(SyntaxProjectItem::Fn {
        fn_keyword_start: fn_keyword_start,
        name: name,
        type_parameters: type_parameters,
        open_paren_start,
        parameters,
        closed_paren_start,
        result_type: result_type,
        documentation: documentation,
        result: result,
    })
}
pub struct ProjectFnSignature {
    pub fn_keyword_start: lsp_types::Position,
    pub name: Option<WithStartPosition<Name>>,
    pub type_parameters: Option<SyntaxAngledTypeParameters>,
    pub open_paren_start: Option<lsp_types::Position>,
    pub parameter_types: Vec<SyntaxType>,
    pub closed_paren_start: Option<lsp_types::Position>,
    pub result_type: Option<SyntaxType>,
}
pub fn parse_project_fn_signature(state: &mut ParseState) -> Option<ProjectFnSignature> {
    let Some(fn_keyword_start) = parse_symbol_as_start(state, "fn") else {
        return None;
    };
    parse_slug_whitespace(state);
    let name = parse_slug_uppercase_name_with_start(state);
    parse_slug_whitespace(state);
    let type_parameters = parse_angled_type_parameters(state);
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    parse_slug_whitespace(state);
    let mut parameter_types = Vec::with_capacity(2);
    while let Some(parameter_type) = parse_type(state) {
        parameter_types.push(parameter_type);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    parse_slug_whitespace(state);
    let result_type = parse_type(state);
    Some(ProjectFnSignature {
        fn_keyword_start: fn_keyword_start,
        name: name,
        type_parameters: type_parameters,
        open_paren_start,
        parameter_types,
        closed_paren_start,
        result_type: result_type,
    })
}
pub fn parse_parameter_pattern<Patterns>(
    state: &mut ParseState,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxPattern<Patterns>> {
    parse_parameter_pattern_variable(state)
        .or_else(|| parse_parameter_pattern_ignored(state))
        .or_else(|| parse_parameter_pattern_variant(state, patterns))
        .or_else(|| parse_parameter_pattern_tuple(state, patterns))
        .or_else(|| parse_parameter_pattern_record(state, patterns))
}
pub fn parse_if_pattern<Patterns>(
    state: &mut ParseState,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxPattern<Patterns>> {
    parse_if_pattern_variable(state)
        .or_else(|| parse_if_pattern_ignored(state))
        .or_else(|| parse_if_pattern_variant(state, patterns))
        .or_else(|| parse_if_pattern_tuple(state, patterns))
        .or_else(|| parse_if_pattern_record(state, patterns))
}
fn parse_parameter_pattern_variable<Patterns>(
    state: &mut ParseState,
) -> Option<SyntaxPattern<Patterns>> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let type_ = parse_type(state);
    Some(SyntaxPattern::Variable {
        name: name,
        type_: type_,
    })
}
fn parse_parameter_pattern_ignored<Patterns>(
    state: &mut ParseState,
) -> Option<SyntaxPattern<Patterns>> {
    let Some(underscore_start) = parse_symbol_as_start(state, "_") else {
        return None;
    };
    parse_slug_whitespace(state);
    let type_ = parse_type(state);
    Some(SyntaxPattern::Ignored {
        underscore_start,
        type_: type_,
    })
}
fn parse_if_pattern_variable<Patterns>(state: &mut ParseState) -> Option<SyntaxPattern<Patterns>> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    Some(SyntaxPattern::Variable {
        name: name,
        type_: None,
    })
}
fn parse_if_pattern_ignored<Patterns>(state: &mut ParseState) -> Option<SyntaxPattern<Patterns>> {
    let Some(underscore_start) = parse_symbol_as_start(state, "_") else {
        return None;
    };
    Some(SyntaxPattern::Ignored {
        underscore_start,
        type_: None,
    })
}
fn parse_parameter_pattern_variant<Patterns>(
    state: &mut ParseState,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxPattern<Patterns>> {
    let Some(open_bracket_start) = parse_symbol_as_start(state, "[") else {
        return None;
    };
    parse_slug_whitespace(state);
    let name = parse_slug_lowercase_name_with_start(state);
    parse_slug_whitespace(state);
    let value = parse_parameter_pattern(state, patterns);
    parse_slug_whitespace(state);
    let closed_bracket_start = parse_symbol_as_start(state, "]");
    Some(SyntaxPattern::Variant {
        open_bracket_start,
        name: name,
        value: value.map(|value| patterns.insert(value)),
        closed_bracket_start,
    })
}
fn parse_if_pattern_variant<Patterns>(
    state: &mut ParseState,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxPattern<Patterns>> {
    let Some(open_bracket_start) = parse_symbol_as_start(state, "[") else {
        return None;
    };
    parse_slug_whitespace(state);
    let name = parse_slug_lowercase_name_with_start(state);
    parse_slug_whitespace(state);
    let value = parse_if_pattern(state, patterns);
    parse_slug_whitespace(state);
    let closed_bracket_start = parse_symbol_as_start(state, "]");
    Some(SyntaxPattern::Variant {
        open_bracket_start,
        name: name,
        value: value.map(|value| patterns.insert(value)),
        closed_bracket_start,
    })
}
fn parse_angled_type_argument(state: &mut ParseState) -> Option<SyntaxAngledTypeArgument> {
    let Some(open_angle_start) = parse_symbol_as_start(state, "<") else {
        return None;
    };
    parse_slug_whitespace(state);
    let type_ = parse_type(state);
    parse_slug_whitespace(state);
    let closed_angle_start = parse_symbol_as_start(state, ">");
    Some(SyntaxAngledTypeArgument {
        open_angle_start,
        type_: type_,
        closed_angle_start,
    })
}
fn parse_angled_type_arguments(state: &mut ParseState) -> Option<SyntaxAngledTypeArguments> {
    let Some(open_angle_start) = parse_symbol_as_start(state, "<") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut types = Vec::with_capacity(1);
    while let Some(type_) = parse_type(state) {
        types.push(type_);
        parse_slug_whitespace(state);
    }
    let closed_angle_start = parse_symbol_as_start(state, ">");
    Some(SyntaxAngledTypeArguments {
        open_angle_start,
        types,
        closed_angle_start,
    })
}
fn parse_parameter_pattern_tuple<Patterns>(
    state: &mut ParseState,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxPattern<Patterns>> {
    let Some(star_start) = parse_symbol_as_start(state, "*") else {
        return None;
    };
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    parse_slug_whitespace(state);
    let mut parts = Vec::new();
    while let Some(field) = parse_parameter_pattern(state, patterns) {
        parts.push(field);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxPattern::Tuple {
        star_start,
        open_paren_start,
        parts,
        closed_paren_start,
    })
}
fn parse_if_pattern_tuple<Patterns>(
    state: &mut ParseState,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxPattern<Patterns>> {
    let Some(star_start) = parse_symbol_as_start(state, "*") else {
        return None;
    };
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    parse_slug_whitespace(state);
    let mut parts = Vec::new();
    while let Some(field) = parse_if_pattern(state, patterns) {
        parts.push(field);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxPattern::Tuple {
        star_start,
        open_paren_start,
        parts,
        closed_paren_start,
    })
}
fn parse_parameter_pattern_record<Patterns>(
    state: &mut ParseState,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxPattern<Patterns>> {
    let Some(open_curly_start) = parse_symbol_as_start(state, "{") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_parameter_pattern_record_field(state, patterns) {
        fields.push(field);
        parse_slug_whitespace(state);
    }
    let closed_curly_start = parse_symbol_as_start(state, "}");
    Some(SyntaxPattern::Record {
        open_curly_start,
        fields,
        closed_curly_start,
    })
}
fn parse_parameter_pattern_record_field<Patterns>(
    state: &mut ParseState,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxRecordField<SyntaxPattern<Patterns>>> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let value = parse_parameter_pattern(state, patterns);
    Some(SyntaxRecordField {
        name: name,
        value: value.map(|value| value),
    })
}
fn parse_if_pattern_record<Patterns>(
    state: &mut ParseState,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxPattern<Patterns>> {
    let Some(open_curly_start) = parse_symbol_as_start(state, "{") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_if_pattern_record_field(state, patterns) {
        fields.push(field);
        parse_slug_whitespace(state);
    }
    let closed_curly_start = parse_symbol_as_start(state, "}");
    Some(SyntaxPattern::Record {
        open_curly_start,
        fields,
        closed_curly_start,
    })
}
fn parse_if_pattern_record_field<Patterns>(
    state: &mut ParseState,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxRecordField<SyntaxPattern<Patterns>>> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let value = parse_if_pattern(state, patterns);
    Some(SyntaxRecordField {
        name: name,
        value: value.map(|value| value),
    })
}

pub fn parse_type(state: &mut ParseState) -> Option<SyntaxType> {
    parse_type_variable(state)
        .or_else(|| parse_type_origin(state))
        .or_else(|| parse_type_construct(state))
        .or_else(|| parse_type_record(state))
        .or_else(|| parse_type_choice(state))
        .or_else(|| parse_type_fn(state))
}
fn parse_type_variable(state: &mut ParseState) -> Option<SyntaxType> {
    let Some(name) = parse_slug_uppercase_name_with_start(state) else {
        return None;
    };
    Some(SyntaxType::Variable(name))
}
fn parse_type_origin(state: &mut ParseState) -> Option<SyntaxType> {
    let Some(hat_start) = parse_symbol_as_start(state, "^") else {
        return None;
    };
    parse_slug_whitespace(state);
    let name = parse_slug_lowercase_name_with_start(state);
    Some(SyntaxType::Origin { hat_start, name })
}
fn parse_type_record(state: &mut ParseState) -> Option<SyntaxType> {
    let Some(open_curly_start) = parse_symbol_as_start(state, "{") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_type_field(state) {
        fields.push(field);
        parse_slug_whitespace(state);
    }
    let closed_curly_start = parse_symbol_as_start(state, "}");
    Some(SyntaxType::Record {
        open_curly_start,
        fields,
        closed_curly_start,
    })
}
fn parse_type_field(state: &mut ParseState) -> Option<SyntaxRecordField<SyntaxType>> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let value = parse_type(state);
    Some(SyntaxRecordField {
        name: name,
        value: value,
    })
}
fn parse_type_choice(state: &mut ParseState) -> Option<SyntaxType> {
    let Some(open_bracket_start) = parse_symbol_as_start(state, "[") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut variants = Vec::new();
    while let Some(variant) = parse_type_variant(state) {
        variants.push(variant);
        parse_slug_whitespace(state);
    }
    let closed_bracket_start = parse_symbol_as_start(state, "]");
    Some(SyntaxType::Choice {
        open_bracket_start,
        variants,
        closed_bracket_start,
    })
}
fn parse_type_variant(state: &mut ParseState) -> Option<SyntaxTypeVariant> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let value = parse_type(state);
    Some(SyntaxTypeVariant {
        name: name,
        value: value,
    })
}
fn parse_type_construct(state: &mut ParseState) -> Option<SyntaxType> {
    let Some(name) = parse_slug_uppercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let arguments = match parse_symbol_as_start(state, "(") {
        None => None,
        Some(open_paren_start) => {
            parse_slug_whitespace(state);
            let mut arguments = Vec::new();
            while let Some(argument_type) = parse_type(state) {
                arguments.push(argument_type);
                parse_slug_whitespace(state);
            }
            let closed_paren_start = parse_symbol_as_start(state, ")");
            Some(SyntaxTypeConstructParenthesizedArguments {
                open_paren_start,
                arguments,
                closed_paren_start,
            })
        }
    };
    Some(SyntaxType::ConstructWithArguments { name, arguments })
}
fn parse_type_fn(state: &mut ParseState) -> Option<SyntaxType> {
    let Some(fn_keyword_start) = parse_symbol_as_start(state, "fn") else {
        return None;
    };
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    parse_slug_whitespace(state);
    let mut parameters = Vec::new();
    while let Some(parameter) = parse_type(state) {
        parameters.push(parameter);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    parse_slug_whitespace(state);
    let result = parse_type(state);
    Some(SyntaxType::Fn {
        fn_keyword_start,
        open_paren_start,
        parameters,
        closed_paren_start,
        result: result.map(Box::new),
    })
}
pub fn parse_expression<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let Some(mut start_expression) = parse_expression_number(state)
        .or_else(|| parse_expression_str(state))
        .or_else(|| parse_expression_fn(state, expressions, patterns))
        .or_else(|| parse_expression_variable_or_call(state, expressions, patterns))
        .or_else(|| parse_expression_variant(state, expressions, patterns))
        .or_else(|| parse_expression_commented(state, expressions, patterns))
        .or_else(|| parse_expression_tuple(state, expressions, patterns))
        .or_else(|| parse_expression_record(state, expressions, patterns))
        .or_else(|| parse_expression_destruct(state, expressions, patterns))
        .or_else(|| parse_expression_if(state, expressions, patterns))
    else {
        return None;
    };
    while let Some(dot_start) = parse_symbol_as_start(state, ".")
        && let () = parse_slug_whitespace(state)
        // this returning None means parsing it as a call would be pointless
        // so we consume the dot and pretend it wasn't there
        && let Some(name) = parse_slug_lowercase_name_with_start(state)
    {
        let start_expression_dot = SyntaxExpressionArgumentDot {
            dot_start: dot_start,
            argument: expressions.insert(start_expression),
        };
        let start_expression_dot_call =
            parse_expression_variable_or_call_after_first_argument_dot_name(
                state,
                expressions,
                patterns,
                Some(start_expression_dot),
                name,
            );
        start_expression = start_expression_dot_call;
    }
    Some(start_expression)
}
fn parse_expression_tuple<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let Some(star_start) = parse_symbol_as_start(state, "*") else {
        return None;
    };
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    parse_slug_whitespace(state);
    let mut parts = Vec::new();
    while let Some(field) = parse_expression(state, expressions, patterns) {
        parts.push(field);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxExpression::Tuple {
        star_start,
        open_paren_start,
        parts,
        closed_paren_start,
    })
}
fn parse_expression_record<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let Some(open_curly_start) = parse_symbol_as_start(state, "{") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_expression_record_field(state, expressions, patterns) {
        fields.push(field);
        parse_slug_whitespace(state);
    }
    let closed_curly_start = parse_symbol_as_start(state, "}");
    Some(SyntaxExpression::Record {
        open_curly_start,
        fields,
        closed_curly_start,
    })
}
fn parse_expression_record_field<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxRecordField<SyntaxExpression<Expressions, Patterns>>> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let value = parse_expression(state, expressions, patterns);
    Some(SyntaxRecordField {
        name: name,
        value: value,
    })
}
fn parse_expression_number<Expressions, Patterns>(
    state: &mut ParseState,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let start = state.position;
    let Some(value) = parse_number(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let type_ = parse_type(state);
    Some(SyntaxExpression::Number {
        value: WithStartPosition {
            value: Box::from(value),
            start: start,
        },
        type_: type_,
    })
}
fn parse_expression_str<Expressions, Patterns>(
    state: &mut ParseState,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let Some(str) = parse_str(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let type_ = parse_type(state);
    Some(SyntaxExpression::Text {
        open_quote_start: str.open_quote_start,
        content: str.content.into_boxed_str(),
        content_end: str.content_end,
        closed_quote_exists: str.closed_quote_exists,
        type_: type_,
    })
}
fn parse_number<'a>(state: &mut ParseState<'a>) -> Option<&'a str> {
    let start_offset_utf8: usize = state.offset_utf8;
    if parse_symbol(state, "-") || parse_symbol(state, "+") {
        let _: bool = parse_unsigned_integer_base10(state);
        true
    } else if parse_unsigned_integer_base10(state) {
        false
    } else {
        return None;
    };
    let has_decimal_point: bool = parse_symbol(state, ".");
    if has_decimal_point {
        parse_same_line_while(state, |c| c.is_ascii_digit());
    }
    Some(&state.source[start_offset_utf8..state.offset_utf8])
}
struct SyntaxStr {
    open_quote_start: lsp_types::Position,
    content: String,
    content_end: lsp_types::Position,
    closed_quote_exists: bool,
}
fn parse_str(state: &mut ParseState) -> Option<SyntaxStr> {
    let Some(open_quote_start) = parse_symbol_as_start(state, "\"") else {
        return None;
    };
    let mut content: String = String::new();
    'parsing_content: loop {
        if let Some(closed_quote_start) = parse_symbol_as_start(state, "\"") {
            return Some(SyntaxStr {
                open_quote_start: open_quote_start,
                content: content,
                content_end: closed_quote_start,
                closed_quote_exists: true,
            });
        }
        match parse_text_content_char(state) {
            Some(next_content_char) => {
                content.push(next_content_char);
            }
            None => break 'parsing_content,
        }
    }
    Some(SyntaxStr {
        open_quote_start: open_quote_start,
        content: content,
        content_end: state.position,
        closed_quote_exists: false,
    })
}
fn parse_text_content_char(state: &mut ParseState) -> Option<char> {
    parse_symbol_as(state, "\\\\", '\\')
        .or_else(|| parse_symbol_as(state, "\\'", '\''))
        .or_else(|| parse_symbol_as(state, "\\n", '\n'))
        .or_else(|| parse_symbol_as(state, "\\r", '\r'))
        .or_else(|| parse_symbol_as(state, "\\t", '\t'))
        .or_else(|| parse_symbol_as(state, "\\\"", '"'))
        .or_else(|| {
            let start_offset_utf8: usize = state.offset_utf8;
            let start_position: lsp_types::Position = state.position;
            let reset_parse_state = |progressed_state: &mut ParseState| {
                progressed_state.offset_utf8 = start_offset_utf8;
                progressed_state.position = start_position;
            };
            if !parse_symbol(state, "\\(") {
                return None;
            }
            let unicode_hex_start_offset_utf8: usize = state.offset_utf8;
            parse_same_line_while(state, |c| c.is_ascii_hexdigit());
            let unicode_hex_str: &str =
                &state.source[unicode_hex_start_offset_utf8..state.offset_utf8];
            let _: bool = parse_symbol(state, ")");
            let Ok(code_point) = u32::from_str_radix(unicode_hex_str, 16) else {
                reset_parse_state(state);
                return None;
            };
            match char::from_u32(code_point) {
                Some(char) => Some(char),
                None => {
                    reset_parse_state(state);
                    None
                }
            }
        })
        .or_else(|| {
            if str_starts_with_linebreak(&state.source[state.offset_utf8..]) {
                None
            } else {
                match state.source[state.offset_utf8..].chars().next() {
                    None => None,
                    Some(plain_char) => {
                        state.offset_utf8 += plain_char.len_utf8();
                        state.position.character += plain_char.len_utf16() as u32;
                        Some(plain_char)
                    }
                }
            }
        })
}
fn parse_expression_commented<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let Some(comments) = parse_slug_comments(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let expression = parse_expression(state, expressions, patterns);
    Some(SyntaxExpression::Commented {
        comments: comments,
        expression: expression.map(|expression| expressions.insert(expression)),
    })
}
fn parse_expression_variable_or_call<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    Some(
        parse_expression_variable_or_call_after_first_argument_dot_name(
            state,
            expressions,
            patterns,
            None,
            name,
        ),
    )
}
fn parse_expression_variable_or_call_after_first_argument_dot_name<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
    first_argument_dot: Option<SyntaxExpressionArgumentDot<core::Slot<Expressions>>>,
    name: WithStartPosition<Name>,
) -> SyntaxExpression<Expressions, Patterns> {
    parse_slug_whitespace(state);
    let type_arguments = parse_angled_type_arguments(state);
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    if type_arguments.is_none() || open_paren_start.is_none() {
        return match first_argument_dot {
            None => SyntaxExpression::Variable(name),
            Some(first_argument_dot) => SyntaxExpression::Call {
                first_argument_dot: Some(first_argument_dot),
                name: name,
                type_arguments: None,
                open_paren_start: None,
                arguments_rest: vec![],
                closed_paren_start: None,
            },
        };
    }
    parse_slug_whitespace(state);
    let mut arguments_rest = Vec::new();
    while let Some(argument) = parse_expression(state, expressions, patterns) {
        arguments_rest.push(argument);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    SyntaxExpression::Call {
        first_argument_dot,
        name,
        type_arguments,
        open_paren_start,
        arguments_rest,
        closed_paren_start,
    }
}

fn parse_expression_variant<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let Some(open_bracket_start) = parse_symbol_as_start(state, "[") else {
        return None;
    };
    parse_slug_whitespace(state);
    let name = parse_slug_lowercase_name_with_start(state);
    parse_slug_whitespace(state);
    let type_argument = parse_angled_type_argument(state);
    parse_slug_whitespace(state);
    let value = parse_expression(state, expressions, patterns);
    parse_slug_whitespace(state);
    let closed_bracket_start = parse_symbol_as_start(state, "[");
    Some(SyntaxExpression::Variant {
        open_bracket_start,
        type_: type_argument,
        name: name,
        value: value.map(|argument| expressions.insert(argument)),
        closed_bracket_start,
    })
}
fn parse_expression_fn<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let Some(fn_keyword_start) = parse_slug_keyword_as_start(state, "fn") else {
        return None;
    };
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    parse_slug_whitespace(state);
    let mut parameters = Vec::with_capacity(2);
    while let Some(parameter) = parse_parameter_pattern(state, patterns) {
        parameters.push(parameter);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    parse_slug_whitespace(state);
    let result = parse_expression(state, expressions, patterns);
    Some(SyntaxExpression::Fn {
        fn_keyword_start,
        open_paren_start,
        parameters,
        closed_paren_start,
        result: result.map(|result| expressions.insert(result)),
    })
}
fn parse_expression_destruct<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let Some(equals_start) = parse_symbol_as_start(state, "=") else {
        return None;
    };
    parse_slug_whitespace(state);
    let pattern = parse_if_pattern(state, patterns);
    parse_slug_whitespace(state);
    let queried = parse_expression(state, expressions, patterns);
    parse_slug_whitespace(state);
    let result = parse_expression(state, expressions, patterns);
    Some(SyntaxExpression::Destruct {
        equals_start,
        pattern,
        queried: queried.map(|queried| expressions.insert(queried)),
        result: result.map(|queried| expressions.insert(queried)),
    })
}
fn parse_expression_if<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxExpression<Expressions, Patterns>> {
    let Some(if_keyword_start) = parse_symbol_as_start(state, "if") else {
        return None;
    };
    parse_slug_whitespace(state);
    let queried = parse_expression(state, expressions, patterns);
    parse_slug_whitespace(state);
    let open_curly_start = parse_symbol_as_start(state, "{");
    parse_slug_whitespace(state);
    let mut cases = Vec::new();
    while let Some(case) = parse_expression_if_case(state, expressions, patterns) {
        cases.push(case);
        parse_slug_whitespace(state);
    }
    let closed_curly_start = parse_symbol_as_start(state, "}");
    Some(SyntaxExpression::If {
        if_keyword_start,
        queried: queried.map(|queried| expressions.insert(queried)),
        open_curly_start,
        cases: cases,
        closed_curly_start,
    })
}
fn parse_expression_if_case<Expressions, Patterns>(
    state: &mut ParseState,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxExpressionIfCase<Expressions, Patterns>> {
    let Some(pattern) = parse_if_pattern(state, patterns) else {
        return None;
    };
    parse_slug_whitespace(state);
    let result = parse_expression(state, expressions, patterns);
    Some(SyntaxExpressionIfCase {
        pattern: pattern,
        result: result,
    })
}

#[derive(Clone, Debug)]
pub struct CheckedTypeAlias {
    pub name_range: Option<lsp_types::Range>,
    pub parameters: Vec<Name>,
    pub documentation: Option<Box<str>>,
    pub type_: Option<Type>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Variable(Name),
    Origin(Name),
    Tuple(Vec<Type>),
    Record(Vec<TypeField>),
    Choice(Vec<TypeVariant>),
    CoreConstruct {
        name: Name,
        arguments: Vec<Type>,
    },
    Fn {
        parameters: Vec<Type>,
        result: Box<Type>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeField {
    pub name: Name,
    pub value: Type,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeVariant {
    pub name: Name,
    pub value: Type,
}
#[derive(Clone, Debug)]
pub struct CheckedProjectFn {
    pub documentation: Option<Box<str>>,
    pub type_parameters: Vec<Name>,
    pub parameter_types: Vec<Option<Type>>,
    pub result_type: Option<Type>,
    pub result_expression_is_invalid: bool,
}
pub fn syntax_project_check<'a, Expressions, Patterns>(
    errors: &mut Vec<ErrorNode>,
    syntax_project: &'a SyntaxProject<Expressions, Patterns>,
    expressions: &'a core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> CheckedSyntaxProject<'a, Expressions, Patterns> {
    let mut type_graph: strongly_connected_components::Neighbors = Vec::new();
    let mut type_graph_node_by_name: std::collections::HashMap<
        &str,
        strongly_connected_components::Vertex,
    > = std::collections::HashMap::new();
    let mut project_type_by_graph_node: std::collections::HashMap<
        strongly_connected_components::Vertex,
        SyntaxProjectTypeInfo,
    > = std::collections::HashMap::new();

    let mut project_fn_graph: strongly_connected_components::Neighbors = Vec::new();
    let mut project_fn_graph_node_by_name: std::collections::HashMap<
        &Name,
        strongly_connected_components::Vertex,
    > = std::collections::HashMap::with_capacity(syntax_project.items.len());
    let mut project_fn_by_graph_node: std::collections::HashMap<
        strongly_connected_components::Vertex,
        SyntaxProjectFnInfo<Expressions, Patterns>,
    > = std::collections::HashMap::with_capacity(syntax_project.items.len());

    let mut records_used: std::collections::HashSet<Vec<Name>> =
        std::collections::HashSet::with_capacity(16);
    let mut choices_used: std::collections::HashSet<Vec<Name>> =
        std::collections::HashSet::with_capacity(4);

    'building_graph_across_items: for project_item in &syntax_project.items {
        match project_item {
            SyntaxProjectItem::Comments(_) => {}
            SyntaxProjectItem::Unrecognized {
                range: unknown_range,
                source: unknown_source,
            } => {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: unknown_range.start,
                        end: position_add_characters(unknown_range.start, 1)
                    },
                    message: format!("unrecognized syntax. {}
If this section of code is in a declaration, there might be another error for that declaration that can give more helpful hints.

If you instead wanted to create a declaration, try one of:
  - fn Some-fn-name some-parameter some-parameter-type : some-result-type = some-result-value
  - ty some-type-name some type",
                    if unknown_source
                        .starts_with('_') {
                        "Identifiers can themselves not contain underscores; try using a dash - instead, like some-nice-name?
If you were trying to start a type variable, no type was expected here. Maybe you're missing a pattern variable name in front, a comma or parenthesis?"
                    } else if unknown_source
                        .starts_with(|c: char| c.is_ascii_lowercase())
                    {
                        "It could be that a name starting with an uppercase letter is expected here (only types with parameters and project function names start uppercase). Also, is it indented correctly?"
                    } else if unknown_source
                        .starts_with(|c: char| c.is_ascii_uppercase())
                    {
                        "It could be that a name starting with a lowercase letter is expected here (only types with parameters and project function names start uppercase). Also, is it indented correctly?"
                    } else if unknown_source
                        .starts_with('#')
                    {
                        "Comments can only be put in front of expressions, after the header of a project fn (before the =) or ty (before the aliased type) or outside of any declarations. Maybe you accidentally indented it?"
                    } else if unknown_source.starts_with("//")
                        || unknown_source.starts_with("--")
                    {
                        "Comments start with #"
                    } else   if unknown_source
                        .starts_with('.')
                    {
                        "Record.field access is not a feature in slug. Instead, use pattern matching, like value ? your-value [.field variable ..other fields..] result. Otherwise, is everything indented correctly?"
                    } else if unknown_source
                        .starts_with(['+', '*', '/', '!', '&', '|'])
                    {
                        "Operator application is not a feature in slug. Instead, use regular function calls like F32-add-clamp, I32-negate or U32-mul-clamp."
                    } else if unknown_source
                        .starts_with(')') {
                        "One too many closing paren ). Try removing it."
                    } else if unknown_source
                        .starts_with(']') {
                        "One too many closing square bracket ]. Try removing it."
                    } else if unknown_source
                        .starts_with('}') {
                        "One too many closing curly bracket }. Try removing it."
                    } else {
                        "Is it indented correctly? Are brackets/braces/parens/quotes or similar closed prematurely or too often?"
                    }).into_boxed_str(),
                });
            }
            SyntaxProjectItem::TypeAlias {
                ty_keyword_start,
                name: maybe_name,
                parameters,
                documentation,
                type_,
            } => {
                let Some(name_node) = maybe_name else {
                    errors.push(ErrorNode {
                        range: symbol_range(*ty_keyword_start, "ty"),
                        message: Box::from("missing name after ty. Type names start with a lowercase (when having no parameters) or uppercase (when having parameters) letter and only use ascii letters, digits and -")
                    });
                    continue 'building_graph_across_items;
                };
                // allocating a string here just to check hurts a little
                if type_graph_node_by_name.contains_key(
                    name_toggle_case_of_first_ascii(name_node.value.to_string()).as_str(),
                ) {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name_node)),
                        message: Box::from(
                            "a type with the same name (with a different case of the first letter) already exists.
This can be confusing and will lead to conflicts if the number of type parameters changes on either of those types.
(It will also generate uglier, harder to find names in the compiled output).
Choose a different ty name.",
                        ),
                    });
                    continue 'building_graph_across_items;
                }
                _ = type_graph_node_by_name
                    .entry(&name_node.value)
                    .and_modify(|_| {
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(name_node)),
                            message: Box::from(
                                "a type with this name already exists. Choose a different ty name",
                            ),
                        });
                    })
                    .or_insert_with(|| {
                        let type_alias_declaration_graph_node: strongly_connected_components::Vertex =
                            strongly_connected_components::neighbors_add_vertex(&mut type_graph);
                        project_type_by_graph_node.insert(
                            type_alias_declaration_graph_node,
                            SyntaxProjectTypeInfo {
                                documentation: documentation,
                                name: name_node,
                                parameters: parameters,
                                type_: type_,
                            },
                        );
                        type_alias_declaration_graph_node
                    });
            }
            SyntaxProjectItem::Fn {
                fn_keyword_start,
                name: maybe_name,
                type_parameters,
                open_paren_start: _,
                parameters,
                closed_paren_start: _,
                result_type,
                documentation,
                result: maybe_result,
            } => {
                let Some(name) = maybe_name else {
                    errors.push(ErrorNode {
                        range: symbol_range(*fn_keyword_start, "fn"),
                        message: Box::from(
                            "missing name. Function names start with an uppercase letter and only use ascii letters, digits and -"
                        )
                    });
                    continue 'building_graph_across_items;
                };
                if core_fns.contains_key(name.value.as_str()) {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: Box::from(
                            "a function with this name is already part of core (which for example includes U32-add-clamp, Buf-empty etc.). Choose a different fn name"
                        )
                    });
                    continue 'building_graph_across_items;
                }
                _ = project_fn_graph_node_by_name
                    .entry(&name.value)
                    .and_modify(|_| {
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(name)),
                            message: Box::from(
                                "a project function with this name is already declared. Rename one of them",
                            ),
                        });
                    })
                    .or_insert_with(|| {
                        let project_fn_graph_node: strongly_connected_components::Vertex =
                            strongly_connected_components::neighbors_add_vertex(&mut project_fn_graph);
                        project_fn_by_graph_node.insert(
                            project_fn_graph_node,
                            SyntaxProjectFnInfo {
                                range: lsp_types::Range {
                                    start: *fn_keyword_start,
                                    end: maybe_result
                                        .as_ref()
                                        .map(|result| {
                                            expression_end(result, expressions, patterns)
                                        })
                                        .or_else(|| documentation.as_ref().map(comments_end))
                                        .or_else(|| {
                                            result_type
                                                .as_ref()
                                                .map(|result_type| type_end(result_type))
                                        })
                                        .or_else(|| {
                                            parameters.last().map(|parameter| {
                                                pattern_end(parameter, patterns)
                                            })
                                        })
                                        .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
                                },
                                name: name,
                                type_parameters: type_parameters,
                                parameters,
                                result_type: result_type,
                                documentation: documentation,
                                result: maybe_result,
                            },
                        );
                        project_fn_graph_node
                    });
            }
        }
    }
    for (&type_declaration_graph_node, &type_declaration_info) in project_type_by_graph_node.iter()
    {
        if let Some(aliased_type) = &type_declaration_info.type_ {
            let mut project_type_referenced_vertexes = std::collections::BTreeSet::new();
            syntax_type_connect_type_names_in_graph_from(
                &type_graph_node_by_name,
                aliased_type,
                &mut project_type_referenced_vertexes,
            );
            type_graph[type_declaration_graph_node] = project_type_referenced_vertexes;
        }
    }
    for (&project_fn_graph_node, project_fn_info) in project_fn_by_graph_node.iter() {
        let project_fn_graph: &mut strongly_connected_components::Neighbors = &mut project_fn_graph;
        if let Some(result_node) = project_fn_info.result {
            let mut project_fn_referenced_vertexes = std::collections::BTreeSet::new();
            syntax_expression_connect_fns_in_graph_from(
                &project_fn_graph_node_by_name,
                expressions,
                result_node,
                &mut project_fn_referenced_vertexes,
            );
            project_fn_graph[project_fn_graph_node] = project_fn_referenced_vertexes;
        }
    }

    // maybe instead start from 0 instead of cloning every time?
    let mut checked_type_aliases: std::collections::HashMap<Name, CheckedTypeAlias> =
        core_type_aliases.clone();
    checked_type_aliases.reserve(project_type_by_graph_node.len());
    for project_type_strongly_connected_component in
        strongly_connected_components::find(&type_graph)
    {
        for project_type in project_type_strongly_connected_component
            .iter()
            .filter_map(|variable_declaration_graph_node| {
                project_type_by_graph_node.get(&variable_declaration_graph_node)
            })
            .copied()
        {
            checked_type_aliases.insert(
                project_type.name.value.clone(),
                project_type_alias_check(
                    errors,
                    &checked_type_aliases,
                    project_type,
                    &mut records_used,
                    &mut choices_used,
                ),
            );
        }
    }

    // maybe instead start from 0 instead of cloning every time?
    let mut checked_project_fns: std::collections::HashMap<Name, CheckedProjectFn> =
        core_fns.clone();
    checked_project_fns.reserve(project_fn_graph.len());
    let mut checked_calls: std::collections::HashMap<lsp_types::Position, CheckedCall> =
        std::collections::HashMap::new();
    let mut checked_local_fns: std::collections::HashMap<lsp_types::Position, CheckedLocalFn> =
        std::collections::HashMap::new();
    let mut checked_queries: std::collections::HashMap<lsp_types::Position, CheckedQuery> =
        std::collections::HashMap::new();
    let mut checked_spread_records: std::collections::HashMap<lsp_types::Position, Vec<Name>> =
        std::collections::HashMap::new();
    for project_fn_strongly_connected_component in
        strongly_connected_components::find(&project_fn_graph)
    {
        let project_fns_in_strongly_connected_component: Vec<
            SyntaxProjectFnInfo<Expressions, Patterns>,
        > = project_fn_strongly_connected_component
            .into_iter()
            .filter_map(|project_fn_graph_node| {
                project_fn_by_graph_node.get(&project_fn_graph_node)
            })
            .copied()
            .collect();
        // possible optimization: skip pre-compile-type-info computation when project_fns_in_strongly_connected_component is single, non-self-referencing node
        for project_fn in project_fns_in_strongly_connected_component.iter().copied() {
            checked_project_fns.insert(
                project_fn.name.value.clone(),
                syntax_project_fn_header_check(
                    errors,
                    &checked_type_aliases,
                    patterns,
                    project_fn,
                    &mut records_used,
                    &mut choices_used,
                    &mut checked_spread_records,
                ),
            );
        }
        for project_fn in project_fns_in_strongly_connected_component {
            checked_project_fns.insert(
                project_fn.name.value.clone(),
                syntax_project_fn_check(
                    errors,
                    &checked_type_aliases,
                    &checked_project_fns,
                    expressions,
                    patterns,
                    project_fn,
                    &mut checked_calls,
                    &mut checked_local_fns,
                    &mut checked_queries,
                    &mut checked_spread_records,
                    &mut records_used,
                    &mut choices_used,
                ),
            );
        }
    }
    CheckedSyntaxProject {
        type_graph: type_graph,
        project_type_by_graph_node: project_type_by_graph_node,
        project_fn_graph: project_fn_graph,
        project_fn_graph_node_by_name: project_fn_graph_node_by_name,
        project_fn_by_graph_node: project_fn_by_graph_node,
        records_used: records_used,
        choices_used: choices_used,
        checked_type_aliases: checked_type_aliases,
        checked_project_fns: checked_project_fns,
        checked_calls: checked_calls,
        checked_local_fns: checked_local_fns,
        checked_queries: checked_queries,
        checked_spread_records: checked_spread_records,
    }
}
pub struct CheckedSyntaxProject<'a, Expressions, Patterns> {
    pub type_graph: strongly_connected_components::Neighbors,
    pub project_type_by_graph_node:
        std::collections::HashMap<strongly_connected_components::Vertex, SyntaxProjectTypeInfo<'a>>,
    pub project_fn_graph: strongly_connected_components::Neighbors,
    pub project_fn_graph_node_by_name:
        std::collections::HashMap<&'a Name, strongly_connected_components::Vertex>,
    pub project_fn_by_graph_node: std::collections::HashMap<
        strongly_connected_components::Vertex,
        SyntaxProjectFnInfo<'a, Expressions, Patterns>,
    >,
    pub records_used: std::collections::HashSet<Vec<Name>>,
    pub choices_used: std::collections::HashSet<Vec<Name>>,
    pub checked_type_aliases: std::collections::HashMap<Name, CheckedTypeAlias>,
    pub checked_project_fns: std::collections::HashMap<Name, CheckedProjectFn>,
    pub checked_calls: std::collections::HashMap<lsp_types::Position, CheckedCall>,
    pub checked_local_fns: std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    pub checked_queries: std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    pub checked_spread_records: std::collections::HashMap<lsp_types::Position, Vec<Name>>,
}
fn syntax_type_connect_type_names_in_graph_from(
    type_graph_node_by_name: &std::collections::HashMap<
        &str,
        strongly_connected_components::Vertex,
    >,
    type_: &SyntaxType,
    type_graph: &mut std::collections::BTreeSet<strongly_connected_components::Vertex>,
) {
    match type_ {
        SyntaxType::Variable(_) => {}
        SyntaxType::Origin { .. } => {}
        SyntaxType::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            for parameter in parameters {
                syntax_type_connect_type_names_in_graph_from(
                    type_graph_node_by_name,
                    parameter,
                    type_graph,
                );
            }
            if let Some(result) = result {
                syntax_type_connect_type_names_in_graph_from(
                    type_graph_node_by_name,
                    result,
                    type_graph,
                );
            }
        }
        SyntaxType::ConstructWithArguments { name, arguments } => {
            if let Some(referenced_type_graph_node) =
                type_graph_node_by_name.get(name.value.as_str()).copied()
            {
                type_graph.insert(referenced_type_graph_node);
            }
            if let Some(SyntaxTypeConstructParenthesizedArguments {
                open_paren_start: _,
                arguments,
                closed_paren_start: _,
            }) = arguments
            {
                for argument in arguments {
                    syntax_type_connect_type_names_in_graph_from(
                        type_graph_node_by_name,
                        argument,
                        type_graph,
                    );
                }
            }
        }
        SyntaxType::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_type_connect_type_names_in_graph_from(
                    type_graph_node_by_name,
                    part,
                    type_graph,
                );
            }
        }
        SyntaxType::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_type_connect_type_names_in_graph_from(
                        type_graph_node_by_name,
                        value,
                        type_graph,
                    );
                }
            }
        }
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => {
            for variant in variants {
                if let Some(value) = &variant.value {
                    syntax_type_connect_type_names_in_graph_from(
                        type_graph_node_by_name,
                        value,
                        type_graph,
                    );
                }
            }
        }
    }
}
// TODO collect variables and exclude them
fn syntax_expression_connect_fns_in_graph_from<Expressions, Patterns>(
    project_fn_graph_node_by_name: &std::collections::HashMap<
        &Name,
        strongly_connected_components::Vertex,
    >,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    expression: &SyntaxExpression<Expressions, Patterns>,
    project_fn_graph: &mut std::collections::BTreeSet<strongly_connected_components::Vertex>,
) {
    match expression {
        SyntaxExpression::Number { .. } => {}
        SyntaxExpression::Text { .. } => {}
        SyntaxExpression::Variable(name) => {
            if let Some(referenced_fn_graph_node) =
                project_fn_graph_node_by_name.get(&name.value).copied()
            {
                project_fn_graph.insert(referenced_fn_graph_node);
            }
        }
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments: _,
            open_paren_start: _,
            arguments_rest: arguments,
            closed_paren_start: _,
        } => {
            if let Some(referenced_fn_graph_node) =
                project_fn_graph_node_by_name.get(&name.value).copied()
            {
                project_fn_graph.insert(referenced_fn_graph_node);
            }
            if let Some(SyntaxExpressionArgumentDot {
                argument,
                dot_start: _,
            }) = first_argument_dot
            {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.item(argument),
                    project_fn_graph,
                );
            }
            for argument in arguments {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    argument,
                    project_fn_graph,
                );
            }
        }
        SyntaxExpression::Variant {
            open_bracket_start: _,
            name: _,
            type_: _,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(value) = value {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.item(value),
                    project_fn_graph,
                );
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters: _,
            closed_paren_start: _,
            result,
        } => {
            if let Some(result) = result {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.item(result),
                    project_fn_graph,
                );
            }
        }
        SyntaxExpression::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    part,
                    project_fn_graph,
                );
            }
        }
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_expression_connect_fns_in_graph_from(
                        project_fn_graph_node_by_name,
                        expressions,
                        value,
                        project_fn_graph,
                    );
                }
            }
        }
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => {
            if let Some(expression) = expression {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.item(expression),
                    project_fn_graph,
                );
            }
        }
        SyntaxExpression::Destruct {
            equals_start: _,
            pattern: _,
            queried,
            result,
        } => {
            if let Some(queried) = queried {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.item(queried),
                    project_fn_graph,
                );
            }
            if let Some(result) = result {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.item(result),
                    project_fn_graph,
                );
            }
        }
        SyntaxExpression::If {
            if_keyword_start: _,
            queried,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => {
            if let Some(queried) = queried {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.item(queried),
                    project_fn_graph,
                );
            }
            for case in cases {
                if let Some(result) = &case.result {
                    syntax_expression_connect_fns_in_graph_from(
                        project_fn_graph_node_by_name,
                        expressions,
                        result,
                        project_fn_graph,
                    );
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct SyntaxProjectFnInfo<'a, Expressions, Patterns> {
    pub range: lsp_types::Range,
    pub name: &'a WithStartPosition<Name>,
    pub type_parameters: &'a Option<SyntaxAngledTypeParameters>,
    pub parameters: &'a Vec<SyntaxPattern<Patterns>>,
    pub result_type: &'a Option<SyntaxType>,
    pub documentation: &'a Option<SyntaxComments>,
    pub result: &'a Option<SyntaxExpression<Expressions, Patterns>>,
}
#[derive(Debug)]
pub struct SyntaxProjectTypeInfo<'a> {
    // consider introducing separate structs instead of separately referencing each field
    pub name: &'a WithStartPosition<Name>,
    pub documentation: &'a Option<SyntaxComments>,
    pub parameters: &'a Option<SyntaxTyParameters>,
    pub type_: &'a Option<SyntaxType>,
}
// Copy & Clone need to be manually implemented because derive(Clone) introduces unnecessary Expressions/Patterns/Types:Clone bounds
impl<'a, Expressions, Patterns> Copy for SyntaxProjectFnInfo<'a, Expressions, Patterns> {}
impl<'a, Expressions, Patterns> Clone for SyntaxProjectFnInfo<'a, Expressions, Patterns> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a> Copy for SyntaxProjectTypeInfo<'a> {}
impl<'a> Clone for SyntaxProjectTypeInfo<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

fn project_type_alias_check(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,

    project_type: SyntaxProjectTypeInfo,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> CheckedTypeAlias {
    let documentation = project_type.documentation.as_ref().map(|documentation| {
        documentation
            .line1_up
            .iter()
            .fold(documentation.line0.value.to_string(), |so_far, line| {
                so_far + "\n" + &line.value
            })
            .into_boxed_str()
    });
    match &project_type.type_ {
        None => {
            errors.push(ErrorNode {
                range: name_range(with_start_position_as_ref(project_type.name)),
                message: Box::from(
                    "missing type after ty ..type-name.. ..potential parameters.. here",
                ),
            });
            CheckedTypeAlias {
                name_range: Some(name_range(with_start_position_as_ref(project_type.name))),
                documentation: documentation,
                parameters: project_type
                    .parameters
                    .iter()
                    .flat_map(|parameters| {
                        parameters.parameters.iter().map(|name| name.value.clone())
                    })
                    .collect::<Vec<_>>(),
                type_: None,
            }
        }
        Some(aliased_syntax_type) => {
            match syntax_type_check(
                aliased_syntax_type,
                errors,
                type_aliases,
                records_used,
                choices_used,
            ) {
                None => CheckedTypeAlias {
                    name_range: Some(name_range(with_start_position_as_ref(project_type.name))),
                    documentation: documentation,
                    parameters: project_type
                        .parameters
                        .iter()
                        .flat_map(|parameters| {
                            parameters.parameters.iter().map(|name| name.value.clone())
                        })
                        .collect::<Vec<_>>(),
                    type_: None,
                },
                Some(aliased_type) => {
                    let mut actually_used_type_variables: std::collections::BTreeSet<&Name> =
                        std::collections::BTreeSet::new();
                    type_variables_into(&mut actually_used_type_variables, &aliased_type);
                    let parameters = if !actually_used_type_variables.is_empty()
                        && project_type
                            .name
                            .value
                            .starts_with(|start: char| start.is_ascii_lowercase())
                    {
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(project_type.name)),
                            message: Box::from(
                                "When declaring a type with parameters, its name needs to start with an uppercase letter. Only concrete types (e.g. u32) have a lowercase name",
                            ),
                        });
                        vec![]
                    } else if actually_used_type_variables.is_empty()
                        && project_type
                            .name
                            .value
                            .starts_with(|start: char| start.is_ascii_uppercase())
                    {
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(project_type.name)),
                            message: Box::from(
                                "When declaring a type without parameters, its name needs to start with a lowercase letter. Only generic types (e.g. buf(Origin _item)) have an uppercase name",
                            ),
                        });
                        vec![]
                    } else {
                        parameters_check_if_different_to_actual_type_parameters(
                            errors,
                            name_range(with_start_position_as_ref(project_type.name)),
                            project_type.parameters.iter().flat_map(|parameters| {
                                parameters.parameters.iter().map(with_start_position_as_ref)
                            }),
                            actually_used_type_variables,
                            "Type parameter variables follow after the name and are separated by commas, so for example ty buf(Origin _item)",
                        )
                    };
                    CheckedTypeAlias {
                        name_range: Some(name_range(with_start_position_as_ref(project_type.name))),
                        documentation: documentation,
                        parameters: parameters,
                        type_: Some(aliased_type),
                    }
                }
            }
        }
    }
}

fn syntax_project_fn_header_check<'a, Expressions, Patterns>(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    project_fn: SyntaxProjectFnInfo<'a, Expressions, Patterns>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
    checked_spread_records: &mut std::collections::HashMap<lsp_types::Position, Vec<Name>>,
) -> CheckedProjectFn {
    let parameter_types = project_fn
        .parameters
        .iter()
        .map(|parameter| {
            syntax_parameter_pattern_check(
                parameter,
                errors,
                &mut std::collections::HashMap::new(),
                type_aliases,
                patterns,
                &std::collections::HashMap::new(),
                checked_spread_records,
                records_used,
                choices_used,
            )
        })
        .collect::<Vec<_>>();
    let result_type: Option<Type> = {
        match &project_fn.result_type {
            Some(syntax_result_type) => syntax_type_check(
                syntax_result_type,
                errors,
                type_aliases,
                records_used,
                choices_used,
            ),
            None => {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(project_fn.name)),
                    message: Box::from(
                        "missing function result type. An example of a full function is fn my-function() str  \":)\" str, where str is the result type",
                    ),
                });
                None
            }
        }
    };
    match result_type {
        Some(result_type) => {
            let mut type_variables_exclusively_used_in_result =
                std::collections::BTreeSet::<&Name>::new();
            type_variables_into(&mut type_variables_exclusively_used_in_result, &result_type);
            // can be optimized
            let mut parameter_type_variables = std::collections::BTreeSet::<&Name>::new();
            for parameter_type in &parameter_types {
                if let Some(parameter_type) = parameter_type {
                    type_variables_into(&mut parameter_type_variables, parameter_type);
                }
            }
            type_variables_exclusively_used_in_result
                .retain(|var| !parameter_type_variables.contains(var));
            let actually_used_parameters = parameters_check_if_different_to_actual_type_parameters(
                errors,
                name_range(with_start_position_as_ref(project_fn.name)),
                project_fn
                    .type_parameters
                    .iter()
                    .flat_map(|type_parameters| {
                        type_parameters
                            .names
                            .iter()
                            .map(|parameter| with_start_position_as_ref(parameter))
                    }),
                type_variables_exclusively_used_in_result,
                "Type parameter variables follow after the name and are each enclosed in curly braces, so for example fn No-twice{_a}{_b}",
            );
            CheckedProjectFn {
                documentation: None,
                type_parameters: actually_used_parameters,
                parameter_types,
                result_type: Some(result_type),
                result_expression_is_invalid: false,
            }
        }
        None => CheckedProjectFn {
            documentation: None,
            type_parameters: project_fn
                .type_parameters
                .iter()
                .flat_map(|parameter| parameter.names.iter().map(|name| name.value.clone()))
                .collect(),
            parameter_types,
            result_type: result_type,
            result_expression_is_invalid: false,
        },
    }
}
fn syntax_project_fn_check<'a, Expressions, Patterns>(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    project_fn: SyntaxProjectFnInfo<'a, Expressions, Patterns>,
    checked_calls: &mut std::collections::HashMap<lsp_types::Position, CheckedCall>,
    checked_local_fns: &mut std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &mut std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &mut std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> CheckedProjectFn {
    let checked_header = project_fns
        .get(&project_fn.name.value)
        .cloned()
        .unwrap_or_else(|| {
            syntax_project_fn_header_check(
                errors,
                type_aliases,
                patterns,
                project_fn,
                records_used,
                choices_used,
                checked_spread_records,
            )
        });
    let documentation = project_fn.documentation.as_ref().map(|documentation| {
        documentation
            .line1_up
            .iter()
            .fold(documentation.line0.value.to_string(), |so_far, line| {
                so_far + "\n" + &line.value
            })
            .into_boxed_str()
    });
    let Some(header_result_type) = checked_header.result_type else {
        // rust top level declarations need explicit types; partial types won't do
        return CheckedProjectFn {
            documentation: documentation,
            type_parameters: checked_header.type_parameters,
            parameter_types: checked_header.parameter_types,
            result_type: None,
            result_expression_is_invalid: true,
        };
    };
    let Some(syntax_result) = project_fn.result else {
        errors.push(ErrorNode {
            range: name_range(with_start_position_as_ref(project_fn.name)),
            message: Box::from(
                "missing expression after the fn result type. An example would be fn My-function . : str = \":)\", where . is an empty record as the parameter",
            ),
        });
        return CheckedProjectFn {
            documentation: documentation,
            type_parameters: checked_header.type_parameters,
            parameter_types: checked_header.parameter_types,
            result_type: Some(header_result_type),
            result_expression_is_invalid: true,
        };
    };
    let mut parameter_introduced_variables = std::collections::HashMap::new();
    for syntax_parameter in project_fn.parameters {
        syntax_parameter_pattern_variables_fold(
            syntax_parameter,
            (),
            &mut |(), name, type_| {
                parameter_introduced_variables.insert(
                    name.value,
                    CheckedLocalVariable {
                        origin_start: name.start,
                        type_: type_.and_then(|type_| syntax_type_to_type(type_, type_aliases)),
                    },
                );
            },
            patterns,
        );
    }
    let Some(checked_result_expression_type) = syntax_expression_check(
        errors,
        type_aliases,
        project_fns,
        expressions,
        patterns,
        &mut parameter_introduced_variables,
        syntax_result,
        checked_calls,
        checked_local_fns,
        checked_queries,
        checked_spread_records,
        records_used,
        choices_used,
    ) else {
        return CheckedProjectFn {
            documentation: documentation,
            type_parameters: checked_header.type_parameters,
            parameter_types: checked_header.parameter_types,
            result_type: Some(header_result_type),
            result_expression_is_invalid: true,
        };
    };
    for (parameter_introduced_variable_name, parameter_introduced_variable_origin) in
        parameter_introduced_variables
    {
        errors.push(error_introduced_pattern_variable_is_unused(
            parameter_introduced_variable_origin.origin_start,
            parameter_introduced_variable_name,
        ));
    }
    if let Some(result_type_diff) = type_diff(&header_result_type, &checked_result_expression_type)
    {
        place_expression_type_diff_errors(
            errors,
            syntax_result,
            &result_type_diff,
            expressions,
            patterns,
        );
        return CheckedProjectFn {
            documentation: documentation,
            type_parameters: checked_header.type_parameters,
            parameter_types: checked_header.parameter_types,
            result_type: Some(header_result_type),
            result_expression_is_invalid: true,
        };
    }
    CheckedProjectFn {
        documentation: documentation,
        type_parameters: checked_header.type_parameters,
        parameter_types: checked_header.parameter_types,
        result_type: Some(header_result_type),
        result_expression_is_invalid: false,
    }
}
/// only use if you know `syntax_type_check` has already been called on it before
pub fn syntax_type_to_type(
    type_: &SyntaxType,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) -> Option<Type> {
    match type_ {
        SyntaxType::Variable(name) => Some(Type::Variable(name.value.clone())),
        SyntaxType::Origin { hat_start: _, name } => {
            name.as_ref().map(|name| Type::Variable(name.value.clone()))
        }
        SyntaxType::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            let Some(result) = result else {
                return None;
            };
            let Some(result) = syntax_type_to_type(result, type_aliases) else {
                return None;
            };
            Some(Type::Fn {
                parameters: parameters
                    .iter()
                    .filter_map(|parameter| syntax_type_to_type(parameter, type_aliases))
                    .collect(),
                result: Box::new(result),
            })
        }
        SyntaxType::ConstructWithArguments { name, arguments } => {
            if let Some(origin_type_alias) = type_aliases.get(&name.value) {
                if let Some(SyntaxTypeConstructParenthesizedArguments {
                    open_paren_start: _,
                    arguments,
                    closed_paren_start: _,
                }) = arguments
                    && !arguments.is_empty()
                {
                    let argument_types = arguments
                        .iter()
                        .map(|argument_type| syntax_type_to_type(argument_type, type_aliases))
                        .collect::<Option<Vec<Type>>>()?;
                    type_construct_resolve_type_alias(origin_type_alias, argument_types)
                } else {
                    origin_type_alias.type_.clone()
                }
            } else {
                None
            }
        }
        SyntaxType::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => Some(Type::Tuple(
            parts
                .iter()
                .filter_map(|part| syntax_type_to_type(part, type_aliases))
                .collect(),
        )),
        SyntaxType::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            let mut field_types: Vec<TypeField> = Vec::with_capacity(fields.len());
            for field in fields {
                let Some(field_value) = &field.value else {
                    return None;
                };
                match syntax_type_to_type(field_value, type_aliases) {
                    None => {}
                    Some(field_value_type) => {
                        field_types.push(TypeField {
                            name: field.name.value.clone(),
                            value: field_value_type,
                        });
                    }
                }
            }
            Some(Type::Record(field_types))
        }
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => {
            let mut variant_types: Vec<TypeVariant> = Vec::with_capacity(variants.len());
            for syntax_variant in variants {
                let Some(syntax_variant_value) = &syntax_variant.value else {
                    return None;
                };
                match syntax_type_to_type(syntax_variant_value, type_aliases) {
                    None => {}
                    Some(variant_value_type) => {
                        variant_types.push(TypeVariant {
                            name: syntax_variant.name.value.clone(),
                            value: variant_value_type,
                        });
                    }
                }
            }
            Some(Type::Choice(variant_types))
        }
    }
}
pub fn syntax_type_check(
    type_: &SyntaxType,
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> Option<Type> {
    match type_ {
        SyntaxType::Variable(name) => Some(Type::Variable(name.value.clone())),
        SyntaxType::Origin { hat_start, name } => {
            let Some(name) = name else {
                errors.push(ErrorNode {
                    range: symbol_range(*hat_start, "^"),
                    message: Box::from("missing origin name after this ^. Origin names must start with a lowercase a-z")
                });
                return None;
            };
            Some(Type::Origin(name.value.clone()))
        }
        SyntaxType::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            let Some(result) = result else {
                return None;
            };
            let Some(result) =
                syntax_type_check(result, errors, type_aliases, records_used, choices_used)
            else {
                return None;
            };
            let Some(parameters) = parameters
                .iter()
                .map(|parameter| {
                    syntax_type_check(parameter, errors, type_aliases, records_used, choices_used)
                })
                .collect::<Option<Vec<_>>>()
            else {
                return None;
            };
            Some(Type::Fn {
                parameters,
                result: Box::new(result),
            })
        }
        SyntaxType::ConstructWithArguments { name, arguments } => {
            let Some(origin_type_alias) = type_aliases.get(&name.value) else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from("no type alias or origin exists with this name"),
                });
                return None;
            };
            let Some(argument_types) = arguments
                .iter()
                .flat_map(|arguments| &arguments.arguments)
                .map(|argument_type| {
                    syntax_type_check(
                        argument_type,
                        errors,
                        type_aliases,
                        records_used,
                        choices_used,
                    )
                })
                .collect::<Option<Vec<Type>>>()
            else {
                return None;
            };
            let argument_count = argument_types.len();
            match origin_type_alias.parameters.len().cmp(&argument_count) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Less => {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: format!(
                            "this type alias has {} less parameters than arguments are provided here.",
                            argument_count - origin_type_alias.parameters.len(),
                        ).into_boxed_str()
                    });
                    return None;
                }
                std::cmp::Ordering::Greater => {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: format!(
                            "this type alias has {} more parameters than arguments are provided here. The additional {} called {}",
                            origin_type_alias.parameters.len() - argument_count,
                            if origin_type_alias.parameters.len() - argument_count == 1 {
                                "parameter is"
                            } else {
                                "parameters are"
                            },
                            origin_type_alias.parameters.iter().map(|parameter| parameter.as_str()).skip(argument_count).collect::<Vec<_>>().join(", ")
                        ).into_boxed_str()
                    });
                    return None;
                }
            }
            type_construct_resolve_type_alias(origin_type_alias, argument_types)
        }
        SyntaxType::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => parts
            .iter()
            .map(|part| syntax_type_check(part, errors, type_aliases, records_used, choices_used))
            .collect::<Option<Vec<_>>>()
            .map(Type::Tuple),
        SyntaxType::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            let mut field_types: Vec<TypeField> = Vec::with_capacity(fields.len());
            let mut any_field_value_has_error: bool = false;
            for field in fields {
                if field_types
                    .iter()
                    .any(|type_field| type_field.name == field.name.value)
                {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(&field.name)),
                        message: Box::from(
                            "a field with this name already exists in the record type",
                        ),
                    });
                    return None;
                }
                let Some(field_value) = &field.value else {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(&field.name)),
                        message: Box::from(
                            "missing field value after this field name .field-name here",
                        ),
                    });
                    return None;
                };
                match syntax_type_check(
                    field_value,
                    errors,
                    type_aliases,
                    records_used,
                    choices_used,
                ) {
                    None => {
                        any_field_value_has_error = true;
                    }
                    Some(field_value_type) => {
                        field_types.push(TypeField {
                            name: field.name.value.clone(),
                            value: field_value_type,
                        });
                    }
                }
            }
            records_used.insert(sorted_field_names(
                field_types.iter().map(|field_type| &field_type.name),
            ));
            if any_field_value_has_error {
                return None;
            }
            Some(Type::Record(field_types))
        }
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => {
            let mut variant_types: Vec<TypeVariant> = Vec::with_capacity(variants.len());
            let mut any_variant_value_has_error: bool = false;
            for syntax_variant in variants {
                if variant_types
                    .iter()
                    .any(|type_variant| type_variant.name == syntax_variant.name.value)
                {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(&syntax_variant.name)),
                        message: Box::from(
                            "a variant with this name already exists in the choice type",
                        ),
                    });
                    return None;
                }
                let Some(syntax_variant_value) = &syntax_variant.value else {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(&syntax_variant.name)),
                        message: Box::from(
                            "missing variant value after this name ..variant-name.. here. Every variant has a value, even if just .",
                        ),
                    });
                    return None;
                };
                match syntax_type_check(
                    syntax_variant_value,
                    errors,
                    type_aliases,
                    records_used,
                    choices_used,
                ) {
                    None => {
                        any_variant_value_has_error = true;
                    }
                    Some(variant_value_type) => {
                        variant_types.push(TypeVariant {
                            name: syntax_variant.name.value.clone(),
                            value: variant_value_type,
                        });
                    }
                }
            }
            choices_used.insert(sorted_variant_names(
                variant_types.iter().map(|variant_type| &variant_type.name),
            ));
            if any_variant_value_has_error {
                return None;
            }
            Some(Type::Choice(variant_types))
        }
    }
}
fn type_construct_resolve_type_alias(
    origin_type_alias: &CheckedTypeAlias,
    argument_types: Vec<Type>,
) -> Option<Type> {
    let Some(type_alias_type) = &origin_type_alias.type_ else {
        return None;
    };
    if origin_type_alias.parameters.is_empty() {
        return Some(type_alias_type.clone());
    }
    let type_parameter_replacements: std::collections::BTreeMap<Name, Type> = origin_type_alias
        .parameters
        .iter()
        .cloned()
        .zip(argument_types)
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut peeled: Type = type_alias_type.clone();
    type_replace_variables(&type_parameter_replacements, &mut peeled);
    Some(peeled)
}
fn type_replace_variables(
    type_parameter_replacements: &std::collections::BTreeMap<Name, Type>,
    type_: &mut Type,
) {
    match type_ {
        Type::Variable(variable) => {
            if let Some(replacement_type_node) = type_parameter_replacements.get(variable) {
                *type_ = replacement_type_node.clone();
            }
        }
        Type::Origin(_) => {}
        Type::CoreConstruct { name: _, arguments } => {
            for argument_type in arguments {
                type_replace_variables(type_parameter_replacements, argument_type);
            }
        }
        Type::Fn { parameters, result } => {
            for parameter in parameters {
                type_replace_variables(type_parameter_replacements, parameter);
            }
            type_replace_variables(type_parameter_replacements, result.as_mut());
        }
        Type::Tuple(parts) => {
            for part in parts {
                type_replace_variables(type_parameter_replacements, part);
            }
        }
        Type::Record(fields) => {
            for field in fields {
                type_replace_variables(type_parameter_replacements, &mut field.value);
            }
        }
        Type::Choice(variants) => {
            for variant in variants {
                type_replace_variables(type_parameter_replacements, &mut variant.value);
            }
        }
    }
}

fn type_variables_into<'a>(
    type_variables: &mut std::collections::BTreeSet<&'a Name>,
    type_: &'a Type,
) {
    match type_ {
        Type::Variable(name) => {
            type_variables.insert(name);
        }
        Type::Origin(_) => {}
        Type::CoreConstruct { name: _, arguments } => {
            for argument in arguments {
                type_variables_into(type_variables, argument);
            }
        }
        Type::Tuple(parts) => {
            for part in parts {
                type_variables_into(type_variables, part);
            }
        }
        Type::Record(fields) => {
            for field in fields {
                type_variables_into(type_variables, &field.value);
            }
        }
        Type::Choice(variants) => {
            for variant in variants {
                type_variables_into(type_variables, &variant.value);
            }
        }
        Type::Fn { parameters, result } => {
            for parameter in parameters {
                type_variables_into(type_variables, parameter);
            }
            type_variables_into(type_variables, result);
        }
    }
}
fn parameters_check_if_different_to_actual_type_parameters<'a>(
    errors: &mut Vec<ErrorNode>,
    origin_name_range: lsp_types::Range,
    parameters: impl Iterator<Item = WithStartPosition<&'a Name>>,
    mut actually_used_type_variables: std::collections::BTreeSet<&Name>,
    syntax_help: &'static str,
) -> Vec<Name> {
    let mut actually_used_parameters = Vec::<Name>::with_capacity(parameters.size_hint().0);
    for parameter in parameters {
        if actually_used_type_variables.remove(parameter.value) {
            actually_used_parameters.push(parameter.value.clone());
        } else {
            errors.push(ErrorNode {
                range: name_range(parameter),
                message: Box::from("this type parameter is not used. Remove it or use it"),
            });
        }
    }
    if !actually_used_type_variables.is_empty() {
        errors.push(ErrorNode {
            range: origin_name_range,
            message: format!(
                "some type parameters are used but not declared, namely {}. Add {}.
{}",
                actually_used_type_variables
                    .iter()
                    .copied()
                    .map(Name::as_str)
                    .collect::<Vec<&str>>()
                    .join(", "),
                if actually_used_type_variables.len() >= 2 {
                    "them"
                } else {
                    "it"
                },
                syntax_help
            )
            .into_boxed_str(),
        });
    }
    actually_used_parameters
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum SpecificPatternCatch<'a> {
    OnlyMatchableByVariable,
    Variant {
        name: &'a Name,
        value: Box<SpecificPatternCatch<'a>>,
    },
    Tuple(Vec<SpecificPatternCatch<'a>>),
    Record(Vec<(&'a Name, SpecificPatternCatch<'a>)>),
}
/// careful. This easily goes exponential
fn type_to_possible_specific_pattern_catches<'a>(
    type_: &'a Type,
    possibilities: &mut Vec<SpecificPatternCatch<'a>>,
) {
    // possible optimizations:
    // - calculate the count of possibilities in advance and pre-allocate
    // - add SpecificPatternCatch::Unexpanded(&'a Type) which expands only on encounter.
    //   The more future-proof solution
    match type_ {
        Type::Variable(_) | Type::Origin(_) | Type::CoreConstruct { .. } | Type::Fn { .. } => {
            possibilities.push(SpecificPatternCatch::OnlyMatchableByVariable);
        }
        Type::Tuple(type_parts) => {
            if type_parts.is_empty() {
                possibilities.push(SpecificPatternCatch::Tuple(vec![]));
            } else {
                // can probably be optimized
                let mut parts_possibilities = vec![Vec::new()];
                for type_part in type_parts {
                    let mut part_possibilities = Vec::new();
                    type_to_possible_specific_pattern_catches(type_part, &mut part_possibilities);
                    parts_possibilities = parts_possibilities
                        .into_iter()
                        .flat_map(|parts_possibility| {
                            part_possibilities
                                .iter()
                                .cloned()
                                .zip(std::iter::repeat_n(
                                    parts_possibility,
                                    part_possibilities.len(),
                                ))
                                .map(|(value_possibility, mut parts_possibility)| {
                                    parts_possibility.push(value_possibility);
                                    parts_possibility
                                })
                        })
                        .collect();
                }
                possibilities.extend(
                    parts_possibilities
                        .into_iter()
                        .map(SpecificPatternCatch::Tuple),
                );
            }
        }
        Type::Record(type_fields) => {
            if type_fields.is_empty() {
                possibilities.push(SpecificPatternCatch::Record(vec![]));
            } else {
                // can probably be optimized
                let mut fields_possibilities = vec![Vec::new()];
                for type_field in type_fields {
                    let mut value_possibilities = Vec::new();
                    type_to_possible_specific_pattern_catches(
                        &type_field.value,
                        &mut value_possibilities,
                    );
                    fields_possibilities = fields_possibilities
                        .into_iter()
                        .flat_map(|fields_possibility| {
                            value_possibilities
                                .iter()
                                .cloned()
                                .zip(std::iter::repeat_n(
                                    fields_possibility,
                                    value_possibilities.len(),
                                ))
                                .map(|(value_possibility, mut fields_possibility)| {
                                    fields_possibility.push((&type_field.name, value_possibility));
                                    fields_possibility
                                })
                        })
                        .collect();
                }
                possibilities.extend(
                    fields_possibilities
                        .into_iter()
                        .map(SpecificPatternCatch::Record),
                );
            }
        }
        Type::Choice(type_variants) => {
            for type_variant in type_variants {
                let mut value_possibilities = Vec::new();
                type_to_possible_specific_pattern_catches(
                    &type_variant.value,
                    &mut value_possibilities,
                );
                possibilities.extend(value_possibilities.into_iter().map(|value_catch| {
                    SpecificPatternCatch::Variant {
                        name: &type_variant.name,
                        value: Box::new(value_catch),
                    }
                }));
            }
        }
    }
}
fn specific_pattern_catch_format(
    specific_pattern_catch: &SpecificPatternCatch,
    output: &mut String,
) {
    match specific_pattern_catch {
        SpecificPatternCatch::OnlyMatchableByVariable => {
            output.push_str("(some-variable)");
        }
        SpecificPatternCatch::Variant { name, value } => {
            output.push('[');
            output.push_str(name);
            output.push(' ');
            specific_pattern_catch_format(value, output);
            output.push(']');
        }
        SpecificPatternCatch::Tuple(parts) => {
            output.push('*');
            if let Some((part0, part1_up)) = parts.split_first() {
                output.push('(');
                specific_pattern_catch_format(part0, output);
                for part in part1_up {
                    output.push(' ');
                    specific_pattern_catch_format(part, output);
                }
                output.push(')');
            }
        }
        SpecificPatternCatch::Record(fields) => {
            output.push('{');
            if let Some(((field0_name, field0_value), field1_up)) = fields.split_first() {
                output.push_str(field0_name);
                output.push(' ');
                specific_pattern_catch_format(field0_value, output);
                for (name, value) in field1_up {
                    output.push(' ');
                    output.push_str(name);
                    output.push(' ');
                    specific_pattern_catch_format(value, output);
                }
            }
            output.push('}');
        }
    }
}
fn specific_pattern_catches_remove_those_matched_by_pattern<Patterns>(
    possibilities: &mut Vec<SpecificPatternCatch>,
    pattern: &SyntaxPattern<Patterns>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
) {
    // we do not use ::retain because one pattern usually only covers one specific pattern catch,
    // and calling retain for every pattern would thus be quadratic.

    // dear rust, add a version of Vec::retain which swap_removes
    let mut possibility_index = 0;
    loop {
        let Some(possibility) = possibilities.get(possibility_index) else {
            return;
        };
        if pattern_matches_specific_pattern_catch(
            pattern,
            possibility,
            patterns,
            checked_spread_records,
        ) {
            possibilities.swap_remove(possibility_index);
        } else {
            possibility_index += 1;
        }
    }
}
fn pattern_matches_specific_pattern_catch<Patterns>(
    pattern: &SyntaxPattern<Patterns>,
    possibility: &SpecificPatternCatch,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
) -> bool {
    match pattern {
        SyntaxPattern::Variable { .. } => true,
        SyntaxPattern::Ignored { .. } => true,
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => match possibility {
            SpecificPatternCatch::Variant {
                name: possibility_name,
                value: possibility_value,
            } if name
                .as_ref()
                .is_some_and(|name| &name.value == possibility_name) =>
            {
                match value {
                    None => true,
                    Some(value) => pattern_matches_specific_pattern_catch(
                        patterns.item(value),
                        possibility_value,
                        patterns,
                        checked_spread_records,
                    ),
                }
            }
            _ => false,
        },
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => match possibility {
            SpecificPatternCatch::Tuple(possibility_parts) => parts
                .iter()
                .zip(possibility_parts)
                .all(|(part, possibility_part)| {
                    pattern_matches_specific_pattern_catch(
                        part,
                        possibility_part,
                        patterns,
                        checked_spread_records,
                    )
                }),
            _ => false,
        },
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => match possibility {
            SpecificPatternCatch::Record(possibility_fields) => {
                fields.iter().all(|field| match &field.value {
                    None => true,
                    Some(value) => {
                        match possibility_fields
                            .iter()
                            .find(|(possibility_name, _)| *possibility_name == &field.name.value)
                        {
                            None => true,
                            Some((_, value_possibility)) => pattern_matches_specific_pattern_catch(
                                value,
                                value_possibility,
                                patterns,
                                checked_spread_records,
                            ),
                        }
                    }
                })
            }
            _ => false,
        },
    }
}

fn syntax_if_case_pattern_check<'a, Patterns>(
    pattern: &'a SyntaxPattern<Patterns>,
    expected_type: &Type,
    errors: &mut Vec<ErrorNode>,
    introduced_variables: &mut std::collections::HashMap<&'a Name, CheckedLocalVariable>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
    existing_local_variables: &std::collections::HashMap<&Name, CheckedLocalVariable>,
    checked_spread_records: &mut std::collections::HashMap<lsp_types::Position, Vec<Name>>,
) {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            if let Some(actual_type) = type_.as_ref() {
                errors.push(ErrorNode {
                    range: type_range(actual_type),
                    message: Box::from(
                        "if pattern variables cannot specify an explicit type. Remove it",
                    ),
                });
            }
            let maybe_existing_variable_with_the_same_name = introduced_variables.insert(
                &name.value,
                CheckedLocalVariable {
                    origin_start: name.start,
                    type_: Some(expected_type.clone()),
                },
            );
            if let Some(_existing_variable_with_the_same_name) =
                maybe_existing_variable_with_the_same_name
            {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from(
                        "a pattern variable with this name already exists in the surrounding pattern. Rename either variable",
                    ),
                });
            } else if let Some(existing_variable_with_the_same_name) =
                existing_local_variables.get(&name.value)
            {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: format!(
                        "a local variable with this name already exists (intruduced at {}). Rename either variable",
                        position_to_string(existing_variable_with_the_same_name.origin_start)
                    ).into_boxed_str(),
                });
            }
        }
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_,
        } => {
            if let Some(actual_type) = type_.as_ref() {
                errors.push(ErrorNode {
                    range: type_range(actual_type),
                    message: Box::from(
                        "ignored if patterns cannot specify an explicit type. Remove it",
                    ),
                });
            }
        }
        SyntaxPattern::Variant {
            open_bracket_start,
            name,
            value,
            closed_bracket_start: _,
        } => {
            let Some(name) = name else {
                errors.push(ErrorNode {
                    range: symbol_range(*open_bracket_start, "["),
                    message: Box::from("missing variant name after this open bracket [. An example of a variant pattern is [yes your-variable]")
                });
                return;
            };
            let Type::Choice(origin_choice_type_variants) = &expected_type else {
                let mut error_message: String = String::from(
                    "A variant is part of a choice type (for example |a u32 |b str) but the expected type here is\n",
                );
                type_format(&mut error_message, 0, expected_type);
                error_message.push_str("\nYou might have intended this pattern to belong to a different if. Use parens for the if case results of ifs with multiple cases");
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: error_message.into_boxed_str(),
                });
                return;
            };
            let Some(expected_value_type) =
                origin_choice_type_variants.iter().find_map(|variant| {
                    if variant.name == name.value {
                        Some(&variant.value)
                    } else {
                        None
                    }
                })
            else {
                let mut error_message: String = format!(
                    "this variant name {} is not included in it's expected type\n",
                    name.value
                );
                type_format(&mut error_message, 0, expected_type);
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: error_message.into_boxed_str(),
                });
                return;
            };
            let Some(value) = value else {
                let mut error_message: String =
                    String::from("this variant is missing its associated value of type\n");
                type_format(&mut error_message, 0, expected_value_type);
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: error_message.into_boxed_str(),
                });
                return;
            };
            let value = patterns.item(value);
            syntax_if_case_pattern_check(
                value,
                expected_value_type,
                errors,
                introduced_variables,
                patterns,
                existing_local_variables,
                checked_spread_records,
            );
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            let Type::Tuple(expected_type_parts) = expected_type else {
                let mut error_message: String =
                    String::from("This pattern matches a tuple but the expected type here is\n");
                type_format(&mut error_message, 0, expected_type);
                error_message.push_str(
                    "\nMaybe you intended this pattern to belong to a different if case?",
                );
                errors.push(ErrorNode {
                    range: pattern_range(pattern, patterns),
                    message: error_message.into_boxed_str(),
                });
                return;
            };
            if parts.len() != expected_type_parts.len() {
                let mut error_message: String = format!(
                    "This pattern matches a tuple with {parts_len} but the expected type here needs {expected_type_parts_len}:\n",
                    parts_len = parts.len(),
                    expected_type_parts_len = expected_type_parts.len()
                );
                type_format(&mut error_message, 0, expected_type);
                error_message.push_str(
                    "\nMaybe you intended this pattern to belong to a different if case?",
                );
                errors.push(ErrorNode {
                    range: pattern_range(pattern, patterns),
                    message: error_message.into_boxed_str(),
                });
                return;
            }
            for (part, expected_type_part) in parts.iter().zip(expected_type_parts) {
                syntax_if_case_pattern_check(
                    part,
                    expected_type_part,
                    errors,
                    introduced_variables,
                    patterns,
                    existing_local_variables,
                    checked_spread_records,
                );
            }
        }
        SyntaxPattern::Record {
            open_curly_start,
            fields,
            closed_curly_start: _,
        } => {
            let Type::Record(expected_type_record_fields) = expected_type else {
                let mut error_message: String =
                    String::from("This pattern matches a record but the expected type here is\n");
                type_format(&mut error_message, 0, expected_type);
                error_message.push_str(
                    "\nMaybe you intended this pattern to belong to a different if case?",
                );
                errors.push(ErrorNode {
                    range: pattern_range(pattern, patterns),
                    message: error_message.into_boxed_str(),
                });
                return;
            };
            let mut remaining_expected_type_fields = expected_type_record_fields.clone();
            for field in fields {
                let Some(value) = &field.value else {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(&field.name)),
                        message: Box::from("missing field value after this field name"),
                    });
                    return;
                };
                let Some(expected_type_field) = remaining_expected_type_fields
                    .iter()
                    .position(|expected_field| expected_field.name == field.name.value)
                    .map(|p| remaining_expected_type_fields.swap_remove(p))
                else {
                    let error_message: String = format!(
                        "This pattern matches a record with the field .{} but the expected record type here only expects these fields
.{}
Are there duplicate fields?
Otherwise, you might have intended this pattern to belong to a different if. Use parens for the if case results of ifs with multiple cases",
                            field.name.value,
                            expected_type_record_fields.iter().map(|expected_type_field| expected_type_field.name.as_str()).collect::<Vec<_>>().join(" .")
                    );
                    errors.push(ErrorNode {
                        range: pattern_range(pattern, patterns),
                        message: error_message.into_boxed_str(),
                    });
                    return;
                };
                syntax_if_case_pattern_check(
                    value,
                    &expected_type_field.value,
                    errors,
                    introduced_variables,
                    patterns,
                    existing_local_variables,
                    checked_spread_records,
                );
            }
            if !remaining_expected_type_fields.is_empty() {
                let mut error_message = String::from("missing fields:\n");
                type_format(
                    &mut error_message,
                    0,
                    &Type::Record(remaining_expected_type_fields),
                );
                error_message.push_str("\nAdd them to the case pattern or use a spread like .. rest to catch the remaining fields");
                errors.push(ErrorNode {
                    range: symbol_range(*open_curly_start, "{"),
                    message: error_message.into_boxed_str(),
                });
            }
        }
    }
}
fn syntax_parameter_pattern_check<'a, Patterns>(
    pattern: &'a SyntaxPattern<Patterns>,
    errors: &mut Vec<ErrorNode>,
    introduced_variables: &mut std::collections::HashMap<&'a Name, CheckedLocalVariable>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,

    existing_local_variables: &std::collections::HashMap<&Name, CheckedLocalVariable>,
    checked_spread_records: &mut std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> Option<Type> {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            let maybe_checked_variable = match type_.as_ref() {
                None => {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: Box::from("function parameter pattern variables need an explicit type. Add one to this pattern variable by appending a type like in your-variable u32 (the whole thing in parens if necessary)"),
                    });
                    None
                }
                Some(actual_type) => {
                    let Some(actual_type) = syntax_type_check(
                        actual_type,
                        errors,
                        type_aliases,
                        records_used,
                        choices_used,
                    ) else {
                        return None;
                    };
                    Some(actual_type)
                }
            };
            let maybe_existing_variable_with_the_same_name = introduced_variables.insert(
                &name.value,
                CheckedLocalVariable {
                    origin_start: name.start,
                    type_: maybe_checked_variable.clone(),
                },
            );
            if let Some(_existing_variable_with_the_same_name) =
                maybe_existing_variable_with_the_same_name
            {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from(
                        "a pattern variable with this name already exists in the surrounding pattern. Rename either variable",
                    ),
                });
                return None;
            } else if let Some(existing_variable_with_the_same_name) =
                existing_local_variables.get(&name.value)
            {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: format!(
                        "a local variable with this name already exists (intruduced at {}). Rename either variable",
                        position_to_string(existing_variable_with_the_same_name.origin_start)
                    ).into_boxed_str(),
                });
                return None;
            }
            maybe_checked_variable
        }
        SyntaxPattern::Ignored {
            underscore_start,
            type_,
        } => match type_.as_ref() {
            None => {
                errors.push(ErrorNode {
                        range: symbol_range(*underscore_start, "_"),
                        message: Box::from("function parameter ignored patterns _ need an explicit type. Add one to this pattern variable by appending a type like in your-variable u32"),
                    });
                None
            }
            Some(actual_type) => {
                let Some(actual_type) = syntax_type_check(
                    actual_type,
                    errors,
                    type_aliases,
                    records_used,
                    choices_used,
                ) else {
                    return None;
                };
                Some(actual_type)
            }
        },
        SyntaxPattern::Variant {
            open_bracket_start,
            name,
            value,
            closed_bracket_start: _,
        } => {
            let Some(name) = name else {
                errors.push(ErrorNode {
                    range: symbol_range(*open_bracket_start, "["),
                    message: Box::from("missing variant name after this open bracket [. An example of a variant pattern is [yes your-variable]")
                });
                return None;
            };
            let Some(value) = value else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from("missing variant value after this variant name. Each variants has a value, even if just ., an example of a variant pattern is [yes your-variable]")
                });
                return None;
            };
            let Some(checked_value_type) = syntax_parameter_pattern_check(
                patterns.item(value),
                errors,
                introduced_variables,
                type_aliases,
                patterns,
                existing_local_variables,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            Some(Type::Choice(vec![TypeVariant {
                name: name.value.clone(),
                value: checked_value_type,
            }]))
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => parts
            .iter()
            .map(|part| {
                syntax_parameter_pattern_check(
                    part,
                    errors,
                    introduced_variables,
                    type_aliases,
                    patterns,
                    existing_local_variables,
                    checked_spread_records,
                    records_used,
                    choices_used,
                )
            })
            .collect::<Option<Vec<_>>>()
            .map(Type::Tuple),
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            let mut type_fields: Vec<TypeField> = Vec::with_capacity(fields.len());
            for field in fields {
                let Some(value) = &field.value else {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(&field.name)),
                        message: Box::from("missing field value after this field name"),
                    });
                    return None;
                };
                if type_fields
                    .iter()
                    .any(|type_field| type_field.name == field.name.value)
                {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(&field.name)),
                        message: Box::from(
                            "a field with this name already exists in the record pattern",
                        ),
                    });
                    return None;
                }
                let Some(checked_field_value_type) = syntax_parameter_pattern_check(
                    value,
                    errors,
                    introduced_variables,
                    type_aliases,
                    patterns,
                    existing_local_variables,
                    checked_spread_records,
                    records_used,
                    choices_used,
                ) else {
                    return None;
                };
                type_fields.push(TypeField {
                    name: field.name.value.clone(),
                    value: checked_field_value_type,
                });
            }
            records_used.insert(sorted_field_names(
                type_fields.iter().map(|type_field| &type_field.name),
            ));
            Some(Type::Record(type_fields))
        }
    }
}

pub fn checked_project_to_zig<Expressions, Patterns>(
    CheckedSyntaxProject {
        type_graph: _,
        project_type_by_graph_node: _,
        project_fn_graph: _,
        project_fn_graph_node_by_name,
        project_fn_by_graph_node,
        records_used: _,
        choices_used,
        checked_type_aliases,
        checked_project_fns,
        checked_calls,
        checked_local_fns,
        checked_queries,
        checked_spread_records,
    }: &CheckedSyntaxProject<Expressions, Patterns>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> String {
    let mut output: String = format!(
        "// jump to compiled code by searching for // compiled

{}


// compiled code //


",
        include_str!("core.zig"),
    );
    output.reserve(checked_project_fns.len() * 12 + checked_type_aliases.len() * 5);

    for (checked_type_alias_name, checked_type_alias) in checked_type_aliases {
        if let Some(checked_aliased_type) = &checked_type_alias.type_
            && !core_type_aliases.contains_key(checked_type_alias_name)
        {
            project_type_alias_to_zig(
                &mut output,
                checked_type_alias.documentation.as_deref(),
                checked_type_alias_name,
                &checked_type_alias.parameters,
                checked_aliased_type,
            );
        }
    }
    for (project_fn_name, checked_project_fn) in checked_project_fns {
        if let Some(syntax_project_fn_node) = project_fn_graph_node_by_name.get(project_fn_name)
            && let Some(syntax_project_fn) = project_fn_by_graph_node.get(syntax_project_fn_node)
            && let Some(result_type) = &checked_project_fn.result_type
            && let Some(result) = syntax_project_fn.result.as_ref()
        {
            syntax_project_fn_to_zig(
                &mut output,
                checked_type_aliases,
                checked_project_fns,
                checked_calls,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                expressions,
                patterns,
                project_fn_name,
                checked_project_fn.documentation.as_deref(),
                &checked_project_fn.parameter_types,
                result_type,
                checked_project_fn.result_expression_is_invalid,
                &syntax_project_fn.parameters,
                result,
            );
        }
    }
    for used_choice_variants in choices_used
        .iter()
        .filter(|choice| !core_choices.contains(choice.as_slice()))
    {
        syntax_choice_to_zig(&mut output, used_choice_variants);
    }
    output
}
fn syntax_choice_to_zig(output: &mut String, variant_names: &[Name]) {
    output.push_str("pub fn ");
    variant_names_to_zig_choice_type_name(output, variant_names);
    output.push('(');
    for variant_name in variant_names {
        name_to_uppercase_local_zig(output, variant_name);
        output.push_str(": type, ");
    }
    output.push_str(") type { return union(enum) { ");
    for variant_name in variant_names {
        output.push_str(&name_to_lowercase_zig(variant_name));
        output.push_str(": ");
        name_to_uppercase_local_zig(output, variant_name);
        output.push_str(", ");
    }
    output.push_str(" }; }\n\n");
}
fn variant_names_to_zig_choice_type_name<'a>(
    output: &mut String,
    variant_names: impl IntoIterator<Item = &'a Name>,
) {
    output.push_str("@\"");
    for variant_name in variant_names {
        output.push('\'');
        // no need to respect keywords etc
        output.push_str(&variant_name.replace("-", "_"));
    }
    output.push('"');
}
fn project_type_alias_to_zig(
    output: &mut String,
    maybe_documentation: Option<&str>,
    name: &Name,
    parameters: &[Name],
    aliased_type: &Type,
) {
    if let Some(documentation) = maybe_documentation {
        for documentation_line in documentation.lines() {
            output.push_str("///");
            output.push_str(documentation_line);
            output.push('\n');
        }
    }
    if parameters.is_empty() {
        output.push_str("pub const ");
        output.push_str(&name_to_uppercase_zig(name));
        output.push_str(": type = ");
        type_to_zig(output, aliased_type);
        output.push(';');
    } else {
        output.push_str("pub fn ");
        output.push_str(&name_to_uppercase_zig(name));
        output.push('(');
        for parameter in parameters {
            name_to_uppercase_local_zig(output, parameter);
            output.push_str(": type, ");
        }
        output.push_str(") type { return ");
        type_to_zig(output, aliased_type);
        output.push_str("; }");
    }
    output.push_str("\n\n");
}
fn syntax_project_fn_to_zig<Expressions, Patterns>(
    output: &mut String,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    checked_calls: &std::collections::HashMap<lsp_types::Position, CheckedCall>,
    checked_local_fns: &std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    name: &Name,
    maybe_documentation: Option<&str>,
    parameter_types: &[Option<Type>],
    result_type: &Type,
    result_expression_is_invalid: bool,
    syntax_parameters: &[SyntaxPattern<Patterns>],
    syntax_result: &SyntaxExpression<Expressions, Patterns>,
) {
    if let Some(documentation) = maybe_documentation {
        for documentation_line in documentation.lines() {
            output.push_str("///");
            output.push_str(documentation_line);
            output.push('\n');
        }
    }
    let function_scope_start = expression_start(syntax_result);
    output.push_str("pub fn ");
    output.push_str(&name_to_lowercase_zig(name));
    output.push('(');
    let mut type_parameters = std::collections::BTreeSet::new();
    for parameter_type in parameter_types.iter().flatten() {
        type_variables_into(&mut type_parameters, parameter_type);
    }
    type_variables_into(&mut type_parameters, result_type);
    for type_parameter in type_parameters {
        name_to_uppercase_local_zig(output, type_parameter);
        output.push_str(": type, ");
    }
    zig_allocator_variable(output, function_scope_start);
    output.push_str(": std.mem.Allocator");
    for (parameter_index, parameter_type) in parameter_types.iter().flatten().enumerate() {
        output.push_str(", @\"%");
        use std::fmt::Write as _;
        _ = write!(output, "{}", parameter_index);
        output.push_str("\": ");
        type_to_zig(output, parameter_type);
    }
    output.push_str(") error{OutOfMemory}!");
    type_to_zig(output, result_type);
    output.push_str(" {\n_ = @TypeOf(");
    zig_allocator_variable(output, function_scope_start);
    output.push_str(");\n");
    let mut parameter_introduced_variables = std::collections::HashMap::new();
    for (parameter_index, (syntax_parameter, _)) in syntax_parameters
        .iter()
        .zip(parameter_types)
        .filter(|(_, parameter_type)| parameter_type.is_some())
        .enumerate()
    {
        {
            syntax_pattern_to_zig_destructuring(
                output,
                syntax_parameter,
                &format!("@\"%{}\"", parameter_index),
                &mut parameter_introduced_variables,
                checked_spread_records,
                patterns,
            );
        }
    }
    output.push_str("return ");
    if result_expression_is_invalid {
        zig_incomplete_expression(output);
    } else {
        syntax_expression_to_zig(
            output,
            type_aliases,
            project_fns,
            expressions,
            patterns,
            checked_calls,
            checked_local_fns,
            checked_queries,
            checked_spread_records,
            &mut parameter_introduced_variables,
            syntax_result,
            function_scope_start,
            ZigReturnContext::Expression,
        );
    }
    output.push_str(";\n}\n\n");
}
fn zig_allocator_variable(output: &mut String, scope_start: lsp_types::Position) {
    use std::fmt::Write as _;
    output.push_str("@\"%allocator:");
    _ = write!(output, "{}", scope_start.line);
    output.push(':');
    _ = write!(output, "{}", scope_start.character);
    output.push('"');
}
fn zig_incomplete_expression(output: &mut String) {
    output.push_str("std.process.abort()");
}
fn syntax_pattern_to_zig_matches_condition<'a, Patterns>(
    output: &mut String,
    pattern: &'a SyntaxPattern<Patterns>,
    to_match: &str,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match pattern {
        SyntaxPattern::Variable { .. } => {
            output.push_str("true");
        }
        SyntaxPattern::Ignored { .. } => {
            output.push_str("true");
        }
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(name) = name
                && let Some(value) = value
            {
                output.push_str("switch (");
                output.push_str(to_match);
                output.push_str(") { .");
                output.push_str(&name_to_lowercase_zig(&name.value));
                output.push_str(" => true, else => false } and ");
                syntax_pattern_to_zig_matches_condition(
                    output,
                    patterns.item(value),
                    &format!("{to_match}.{}", name_to_lowercase_zig(&name.value)),
                    patterns,
                );
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for (part_index, part) in parts.iter().enumerate() {
                syntax_pattern_to_zig_matches_condition(
                    output,
                    part,
                    &format!("{to_match}.@\"{part_index}\""),
                    patterns,
                );
                output.push_str(" and ");
            }
            output.push_str("true");
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_pattern_to_zig_matches_condition(
                        output,
                        value,
                        &format!("{to_match}.{}", name_to_lowercase_zig(&field.name.value)),
                        patterns,
                    );
                    output.push_str(" and ");
                }
            }
            output.push_str("true");
        }
    }
}
fn syntax_pattern_to_zig_destructuring<'a, Patterns>(
    output: &mut String,
    pattern: &'a SyntaxPattern<Patterns>,
    to_destructure: &str,
    introduced_variables: &mut std::collections::HashMap<&'a Name, lsp_types::Position>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match pattern {
        SyntaxPattern::Variable { name, type_: _ } => {
            output.push_str("const ");
            name_to_lowercase_local_zig_introduced_at(output, &name.value, name.start);
            output.push_str(" = ");
            output.push_str(to_destructure);
            output.push_str(";\n_ = @TypeOf(");
            name_to_lowercase_local_zig_introduced_at(output, &name.value, name.start);
            output.push_str(");\n");
            introduced_variables.insert(&name.value, name.start);
        }
        SyntaxPattern::Ignored { .. } => {}
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(name) = name
                && let Some(value) = value
            {
                syntax_pattern_to_zig_destructuring(
                    output,
                    patterns.item(value),
                    &format!("{to_destructure}.{}", name_to_lowercase_zig(&name.value)),
                    introduced_variables,
                    checked_spread_records,
                    patterns,
                );
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for (part_index, part) in parts.iter().enumerate() {
                syntax_pattern_to_zig_destructuring(
                    output,
                    part,
                    &format!("{to_destructure}.@\"{part_index}\"",),
                    introduced_variables,
                    checked_spread_records,
                    patterns,
                );
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_pattern_to_zig_destructuring(
                        output,
                        value,
                        &format!(
                            "{to_destructure}.{}",
                            name_to_lowercase_zig(&field.name.value)
                        ),
                        introduced_variables,
                        checked_spread_records,
                        patterns,
                    );
                }
            }
        }
    }
}
enum ZigReturnContext {
    Expression,
    StatementsFollowedByBreak(lsp_types::Position),
}
fn syntax_expression_to_zig<'a, Expressions, Patterns>(
    output: &mut String,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    expressions: &'a core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
    checked_calls: &std::collections::HashMap<lsp_types::Position, CheckedCall>,
    checked_local_fns: &std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    local_variables: &mut std::collections::HashMap<&'a Name, lsp_types::Position>,
    expression: &'a SyntaxExpression<Expressions, Patterns>,
    function_scope_start: lsp_types::Position,
    return_context: ZigReturnContext,
) {
    match expression {
        SyntaxExpression::Number {
            value,
            type_: syntax_type,
        } => {
            let Some(syntax_type) = syntax_type else {
                zig_incomplete_expression(output);
                return;
            };
            let Some(type_) = syntax_type_to_type(syntax_type, type_aliases) else {
                zig_incomplete_expression(output);
                return;
            };
            use std::fmt::Write as _;
            match &type_ {
                Type::CoreConstruct { name, arguments: _ } => match name.as_str() {
                    "p32" => match value.value.parse::<std::num::NonZeroU32>() {
                        Ok(number) => {
                            if let ZigReturnContext::StatementsFollowedByBreak(label) =
                                return_context
                            {
                                zig_break_start(output, label);
                            }
                            output.push_str("P32 { .positive = ");
                            _ = write!(output, "{}", number);
                            output.push_str(" }");
                            if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                                zig_break_end(output);
                            }
                        }
                        Err(_) => zig_incomplete_expression(output),
                    },
                    "u32" => match value.value.parse::<u32>() {
                        Ok(number) => {
                            if let ZigReturnContext::StatementsFollowedByBreak(label) =
                                return_context
                            {
                                zig_break_start(output, label);
                            }
                            output.push_str("@as(u32, ");
                            _ = write!(output, "{}", number);
                            output.push(')');
                            if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                                zig_break_end(output);
                            }
                        }
                        Err(_) => zig_incomplete_expression(output),
                    },
                    "i32" => match value.value.parse::<i32>() {
                        Ok(number) => {
                            if let ZigReturnContext::StatementsFollowedByBreak(label) =
                                return_context
                            {
                                zig_break_start(output, label);
                            }
                            output.push_str("@as(i32, ");
                            _ = write!(output, "{}", number);
                            output.push(')');
                            if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                                zig_break_end(output);
                            }
                        }
                        Err(_) => zig_incomplete_expression(output),
                    },
                    "f32" => match value.value.parse::<f32>() {
                        Ok(number) => {
                            if let ZigReturnContext::StatementsFollowedByBreak(label) =
                                return_context
                            {
                                zig_break_start(output, label);
                            }
                            output.push_str("@as(f32, ");
                            _ = write!(output, "{}", number);
                            output.push(')');
                            if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                                zig_break_end(output);
                            }
                        }
                        Err(_) => zig_incomplete_expression(output),
                    },
                    _ => zig_incomplete_expression(output),
                },
                _ => zig_incomplete_expression(output),
            }
        }
        SyntaxExpression::Text {
            open_quote_start: _,
            content,
            content_end: _,
            closed_quote_exists: _,
            type_,
        } => {
            if let ZigReturnContext::StatementsFollowedByBreak(label) = return_context {
                zig_break_start(output, label);
            }
            if let Some(type_) = type_
                && let Some(type_) = syntax_type_to_type(type_, type_aliases)
                && let Type::CoreConstruct { name, arguments: _ } = type_
                && name == "char"
            {
                let Some(content) = content.chars().next() else {
                    zig_incomplete_expression(output);
                    return;
                };
                output.push_str("@as(Char, '");
                output.extend(content.escape_debug());
                output.push_str("\')");
            } else {
                if content.is_empty() {
                    zig_incomplete_expression(output);
                    return;
                }
                // improvement possibility: use direct construction instead
                // to avoid needless compile-time checks in zig
                output.push_str("Str.fromComptime(\"");
                output.extend(content.escape_debug());
                output.push_str("\")");
            }
            if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                zig_break_end(output);
            }
        }
        SyntaxExpression::Variable(name) => {
            if let ZigReturnContext::StatementsFollowedByBreak(label) = return_context {
                zig_break_start(output, label);
            }
            if let Some(local_variable_introduced_start) = local_variables.remove(&name.value) {
                name_to_lowercase_local_zig_introduced_at(
                    output,
                    &name.value,
                    local_variable_introduced_start,
                );
            } else {
                name_to_lowercase_local_zig(output, &name.value);
            }
            if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                zig_break_end(output);
            }
        }
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments,
            open_paren_start: _,
            arguments_rest,
            closed_paren_start: _,
        } => {
            let Some(called_function) = project_fns.get(&name.value) else {
                zig_incomplete_expression(output);
                return;
            };
            let Some(checked_call) = checked_calls.get(&name.start) else {
                zig_incomplete_expression(output);
                return;
            };
            // can be optimized by avoiding the alloction
            let mut type_variable_arguments =
                checked_call.argument_type_variable_replacements.clone();
            type_variable_arguments.extend(
                called_function.type_parameters.iter().cloned().zip(
                    type_arguments
                        .iter()
                        .flat_map(|type_arguments| &type_arguments.types)
                        .filter_map(|type_argument| {
                            syntax_type_to_type(type_argument, type_aliases)
                        }),
                ),
            );
            if let ZigReturnContext::StatementsFollowedByBreak(label) = return_context {
                zig_break_start(output, label);
            }
            if is_core_fn_that_can_run_out_of_memory_in_zig(&name.value)
                || !core_fns.contains_key(&name.value)
            {
                output.push_str("try ");
            }
            output.push_str(&name_to_lowercase_zig(&name.value));
            output.push('(');
            for type_variable_argument in type_variable_arguments.into_values() {
                type_to_zig(output, &type_variable_argument);
                output.push_str(", ");
            }
            if is_core_fn_taking_allocator_in_zig(&name.value)
                || !core_fns.contains_key(&name.value)
            {
                zig_allocator_variable(output, function_scope_start);
                output.push_str(", ");
            }
            let maybe_first_argument = first_argument_dot
                .as_ref()
                .map(|first_argument_dot| expressions.item(&first_argument_dot.argument));
            for argument in maybe_first_argument.into_iter().chain(arguments_rest) {
                syntax_expression_to_zig(
                    output,
                    type_aliases,
                    project_fns,
                    expressions,
                    patterns,
                    checked_calls,
                    checked_local_fns,
                    checked_queries,
                    checked_spread_records,
                    local_variables,
                    argument,
                    function_scope_start,
                    ZigReturnContext::Expression,
                );
                output.push_str(", ");
            }
            output.push(')');
            if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                zig_break_end(output);
            }
        }
        SyntaxExpression::Variant {
            open_bracket_start: _,
            type_,
            name,
            value,
            closed_bracket_start: _,
        } => {
            let Some(name) = name else {
                zig_incomplete_expression(output);
                return;
            };
            let Some(SyntaxAngledTypeArgument {
                open_angle_start: _,
                type_: Some(syntax_type),
                closed_angle_start: _,
            }) = type_
            else {
                zig_incomplete_expression(output);
                return;
            };
            let Some(Type::Choice(provided_type_variants)) =
                syntax_type_to_type(syntax_type, type_aliases)
            else {
                zig_incomplete_expression(output);
                return;
            };
            let Some(value) = value else {
                zig_incomplete_expression(output);
                return;
            };
            let label = match return_context {
                ZigReturnContext::StatementsFollowedByBreak(label) => label,
                ZigReturnContext::Expression => {
                    let label = expression_start(expression);
                    zig_block_start(output, label);
                    label
                }
            };
            let value = expressions.item(value);
            fn value_variable_name(output: &mut String, start: lsp_types::Position) {
                use std::fmt::Write as _;
                output.push_str("@\"%value:");
                _ = write!(output, "{}", start.line);
                output.push(':');
                _ = write!(output, "{}", start.character);
                output.push('"');
            }
            let value_start = expression_start(value);
            output.push_str("const ");
            value_variable_name(output, value_start);
            output.push_str(" = ");
            syntax_expression_to_zig(
                output,
                type_aliases,
                project_fns,
                expressions,
                patterns,
                checked_calls,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                local_variables,
                value,
                function_scope_start,
                ZigReturnContext::Expression,
            );
            output.push(';');
            zig_break_start(output, label);
            let mut type_variants_sorted = provided_type_variants
                .iter()
                .map(
                    |TypeVariant {
                         name: type_variant_name,
                         value: type_variant_value,
                     }| (type_variant_name, Some(type_variant_value)),
                )
                .chain(
                    if provided_type_variants
                        .iter()
                        .any(|variant| variant.name == name.value)
                    {
                        None
                    } else {
                        Some((&name.value, None))
                    },
                )
                .collect::<Vec<_>>();
            type_variants_sorted.sort_unstable_by_key(|(type_variant_name, _)| *type_variant_name);
            variant_names_to_zig_choice_type_name(
                output,
                type_variants_sorted
                    .iter()
                    .map(|(type_variant_name, _)| *type_variant_name),
            );
            output.push('(');
            for (_, type_variant_value) in type_variants_sorted {
                match type_variant_value {
                    Some(type_variant_value) => {
                        type_to_zig(output, type_variant_value);
                    }
                    None => {
                        output.push_str("@TypeOf(");
                        value_variable_name(output, value_start);
                        output.push(')');
                    }
                }
                output.push_str(", ");
            }
            output.push_str(") { .");
            output.push_str(&name_to_lowercase_zig(&name.value));
            output.push_str(" = ");
            value_variable_name(output, value_start);
            output.push_str(" }");
            zig_break_end(output);
            if let ZigReturnContext::Expression = return_context {
                zig_block_end(output);
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            let Some(checked_local_fn) = checked_local_fns.get(fn_keyword_start) else {
                zig_incomplete_expression(output);
                return;
            };
            let local_function_scope_start = *fn_keyword_start;
            fn function_parameter_name(output: &mut String, parameter_start: lsp_types::Position) {
                use std::fmt::Write as _;
                output.push_str("@\"%");
                _ = write!(output, "{}", parameter_start.line);
                output.push(':');
                _ = write!(output, "{}", parameter_start.character);
                output.push('"');
            }
            if let ZigReturnContext::StatementsFollowedByBreak(label) = return_context {
                zig_break_start(output, label);
            }
            // TODO
            output.push_str("@as(Fn(");
            for parameter_type in &checked_local_fn.parameter_types {
                type_to_zig(output, parameter_type);
                output.push_str(", ");
            }
            type_to_zig(output, &checked_local_fn.result_type);
            output.push_str(",), struct {\npub fn f(");
            zig_allocator_variable(output, local_function_scope_start);
            output.push_str(": std.mem.Allocator, ");
            for (parameter, parameter_type) in
                parameters.iter().zip(&checked_local_fn.parameter_types)
            {
                function_parameter_name(output, pattern_start(parameter));
                output.push_str(": ");
                type_to_zig(output, parameter_type);
                output.push_str(",");
            }
            output.push_str(") error{OutOfMemory}!");
            type_to_zig(output, &checked_local_fn.result_type);
            output.push_str(" {\n");
            output.push_str("_ = @TypeOf(");
            zig_allocator_variable(output, local_function_scope_start);
            output.push_str(");\n");
            let mut parameter_introduced_variables = std::collections::HashMap::new();
            for syntax_parameter in parameters {
                let mut parameter_name = String::new();
                function_parameter_name(&mut parameter_name, pattern_start(syntax_parameter));
                syntax_pattern_to_zig_destructuring(
                    output,
                    syntax_parameter,
                    &parameter_name,
                    &mut parameter_introduced_variables,
                    checked_spread_records,
                    patterns,
                );
            }
            output.push_str("return ");
            match result {
                None => {
                    zig_incomplete_expression(output);
                }
                Some(result) => {
                    syntax_expression_to_zig(
                        output,
                        type_aliases,
                        project_fns,
                        expressions,
                        patterns,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        checked_spread_records,
                        &mut parameter_introduced_variables,
                        expressions.item(result),
                        local_function_scope_start,
                        ZigReturnContext::Expression,
                    );
                }
            }
            output.push_str(";\n}\n}.f)");
            if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                zig_break_end(output);
            }
        }
        SyntaxExpression::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            let label = match return_context {
                ZigReturnContext::StatementsFollowedByBreak(label) => label,
                ZigReturnContext::Expression => {
                    let label = expression_start(expression);
                    label
                }
            };
            if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                zig_break_start(output, label);
            }
            output.push_str(".{ ");
            for part in parts {
                syntax_expression_to_zig(
                    output,
                    type_aliases,
                    project_fns,
                    expressions,
                    patterns,
                    checked_calls,
                    checked_local_fns,
                    checked_queries,
                    checked_spread_records,
                    local_variables,
                    part,
                    function_scope_start,
                    ZigReturnContext::Expression,
                );
                output.push_str(", ");
            }
            output.push_str(" }");
            match return_context {
                ZigReturnContext::Expression => {}
                ZigReturnContext::StatementsFollowedByBreak(_) => {
                    zig_break_end(output);
                }
            }
        }
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            let label = match return_context {
                ZigReturnContext::StatementsFollowedByBreak(label) => label,
                ZigReturnContext::Expression => {
                    let label = expression_start(expression);
                    label
                }
            };
            let mut sorted_field_setters = std::collections::BTreeMap::<
                &str,
                Option<&'a SyntaxExpression<Expressions, Patterns>>,
            >::new();
            for field in fields {
                sorted_field_setters.insert(&field.name.value, field.value.as_ref());
            }
            if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                zig_break_start(output, label);
            }
            output.push_str("record(.{ ");
            for (name, value) in sorted_field_setters {
                output.push('.');
                output.push_str(&name_to_lowercase_zig(name));
                output.push_str(" = ");
                match value {
                    None => {
                        zig_incomplete_expression(output);
                    }
                    Some(value) => {
                        syntax_expression_to_zig(
                            output,
                            type_aliases,
                            project_fns,
                            expressions,
                            patterns,
                            checked_calls,
                            checked_local_fns,
                            checked_queries,
                            checked_spread_records,
                            local_variables,
                            value,
                            function_scope_start,
                            ZigReturnContext::Expression,
                        );
                    }
                }
                output.push_str(", ");
            }
            output.push_str(" })");
            match return_context {
                ZigReturnContext::Expression => {}
                ZigReturnContext::StatementsFollowedByBreak(_) => {
                    zig_break_end(output);
                }
            }
        }
        SyntaxExpression::Commented {
            comments,
            expression,
        } => {
            for comment_line in std::iter::once(&comments.line0).chain(&comments.line1_up) {
                output.push_str("// ");
                output.push_str(&comment_line.value);
                output.push('\n');
            }
            let Some(expression) = expression else {
                zig_incomplete_expression(output);
                return;
            };
            syntax_expression_to_zig(
                output,
                type_aliases,
                project_fns,
                expressions,
                patterns,
                checked_calls,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                local_variables,
                expressions.item(expression),
                function_scope_start,
                return_context,
            );
        }

        SyntaxExpression::Destruct {
            equals_start,
            pattern,
            queried,
            result,
        } => {
            let queried_variable_name = {
                let mut queried_variable_name = String::new();
                use std::fmt::Write as _;
                queried_variable_name.push_str("@\"%queried:");
                _ = write!(queried_variable_name, "{}", equals_start.line);
                queried_variable_name.push(':');
                _ = write!(queried_variable_name, "{}", equals_start.character);
                queried_variable_name.push('"');
                queried_variable_name
            };
            let label = match return_context {
                ZigReturnContext::Expression => {
                    zig_block_start(output, *equals_start);
                    *equals_start
                }
                ZigReturnContext::StatementsFollowedByBreak(label) => label,
            };
            output.push_str("const ");
            output.push_str(&queried_variable_name);
            output.push_str(" = ");
            match queried {
                None => {
                    zig_incomplete_expression(output);
                }
                Some(queried) => {
                    syntax_expression_to_zig(
                        output,
                        type_aliases,
                        project_fns,
                        expressions,
                        patterns,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        checked_spread_records,
                        local_variables,
                        expressions.item(queried),
                        function_scope_start,
                        ZigReturnContext::Expression,
                    );
                }
            }
            output.push_str(";\n");
            if let Some(pattern) = pattern {
                syntax_pattern_to_zig_destructuring(
                    output,
                    pattern,
                    &queried_variable_name,
                    local_variables,
                    checked_spread_records,
                    patterns,
                );
            }
            match result {
                None => {
                    zig_incomplete_expression(output);
                }
                Some(result) => {
                    syntax_expression_to_zig(
                        output,
                        type_aliases,
                        project_fns,
                        expressions,
                        patterns,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        checked_spread_records,
                        local_variables,
                        expressions.item(result),
                        function_scope_start,
                        ZigReturnContext::StatementsFollowedByBreak(label),
                    );
                }
            }
            if let ZigReturnContext::Expression = return_context {
                zig_block_end(output);
            }
        }
        SyntaxExpression::If {
            if_keyword_start,
            queried,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => {
            let Some(checked_query) = checked_queries.get(if_keyword_start) else {
                zig_incomplete_expression(output);
                return;
            };
            let queried_variable_name = {
                let mut queried_variable_name = String::new();
                use std::fmt::Write as _;
                queried_variable_name.push_str("@\"%queried:");
                _ = write!(queried_variable_name, "{}", if_keyword_start.line);
                queried_variable_name.push(':');
                _ = write!(queried_variable_name, "{}", if_keyword_start.character);
                queried_variable_name.push('"');
                queried_variable_name
            };
            let label = match return_context {
                ZigReturnContext::Expression => {
                    zig_block_start(output, *if_keyword_start);
                    *if_keyword_start
                }
                ZigReturnContext::StatementsFollowedByBreak(label) => label,
            };
            output.push_str("const ");
            output.push_str(&queried_variable_name);
            output.push_str(" = ");
            match queried {
                None => {
                    zig_incomplete_expression(output);
                }
                Some(queried) => {
                    syntax_expression_to_zig(
                        output,
                        type_aliases,
                        project_fns,
                        expressions,
                        patterns,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        checked_spread_records,
                        local_variables,
                        expressions.item(queried),
                        function_scope_start,
                        ZigReturnContext::Expression,
                    );
                }
            }
            output.push_str(";\n");
            for (case_index, case) in cases.iter().enumerate() {
                if !checked_query.invalid_case_indexes.contains(&case_index) {
                    if cases.len() >= 2 {
                        output.push_str("if (");
                        syntax_pattern_to_zig_matches_condition(
                            output,
                            &case.pattern,
                            &queried_variable_name,
                            patterns,
                        );
                        output.push_str(") {\n");
                    }
                    // can be optimized by not cloning if there is only one case
                    let mut case_local_variables = local_variables.clone();
                    syntax_pattern_to_zig_destructuring(
                        output,
                        &case.pattern,
                        &queried_variable_name,
                        &mut case_local_variables,
                        checked_spread_records,
                        patterns,
                    );
                    match &case.result {
                        None => {
                            zig_incomplete_expression(output);
                        }
                        Some(case_result) => {
                            syntax_expression_to_zig(
                                output,
                                type_aliases,
                                project_fns,
                                expressions,
                                patterns,
                                checked_calls,
                                checked_local_fns,
                                checked_queries,
                                checked_spread_records,
                                &mut case_local_variables,
                                case_result,
                                function_scope_start,
                                ZigReturnContext::StatementsFollowedByBreak(label),
                            );
                        }
                    }
                    if cases.len() >= 2 {
                        output.push_str("\n}");
                    }
                }
            }
            if cases.is_empty() {
                zig_incomplete_expression(output);
            } else if cases.len() >= 2 {
                // in case none of the above patterns have matched, crash
                output.push_str("\nunreachable;");
            }
            if let ZigReturnContext::Expression = return_context {
                zig_block_end(output);
            }
        }
    }
}
fn zig_block_start(output: &mut String, label: lsp_types::Position) {
    zig_block_label_for_start_position(output, label);
    output.push_str(": {\n");
}
fn zig_block_end(output: &mut String) {
    output.push_str("\n}");
}
fn zig_break_start(output: &mut String, label: lsp_types::Position) {
    output.push_str("break :");
    zig_block_label_for_start_position(output, label);
    output.push(' ');
}
fn zig_break_end(output: &mut String) {
    output.push(';');
}
fn zig_block_label_for_start_position(output: &mut String, position: lsp_types::Position) {
    use std::fmt::Write as _;
    output.push_str("@\"%block:");
    _ = write!(output, "{}", position.line);
    output.push(':');
    _ = write!(output, "{}", position.character);
    output.push('"');
}
fn type_to_zig(output: &mut String, type_: &Type) {
    match type_ {
        Type::Variable(name) => name_to_uppercase_local_zig(output, name),
        Type::Origin(name) => origin_name_to_uppercase_local_zig(output, name),
        Type::Tuple(parts) => {
            if parts.is_empty() {
                output.push_str(empty_zig_type_name);
            } else {
                output.push_str("struct { ");
                for part in parts {
                    type_to_zig(output, part);
                    output.push_str(", ");
                }
                output.push_str("}");
            }
        }
        Type::Record(fields) => {
            if fields.is_empty() {
                output.push_str(empty_zig_type_name);
            } else {
                let mut fields_sorted = fields.iter().collect::<Vec<_>>();
                fields_sorted.sort_unstable_by_key(|field| &field.name);
                output.push_str("Record(struct { ");
                for TypeField { name, value } in fields_sorted {
                    output.push_str(&name_to_lowercase_zig(name));
                    output.push_str(": ");
                    type_to_zig(output, value);
                    output.push_str(", ");
                }
                output.push_str("})");
            }
        }
        Type::Choice(variants) => {
            if variants.is_empty() {
                output.push_str(choice_empty_zig_type_name);
            } else {
                let mut variants_sorted = variants.iter().collect::<Vec<_>>();
                variants_sorted.sort_unstable_by_key(|variant| &variant.name);
                variant_names_to_zig_choice_type_name(
                    output,
                    variants_sorted.iter().map(|variant| &variant.name),
                );
                output.push('(');
                for TypeVariant { name: _, value } in variants_sorted {
                    type_to_zig(output, value);
                    output.push_str(", ");
                }
                output.push(')');
            }
        }
        Type::CoreConstruct { name, arguments } => {
            output.push_str(&name_to_uppercase_zig(name));
            if !arguments.is_empty() {
                output.push('(');
                for argument in arguments {
                    type_to_zig(output, argument);
                    output.push_str(", ");
                }
                output.push(')');
            }
        }
        Type::Fn { parameters, result } => {
            output.push_str("fn(std.mem.Allocator, ");
            for parameter in parameters {
                type_to_zig(output, parameter);
                output.push_str(", ");
            }
            output.push_str(") error{OutOfMemory}!");
            type_to_zig(output, result);
        }
    }
}
fn name_to_uppercase_zig(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_uppercase();
    }
    // Not sure if type variables in core code can actually collide with generated type names?
    match sanitized.as_str() {
        "Record" | "Range" => sanitized + "ø",
        _ => sanitized,
    }
}
fn name_to_uppercase_local_zig(output: &mut String, name: &str) {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_uppercase();
    }
    output.push_str("@\"%");
    output.push_str(&sanitized);
    output.push('"');
}
fn origin_name_to_uppercase_local_zig(output: &mut String, name: &str) {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_uppercase();
    }
    output.push_str("@\"^");
    output.push_str(&sanitized);
    output.push('"');
}
const empty_zig_type_name: &str = "void";
const choice_empty_zig_type_name: &str = "Choice";
pub fn name_to_lowercase_zig(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_lowercase();
    }
    if zig_lowercase_keywords.contains(&sanitized.as_str()) {
        format!("@\"{sanitized}\"")
    } else if zig_unrelated_top_level_lowercase_names.contains(&sanitized.as_str()) {
        sanitized + "ø"
    } else {
        sanitized
    }
}
fn name_to_lowercase_local_zig(output: &mut String, name: &str) {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_lowercase();
    }
    output.push_str("@\"%");
    output.push_str(&sanitized);
    output.push('"');
}
fn name_to_lowercase_local_zig_introduced_at(
    output: &mut String,
    name: &str,
    introduced_start: lsp_types::Position,
) {
    use std::fmt::Write as _;
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_lowercase();
    }
    output.push_str("@\"%");
    output.push_str(&sanitized);
    output.push(':');
    _ = write!(output, "{}", introduced_start.line);
    output.push(':');
    _ = write!(output, "{}", introduced_start.character);
    output.push('"');
}
const zig_unrelated_top_level_lowercase_names: [&str; 9] = [
    local_unnamed_function_name,
    empty_zig_type_name,
    "std",
    "record",
    "recordToArray",
    "u32AddOrOutOfMem",
    "usizeAddOrOutOfMem",
    "strideOf",
    "mathOrderToOrder",
];
/// see <https://ziglang.org/documentation/master/#Keyword-Reference>
const zig_lowercase_keywords: [&str; 46] = [
    "addrspace",
    "align",
    "allowzero",
    "and",
    "anyframe",
    "anytype",
    "asm",
    "asm",
    "asm",
    "catch",
    "comptime",
    "const",
    "continue",
    "defer",
    "else",
    "enum",
    "errdefer",
    "error",
    "export",
    "extern",
    "fn",
    "for",
    "if",
    "inline",
    "linksection",
    "noalias",
    "noinline",
    "nosuspend",
    "opaque",
    "or",
    "orelse",
    "packed",
    "pub",
    "resume",
    "return",
    "struct",
    "suspend",
    "switch",
    "test",
    "threadlocal",
    "try",
    "union",
    "unreachable",
    "var",
    "volatile",
    "while",
];

pub fn checked_project_to_js<Expressions, Patterns>(
    CheckedSyntaxProject {
        type_graph: _,
        project_type_by_graph_node: _,
        project_fn_graph: _,
        project_fn_graph_node_by_name,
        project_fn_by_graph_node,
        records_used: _,
        choices_used: _,
        checked_type_aliases,
        checked_project_fns,
        checked_calls: _,
        checked_local_fns: _,
        checked_queries,
        checked_spread_records,
    }: &CheckedSyntaxProject<Expressions, Patterns>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> String {
    let mut output: String = format!(
        "// jump to compiled code by searching for // compiled

{}


// compiled code //


",
        include_str!("core.mjs"),
    );
    output.reserve(checked_project_fns.len() * 12 + checked_type_aliases.len() * 5);

    for (checked_type_alias_name, checked_type_alias) in checked_type_aliases {
        if let Some(checked_aliased_type) = &checked_type_alias.type_
            && !core_type_aliases.contains_key(checked_type_alias_name)
        {
            project_type_alias_to_js(
                &mut output,
                checked_type_alias.documentation.as_deref(),
                checked_type_alias_name,
                &checked_type_alias.parameters,
                checked_aliased_type,
            );
        }
    }
    for (project_fn_name, checked_project_fn) in checked_project_fns {
        if let Some(syntax_project_fn_node) = project_fn_graph_node_by_name.get(project_fn_name)
            && let Some(syntax_project_fn) = project_fn_by_graph_node.get(syntax_project_fn_node)
            && let Some(result_type) = &checked_project_fn.result_type
            && let Some(result) = syntax_project_fn.result.as_ref()
        {
            syntax_project_fn_to_js(
                &mut output,
                checked_type_aliases,
                checked_queries,
                checked_spread_records,
                expressions,
                patterns,
                project_fn_name,
                checked_project_fn.documentation.as_deref(),
                &checked_project_fn.parameter_types,
                &syntax_project_fn.parameters,
                result_type,
                checked_project_fn.result_expression_is_invalid,
                result,
            );
        }
    }
    output
}
fn project_type_alias_to_js(
    output: &mut String,
    maybe_documentation: Option<&str>,
    name: &Name,
    parameters: &[Name],
    aliased_type: &Type,
) {
    output.push_str("/** ");
    if let Some(documentation) = maybe_documentation {
        output.push_str(&documentation.replace("@", "(@)"));
    }
    output.push('\n');
    for parameter in parameters {
        output.push_str("@template ");
        name_to_uppercase_local_js(output, parameter);
    }
    output.push_str(" @typedef {");
    type_to_js(output, aliased_type);
    output.push_str("} ");
    output.push_str(&name_to_uppercase_js(name));
    output.push_str("\n*/\n\n");
}
fn syntax_project_fn_to_js<Expressions, Patterns>(
    output: &mut String,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,

    name: &Name,
    maybe_documentation: Option<&str>,
    parameter_types: &[Option<Type>],
    syntax_parameters: &[SyntaxPattern<Patterns>],
    result_type: &Type,
    result_expression_is_invalid: bool,
    syntax_result: &SyntaxExpression<Expressions, Patterns>,
) {
    output.push_str("/** ");
    if let Some(documentation) = maybe_documentation {
        output.push_str(&documentation.replace("@", "(@)"));
    }
    output.push('\n');
    let mut type_parameters = std::collections::BTreeSet::new();
    for parameter_type in parameter_types.iter().flatten() {
        type_variables_into(&mut type_parameters, parameter_type);
    }
    type_variables_into(&mut type_parameters, result_type);
    for type_parameter in type_parameters {
        output.push_str("@template ");
        name_to_uppercase_local_js(output, type_parameter);
        output.push(' ');
    }
    for (parameter_type_index, parameter_type) in parameter_types.iter().flatten().enumerate() {
        output.push_str("@param {");
        type_to_js(output, parameter_type);
        output.push_str("} $");
        use std::fmt::Write as _;
        _ = write!(output, "{}", parameter_type_index);
        output.push(' ');
    }
    output.push_str("@returns {");
    type_to_js(output, result_type);
    output.push_str("}\n*/\n");
    let function_scope_start = expression_start(syntax_result);
    output.push_str("export function ");
    output.push_str(&name_to_lowercase_js(name));
    output.push_str("($) {");
    let mut parameter_introduced_variables = std::collections::HashMap::new();
    for (syntax_parameter_index, (syntax_parameter, _)) in syntax_parameters
        .iter()
        .zip(parameter_types)
        .filter(|(_, type_)| type_.is_some())
        .enumerate()
    {
        output.push('\n');
        syntax_pattern_to_js_destructuring(
            output,
            syntax_parameter,
            &format!("${}", syntax_parameter_index),
            &mut parameter_introduced_variables,
            checked_spread_records,
            patterns,
        );
    }
    output.push_str("let ");
    js_scope_result_variable(output, function_scope_start);
    output.push_str(";\n");
    if result_expression_is_invalid {
        js_incomplete_statement();
    } else {
        syntax_expression_to_js(
            output,
            type_aliases,
            expressions,
            patterns,
            checked_queries,
            checked_spread_records,
            &mut parameter_introduced_variables,
            syntax_result,
            function_scope_start,
        );
    }
    output.push_str("return ");
    js_scope_result_variable(output, function_scope_start);
    output.push_str(";\n}\n\n");
}
fn type_to_js(output: &mut String, type_: &Type) {
    match type_ {
        Type::Variable(name) => {
            name_to_uppercase_local_js(output, name);
        }
        Type::Origin(name) => {
            origin_name_to_uppercase_js(output, name);
        }
        Type::Tuple(parts) => {
            output.push_str("[ ");
            for part in parts {
                type_to_js(output, part);
                output.push_str(", ");
            }
            output.push(']');
        }
        Type::Record(fields) => {
            if fields.is_empty() {
                output.push_str(record_empty_js_type_name);
            } else {
                output.push_str("{ ");
                for field in fields {
                    output.push_str(&name_to_lowercase_js(&field.name));
                    output.push_str(": ");
                    type_to_js(output, &field.value);
                    output.push_str(", ");
                }
                output.push('}');
            }
        }
        Type::Choice(variants) => match variants.split_first() {
            None => {
                output.push_str(choice_empty_js_type_name);
            }
            Some((variant0, variant1_up)) => {
                output.push_str("{ ");
                output.push_str(&name_to_lowercase_js(&variant0.name));
                output.push_str(": ");
                type_to_js(output, &variant0.value);
                output.push_str(" }");
                for variant in variant1_up {
                    output.push_str(" | { ");
                    output.push_str(&name_to_lowercase_js(&variant.name));
                    output.push_str(": ");
                    type_to_js(output, &variant.value);
                    output.push_str(" }");
                }
            }
        },
        Type::CoreConstruct { name, arguments } => {
            output.push_str(&name_to_uppercase_js(name));
            if let Some((argument0, argument1_up)) = arguments.split_first() {
                output.push('<');
                type_to_js(output, argument0);
                for argument in argument1_up {
                    output.push_str(", ");
                    type_to_js(output, argument);
                }
                output.push('>');
            }
        }
        Type::Fn { parameters, result } => {
            output.push('(');
            for parameter in parameters {
                output.push_str("_: ");
                type_to_js(output, parameter);
                output.push_str(", ");
            }
            output.push_str(") => ");
            type_to_js(output, result);
        }
    }
}
fn syntax_pattern_to_js_matches_condition<'a, Patterns>(
    output: &mut String,
    pattern: &'a SyntaxPattern<Patterns>,
    to_match: &str,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match pattern {
        SyntaxPattern::Variable { .. } => {
            output.push_str("true");
        }
        SyntaxPattern::Ignored { .. } => {
            output.push_str("true");
        }
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(name) = name
                && let Some(value) = value
            {
                output.push('"');
                output.push_str(&name_to_lowercase_js(&name.value));
                output.push_str("\" in ");
                output.push_str(to_match);
                output.push_str(" && ");
                syntax_pattern_to_js_matches_condition(
                    output,
                    patterns.item(value),
                    &format!("{to_match}.{}", name_to_lowercase_js(&name.value)),
                    patterns,
                );
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for (part_index, part) in parts.iter().enumerate() {
                syntax_pattern_to_js_matches_condition(
                    output,
                    part,
                    &format!("{to_match}[{part_index}]"),
                    patterns,
                );
                output.push_str(" && ");
            }
            output.push_str("true");
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_pattern_to_js_matches_condition(
                        output,
                        value,
                        &format!("{to_match}.{}", name_to_lowercase_js(&field.name.value)),
                        patterns,
                    );
                    output.push_str(" && ");
                }
            }
            output.push_str("true");
        }
    }
}
fn syntax_pattern_to_js_destructuring<'a, Patterns>(
    output: &mut String,
    pattern: &'a SyntaxPattern<Patterns>,
    to_destructure: &str,
    introduced_variables: &mut std::collections::HashMap<&'a Name, lsp_types::Position>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match pattern {
        SyntaxPattern::Ignored { .. } => {}
        SyntaxPattern::Variable { name, type_: _ } => {
            output.push_str("const ");
            name_to_lowercase_local_js_introduced_at(output, &name.value, name.start);
            output.push_str(" = ");
            output.push_str(to_destructure);
            output.push_str(";\n");
            introduced_variables.insert(&name.value, name.start);
        }
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(name) = name
                && let Some(value) = value
            {
                syntax_pattern_to_js_destructuring(
                    output,
                    patterns.item(value),
                    &format!("{to_destructure}.{}", name_to_lowercase_js(&name.value)),
                    introduced_variables,
                    checked_spread_records,
                    patterns,
                );
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for (part_index, part) in parts.iter().enumerate() {
                syntax_pattern_to_js_destructuring(
                    output,
                    part,
                    &format!("{to_destructure}[{part_index}]"),
                    introduced_variables,
                    checked_spread_records,
                    patterns,
                );
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_pattern_to_js_destructuring(
                        output,
                        value,
                        &format!(
                            "{to_destructure}.{}",
                            name_to_lowercase_js(&field.name.value)
                        ),
                        introduced_variables,
                        checked_spread_records,
                        patterns,
                    );
                }
            }
        }
    }
}
fn syntax_expression_to_js<'a, Expressions, Patterns>(
    output: &mut String,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    expressions: &'a core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    local_variables: &mut std::collections::HashMap<&'a Name, lsp_types::Position>,
    expression: &'a SyntaxExpression<Expressions, Patterns>,
    scope_start: lsp_types::Position,
) {
    match expression {
        SyntaxExpression::Number {
            value,
            type_: syntax_type,
        } => {
            let Some(syntax_type) = syntax_type else {
                js_incomplete_statement();
                return;
            };
            let Some(type_) = syntax_type_to_type(syntax_type, type_aliases) else {
                js_incomplete_statement();
                return;
            };
            use std::fmt::Write as _;
            match &type_ {
                Type::CoreConstruct { name, arguments: _ } => match name.as_str() {
                    "p32" => match value.value.parse::<std::num::NonZeroU32>() {
                        Ok(number) => {
                            js_scope_result_variable(output, scope_start);
                            output.push_str(" = ");
                            _ = write!(output, "{}", number);
                            output.push_str(";\n");
                        }
                        Err(_) => js_incomplete_statement(),
                    },
                    "u32" => match value.value.parse::<u32>() {
                        Ok(number) => {
                            js_scope_result_variable(output, scope_start);
                            output.push_str(" = ");
                            _ = write!(output, "{}", number);
                            output.push_str(";\n");
                        }
                        Err(_) => js_incomplete_statement(),
                    },
                    "i32" => match value.value.parse::<i32>() {
                        Ok(number) => {
                            js_scope_result_variable(output, scope_start);
                            output.push_str(" = ");
                            _ = write!(output, "{}", number);
                            output.push_str(";\n");
                        }
                        Err(_) => js_incomplete_statement(),
                    },
                    "f32" => match value.value.parse::<f32>() {
                        Ok(number) => {
                            js_scope_result_variable(output, scope_start);
                            output.push_str(" = ");
                            _ = write!(output, "{}", number);
                            output.push_str(";\n");
                        }
                        Err(_) => js_incomplete_statement(),
                    },
                    _ => js_incomplete_statement(),
                },
                _ => js_incomplete_statement(),
            }
        }
        SyntaxExpression::Text {
            open_quote_start: _,
            content,
            content_end: _,
            closed_quote_exists: _,
            type_: _,
        } => {
            if content.is_empty() {
                js_incomplete_statement();
                return;
            }
            js_scope_result_variable(output, scope_start);
            output.push_str(" = \"");
            output.extend(content.escape_debug());
            output.push_str("\";\n");
        }
        SyntaxExpression::Variable(name) => {
            js_scope_result_variable(output, scope_start);
            output.push_str(" = ");
            if let Some(local_variable_introduced_start) = local_variables.remove(&name.value) {
                name_to_lowercase_local_js_introduced_at(
                    output,
                    &name.value,
                    local_variable_introduced_start,
                );
            } else {
                name_to_lowercase_local_js(output, &name.value);
            }
            output.push_str(";\n");
        }
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments: _,
            open_paren_start: _,
            arguments_rest: rest_arguments,
            closed_paren_start: _,
        } => {
            // TODO call local variables instead when available and no type_arguments
            let maybe_first_argument = first_argument_dot
                .as_ref()
                .map(|first_argument_dot| expressions.item(&first_argument_dot.argument));
            for argument in maybe_first_argument.into_iter().chain(rest_arguments) {
                let argument_scope_start = expression_start(argument);
                output.push_str("let ");
                js_scope_result_variable(output, argument_scope_start);
                output.push_str(";\n");
                syntax_expression_to_js(
                    output,
                    type_aliases,
                    expressions,
                    patterns,
                    checked_queries,
                    checked_spread_records,
                    local_variables,
                    argument,
                    argument_scope_start,
                );
                js_scope_result_variable(output, scope_start);
                output.push_str(" = ");
            }
            output.push_str(&name_to_lowercase_js(&name.value));
            output.push('(');
            for argument in rest_arguments {
                js_scope_result_variable(output, expression_start(argument));
            }
            output.push_str(");\n");
        }
        SyntaxExpression::Variant {
            open_bracket_start: _,
            type_: _,
            name,
            value,
            closed_bracket_start: _,
        } => {
            let Some(name) = name else {
                js_incomplete_statement();
                return;
            };
            let Some(value) = value else {
                js_incomplete_statement();
                return;
            };
            let value = expressions.item(value);
            let value_scope_start = expression_start(value);
            output.push_str("let ");
            js_scope_result_variable(output, value_scope_start);
            output.push_str(";\n");
            syntax_expression_to_js(
                output,
                type_aliases,
                expressions,
                patterns,
                checked_queries,
                checked_spread_records,
                local_variables,
                value,
                value_scope_start,
            );
            js_scope_result_variable(output, scope_start);
            output.push_str(" = { ");
            output.push_str(&name_to_lowercase_js(&name.value));
            output.push_str(": ");
            js_scope_result_variable(output, value_scope_start);
            output.push_str("};\n");
        }
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            js_scope_result_variable(output, scope_start);
            output.push_str(" = ($) => {\n");
            'in_function: {
                let mut parameter_introduced_variables = std::collections::HashMap::new();
                for syntax_parameter in parameters {
                    syntax_pattern_to_js_destructuring(
                        output,
                        syntax_parameter,
                        "$",
                        &mut parameter_introduced_variables,
                        checked_spread_records,
                        patterns,
                    );
                }
                let Some(result) = result else {
                    js_incomplete_statement();
                    break 'in_function;
                };
                let result = expressions.item(result);
                let result_scope_start = expression_start(result);
                output.push_str("let ");
                js_scope_result_variable(output, result_scope_start);
                output.push_str(";\n");
                syntax_expression_to_js(
                    output,
                    type_aliases,
                    expressions,
                    patterns,
                    checked_queries,
                    checked_spread_records,
                    &mut parameter_introduced_variables,
                    result,
                    result_scope_start,
                );
                output.push_str("return ");
                js_scope_result_variable(output, result_scope_start);
            }
            output.push_str(";\n};\n");
        }
        SyntaxExpression::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                output.push_str("let ");
                js_scope_result_variable(output, expression_start(part));
                output.push_str(";\n");
                syntax_expression_to_js(
                    output,
                    type_aliases,
                    expressions,
                    patterns,
                    checked_queries,
                    checked_spread_records,
                    local_variables,
                    part,
                    expression_start(part),
                );
                output.push('\n')
            }
            js_scope_result_variable(output, scope_start);
            output.push_str(" = [ ");
            for part in parts {
                js_scope_result_variable(output, expression_start(part));
                output.push_str(", ");
            }
            output.push_str("];\n");
        }
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    output.push_str("let ");
                    js_scope_result_variable(output, expression_start(value));
                    output.push_str(";\n");
                    syntax_expression_to_js(
                        output,
                        type_aliases,
                        expressions,
                        patterns,
                        checked_queries,
                        checked_spread_records,
                        local_variables,
                        value,
                        expression_start(value),
                    );
                    output.push('\n')
                }
            }
            js_scope_result_variable(output, scope_start);
            output.push_str(" = { ");
            for field in fields {
                if let Some(value) = &field.value {
                    output.push_str(&name_to_lowercase_js(&field.name.value));
                    output.push_str(": ");
                    js_scope_result_variable(output, expression_start(value));
                    output.push_str(", ");
                }
            }
            output.push_str("};\n");
        }
        SyntaxExpression::Commented {
            comments,
            expression,
        } => {
            for comment_line in std::iter::once(&comments.line0).chain(&comments.line1_up) {
                output.push_str("// ");
                output.push_str(&comment_line.value);
                output.push('\n');
            }
            match expression {
                None => {
                    js_incomplete_statement();
                }
                Some(expression) => {
                    syntax_expression_to_js(
                        output,
                        type_aliases,
                        expressions,
                        patterns,
                        checked_queries,
                        checked_spread_records,
                        local_variables,
                        expressions.item(expression),
                        scope_start,
                    );
                }
            }
        }

        SyntaxExpression::Destruct {
            equals_start: _,
            pattern,
            queried,
            result,
        } => {
            if let Some(queried) = queried {
                let queried = expressions.item(queried);
                let queried_scope_start = expression_start(queried);
                let queried_variable_name = {
                    let mut queried_variable_name = String::new();
                    js_scope_result_variable(&mut queried_variable_name, queried_scope_start);
                    queried_variable_name
                };
                output.push_str("let ");
                output.push_str(&queried_variable_name);
                output.push_str(";\n");
                syntax_expression_to_js(
                    output,
                    type_aliases,
                    expressions,
                    patterns,
                    checked_queries,
                    checked_spread_records,
                    local_variables,
                    queried,
                    queried_scope_start,
                );
                output.push_str(";\n");
                if let Some(pattern) = pattern {
                    syntax_pattern_to_js_destructuring(
                        output,
                        pattern,
                        &queried_variable_name,
                        local_variables,
                        checked_spread_records,
                        patterns,
                    );
                }
            }
            match result {
                None => {
                    js_incomplete_statement();
                }
                Some(result) => {
                    syntax_expression_to_js(
                        output,
                        type_aliases,
                        expressions,
                        patterns,
                        checked_queries,
                        checked_spread_records,
                        local_variables,
                        expressions.item(result),
                        scope_start,
                    );
                }
            }
        }
        SyntaxExpression::If {
            if_keyword_start,
            queried,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => {
            let Some(checked_query) = checked_queries.get(if_keyword_start) else {
                js_incomplete_statement();
                return;
            };
            let Some(queried) = queried else {
                js_incomplete_statement();
                return;
            };
            let queried = expressions.item(queried);
            let queried_scope_start = expression_start(queried);
            let queried_variable_name = {
                let mut queried_variable_name = String::new();
                js_scope_result_variable(&mut queried_variable_name, queried_scope_start);
                queried_variable_name
            };
            output.push_str("let ");
            output.push_str(&queried_variable_name);
            output.push_str(";\n");
            syntax_expression_to_js(
                output,
                type_aliases,
                expressions,
                patterns,
                checked_queries,
                checked_spread_records,
                local_variables,
                queried,
                queried_scope_start,
            );
            output.push_str(";\n");
            for (case_index, case) in cases.iter().enumerate() {
                if !checked_query.invalid_case_indexes.contains(&case_index) {
                    if cases.len() >= 2 {
                        output.push_str("if (");
                        syntax_pattern_to_js_matches_condition(
                            output,
                            &case.pattern,
                            &queried_variable_name,
                            patterns,
                        );
                        output.push_str(") {\n");
                    }
                    // can be optimized by not cloning if only one case exists
                    let mut case_local_variables = local_variables.clone();
                    syntax_pattern_to_js_destructuring(
                        output,
                        &case.pattern,
                        &queried_variable_name,
                        &mut case_local_variables,
                        checked_spread_records,
                        patterns,
                    );
                    match &case.result {
                        None => {
                            js_incomplete_statement();
                        }
                        Some(case_result) => {
                            syntax_expression_to_js(
                                output,
                                type_aliases,
                                expressions,
                                patterns,
                                checked_queries,
                                checked_spread_records,
                                &mut case_local_variables,
                                case_result,
                                scope_start,
                            );
                        }
                    }
                    if cases.len() >= 2 {
                        output.push_str("\n} else ");
                    }
                }
            }
            if cases.len() >= 2 {
                // in case none of the above patterns have matched, crash
                output.push_str("{\n");
                js_incomplete_statement();
                output.push('}');
            }
        }
    }
}
fn js_incomplete_statement() {
    // just don't set the result to anything
}
const record_empty_js_type_name: &str = "void";
const choice_empty_js_type_name: &str = "never";
fn name_to_uppercase_js(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_uppercase();
    }
    sanitized
}
fn name_to_uppercase_local_js(output: &mut String, name: &str) {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_uppercase();
    }
    output.push('$');
    output.push_str(&sanitized);
}
pub fn name_to_lowercase_js(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_lowercase();
    }
    if js_lowercase_keywords.contains(&sanitized.as_str()) {
        sanitized + "ø"
    } else {
        sanitized
    }
}
fn name_to_lowercase_local_js(output: &mut String, name: &str) {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_lowercase();
    }
    output.push('$');
    output.push_str(&sanitized);
}
fn js_scope_result_variable(output: &mut String, scope_start: lsp_types::Position) {
    use std::fmt::Write as _;
    output.push_str("$result$");
    _ = write!(output, "{}", scope_start.line);
    output.push('_');
    _ = write!(output, "{}", scope_start.character);
}
fn name_to_lowercase_local_js_introduced_at(
    output: &mut String,
    name: &str,
    introduced_start: lsp_types::Position,
) {
    use std::fmt::Write as _;
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_lowercase();
    }
    output.push_str(&sanitized);
    output.push('$');
    _ = write!(output, "{}", introduced_start.line);
    output.push('_');
    _ = write!(output, "{}", introduced_start.character);
}
fn origin_name_to_uppercase_js(output: &mut String, name: &str) {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_uppercase();
    }
    output.push_str(&sanitized);
    output.push_str("$origin");
}
/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#keywords
const js_lowercase_keywords: [&str; 70] = [
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "new",
    "null",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "let",
    "static",
    "yield",
    "await",
    "enum",
    "implements",
    "interface",
    "package",
    "private",
    "protected",
    "public",
    "abstract",
    "boolean",
    "byte",
    "char",
    "double",
    "final",
    "float",
    "goto",
    "int",
    "long",
    "native",
    "short",
    "synchronized",
    "throws",
    "transient",
    "volatile",
    "arguments",
    "as",
    "async",
    "eval",
    "from",
    "get",
    "of",
    "set",
];

#[derive(Clone, Debug)]
struct CheckedLocalVariable {
    origin_start: lsp_types::Position,
    type_: Option<Type>,
}
fn syntax_expression_check<'a, Expressions, Patterns>(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    expressions: &'a core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
    local_variables: &mut std::collections::HashMap<&'a Name, CheckedLocalVariable>,
    expression: &'a SyntaxExpression<Expressions, Patterns>,
    checked_calls: &mut std::collections::HashMap<lsp_types::Position, CheckedCall>,
    checked_local_fns: &mut std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &mut std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &mut std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> Option<Type> {
    match expression {
        SyntaxExpression::Number {
            value,
            type_: syntax_type,
        } => match syntax_type {
            None => {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: value.start,
                        end: position_add_characters(value.start, value.value.len() as u32),
                    },
                    message: Box::from("missing type after this number. Each number requires an explicit type to know its precision and range, like 0 u32 or 0 f32"),
                });
                None
            }
            Some(syntax_type) => {
                let Some(type_) = syntax_type_check(
                    syntax_type,
                    errors,
                    type_aliases,
                    records_used,
                    choices_used,
                ) else {
                    return None;
                };
                match &type_ {
                    Type::CoreConstruct { name, arguments: _ } => match name.as_str() {
                        "p32" => match value.value.parse::<std::num::NonZeroU32>() {
                            Ok(_) => Some(type_p32),
                            Err(parse_error) => {
                                errors.push(ErrorNode {
                                    range: lsp_types::Range {
                                        start: value.start,
                                        end: position_add_characters(
                                            value.start,
                                            value.value.len() as u32,
                                        ),
                                    },
                                    message: Box::from(format!(
                                        "number cannot be parsed as a p32: {parse_error}"
                                    )),
                                });
                                None
                            }
                        },
                        "u32" => match value.value.parse::<u32>() {
                            Ok(_) => Some(type_u32),
                            Err(parse_error) => {
                                errors.push(ErrorNode {
                                    range: lsp_types::Range {
                                        start: value.start,
                                        end: position_add_characters(
                                            value.start,
                                            value.value.len() as u32,
                                        ),
                                    },
                                    message: Box::from(format!(
                                        "number cannot be parsed as an u32: {parse_error}"
                                    )),
                                });
                                None
                            }
                        },
                        "i32" => match value.value.parse::<i32>() {
                            Ok(_) => Some(type_i32),
                            Err(parse_error) => {
                                errors.push(ErrorNode {
                                    range: lsp_types::Range {
                                        start: value.start,
                                        end: position_add_characters(
                                            value.start,
                                            value.value.len() as u32,
                                        ),
                                    },
                                    message: Box::from(format!(
                                        "number cannot be parsed as an i32: {parse_error}"
                                    )),
                                });
                                None
                            }
                        },
                        "f32" => match value.value.parse::<f32>() {
                            Ok(_) => Some(type_f32),
                            Err(parse_error) => {
                                errors.push(ErrorNode {
                                    range: lsp_types::Range {
                                        start: value.start,
                                        end: position_add_characters(
                                            value.start,
                                            value.value.len() as u32,
                                        ),
                                    },
                                    message: Box::from(format!(
                                        "number cannot be parsed as an f32: {parse_error}"
                                    )),
                                });
                                None
                            }
                        },
                        _ => {
                            errors.push(ErrorNode {
                                range: lsp_types::Range {
                                    start: value.start,
                                    end: position_add_characters(value.start, value.value.len() as u32),
                                },
                                message: Box::from("the type after this number is not a number type. The possible types are: p32 u32 i32 f32"),
                            });
                            None
                        }
                    },
                    _ => {
                        errors.push(ErrorNode {
                            range: lsp_types::Range {
                                start: value.start,
                                end: position_add_characters(value.start, value.value.len() as u32),
                            },
                            message: Box::from("the type after this number is not a number type. The possible types are: p32 u32 i32 f32"),
                        });
                        None
                    }
                }
            }
        },
        SyntaxExpression::Text {
            open_quote_start,
            content,
            content_end,
            closed_quote_exists,
            type_: syntax_type,
        } => {
            if content.is_empty() {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: *open_quote_start,
                        end: if *closed_quote_exists {
                            symbol_end(*content_end, "\"")
                        } else {
                            *content_end
                        },
                    },
                    message: Box::from("missing characters between the double quotes \"here\". There always needs to be at least one char, otherwise wrap it in an `opt`"),
                });
                return None;
            }
            if u32::try_from(content.len()).is_err() {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: *open_quote_start,
                        end: if *closed_quote_exists {
                            symbol_end(*content_end, "\"")
                        } else {
                            *content_end
                        },
                    },
                    message: Box::from("too many characters between the double quotes \"here\". A `str` in slug can only hold up to 2^32 bytes just like `buf` for example. Sorry! Open an issue if this limitation is blocking you"),
                });
                return None;
            }
            let Some(syntax_type) = syntax_type else {
                errors.push(ErrorNode {
                    range: lsp_types::Range {
                        start: *open_quote_start,
                        end: if *closed_quote_exists {
                            symbol_end(*content_end, "\"")
                        } else {
                            *content_end
                        },
                    },
                    message: Box::from("missing type after this quoted text. To represent a single unicode scalar, add type char. To represent a string of characters, add type str. An example of a valid string is \"Diego Schissi\" str"),
                });
                return None;
            };
            let Some(type_) = syntax_type_to_type(syntax_type, type_aliases) else {
                return None;
            };
            match &type_ {
                Type::CoreConstruct { name, arguments: _ } if name.as_str() == "str" => {
                    Some(type_str)
                }
                Type::CoreConstruct { name, arguments: _ } if name.as_str() == "char" => {
                    let mut chars = content.chars();
                    if chars.nth(1).is_some() {
                        errors.push(ErrorNode {
                            range: lsp_types::Range {
                                start: *open_quote_start,
                                end: if *closed_quote_exists {
                                    symbol_end(*content_end, "\"")
                                } else {
                                    *content_end
                                },
                            },
                            message: Box::from("There are multiple unicode scalars in this quoted text which means they don't fit into one char. Note that visual units like 🦸🏽‍♂️ are actually composed of multiple (5!) unicode scalars and what you're looking for may instead be a grapheme cluster, best represented as a simple str"),
                        });
                    }
                    Some(type_char)
                }
                _ => {
                    errors.push(ErrorNode {
                        range: type_range(syntax_type),
                        message: Box::from("unexpected type after quoted text. The possible types are: str (for multiple characters) or char (for a single unicode scalar). Every quoted text needs to be followed by one of these"),
                    });
                    Some(type_str)
                }
            }
        }
        SyntaxExpression::Variable(name) => {
            if let Some(variable_info) = local_variables.remove(&name.value) {
                let Some(variable_type) = variable_info.type_ else {
                    return None;
                };
                Some(variable_type)
            } else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: format!(
                        "No local variable in scope has this name. Hints:
- each variable can only be used once, even simple numbers etc.. To duplicate the value, use the helpers like U32-dup, Char-dup or create your own dup helpers
- a function name always starts uppercase
- a local function result can not refer to any variable from the outside. Otherwise check for typos.

Available variable names are {}",
                        local_variables.keys().map(|var| var.as_str()).collect::<Vec<_>>().join(", ")
                    ).into_boxed_str()
                });
                None
            }
        }
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments: syntax_type_arguments,
            open_paren_start: _,
            arguments_rest: rest_syntax_arguments,
            closed_paren_start: _,
        } => {
            // TODO handle name being local variable
            let Some(project_fn_info) = project_fns.get(name.value.as_str()) else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from("unknown function name. No fn in core or in this project has this name. Note that a local fn expression can not refer to any variable from the outside. Otherwise check for typos.")
                });
                return None;
            };
            let Some(project_fn_result_type) = &project_fn_info.result_type else {
                return None;
            };
            let type_argument_count = syntax_type_arguments
                .as_ref()
                .map(|args| args.types.len())
                .unwrap_or(0);
            if type_argument_count != project_fn_info.type_parameters.len() {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: format!("incorrect number of type parameters. The project fn has {parameter_count} type {parameter_pluralized}, but you provided {type_argument_count} as arguments.
Type arguments are provided each wrapped in curly braces after the fn name, like in Buf-empty{{u32}} origin.",
                        parameter_count = project_fn_info.type_parameters.len(),
                        parameter_pluralized = if project_fn_info.type_parameters.len() == 1 {
                            "parameter"
                        } else {
                            "parameters"
                        },
                    ).into_boxed_str()
                });
                return None;
            }
            let mut type_arguments = Vec::new();
            for syntax_type_argument in syntax_type_arguments.iter().flat_map(|args| &args.types) {
                let Some(type_argument) = syntax_type_check(
                    syntax_type_argument,
                    errors,
                    type_aliases,
                    records_used,
                    choices_used,
                ) else {
                    return None;
                };
                type_arguments.push(type_argument);
            }
            let mut expected_argument_types = project_fn_info.parameter_types.clone();
            let mut result_type = project_fn_result_type.clone();
            let mut type_variable_replacements = project_fn_info
                .type_parameters
                .iter()
                .zip(type_arguments)
                .map(|(type_parameter, type_argument)| (type_parameter.clone(), type_argument))
                .collect();
            let maybe_first_argument = first_argument_dot
                .as_ref()
                .map(|first_argument_dot| expressions.item(&first_argument_dot.argument));
            // TODO(important) compare actual to expected argument count and repeat extraneous and missing arguments
            for (expected_argument_type, syntax_argument) in
                expected_argument_types.iter_mut().flatten().zip(
                    maybe_first_argument
                        .into_iter()
                        .chain(rest_syntax_arguments),
                )
            {
                let Some(checked_argument_type) = syntax_expression_check(
                    errors,
                    type_aliases,
                    project_fns,
                    expressions,
                    patterns,
                    local_variables,
                    syntax_argument,
                    checked_calls,
                    checked_local_fns,
                    checked_queries,
                    checked_spread_records,
                    records_used,
                    choices_used,
                ) else {
                    return None;
                };
                {
                    let mut expected_argument_type = expected_argument_type.clone();
                    type_replace_variables(
                        &type_variable_replacements,
                        &mut expected_argument_type,
                    );
                    type_collect_variables_that_are_concrete_into(
                        &mut type_variable_replacements,
                        &expected_argument_type,
                        &checked_argument_type,
                    );
                }
                type_replace_variables(&type_variable_replacements, expected_argument_type);
                type_replace_variables(&type_variable_replacements, &mut result_type);
                if let Some(argument_variable_input_type_diff) =
                    type_diff(&expected_argument_type, &checked_argument_type)
                {
                    place_expression_type_diff_errors(
                        errors,
                        syntax_argument,
                        &argument_variable_input_type_diff,
                        expressions,
                        patterns,
                    );
                    return None;
                }
            }
            checked_calls.insert(
                name.start,
                CheckedCall {
                    argument_type_variable_replacements: type_variable_replacements,
                },
            );
            Some(result_type)
        }
        SyntaxExpression::Variant {
            open_bracket_start: tick_start,
            name,
            type_: other_variants_type,
            value,
            closed_bracket_start: _,
        } => {
            let Some(name) = name else {
                errors.push(ErrorNode {
                    range: symbol_range(*tick_start, "'"),
                    message: Box::from("missing variant name after this single quote '..here.. . An example of a variant is 'yes{'no .} \"hi c:\" str"),
                });
                return None;
            };
            let Some(syntax_type_argument) = other_variants_type else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from("missing type of the remaining variants in curly braces after this variant name. Examples of valid variants are 'yes{'no .} \"hi c:\" str and 'yes{Opt str} \"c:\" str.
If there should only ever by one variant, using a record with a single field is recommended over a single variant choice."),
                });
                return None;
            };
            let Some(syntax_type) = &syntax_type_argument.type_ else {
                errors.push(ErrorNode {
                    range: symbol_range(syntax_type_argument.open_angle_start, "{"),
                    message: Box::from("missing type argument in curly braces. Examples of valid variants are 'yes{'no .} \"hi c:\" str and 'yes{Opt str} \"c:\" str"),
                });
                return None;
            };
            let Some(value) = value else {
                errors.push(ErrorNode {
                    range: syntax_type_argument.closed_angle_start
                        .map(|closed_brace_start|symbol_range(closed_brace_start, "}"))
                        .unwrap_or_else(|| type_range(syntax_type)),
                    message: Box::from(
                        "this variant is missing an associated value after these curly braces. Every variant has a value, even if it's just . (an empy record).",
                    ),
                });
                return None;
            };
            let value = expressions.item(value);
            let Some(checked_type) = syntax_type_check(
                syntax_type,
                errors,
                type_aliases,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            let Some(checked_value_type) = syntax_expression_check(
                errors,
                type_aliases,
                project_fns,
                expressions,
                patterns,
                local_variables,
                value,
                checked_calls,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            let Type::Choice(origin_choice_type) = &checked_type else {
                let mut error_message: String = String::from(
                    "this variant type should be a choice (for example 'a u32 'b str  or  Opt u32) but it's\n",
                );
                type_format(&mut error_message, 0, &checked_type);
                error_message.push_str("\nMaybe you forgot to wrap it in a type like Opt?");
                errors.push(ErrorNode {
                    range: angled_type_argument_range(syntax_type_argument),
                    message: error_message.into_boxed_str(),
                });
                return None;
            };
            match origin_choice_type.iter().find_map(|variant| {
                if variant.name == name.value {
                    Some(&variant.value)
                } else {
                    None
                }
            }) {
                Some(expected_value_type) => {
                    if let Some(variant_value_type_diff) =
                        type_diff(expected_value_type, &checked_value_type)
                    {
                        place_expression_type_diff_errors(
                            errors,
                            value,
                            &variant_value_type_diff,
                            expressions,
                            patterns,
                        );
                        return None;
                    }
                    Some(checked_type)
                }
                None => Some(Type::Choice(
                    origin_choice_type
                        .iter()
                        .cloned()
                        .chain(std::iter::once(TypeVariant {
                            name: name.value.clone(),
                            value: checked_value_type,
                        }))
                        .collect(),
                )),
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start,
            open_paren_start,
            parameters,
            closed_paren_start: closed_bracket_start,
            result,
        } => {
            let mut parameter_introduced_variables: std::collections::HashMap<
                &Name,
                CheckedLocalVariable,
            > = std::collections::HashMap::new();
            let mut parameter_types = Vec::with_capacity(parameters.len());
            for parameter in parameters {
                let Some(checked_parmeter_type) = syntax_parameter_pattern_check(
                    parameter,
                    errors,
                    &mut parameter_introduced_variables,
                    type_aliases,
                    patterns,
                    &std::collections::HashMap::new(),
                    checked_spread_records,
                    records_used,
                    choices_used,
                ) else {
                    return None;
                };
                parameter_types.push(checked_parmeter_type);
            }
            let Some(result) = result else {
                errors.push(ErrorNode {
                    range: closed_bracket_start
                        .map(|closed_bracket_start| symbol_range(closed_bracket_start, ")"))
                        .or_else(|| {
                            open_paren_start
                                .map(|open_paren_start| symbol_range(open_paren_start, "("))
                        })
                        .unwrap_or_else(|| symbol_range(*fn_keyword_start, "fn")),
                    message: Box::from("missing function result after fn(..patterns..) here"),
                });
                return None;
            };
            let Some(checked_result_type) = syntax_expression_check(
                errors,
                type_aliases,
                project_fns,
                expressions,
                patterns,
                &mut parameter_introduced_variables,
                expressions.item(result),
                checked_calls,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            for (parameter_introduced_variable_name, parameter_introduced_variable_origin) in
                parameter_introduced_variables
            {
                errors.push(error_introduced_pattern_variable_is_unused(
                    parameter_introduced_variable_origin.origin_start,
                    parameter_introduced_variable_name,
                ));
            }
            checked_local_fns.insert(
                *fn_keyword_start,
                CheckedLocalFn {
                    parameter_types: parameter_types.clone(),
                    result_type: checked_result_type.clone(),
                },
            );
            Some(type_fn(parameter_types, checked_result_type))
        }
        SyntaxExpression::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            let maybe_part_types: Option<Vec<Type>> = parts
                .iter()
                .map(|part| {
                    syntax_expression_check(
                        errors,
                        type_aliases,
                        project_fns,
                        expressions,
                        patterns,
                        local_variables,
                        part,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        checked_spread_records,
                        records_used,
                        choices_used,
                    )
                })
                .collect();
            maybe_part_types.map(Type::Tuple)
        }
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            let mut maybe_field_types: Option<Vec<TypeField>> =
                Some(Vec::with_capacity(fields.len()));
            for field in fields {
                let checked_field_value_type: Option<Type> = match &field.value {
                    None => {
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(&field.name)),
                            message: Box::from(
                                "missing field value expression after this field name",
                            ),
                        });
                        None
                    }
                    Some(field_value) => syntax_expression_check(
                        errors,
                        type_aliases,
                        project_fns,
                        expressions,
                        patterns,
                        local_variables,
                        field_value,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        checked_spread_records,
                        records_used,
                        choices_used,
                    ),
                };
                if let Some(field_types) = &mut maybe_field_types {
                    match checked_field_value_type {
                        None => {
                            maybe_field_types = None;
                        }
                        Some(checked_field_value_type) => {
                            field_types.push(TypeField {
                                name: field.name.value.clone(),
                                value: checked_field_value_type,
                            });
                        }
                    }
                }
            }
            // the only way to use a record expression is through
            // a pattern or type. Nothing to add to used_records here
            maybe_field_types.map(Type::Record)
        }
        SyntaxExpression::Commented {
            comments,
            expression,
        } => match expression {
            None => {
                errors.push(ErrorNode {
                    range: symbol_range(comments.line0.start, "#"),
                    message: Box::from(
                        "missing expression after comments # your comment \\n ..here..",
                    ),
                });
                None
            }
            Some(expression) => syntax_expression_check(
                errors,
                type_aliases,
                project_fns,
                expressions,
                patterns,
                local_variables,
                expressions.item(expression),
                checked_calls,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ),
        },
        SyntaxExpression::Destruct {
            equals_start,
            pattern,
            queried,
            result,
        } => {
            let Some(queried) = queried else {
                errors.push(ErrorNode {
                    range: symbol_range(*equals_start, "="),
                    message: Box::from("missing queried expression after this if keyword. An example of a destruct is = { some x field y } function(z)  result-expression")
                });
                return None;
            };
            let queried = expressions.item(queried);
            let Some(checked_queried_type) = syntax_expression_check(
                errors,
                type_aliases,
                project_fns,
                expressions,
                patterns,
                local_variables,
                queried,
                checked_calls,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            let Some(pattern) = pattern else {
                errors.push(ErrorNode {
                    range: symbol_range(*equals_start, "="),
                    message: Box::from("missing pattern after this destruct =. An example of a destruct is = { some x field y } function(z)  result-expression")
                });
                return None;
            };
            let Some(result) = result else {
                errors.push(ErrorNode {
                    range: pattern_range(pattern, patterns),
                    message: Box::from("missing result expression after this destruct. An example of a destruct is = { some x field y } function(z)  result-expression")
                });
                return None;
            };
            let mut pattern_introduced_variables: std::collections::HashMap<
                &Name,
                CheckedLocalVariable,
            > = std::collections::HashMap::new();
            syntax_if_case_pattern_check(
                pattern,
                &checked_queried_type,
                errors,
                &mut pattern_introduced_variables,
                patterns,
                local_variables,
                checked_spread_records,
            );
            let mut remaining_specific_pattern_catch_possibilities = Vec::new();
            type_to_possible_specific_pattern_catches(
                &checked_queried_type,
                &mut remaining_specific_pattern_catch_possibilities,
            );
            specific_pattern_catches_remove_those_matched_by_pattern(
                &mut remaining_specific_pattern_catch_possibilities,
                pattern,
                patterns,
                checked_spread_records,
            );
            local_variables.extend(
                pattern_introduced_variables
                    .iter()
                    .map(|(binding, info)| (*binding, info.clone())),
            );
            let Some(checked_result_type) = syntax_expression_check(
                errors,
                type_aliases,
                project_fns,
                expressions,
                patterns,
                local_variables,
                expressions.item(result),
                checked_calls,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            for (case0_pattern_introduced_variable, case0_pattern_introduced_variable_origin) in
                pattern_introduced_variables
            {
                if local_variables
                    .remove(case0_pattern_introduced_variable)
                    .is_some()
                {
                    errors.push(error_introduced_pattern_variable_is_unused(
                        case0_pattern_introduced_variable_origin.origin_start,
                        case0_pattern_introduced_variable,
                    ));
                }
            }
            if !remaining_specific_pattern_catch_possibilities.is_empty() {
                let mut error_message = "inexhaustive destruct pattern. Missing cases:".to_string();
                for remaining_specific_pattern_catch_possibility in
                    &remaining_specific_pattern_catch_possibilities
                {
                    error_message.push_str("\n    - ");
                    specific_pattern_catch_format(
                        remaining_specific_pattern_catch_possibility,
                        &mut error_message,
                    );
                }
                errors.push(ErrorNode {
                    range: symbol_range(*equals_start, "="),
                    message: (error_message + "

A destruct pattern must cover all possible cases, otherwise the program would need to crash if such a value was matched on.
If you want to handle cases in separate branches, use an if like  if option { [yes n] n [no *] 0 u32 }"
                        ).into_boxed_str(),
                });
            }
            checked_queries.insert(
                *equals_start,
                CheckedQuery {
                    is_exhaustive: remaining_specific_pattern_catch_possibilities.is_empty(),
                    queried_type: checked_queried_type,
                    invalid_case_indexes: Vec::new(),
                },
            );
            Some(checked_result_type)
        }
        SyntaxExpression::If {
            if_keyword_start,
            queried,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => {
            let Some(queried) = queried else {
                errors.push(ErrorNode {
                    range: symbol_range(*if_keyword_start, "if"),
                    message: Box::from("missing queried expression after this if keyword. An example of a if is if option { [yes n] n [no *] 0 u32 }")
                });
                return None;
            };
            let queried = expressions.item(queried);
            let Some((case0, case1_up)) = cases.split_first() else {
                errors.push(ErrorNode {
                    range: symbol_range(*if_keyword_start, "if"),
                    message: Box::from("missing case(s) after the queried expression. Cases look like  pattern result-expression. An example of a if is if option { [yes n] n [no *] 0 u32 }")
                });
                return None;
            };
            let Some(checked_queried_type) = syntax_expression_check(
                errors,
                type_aliases,
                project_fns,
                expressions,
                patterns,
                local_variables,
                queried,
                checked_calls,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            let Some(case0_result) = &case0.result else {
                errors.push(ErrorNode {
                    range: pattern_range(&case0.pattern, patterns),
                    message: Box::from("missing result expression after this if case pattern. Cases look like  pattern result-expression. An example of a if is if option { [yes n] n [no *] 0 u32 }")
                });
                return None;
            };
            let mut case0_pattern_introduced_variables: std::collections::HashMap<
                &Name,
                CheckedLocalVariable,
            > = std::collections::HashMap::new();
            syntax_if_case_pattern_check(
                &case0.pattern,
                &checked_queried_type,
                errors,
                &mut case0_pattern_introduced_variables,
                patterns,
                local_variables,
                checked_spread_records,
            );
            let mut remaining_specific_pattern_catch_possibilities = Vec::new();
            type_to_possible_specific_pattern_catches(
                &checked_queried_type,
                &mut remaining_specific_pattern_catch_possibilities,
            );
            specific_pattern_catches_remove_those_matched_by_pattern(
                &mut remaining_specific_pattern_catch_possibilities,
                &case0.pattern,
                patterns,
                checked_spread_records,
            );

            let mut case0_result_local_variables = local_variables.clone();
            case0_result_local_variables.extend(
                case0_pattern_introduced_variables
                    .iter()
                    .map(|(binding, info)| (*binding, info.clone())),
            );
            let Some(checked_query_result_type) = syntax_expression_check(
                errors,
                type_aliases,
                project_fns,
                expressions,
                patterns,
                &mut case0_result_local_variables,
                case0_result,
                checked_calls,
                checked_local_fns,
                checked_queries,
                checked_spread_records,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            for (case0_pattern_introduced_variable, case0_pattern_introduced_variable_origin) in
                case0_pattern_introduced_variables
            {
                if case0_result_local_variables
                    .remove(case0_pattern_introduced_variable)
                    .is_some()
                {
                    errors.push(error_introduced_pattern_variable_is_unused(
                        case0_pattern_introduced_variable_origin.origin_start,
                        case0_pattern_introduced_variable,
                    ));
                }
            }
            let mut invalid_case_indexes = Vec::new();
            'checking_case1_up: for (case_index, case) in case1_up
                .iter()
                .enumerate()
                .map(|(i_in_1up, case)| (i_in_1up + 1, case))
            {
                let mut case_pattern_introduced_variables: std::collections::HashMap<
                    &Name,
                    CheckedLocalVariable,
                > = std::collections::HashMap::new();
                syntax_if_case_pattern_check(
                    &case.pattern,
                    &checked_queried_type,
                    errors,
                    &mut case_pattern_introduced_variables,
                    patterns,
                    local_variables,
                    checked_spread_records,
                );
                let remaining_specific_pattern_catch_possibilities_count_before_case =
                    remaining_specific_pattern_catch_possibilities.len();
                specific_pattern_catches_remove_those_matched_by_pattern(
                    &mut remaining_specific_pattern_catch_possibilities,
                    &case.pattern,
                    patterns,
                    checked_spread_records,
                );
                if remaining_specific_pattern_catch_possibilities.len()
                    == remaining_specific_pattern_catch_possibilities_count_before_case
                {
                    errors.push(ErrorNode {
                        range: pattern_range(&case.pattern, patterns),
                        message: Box::from("this if case pattern only covers possibilities that earlier case patterns will already catch. This means that any code in this case result won't ever be called. Remove this case or correct the pattern.")
                    });
                    invalid_case_indexes.push(case_index);
                    continue 'checking_case1_up;
                }
                let Some(case_result) = &case.result else {
                    errors.push(ErrorNode {
                        range: pattern_range(&case.pattern, patterns),
                        message: Box::from("missing result expression after this if case pattern. Cases are written as  pattern result-expression. An example of a if is if option { [yes n] n [no *] 0 u32 }")
                    });
                    continue 'checking_case1_up;
                };
                let mut case_local_variables = local_variables.clone();
                case_local_variables.extend(
                    case_pattern_introduced_variables
                        .iter()
                        .map(|(binding, info)| (*binding, info.clone())),
                );
                let Some(checked_case_result_type) = syntax_expression_check(
                    errors,
                    type_aliases,
                    project_fns,
                    expressions,
                    patterns,
                    &mut case_local_variables,
                    case_result,
                    checked_calls,
                    checked_local_fns,
                    checked_queries,
                    checked_spread_records,
                    records_used,
                    choices_used,
                ) else {
                    invalid_case_indexes.push(case_index);
                    continue 'checking_case1_up;
                };
                for (case_pattern_introduced_variable, case0_pattern_introduced_variable_origin) in
                    case_pattern_introduced_variables
                {
                    if case_local_variables
                        .remove(case_pattern_introduced_variable)
                        .is_some()
                    {
                        errors.push(error_introduced_pattern_variable_is_unused(
                            case0_pattern_introduced_variable_origin.origin_start,
                            case_pattern_introduced_variable,
                        ));
                    }
                }
                for (pattern_variable, pattern_variable_origin) in local_variables.iter() {
                    match (
                        case0_result_local_variables.contains_key(pattern_variable),
                        case_local_variables.contains_key(pattern_variable),
                    ) {
                        (false, false) => {
                            // both cases use this pattern variable
                        }
                        (true, true) => {
                            // neither case uses this pattern variable
                        }
                        (true, false) => {
                            // only this case uses this pattern variable, not the first case
                            errors.push(ErrorNode {
                                range: name_range(WithStartPosition { value: pattern_variable, start: pattern_variable_origin.origin_start }),
                                message: format!("this pattern variable is not used in the result of the first case starting at {}.
This is problematic because accidentally not handling a value in one branch could lead to leaked memory (or worse).
If you do not need to use this variable in that case, use any of the -rid functions to scrap it, like ? U32-rid your-variable [.] ..your existing case result..",
                                    position_to_string(expression_start(case0_result))
                                ).into_boxed_str()
                            });
                            // pretend the first case does use this pattern variable
                            // so that this variable is not reported as unused twice
                            case0_result_local_variables.remove(pattern_variable);
                        }
                        (false, true) => {
                            // only the first case uses this pattern variable, not this case
                            errors.push(ErrorNode {
                                range: name_range(WithStartPosition { value: pattern_variable, start: pattern_variable_origin.origin_start }),
                                message: format!(
                                    "this pattern variable is not used in the result of the {} case starting at {}.
This is problematic because accidentally not handling a value in one branch could lead to leaked memory (or worse).
If you do not need to use this variable in that case, use any of the -rid functions to scrap it, like ? U32-rid your-variable [.] ..your existing case result..",
                                    index_to_th(case_index),
                                    position_to_string(expression_start(case_result))
                                ).into_boxed_str()
                            });
                        }
                    }
                }
                if let Some(match_result_case_result_type_diff) =
                    type_diff(&checked_query_result_type, &checked_case_result_type)
                {
                    place_expression_type_diff_errors(
                        errors,
                        case_result,
                        &match_result_case_result_type_diff,
                        expressions,
                        patterns,
                    );
                    invalid_case_indexes.push(case_index);
                }
            }
            if !remaining_specific_pattern_catch_possibilities.is_empty()
                && invalid_case_indexes.is_empty()
            {
                let mut error_message = "inexhaustive if. Missing cases:".to_string();
                for remaining_specific_pattern_catch_possibility in
                    &remaining_specific_pattern_catch_possibilities
                {
                    error_message.push_str("\n    - [");
                    specific_pattern_catch_format(
                        remaining_specific_pattern_catch_possibility,
                        &mut error_message,
                    );
                    error_message.push(']');
                }
                errors.push(ErrorNode {
                    range: symbol_range(*if_keyword_start, "?"),
                    message: (error_message + "

A pattern match must cover all possible cases, otherwise the program would need to crash if such a value was matched on.
You might have forgotten to parenthesize the result of a case.
If not, add patterns for the cases above"
                        ).into_boxed_str(),
                });
            }
            *local_variables = case0_result_local_variables;
            checked_queries.insert(
                *if_keyword_start,
                CheckedQuery {
                    is_exhaustive: remaining_specific_pattern_catch_possibilities.is_empty(),
                    queried_type: checked_queried_type,
                    invalid_case_indexes: invalid_case_indexes,
                },
            );
            Some(checked_query_result_type)
        }
    }
}
fn error_introduced_pattern_variable_is_unused(
    origin_start: lsp_types::Position,
    binding_name: &Name,
) -> ErrorNode {
    ErrorNode {
        range: name_range(WithStartPosition {
            value: binding_name,
            start: origin_start,
        }),
        message: Box::from(
            "this patern variable is not used in the resulting expression. Use it or use any of the -rid functions to scrap it, like ? U32-rid your-variable [.] ..your existing case result..",
        ),
    }
}
pub struct CheckedCall {
    argument_type_variable_replacements: std::collections::BTreeMap<Name, Type>,
}
pub struct CheckedLocalFn {
    pub parameter_types: Vec<Type>,
    pub result_type: Type,
}
pub struct CheckedQuery {
    pub is_exhaustive: bool,
    pub queried_type: Type,
    pub invalid_case_indexes: Vec<usize>,
}

pub fn syntax_type_variables_into<'a>(
    type_variables: &mut std::collections::HashSet<&'a Name>,
    type_: &'a SyntaxType,
) {
    match type_ {
        SyntaxType::Variable(name) => {
            type_variables.insert(&name.value);
        }
        SyntaxType::Origin { .. } => {}
        SyntaxType::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            for parameter in parameters {
                syntax_type_variables_into(type_variables, parameter);
            }
            if let Some(result) = result {
                syntax_type_variables_into(type_variables, result);
            }
        }
        SyntaxType::ConstructWithArguments { name: _, arguments } => {
            if let Some(SyntaxTypeConstructParenthesizedArguments {
                open_paren_start: _,
                arguments,
                closed_paren_start: _,
            }) = arguments
            {
                for argument in arguments {
                    syntax_type_variables_into(type_variables, argument);
                }
            }
        }
        SyntaxType::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_type_variables_into(type_variables, part);
            }
        }
        SyntaxType::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_type_variables_into(type_variables, value);
                }
            }
        }
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => {
            for variant in variants {
                if let Some(value) = &variant.value {
                    syntax_type_variables_into(type_variables, value);
                }
            }
        }
    }
}
pub fn syntax_pattern_type_variables_into<'a, Patterns>(
    type_variables: &mut std::collections::HashSet<&'a Name>,
    pattern: &'a SyntaxPattern<Patterns>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match pattern {
        SyntaxPattern::Variable { name: _, type_ } => {
            if let Some(type_) = type_ {
                syntax_type_variables_into(type_variables, type_);
            }
        }
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_,
        } => {
            if let Some(type_) = type_ {
                syntax_type_variables_into(type_variables, type_);
            }
        }
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name: _,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(value) = value {
                syntax_pattern_type_variables_into(type_variables, patterns.item(value), patterns);
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_pattern_type_variables_into(type_variables, part, patterns);
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_pattern_type_variables_into(type_variables, value, patterns);
                }
            }
        }
    }
}

/// Fully validated type
#[derive(Clone, Debug)]
enum TypeDiff<'a> {
    /// None means no difference
    Conflict {
        expected: &'a Type,
        actual: &'a Type,
    },
    CoreConstruct {
        name: &'a Name,
        arguments: Vec<Option<TypeDiff<'a>>>,
    },
    Tuple(Vec<Option<TypeDiff<'a>>>),
    Record {
        fields: Vec<TypeDiffField<'a>>,
        actual_missing_fields: Vec<&'a Name>,
        actual_extraneous_fields: Vec<&'a Name>,
    },
    Choice(Vec<TypeDiffVariant<'a>>),
    Fn {
        parameters: Vec<Option<TypeDiff<'a>>>,
        result: Option<Box<TypeDiff<'a>>>,
    },
}
#[derive(Clone, Debug)]
struct TypeDiffField<'a> {
    name: &'a Name,
    value: Option<TypeDiff<'a>>,
}
#[derive(Clone, Debug)]
struct TypeDiffVariant<'a> {
    name: &'a Name,
    value: Option<TypeDiff<'a>>,
}

fn type_collect_variables_that_are_concrete_into(
    type_parameter_replacements: &mut std::collections::BTreeMap<Name, Type>,
    type_with_variables: &Type,
    concrete_type: &Type,
) {
    match type_with_variables {
        Type::Origin(_) => {}
        Type::Variable(variable_name) => {
            // if there is already a replacement for a variable name,
            // ignore concrete_type. If they are not equivalent,
            // this will be reported later
            type_parameter_replacements
                .entry(variable_name.clone())
                .or_insert_with(|| concrete_type.clone());
        }
        Type::CoreConstruct { name, arguments } => {
            if let Type::CoreConstruct {
                name: concrete_choice_type_construct_name,
                arguments: concrete_choice_type_construct_arguments,
            } = concrete_type
                && name == concrete_choice_type_construct_name
            {
                for (argument_type, concrete_argument_type) in arguments
                    .iter()
                    .zip(concrete_choice_type_construct_arguments.iter())
                {
                    type_collect_variables_that_are_concrete_into(
                        type_parameter_replacements,
                        argument_type,
                        concrete_argument_type,
                    );
                }
            }
        }
        Type::Fn { parameters, result } => {
            if let Type::Fn {
                parameters: concrete_parameters,
                result: concrete_result,
            } = concrete_type
            {
                for (argument_type, concrete_argument_type) in
                    parameters.iter().zip(concrete_parameters)
                {
                    type_collect_variables_that_are_concrete_into(
                        type_parameter_replacements,
                        argument_type,
                        concrete_argument_type,
                    );
                }
                type_collect_variables_that_are_concrete_into(
                    type_parameter_replacements,
                    result,
                    concrete_result,
                );
            }
        }
        Type::Tuple(parts) => {
            if let Type::Tuple(concrete_parts) = concrete_type {
                for (part, matching_concrete_part) in parts.iter().zip(concrete_parts) {
                    type_collect_variables_that_are_concrete_into(
                        type_parameter_replacements,
                        part,
                        matching_concrete_part,
                    );
                }
            }
        }
        Type::Record(fields) => {
            if let Type::Record(concrete_fields) = concrete_type {
                for field in fields {
                    if let Some(matching_concrete_field) = concrete_fields
                        .iter()
                        .find(|concrete_field| concrete_field.name == field.name)
                    {
                        type_collect_variables_that_are_concrete_into(
                            type_parameter_replacements,
                            &field.value,
                            &matching_concrete_field.value,
                        );
                    }
                }
            }
        }
        Type::Choice(variants) => {
            if let Type::Choice(concrete_variants) = concrete_type {
                for variant in variants {
                    if let Some(matching_concrete_variant) = concrete_variants
                        .iter()
                        .find(|concrete_variant| concrete_variant.name == variant.name)
                    {
                        type_collect_variables_that_are_concrete_into(
                            type_parameter_replacements,
                            &variant.value,
                            &matching_concrete_variant.value,
                        );
                    }
                }
            }
        }
    }
}

/// None means the types are equal
fn type_diff<'a>(expected_type: &'a Type, actual_type: &'a Type) -> Option<TypeDiff<'a>> {
    match expected_type {
        Type::Variable(expected_variable) => {
            if let Type::Variable(actual_variable) = actual_type
                && expected_variable == actual_variable
            {
                None
            } else {
                Some(TypeDiff::Conflict {
                    expected: expected_type,
                    actual: actual_type,
                })
            }
        }
        Type::CoreConstruct {
            name: expected_name,
            arguments: expected_arguments,
        } => {
            if let Type::CoreConstruct {
                name: actual_choice_type_construct_name,
                arguments: actual_choice_type_construct_arguments,
            } = actual_type
                && expected_name == actual_choice_type_construct_name
            {
                if expected_arguments
                    .iter()
                    .zip(actual_choice_type_construct_arguments.iter())
                    .all(|(expected_argument, actual_argument)| {
                        type_diff(expected_argument, actual_argument).is_none()
                    })
                {
                    return None;
                }
                Some(TypeDiff::CoreConstruct {
                    name: expected_name,
                    arguments: expected_arguments
                        .iter()
                        .zip(actual_choice_type_construct_arguments.iter())
                        .map(|(expected_argument, actual_argument)| {
                            type_diff(expected_argument, actual_argument)
                        })
                        .collect(),
                })
            } else {
                Some(TypeDiff::Conflict {
                    expected: expected_type,
                    actual: actual_type,
                })
            }
        }
        Type::Fn {
            parameters: expected_parameters,
            result: expected_result,
        } => {
            if let Type::Fn {
                parameters: actual_parameters,
                result: actual_result,
            } = actual_type
            {
                let result_type_diff = type_diff(expected_result, actual_result);
                if result_type_diff.is_none()
                    && expected_parameters
                        .iter()
                        .zip(actual_parameters)
                        .all(|(expected, actual)| type_diff(expected, actual).is_none())
                {
                    return None;
                }
                Some(TypeDiff::Fn {
                    parameters: expected_parameters
                        .iter()
                        .zip(actual_parameters.iter())
                        .map(|(expected, actual)| type_diff(expected, actual))
                        .collect(),
                    result: type_diff(expected_result, actual_result).map(Box::new),
                })
            } else {
                Some(TypeDiff::Conflict {
                    expected: expected_type,
                    actual: actual_type,
                })
            }
        }
        Type::Origin(expected_name) => {
            if let Type::Origin(actual_name) = actual_type
                && expected_name == actual_name
            {
                None
            } else {
                Some(TypeDiff::Conflict {
                    expected: expected_type,
                    actual: actual_type,
                })
            }
        }
        Type::Tuple(expected_parts) => {
            let Type::Tuple(actual_parts) = actual_type else {
                return Some(TypeDiff::Conflict {
                    expected: expected_type,
                    actual: actual_type,
                });
            };
            if expected_parts.len() != actual_parts.len() {
                return Some(TypeDiff::Conflict {
                    expected: expected_type,
                    actual: actual_type,
                });
            }
            Some(TypeDiff::Tuple(
                expected_parts
                    .iter()
                    .zip(actual_parts)
                    .map(|(expected, actual)| type_diff(expected, actual))
                    .collect(),
            ))
        }
        Type::Record(expected_fields) => {
            if let Type::Record(actual_fields) = actual_type {
                // all these filter(find) operations are quadratic.
                // should be fine since records usually do not contain many fields
                if expected_fields.len() == actual_fields.len()
                    && expected_fields.iter().all(|expected_field| {
                        actual_fields
                            .iter()
                            .find(|actual_field| actual_field.name == expected_field.name)
                            .is_some_and(|actual_field| {
                                type_diff(&expected_field.value, &actual_field.value).is_none()
                            })
                    })
                {
                    return None;
                }
                Some(TypeDiff::Record {
                    actual_missing_fields: expected_fields
                        .iter()
                        .filter(|expected_field| {
                            !actual_fields
                                .iter()
                                .any(|actual_field| actual_field.name == expected_field.name)
                        })
                        .map(|expected_field| &expected_field.name)
                        .collect(),
                    actual_extraneous_fields: actual_fields
                        .iter()
                        .filter(|expected_field| {
                            !expected_fields
                                .iter()
                                .any(|actual_field| actual_field.name == expected_field.name)
                        })
                        .map(|actual_field| &actual_field.name)
                        .collect(),
                    fields: expected_fields
                        .iter()
                        .filter_map(|expected_field| {
                            actual_fields
                                .iter()
                                .find(|actual_field| actual_field.name == expected_field.name)
                                .map(|actual_field| (expected_field, &actual_field.value))
                        })
                        .map(|(expected_field, actual_field_value)| TypeDiffField {
                            name: &expected_field.name,
                            value: type_diff(&expected_field.value, actual_field_value),
                        })
                        .collect(),
                })
            } else {
                Some(TypeDiff::Conflict {
                    expected: expected_type,
                    actual: actual_type,
                })
            }
        }
        Type::Choice(expected_variants) => {
            if let Type::Choice(actual_variants) = actual_type
                && expected_variants.len() == actual_variants.len()
                && expected_variants.iter().all(|expected_variant| {
                    actual_variants
                        .iter()
                        .any(|actual_variant| actual_variant.name == expected_variant.name)
                })
            {
                if expected_variants
                    .iter()
                    .filter_map(|expected_variant| {
                        actual_variants
                            .iter()
                            .find(|actual_variant| actual_variant.name == expected_variant.name)
                            .map(|actual_variant| (&expected_variant.value, &actual_variant.value))
                    })
                    .all(|(expected_variant_value, actual_variant_value)| {
                        type_diff(expected_variant_value, actual_variant_value).is_none()
                    })
                {
                    return None;
                }
                Some(TypeDiff::Choice(
                    expected_variants
                        .iter()
                        .filter_map(|expected_variant| {
                            actual_variants
                                .iter()
                                .find(|actual_variant| actual_variant.name == expected_variant.name)
                                .map(|actual_variant| (expected_variant, &actual_variant.value))
                        })
                        .map(|(expected_variant, actual_variant_value)| TypeDiffVariant {
                            name: &expected_variant.name,
                            value: type_diff(&expected_variant.value, actual_variant_value),
                        })
                        .collect(),
                ))
            } else {
                Some(TypeDiff::Conflict {
                    expected: expected_type,
                    actual: actual_type,
                })
            }
        }
    }
}
fn type_diff_error_message(type_diff: &TypeDiff) -> String {
    let mut builder: String = String::from("type mismatch:\n");
    type_diff_format(&mut builder, 0, type_diff);
    builder
}
fn type_diff_format(formatted: &mut String, indent: usize, type_diff: &TypeDiff) {
    match type_diff {
        TypeDiff::Conflict { expected, actual } => {
            formatted.push_str("expected:");
            space_or_linebreak_indented_into(
                formatted,
                type_line_span(expected),
                next_indent(indent),
            );
            type_format(formatted, next_indent(indent), expected);
            linebreak_indented_into(formatted, indent);
            formatted.push_str("actual:");
            space_or_linebreak_indented_into(
                formatted,
                type_line_span(actual),
                next_indent(indent),
            );
            type_format(formatted, next_indent(indent), actual);
        }
        TypeDiff::CoreConstruct { name, arguments } => {
            formatted.push_str(name);
            if let Some((argument0, argument1_up)) = arguments.split_first() {
                formatted.push('(');
                let line_span: LineSpan = type_diff_line_span(type_diff);
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                option_type_diff_format(formatted, next_indent(indent), argument0.as_ref());
                for argument in argument1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    option_type_diff_format(formatted, next_indent(indent), argument.as_ref());
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                formatted.push(')');
            }
        }
        TypeDiff::Fn { parameters, result } => {
            formatted.push_str("fn(");
            let line_span: LineSpan = type_diff_line_span(type_diff);
            if let Some((parameter0, parameter1_up)) = parameters.split_first() {
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                option_type_diff_format(
                    formatted,
                    next_indent(next_indent(indent)),
                    parameter0.as_ref(),
                );
                for parameter in parameter1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    option_type_diff_format(
                        formatted,
                        next_indent(next_indent(indent)),
                        parameter.as_ref(),
                    );
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                formatted.push(')');
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                option_type_diff_format(formatted, next_indent(indent), result.as_deref());
            }
        }
        TypeDiff::Tuple(parts) => {
            formatted.push('*');
            if let Some((part0, part1_up)) = parts.split_first() {
                formatted.push('(');
                let line_span: LineSpan = type_diff_line_span(type_diff);
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                option_type_diff_format(formatted, next_indent(indent), part0.as_ref());
                for part in part1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    option_type_diff_format(formatted, next_indent(indent), part.as_ref());
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                formatted.push(')');
            }
        }
        TypeDiff::Record {
            fields,
            actual_missing_fields,
            actual_extraneous_fields,
        } => {
            formatted.push('{');
            match fields.split_first() {
                None => {}
                Some((field0, field1_up)) => {
                    type_diff_field_format(formatted, indent, field0);
                    let line_span: LineSpan = if actual_missing_fields.is_empty()
                        && actual_extraneous_fields.is_empty()
                    {
                        type_diff_line_span(type_diff)
                    } else {
                        LineSpan::Multiple
                    };
                    for field in field1_up {
                        space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                        type_diff_field_format(formatted, indent, field);
                    }
                    space_or_linebreak_indented_into(formatted, line_span, indent);
                }
            }
            formatted.push('}');
            if !actual_missing_fields.is_empty() {
                linebreak_indented_into(formatted, indent);
                formatted.push_str("missing fields:");
                for actual_missing_field_name in actual_missing_fields {
                    formatted.push(' ');
                    formatted.push_str(actual_missing_field_name);
                }
            }
            if !actual_extraneous_fields.is_empty() {
                linebreak_indented_into(formatted, indent);
                formatted.push_str("extraneous fields:");
                for actual_extraneous_field_name in actual_extraneous_fields {
                    formatted.push(' ');
                    formatted.push_str(actual_extraneous_field_name);
                }
            }
        }
        TypeDiff::Choice(variants) => match variants.split_first() {
            None => {
                formatted.push('\'');
            }
            Some((variant0, variant1_up)) => {
                type_diff_variant_format(formatted, indent, variant0);
                let line_span: LineSpan = type_diff_line_span(type_diff);
                for variant in variant1_up {
                    space_or_linebreak_indented_into(formatted, line_span, indent);
                    type_diff_variant_format(formatted, indent, variant);
                }
            }
        },
    }
}
fn option_type_diff_format(formatted: &mut String, indent: usize, type_diff: Option<&TypeDiff>) {
    match type_diff {
        None => {
            formatted.push_str("(...)");
        }
        Some(type_diff) => type_diff_format(formatted, indent, type_diff),
    }
}
fn type_diff_field_format(formatted: &mut String, indent: usize, type_diff_field: &TypeDiffField) {
    formatted.push('.');
    formatted.push_str(type_diff_field.name);
    space_or_linebreak_indented_into(
        formatted,
        option_type_diff_line_span(type_diff_field.value.as_ref()),
        next_indent(indent),
    );
    option_type_diff_format(
        formatted,
        next_indent(indent),
        type_diff_field.value.as_ref(),
    );
}
fn type_diff_variant_format(
    formatted: &mut String,
    indent: usize,
    type_diff_variant: &TypeDiffVariant,
) {
    formatted.push('\'');
    formatted.push_str(type_diff_variant.name);
    space_or_linebreak_indented_into(
        formatted,
        option_type_diff_line_span(type_diff_variant.value.as_ref()),
        next_indent(indent),
    );
    option_type_diff_format(
        formatted,
        next_indent(indent),
        type_diff_variant.value.as_ref(),
    );
}
/// this is set to a low-seeming number because hover windows and similar
/// are often small and sometimes unresizable
const type_info_line_length_estimate_maximum: usize = 50;
fn type_diff_line_span(type_diff: &TypeDiff) -> LineSpan {
    if type_diff_length_estimate(type_diff) <= type_info_line_length_estimate_maximum {
        LineSpan::Single
    } else {
        LineSpan::Multiple
    }
}
fn option_type_diff_line_span(type_diff: Option<&TypeDiff>) -> LineSpan {
    match type_diff {
        Some(type_diff) => {
            if type_diff_length_estimate(type_diff) <= type_info_line_length_estimate_maximum {
                LineSpan::Single
            } else {
                LineSpan::Multiple
            }
        }
        None => LineSpan::Single,
    }
}
fn option_type_diff_length_estimate(type_diff: Option<&TypeDiff>) -> usize {
    match type_diff {
        None => 5,
        Some(type_diff) => type_diff_length_estimate(type_diff),
    }
}
fn type_diff_length_estimate(type_diff: &TypeDiff) -> usize {
    match type_diff {
        TypeDiff::Conflict { .. } => type_info_line_length_estimate_maximum + 1,
        TypeDiff::CoreConstruct { name, arguments } => {
            1 + name.len()
                + arguments
                    .iter()
                    .map(|argument| 2 + option_type_diff_length_estimate(argument.as_ref()))
                    .sum::<usize>()
        }
        TypeDiff::Fn { parameters, result } => {
            5 + parameters
                .iter()
                .map(|parameter| 1 + option_type_diff_length_estimate(parameter.as_ref()))
                .sum::<usize>()
                + option_type_diff_length_estimate(result.as_deref())
        }
        TypeDiff::Tuple(parts) => {
            3 + parts
                .iter()
                .map(|part| 1 + option_type_diff_length_estimate(part.as_ref()))
                .sum::<usize>()
        }
        TypeDiff::Record {
            fields,
            actual_missing_fields,
            actual_extraneous_fields,
        } => {
            if actual_missing_fields.is_empty() && actual_extraneous_fields.is_empty() {
                fields
                    .iter()
                    .map(|field| {
                        3 + field.name.len()
                            + option_type_diff_length_estimate(field.value.as_ref())
                    })
                    .sum()
            } else {
                type_info_line_length_estimate_maximum + 1
            }
        }
        TypeDiff::Choice(variants) => variants
            .iter()
            .map(|variant| {
                3 + variant.name.len() + option_type_diff_length_estimate(variant.value.as_ref())
            })
            .sum(),
    }
}

fn place_expression_type_diff_errors<Expressions, Patterns>(
    errors: &mut Vec<ErrorNode>,
    expression: &SyntaxExpression<Expressions, Patterns>,
    type_diff: &TypeDiff,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match expression {
        SyntaxExpression::Number { .. } => {
            errors.push(ErrorNode {
                range: expression_range(expression, expressions, patterns),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxExpression::Text { .. } => {
            errors.push(ErrorNode {
                range: expression_range(expression, expressions, patterns),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxExpression::Variable(_) => {
            errors.push(ErrorNode {
                range: expression_range(expression, expressions, patterns),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxExpression::Call { .. } => {
            errors.push(ErrorNode {
                range: expression_range(expression, expressions, patterns),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxExpression::Variant {
            open_bracket_start: _,
            type_: _,
            name,
            value,
            closed_bracket_start: _,
        } => match name {
            Some(name) => {
                let TypeDiff::Choice(variant_type_diffs) = type_diff else {
                    errors.push(ErrorNode {
                        range: expression_range(expression, expressions, patterns),
                        message: type_diff_error_message(type_diff).into_boxed_str(),
                    });
                    return;
                };
                match variant_type_diffs
                    .iter()
                    .find(|variant_type_diff| variant_type_diff.name == &name.value)
                {
                    Some(variant_type_diff) => {
                        if let Some(value) = value
                            && let Some(variant_value_type_diff) = &variant_type_diff.value
                        {
                            place_expression_type_diff_errors(
                                errors,
                                expressions.item(value),
                                variant_value_type_diff,
                                expressions,
                                patterns,
                            );
                        }
                    }
                    None => {
                        errors.push(ErrorNode {
                            range: expression_range(expression, expressions, patterns),
                            message: type_diff_error_message(type_diff).into_boxed_str(),
                        });
                    }
                }
            }
            None => {
                errors.push(ErrorNode {
                    range: expression_range(expression, expressions, patterns),
                    message: type_diff_error_message(type_diff).into_boxed_str(),
                });
            }
        },
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => match type_diff {
            TypeDiff::Fn {
                parameters: type_diff_parameters,
                result: type_diff_result,
            } => {
                for (parameter, type_diff_parameter) in parameters.iter().zip(type_diff_parameters)
                {
                    if let Some(type_diff_parameter) = type_diff_parameter {
                        place_pattern_type_diff_errors(
                            errors,
                            parameter,
                            type_diff_parameter,
                            patterns,
                        );
                    }
                }
                if let Some(type_diff_result) = type_diff_result
                    && let Some(result) = result
                {
                    place_expression_type_diff_errors(
                        errors,
                        expressions.item(result),
                        type_diff_result,
                        expressions,
                        patterns,
                    );
                }
            }
            _ => {
                errors.push(ErrorNode {
                    range: expression_range(expression, expressions, patterns),
                    message: type_diff_error_message(type_diff).into_boxed_str(),
                });
            }
        },
        SyntaxExpression::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            let TypeDiff::Tuple(type_diff_parts) = type_diff else {
                errors.push(ErrorNode {
                    range: expression_range(expression, expressions, patterns),
                    message: type_diff_error_message(type_diff).into_boxed_str(),
                });
                return;
            };
            for (part, type_diff_part) in parts.iter().zip(type_diff_parts) {
                if let Some(type_diff_part) = type_diff_part {
                    place_expression_type_diff_errors(
                        errors,
                        part,
                        type_diff_part,
                        expressions,
                        patterns,
                    );
                }
            }
        }
        SyntaxExpression::Record {
            open_curly_start,
            fields,
            closed_curly_start: _,
        } => match type_diff {
            TypeDiff::Record {
                fields: type_diff_fields,
                actual_extraneous_fields: _,
                actual_missing_fields,
            } => {
                if !actual_missing_fields.is_empty() {
                    errors.push(ErrorNode {
                        range: symbol_range(
                            *open_curly_start,
                            "{"
                        ),
                        message: format!(
                            "missing fields:{}. Those fields are included in the record type expected for this expression. Add them",
                            actual_missing_fields.iter().fold(String::new(), |s, f| s + " ." + f)
                        ).into_boxed_str(),
                    });
                }
                for field in fields {
                    if let Some(field_type_diff) = type_diff_fields
                        .iter()
                        .find(|field_type_diff| field.name.value == field_type_diff.name)
                        && let Some(value) = &field.value
                    {
                        if let Some(field_value_type_diff) = &field_type_diff.value {
                            place_expression_type_diff_errors(
                                errors,
                                value,
                                field_value_type_diff,
                                expressions,
                                patterns,
                            );
                        }
                    } else {
                        // actual_extraneous_fields.contains(&name_value)
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(&field.name)),
                            message: Box::from("extraneous field. This field is not included in the record type expected for this expression. Remove it"),
                        });
                    }
                }
            }
            _ => {
                errors.push(ErrorNode {
                    range: expression_range(expression, expressions, patterns),
                    message: type_diff_error_message(type_diff).into_boxed_str(),
                });
            }
        },

        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => {
            if let Some(after_comments) = expression {
                place_expression_type_diff_errors(
                    errors,
                    expressions.item(after_comments),
                    type_diff,
                    expressions,
                    patterns,
                );
            }
        }
        SyntaxExpression::Destruct {
            equals_start: _,
            pattern: _,
            queried: _,
            result,
        } => {
            if let Some(result) = result {
                place_expression_type_diff_errors(
                    errors,
                    expressions.item(result),
                    type_diff,
                    expressions,
                    patterns,
                );
            }
        }
        SyntaxExpression::If {
            if_keyword_start: _,
            queried: _,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => {
            if let Some(SyntaxExpressionIfCase {
                pattern: _,
                result: case_result,
            }) = cases.first()
                && let Some(case_result) = case_result
            {
                place_expression_type_diff_errors(
                    errors,
                    case_result,
                    type_diff,
                    expressions,
                    patterns,
                );
            }
        }
    }
}
fn place_pattern_type_diff_errors<Patterns>(
    errors: &mut Vec<ErrorNode>,
    pattern: &SyntaxPattern<Patterns>,
    type_diff: &TypeDiff,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match pattern {
        SyntaxPattern::Variable { .. } => {
            // TODO can be refined
            errors.push(ErrorNode {
                range: pattern_range(pattern, patterns),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxPattern::Ignored { .. } => {
            // TODO can be refined
            errors.push(ErrorNode {
                range: pattern_range(pattern, patterns),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => match name {
            Some(name) => match type_diff {
                TypeDiff::Choice(variant_type_diffs) => {
                    match variant_type_diffs
                        .iter()
                        .find(|variant_type_diff| variant_type_diff.name == &name.value)
                    {
                        Some(variant_type_diff) => {
                            if let Some(value) = value
                                && let Some(variant_value_type_diff) = &variant_type_diff.value
                            {
                                place_pattern_type_diff_errors(
                                    errors,
                                    patterns.item(value),
                                    variant_value_type_diff,
                                    patterns,
                                );
                            }
                        }
                        None => {
                            errors.push(ErrorNode {
                                range: pattern_range(pattern, patterns),
                                message: type_diff_error_message(type_diff).into_boxed_str(),
                            });
                        }
                    }
                }
                _ => {
                    errors.push(ErrorNode {
                        range: pattern_range(pattern, patterns),
                        message: type_diff_error_message(type_diff).into_boxed_str(),
                    });
                }
            },
            None => {
                errors.push(ErrorNode {
                    range: pattern_range(pattern, patterns),
                    message: type_diff_error_message(type_diff).into_boxed_str(),
                });
            }
        },
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            let TypeDiff::Tuple(type_diff_parts) = type_diff else {
                errors.push(ErrorNode {
                    range: pattern_range(pattern, patterns),
                    message: type_diff_error_message(type_diff).into_boxed_str(),
                });
                return;
            };
            for (part, type_diff_part) in parts.iter().zip(type_diff_parts) {
                if let Some(type_diff_part) = type_diff_part {
                    place_pattern_type_diff_errors(errors, part, type_diff_part, patterns);
                }
            }
        }
        SyntaxPattern::Record {
            open_curly_start,
            fields,
            closed_curly_start: _,
        } => match type_diff {
            TypeDiff::Record {
                fields: type_diff_fields,
                actual_missing_fields,
                actual_extraneous_fields: _,
            } => {
                if !actual_missing_fields.is_empty() {
                    errors.push(ErrorNode {
                        range: symbol_range(
                            *open_curly_start,
                            "{"
                        ),
                        message: format!(
                            "missing fields:{}. Those fields are included in the record type expected for this expression. Add them",
                            actual_missing_fields.iter().fold(String::new(), |s, f| s + " ." + f)
                        ).into_boxed_str(),
                    });
                }
                for field in fields {
                    if let Some(field_type_diff) = type_diff_fields
                        .iter()
                        .find(|field_type_diff| field.name.value == field_type_diff.name)
                        && let Some(value) = &field.value
                    {
                        if let Some(field_value_type_diff) = &field_type_diff.value {
                            place_pattern_type_diff_errors(
                                errors,
                                value,
                                field_value_type_diff,
                                patterns,
                            );
                        }
                    } else {
                        // actual_extraneous_fields.contains(&name_value)
                        errors.push(ErrorNode {
                            range: name_range(with_start_position_as_ref(&field.name)),
                            message: Box::from("extraneous field. This field is not included in the record type expected for this pattern. Remove it"),
                        });
                    }
                }
            }
            _ => {
                errors.push(ErrorNode {
                    range: pattern_range(pattern, patterns),
                    message: type_diff_error_message(type_diff).into_boxed_str(),
                });
            }
        },
    }
}

fn name_toggle_case_of_first_ascii(mut name: String) -> String {
    if let Some(first) = name.chars().next()
        && let Some(first_mut) = name.get_mut(0..=0)
    {
        if first.is_ascii_uppercase() {
            first_mut.make_ascii_lowercase();
        } else {
            first_mut.make_ascii_uppercase();
        }
    }
    name
}

const local_unnamed_function_name: &str = "local_unnamed_function";

fn sorted_field_names<'a>(field_names: impl Iterator<Item = &'a Name>) -> Vec<Name> {
    let mut field_names_vec: Vec<Name> = field_names.map(Name::clone).collect();
    field_names_vec.sort_unstable();
    field_names_vec
}

fn sorted_variant_names<'a>(variant_names: impl Iterator<Item = &'a Name>) -> Vec<Name> {
    let mut variant_names_vec: Vec<Name> = variant_names.map(Name::clone).collect();
    variant_names_vec.sort_unstable();
    variant_names_vec
}

const fn type_variable(name: &'static str) -> Type {
    // TODO to upper or crash
    Type::Variable(Name::from_static(name))
}
fn type_tuple(fields: impl IntoIterator<Item = Type>) -> Type {
    Type::Tuple(fields.into_iter().collect())
}
fn type_record(fields: impl IntoIterator<Item = (&'static str, Type)>) -> Type {
    Type::Record(
        fields
            .into_iter()
            .map(|(field_name, field_value)| TypeField {
                name: Name::from_static(field_name),
                value: field_value,
            })
            .collect(),
    )
}
/// preferred type to hold no information
const type_empty: Type = type_tuple_empty;
const type_tuple_empty: Type = Type::Tuple(vec![]);
const type_choice_empty: Type = Type::Choice(vec![]);
fn type_choice(variants: impl IntoIterator<Item = (&'static str, Type)>) -> Type {
    Type::Choice(
        variants
            .into_iter()
            .map(|(variant_name, variant_value)| TypeVariant {
                name: Name::from_static(variant_name),
                value: variant_value,
            })
            .collect(),
    )
}
const type_p32: Type = Type::CoreConstruct {
    name: Name::from_static("p32"),
    arguments: vec![],
};
const type_u32: Type = Type::CoreConstruct {
    name: Name::from_static("u32"),
    arguments: vec![],
};
const type_i32: Type = Type::CoreConstruct {
    name: Name::from_static("i32"),
    arguments: vec![],
};
const type_f32: Type = Type::CoreConstruct {
    name: Name::from_static("f32"),
    arguments: vec![],
};
const type_char: Type = Type::CoreConstruct {
    name: Name::from_static("char"),
    arguments: vec![],
};
const type_str: Type = Type::CoreConstruct {
    name: Name::from_static("str"),
    arguments: vec![],
};

fn type_opt(yes: Type) -> Type {
    type_choice([("no", type_empty), ("yes", yes)])
}
fn type_order() -> Type {
    Type::Choice(vec![
        TypeVariant {
            name: Name::from_static("less"),
            value: type_empty,
        },
        TypeVariant {
            name: Name::from_static("equal"),
            value: type_empty,
        },
        TypeVariant {
            name: Name::from_static("greater"),
            value: type_empty,
        },
    ])
}
fn type_fn(in_: Vec<Type>, out: Type) -> Type {
    Type::Fn {
        parameters: in_,
        result: Box::new(out),
    }
}
fn type_buf(origin: Type, item: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("buf"),
        arguments: vec![origin, item],
    }
}
fn type_slot(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("slot"),
        arguments: vec![origin],
    }
}
fn type_span(origin: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("span"),
        arguments: vec![origin],
    }
}
fn type_unset_slice(item: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static("unset-slice"),
        arguments: vec![item],
    }
}
struct CoreFnInfo {
    name: &'static str,
    documentation: &'static str,
    type_parameters: Vec<Name>,
    parameters: Vec<Type>,
    result: Type,
}
pub static core_fns: std::sync::LazyLock<std::collections::HashMap<Name, CheckedProjectFn>> =
    std::sync::LazyLock::new(|| {
        std::collections::HashMap::from([
            CoreFnInfo {
                name: "p32-to-u32",
                documentation: "Convert to u32",
                type_parameters: vec![],
                parameters:  vec![type_p32],
                result: type_u32,
            },
            CoreFnInfo {
                name: "p32-add-clamp",
                documentation: "Saturating a + b",
                type_parameters: vec![],
                parameters: vec![type_p32, type_u32],
                result: type_p32,
            },
            CoreFnInfo {
                name: "p32-mul-clamp",
                documentation: "Saturating a * b",
                type_parameters: vec![],
                parameters: vec![type_p32, type_u32],
                result: type_p32,
            },
            CoreFnInfo {
                name: "p32-order",
                documentation: "Compare left to right. For example 'less means left < right",
                type_parameters: vec![],
                parameters: vec![type_p32, type_p32],
                result: type_order(),
            },
            CoreFnInfo {
                name: "u32-to-p32",
                documentation: "Convert to a p32. If 0, returns 'no .",
                type_parameters: vec![],
                parameters: vec![type_u32],
                result: type_opt(type_p32),
            },
            CoreFnInfo {
                name: "u32-successor-clamp",
                documentation: "Saturating n + 1, returning a p32",
                type_parameters: vec![],
                parameters: vec![type_u32],
                result: type_p32,
            },
            CoreFnInfo {
                name: "u32-to-i32-clamp",
                documentation: "Saturating cast to fit it into an i32",
                type_parameters: vec![],
                parameters: vec![type_u32],
                result: type_p32,
            },
            CoreFnInfo {
                name: "u32-round-to-nearest-f32-else-even",
                documentation: "Lossy cast to fit it into an f32.
Chooses the closest f32 representation, breaking exact ties towards the even significand (\"mantissa\")",
                type_parameters: vec![],
                parameters: vec![type_u32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "u32-add-clamp",
                documentation: "Saturating a + b",
                type_parameters: vec![],
                parameters: vec![type_u32, type_u32],
                result: type_u32,
            },
            CoreFnInfo {
                name: "u32-add-i32-clamp",
                documentation: "Saturating u + i",
                type_parameters: vec![],
                parameters: vec![type_u32, type_i32],
                result: type_u32,
            },
            CoreFnInfo {
                name: "u32-mul-clamp",
                documentation: "Saturating a * b",
                type_parameters: vec![],
                parameters: vec![type_u32, type_u32],
                result: type_u32,
            },
            CoreFnInfo {
                name: "u32-mul-wrap",
                documentation: "a * b, wrapping around from 0 on overflow (like the remainder of (a * b) / 2^32).
Prefer over U32-mul-clamp if you explicitly want this overflow behavior
or performance is critical.",
                type_parameters: vec![],
                parameters: vec![type_u32, type_u32],
                result: type_u32,
            },
            CoreFnInfo {
                name: "u32-pow-clamp",
                documentation: "Saturating base ^ exponent",
                type_parameters: vec![],
                parameters: vec![type_u32, type_p32],
                result: type_u32,
            },
            CoreFnInfo {
                name: "u32-order",
                documentation: "Compare left to right. For example 'less means left < right",
                type_parameters: vec![],
                parameters: vec![type_u32, type_u32],
                result: type_order(),
            },
            CoreFnInfo {
                name: "i32-to-u32",
                documentation: "Convert to an u32. If negative, returns 'no .",
                type_parameters: vec![],
                parameters: vec![type_i32],
                result: type_opt(type_u32),
            },
            CoreFnInfo {
                name: "i32-round-to-nearest-f32-else-even",
                documentation: "Lossy cast to fit it into an f32.
Chooses the closest f32 representation, breaking exact ties towards the even significand (\"mantissa\")",
                type_parameters: vec![],
                parameters: vec![type_i32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "i32-abs-to-u32",
                documentation: "If negative, negate, resulting in a u32",
                type_parameters: vec![],
                parameters: vec![type_i32],
                result: type_opt(type_u32),
            },
            CoreFnInfo {
                name: "i32-negate-clamp",
                documentation: "Saturating negation -n",
                type_parameters: vec![],
                parameters: vec![type_i32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "i32-add-clamp",
                documentation: "Saturating a + b",
                type_parameters: vec![],
                parameters: vec![type_i32, type_i32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "i32-mul-clamp",
                documentation: "Saturating a * b",
                type_parameters: vec![],
                parameters: vec![type_i32, type_i32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "i32-pow-clamp",
                documentation: "Saturating a ^ b",
                type_parameters: vec![],
                parameters: vec![type_i32, type_p32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "i32-order",
                documentation: "Compare left to right. For example 'less means left < right",
                type_parameters: vec![],
                parameters: vec![type_i32, type_i32],
                result: type_order(),
            },
            CoreFnInfo {
                name: "f32-pi",
                documentation: "Archimedes’ constant π",
                type_parameters: vec![],
                parameters:  vec![],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-abs",
                documentation: "Set its sign to positive",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-negate",
                documentation: "Flip its sign",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-ln",
                documentation: "Its natural logarithm.
If the result is too negative or the input is not positive, returns 'no .

Warning: Precision is unspecified and as such this function should never be used when determinism is desired.",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_opt(type_f32),
            },
            CoreFnInfo {
                name: "f32-exp",
                documentation: "e to the power of the given f32, known as the exponential function.

Warning: Precision is unspecified and as such this function should never be used when determinism is desired.",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-sin",
                documentation: "The sine of given radians.

Warning: Precision is unspecified and as such this function should never be used when determinism is desired.",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-cos",
                documentation: "The cosine of given radians.

Warning: Precision is unspecified and as such this function should never be used when determinism is desired.",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-tan",
                documentation: "The tangent of given radians.

Warning: Precision is unspecified and as such this function should never be used when determinism is desired.",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-atan",
                documentation: "The arctangent of given radians, returned in radians from -pi/2 to pi/2.

Warning: Precision is unspecified and as such this function should never be used when determinism is desired.",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-add-clamp",
                documentation: "Saturating a + b",
                type_parameters: vec![],
                parameters: vec![type_f32, type_f32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-mul-clamp",
                documentation: "Saturating a * b",
                type_parameters: vec![],
                parameters: vec![type_f32, type_f32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-div-clamp",
                documentation: "Saturating n / by.
Try not to divide by 0.0, as 0.0 will be returned which is not mathematically correct. This behaviour is consistent with gleam, pony, coq, lean.",
                type_parameters: vec![],
                parameters: vec![type_f32, type_f32],
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-square-root",
                documentation: "n ^ 0.5, returning 'no . when n < 0.0",
                type_parameters: vec![],
                parameters:  vec![type_i32],
                result: type_opt(type_f32),
            },
            CoreFnInfo {
                name: "f32-pow-i32",
                documentation: "a ^ b, returning 'no . when the result is too large, too negative or undefined.

Warning: Precision is unspecified and as such this function should never be used when determinism is desired.",
                type_parameters: vec![],
                parameters: vec![type_f32, type_i32],
                result: type_opt(type_f32),
            },
            CoreFnInfo {
                name: "f32-pow",
                documentation: "a ^ b, returning 'no . when the result is too large, too negative or undefined.

Warning: Precision is unspecified and as such this function should never be used when determinism is desired.
As such, prefer F32-square-root if you can",
                type_parameters: vec![],
                parameters: vec![type_f32, type_f32],
                result: type_opt(type_f32),
            },
            CoreFnInfo {
                name: "f32-round-nearest-else-away-from-0",
                documentation: "If not already equal to an integer value, find the closest neighboring integer.
Round midpoint of a negative number to the lower neighbor
and round midpoint of a positive number to the higher neighbor.
This is (I think) often the semantically correct mode for geometry, ui and similar use cases
where behavior should be the same everywhere and e.g adding 1 should not change it.
It's the default round operation implementation in e.g. [C](https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf#page=271), [rust](https://doc.rust-lang.org/std/primitive.f32.html#method.round), [zig](https://ziglang.org/documentation/master/#round), ([llvm](https://llvm.org/docs/LangRef.html#llvm-round-intrinsic)), python 2.
However, it's slower than `F32-round-nearest-else-even` on most architectures.
```slug
fn Age . : f32 =
    F32-round-nearest-else-away-from-0 68.8 f32
```",
                type_parameters: vec![],
                parameters: vec![type_f32] ,
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-nearest-else-even",
                documentation: r#"If not already equal to an integer value, find the closest neighboring integer.
Round negative midpoint to the nearest even neighbor.
This is sometimes called banker's rounding and is well-supported by architectures.
An argument could be made that this is more "fair" than `F32-round-nearest-else-away-from-0`
when operating in a linear scale where numbers are distributed evenly (even numbers as likely as uneven in all cases)
but as a result it can feel less predictable.
If fairness is a real concern, the midpoint should be explicitly handled, e.g.
by ignoring midpoint values, counting midpoint values or actually [randomizing their outcome](https://en.wikipedia.org/wiki/Rounding#Randomized_rounding_to_an_integer).
It's the default round operation implementation in e.g. python 3, dotnet, haskell, [erlang](https://erlangcentral.org/wiki/index.php?title=Floating_Point_Rounding), lisp"#,
                type_parameters: vec![],
                parameters: vec![type_f32] ,
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-up",
                documentation: "If not already equal to an integer value, find the closest greater (not absolute greater) neighboring integer.
Often called ceiling",
                type_parameters: vec![],
                parameters: vec![type_f32] ,
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-down",
                documentation: "If not already equal to an integer value, find the closest smaller (not absolute smaller) neighboring integer.
Often called floor",
                type_parameters: vec![],
                parameters: vec![type_f32] ,
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-toward-0",
                documentation: "If not already equal to an integer value, find the closest neighboring integer with a smaller absolute value.
Often called truncate",
                type_parameters: vec![],
                parameters: vec![type_f32] ,
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-away-from-0",
                documentation: "If not already equal to an integer value, find the closest neighboring integer with a greater absolute value",
                type_parameters: vec![],
                parameters: vec![type_f32] ,
                result: type_f32,
            },
            CoreFnInfo {
                name: "f32-round-up-to-i32-clamp",
                documentation: "`F32-round-up`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-down-to-i32-clamp",
                documentation: "`F32-round-down`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-toward-0-to-i32-clamp",
                documentation: "`F32-round-toward-0`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-away-from-0-to-i32-clamp",
                documentation: "`F32-round-away-from-0`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-nearest-else-away-from-0-to-i32-clamp",
                documentation: "`F32-round-nearest-else-away-from-0`, then clamp to within 32 bits.
In effect, `F32-round-toward-0-to-i32-clamp` truncates off at the decimal point:
```slug
fn Age . : f32 =
    F32-round-toward-0-to-i32-clamp 68.8 f32
```",
                type_parameters: vec![],
                parameters: vec![type_f32] ,
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-nearest-else-even-to-i32-clamp",
                documentation: "`F32-round-nearest-else-even`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-order",
                documentation: "Compare left to right. For example 'less means left < right",
                type_parameters: vec![],
                parameters: vec![type_f32, type_f32],
                result: type_order(),
            },
            CoreFnInfo {
                name: "char-to-u32",
                documentation: "Its code as a u32",
                type_parameters: vec![],
                parameters: vec![type_char],
                result: type_u32,
            },
            CoreFnInfo {
                name: "str-utf8-length",
                documentation: "Its number of bytes when encoded as UTF-8.
You usually want to use the slower Str-char-count.",
                type_parameters: vec![],
                parameters: vec![type_str],
                result: type_p32,
            },
            CoreFnInfo {
                name: "str-char-count",
                documentation: "Its number of unicode codepoints aka chars",
                type_parameters: vec![],
                parameters: vec![type_str],
                result: type_p32,
            },
            CoreFnInfo {
                name: "str-start",
                documentation: "Split off its first char",
                type_parameters: vec![],
                parameters: vec![type_str],
                result: type_record([("start", type_char), ("after", type_opt(type_str))]),
            },
            CoreFnInfo {
                name: "str-end",
                documentation: "Split off its last char",
                type_parameters: vec![],
                parameters: vec![type_str],
                result: type_record([("end", type_char), ("before", type_opt(type_str))]),
            },
            CoreFnInfo {
                name: "Choice-empty-to",
                documentation: r#"The empty choice type (`'`) is a weird one.
No value of this type can ever exist and it's only use is some type trickery.
You may already know it under one of these names:
- never
- noreturn
- unreachable
- uninhabited
- bottom (⊥) or initial object
- impossible or more rarely (and incorrectly) infallible

For example, values of type `'success u32 'failure '` can be passed to any function expecting
a success and failure variant to be possible.
But the interesting thing about having a value of this type is that since the type knows
the failure variant could never have been created, we can safely unwrap it!
```slug
fn Tried-unwrap tried 'success _ok 'failure ' : _ok =
    ? tried
    ['success ok] ok
    ['failure impossible] Choice-empty-to{_ok} impossible
```
You really can ask for any type of data, like emulating a dup or rid operation
```slug
fn Choice-empty-rid choice-empty ' : . =
    Choice-empty-to{.} choice-empty
fn Choice-empty-dup choice-empty ' : .a ' .b ' =
    Choice-empty-to{.a ' .b '} choice-empty
```
Another nice example: Creating a value whose only purpose is transporting type info:
```slug
ty Type-only _type
    # enables what is often called "phantom types"
    'a .
    'unconstructable .type _type .impossible '

fn Type-only-rid type-only Type-only _type : . =
    ? type-only
    ['a .] .
    ['unconstructable .type type .impossible imp]
        # quite funny: To get rid of the type value
        # which can never exist here, we use the imp value
        # to pull us a generic _type rid function from the ether
        Call .fn Choice-empty-to{Fn _type, .} imp .in type

# usage example

ty Weak-slot _origin
    # Alternative to using a plain u32 which is tiny bit harder to abuse.
    # Can still point out of bounds,
    # alias another Weak-slot or be used for the wrong span.
    # As a result, access will always return Opt and should be checked with care
    .origin Type-only _origin
    .index u32

fn Index-to-weak-slot{_origin} index u32 : Weak-slot _origin =
    .index index .origin 'of{Type-only _origin} .

fn Span-start-weak
    span Span _origin
: .weak Weak-slot _origin .slot Slot _origin =
    ? Span-start-index slot [.span span .index index]
    .span span
    .weak Index-to-weak-slot{_origin} index
```
This `Type-only` pattern is very easy to overuse,
please limit your creativity for the rest of us mortals."#,
                type_parameters: vec![Name::from_static("result")],
                parameters: vec![type_choice_empty],
                result: type_variable("result")
            },
            CoreFnInfo {
                name: "slot-index",
                documentation: "At what offset the referenced item is located.
For example, the second item in a Buf is at index 1",
                type_parameters: vec![],
                parameters: vec![type_slot(type_variable("Origin"))],
                result: type_tuple([type_slot(type_variable("Origin")), type_u32])
            },
            CoreFnInfo {
                name: "slot-to-span",
                documentation: "Create a span covering just the one given slot",
                type_parameters: vec![],
                parameters: vec![type_slot(type_variable("Origin"))],
                result: type_span(type_variable("Origin")),
            },
            CoreFnInfo {
                name: "span-start-index",
                documentation: "At what offset the first contained slot is located.
For example, the second item in a Buf is at index 1",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"))],
                result: type_tuple([type_span(type_variable("Origin")), type_u32]),
            },
            CoreFnInfo {
                name: "span-length",
                documentation: "How many slots it spans",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"))],
                result: type_tuple([type_span(type_variable("Origin")), type_p32])
            },
            CoreFnInfo {
                name: "opt-span-length",
                documentation: "How many slots it spans",
                type_parameters: vec![],
                parameters: vec![type_opt(type_span(type_variable("Origin")))],
                result: type_tuple([type_opt(type_span(type_variable("Origin"))), type_u32])
            },
            CoreFnInfo {
                name: "span-start",
                documentation: "Split into the first slot and span after.
To join disconnected slots and spans back together, use helpers like `buf-span-add-own-opt-span`.
Note that to only read the start index you can directly call `span-start-index`",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"))],
                result: type_tuple([
                    type_slot(type_variable("Origin")),
                    type_opt(type_span(type_variable("Origin")))
                ]),
            },
            CoreFnInfo {
                name: "span-end",
                documentation: "Split into the last slot and span before",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"))],
                result: type_tuple([
                    type_opt(type_span(type_variable("Origin"))),
                    type_slot(type_variable("Origin")),
                ]),
            },
            CoreFnInfo {
                name: "span-start-of-length-positive",
                documentation: "Split after a given length with the start half known to have positive length.
If the length is greater than the given span's length, .start will be the existing span and .after will be empty.
```slug
fn Span-slot-at(span span(Origin) index u32)
    {
    before opt(span(Origin))
    at slot(Origin)
    after opt(span(Origin))
    }
    = *(until after)
        span-start-of-length-positive(span p32-add(1 p32 index))
    = *(before at) span-end(until)
    {before before at at after after}
```
See also `span-end-of-length-positive`, `span-start`.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_p32,
                ],
                result: type_tuple([
                    type_span(type_variable("Origin")),
                    type_opt(type_span(type_variable("Origin")))
                ]),
            },
            CoreFnInfo {
                name: "span-end-of-length-positive",
                documentation: "Split before a given length from the end with the end half known to have positive length.
If the length is greater than the given span's length, .end will be the existing span and .before will be empty.
See also `span-start-of-length-positive`, `span-end`.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_p32,
                ],
                result: type_tuple([
                    type_opt(type_span(type_variable("Origin"))),
                    type_span(type_variable("Origin")),
                ]),
            },
            CoreFnInfo {
                name: "span-step",
                documentation: "Step through all slots, updating the given initial state for each taken slot in line.
A less powerful version that preserves the Span is `buf-span-step`",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_choice([("up", type_empty), ("down", type_empty)]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_slot(type_variable("Origin")),
                        ],
                        type_variable("State")
                    )
                ],
                result: type_variable("State"),
            },
            CoreFnInfo {
                name: "opt-span-step",
                documentation: "Step through all slots, updating the given initial state for each taken slot in line.
A less powerful version that preserves the Span is `buf-opt-span-step`",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_choice([("up", type_empty), ("down", type_empty)]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_slot(type_variable("Origin")),
                        ],
                        type_variable("State")
                    )
                ],
                result: type_variable("State"),
            },
            CoreFnInfo {
                name: "span-step-while",
                documentation: "Step through all slots, updating the given initial state for each taken slot in line
by returning `'going` or exiting early with `'done` (like calling `break` in other languages).
An example can be found in `opt-span-step-while`",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_choice([("up", type_empty), ("down", type_empty)]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_slot(type_variable("Origin")),
                        ],
                        type_choice([("done", type_variable("done")), ("going", type_variable("State"))])
                    )
                ],
                result: type_choice([
                    (
                        "done",
                        type_record([
                            ("done", type_variable("done")),
                            ("rest", type_opt(type_span(type_variable("Origin"))))
                        ])
                    ),
                    ("going", type_variable("State"))
                ]),
            },
            CoreFnInfo {
                name: "opt-span-step-while",
                documentation: "Step through all slots, updating the given initial state for each taken slot in line
by returning `'going` or exiting early with `'done` (like calling `break` in other languages).
```slug
fn next-non-space(
    span opt(span(Origin))
    chars buf(Origin char)
)
    {
    chars buf(Origin char)
    non-space opt(char)
    after opt(span(Origin))
    }
    if
        opt-span-step-while(
            span
            [up<[down *]> *]
            chars
            fn(slot slot(Origin) chars buf(Origin char)
            = char buf-remove(chars slot)
            if u32-order(char-to-u32(char) char-to-u32(\" \" char)) {
            [equal *] (
                [going<[done { chars buf(Origin char) non-space char }]> chars]
            )
            [less *] [done<[going buf(Origin char)]> { chars chars non-space char }]
            [greater *] [done<[going buf(Origin char)]> { chars chars non-space char }]
            }
        )
    {
    [going chars]
        { chars chars non-space [no<opt(char)> *] after [no<opt(span(Origin)) *] }
    [done { rest span-after done { chars chars non-space non-space } }]
        { chars chars non-space [yes<[no *]> non-space] after span-after }
    }
```
Note that `.rest` does not include any slot given to the step function, even the slot that resulted in `[done]`.
(This example looks convoluted. If you introduce helpers like char-equal it gets more resonable)",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_choice([("up", type_empty), ("down", type_empty)]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_slot(type_variable("Origin")),
                        ],
                        type_choice([("done", type_variable("done")), ("going", type_variable("State"))])
                    )
                ],
                result: type_choice([
                    (
                        "done",
                        type_record([
                            ("done", type_variable("done")),
                            ("rest", type_opt(type_span(type_variable("Origin"))))
                        ])
                    ),
                    ("going", type_variable("State"))
                ]),
            },
            CoreFnInfo {
                name: "buf-empty",
                documentation: "Create an empty buf.
To modify it, use `buf-pre-allocate-at-least`, `buf-add`, `buf-add-unset` etc.",
                type_parameters: vec![Name::from_static("Origin"), Name::from_static("Item")],
                parameters: vec![],
                result: type_buf(type_variable("Origin"), type_variable("Item")),
            },
            CoreFnInfo {
                name: "buf-reuse",
                documentation: "Initialize a `buf` with 0 items and spare allocated memory from an `Unset-slice`.
This can be used to recycle `buf` memory from one `buf` with one origin into another `buf` with a different origin.
```slug
fn buf-recycle(old buf(OldOrigin Item))
    buf(New-origin Item)
    buf-reuse<NewOrigin>(buf-to-unset(old))
```",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![type_unset_slice(type_variable("Item"))],
                result: type_buf(type_variable("Origin"), type_variable("Item")),
            },
            CoreFnInfo {
                name: "buf-pre-allocate-at-least",
                documentation: "Reserves spare capacity for at least a given length more items to be added.
This can prevent frequent re-allocation of the underlying array.
If you can guesstimate a lower bound of how many items are ultimately added, this is always worth it!

If for example memory is limited and you want to handle out of memory errors more gracefully,
use `buf-try-pre-allocate-at-least` instead.",
                type_parameters: vec![],
                parameters: vec![type_u32],
                result: type_empty,
            },
            CoreFnInfo {
                name: "buf-try-pre-allocate-at-least",
                documentation: "Reserves spare capacity for at least a given length more items to be added
and if not possible returns `.out-of-memory 'yes .` (if the output language target can detect this).
A version which only checks for errors behind the scenes is `buf-pre-allocate-at-least`",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![type_u32],
                result: type_choice([
                    ("out-of-memory", type_empty),
                    ("success", type_empty)
                ]),
            },
            CoreFnInfo {
                name: "buf-pre-allocation-rid",
                documentation: "Shrinks down spare capacity as much as possible.
Some allocators may scrap and re-allocate the whole `buf` as a result.
It's rarely useful but can reuce idle memory usage for `buf`s that are very unlikely to be added to in the future.
You may also use it to adjust memory usage after `buf-reuse` when the given `Unset-slice` was large",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![],
                result: type_empty,
            },
            CoreFnInfo {
                name: "buf-insert",
                documentation: "Add a new item into the `buf` and keep a slot to it,
reusing unset space closest to the start of the `buf` when available.
Use `buf-add` if you don't care about reuse.
Use `buf-set-or-insert` if you already know of an index that's likely unset",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![type_variable("Item") ],
                result: type_slot(type_variable("Origin")),
            },
            CoreFnInfo {
                name: "buf-set-or-insert",
                documentation: "Add a new item into the `buf`
a a specific index (starting at 0 for the first item) and keep a slot to it.
If the given index is already occupied, chooses an unset index closest to the start of the `buf` when available.

You can use this to more cheaply return temporarily removed items to their original spots.
`span-start-index` is also often useful here.
Note that `buf-item-step`, `buf-swap` etc. will be enough to fulfill most of the same purposes without indexes flying about.

Use `buf-add` if you don't care about reuse.",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![
                    type_u32,
                    type_variable("Item"),
                ],
                result: type_slot(type_variable("Origin")),
            },
            CoreFnInfo {
                name: "buf-add",
                documentation: "Add a new item to the end of the `buf` and keep a slot to it without trying to reuse unset slots.
Can be faster than `buf-insert` when you expect no unset space or when all the storage gets scrapped soon anyway.",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![type_variable("Item")],
                result: type_slot(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-add-str-chars",
                documentation: "Attach the chars in a given `str` to the end of the `buf` and keep a span to it without trying to reuse unset slots.
Convenient equivalent to `buf-opt-span-add-str-chars` with an empty span.",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![type_str],
                result: type_span(type_variable("Origin")),
            },
            CoreFnInfo {
                name: "buf-remove",
                documentation: "Unset and retrieve an item from the `buf` at a given slot (the inverse of `buf-insert`/`buf-add`)",
                type_parameters: vec![],
                parameters: vec![type_slot(type_variable("Origin"))],
                result: type_variable("Item"),
            },
            CoreFnInfo {
                name: "buf-swap",
                documentation: "Replace the item at the first given slot with the item at the second given slot and the other way around.
Important: the first and second resulting slot will each refer to the items of the first and second given slot
(which is now at at different position in the buf).

This may not seem very useful considering that you could also just switch out only the slots themselves.
An example use case is removing a specific index in a span
where you don't care about order among the items:
You can take the last slot and swap its item with the item at the specific index,
then remove the last slot with the item from the original certain index.
(Drectly removing the last item and replacing the item at the specific index with the last item also works)",
                type_parameters: vec![],
                parameters: vec![
                    type_slot(type_variable("Origin")),
                    type_slot(type_variable("Origin")),
                ],
                result: type_tuple([
                    type_slot(type_variable("Origin")),
                    type_slot(type_variable("Origin")),
                ]),
            },
            CoreFnInfo {
                name: "buf-replace",
                documentation: "Switch out the item from the `buf` at a given slot and return the original item.
A more versatile version is `buf-item-step`.",
                type_parameters: vec![],
                parameters: vec![
                    type_slot(type_variable("Origin")),
                    type_variable("Item")
                ],
                result: type_variable("Item"),
            },
            CoreFnInfo {
                name: "buf-item-step",
                documentation: "Look at an item from the `buf` at a given slot, change it and return something out.
It behaves like `buf-remove` followed by `buf-insert` but is more performant because it always operates on the same slot.
It's common to create simple helpers using `buf-item-step`, for example
To just switch out an item, use `buf-replace`",
                type_parameters: vec![],
                parameters: vec![
                    type_slot(type_variable("Origin")),
                    type_variable("In"),
                    type_fn(
                        vec![type_variable("In"), type_variable("Item")],
                        type_record([("out", type_variable("Out")), ("item", type_variable("Item"))])
                    )
                ],
                result: type_variable("Out")
            },
            CoreFnInfo {
                name: "buf-span-reverse",
                documentation: "Order the referenced items such that the previously last is now first, second last is second etc.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "buf-opt-span-reverse",
                documentation: "Order the referenced items such that the previously last is now first, second last is second etc.",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "buf-span-sort",
                documentation: r#"Order the referenced items, less before greater according to a given function.
Items whose comparison returns `'equal .` are ordered according to their original position in the Span ("stable sort").
While this function allows the compared items to be altered, I strongly recommend against doing so
as for example the new altered items may violate the expected ordering at that point.
Do no rely on a specific order of when items are compared."#,
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_fn(
                        vec![type_variable("Item"), type_variable("Item")],
                        type_order()
                    )
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "buf-opt-span-sort",
                documentation: r#"Order the referenced items, less before greater according to a given function.
Items whose comparison returns `'equal .` are ordered according to their original position in the Span ("stable sort").
While this function allows the compared items to be altered, I strongly recommend against doing so
as for example the new altered items may violate the expected ordering at that point.
Do no rely on a specific order of when items are compared."#,
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_fn(
                        vec![type_variable("Item"), type_variable("Item")],
                        type_order()
                    )
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "buf-span-alter",
                documentation: "Change every item referenced by the given Span with a given function to a new item of the same type.
If your change function needs extra context from the outside, use `buf-span-step` instead",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_fn(vec![type_variable("Item")], type_variable("Item")),
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "buf-opt-span-alter",
                documentation: "Change every item referenced by the given Span with a given function to a new item of the same type.
If your change function needs extra context from the outside, use `buf-opt-span-step` instead",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_fn(vec![type_variable("Item")], type_variable("Item")),
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "buf-span-step",
                documentation: "Step through each item for a given Span,
change it to a new item of the same type and update the given initial state.
If you want to be able to exit anytime, use `buf-span-step-while` instead.
If you need more control (for example accessing the given Buf during a step), use `span-step` instead.
To change every item without any state, use `buf-span-alter`",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_choice([("up", type_empty), ("down", type_empty)]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_variable("Item"),
                        ],
                        type_tuple([
                            type_variable("State"),
                            type_variable("Item"),
                        ])
                    )
                ],
                result: type_variable("State")
            },
            CoreFnInfo {
                name: "buf-opt-span-step",
                documentation: "Step through each item for a given Opt Span,
change it to a new item of the same type and update the given initial state.
If you want to be able to exit anytime, use `buf-opt-span-step-while` instead.
If you need more control (for example accessing the given Buf during a step), use `opt-span-step` instead.
To change every item without any state, use `buf-opt-span-alter`",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_choice([("up", type_empty), ("down", type_empty)]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_variable("Item"),
                        ],
                        type_tuple([
                            type_variable("State"),
                            type_variable("Item"),
                        ])
                    )
                ],
                result: type_variable("State")
            },
            CoreFnInfo {
                name: "buf-span-step-while",
                documentation: "Step through each item for a given span,
change it to a new item of the same type and update the given initial state or exit early.
```slug
fn buf-span-last-negative-i32(
    buf buf(Origin i32)
    span span(Origin)
)
    opt(i32)
    if
        buf-span-step-while(
            buf
            span
            [down<[up *]> *]
            {},
            fn({} i i32)
            { item i
              state (
                if i32-order(i 0 i32) {
                [less *] [done<[going *]> i]
                [equal *] [going<[done i32]> *]
                [greater *] [going<[done i32]> *]
                }
            }
        )
    [going *] [no<opt(i32)> *]
    [done i] [yes<opt(i32)> i]
```
If you want to step through all items instead, use `buf-span-step`.
If you need more control (for example accessing the given Buf during a step), use `span-step-while` instead.
To change every item without any state, use `buf-span-alter`",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_choice([("up", type_empty), ("down", type_empty)]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_variable("Item"),
                        ],
                        type_tuple([
                            type_choice([
                                ("going", type_variable("State")),
                                ("done", type_variable("done")),
                            ]),
                            type_variable("Item")
                        ]),
                    )
                ],
                result: type_choice([
                    ("going", type_variable("State")),
                    ("done", type_variable("done")),
                ]),
            },
            CoreFnInfo {
                name: "buf-opt-span-step-while",
                documentation: "Step through each item for a given Span,
change it to a new item of the same type and update the given initial state with the possibility to exit early.
If you want to step through all items instead, use `buf-opt-span-step`.
If you need more control (for example accessing the given Buf during a step), use `opt-span-step-while` instead.
To change every item without any state, use `buf-opt-span-alter`",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_choice([("up", type_empty), ("down", type_empty)]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_variable("Item"),
                        ],
                        type_tuple([
                            type_choice([
                                ("going", type_variable("State")),
                                ("done", type_variable("done")),
                            ]),
                            type_variable("Item")
                        ]),
                    )
                ],
                result: type_choice([
                    ("going", type_variable("State")),
                    ("done", type_variable("done")),
                ]),
            },
            CoreFnInfo {
                name: "buf-opt-span-add",
                documentation: "Attach a given item at the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_variable("Item"),
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-span-add",
                documentation: "Attach a given item at the end of the span",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_variable("Item"),
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-opt-span-add-str-chars",
                documentation: "Attach a given `str` at the end of the span",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_str,
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-span-add-str-chars",
                documentation: "Attach a given `str` to the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_str,
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-span-add-u32-chars",
                documentation: "Print a given `u32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_u32,
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-opt-span-add-u32-chars",
                documentation: "Print a given `u32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_u32,
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-span-add-i32-chars",
                documentation: "Print a given `i32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_i32,
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-opt-span-add-i32-chars",
                documentation: "Print a given `i32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_i32,
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-span-add-f32-chars",
                documentation: "Print a given `f32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_f32,
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-opt-span-add-f32-chars",
                documentation: "Print a given `f32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_f32,
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-opt-span-move-to-unset",
                documentation: "Move the given span to a unset range if there is unset space available where moving the given span to would reduce the amount of unset space.
Try to use sparingly",
                type_parameters: vec![],
                parameters: vec![type_opt(type_span(type_variable("Origin")))],
                result: type_opt(type_span(type_variable("Origin")))
            },
            CoreFnInfo {
                name: "buf-span-move-to-unset",
                documentation: "Move the given span to a unset range if there is unset space available where moving the given span to would reduce the amount of unset space.",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"))],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-span-move-to-end",
                documentation: "Move the given span to after all existing items if necessary.",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"))],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-opt-span-move-to-end",
                documentation: "Move the given span to after all existing items if necessary.",
                type_parameters: vec![],
                parameters: vec![type_opt(type_span(type_variable("Origin")))],
                result: type_opt(type_span(type_variable("Origin")))
            },
            CoreFnInfo {
                name: "buf-span-add-own-span",
                documentation: "Append the items of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are unset.
As an example, you could implement `buf-span-add` in slug itself as
```slug
fn Buf-span-add
    .buf buf buf(Origin _item)
    .span span Span _origin
    .new new _item
:
    .buf buf(Origin _item)
    .span Span _origin
=
    # the first line is optional: it ensures that the new slot will actually be connected,
    # meaning the new item can stay at its position
    ? Buf-span-move-to-end .buf buf .span span [.buf buf .span .span]
    ? Buf-add .buf buf .new new [.buf buf .slot new-slot]
    Buf-span-add-own-span
    .buf buf
    .start span
    .end Slot-to-span new-slot
```",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_span(type_variable("Origin")),
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-span-add-own-opt-span",
                documentation: "Append the items of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are unset.
The most common use case is re-combining spans that have been split up with e.g. `span-start` (see also `slot-to-span`)",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin")),
                    type_opt(type_span(type_variable("Origin"))),
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-opt-span-add-own-span",
                documentation: "Append the items of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are unset.
The most common use case is re-combining spans that have been split up with e.g. `span-end` (see also `slot-to-span`)",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_span(type_variable("Origin")),
                ],
                result: type_span(type_variable("Origin"))
            },
            CoreFnInfo {
                name: "buf-opt-span-add-own-opt-span",
                documentation: "Append the items of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are unset",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"))),
                    type_opt(type_span(type_variable("Origin"))),
                ],
                result: type_opt(type_span(type_variable("Origin"))),
            },
            CoreFnInfo {
                name: "buf-to-unset",
                documentation: "Extract the underlying slice that been used to store items in including spare capacity.
This `Unset-slice` can if necessary be casted to a new item type with `Unset-slice-cast-or-rid-and-allocate`.
Finally, the allocation can be the base of a new `buf` with `buf-reuse` or be scrapped with `Unset-slice-rid`",
                type_parameters: vec![],
                parameters: vec![type_buf(type_variable("Origin"), type_variable("Item"))],
                result: type_unset_slice(type_variable("Item")),
            },
            CoreFnInfo {
                name: "Unset-slice-allocate-length",
                documentation: "Create a new `Unset-slice` with a given length.
There rarely is a need to use this except when a function expects an `Unset-slice` as an argument",
                type_parameters: vec![Name::from_static("Item")],
                parameters: vec![type_u32],
                result: type_unset_slice(type_variable("Item")),
            },
            CoreFnInfo {
                name: "Unset-slice-cast-or-rid-and-allocate",
                documentation: r#"Reinterpret the slice of unset bytes as a slice of a different item type.
This only works when the new item type has the same "size" (byte count including padding bits)
and equal "alignment" (byte count of the biggest field).
For example, `u32` consists of 4 bytes, just like `i32`, so they can be reinterpreted.
Similarly `.x f32 .y f32` can be reinterpreted as `.width i32 .height i32` and so forth.
If the types are incompatible, this function calls `Unset-slice-rid` on the old slice
and allocates a new one with the same length as the scrapped one.

Note: This difference in behavior does not get reported at compile-time
which is a bit of a stinker. The reasons are:
- different compilation targets may have a different memory packing of slug values,
  so we can never really guarantee this reinterpretation works
- it could be possible to fit different item sizes into the slice
  (if size of NewItem is multiple of unset-slice length).
  For example, 3 u32s could be transformed into 6 u16s and the other way around.
  For bigger sizes, this is only possible at runtime
- it could be possible to reinterpret item types with different alignments.
  For growing alignments, this is only possible at runtime
  when the slice pointer happens to align correctly
- slug has a simple type system so I'm happy such a compromise can exist at all"#,
                type_parameters: vec![Name::from_static("new-item")],
                parameters: vec![type_unset_slice(type_variable("Item"))],
                result: type_unset_slice(type_variable("new-item"))
            },
        ].map(|core_fn_info| {
            (
                Name::from_static(core_fn_info.name),
                CheckedProjectFn {
                    documentation: Some(Box::from(core_fn_info.documentation)),
                    type_parameters: core_fn_info.type_parameters,
                    parameter_types: core_fn_info.parameters.into_iter().map(Some).collect(),
                    result_type: Some(core_fn_info.result),
                    result_expression_is_invalid: false
                },
            )
        }))
    });
pub static core_type_aliases: std::sync::LazyLock<
    std::collections::HashMap<Name, CheckedTypeAlias>,
> = std::sync::LazyLock::new(|| {
    std::collections::HashMap::from([
        (
            Name::from_static("p32"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"A natural number >= 1 (positive integer) with 32 bits.
```slug
fn Answer . : p32 =
    P32-add-clamp .p 2 p32 .u 40 u32
```",
                )),
                parameters: vec![],
                type_: Some(type_p32),
            },
        ),
        (
            Name::from_static("u32"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"A natural number >= 0 (unsigned integer) with 32 bits.
```slug
fn Answer . : u32 =
    U32-add-clamp .a 2 u32 .b 40 u32
```",
                )),
                parameters: vec![],
                type_: Some(type_u32),
            },
        ),
        (
            Name::from_static("i32"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"A signed whole number (integer) with 32 bits.
```slug
fn Answer . : i32 =
    I32-add-clamp .a -8 i32 .b 50 i32
```",
                )),
                parameters: vec![],
                type_: Some(type_i32),
            },
        ),
        (
            Name::from_static("f32"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"A signed decimal number (floating-point) with 32 bit precision.
Does not allow infinities or NaN. If you need these error states, explicitly model them with a choice type.
```slug
fn Answer . : f32 =
    F32-add-clamp .a -8.5 f32 .b 50.5 f32
```",
                )),
                parameters: vec![],
                type_: Some(type_f32),
            },
        ),
        (
            Name::from_static("char"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r#"A unicode scalar like `"a" char` or `"👀" char` or `"\(2665)" char` (hex code for ♥).
Keep in mind that a human-readable visual symbol can be composed of multiple such unicode scalars (forming a grapheme cluster), For example:
```slug
Str-start "🇺🇸" str
# = .start "\(1F1FA)" char .after 'yes "\(1F1F8)" str
# Indicator U followed by Indicator S
```
Read if interested: [swift's grapheme cluster docs](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/stringsandcharacters/#Extended-Grapheme-Clusters)"#,
                )),
                parameters: vec![],
                type_: Some(type_char),
            },
        ),
        (
            Name::from_static("str"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r#"A positive-length slice of known text which is valid for the entire program,
like `"abc" str` or `"\"hello 👀 \\\r\n world \(2665)\"" str`
- `\(2665)` represents the hex code for special characters, here ♥
- `\"` represents a double quote "
- `\\` represents one backslash \
- `\n` represents line break
- `\r` represents carriage return

Internally, a string is compactly represented as UTF-8 bytes and can be accessed as such.
When building new strings at runtime, use functions like `buf-opt-span-add-str-chars`."#,
                )),
                parameters: vec![],
                type_: Some(type_str),
            },
        ),
        (
            Name::from_static("order"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "Result of a binary comparison.
```slug
u32-order(12 u32 20 u32)
# = [less<order> *]

fn u32-max(a u32 b u32) u32
    if u32-order(a b) {
    [less *] b
    [equal *] b
    [greater *] a
    }
```",
                )),
                parameters: vec![],
                type_: Some(type_order()),
            },
        ),
        (
            Name::from_static("opt"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "Either you have some value (yes) or you have nothing (no).",
                )),
                parameters: vec![Name::from_static("yes")],
                type_: Some(type_opt(type_variable("yes"))),
            },
        ),
        (
            Name::from_static("buf"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A grow- and shrinkable array buffer of items.
Has constant time access, remove, step and add."
                )),
                parameters: vec![Name::from_static("Origin"), Name::from_static("Item")],
                type_: Some(type_buf(type_variable("Origin"), type_variable("Item"))),
            },
        ),
        (
            Name::from_static("slot"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A valid position of an item in a collection.
This works because each collection has a unique origin and only gives out one slot for each position.
For consecutive `slot`s, check out `span`."
                )),
                parameters: vec![Name::from_static("Origin")],
                type_: Some(type_slot(type_variable("Origin"))),
            },
        ),
        (
            Name::from_static("span"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A range of ≥1 consecutive valid slots in a collection.
This works because each collection has a unique origin and only gives out one span for each range.
For potentially 0-length spans, use `opt Span`"
                )),
                parameters: vec![Name::from_static("Origin")],
                type_: Some(type_span(type_variable("Origin"))),
            },
        ),
        (
            Name::from_static("unset-slice"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A heap-allocated array with unknown length and undefined contents.
Can be constructed manually or as an intermediate type when recycling the allocated space of a collection,
see `buf-to-unset`.
Since you can't read from it, you can also safely attempt to reuse this allocation for a different item type,
see `Unset-slice-cast-or-rid-and-allocate`."
                )),
                parameters: vec![Name::from_static("Origin")],
                type_: Some(type_unset_slice(type_variable("Origin"))),
            },
        ),
    ])
});
pub static core_records: std::sync::LazyLock<std::collections::HashSet<&'static [Name]>> =
    std::sync::LazyLock::new(|| {
        fn type_records(type_: &Type, records: &mut std::collections::HashSet<&'static [Name]>) {
            match type_ {
                Type::Variable(_) => {}
                Type::Tuple(parts) => {
                    for part in parts {
                        type_records(part, records);
                    }
                }
                Type::Record(fields) => {
                    records.insert(
                        sorted_field_names(fields.iter().map(|field| &field.name))
                            // static variables will only be created once and won't be dropped anyway
                            .leak(),
                    );
                    for field in fields {
                        type_records(&field.value, records);
                    }
                }
                Type::Origin(_) => {}
                Type::Choice(variants) => {
                    for variant in variants {
                        type_records(&variant.value, records);
                    }
                }
                Type::CoreConstruct { name: _, arguments } => {
                    for argument in arguments {
                        type_records(argument, records);
                    }
                }
                Type::Fn { parameters, result } => {
                    type_records(result, records);
                    for parameter in parameters {
                        type_records(parameter, records);
                    }
                }
            }
        }
        let mut records = std::collections::HashSet::new();
        for core_fn_info in core_fns.values() {
            for parameter_type in core_fn_info.parameter_types.iter().flatten() {
                type_records(parameter_type, &mut records);
            }
            if let Some(result_type) = &core_fn_info.result_type {
                type_records(result_type, &mut records);
            }
        }
        records
    });
pub static core_choices: std::sync::LazyLock<std::collections::HashSet<&'static [Name]>> =
    std::sync::LazyLock::new(|| {
        fn type_choices(type_: &Type, choices: &mut std::collections::HashSet<&'static [Name]>) {
            match type_ {
                Type::Variable(_) => {}
                Type::Tuple(parts) => {
                    for part in parts {
                        type_choices(part, choices);
                    }
                }
                Type::Record(fields) => {
                    for field in fields {
                        type_choices(&field.value, choices);
                    }
                }
                Type::Origin(_) => {}
                Type::Choice(variants) => {
                    choices.insert(
                        sorted_variant_names(variants.iter().map(|field| &field.name))
                            // static variables will only be created once and won't be dropped anyway
                            .leak(),
                    );
                    for variant in variants {
                        type_choices(&variant.value, choices);
                    }
                }
                Type::CoreConstruct { name: _, arguments } => {
                    for argument in arguments {
                        type_choices(argument, choices);
                    }
                }
                Type::Fn { parameters, result } => {
                    type_choices(result, choices);
                    for parameter in parameters {
                        type_choices(parameter, choices);
                    }
                }
            }
        }
        let mut variants = std::collections::HashSet::new();
        for core_fn_info in core_fns.values() {
            for parameter_type in core_fn_info.parameter_types.iter().flatten() {
                type_choices(parameter_type, &mut variants);
            }
            if let Some(result_type) = &core_fn_info.result_type {
                type_choices(result_type, &mut variants);
            }
        }
        variants
    });
pub fn is_core_fn_taking_allocator_in_zig(fn_name: &str) -> bool {
    match fn_name {
        "buf-to-unset" => true,
        _ => is_core_fn_that_can_run_out_of_memory_in_zig(fn_name),
    }
}
pub fn is_core_fn_that_can_run_out_of_memory_in_zig(fn_name: &str) -> bool {
    match fn_name {
        "span-step"
        | "opt-span-step"
        | "span-step-while"
        | "opt-span-step-while"
        | "unset-slice-cast-or-rid-and-allocate"
        | "unset-slice-allocate-length"
        | "buf-item-step"
        | "buf-opt-span-sort"
        | "buf-span-sort"
        | "buf-opt-span-alter"
        | "buf-span-alter"
        | "buf-opt-span-step"
        | "buf-span-step"
        | "buf-opt-span-step-while"
        | "buf-span-step-while"
        | "buf-opt-span-move-to-end"
        | "buf-span-move-to-end"
        | "buf-opt-span-add-own-opt-span"
        | "buf-span-add-own-opt-span"
        | "buf-opt-span-add-own-span"
        | "buf-span-add-own-span"
        | "buf-opt-span-add-buf-span"
        | "buf-opt-span-add-buf-opt-span"
        | "buf-span-add-buf-span"
        | "buf-span-add-buf-opt-span"
        | "buf-opt-span-add-f32-chars"
        | "buf-span-add-f32-chars"
        | "buf-opt-span-add-i32-chars"
        | "buf-span-add-i32-chars"
        | "buf-opt-span-add-u32-chars"
        | "buf-span-add-u32-chars"
        | "buf-span-add-str-chars"
        | "buf-opt-span-add-str-chars"
        | "buf-span-add"
        | "buf-opt-span-add"
        | "buf-add-array"
        | "buf-add-str-chars"
        | "buf-add"
        | "buf-insert"
        | "buf-set-or-insert"
        | "buf-pre-allocation-rid"
        | "buf-pre-allocate-at-least" => true,
        _ => false,
    }
}

pub struct ErrorNode {
    // possible optimization: change to cow
    pub message: Box<str>,
    pub range: lsp_types::Range,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LineSpan {
    Single,
    Multiple,
}
fn range_line_span(range: lsp_types::Range) -> LineSpan {
    if range.start.line == range.end.line {
        LineSpan::Single
    } else {
        LineSpan::Multiple
    }
}
fn linebreak_indented_into(formatted: &mut String, indent: usize) {
    formatted.push('\n');
    indented_into(formatted, indent);
}
fn indented_into(formatted: &mut String, indent: usize) {
    formatted.extend(std::iter::repeat_n(' ', indent));
}
fn space_or_linebreak_indented_into(formatted: &mut String, line_span: LineSpan, indent: usize) {
    match line_span {
        LineSpan::Single => {
            formatted.push(' ');
        }
        LineSpan::Multiple => {
            linebreak_indented_into(formatted, indent);
        }
    }
}
fn next_indent(current_indent: usize) -> usize {
    (current_indent + 1).next_multiple_of(4)
}

fn syntax_comments_format(formatted: &mut String, indent: usize, comments: &SyntaxComments) {
    for line in std::iter::once(&comments.line0).chain(&comments.line1_up) {
        formatted.push_str("# ");
        formatted.push_str(
            line.value
                .strip_prefix(' ')
                .unwrap_or(&line.value)
                .trim_end(),
        );
        linebreak_indented_into(formatted, indent);
    }
}
pub fn syntax_project_format<Expressions, Patterns>(
    project: &SyntaxProject<Expressions, Patterns>,
    source: &str,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> String {
    let mut formatted = String::with_capacity(source.len() + 8);
    let mut items_iterator = project.items.iter().peekable();
    'formatting_items: while let Some(item) = items_iterator.next() {
        if let Some(SyntaxProjectItem::Unrecognized {
            range: unrecognized_range,
            source: _,
        }) = items_iterator.peek()
        {
            // we handle the unrecognized range now so we skip it in the iterator
            let item_after_unrecognized = items_iterator.nth(1);
            formatted.push('\n');
            if let Some(unformatted_source) = source.get(str_lsp_range_to_utf8_range(
                source,
                lsp_types::Range {
                    start: project_item_start(item),
                    end: item_after_unrecognized
                        .map(|item_after_unrecognized| {
                            project_item_end(item_after_unrecognized, expressions, patterns)
                        })
                        .unwrap_or(unrecognized_range.end),
                },
            )) {
                formatted.push_str(unformatted_source);
            }
            formatted.push('\n');
            continue 'formatting_items;
        }
        match item {
            SyntaxProjectItem::TypeAlias {
                ty_keyword_start: _,
                name,
                parameters,
                documentation,
                type_,
            } => {
                formatted.push_str("\nty ");
                if let Some(name) = name {
                    formatted.push_str(&name.value);
                }
                match parameters
                    .as_ref()
                    .and_then(|params| params.parameters.split_first())
                {
                    Some((parameter0, parameter1_up)) => {
                        formatted.push('(');
                        formatted.push_str(&parameter0.value);
                        for parameter in parameter1_up {
                            formatted.push(' ');
                            formatted.push_str(&parameter.value);
                        }
                    }
                    None => {
                        if let Some(name) = name {
                            formatted.push_str(&name.value);
                        }
                    }
                }
                match documentation {
                    Some(documentation) => {
                        linebreak_indented_into(&mut formatted, 0);
                        syntax_comments_format(&mut formatted, 0, documentation);
                        indented_into(&mut formatted, next_indent(0));
                    }
                    None => match type_ {
                        Some(type_) => {
                            space_or_linebreak_indented_into(
                                &mut formatted,
                                range_line_span(type_range(type_)),
                                next_indent(0),
                            );
                        }
                        None => {
                            formatted.push(' ');
                        }
                    },
                }
                if let Some(type_) = type_ {
                    syntax_type_format(&mut formatted, next_indent(0), type_);
                }
                formatted.push('\n');
            }
            SyntaxProjectItem::Fn {
                fn_keyword_start,
                name,
                type_parameters,
                open_paren_start,
                parameters,
                closed_paren_start,
                result_type,
                documentation,
                result,
            } => {
                formatted.push_str("\nfn ");
                if let Some(name) = name {
                    formatted.push_str(&name.value);
                }
                if let Some(type_parameter) = type_parameters {
                    syntax_angled_type_parameters_format(&mut formatted, type_parameter);
                }
                let header_line_span = range_line_span(lsp_types::Range {
                    start: *fn_keyword_start,
                    end: result_type
                        .as_ref()
                        .map(|result_type| type_end(result_type))
                        .or_else(|| {
                            closed_paren_start
                                .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
                        })
                        .or_else(|| {
                            parameters
                                .last()
                                .map(|parameter| pattern_end(parameter, patterns))
                        })
                        .or_else(|| {
                            open_paren_start
                                .map(|open_paren_start| symbol_end(open_paren_start, "("))
                        })
                        .or_else(|| {
                            name.as_ref()
                                .map(|name| name_end(with_start_position_as_ref(name)))
                        })
                        .unwrap_or_else(|| symbol_end(*fn_keyword_start, "fn")),
                });
                space_or_linebreak_indented_into(&mut formatted, header_line_span, next_indent(0));
                formatted.push('(');
                for parameter in parameters {
                    syntax_pattern_format(&mut formatted, next_indent(0), patterns, parameter);
                    space_or_linebreak_indented_into(
                        &mut formatted,
                        header_line_span,
                        next_indent(0),
                    );
                }
                space_or_linebreak_indented_into(&mut formatted, header_line_span, 0);
                formatted.push(')');
                match result_type {
                    Some(result_type) => {
                        let result_type_lne_span = range_line_span(type_range(result_type));
                        space_or_linebreak_indented_into(
                            &mut formatted,
                            result_type_lne_span,
                            next_indent(0),
                        );
                        syntax_type_format(&mut formatted, next_indent(0), result_type);
                        space_or_linebreak_indented_into(&mut formatted, result_type_lne_span, 0);
                    }
                    None => {
                        space_or_linebreak_indented_into(&mut formatted, header_line_span, 0);
                    }
                }
                if let Some(documentation) = documentation {
                    syntax_comments_format(&mut formatted, 0, documentation);
                }
                formatted.push('=');
                linebreak_indented_into(&mut formatted, next_indent(0));
                if let Some(result) = result {
                    syntax_expression_format(
                        &mut formatted,
                        next_indent(0),
                        expressions,
                        patterns,
                        result,
                    );
                }
                formatted.push('\n');
            }
            SyntaxProjectItem::Comments(comments) => {
                if comments.line0.start.line >= 1 {
                    formatted.push('\n');
                }
                syntax_comments_format(&mut formatted, 0, comments);
            }
            SyntaxProjectItem::Unrecognized {
                range: _,
                source: unrecognized_source,
            } => {
                formatted.push_str(unrecognized_source);
            }
        }
    }
    formatted
}
fn syntax_angled_type_parameters_format(
    formatted: &mut String,
    angled_type_parameters: &SyntaxAngledTypeParameters,
) {
    formatted.push('<');
    if let Some((angled_type_parameter0, angled_type_parameter1_up)) =
        angled_type_parameters.names.split_first()
    {
        formatted.push_str(&angled_type_parameter0.value);
        for angled_type_parameter in angled_type_parameter1_up {
            formatted.push_str(&angled_type_parameter.value);
            formatted.push(' ');
        }
    }
    formatted.push('>');
}

fn char_needs_unicode_escaping(char: char) -> bool {
    char.is_control()
}
fn unicode_char_escape_into(so_far: &mut String, char: char) {
    let code: u32 = char.into();
    use std::fmt::Write as _;
    _ = write!(so_far, "\\({:X})", code);
}
fn syntax_string_format(formatted: &mut String, content: &str) {
    formatted.push('"');
    for char in content.chars() {
        match char {
            '\"' => formatted.push_str("\\\""),
            '\\' => formatted.push_str("\\\\"),
            '\t' => formatted.push_str("\\t"),
            '\n' => formatted.push_str("\\n"),
            '\u{000D}' => formatted.push_str("\\(000D)"),
            other_character => {
                if char_needs_unicode_escaping(other_character) {
                    unicode_char_escape_into(formatted, other_character);
                } else {
                    formatted.push(other_character);
                }
            }
        }
    }
    formatted.push('"');
}
fn syntax_expression_format<Expressions, Patterns>(
    formatted: &mut String,
    indent: usize,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,

    expression: &SyntaxExpression<Expressions, Patterns>,
) {
    match expression {
        SyntaxExpression::Number { value, type_ } => {
            if value.value.starts_with("0") {
                let after_0s = value.value[1..].trim_start_matches("0");
                if after_0s.is_empty() {
                    formatted.push('0');
                } else {
                    formatted.push_str(after_0s);
                }
            } else if value.value.starts_with('.') {
                formatted.push('0');
                formatted.push_str(&value.value);
            } else {
                formatted.push_str(&value.value);
            }
            formatted.push(' ');
            if let Some(type_) = type_ {
                syntax_type_format(formatted, next_indent(indent), type_);
            }
        }
        SyntaxExpression::Text {
            open_quote_start: _,
            content,
            content_end: _,
            closed_quote_exists: _,
            type_,
        } => {
            syntax_string_format(formatted, content);
            formatted.push(' ');
            if let Some(type_) = type_ {
                syntax_type_format(formatted, next_indent(indent), type_);
            }
        }
        SyntaxExpression::Variable(name) => {
            formatted.push_str(&name.value);
        }
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments,
            open_paren_start,
            arguments_rest: arguments,
            closed_paren_start,
        } => {
            if let Some(first_argument_dot) = first_argument_dot {
                syntax_expression_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    expressions.item(&first_argument_dot.argument),
                );
                linebreak_indented_into(formatted, indent);
                formatted.push('.');
            }
            formatted.push_str(&name.value);
            if let Some(type_arguments) = type_arguments {
                formatted.push('<');
                if let Some((type_argument0, type_argument1_up)) =
                    type_arguments.types.split_first()
                {
                    let type_arguments_line_span = range_line_span(lsp_types::Range {
                        start: type_arguments.open_angle_start,
                        end: type_arguments.closed_angle_start.unwrap_or_else(|| {
                            type_end(type_arguments.types.last().unwrap_or(type_argument0))
                        }),
                    });
                    if type_arguments_line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, next_indent(indent));
                    }
                    syntax_type_format(formatted, next_indent(indent), type_argument0);
                    for type_argument in type_argument1_up {
                        space_or_linebreak_indented_into(
                            formatted,
                            type_arguments_line_span,
                            next_indent(indent),
                        );
                        syntax_type_format(formatted, next_indent(indent), type_argument);
                    }
                    if type_arguments_line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, indent);
                    }
                }
                formatted.push('>');
            }
            formatted.push('(');
            if let Some((argument0, argument1_up)) = arguments.split_first() {
                let arguments_line_span = range_line_span(lsp_types::Range {
                    start: open_paren_start
                        .or_else(|| {
                            type_arguments
                                .as_ref()
                                .map(|type_arguments| angled_type_arguments_end(type_arguments))
                        })
                        .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
                    end: closed_paren_start.unwrap_or_else(|| {
                        expression_end(arguments.last().unwrap_or(argument0), expressions, patterns)
                    }),
                });
                if arguments_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_expression_format(formatted, indent, expressions, patterns, argument0);
                for argument in argument1_up {
                    space_or_linebreak_indented_into(formatted, arguments_line_span, indent);
                    syntax_expression_format(formatted, indent, expressions, patterns, argument);
                }
                if arguments_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
            }
            formatted.push(')');
        }
        SyntaxExpression::Variant {
            open_bracket_start,
            type_,
            name,
            value,
            closed_bracket_start: _,
        } => {
            formatted.push('[');
            match name {
                Some(name) => {
                    formatted.push_str(&name.value);
                }
                None => {
                    formatted.push(' ');
                }
            }
            match type_ {
                None => {
                    formatted.push_str("<[]>");
                }
                Some(type_) => {
                    syntax_angled_type_argument_format(formatted, indent, type_);
                }
            }
            if let Some(value) = value {
                let value = expressions.item(value);
                space_or_linebreak_indented_into(
                    formatted,
                    range_line_span(lsp_types::Range {
                        start: *open_bracket_start,
                        end: expression_end(value, expressions, patterns),
                    }),
                    indent,
                );
                syntax_expression_format(formatted, indent, expressions, patterns, value);
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start: open_bracket_start,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            formatted.push_str("fn(");
            // TODO line span across all parameters
            for parameter in parameters {
                let parameter_line_span = range_line_span(pattern_range(parameter, patterns));
                if parameter_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_pattern_format(formatted, next_indent(indent), patterns, parameter);
                if parameter_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
            }
            formatted.push(')');
            if let Some(result) = result {
                space_or_linebreak_indented_into(
                    formatted,
                    range_line_span(lsp_types::Range {
                        start: *open_bracket_start,
                        end: expression_end(expression, expressions, patterns),
                    }),
                    indent,
                );
                syntax_expression_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    expressions.item(result),
                );
            }
        }
        SyntaxExpression::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            formatted.push('*');
            if let Some((part0, part1_up)) = parts.split_first() {
                formatted.push('(');
                let line_span =
                    range_line_span(expression_range(expression, expressions, patterns));
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                syntax_expression_format(
                    formatted,
                    next_indent(indent),
                    expressions,
                    patterns,
                    part0,
                );
                for part in part1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_expression_format(
                        formatted,
                        next_indent(indent),
                        expressions,
                        patterns,
                        part,
                    );
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                formatted.push(')');
            }
        }
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            let line_span = range_line_span(expression_range(expression, expressions, patterns));
            for field in fields {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                syntax_expression_record_field_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    field,
                );
            }
        }
        SyntaxExpression::Commented {
            comments,
            expression,
        } => {
            syntax_comments_format(formatted, indent, comments);
            if let Some(expression) = expression {
                syntax_expression_format(
                    formatted,
                    indent,
                    expressions,
                    patterns,
                    expressions.item(expression),
                );
            }
        }
        SyntaxExpression::Destruct {
            equals_start: _,
            pattern,
            queried,
            result,
        } => {
            formatted.push('=');
            match pattern {
                None => {
                    formatted.push_str("  ");
                }
                Some(pattern) => {
                    space_or_linebreak_indented_into(
                        formatted,
                        range_line_span(pattern_range(pattern, patterns)),
                        next_indent(indent),
                    );
                    syntax_pattern_format(formatted, next_indent(indent), patterns, pattern);
                }
            }
            match queried {
                None => {}
                Some(queried) => {
                    let queried = expressions.item(queried);
                    let line_span =
                        range_line_span(expression_range(queried, expressions, patterns));
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_expression_format(
                        formatted,
                        next_indent(indent),
                        expressions,
                        patterns,
                        queried,
                    );
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                }
            }
            linebreak_indented_into(formatted, indent);
            if let Some(result) = result {
                syntax_expression_format(
                    formatted,
                    next_indent(indent),
                    expressions,
                    patterns,
                    expressions.item(result),
                );
            }
        }
        SyntaxExpression::If {
            if_keyword_start: _,
            queried,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => {
            formatted.push_str("if");
            match queried {
                None => {
                    formatted.push(' ');
                }
                Some(queried) => {
                    let queried = expressions.item(queried);
                    let line_span =
                        range_line_span(expression_range(queried, expressions, patterns));
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_expression_format(
                        formatted,
                        next_indent(indent),
                        expressions,
                        patterns,
                        queried,
                    );
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                }
            }
            formatted.push('{');
            for case in cases {
                linebreak_indented_into(formatted, indent);
                syntax_expression_if_case_format(formatted, indent, expressions, patterns, case);
            }
            formatted.push('}');
        }
    }
}
fn syntax_angled_type_argument_format(
    formatted: &mut String,
    indent: usize,

    type_argument: &SyntaxAngledTypeArgument,
) {
    match &type_argument.type_ {
        None => {
            formatted.push_str("<[]>");
        }
        Some(type_) => {
            formatted.push('<');
            let line_span = range_line_span(lsp_types::Range {
                start: type_argument.open_angle_start,
                end: type_end(type_),
            });
            if line_span == LineSpan::Multiple {
                linebreak_indented_into(formatted, next_indent(indent));
            }
            syntax_type_format(formatted, next_indent(indent), type_);
            if line_span == LineSpan::Multiple {
                linebreak_indented_into(formatted, indent);
            }
            formatted.push('>');
        }
    }
}
fn syntax_expression_record_field_format<Expressions, Patterns>(
    formatted: &mut String,
    indent: usize,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    field: &SyntaxRecordField<SyntaxExpression<Expressions, Patterns>>,
) {
    formatted.push_str(&field.name.value);
    match &field.value {
        None => {
            formatted.push(' ');
        }
        Some(value) => {
            let line_span = range_line_span(lsp_types::Range {
                start: field.name.start,
                end: expression_range(value, expressions, patterns).end,
            });
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            syntax_expression_format(formatted, next_indent(indent), expressions, patterns, value);
        }
    }
}
fn syntax_expression_if_case_format<Expressions, Patterns>(
    formatted: &mut String,
    indent: usize,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    case: &SyntaxExpressionIfCase<Expressions, Patterns>,
) {
    syntax_pattern_format(formatted, next_indent(indent), patterns, &case.pattern);
    match &case.result {
        None => {
            formatted.push(' ');
        }
        Some(result) => {
            let line_span = range_line_span(pattern_range(&case.pattern, patterns));
            if line_span == LineSpan::Multiple {
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            }
            let line_span = range_line_span(lsp_types::Range {
                start: pattern_start(&case.pattern),
                end: expression_end(result, expressions, patterns),
            });
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            syntax_expression_format(
                formatted,
                next_indent(indent),
                expressions,
                patterns,
                result,
            );
        }
    }
}
fn syntax_pattern_format<Patterns>(
    formatted: &mut String,
    indent: usize,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    pattern: &SyntaxPattern<Patterns>,
) {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            formatted.push_str(&name.value);
            if let Some(type_) = type_ {
                space_or_linebreak_indented_into(
                    formatted,
                    range_line_span(pattern_range(pattern, patterns)),
                    indent,
                );
                syntax_type_format(formatted, indent, type_);
            }
        }
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_,
        } => {
            formatted.push('_');
            if let Some(type_) = type_ {
                space_or_linebreak_indented_into(
                    formatted,
                    range_line_span(pattern_range(pattern, patterns)),
                    indent,
                );
                syntax_type_format(formatted, indent, type_);
            }
        }
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => {
            // TODO
            if let Some(name) = name {
                formatted.push_str(&name.value.as_ref());
            }
            space_or_linebreak_indented_into(
                formatted,
                range_line_span(pattern_range(pattern, patterns)),
                indent,
            );
            if let Some(value) = value {
                syntax_pattern_format(formatted, indent, patterns, patterns.item(value));
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            formatted.push('*');
            if let Some((part0, part1_up)) = parts.split_first() {
                formatted.push('(');
                let line_span = range_line_span(pattern_range(pattern, patterns));
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_pattern_format(formatted, next_indent(indent), patterns, part0);
                for part in part1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_pattern_format(formatted, next_indent(indent), patterns, part);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                formatted.push(')');
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            // TODO
            formatted.push('{');
            let line_span = range_line_span(pattern_range(pattern, patterns));
            for field in fields {
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                syntax_pattern_record_field_format(formatted, indent, patterns, field);
            }
            space_or_linebreak_indented_into(formatted, line_span, indent);
            formatted.push('}');
        }
    }
}
fn syntax_pattern_record_field_format<Patterns>(
    formatted: &mut String,
    indent: usize,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    field: &SyntaxRecordField<SyntaxPattern<Patterns>>,
) {
    formatted.push_str(&field.name.value);
    match &field.value {
        None => {
            formatted.push(' ');
        }
        Some(value) => {
            let line_span = range_line_span(lsp_types::Range {
                start: field.name.start,
                end: pattern_range(value, patterns).end,
            });
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            syntax_pattern_format(formatted, next_indent(indent), patterns, value);
        }
    }
}
fn syntax_type_format(formatted: &mut String, indent: usize, type_: &SyntaxType) {
    match type_ {
        SyntaxType::Variable(name) => {
            formatted.push_str(&name.value);
        }
        SyntaxType::Origin { hat_start: _, name } => {
            formatted.push('^');
            if let Some(name) = name {
                formatted.push_str(&name.value);
            }
        }
        SyntaxType::Fn {
            fn_keyword_start,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            formatted.push_str("fn(");
            if let Some((parameter0, parameter1_up)) = parameters.split_first() {
                let parameters_line_span = match parameters.last() {
                    None => LineSpan::Single,
                    Some(last_parameter) => range_line_span(gen_lsp_types::Range {
                        start: *fn_keyword_start,
                        end: type_end(last_parameter),
                    }),
                };
                if parameters_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_type_format(formatted, next_indent(indent), parameter0);
                for parameter in parameter1_up {
                    space_or_linebreak_indented_into(
                        formatted,
                        parameters_line_span,
                        next_indent(indent),
                    );
                    syntax_type_format(formatted, next_indent(indent), parameter);
                }
                if parameters_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
            }
            formatted.push(')');
            let line_span = range_line_span(type_range(type_));
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            if let Some(result) = result {
                syntax_type_format(formatted, next_indent(indent), result);
            }
        }
        SyntaxType::ConstructWithArguments { name, arguments } => {
            formatted.push_str(&name.value);
            if let Some(SyntaxTypeConstructParenthesizedArguments {
                open_paren_start: _,
                arguments,
                closed_paren_start: _,
            }) = arguments
                && let Some((argument0, argument1_up)) = arguments.split_first()
            {
                formatted.push('(');
                let line_span = range_line_span(lsp_types::Range {
                    start: name.start,
                    end: type_range(argument1_up.last().unwrap_or(argument0)).end,
                });
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                syntax_type_format(formatted, next_indent(indent), argument0);
                for argument_type in argument1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_type_format(formatted, next_indent(indent), argument_type);
                }
                formatted.push(')');
            }
        }
        SyntaxType::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            formatted.push('*');
            if let Some((part0, part1_up)) = parts.split_first() {
                formatted.push('(');
                let line_span = range_line_span(type_range(type_));
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_type_format(formatted, next_indent(indent), part0);
                for part in part1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_type_format(formatted, next_indent(indent), part);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                formatted.push(')');
            }
        }
        SyntaxType::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            // TODO
            formatted.push('{');
            let line_span = range_line_span(type_range(type_));
            for field in fields {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                formatted.push_str(&field.name.value);
                match &field.value {
                    None => {
                        formatted.push(' ');
                    }
                    Some(value) => {
                        let line_span = range_line_span(lsp_types::Range {
                            start: field.name.start,
                            end: type_range(value).end,
                        });
                        space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                        syntax_type_format(formatted, next_indent(indent), value);
                    }
                }
            }
            if line_span == LineSpan::Multiple {
                linebreak_indented_into(formatted, indent);
            }
            formatted.push('}');
        }
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => {
            // TODO
            formatted.push('[');
            let line_span = range_line_span(type_range(type_));
            for variant in variants {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                formatted.push_str(&variant.name.value);
                match &variant.value {
                    None => {
                        formatted.push(' ');
                    }
                    Some(value) => {
                        let line_span = range_line_span(lsp_types::Range {
                            start: variant.name.start,
                            end: type_end(value),
                        });
                        space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                        syntax_type_format(formatted, next_indent(indent), value);
                    }
                }
            }
            formatted.push(']');
        }
    }
}

pub enum SyntaxSymbol<'a, Expressions, Patterns> {
    ProjectTypeOrUnknown {
        name: WithStartPosition<&'a Name>,
        construct_info: ConstructInfo,
    },
    // applies to both type and variable name
    Origin {
        name: &'a Name,
        use_start: lsp_types::Position,
    },
    TypeVariable {
        name: &'a Name,
        use_start: lsp_types::Position,
        scope: &'a SyntaxProjectItem<Expressions, Patterns>,
    },
    VariantOrUnknown(WithStartPosition<&'a Name>),
    ProjectFnOrUnknown {
        name: WithStartPosition<&'a Name>,
        construct_info: ConstructInfo,
    },
    LocalVariable {
        name: &'a Name,
        use_start: lsp_types::Position,
        origin: LocalVariableSymbolOrigin<'a, Expressions, Patterns>,
    },
    VariableUnknown {
        local_variables: std::collections::HashMap<
            &'a Name,
            LocalVariableSymbolOrigin<'a, Expressions, Patterns>,
        >,
    },
}
pub enum ConstructInfo {
    NotExpectingArgument,
    ArgumentMissing,
    ArgumentExists,
    Declaration,
}
pub struct LocalVariableSymbolOrigin<'a, Expressions, Patterns> {
    pub start: lsp_types::Position,
    pub scope: Option<&'a SyntaxExpression<Expressions, Patterns>>,
    pub type_: Option<Type>,
}
pub fn project_symbol_at_position<'a, Expressions, Patterns>(
    project: &'a SyntaxProject<Expressions, Patterns>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    position: lsp_types::Position,
    expressions: &'a core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns>> {
    let Ok(item_containing_position_index) =
        project
            .items
            .binary_search_by(|item| match project_item_start(item).cmp(&position) {
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    match project_item_end(item, expressions, patterns).cmp(&position) {
                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                            std::cmp::Ordering::Equal
                        }
                    }
                }
            })
    else {
        return None;
    };
    let item_containing_position = project.items.get(item_containing_position_index);
    item_containing_position.and_then(|item| match item {
        SyntaxProjectItem::TypeAlias {
            ty_keyword_start: _,
            name,
            parameters,
            documentation: _,
            type_,
        } => {
            if let Some(name) = name
                && range_includes_position(name_range(with_start_position_as_ref(name)), position)
            {
                return Some(SyntaxSymbol::ProjectTypeOrUnknown {
                    name: with_start_position_as_ref(name),
                    construct_info: ConstructInfo::Declaration,
                });
            }
            parameters
                .as_ref()
                .and_then(|parameters| {
                    parameters.parameters.iter().find_map(|parameter_name| {
                        if range_includes_position(
                            name_range(with_start_position_as_ref(parameter_name)),
                            position,
                        ) {
                            Some(SyntaxSymbol::TypeVariable {
                                name: &parameter_name.value,
                                use_start: parameter_name.start,
                                scope: item,
                            })
                        } else {
                            None
                        }
                    })
                })
                .or_else(|| {
                    type_
                        .as_ref()
                        .and_then(|value| type_symbol_at_position(value, position, item))
                })
        }
        SyntaxProjectItem::Fn {
            fn_keyword_start: _,
            name,
            type_parameters,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result_type,
            documentation: _,
            result,
        } => {
            if let Some(name) = name
                && range_includes_position(name_range(with_start_position_as_ref(name)), position)
            {
                return Some(SyntaxSymbol::ProjectFnOrUnknown {
                    name: with_start_position_as_ref(name),
                    construct_info: ConstructInfo::Declaration,
                });
            }
            type_parameters
                .iter()
                .flat_map(|type_parameters| type_parameters.names.iter())
                .find_map(|parameter_name| {
                    if range_includes_position(
                        name_range(with_start_position_as_ref(parameter_name)),
                        position,
                    ) {
                        Some(SyntaxSymbol::TypeVariable {
                            name: &parameter_name.value,
                            use_start: parameter_name.start,
                            scope: item,
                        })
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    parameters.iter().find_map(|parameter| {
                        pattern_symbol_at_position(
                            parameter,
                            None,
                            position,
                            type_aliases,
                            checked_spread_records,
                            patterns,
                            item,
                            result.as_ref(),
                        )
                    })
                })
                .or_else(|| {
                    result_type.as_ref().and_then(|result_type| {
                        type_symbol_at_position(result_type, position, item)
                    })
                })
                .or_else(|| {
                    result.as_ref().and_then(|result| {
                        let mut local_variables = std::collections::HashMap::new();
                        for parameter in parameters {
                            syntax_parameter_pattern_variables_fold(
                                parameter,
                                (),
                                &mut |(), parameter_variable_name, parameter_variable_type| {
                                    local_variables.insert(
                                        parameter_variable_name.value,
                                        LocalVariableSymbolOrigin {
                                            start: parameter_variable_name.start,
                                            scope: Some(result),
                                            type_: parameter_variable_type.and_then(|type_| {
                                                syntax_type_to_type(type_, type_aliases)
                                            }),
                                        },
                                    );
                                },
                                patterns,
                            );
                        }
                        expression_symbol_at_position(
                            result,
                            position,
                            type_aliases,
                            checked_queries,
                            checked_spread_records,
                            expressions,
                            patterns,
                            item,
                            &mut local_variables,
                        )
                    })
                })
        }
        SyntaxProjectItem::Comments(_) => None,
        SyntaxProjectItem::Unrecognized { .. } => None,
    })
}
fn expression_symbol_at_position<'a, Expressions, Patterns>(
    expression: &'a SyntaxExpression<Expressions, Patterns>,
    position: lsp_types::Position,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    expressions: &'a core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
    scope: &'a SyntaxProjectItem<Expressions, Patterns>,
    local_variables: &mut std::collections::HashMap<
        &'a Name,
        LocalVariableSymbolOrigin<'a, Expressions, Patterns>,
    >,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns>> {
    if !range_includes_position(
        expression_range(expression, expressions, patterns),
        position,
    ) {
        return None;
    }
    match expression {
        SyntaxExpression::Number { value: _, type_ } => type_
            .as_ref()
            .and_then(|value| type_symbol_at_position(value, position, scope)),
        SyntaxExpression::Text {
            open_quote_start: _,
            content: _,
            content_end: _,
            closed_quote_exists: _,
            type_,
        } => type_
            .as_ref()
            .and_then(|value| type_symbol_at_position(value, position, scope)),
        SyntaxExpression::Variable(name) => Some(
            local_variables
                .remove(&name.value)
                .map(|local_variable| SyntaxSymbol::LocalVariable {
                    name: &name.value,
                    use_start: name.start,
                    origin: local_variable,
                })
                .unwrap_or_else(|| SyntaxSymbol::VariableUnknown {
                    local_variables: std::mem::take(local_variables),
                }),
        ),
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments,
            open_paren_start: _,
            arguments_rest: arguments,
            closed_paren_start: _,
        } => {
            if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
                return Some(SyntaxSymbol::ProjectFnOrUnknown {
                    name: with_start_position_as_ref(name),
                    construct_info: ConstructInfo::ArgumentExists,
                });
            }
            type_arguments
                .iter()
                .flat_map(|type_argument| &type_argument.types)
                .find_map(|type_argument| type_symbol_at_position(type_argument, position, scope))
                .or_else(|| {
                    first_argument_dot.as_ref().and_then(|first_argument_dot| {
                        expression_symbol_at_position(
                            expressions.item(&first_argument_dot.argument),
                            position,
                            type_aliases,
                            checked_queries,
                            checked_spread_records,
                            expressions,
                            patterns,
                            scope,
                            local_variables,
                        )
                    })
                })
                .or_else(|| {
                    arguments.iter().find_map(|argument| {
                        expression_symbol_at_position(
                            argument,
                            position,
                            type_aliases,
                            checked_queries,
                            checked_spread_records,
                            expressions,
                            patterns,
                            scope,
                            local_variables,
                        )
                    })
                })
        }
        SyntaxExpression::Variant {
            open_bracket_start: _,
            type_,
            name,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(name) = name
                && range_includes_position(name_range(with_start_position_as_ref(name)), position)
            {
                return Some(SyntaxSymbol::VariantOrUnknown(WithStartPosition {
                    start: name.start,
                    value: &name.value,
                }));
            }
            type_
                .as_ref()
                .and_then(|type_argument| type_argument.type_.as_ref())
                .and_then(|type_| type_symbol_at_position(type_, position, scope))
                .or_else(|| {
                    value.as_ref().and_then(|value| {
                        expression_symbol_at_position(
                            expressions.item(value),
                            position,
                            type_aliases,
                            checked_queries,
                            checked_spread_records,
                            expressions,
                            patterns,
                            scope,
                            local_variables,
                        )
                    })
                })
        }
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            let result = result.as_ref().map(|result| expressions.item(result));
            local_variables.clear();
            parameters
                .iter()
                .find_map(|parameter| {
                    pattern_symbol_at_position(
                        parameter,
                        None,
                        position,
                        type_aliases,
                        checked_spread_records,
                        patterns,
                        scope,
                        result,
                    )
                })
                .or_else(|| {
                    result.as_ref().and_then(|result| {
                        for parameter in parameters {
                            syntax_parameter_pattern_variables_fold(
                                parameter,
                                (),
                                &mut |(), name, type_| {
                                    local_variables.insert(
                                        name.value,
                                        LocalVariableSymbolOrigin {
                                            start: name.start,
                                            scope: Some(result),
                                            type_: type_.and_then(|type_| {
                                                syntax_type_to_type(type_, type_aliases)
                                            }),
                                        },
                                    );
                                },
                                patterns,
                            );
                        }
                        expression_symbol_at_position(
                            result,
                            position,
                            type_aliases,
                            checked_queries,
                            checked_spread_records,
                            expressions,
                            patterns,
                            scope,
                            local_variables,
                        )
                    })
                })
        }
        SyntaxExpression::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => parts.iter().find_map(|part| {
            expression_symbol_at_position(
                part,
                position,
                type_aliases,
                checked_queries,
                checked_spread_records,
                expressions,
                patterns,
                scope,
                local_variables,
            )
        }),
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => fields.iter().find_map(|field| {
            field.value.as_ref().and_then(|value| {
                expression_symbol_at_position(
                    value,
                    position,
                    type_aliases,
                    checked_queries,
                    checked_spread_records,
                    expressions,
                    patterns,
                    scope,
                    local_variables,
                )
            })
        }),
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => expression.as_ref().and_then(|expression| {
            expression_symbol_at_position(
                expressions.item(expression),
                position,
                type_aliases,
                checked_queries,
                checked_spread_records,
                expressions,
                patterns,
                scope,
                local_variables,
            )
        }),
        SyntaxExpression::Destruct {
            equals_start,
            pattern,
            queried,
            result,
        } => queried
            .as_ref()
            .and_then(|queried| {
                expression_symbol_at_position(
                    expressions.item(queried),
                    position,
                    type_aliases,
                    checked_queries,
                    checked_spread_records,
                    expressions,
                    patterns,
                    scope,
                    local_variables,
                )
            })
            .or_else(|| {
                let result = result.as_ref().map(|result| expressions.item(result));
                let expected_pattern_type = checked_queries
                    .get(equals_start)
                    .map(|checked_query| &checked_query.queried_type);
                pattern
                    .as_ref()
                    .and_then(|pattern| {
                        pattern_symbol_at_position(
                            pattern,
                            expected_pattern_type,
                            position,
                            type_aliases,
                            checked_spread_records,
                            patterns,
                            scope,
                            result,
                        )
                    })
                    .or_else(|| {
                        let Some(result) = result else {
                            return None;
                        };
                        if let Some(pattern) = pattern {
                            syntax_if_pattern_variables_fold(
                                pattern,
                                expected_pattern_type,
                                (),
                                &mut |(), name, type_| {
                                    local_variables.insert(
                                        name.value,
                                        LocalVariableSymbolOrigin {
                                            start: name.start,
                                            scope: Some(result),
                                            type_: type_,
                                        },
                                    );
                                },
                                patterns,
                            );
                        }
                        expression_symbol_at_position(
                            result,
                            position,
                            type_aliases,
                            checked_queries,
                            checked_spread_records,
                            expressions,
                            patterns,
                            scope,
                            local_variables,
                        )
                    })
            }),
        SyntaxExpression::If {
            if_keyword_start,
            queried,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => queried
            .as_ref()
            .and_then(|queried| {
                expression_symbol_at_position(
                    expressions.item(queried),
                    position,
                    type_aliases,
                    checked_queries,
                    checked_spread_records,
                    expressions,
                    patterns,
                    scope,
                    local_variables,
                )
            })
            .or_else(|| {
                cases.iter().find_map(|case| {
                    let checked_query = checked_queries.get(if_keyword_start);
                    expression_if_case_symbol_at_position(
                        case,
                        checked_query.map(|checked_query| &checked_query.queried_type),
                        position,
                        type_aliases,
                        checked_queries,
                        checked_spread_records,
                        expressions,
                        patterns,
                        scope,
                        local_variables,
                    )
                })
            }),
    }
}
fn expression_if_case_symbol_at_position<'a, Expressions, Patterns>(
    case: &'a SyntaxExpressionIfCase<Expressions, Patterns>,
    expected_pattern_type: Option<&Type>,
    position: lsp_types::Position,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    expressions: &'a core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
    scope: &'a SyntaxProjectItem<Expressions, Patterns>,
    local_variables: &mut std::collections::HashMap<
        &'a Name,
        LocalVariableSymbolOrigin<'a, Expressions, Patterns>,
    >,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns>> {
    pattern_symbol_at_position(
        &case.pattern,
        expected_pattern_type,
        position,
        type_aliases,
        checked_spread_records,
        patterns,
        scope,
        case.result.as_ref(),
    )
    .or_else(|| {
        let Some(result) = &case.result else {
            return None;
        };
        // don't modify local_variables unless known to be in the right case
        if !range_includes_position(expression_range(result, expressions, patterns), position) {
            return None;
        }
        syntax_if_pattern_variables_fold(
            &case.pattern,
            expected_pattern_type,
            (),
            &mut |(), name, type_| {
                local_variables.insert(
                    name.value,
                    LocalVariableSymbolOrigin {
                        start: name.start,
                        scope: Some(result),
                        type_: type_,
                    },
                );
            },
            patterns,
        );
        expression_symbol_at_position(
            result,
            position,
            type_aliases,
            checked_queries,
            checked_spread_records,
            expressions,
            patterns,
            scope,
            local_variables,
        )
    })
}
fn syntax_parameter_pattern_variables_fold<'a, Patterns, State>(
    pattern: &'a SyntaxPattern<Patterns>,
    state: State,
    reduce: &mut impl FnMut(State, WithStartPosition<&'a Name>, Option<&'a SyntaxType>) -> State,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> State {
    match pattern {
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_: _,
        } => state,
        SyntaxPattern::Variable { name, type_ } => {
            reduce(state, with_start_position_as_ref(name), type_.as_ref())
        }
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name: _,
            value,
            closed_bracket_start: _,
        } => match value {
            None => state,
            Some(value) => syntax_parameter_pattern_variables_fold(
                patterns.item(value),
                state,
                reduce,
                patterns,
            ),
        },
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => parts.iter().fold(state, |state, part| {
            syntax_parameter_pattern_variables_fold(part, state, reduce, patterns)
        }),
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => fields
            .iter()
            .fold(state, |state, field| match &field.value {
                None => state,
                Some(value) => {
                    syntax_parameter_pattern_variables_fold(value, state, reduce, patterns)
                }
            }),
    }
}
fn syntax_if_pattern_variables_fold<'a, Patterns, State>(
    pattern: &'a SyntaxPattern<Patterns>,
    expected_type: Option<&Type>,
    state: State,
    reduce: &mut impl FnMut(State, WithStartPosition<&'a Name>, Option<Type>) -> State,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> State {
    match pattern {
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_: _,
        } => state,
        SyntaxPattern::Variable { name, type_: _ } => reduce(
            state,
            with_start_position_as_ref(name),
            expected_type.cloned(),
        ),
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => match value {
            None => state,
            Some(value) => syntax_if_pattern_variables_fold(
                patterns.item(value),
                name.as_ref().and_then(|name| {
                    expected_type
                        .and_then(|expected_type| match expected_type {
                            Type::Choice(expected_variants) => expected_variants
                                .iter()
                                .find(|expected_variant| name.value == expected_variant.name),
                            _ => None,
                        })
                        .map(|variant| &variant.value)
                }),
                state,
                reduce,
                patterns,
            ),
        },
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => parts
            .iter()
            .enumerate()
            .fold(state, |state, (part_index, part)| {
                syntax_if_pattern_variables_fold(
                    part,
                    match expected_type {
                        Some(Type::Tuple(expected_parts)) => expected_parts.get(part_index),
                        _ => None,
                    },
                    state,
                    reduce,
                    patterns,
                )
            }),
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => fields
            .iter()
            .fold(state, |state, field| match &field.value {
                None => state,
                Some(value) => syntax_if_pattern_variables_fold(
                    value,
                    expected_type
                        .and_then(|expected_type| match expected_type {
                            Type::Record(expected_fields) => expected_fields
                                .iter()
                                .find(|expected_field| field.name.value == expected_field.name),
                            _ => None,
                        })
                        .map(|field| &field.value),
                    state,
                    reduce,
                    patterns,
                ),
            }),
    }
}
fn pattern_symbol_at_position<'a, Expressions, Patterns>(
    pattern: &'a SyntaxPattern<Patterns>,
    expected_type: Option<&Type>,
    position: lsp_types::Position,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_spread_records: &std::collections::HashMap<lsp_types::Position, Vec<Name>>,
    patterns: &'a core::Buf<Patterns, SyntaxPattern<Patterns>>,
    project_item_scope: &'a SyntaxProjectItem<Expressions, Patterns>,
    expression_scope: Option<&'a SyntaxExpression<Expressions, Patterns>>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns>> {
    if !range_includes_position(pattern_range(pattern, patterns), position) {
        return None;
    }
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
                return Some(SyntaxSymbol::LocalVariable {
                    name: &name.value,
                    use_start: name.start,
                    origin: LocalVariableSymbolOrigin {
                        start: name.start,
                        scope: expression_scope,
                        type_: expected_type.cloned().or_else(|| {
                            type_
                                .as_ref()
                                .and_then(|type_| syntax_type_to_type(type_, type_aliases))
                        }),
                    },
                });
            }
            type_
                .as_ref()
                .and_then(|type_| type_symbol_at_position(type_, position, project_item_scope))
        }
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_,
        } => type_
            .as_ref()
            .and_then(|type_| type_symbol_at_position(type_, position, project_item_scope)),
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => {
            let Some(name) = name else {
                return None;
            };
            if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
                return Some(SyntaxSymbol::VariantOrUnknown(with_start_position_as_ref(
                    name,
                )));
            }
            value.as_ref().and_then(|value| {
                pattern_symbol_at_position(
                    patterns.item(value),
                    expected_type
                        .and_then(|expected_type| match expected_type {
                            Type::Choice(expected_variants) => expected_variants
                                .iter()
                                .find(|expected_variant| name.value == expected_variant.name),
                            _ => None,
                        })
                        .map(|variant| &variant.value),
                    position,
                    type_aliases,
                    checked_spread_records,
                    patterns,
                    project_item_scope,
                    expression_scope,
                )
            })
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => parts.iter().enumerate().find_map(|(part_index, part)| {
            pattern_symbol_at_position(
                part,
                match expected_type {
                    Some(Type::Tuple(expected_parts)) => expected_parts.get(part_index),
                    _ => None,
                },
                position,
                type_aliases,
                checked_spread_records,
                patterns,
                project_item_scope,
                expression_scope,
            )
        }),
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => fields.iter().find_map(|field| {
            field.value.as_ref().and_then(|value| {
                pattern_symbol_at_position(
                    value,
                    expected_type
                        .and_then(|expected_type| match expected_type {
                            Type::Record(expected_fields) => expected_fields
                                .iter()
                                .find(|expected_field| field.name.value == &expected_field.name),
                            _ => None,
                        })
                        .map(|field| &field.value),
                    position,
                    type_aliases,
                    checked_spread_records,
                    patterns,
                    project_item_scope,
                    expression_scope,
                )
            })
        }),
    }
}
fn type_symbol_at_position<'a, Expressions, Patterns>(
    type_: &'a SyntaxType,
    position: lsp_types::Position,

    scope: &'a SyntaxProjectItem<Expressions, Patterns>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns>> {
    if !range_includes_position(type_range(type_), position) {
        return None;
    }
    match type_ {
        SyntaxType::Variable(name) => {
            if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
                Some(SyntaxSymbol::TypeVariable {
                    name: &name.value,
                    use_start: name.start,
                    scope: scope,
                })
            } else {
                None
            }
        }
        SyntaxType::Origin { hat_start: _, name } => {
            let Some(name) = name else {
                return None;
            };
            if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
                Some(SyntaxSymbol::Origin {
                    name: &name.value,
                    use_start: name.start,
                })
            } else {
                None
            }
        }
        SyntaxType::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => parameters
            .iter()
            .find_map(|parameter| type_symbol_at_position(parameter, position, scope))
            .or_else(|| {
                result
                    .as_ref()
                    .and_then(|result| type_symbol_at_position(result, position, scope))
            }),
        SyntaxType::ConstructWithArguments { name, arguments } => {
            if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
                return Some(SyntaxSymbol::ProjectTypeOrUnknown {
                    name: with_start_position_as_ref(name),
                    construct_info: match arguments {
                        None => ConstructInfo::ArgumentMissing,
                        Some(_) => ConstructInfo::ArgumentExists,
                    },
                });
            }
            arguments
                .iter()
                .flat_map(|arguments| &arguments.arguments)
                .find_map(|argument| type_symbol_at_position(argument, position, scope))
        }
        SyntaxType::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => parts
            .iter()
            .find_map(|part| type_symbol_at_position(part, position, scope)),
        SyntaxType::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => fields_find_symbol_at_position(fields, |_, value| {
            type_symbol_at_position(value, position, scope)
        }),
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => variants
            .iter()
            .filter_map(|variant| variant.value.as_ref())
            .find_map(|value| type_symbol_at_position(value, position, scope)),
    }
}
// TODO inline or fix closure type
fn fields_find_symbol_at_position<'a, Value, Expressions, Patterns>(
    fields: &'a [SyntaxRecordField<Value>],
    mut value_symbol_at_position: impl FnMut(
        Option<&Name>,
        &'a Value,
    ) -> Option<SyntaxSymbol<'a, Expressions, Patterns>>,
) -> Option<SyntaxSymbol<'a, Expressions, Patterns>> {
    fields.iter().find_map(|field| {
        field
            .value
            .as_ref()
            .and_then(|field_value| value_symbol_at_position(Some(&field.name.value), field_value))
    })
}

pub fn syntax_project_symbol_origin_range<Expressions, Patterns>(
    project: &SyntaxProject<Expressions, Patterns>,
    symbol: &SyntaxSymbol<Expressions, Patterns>,
) -> Option<lsp_types::Range> {
    match symbol {
        SyntaxSymbol::ProjectTypeOrUnknown {
            name: symbol_name,
            construct_info: _,
        } => project.items.iter().find_map(|item| match item {
            SyntaxProjectItem::TypeAlias {
                ty_keyword_start: _,
                name: type_alias_name,
                parameters: _,
                documentation: _,
                type_: _,
            } => {
                if let Some(type_alias_name) = type_alias_name
                    && &type_alias_name.value == symbol_name.value
                {
                    Some(name_range(with_start_position_as_ref(type_alias_name)))
                } else {
                    None
                }
            }
            SyntaxProjectItem::Fn { .. }
            | SyntaxProjectItem::Comments(_)
            | SyntaxProjectItem::Unrecognized { .. } => None,
        }),
        SyntaxSymbol::Origin { name, use_start } => Some(name_range(WithStartPosition {
            value: name,
            start: *use_start,
        })),
        &SyntaxSymbol::TypeVariable {
            name: symbol_name,
            use_start: _,
            scope,
        } => match scope {
            SyntaxProjectItem::TypeAlias {
                ty_keyword_start: _,
                name: _,
                parameters,
                documentation: _,
                type_: _,
            } => parameters.as_ref().and_then(|parameters| {
                parameters.parameters.iter().find_map(|name| {
                    if &name.value == symbol_name {
                        Some(name_range(with_start_position_as_ref(name)))
                    } else {
                        None
                    }
                })
            }),
            SyntaxProjectItem::Fn {
                fn_keyword_start: _,
                name: _,
                type_parameters,
                open_paren_start: _,
                parameters: _,
                closed_paren_start: _,
                result_type: _,
                documentation: _,
                result: _,
            } => type_parameters
                .iter()
                .flat_map(|parameter| &parameter.names)
                .find_map(|name| {
                    if &name.value == symbol_name {
                        Some(name_range(with_start_position_as_ref(name)))
                    } else {
                        None
                    }
                }),
            SyntaxProjectItem::Comments(_) | SyntaxProjectItem::Unrecognized { .. } => None,
        },
        SyntaxSymbol::VariantOrUnknown(_) => None,
        SyntaxSymbol::ProjectFnOrUnknown {
            name: symbol_name,
            construct_info: _,
        } => project.items.iter().find_map(|item| match item {
            SyntaxProjectItem::Fn {
                name: Some(fn_name),
                ..
            } if &fn_name.value == symbol_name.value => {
                Some(name_range(with_start_position_as_ref(fn_name)))
            }
            _ => None,
        }),
        SyntaxSymbol::LocalVariable {
            name,
            use_start: _,
            origin,
        } => Some(name_range(WithStartPosition {
            value: name,
            start: origin.start,
        })),
        SyntaxSymbol::VariableUnknown { .. } => None,
    }
}
/// resulting uses do not include the origin. For that, use syntax_project_symbol_origin_range
pub fn syntax_project_symbol_uses<Expressions, Patterns>(
    project: &SyntaxProject<Expressions, Patterns>,
    symbol: &SyntaxSymbol<Expressions, Patterns>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) -> Vec<lsp_types::Range> {
    let mut uses = Vec::with_capacity(2);
    match symbol {
        SyntaxSymbol::TypeVariable {
            name: _,
            use_start: _,
            scope,
        } => match scope {
            SyntaxProjectItem::TypeAlias {
                ty_keyword_start: _,
                name: _,
                parameters: _,
                documentation: _,
                type_,
            } => {
                if let Some(type_) = type_ {
                    syntax_type_symbol_uses_into(&mut uses, type_, symbol);
                }
            }
            SyntaxProjectItem::Fn {
                fn_keyword_start: _,
                name: _,
                type_parameters: _,
                open_paren_start: _,
                parameters: parameter,
                closed_paren_start: _,
                result_type,
                documentation: _,
                result,
            } => {
                for parameter in parameter {
                    syntax_pattern_symbol_uses_into(&mut uses, parameter, symbol, patterns);
                }
                if let Some(result_type) = result_type {
                    syntax_type_symbol_uses_into(&mut uses, result_type, symbol);
                }
                if let Some(result) = result {
                    syntax_expression_symbol_uses_into(
                        &mut uses,
                        result,
                        symbol,
                        expressions,
                        patterns,
                        &std::collections::HashSet::new(),
                    );
                }
            }
            SyntaxProjectItem::Comments(_) => {}
            SyntaxProjectItem::Unrecognized { .. } => {}
        },
        SyntaxSymbol::LocalVariable {
            name: _,
            use_start: _,
            origin,
        } => {
            if let Some(scope) = origin.scope {
                syntax_expression_symbol_uses_into(
                    &mut uses,
                    scope,
                    symbol,
                    expressions,
                    patterns,
                    &std::collections::HashSet::new(),
                );
            }
        }
        SyntaxSymbol::Origin {
            name: _,
            use_start: _,
        } => {
            for item in &project.items {
                match item {
                    SyntaxProjectItem::Fn {
                        fn_keyword_start: _,
                        name: _,
                        type_parameters: _,
                        open_paren_start: _,
                        parameters,
                        closed_paren_start: _,
                        result_type,
                        documentation: _,
                        result,
                    } => {
                        for parameter in parameters {
                            syntax_pattern_symbol_uses_into(&mut uses, parameter, symbol, patterns);
                        }
                        if let Some(result_type) = result_type {
                            syntax_type_symbol_uses_into(&mut uses, result_type, symbol);
                        }
                        if let Some(result) = result {
                            syntax_expression_symbol_uses_into(
                                &mut uses,
                                result,
                                symbol,
                                expressions,
                                patterns,
                                &std::collections::HashSet::new(),
                            );
                        }
                    }
                    SyntaxProjectItem::TypeAlias {
                        ty_keyword_start: _,
                        name: _,
                        parameters: _,
                        documentation: _,
                        type_,
                    } => {
                        if let Some(type_) = type_ {
                            syntax_type_symbol_uses_into(&mut uses, type_, symbol);
                        }
                    }
                    SyntaxProjectItem::Comments(_) => {}
                    SyntaxProjectItem::Unrecognized { .. } => {}
                }
            }
        }
        SyntaxSymbol::ProjectTypeOrUnknown { .. } => {
            for item in &project.items {
                match item {
                    SyntaxProjectItem::Fn {
                        fn_keyword_start: _,
                        name: _,
                        type_parameters: _,
                        open_paren_start: _,
                        parameters,
                        closed_paren_start: _,
                        result_type,
                        documentation: _,
                        result,
                    } => {
                        for parameter in parameters {
                            syntax_pattern_symbol_uses_into(&mut uses, parameter, symbol, patterns);
                        }
                        if let Some(result_type) = result_type {
                            syntax_type_symbol_uses_into(&mut uses, result_type, symbol);
                        }
                        if let Some(result) = result {
                            syntax_expression_symbol_uses_into(
                                &mut uses,
                                result,
                                symbol,
                                expressions,
                                patterns,
                                &std::collections::HashSet::new(),
                            );
                        }
                    }
                    SyntaxProjectItem::TypeAlias {
                        ty_keyword_start: _,
                        name: _,
                        parameters: _,
                        documentation: _,
                        type_,
                    } => {
                        if let Some(type_) = type_ {
                            syntax_type_symbol_uses_into(&mut uses, type_, symbol);
                        }
                    }
                    SyntaxProjectItem::Comments(_) => {}
                    SyntaxProjectItem::Unrecognized { .. } => {}
                }
            }
        }
        SyntaxSymbol::VariantOrUnknown(_) | SyntaxSymbol::ProjectFnOrUnknown { .. } => {
            for item in &project.items {
                match item {
                    SyntaxProjectItem::Fn {
                        fn_keyword_start: _,
                        name: _,
                        type_parameters: _,
                        open_paren_start: _,
                        parameters,
                        closed_paren_start: _,
                        result_type,
                        documentation: _,
                        result,
                    } => {
                        for parameter in parameters {
                            syntax_pattern_symbol_uses_into(&mut uses, parameter, symbol, patterns);
                        }
                        if let Some(result_type) = result_type {
                            syntax_type_symbol_uses_into(&mut uses, result_type, symbol);
                        }
                        if let Some(result) = result {
                            let mut parameter_introduced_variables =
                                std::collections::HashSet::new();
                            for parameter in parameters {
                                syntax_parameter_pattern_variables_fold(
                                    parameter,
                                    (),
                                    &mut |(), parameter_introduced_variable_name, _type_| {
                                        parameter_introduced_variables
                                            .insert(parameter_introduced_variable_name.value);
                                    },
                                    patterns,
                                );
                            }
                            syntax_expression_symbol_uses_into(
                                &mut uses,
                                result,
                                symbol,
                                expressions,
                                patterns,
                                &parameter_introduced_variables,
                            );
                        }
                    }
                    SyntaxProjectItem::TypeAlias { .. } => {}
                    SyntaxProjectItem::Comments(_) => {}
                    SyntaxProjectItem::Unrecognized { .. } => {}
                }
            }
        }
        SyntaxSymbol::VariableUnknown { .. } => {}
    }
    uses
}
fn syntax_type_symbol_uses_into<Expressions, Patterns>(
    uses: &mut Vec<lsp_types::Range>,
    type_: &SyntaxType,
    symbol: &SyntaxSymbol<Expressions, Patterns>,
) {
    match type_ {
        SyntaxType::Variable(name) => {
            if let &SyntaxSymbol::TypeVariable {
                name: symbol_name,
                use_start: _,
                scope: _,
            } = symbol
                && name.value == symbol_name
            {
                uses.push(name_range(with_start_position_as_ref(name)));
            }
        }
        SyntaxType::Origin { hat_start: _, name } => {
            if let Some(name) = name
                && let &SyntaxSymbol::Origin {
                    name: symbol_name,
                    use_start: _,
                } = symbol
                && name.value == symbol_name
            {
                uses.push(name_range(with_start_position_as_ref(name)));
            }
        }
        SyntaxType::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            for parameter in parameters {
                syntax_type_symbol_uses_into(uses, parameter, symbol);
            }
            if let Some(result) = result {
                syntax_type_symbol_uses_into(uses, result, symbol);
            }
        }
        SyntaxType::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_type_symbol_uses_into(uses, part, symbol);
            }
        }
        SyntaxType::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field_value in fields.iter().filter_map(|field| field.value.as_ref()) {
                syntax_type_symbol_uses_into(uses, field_value, symbol);
            }
        }
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => {
            for variant_value in variants.iter().filter_map(|variant| variant.value.as_ref()) {
                syntax_type_symbol_uses_into(uses, variant_value, symbol);
            }
        }
        SyntaxType::ConstructWithArguments { name, arguments } => {
            if let SyntaxSymbol::ProjectTypeOrUnknown {
                name: symbol_name,
                construct_info: _,
            } = symbol
                && &name.value == symbol_name.value
            {
                uses.push(name_range(with_start_position_as_ref(name)));
            }
            if let Some(SyntaxTypeConstructParenthesizedArguments {
                open_paren_start: _,
                arguments,
                closed_paren_start: _,
            }) = arguments
            {
                for argument in arguments {
                    syntax_type_symbol_uses_into(uses, argument, symbol);
                }
            }
        }
    }
}
fn syntax_pattern_symbol_uses_into<Expressions, Patterns>(
    uses: &mut Vec<lsp_types::Range>,
    pattern: &SyntaxPattern<Patterns>,
    symbol: &SyntaxSymbol<Expressions, Patterns>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match pattern {
        SyntaxPattern::Variable { name: _, type_ } => {
            if let Some(type_) = type_ {
                syntax_type_symbol_uses_into(uses, type_, symbol);
            }
        }
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_,
        } => {
            if let Some(type_) = type_ {
                syntax_type_symbol_uses_into(uses, type_, symbol);
            }
        }
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(name) = name
                && let SyntaxSymbol::VariantOrUnknown(symbol_name) = symbol
                && name.value == symbol_name.value
            {
                uses.push(name_range(with_start_position_as_ref(name)));
            }
            if let Some(value) = value {
                syntax_pattern_symbol_uses_into(uses, patterns.item(value), symbol, patterns);
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_pattern_symbol_uses_into(uses, part, symbol, patterns);
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_pattern_symbol_uses_into(uses, value, symbol, patterns);
                }
            }
        }
    }
}
// it would most likely be better to separate out variables as e.g. there can only be one
fn syntax_expression_symbol_uses_into<Expressions, Patterns>(
    uses: &mut Vec<lsp_types::Range>,
    expression: &SyntaxExpression<Expressions, Patterns>,
    symbol: &SyntaxSymbol<Expressions, Patterns>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    local_variables: &std::collections::HashSet<&Name>,
) {
    match expression {
        SyntaxExpression::Number { value: _, type_ } => {
            if let Some(type_) = type_ {
                syntax_type_symbol_uses_into(uses, type_, symbol);
            }
        }
        SyntaxExpression::Text {
            open_quote_start: _,
            content: _,
            content_end: _,
            closed_quote_exists: _,
            type_,
        } => {
            if let Some(type_) = type_ {
                syntax_type_symbol_uses_into(uses, type_, symbol);
            }
        }
        SyntaxExpression::Variable(name) => match symbol {
            SyntaxSymbol::TypeVariable { .. }
            | SyntaxSymbol::VariantOrUnknown(_)
            | SyntaxSymbol::ProjectFnOrUnknown { .. }
            | SyntaxSymbol::VariableUnknown { .. }
            | SyntaxSymbol::Origin { .. } => {}
            SyntaxSymbol::ProjectTypeOrUnknown {
                name:
                    WithStartPosition {
                        start: _,
                        value: symbol_name,
                    },
                ..
            } => {
                if *symbol_name == &name.value && !local_variables.contains(&name.value) {
                    uses.push(name_range(with_start_position_as_ref(name)));
                }
            }
            SyntaxSymbol::LocalVariable {
                name: symbol_name,
                use_start: _,
                origin: _,
            } => {
                if *symbol_name == &name.value && !local_variables.contains(&name.value) {
                    uses.push(name_range(with_start_position_as_ref(name)));
                }
            }
        },
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments,
            open_paren_start: _,
            arguments_rest: arguments,
            closed_paren_start: _,
        } => {
            match symbol {
                SyntaxSymbol::TypeVariable { .. }
                | SyntaxSymbol::Origin { .. }
                | SyntaxSymbol::ProjectTypeOrUnknown { .. }
                | SyntaxSymbol::VariantOrUnknown(_)
                | SyntaxSymbol::VariableUnknown { .. } => {}
                SyntaxSymbol::LocalVariable {
                    name: symbol_name, ..
                } => {
                    if *symbol_name == &name.value {
                        uses.push(name_range(with_start_position_as_ref(name)));
                    }
                }
                SyntaxSymbol::ProjectFnOrUnknown {
                    name:
                        WithStartPosition {
                            start: _,
                            value: symbol_name,
                        },
                    construct_info: _,
                } => {
                    if *symbol_name == &name.value && !local_variables.contains(&name.value) {
                        uses.push(name_range(with_start_position_as_ref(name)));
                    }
                }
            }
            for type_argument in type_arguments.iter().flat_map(|args| &args.types) {
                syntax_type_symbol_uses_into(uses, type_argument, symbol);
            }
            if let Some(first_argument_dot) = first_argument_dot {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.item(&first_argument_dot.argument),
                    symbol,
                    expressions,
                    patterns,
                    local_variables,
                );
            }
            for argument in arguments {
                syntax_expression_symbol_uses_into(
                    uses,
                    argument,
                    symbol,
                    expressions,
                    patterns,
                    local_variables,
                );
            }
        }
        SyntaxExpression::Variant {
            open_bracket_start: _,
            type_: type_argument,
            name,
            value,
            closed_bracket_start: _,
        } => {
            if let SyntaxSymbol::VariantOrUnknown(symbol_name) = symbol
                && let Some(name) = name
                && &name.value == symbol_name.value
            {
                uses.push(name_range(with_start_position_as_ref(name)));
            }
            if let Some(type_argument) = type_argument
                && let Some(type_) = &type_argument.type_
            {
                syntax_type_symbol_uses_into(uses, type_, symbol);
            }
            if let Some(value) = value {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.item(value),
                    symbol,
                    expressions,
                    patterns,
                    local_variables,
                );
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            match symbol {
                SyntaxSymbol::LocalVariable { .. } => return,
                SyntaxSymbol::ProjectTypeOrUnknown { .. }
                | SyntaxSymbol::Origin { .. }
                | SyntaxSymbol::TypeVariable { .. }
                | SyntaxSymbol::VariantOrUnknown(_)
                | SyntaxSymbol::ProjectFnOrUnknown { .. }
                | SyntaxSymbol::VariableUnknown { .. } => {}
            }
            for parameter in parameters {
                syntax_pattern_symbol_uses_into(uses, parameter, symbol, patterns);
            }
            if let Some(result) = result {
                let mut parameter_pattern_variables = std::collections::HashSet::new();
                for parameter in parameters {
                    syntax_pattern_symbol_uses_into(uses, parameter, symbol, patterns);
                    syntax_parameter_pattern_variables_fold(
                        parameter,
                        (),
                        &mut |(), pattern_variable_name, _type_| {
                            parameter_pattern_variables.insert(pattern_variable_name.value);
                        },
                        patterns,
                    );
                }
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.item(result),
                    symbol,
                    expressions,
                    patterns,
                    &parameter_pattern_variables,
                );
            }
        }
        SyntaxExpression::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_expression_symbol_uses_into(
                    uses,
                    part,
                    symbol,
                    expressions,
                    patterns,
                    local_variables,
                );
            }
        }
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                syntax_expression_record_field_symbol_uses_into(
                    uses,
                    field,
                    symbol,
                    expressions,
                    patterns,
                    local_variables,
                );
            }
        }
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => {
            if let Some(expression) = expression {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.item(expression),
                    symbol,
                    expressions,
                    patterns,
                    local_variables,
                );
            }
        }
        SyntaxExpression::Destruct {
            equals_start: _,
            pattern,
            queried,
            result,
        } => {
            if let Some(queried) = queried {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.item(queried),
                    symbol,
                    expressions,
                    patterns,
                    local_variables,
                );
            }
            if let Some(pattern) = pattern {
                syntax_pattern_symbol_uses_into(uses, pattern, symbol, patterns);
            }
            if let Some(result) = result {
                let mut local_variables = std::borrow::Cow::Borrowed(local_variables);
                if let Some(pattern) = pattern {
                    syntax_parameter_pattern_variables_fold(
                        pattern,
                        (),
                        &mut |(), pattern_variable_name, _type_| {
                            local_variables.to_mut().insert(pattern_variable_name.value);
                        },
                        patterns,
                    );
                }
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.item(result),
                    symbol,
                    expressions,
                    patterns,
                    &local_variables,
                );
            }
        }
        SyntaxExpression::If {
            if_keyword_start: _,
            queried,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => {
            if let Some(queried) = queried {
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.item(queried),
                    symbol,
                    expressions,
                    patterns,
                    local_variables,
                );
            }
            for case in cases {
                if let Some(result) = &case.result {
                    let mut local_variables = std::borrow::Cow::Borrowed(local_variables);
                    syntax_pattern_symbol_uses_into(uses, &case.pattern, symbol, patterns);
                    syntax_if_pattern_variables_fold(
                        &case.pattern,
                        None,
                        (),
                        &mut |(), pattern_variable_name, _type_| {
                            local_variables.to_mut().insert(pattern_variable_name.value);
                        },
                        patterns,
                    );
                    syntax_expression_symbol_uses_into(
                        uses,
                        result,
                        symbol,
                        expressions,
                        patterns,
                        &local_variables,
                    );
                }
            }
        }
    }
}
fn syntax_expression_record_field_symbol_uses_into<Expressions, Patterns>(
    uses: &mut Vec<lsp_types::Range>,
    field: &SyntaxRecordField<SyntaxExpression<Expressions, Patterns>>,
    symbol: &SyntaxSymbol<Expressions, Patterns>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    local_variables: &std::collections::HashSet<&Name>,
) {
    if let Some(field_value) = &field.value {
        syntax_expression_symbol_uses_into(
            uses,
            field_value,
            symbol,
            expressions,
            patterns,
            local_variables,
        );
    }
}

pub struct HighlightState {
    pub tokens: Vec<lsp_types::SemanticToken>,
    pub previous_token_start: lsp_types::Position,
}
#[allow(clippy::needless_pass_by_value)]
fn highlight_state_add_token_with_start_and_length(
    state: &mut HighlightState,
    new_token_kind: lsp_types::SemanticTokenTypes,
    new_token_start: lsp_types::Position,
    new_token_length: usize,
) {
    if new_token_length == 0 {
        return;
    }
    match lsp_position_positive_delta(state.previous_token_start, new_token_start) {
        Err(error) => {
            eprintln!("bad highlight token order {error}");
        }
        Ok(delta) => {
            let token = lsp_types::SemanticToken {
                delta_line: delta.line,
                delta_start: delta.character,
                length: new_token_length as u32,
                token_type: semantic_token_type_to_id(&new_token_kind),
                token_modifiers_bitset: 0_u32,
            };
            state.previous_token_start = new_token_start;
            state.tokens.push(token);
        }
    }
}
fn symbol_highlight(
    state: &mut HighlightState,
    symbol: &'static str,
    new_token_start: lsp_types::Position,
    new_token_kind: lsp_types::SemanticTokenTypes,
) {
    highlight_state_add_token_with_start_and_length(
        state,
        new_token_kind,
        new_token_start,
        symbol.len(),
    );
}
fn keyword_highlight(
    state: &mut HighlightState,
    keyword: &'static str,
    new_token_start: lsp_types::Position,
) {
    symbol_highlight(
        state,
        keyword,
        new_token_start,
        lsp_types::SemanticTokenTypes::Keyword,
    );
}
pub fn project_highlight<Expressions, Patterns>(
    state: &mut HighlightState,
    project: &SyntaxProject<Expressions, Patterns>,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    for item in &project.items {
        match item {
            SyntaxProjectItem::Comments(comments) => {
                syntax_comments_highlight(state, comments);
            }
            SyntaxProjectItem::TypeAlias {
                ty_keyword_start,
                name,
                parameters,
                documentation,
                type_,
            } => {
                keyword_highlight(state, "ty", *ty_keyword_start);
                if let Some(name) = name {
                    syntax_name_highlight(
                        state,
                        name,
                        match parameters {
                            None => lsp_types::SemanticTokenTypes::Type,
                            Some(_) => lsp_types::SemanticTokenTypes::Function,
                        },
                    );
                }
                if let Some(parameters) = parameters {
                    for parameter_name in &parameters.parameters {
                        syntax_name_highlight(
                            state,
                            parameter_name,
                            lsp_types::SemanticTokenTypes::TypeParameter,
                        );
                    }
                }
                if let Some(documentation) = documentation {
                    syntax_comments_highlight(state, documentation);
                }
                if let Some(type_) = type_ {
                    syntax_type_highlight(state, type_);
                }
            }
            SyntaxProjectItem::Fn {
                fn_keyword_start,
                name,
                type_parameters,
                open_paren_start: _,
                parameters: parameter,
                closed_paren_start: _,
                result_type,
                documentation,
                result,
            } => {
                keyword_highlight(state, "fn", *fn_keyword_start);
                if let Some(name) = name {
                    highlight_state_add_token_with_start_and_length(
                        state,
                        lsp_types::SemanticTokenTypes::Function,
                        name.start,
                        name.value.len(),
                    );
                }
                if let Some(SyntaxAngledTypeParameters {
                    open_angle_start: _,
                    names: type_parameter_names,
                    closed_angle_start: _,
                }) = type_parameters
                {
                    for type_parameter_name in type_parameter_names {
                        syntax_name_highlight(
                            state,
                            type_parameter_name,
                            lsp_types::SemanticTokenTypes::TypeParameter,
                        );
                    }
                }
                for parameter in parameter {
                    syntax_pattern_highlight(state, patterns, parameter);
                }
                if let Some(result_type) = result_type {
                    syntax_type_highlight(state, result_type);
                }
                if let Some(documentation) = documentation {
                    syntax_comments_highlight(state, documentation);
                }
                if let Some(result) = result {
                    syntax_expression_highlight(state, expressions, patterns, result);
                }
            }
            SyntaxProjectItem::Unrecognized { .. } => {}
        }
    }
}
pub fn project_fn_signature_highlight(
    state: &mut HighlightState,
    ProjectFnSignature {
        fn_keyword_start,
        name,
        type_parameters,
        open_paren_start: _,
        parameter_types,
        closed_paren_start: _,
        result_type,
    }: &ProjectFnSignature,
) {
    keyword_highlight(state, "fn", *fn_keyword_start);
    if let Some(name) = name {
        highlight_state_add_token_with_start_and_length(
            state,
            lsp_types::SemanticTokenTypes::Variable,
            name.start,
            name.value.len(),
        );
    }
    if let Some(SyntaxAngledTypeParameters {
        open_angle_start: _,
        names: type_parameter_names,
        closed_angle_start: _,
    }) = type_parameters
    {
        for type_parameter_name in type_parameter_names {
            syntax_name_highlight(
                state,
                type_parameter_name,
                lsp_types::SemanticTokenTypes::TypeParameter,
            );
        }
    }
    for parameter_type in parameter_types {
        syntax_type_highlight(state, parameter_type);
    }
    if let Some(result_type) = result_type {
        syntax_type_highlight(state, result_type);
    }
}
fn syntax_comments_highlight(state: &mut HighlightState, comments: &SyntaxComments) {
    for line in std::iter::once(&comments.line0).chain(comments.line1_up.iter()) {
        highlight_state_add_token_with_start_and_length(
            state,
            lsp_types::SemanticTokenTypes::Comment,
            line.start,
            1 + line.value.encode_utf16().count(),
        );
    }
}
fn syntax_name_highlight(
    state: &mut HighlightState,
    name: &WithStartPosition<Name>,
    kind: lsp_types::SemanticTokenTypes,
) {
    highlight_state_add_token_with_start_and_length(
        state,
        kind,
        name.start,
        name.value.encode_utf16().count(),
    );
}
fn syntax_field_name_highlight(state: &mut HighlightState, field_name: &WithStartPosition<Name>) {
    syntax_name_highlight(state, field_name, lsp_types::SemanticTokenTypes::Property);
}
fn syntax_variant_name_highlight(
    state: &mut HighlightState,
    variant_name: &WithStartPosition<Name>,
) {
    syntax_name_highlight(
        state,
        variant_name,
        lsp_types::SemanticTokenTypes::EnumMember,
    );
}
fn syntax_pattern_highlight<Patterns>(
    state: &mut HighlightState,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,

    pattern: &SyntaxPattern<Patterns>,
) {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            syntax_name_highlight(state, name, lsp_types::SemanticTokenTypes::Parameter);
            if let Some(type_) = type_ {
                syntax_type_highlight(state, type_);
            }
        }
        SyntaxPattern::Ignored {
            underscore_start,
            type_,
        } => {
            keyword_highlight(state, "_", *underscore_start);
            if let Some(type_) = type_ {
                syntax_type_highlight(state, type_);
            }
        }
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(name) = name {
                syntax_variant_name_highlight(state, name);
            }
            if let Some(value) = value {
                syntax_pattern_highlight(state, patterns, patterns.item(value));
            }
        }
        SyntaxPattern::Tuple {
            star_start,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            keyword_highlight(state, "*", *star_start);
            for part in parts {
                syntax_pattern_highlight(state, patterns, part);
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                syntax_field_name_highlight(state, &field.name);
                if let Some(value) = &field.value {
                    syntax_pattern_highlight(state, patterns, value);
                }
            }
        }
    }
}
pub fn syntax_type_highlight(state: &mut HighlightState, type_: &SyntaxType) {
    match type_ {
        SyntaxType::Variable(name) => {
            syntax_name_highlight(state, name, lsp_types::SemanticTokenTypes::TypeParameter);
        }
        SyntaxType::Origin { hat_start, name } => {
            keyword_highlight(state, "^", *hat_start);
            if let Some(name) = name {
                syntax_name_highlight(state, name, lsp_types::SemanticTokenTypes::TypeParameter);
            }
        }
        SyntaxType::Fn {
            fn_keyword_start,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            keyword_highlight(state, "fn", *fn_keyword_start);
            for parameter in parameters {
                syntax_type_highlight(state, parameter);
            }
            if let Some(result) = result {
                syntax_type_highlight(state, result);
            }
        }
        SyntaxType::ConstructWithArguments { name, arguments } => {
            syntax_name_highlight(state, name, lsp_types::SemanticTokenTypes::Function);
            if let Some(SyntaxTypeConstructParenthesizedArguments {
                open_paren_start: _,
                arguments,
                closed_paren_start: _,
            }) = arguments
            {
                for argument in arguments {
                    syntax_type_highlight(state, argument);
                }
            }
        }
        SyntaxType::Tuple {
            star_start,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            keyword_highlight(state, "*", *star_start);
            for part in parts {
                syntax_type_highlight(state, part);
            }
        }
        SyntaxType::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                syntax_field_name_highlight(state, &field.name);
                if let Some(value) = &field.value {
                    syntax_type_highlight(state, value);
                }
            }
        }
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => {
            for variant in variants {
                syntax_variant_name_highlight(state, &variant.name);
                if let Some(value) = &variant.value {
                    syntax_type_highlight(state, value);
                }
            }
        }
    }
}
fn syntax_expression_highlight<Expressions, Patterns>(
    state: &mut HighlightState,
    expressions: &core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &core::Buf<Patterns, SyntaxPattern<Patterns>>,
    expression: &SyntaxExpression<Expressions, Patterns>,
) {
    match expression {
        SyntaxExpression::Number { value, type_ } => {
            highlight_state_add_token_with_start_and_length(
                state,
                lsp_types::SemanticTokenTypes::Number,
                value.start,
                value.value.len(),
            );
            if let Some(type_) = type_ {
                syntax_type_highlight(state, type_);
            }
        }
        SyntaxExpression::Text {
            open_quote_start,
            content: _,
            content_end,
            closed_quote_exists,
            type_,
        } => {
            highlight_state_add_token_with_start_and_length(
                state,
                lsp_types::SemanticTokenTypes::String,
                *open_quote_start,
                (content_end.character - open_quote_start.character
                    + (if *closed_quote_exists { 1 } else { 0 })) as usize,
            );
            if let Some(type_) = type_ {
                syntax_type_highlight(state, type_);
            }
        }
        SyntaxExpression::Variable(name) => {
            syntax_name_highlight(state, name, lsp_types::SemanticTokenTypes::Variable);
        }
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments,
            open_paren_start: _,
            arguments_rest: arguments,
            closed_paren_start: _,
        } => {
            if let Some(first_argument_dot) = first_argument_dot {
                syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    expressions.item(&first_argument_dot.argument),
                );
                keyword_highlight(state, ".", first_argument_dot.dot_start);
            }
            syntax_name_highlight(state, name, lsp_types::SemanticTokenTypes::Function);
            if let Some(type_arguments) = type_arguments {
                for type_ in &type_arguments.types {
                    syntax_type_highlight(state, type_);
                }
            }
            for argument in arguments {
                syntax_expression_highlight(state, expressions, patterns, argument);
            }
        }
        SyntaxExpression::Variant {
            open_bracket_start: _,
            name,
            type_: type_argument,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(name) = name {
                syntax_name_highlight(state, name, lsp_types::SemanticTokenTypes::EnumMember);
            }
            if let Some(type_argument) = type_argument {
                if let Some(type_) = &type_argument.type_ {
                    syntax_type_highlight(state, type_);
                }
            }
            if let Some(value) = value {
                syntax_expression_highlight(state, expressions, patterns, expressions.item(value));
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            keyword_highlight(state, "fn", *fn_keyword_start);
            for parameter in parameters {
                syntax_pattern_highlight(state, patterns, parameter);
            }
            if let Some(result) = result {
                syntax_expression_highlight(state, expressions, patterns, expressions.item(result));
            }
        }
        SyntaxExpression::Tuple {
            star_start,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            keyword_highlight(state, "*", *star_start);
            for part in parts {
                syntax_expression_highlight(state, expressions, patterns, part);
            }
        }
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                syntax_field_name_highlight(state, &field.name);
                if let Some(value) = &field.value {
                    syntax_expression_highlight(state, expressions, patterns, value);
                }
            }
        }
        SyntaxExpression::Commented {
            comments,
            expression,
        } => {
            syntax_comments_highlight(state, comments);
            if let Some(expression) = expression {
                syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    expressions.item(expression),
                );
            }
        }
        SyntaxExpression::Destruct {
            equals_start,
            pattern,
            queried,
            result,
        } => {
            keyword_highlight(state, "=", *equals_start);
            if let Some(pattern) = pattern {
                syntax_pattern_highlight(state, patterns, pattern);
            }
            if let Some(queried) = queried {
                syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    expressions.item(queried),
                );
            }
            if let Some(result) = result {
                syntax_expression_highlight(state, expressions, patterns, expressions.item(result));
            }
        }
        SyntaxExpression::If {
            if_keyword_start,
            queried,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => {
            keyword_highlight(state, "if", *if_keyword_start);
            if let Some(queried) = queried {
                syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    expressions.item(queried),
                );
            }
            for case in cases {
                syntax_pattern_highlight(state, patterns, &case.pattern);
                if let Some(result) = &case.result {
                    syntax_expression_highlight(state, expressions, patterns, result);
                }
            }
        }
    }
}
pub const token_types: [lsp_types::SemanticTokenTypes; 12] = [
    lsp_types::SemanticTokenTypes::Number,
    lsp_types::SemanticTokenTypes::String,
    lsp_types::SemanticTokenTypes::Namespace,
    lsp_types::SemanticTokenTypes::Variable,
    lsp_types::SemanticTokenTypes::Type,
    lsp_types::SemanticTokenTypes::TypeParameter,
    lsp_types::SemanticTokenTypes::Keyword,
    lsp_types::SemanticTokenTypes::EnumMember,
    lsp_types::SemanticTokenTypes::Property,
    lsp_types::SemanticTokenTypes::Comment,
    lsp_types::SemanticTokenTypes::Function,
    lsp_types::SemanticTokenTypes::Parameter,
];

fn semantic_token_type_to_id(semantic_token: &lsp_types::SemanticTokenTypes) -> u32 {
    token_types
        .iter()
        .enumerate()
        .find_map(|(i, token)| {
            if token == semantic_token {
                Some(i as u32)
            } else {
                None
            }
        })
        .unwrap_or(0_u32)
}
#[derive(Copy, Clone)]
struct PositionDelta {
    pub line: u32,
    pub character: u32,
}
fn lsp_position_positive_delta(
    before: lsp_types::Position,
    after: lsp_types::Position,
) -> Result<PositionDelta, String> {
    match before.line.cmp(&after.line) {
        std::cmp::Ordering::Greater => Err(format!(
            "before line > after line (before: {}, after: {})",
            position_to_string(before),
            position_to_string(after)
        )),
        std::cmp::Ordering::Equal => {
            if before.character > after.character {
                Err(format!(
                    "before character > after character (before: {}, after: {})",
                    position_to_string(before),
                    position_to_string(after)
                ))
            } else {
                Ok(PositionDelta {
                    line: 0,
                    character: after.character - before.character,
                })
            }
        }
        std::cmp::Ordering::Less => Ok(PositionDelta {
            line: after.line - before.line,
            character: after.character,
        }),
    }
}

pub fn checked_project_fn_format(
    formatted: &mut String,
    name: &Name,
    project_fn: &CheckedProjectFn,
) {
    formatted.push_str("fn ");
    formatted.push_str(name);
    angled_type_parameters_format(formatted, &project_fn.type_parameters);
    formatted.push('(');
    for fn_parameter_type in project_fn.parameter_types.iter().flatten() {
        formatted.push_str("\n    ");
        type_format(formatted, 4, fn_parameter_type);
    }
    formatted.push_str("\n)\n    ");
    if let Some(fn_result_type) = &project_fn.result_type {
        type_format(formatted, 4, fn_result_type);
    }
}
pub fn angled_type_parameters_format(formatted: &mut String, type_parameters: &[Name]) {
    formatted.push('<');
    if let Some((angled_type_parameter0, angled_type_parameter1_up)) = type_parameters.split_first()
    {
        formatted.push_str(angled_type_parameter0);
        for angled_type_parameter in angled_type_parameter1_up {
            formatted.push_str(angled_type_parameter);
            formatted.push(' ');
        }
    }
    formatted.push('>');
}
pub fn checked_type_alias_format(
    formatted: &mut String,
    name: &Name,
    type_alias: &CheckedTypeAlias,
) {
    formatted.push_str("ty ");
    formatted.push_str(name);
    if let Some((parameter0, parameter1_up)) = type_alias.parameters.split_first() {
        formatted.push_str(" _");
        formatted.push_str(parameter0);
        for parameter in parameter1_up {
            formatted.push_str(", _");
            formatted.push_str(parameter);
        }
    }
    formatted.push_str("\n    ");
    if let Some(type_) = &type_alias.type_ {
        type_format(formatted, 4, type_);
    }
}
pub fn type_format(formatted: &mut String, indent: usize, type_: &Type) {
    match type_ {
        Type::Variable(name) => {
            formatted.push_str(name);
        }
        Type::Origin(name) => {
            formatted.push('^');
            formatted.push_str(name);
        }
        Type::CoreConstruct { name, arguments } => {
            formatted.push_str(name);
            if let Some((argument0, argument1_up)) = arguments.split_first() {
                formatted.push('(');
                let line_span: LineSpan = type_line_span(type_);
                type_format(formatted, next_indent(indent), argument0);
                for argument in argument1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    type_format(formatted, next_indent(indent), argument);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                formatted.push(')');
            }
        }
        Type::Fn { parameters, result } => {
            formatted.push_str("fn(");
            let line_span: LineSpan = type_line_span(type_);
            if let Some((parameter0, parameter1_up)) = parameters.split_first() {
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                type_format(formatted, next_indent(indent), parameter0);
                for parameter in parameter1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    type_format(formatted, next_indent(indent), parameter);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
            }
            formatted.push(')');
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            type_format(formatted, next_indent(indent), result);
        }
        Type::Tuple(parts) => {
            formatted.push('*');
            if let Some((part0, part1_up)) = parts.split_first() {
                formatted.push('(');
                let line_span: LineSpan = type_line_span(type_);
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                type_format(formatted, next_indent(indent), part0);
                for part in part1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    type_format(formatted, next_indent(indent), part);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                formatted.push(')');
            }
        }
        Type::Record(fields) => type_record_format(formatted, indent, fields),
        Type::Choice(variants) => {
            formatted.push('[');
            match variants.split_first() {
                None => {}
                Some((variant0, variant1_up)) => {
                    let line_span: LineSpan = type_line_span(type_);
                    if line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, next_indent(indent));
                    }
                    type_variant_format(formatted, indent, variant0);
                    for variant in variant1_up {
                        space_or_linebreak_indented_into(formatted, line_span, indent);
                        type_variant_format(formatted, indent, variant);
                    }
                    if line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, next_indent(indent));
                    }
                }
            }
            formatted.push(']');
        }
    }
}
fn type_record_format(formatted: &mut String, indent: usize, fields: &[TypeField]) {
    formatted.push('{');
    match fields.split_first() {
        None => {}
        Some((field0, field1_up)) => {
            let line_span: LineSpan = type_record_line_span(fields);
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            type_field_format(formatted, next_indent(indent), field0);
            for field in field1_up {
                space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                type_field_format(formatted, next_indent(indent), field);
            }
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
        }
    }
    formatted.push('}');
}

fn type_field_format(formatted: &mut String, indent: usize, type_field: &TypeField) {
    formatted.push_str(&type_field.name);
    let line_span = type_line_span(&type_field.value);
    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
    type_format(formatted, next_indent(indent), &type_field.value);
}
fn type_variant_format(formatted: &mut String, indent: usize, type_variant: &TypeVariant) {
    formatted.push_str(&type_variant.name);
    let line_span = type_line_span(&type_variant.value);
    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
    type_format(formatted, next_indent(indent), &type_variant.value);
}
fn type_line_span(type_: &Type) -> LineSpan {
    if type_length_estimate(type_) <= type_info_line_length_estimate_maximum {
        LineSpan::Single
    } else {
        LineSpan::Multiple
    }
}
fn type_record_line_span(fields: &[TypeField]) -> LineSpan {
    if type_record_length_estimate(fields) <= type_info_line_length_estimate_maximum {
        LineSpan::Single
    } else {
        LineSpan::Multiple
    }
}
fn type_length_estimate(type_: &Type) -> usize {
    match type_ {
        Type::Variable(variable_name) => variable_name.len(),
        Type::Origin(name) => name.len(),
        Type::CoreConstruct { name, arguments } => {
            1 + name.len()
                + arguments
                    .iter()
                    .map(|argument| 2 + type_length_estimate(argument))
                    .sum::<usize>()
        }
        Type::Fn { parameters, result } => {
            5 + parameters
                .iter()
                .map(|parameter| 1 + type_length_estimate(parameter))
                .sum::<usize>()
                + type_length_estimate(result)
        }
        Type::Tuple(parts) => {
            3 + parts
                .iter()
                .map(|part| 1 + type_length_estimate(part))
                .sum::<usize>()
        }
        Type::Record(fields) => type_record_length_estimate(fields),
        Type::Choice(variants) => variants
            .iter()
            .map(|variant| 3 + variant.name.len() + type_length_estimate(&variant.value))
            .sum(),
    }
}
fn type_record_length_estimate(fields: &[TypeField]) -> usize {
    4 + fields
        .iter()
        .map(|field| 2 + field.name.len() + type_length_estimate(&field.value))
        .sum::<usize>()
}

pub fn syntax_project_item_rid<Expressions, Patterns>(
    item: SyntaxProjectItem<Expressions, Patterns>,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match item {
        SyntaxProjectItem::TypeAlias {
            ty_keyword_start: _,
            name: _,
            parameters: _,
            documentation: _,
            type_: _,
        } => {}
        SyntaxProjectItem::Fn {
            fn_keyword_start: _,
            name: _,
            type_parameters: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result_type: _,
            documentation: _,
            result,
        } => {
            for parameter in parameters {
                syntax_pattern_rid(parameter, patterns);
            }
            if let Some(result) = result {
                syntax_expression_rid(result, expressions, patterns);
            }
        }
        SyntaxProjectItem::Comments(_) => {}
        SyntaxProjectItem::Unrecognized {
            range: _,
            source: _,
        } => {}
    }
}
fn syntax_pattern_rid<Patterns>(
    pattern: SyntaxPattern<Patterns>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match pattern {
        SyntaxPattern::Variable { name: _, type_: _ } => {}
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_: _,
        } => {}
        SyntaxPattern::Variant {
            open_bracket_start: _,
            name: _,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(value) = value {
                syntax_pattern_rid(patterns.remove(value), patterns);
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_pattern_rid(part, patterns);
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = field.value {
                    syntax_pattern_rid(value, patterns);
                }
            }
        }
    }
}
fn syntax_expression_rid<Expressions, Patterns>(
    expression: SyntaxExpression<Expressions, Patterns>,
    expressions: &mut core::Buf<Expressions, SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut core::Buf<Patterns, SyntaxPattern<Patterns>>,
) {
    match expression {
        SyntaxExpression::Number { value: _, type_: _ } => {}
        SyntaxExpression::Text {
            open_quote_start: _,
            content: _,
            content_end: _,
            closed_quote_exists: _,
            type_: _,
        } => {}
        SyntaxExpression::Variable(_) => {}
        SyntaxExpression::Call {
            first_argument_dot,
            name: _,
            type_arguments: _,
            open_paren_start: _,
            arguments_rest: arguments,
            closed_paren_start: _,
        } => {
            if let Some(SyntaxExpressionArgumentDot {
                argument,
                dot_start: _,
            }) = first_argument_dot
            {
                syntax_expression_rid(expressions.remove(argument), expressions, patterns);
            }
            for argument in arguments {
                syntax_expression_rid(argument, expressions, patterns);
            }
        }
        SyntaxExpression::Variant {
            open_bracket_start: _,
            type_: _,
            name: _,
            value,
            closed_bracket_start: _,
        } => {
            if let Some(value) = value {
                syntax_expression_rid(expressions.remove(value), expressions, patterns);
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            for paramter in parameters {
                syntax_pattern_rid(paramter, patterns);
            }
            if let Some(result) = result {
                syntax_expression_rid(expressions.remove(result), expressions, patterns);
            }
        }
        SyntaxExpression::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_expression_rid(part, expressions, patterns);
            }
        }
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = field.value {
                    syntax_expression_rid(value, expressions, patterns);
                }
            }
        }
        SyntaxExpression::Commented {
            comments: _,
            expression: after_comments,
        } => {
            if let Some(after_comments) = after_comments {
                syntax_expression_rid(expressions.remove(after_comments), expressions, patterns);
            }
        }
        SyntaxExpression::Destruct {
            equals_start: _,
            pattern,
            queried,
            result,
        } => {
            if let Some(pattern) = pattern {
                syntax_pattern_rid(pattern, patterns);
            }
            if let Some(queried) = queried {
                syntax_expression_rid(expressions.remove(queried), expressions, patterns);
            }
            if let Some(result) = result {
                syntax_expression_rid(expressions.remove(result), expressions, patterns);
            }
        }
        SyntaxExpression::If {
            if_keyword_start: _,
            queried,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => {
            if let Some(queried) = queried {
                syntax_expression_rid(expressions.remove(queried), expressions, patterns);
            }
            for SyntaxExpressionIfCase { pattern, result } in cases {
                syntax_pattern_rid(pattern, patterns);
                if let Some(result) = result {
                    syntax_expression_rid(result, expressions, patterns);
                }
            }
        }
    }
}

fn symbol_range(start: lsp_types::Position, symbol: &'static str) -> lsp_types::Range {
    lsp_types::Range {
        start: start,
        end: symbol_end(start, symbol),
    }
}
fn symbol_end(start: lsp_types::Position, symbol: &'static str) -> lsp_types::Position {
    position_add_characters(start, symbol.len() as u32)
}
pub fn with_start_position_as_ref<Value>(
    with_start_position: &WithStartPosition<Value>,
) -> WithStartPosition<&Value> {
    WithStartPosition {
        start: with_start_position.start,
        value: &with_start_position.value,
    }
}
pub fn str_lsp_range_to_utf8_range(str: &str, range: lsp_types::Range) -> std::ops::Range<usize> {
    let start_line_offset: usize =
        str_utf8_offset_after_n_lsp_linebreaks(str, range.start.line as usize);
    let start_offset: usize = start_line_offset
        + str_starting_utf8_length_for_utf16_length(
            &str[start_line_offset..],
            range.start.character as usize,
        );
    // can be optimized by only counting after the start line
    let end_line_offset: usize =
        str_utf8_offset_after_n_lsp_linebreaks(str, range.end.line as usize);
    let end_offset: usize = end_line_offset
        + str_starting_utf8_length_for_utf16_length(
            &str[end_line_offset..],
            range.end.character as usize,
        );
    start_offset..end_offset
}
pub fn str_utf8_offset_after_n_lsp_linebreaks(str: &str, linebreak_count_to_skip: usize) -> usize {
    if linebreak_count_to_skip == 0 {
        return 0;
    }
    let mut offset_after_n_linebreaks: usize = 0;
    let mut encountered_linebreaks: usize = 0;
    'finding_after_n_linebreaks_offset: loop {
        if str[offset_after_n_linebreaks..].starts_with("\r\n") {
            encountered_linebreaks += 1;
            offset_after_n_linebreaks += 2;
            if encountered_linebreaks >= linebreak_count_to_skip {
                break 'finding_after_n_linebreaks_offset;
            }
        } else {
            match str[offset_after_n_linebreaks..].chars().next() {
                None => {
                    break 'finding_after_n_linebreaks_offset;
                }
                // see EOL in https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocuments
                Some('\r' | '\n') => {
                    encountered_linebreaks += 1;
                    offset_after_n_linebreaks += 1;
                    if encountered_linebreaks >= linebreak_count_to_skip {
                        break 'finding_after_n_linebreaks_offset;
                    }
                }
                Some(next_char) => {
                    offset_after_n_linebreaks += next_char.len_utf8();
                }
            }
        }
    }
    offset_after_n_linebreaks
}
pub fn str_starting_utf8_length_for_utf16_length(
    slice: &str,
    starting_utf16_length: usize,
) -> usize {
    let mut utf8_length: usize = 0;
    let mut so_far_length_utf16: usize = 0;
    'traversing_utf16_length: for char in slice.chars() {
        if so_far_length_utf16 >= starting_utf16_length {
            break 'traversing_utf16_length;
        }
        utf8_length += char.len_utf8();
        so_far_length_utf16 += char.len_utf16();
    }
    utf8_length
}
fn range_includes_position(range: lsp_types::Range, position: lsp_types::Position) -> bool {
    position >= range.start && position <= range.end
}
fn position_add_characters(
    position: lsp_types::Position,
    additional_characters: u32,
) -> lsp_types::Position {
    lsp_types::Position {
        line: position.line,
        character: position.character + additional_characters,
    }
}
fn position_to_string(lsp_position: lsp_types::Position) -> String {
    format!("{}:{}", lsp_position.line + 1, lsp_position.character + 1)
}
fn index_to_th(index: usize) -> String {
    let n = index + 1;
    let last_digit = n % 10;
    let th = match last_digit {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{n}{th}")
}

mod strongly_connected_components {
    pub type Vertex = usize;
    pub type Neighbors = Vec<std::collections::BTreeSet<Vertex>>;

    pub fn neighbors_add_vertex(graph: &mut Neighbors) -> Vertex {
        let new_node = graph.len();
        graph.push(std::collections::BTreeSet::new());
        new_node
    }

    /// using Tarjan's algorithm
    pub fn find(neighbors: &Neighbors) -> Vec<std::collections::BTreeSet<Vertex>> {
        // won't be reached by regular Vertexes as a Vec can never hold that many items
        const vertex_unvisited: Vertex = usize::MAX;
        struct TarjanState {
            index: Vertex,
            stack: Vec<Vertex>,
            // consider splitting up vertex_state into 3 vecs, one for each property
            vertex_state: Vec<TarjanVertexState>,
            strongly_connected_components: Vec<std::collections::BTreeSet<Vertex>>,
        }
        #[derive(Clone, Copy)]
        struct TarjanVertexState {
            index: Vertex,
            low_link: Vertex,
            // invariant: the same vertex is contained in TarjanState::stack
            is_on_stack: bool,
        }
        let mut state = TarjanState {
            index: 0,
            stack: Vec::new(),
            vertex_state: std::iter::repeat_n(
                TarjanVertexState {
                    index: vertex_unvisited,
                    low_link: vertex_unvisited,
                    is_on_stack: false,
                },
                neighbors.len(),
            )
            .collect(),
            strongly_connected_components: Vec::new(),
        };
        for (vertex, _) in neighbors.iter().enumerate() {
            if state.vertex_state[vertex].index == vertex_unvisited {
                visit(neighbors, &mut state, vertex);
            }
        }
        fn visit(neighbors: &Neighbors, state: &mut TarjanState, vertex: Vertex) {
            // (don't ask me about any of this. I more or less just ported pseudo-code from wikipedia)
            let vertex_mut = &mut state.vertex_state[vertex];
            vertex_mut.index = state.index;
            vertex_mut.low_link = state.index;
            state.index += 1;
            state.stack.push(vertex);
            vertex_mut.is_on_stack = true;

            for &vertex_neighbor in &neighbors[vertex] {
                let vertex_neighbor_state = state.vertex_state[vertex_neighbor];
                if vertex_neighbor_state.index == vertex_unvisited {
                    visit(neighbors, state, vertex_neighbor);
                    let vertex_neighbor_low_link = state.vertex_state[vertex_neighbor].low_link;
                    let vertex_mut = &mut state.vertex_state[vertex];
                    vertex_mut.low_link = vertex_mut.low_link.min(vertex_neighbor_low_link);
                } else if vertex_neighbor_state.is_on_stack {
                    let vertex_neighbor_index = state.vertex_state[vertex_neighbor].index;
                    let vertex_mut = &mut state.vertex_state[vertex];
                    vertex_mut.low_link = vertex_mut.low_link.min(vertex_neighbor_index);
                }
            }
            if {
                let vertex_state = state.vertex_state[vertex];
                vertex_state.low_link == vertex_state.index
            } {
                let mut strongly_connected_component = std::collections::BTreeSet::new();
                loop {
                    let vertex_off_stack = state.stack.pop().unwrap();
                    state.vertex_state[vertex_off_stack].is_on_stack = false;
                    strongly_connected_component.insert(vertex_off_stack);
                    if vertex_off_stack == vertex {
                        break;
                    }
                }
                state
                    .strongly_connected_components
                    .push(strongly_connected_component);
            }
        }
        state.strongly_connected_components
    }
}

#[cfg(test)]
mod core_declarations_are_implemented {
    use super::*;
    #[test]
    fn in_rust() {
        let core_rs = include_str!("core.rs");
        for (core_fn_name, _) in core_fns.iter() {
            let core_fn_name = name_to_lowercase_rust(core_fn_name);
            assert!(
                core_rs.contains(&format!("fn {}", core_fn_name)),
                "core.rs does not contain fn {}",
                core_fn_name
            );
        }
        for (core_fn_name, _) in core_type_aliases.iter() {
            let core_ty_name = name_to_uppercase_rust(core_fn_name);
            assert!(
                core_rs.contains(&format!("type {}", core_ty_name))
                    || core_rs.contains(&format!("struct {}", core_ty_name))
                    || core_rs.contains(&format!("enum {}", core_ty_name)),
                "core.rs does not contain ty {}",
                core_ty_name
            );
        }
    }
    #[test]
    fn in_zig() {
        let core_zig = include_str!("core.zig");
        for (core_fn_name, _) in core_fns.iter() {
            let core_fn_name = name_to_lowercase_zig(core_fn_name);
            assert!(
                core_zig.contains(&format!("pub fn {}", core_fn_name))
                    || core_zig.contains(&format!("pub inline fn {}", core_fn_name)),
                "core.zig does not contain fn {}",
                core_fn_name
            );
        }
        for (core_fn_name, _) in core_type_aliases.iter() {
            // switch to name_to_uppercase_rust when available
            let core_ty_name = name_to_uppercase_zig(core_fn_name);
            assert!(
                core_zig.contains(&format!("fn {}", core_ty_name))
                    || core_zig.contains(&format!("const {}", core_ty_name)),
                "core.zig does not contain ty {}",
                core_ty_name
            );
        }
    }
    #[test]
    fn in_mjs() {
        let core_mjs = include_str!("core.mjs");
        for (core_fn_name, _) in core_fns.iter() {
            let core_fn_name = name_to_lowercase_js(core_fn_name);
            assert!(
                core_mjs.contains(&format!("export function {}", core_fn_name)),
                "core.mjs does not contain fn {}",
                core_fn_name
            );
        }
        for (core_fn_name, _) in core_type_aliases.iter() {
            let core_ty_name = name_to_uppercase_js(core_fn_name);
            // this is quite approximate
            assert!(
                core_mjs.contains(&format!("}} {} */", core_ty_name))
                    || core_mjs.contains(&format!("}} {}\n", core_ty_name)),
                "core.mjs does not contain ty {}",
                core_ty_name
            );
        }
    }
}
