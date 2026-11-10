#![allow(non_upper_case_globals)]

use gen_lsp_types as lsp_types;

pub type Name = byteyarn::Yarn;
#[derive(Clone, Copy, Debug)]
pub struct WithStartPosition<Value> {
    pub value: Value,
    pub start: lsp_types::Position,
}

#[derive(Debug)]
pub struct SyntaxProject {
    pub items: Vec<SyntaxProjectItem>,
    pub expressions: Buf<Expressions, SyntaxExpression>,
}
#[derive(Debug)]
pub enum Expressions {}

#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    reason = "::Fn is the largest but most variants in practice are ::Fn anyway"
)]
pub enum SyntaxProjectItem {
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
        type_parameters: Vec<WithStartPosition<Name>>,
        open_paren_start: Option<lsp_types::Position>,
        parameters: Vec<SyntaxPattern>,
        closed_paren_start: Option<lsp_types::Position>,
        result_type: Option<SyntaxType>,
        documentation: Option<SyntaxComments>,
        result: Option<SyntaxExpression>,
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
pub struct SyntaxTyParameters {
    pub open_paren_start: lsp_types::Position,
    pub parameters: Vec<WithStartPosition<Name>>,
    pub closed_paren_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub enum SyntaxType {
    Variable(WithStartPosition<Name>),
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
    pub values: Option<SyntaxTypeVariantValues>,
}
#[derive(Debug)]
pub struct SyntaxTypeVariantValues {
    pub open_paren_start: lsp_types::Position,
    pub values: Vec<SyntaxType>,
    pub closed_paren_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub struct SyntaxTypeConstructParenthesizedArguments {
    pub open_paren_start: lsp_types::Position,
    pub arguments: Vec<SyntaxType>,
    pub closed_paren_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub enum SyntaxPattern {
    Variable {
        name: WithStartPosition<Name>,
        type_: Option<SyntaxType>,
    },
    Ignored {
        underscore_start: lsp_types::Position,
        type_: Option<SyntaxType>,
    },
    Variant {
        name: WithStartPosition<Name>,
        values: Option<SyntaxPatternVariantValues>,
    },
    Tuple {
        star_start: lsp_types::Position,
        open_paren_start: Option<lsp_types::Position>,
        parts: Vec<SyntaxPattern>,
        closed_paren_start: Option<lsp_types::Position>,
    },
    Record {
        open_curly_start: lsp_types::Position,
        fields: Vec<SyntaxRecordField<SyntaxPattern>>,
        closed_curly_start: Option<lsp_types::Position>,
    },
}
#[derive(Debug)]
pub struct SyntaxPatternVariantValues {
    pub open_paren_start: lsp_types::Position,
    pub values: Vec<SyntaxPattern>,
    pub closed_paren_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub struct SyntaxExpressionCallTypeArguments {
    pub open_angle_start: lsp_types::Position,
    pub types: Vec<SyntaxType>,
    pub closed_angle_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub enum SyntaxExpression {
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
        first_argument_dot: Option<SyntaxExpressionArgumentDot<Slot<Expressions>>>,
        name: WithStartPosition<Name>,
        type_arguments: Option<SyntaxExpressionCallTypeArguments>,
        open_paren_start: Option<lsp_types::Position>,
        arguments_rest: Vec<SyntaxExpression>,
        closed_paren_start: Option<lsp_types::Position>,
    },
    Variant {
        name: WithStartPosition<Name>,
        type_: Option<SyntaxExpressionVariantType>,
        values: Option<SyntaxExpressionVariantValues>,
    },
    Fn {
        fn_keyword_start: lsp_types::Position,
        open_paren_start: Option<lsp_types::Position>,
        parameters: Vec<SyntaxPattern>,
        closed_paren_start: Option<lsp_types::Position>,
        result: Option<Slot<Expressions>>,
    },
    Tuple {
        star_start: lsp_types::Position,
        open_paren_start: Option<lsp_types::Position>,
        parts: Vec<SyntaxExpression>,
        closed_paren_start: Option<lsp_types::Position>,
    },
    Record {
        open_curly_start: lsp_types::Position,
        fields: Vec<SyntaxRecordField<SyntaxExpression>>,
        closed_curly_start: Option<lsp_types::Position>,
    },
    Commented {
        comments: SyntaxComments,
        expression: Option<Slot<Expressions>>,
    },
    Destruct {
        equals_start: lsp_types::Position,
        pattern: Option<SyntaxPattern>,
        queried: Option<Slot<Expressions>>,
        result: Option<Slot<Expressions>>,
    },
    If {
        if_keyword_start: lsp_types::Position,
        queried: Option<Slot<Expressions>>,
        open_curly_start: Option<lsp_types::Position>,
        cases: Vec<SyntaxExpressionIfCase>,
        closed_curly_start: Option<lsp_types::Position>,
    },
}
#[derive(Debug)]
pub struct SyntaxExpressionArgumentDot<Argument> {
    pub argument: Argument,
    pub dot_start: lsp_types::Position,
}
#[derive(Debug)]
pub struct SyntaxExpressionVariantValues {
    pub open_paren_start: lsp_types::Position,
    pub values: Vec<SyntaxExpression>,
    pub closed_paren_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub struct SyntaxExpressionVariantType {
    pub open_angle_start: lsp_types::Position,
    pub type_: Option<SyntaxType>,
    pub closed_angle_start: Option<lsp_types::Position>,
}
#[derive(Debug)]
pub struct SyntaxExpressionIfCase {
    pub pattern: SyntaxPattern,
    pub result: Option<SyntaxExpression>,
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
pub fn project_item_range(
    item: &SyntaxProjectItem,
    expressions: &Buf<Expressions, SyntaxExpression>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: project_item_start(item),
        end: project_item_end(item, expressions),
    }
}
pub fn project_item_start(item: &SyntaxProjectItem) -> lsp_types::Position {
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
pub fn project_item_end(
    item: &SyntaxProjectItem,
    expressions: &Buf<Expressions, SyntaxExpression>,
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
            .map(|result| expression_end(result, expressions))
            .or_else(|| documentation.as_ref().map(|doc| comments_end(doc)))
            .or_else(|| {
                result_type
                    .as_ref()
                    .map(|result_type| type_end(result_type))
            })
            .or(*closed_paren_start)
            .or_else(|| parameters.last().map(|parameter| pattern_end(parameter)))
            .or(*open_paren_start)
            .or_else(|| {
                type_parameters
                    .last()
                    .map(|type_parameter| name_end(with_start_position_as_ref(type_parameter)))
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
pub fn type_range(type_: &SyntaxType) -> lsp_types::Range {
    lsp_types::Range {
        start: type_start(type_),
        end: type_end(type_),
    }
}
pub fn type_start(type_: &SyntaxType) -> lsp_types::Position {
    match type_ {
        SyntaxType::Variable(name) => name.start,
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
                        .values
                        .as_ref()
                        .and_then(|values| values.values.last())
                        .map(|value| type_end(value))
                        .unwrap_or_else(|| name_end(with_start_position_as_ref(&last_variant.name)))
                })
            })
            .unwrap_or_else(|| symbol_end(*open_curly_start, "{")),
    }
}

pub fn pattern_range(pattern: &SyntaxPattern) -> lsp_types::Range {
    lsp_types::Range {
        start: pattern_start(pattern),
        end: pattern_end(pattern),
    }
}
pub fn pattern_start(pattern: &SyntaxPattern) -> lsp_types::Position {
    match pattern {
        SyntaxPattern::Variable { name, type_: _ } => name.start,
        SyntaxPattern::Ignored {
            underscore_start,
            type_: _,
        } => *underscore_start,
        SyntaxPattern::Variant { name, .. } => name.start,
        SyntaxPattern::Tuple { star_start, .. } => *star_start,
        SyntaxPattern::Record {
            open_curly_start, ..
        } => *open_curly_start,
    }
}
pub fn pattern_end(pattern: &SyntaxPattern) -> lsp_types::Position {
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
        SyntaxPattern::Variant { name, values } => values
            .as_ref()
            .and_then(|values| values.values.last())
            .map(|value| pattern_end(value))
            .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
        SyntaxPattern::Tuple {
            star_start,
            open_paren_start,
            parts,
            closed_paren_start,
        } => closed_paren_start
            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
            .or_else(|| parts.last().map(|part| pattern_end(part)))
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
                        .map(|last_value| pattern_end(last_value))
                        .unwrap_or_else(|| name_end(with_start_position_as_ref(&last.name)))
                })
            })
            .unwrap_or_else(|| symbol_end(*open_curly_start, "{")),
    }
}
pub fn expression_call_type_arguments_end(
    SyntaxExpressionCallTypeArguments {
        open_angle_start,
        types,
        closed_angle_start,
    }: &SyntaxExpressionCallTypeArguments,
) -> lsp_types::Position {
    closed_angle_start
        .map(|closed_angle_start| symbol_end(closed_angle_start, ">"))
        .or_else(|| types.last().map(|last_type| type_end(last_type)))
        .unwrap_or_else(|| symbol_end(*open_angle_start, "<"))
}
pub fn expression_range(
    expression: &SyntaxExpression,
    expressions: &Buf<Expressions, SyntaxExpression>,
) -> lsp_types::Range {
    lsp_types::Range {
        start: expression_start(expression, expressions),
        end: expression_end(expression, expressions),
    }
}
pub fn expression_start(
    expression: &SyntaxExpression,
    expressions: &Buf<Expressions, SyntaxExpression>,
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
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            ..
        } => first_argument_dot
            .as_ref()
            .map(
                |SyntaxExpressionArgumentDot {
                     argument,
                     dot_start: _,
                 }| expression_start(expressions.item(argument), expressions),
            )
            .unwrap_or(name.start),
        SyntaxExpression::Variant { name, .. } => name.start,
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
pub fn expression_end(
    expression: &SyntaxExpression,
    expressions: &Buf<Expressions, SyntaxExpression>,
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
                    .map(|argument| expression_end(argument, expressions))
            })
            .or_else(|| open_paren_start.map(|open_paren_start| symbol_end(open_paren_start, "(")))
            .or_else(|| {
                type_arguments
                    .as_ref()
                    .map(|type_arguments| expression_call_type_arguments_end(type_arguments))
            })
            .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
        SyntaxExpression::Variant {
            name,
            type_,
            values,
        } => values
            .as_ref()
            .and_then(|values| values.values.last())
            .map(|value| expression_end(value, expressions))
            .or_else(|| {
                type_.as_ref().map(|type_| {
                    type_
                        .closed_angle_start
                        .map(|closed_angle_start| symbol_end(closed_angle_start, ">"))
                        .or_else(|| type_.type_.as_ref().map(|type_| type_end(type_)))
                        .unwrap_or_else(|| symbol_end(type_.open_angle_start, "<"))
                })
            })
            .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
        SyntaxExpression::Fn {
            fn_keyword_start,
            open_paren_start,
            parameters,
            closed_paren_start,
            result,
        } => result
            .as_ref()
            .map(|result| expression_end(expressions.item(result), expressions))
            .or_else(|| {
                closed_paren_start.map(|closed_bracket_start| symbol_end(closed_bracket_start, ")"))
            })
            .or_else(|| parameters.last().map(|parameter| pattern_end(parameter)))
            .or_else(|| open_paren_start.map(|open_paren_start| symbol_end(open_paren_start, "(")))
            .unwrap_or_else(|| symbol_end(*fn_keyword_start, "fn")),
        SyntaxExpression::Tuple {
            star_start,
            open_paren_start,
            parts,
            closed_paren_start,
        } => closed_paren_start
            .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
            .or_else(|| parts.last().map(|part| expression_end(part, expressions)))
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
                        .map(|value| expression_end(value, expressions))
                        .unwrap_or_else(|| name_end(with_start_position_as_ref(&last_field.name)))
                })
            })
            .unwrap_or_else(|| symbol_end(*open_curly_start, "{")),
        SyntaxExpression::Commented {
            comments,
            expression,
        } => expression
            .as_ref()
            .map(|inner| expression_end(expressions.item(inner), expressions))
            .unwrap_or_else(|| comments_end(comments)),
        SyntaxExpression::Destruct {
            equals_start,
            pattern,
            queried,
            result,
        } => result
            .as_ref()
            .map(|result| expression_end(expressions.item(result), expressions))
            .or_else(|| {
                queried
                    .as_ref()
                    .map(|queried| expression_end(expressions.item(queried), expressions))
            })
            .or_else(|| pattern.as_ref().map(|pattern| pattern_end(pattern)))
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
                        .map(|result| expression_end(result, expressions))
                        .unwrap_or_else(|| pattern_end(&last_case.pattern))
                })
            })
            .or_else(|| open_curly_start.map(|open_curly_start| symbol_end(open_curly_start, "{")))
            .or_else(|| {
                queried
                    .as_ref()
                    .map(|queried| expression_end(expressions.item(queried), expressions))
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
    while parse_linebreak(state)
        || parse_same_line_char_if(state, |c| c.is_whitespace() || c == ',')
    {}
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

pub fn parse_project(project_source: &str) -> SyntaxProject {
    // entirely unscientific lower estimates c:
    let mut items = Vec::with_capacity(project_source.len() / 24);
    let mut expressions = Buf::new();
    expressions.pre_allocate_at_least_usize(project_source.len() / 4);

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
        match parse_project_item(&mut state, &mut expressions) {
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
    SyntaxProject { items, expressions }
}

fn parse_project_item(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxProjectItem> {
    parse_project_fn(state, expressions)
        .or_else(|| parse_project_ty(state))
        .or_else(|| parse_slug_comments(state).map(SyntaxProjectItem::Comments))
}
fn parse_project_ty(state: &mut ParseState) -> Option<SyntaxProjectItem> {
    let Some(ty_keyword_start) = parse_symbol_as_start(state, "ty") else {
        return None;
    };
    parse_slug_whitespace(state);
    let name = parse_slug_lowercase_name_with_start(state);
    parse_slug_whitespace(state);
    let parameters = parse_ty_parameters(state);
    parse_slug_whitespace(state);
    let documentation = parse_slug_comments(state);
    parse_slug_whitespace(state);
    let type_ = parse_type(state);
    Some(SyntaxProjectItem::TypeAlias {
        ty_keyword_start,
        name,
        parameters,
        documentation,
        type_,
    })
}
fn parse_ty_parameters(state: &mut ParseState) -> Option<SyntaxTyParameters> {
    let Some(open_paren_start) = parse_symbol_as_start(state, "(") else {
        return None;
    };
    let mut parameters = Vec::new();
    while let Some(parameter) = parse_slug_uppercase_name_with_start(state) {
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
fn parse_project_fn(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxProjectItem> {
    let Some(fn_keyword_start) = parse_symbol_as_start(state, "fn") else {
        return None;
    };
    parse_slug_whitespace(state);
    let name = parse_slug_lowercase_name_with_start(state);
    parse_slug_whitespace(state);
    let _ = parse_symbol(state, "<");
    parse_slug_whitespace(state);
    let mut type_parameters = Vec::new();
    while let Some(type_parameter_name) = parse_slug_uppercase_name_with_start(state) {
        type_parameters.push(type_parameter_name);
        parse_slug_whitespace(state);
    }
    let _ = parse_symbol(state, ">");
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    parse_slug_whitespace(state);
    let mut parameters = Vec::with_capacity(2);
    while let Some(parameter) = parse_parameter_pattern(state) {
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
    let result = parse_expression(state, expressions);
    Some(SyntaxProjectItem::Fn {
        fn_keyword_start,
        name,
        type_parameters,
        open_paren_start,
        parameters,
        closed_paren_start,
        result_type,
        documentation,
        result,
    })
}
pub struct ProjectFnSignature {
    pub fn_keyword_start: lsp_types::Position,
    pub name: Option<WithStartPosition<Name>>,
    pub type_parameters: Vec<WithStartPosition<Name>>,
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
    let name = parse_slug_lowercase_name_with_start(state);
    parse_slug_whitespace(state);
    let _ = parse_symbol(state, ":");
    parse_slug_whitespace(state);
    let mut type_parameters = Vec::with_capacity(1);
    while let Some(type_parameter_name) = parse_slug_uppercase_name_with_start(state) {
        type_parameters.push(type_parameter_name);
        parse_slug_whitespace(state);
    }
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
pub fn parse_parameter_pattern(state: &mut ParseState) -> Option<SyntaxPattern> {
    parse_parameter_pattern_variable(state)
        .or_else(|| parse_parameter_pattern_ignored(state))
        .or_else(|| parse_parameter_pattern_variant(state))
        .or_else(|| parse_parameter_pattern_tuple(state))
        .or_else(|| parse_parameter_pattern_record(state))
}
pub fn parse_query_pattern(state: &mut ParseState) -> Option<SyntaxPattern> {
    parse_query_pattern_variable(state)
        .or_else(|| parse_query_pattern_ignored(state))
        .or_else(|| parse_query_pattern_variant(state))
        .or_else(|| parse_query_pattern_tuple(state))
        .or_else(|| parse_query_pattern_record(state))
}
fn parse_parameter_pattern_variable(state: &mut ParseState) -> Option<SyntaxPattern> {
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
fn parse_parameter_pattern_ignored(state: &mut ParseState) -> Option<SyntaxPattern> {
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
fn parse_query_pattern_variable(state: &mut ParseState) -> Option<SyntaxPattern> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    Some(SyntaxPattern::Variable {
        name: name,
        type_: None,
    })
}
fn parse_query_pattern_ignored(state: &mut ParseState) -> Option<SyntaxPattern> {
    let Some(underscore_start) = parse_symbol_as_start(state, "_") else {
        return None;
    };
    Some(SyntaxPattern::Ignored {
        underscore_start,
        type_: None,
    })
}
fn parse_parameter_pattern_variant(state: &mut ParseState) -> Option<SyntaxPattern> {
    parse_pattern_variant(state, parse_parameter_pattern)
}
fn parse_query_pattern_variant(state: &mut ParseState) -> Option<SyntaxPattern> {
    parse_pattern_variant(state, parse_query_pattern)
}
fn parse_pattern_variant(
    state: &mut ParseState,
    parse_value: impl Fn(&mut ParseState) -> Option<SyntaxPattern>,
) -> Option<SyntaxPattern> {
    let Some(name) = parse_slug_uppercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let values = parse_pattern_variant_values(state, parse_value);
    Some(SyntaxPattern::Variant { name, values })
}
fn parse_pattern_variant_values(
    state: &mut ParseState,
    parse_value: impl Fn(&mut ParseState) -> Option<SyntaxPattern>,
) -> Option<SyntaxPatternVariantValues> {
    let Some(open_paren_start) = parse_symbol_as_start(state, "(") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut values = Vec::with_capacity(1);
    while let Some(value) = parse_value(state) {
        values.push(value);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxPatternVariantValues {
        open_paren_start,
        values,
        closed_paren_start,
    })
}
fn parse_colon_type_arguments(state: &mut ParseState) -> Option<SyntaxExpressionCallTypeArguments> {
    let Some(open_angle_start) = parse_symbol_as_start(state, "<") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut types = Vec::with_capacity(1);
    parse_slug_whitespace(state);
    while let Some(type_) = parse_type(state) {
        types.push(type_);
        parse_slug_whitespace(state);
    }
    let closed_angle_start = parse_symbol_as_start(state, ">");
    Some(SyntaxExpressionCallTypeArguments {
        open_angle_start,
        types,
        closed_angle_start,
    })
}
fn parse_parameter_pattern_tuple(state: &mut ParseState) -> Option<SyntaxPattern> {
    let Some(star_start) = parse_symbol_as_start(state, "*") else {
        return None;
    };
    parse_slug_whitespace(state);
    match parse_symbol_as_start(state, "(") {
        None => Some(SyntaxPattern::Tuple {
            star_start,
            open_paren_start: None,
            parts: vec![],
            closed_paren_start: None,
        }),
        Some(open_paren_start) => {
            parse_slug_whitespace(state);
            let mut parts = Vec::new();
            while let Some(part) = parse_parameter_pattern(state) {
                parts.push(part);
                parse_slug_whitespace(state);
            }
            let closed_paren_start = parse_symbol_as_start(state, ")");
            Some(SyntaxPattern::Tuple {
                star_start,
                open_paren_start: Some(open_paren_start),
                parts,
                closed_paren_start,
            })
        }
    }
}
fn parse_query_pattern_tuple(state: &mut ParseState) -> Option<SyntaxPattern> {
    let Some(star_start) = parse_symbol_as_start(state, "*") else {
        return None;
    };
    parse_slug_whitespace(state);
    match parse_symbol_as_start(state, "(") {
        None => Some(SyntaxPattern::Tuple {
            star_start,
            open_paren_start: None,
            parts: vec![],
            closed_paren_start: None,
        }),
        Some(open_paren_start) => {
            parse_slug_whitespace(state);
            let mut parts = Vec::new();
            while let Some(part) = parse_query_pattern(state) {
                parts.push(part);
                parse_slug_whitespace(state);
            }
            let closed_paren_start = parse_symbol_as_start(state, ")");
            Some(SyntaxPattern::Tuple {
                star_start,
                open_paren_start: Some(open_paren_start),
                parts,
                closed_paren_start,
            })
        }
    }
}
fn parse_parameter_pattern_record(state: &mut ParseState) -> Option<SyntaxPattern> {
    let Some(open_curly_start) = parse_symbol_as_start(state, "{") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_parameter_pattern_record_field(state) {
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
fn parse_parameter_pattern_record_field(
    state: &mut ParseState,
) -> Option<SyntaxRecordField<SyntaxPattern>> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let value = parse_parameter_pattern(state);
    Some(SyntaxRecordField {
        name: name,
        value: value,
    })
}
fn parse_query_pattern_record(state: &mut ParseState) -> Option<SyntaxPattern> {
    let Some(open_curly_start) = parse_symbol_as_start(state, "{") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_query_pattern_record_field(state) {
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
fn parse_query_pattern_record_field(
    state: &mut ParseState,
) -> Option<SyntaxRecordField<SyntaxPattern>> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let value = parse_query_pattern(state);
    Some(SyntaxRecordField {
        name: name,
        value: value,
    })
}

pub fn parse_type(state: &mut ParseState) -> Option<SyntaxType> {
    parse_type_variable(state)
        // fn must be checked before construct
        .or_else(|| parse_type_fn(state))
        .or_else(|| parse_type_construct(state))
        .or_else(|| parse_type_tuple(state))
        .or_else(|| parse_type_record(state))
        .or_else(|| parse_type_choice(state))
}
fn parse_type_variable(state: &mut ParseState) -> Option<SyntaxType> {
    let Some(name) = parse_slug_uppercase_name_with_start(state) else {
        return None;
    };
    Some(SyntaxType::Variable(name))
}
fn parse_type_tuple(state: &mut ParseState) -> Option<SyntaxType> {
    let Some(star_start) = parse_symbol_as_start(state, "*") else {
        return None;
    };
    parse_slug_whitespace(state);
    match parse_symbol_as_start(state, "(") {
        None => Some(SyntaxType::Tuple {
            star_start,
            open_paren_start: None,
            parts: vec![],
            closed_paren_start: None,
        }),
        Some(open_paren_start) => {
            parse_slug_whitespace(state);
            let mut parts = Vec::new();
            while let Some(part) = parse_type(state) {
                parts.push(part);
                parse_slug_whitespace(state);
            }
            let closed_paren_start = parse_symbol_as_start(state, ")");
            Some(SyntaxType::Tuple {
                star_start,
                open_paren_start: Some(open_paren_start),
                parts,
                closed_paren_start,
            })
        }
    }
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
    let Some(name) = parse_slug_uppercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let values = parse_type_variant_values(state);
    Some(SyntaxTypeVariant { name: name, values })
}
fn parse_type_variant_values(state: &mut ParseState) -> Option<SyntaxTypeVariantValues> {
    let Some(open_paren_start) = parse_symbol_as_start(state, "(") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut values = Vec::with_capacity(1);
    while let Some(value) = parse_type(state) {
        values.push(value);
        parse_slug_whitespace(state);
    }
    parse_slug_whitespace(state);
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxTypeVariantValues {
        open_paren_start,
        values,
        closed_paren_start,
    })
}
fn parse_type_construct(state: &mut ParseState) -> Option<SyntaxType> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
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
    let Some(fn_keyword_start) = parse_slug_keyword_as_start(state, "fn") else {
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
pub fn parse_expression(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpression> {
    let Some(mut start_expression) = parse_expression_number(state)
        .or_else(|| parse_expression_str(state))
        // fn and if must be checked before variable/call
        .or_else(|| parse_expression_fn(state, expressions))
        .or_else(|| parse_expression_if(state, expressions))
        .or_else(|| parse_expression_variable_or_call(state, expressions))
        .or_else(|| parse_expression_variant(state, expressions))
        .or_else(|| parse_expression_commented(state, expressions))
        .or_else(|| parse_expression_tuple(state, expressions))
        .or_else(|| parse_expression_record(state, expressions))
        .or_else(|| parse_expression_destruct(state, expressions))
    else {
        return None;
    };
    while let () = parse_slug_whitespace(state)
        && let Some(dot_start) = parse_symbol_as_start(state, ".")
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
                Some(start_expression_dot),
                name,
            );
        start_expression = start_expression_dot_call;
    }
    Some(start_expression)
}
fn parse_expression_tuple(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpression> {
    let Some(star_start) = parse_symbol_as_start(state, "*") else {
        return None;
    };
    parse_slug_whitespace(state);
    match parse_symbol_as_start(state, "(") {
        None => Some(SyntaxExpression::Tuple {
            star_start,
            open_paren_start: None,
            parts: vec![],
            closed_paren_start: None,
        }),
        Some(open_paren_start) => {
            parse_slug_whitespace(state);
            let mut parts = Vec::new();
            while let Some(part) = parse_expression(state, expressions) {
                parts.push(part);
                parse_slug_whitespace(state);
            }
            let closed_paren_start = parse_symbol_as_start(state, ")");
            Some(SyntaxExpression::Tuple {
                star_start,
                open_paren_start: Some(open_paren_start),
                parts,
                closed_paren_start,
            })
        }
    }
}
fn parse_expression_record(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpression> {
    let Some(open_curly_start) = parse_symbol_as_start(state, "{") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut fields = Vec::new();
    while let Some(field) = parse_expression_record_field(state, expressions) {
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
fn parse_expression_record_field(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxRecordField<SyntaxExpression>> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let value = parse_expression(state, expressions);
    Some(SyntaxRecordField {
        name: name,
        value: value,
    })
}
fn parse_expression_number(state: &mut ParseState) -> Option<SyntaxExpression> {
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
fn parse_expression_str(state: &mut ParseState) -> Option<SyntaxExpression> {
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
fn parse_expression_commented(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpression> {
    let Some(comments) = parse_slug_comments(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let expression = parse_expression(state, expressions);
    Some(SyntaxExpression::Commented {
        comments: comments,
        expression: expression.map(|expression| expressions.insert(expression)),
    })
}
fn parse_expression_variable_or_call(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpression> {
    let Some(name) = parse_slug_lowercase_name_with_start(state) else {
        return None;
    };
    Some(
        parse_expression_variable_or_call_after_first_argument_dot_name(
            state,
            expressions,
            None,
            name,
        ),
    )
}
fn parse_expression_variable_or_call_after_first_argument_dot_name(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,

    first_argument_dot: Option<SyntaxExpressionArgumentDot<Slot<Expressions>>>,
    name: WithStartPosition<Name>,
) -> SyntaxExpression {
    parse_slug_whitespace(state);
    let type_arguments = parse_colon_type_arguments(state);
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    if type_arguments.is_none() && open_paren_start.is_none() {
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
    while let Some(argument) = parse_expression(state, expressions) {
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

fn parse_expression_variant(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpression> {
    let Some(name) = parse_slug_uppercase_name_with_start(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    // this will parse things like Variant * () as a variant without any parenthesized arguments.
    // This is fine since either interpretation is invalid
    let type_ = parse_expression_variant_type_argument(state);
    parse_slug_whitespace(state);
    let values = parse_expression_variant_values(state, expressions);
    Some(SyntaxExpression::Variant {
        name,
        type_,
        values,
    })
}
fn parse_expression_variant_type_argument(
    state: &mut ParseState,
) -> Option<SyntaxExpressionVariantType> {
    let Some(open_angle_start) = parse_symbol_as_start(state, "<") else {
        return None;
    };
    parse_slug_whitespace(state);
    let type_ = parse_type(state);
    parse_slug_whitespace(state);
    let closed_angle_start = parse_symbol_as_start(state, ">");
    Some(SyntaxExpressionVariantType {
        open_angle_start,
        type_,
        closed_angle_start,
    })
}
fn parse_expression_variant_values(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpressionVariantValues> {
    let Some(open_paren_start) = parse_symbol_as_start(state, "(") else {
        return None;
    };
    parse_slug_whitespace(state);
    let mut values = Vec::with_capacity(1);
    while let Some(value) = parse_expression(state, expressions) {
        values.push(value);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    Some(SyntaxExpressionVariantValues {
        open_paren_start,
        values,
        closed_paren_start,
    })
}
fn parse_expression_fn(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpression> {
    let Some(fn_keyword_start) = parse_slug_keyword_as_start(state, "fn") else {
        return None;
    };
    parse_slug_whitespace(state);
    let open_paren_start = parse_symbol_as_start(state, "(");
    parse_slug_whitespace(state);
    let mut parameters = Vec::with_capacity(2);
    while let Some(parameter) = parse_parameter_pattern(state) {
        parameters.push(parameter);
        parse_slug_whitespace(state);
    }
    let closed_paren_start = parse_symbol_as_start(state, ")");
    parse_slug_whitespace(state);
    let result = parse_expression(state, expressions);
    Some(SyntaxExpression::Fn {
        fn_keyword_start,
        open_paren_start,
        parameters,
        closed_paren_start,
        result: result.map(|result| expressions.insert(result)),
    })
}
fn parse_expression_destruct(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpression> {
    let Some(equals_start) = parse_symbol_as_start(state, "=") else {
        return None;
    };
    parse_slug_whitespace(state);
    let pattern = parse_query_pattern(state);
    parse_slug_whitespace(state);
    let queried = parse_expression(state, expressions);
    parse_slug_whitespace(state);
    let result = parse_expression(state, expressions);
    Some(SyntaxExpression::Destruct {
        equals_start,
        pattern,
        queried: queried.map(|queried| expressions.insert(queried)),
        result: result.map(|result| expressions.insert(result)),
    })
}
fn parse_expression_if(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpression> {
    let Some(if_keyword_start) = parse_slug_keyword_as_start(state, "if") else {
        return None;
    };
    parse_slug_whitespace(state);
    let queried = parse_expression(state, expressions);
    parse_slug_whitespace(state);
    let open_curly_start = parse_symbol_as_start(state, "{");
    parse_slug_whitespace(state);
    let mut cases = Vec::new();
    while let Some(case) = parse_expression_if_case(state, expressions) {
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
fn parse_expression_if_case(
    state: &mut ParseState,
    expressions: &mut Buf<Expressions, SyntaxExpression>,
) -> Option<SyntaxExpressionIfCase> {
    let Some(pattern) = parse_query_pattern(state) else {
        return None;
    };
    parse_slug_whitespace(state);
    let result = parse_expression(state, expressions);
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Variable(Name),
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
    Alias {
        name: Name,
        arguments: Vec<Type>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeField {
    pub name: Name,
    pub value: Type,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeVariant {
    pub name: Name,
    pub values: Vec<Type>,
}
#[derive(Clone, Debug)]
pub struct CheckedProjectFn {
    pub documentation: Option<Box<str>>,
    pub type_parameters: Vec<Name>,
    pub parameter_types: Vec<Option<Type>>,
    pub result_type: Option<Type>,
    pub result_expression_is_invalid: bool,
}
pub fn syntax_project_check<'a>(
    errors: &mut Vec<ErrorNode>,
    syntax_project: &'a SyntaxProject,
) -> CheckedSyntaxProject<'a> {
    let expressions = &syntax_project.expressions;

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
        SyntaxProjectFnInfo,
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
                                documentation,
                                name: name_node,
                                parameters,
                                type_,
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
                                            expression_end(result, expressions)
                                        })
                                        .or_else(|| documentation.as_ref().map(comments_end))
                                        .or_else(|| {
                                            result_type
                                                .as_ref()
                                                .map(|result_type| type_end(result_type))
                                        })
                                        .or_else(|| {
                                            parameters.last().map(|parameter| {
                                                pattern_end(parameter)
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
            let mut parameter_variables = Vec::with_capacity(2);
            for parameter in project_fn_info.parameters {
                syntax_parameter_pattern_variables_for_each(parameter, &mut |name, _type| {
                    parameter_variables.push(name.value);
                });
            }
            let mut project_fn_referenced_vertexes = std::collections::BTreeSet::new();
            syntax_expression_connect_fns_in_graph_from(
                &project_fn_graph_node_by_name,
                expressions,
                result_node,
                &mut parameter_variables,
                &mut project_fn_referenced_vertexes,
            );
            project_fn_graph[project_fn_graph_node] = project_fn_referenced_vertexes;
        }
    }

    // maybe instead start from 0 instead of cloning every time?
    let mut checked_type_aliases =
        std::collections::HashMap::<Name, CheckedTypeAlias>::with_capacity(
            project_type_by_graph_node.len(),
        );
    for project_type_strongly_connected_component in
        strongly_connected_components::find(&type_graph)
    {
        // possible optimization: skip this computation when project_type_strongly_connected_component is single, non-self-referencing node
        let project_type_strongly_connected_component_parameters =
            project_type_strongly_connected_component
                .iter()
                .filter_map(|variable_declaration_graph_node| {
                    project_type_by_graph_node.get(variable_declaration_graph_node)
                })
                .copied()
                .map(|project_type| {
                    (
                        &project_type.name.value,
                        project_type
                            .parameters
                            .iter()
                            .flat_map(|parameters| {
                                parameters
                                    .parameters
                                    .iter()
                                    .map(|parameter| parameter.value.clone())
                            })
                            .collect(),
                    )
                })
                .collect::<std::collections::HashMap<_, _>>();
        for project_type in project_type_strongly_connected_component
            .iter()
            .filter_map(|variable_declaration_graph_node| {
                project_type_by_graph_node.get(variable_declaration_graph_node)
            })
            .copied()
        {
            checked_type_aliases.insert(
                project_type.name.value.clone(),
                project_type_alias_check(
                    project_type,
                    errors,
                    &project_type_strongly_connected_component_parameters,
                    &checked_type_aliases,
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
    for project_fn_strongly_connected_component in
        strongly_connected_components::find(&project_fn_graph)
    {
        let project_fns_in_strongly_connected_component: Vec<SyntaxProjectFnInfo> =
            project_fn_strongly_connected_component
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
                    project_fn,
                    &mut records_used,
                    &mut choices_used,
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
                    project_fn,
                    &mut checked_calls,
                    &mut checked_local_fns,
                    &mut checked_queries,
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
    }
}
pub struct CheckedSyntaxProject<'a> {
    pub type_graph: strongly_connected_components::Neighbors,
    pub project_type_by_graph_node:
        std::collections::HashMap<strongly_connected_components::Vertex, SyntaxProjectTypeInfo<'a>>,
    pub project_fn_graph: strongly_connected_components::Neighbors,
    pub project_fn_graph_node_by_name:
        std::collections::HashMap<&'a Name, strongly_connected_components::Vertex>,
    pub project_fn_by_graph_node:
        std::collections::HashMap<strongly_connected_components::Vertex, SyntaxProjectFnInfo<'a>>,
    pub records_used: std::collections::HashSet<Vec<Name>>,
    pub choices_used: std::collections::HashSet<Vec<Name>>,
    pub checked_type_aliases: std::collections::HashMap<Name, CheckedTypeAlias>,
    pub checked_project_fns: std::collections::HashMap<Name, CheckedProjectFn>,
    pub checked_calls: std::collections::HashMap<lsp_types::Position, CheckedCall>,
    pub checked_local_fns: std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    pub checked_queries: std::collections::HashMap<lsp_types::Position, CheckedQuery>,
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
                if let Some(SyntaxTypeVariantValues {
                    open_paren_start: _,
                    values,
                    closed_paren_start: _,
                }) = &variant.values
                {
                    for value in values {
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
}
fn syntax_expression_connect_fns_in_graph_from<'a>(
    project_fn_graph_node_by_name: &std::collections::HashMap<
        &Name,
        strongly_connected_components::Vertex,
    >,
    expressions: &'a Buf<Expressions, SyntaxExpression>,
    expression: &'a SyntaxExpression,
    // after the call the Vec contents will be the same as before
    local_variables: &mut Vec<&'a Name>,
    project_fn_graph: &mut std::collections::BTreeSet<strongly_connected_components::Vertex>,
) {
    match expression {
        SyntaxExpression::Number { .. } => {}
        SyntaxExpression::Text { .. } => {}
        SyntaxExpression::Variable(_) => {}
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments: _,
            open_paren_start: _,
            arguments_rest: arguments,
            closed_paren_start: _,
        } => {
            if !local_variables.contains(&&name.value)
                && let Some(referenced_fn_graph_node) =
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
                    local_variables,
                    project_fn_graph,
                );
            }
            for argument in arguments {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    argument,
                    local_variables,
                    project_fn_graph,
                );
            }
        }
        SyntaxExpression::Variant {
            name: _,
            type_: _,
            values,
        } => {
            for value in values.iter().flat_map(|values| &values.values) {
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    value,
                    local_variables,
                    project_fn_graph,
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
            if let Some(result) = result {
                let mut parameter_introduced_variables = Vec::with_capacity(2);
                for parameter in parameters {
                    syntax_parameter_pattern_variables_for_each(parameter, &mut |name, _type| {
                        parameter_introduced_variables.push(name.value);
                    });
                }
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.item(result),
                    &mut parameter_introduced_variables,
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
                    local_variables,
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
                        local_variables,
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
                    local_variables,
                    project_fn_graph,
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
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.item(queried),
                    local_variables,
                    project_fn_graph,
                );
            }
            if let Some(result) = result {
                let local_variables_count_without_destruct_pattern = local_variables.len();
                if let Some(pattern) = pattern {
                    syntax_parameter_pattern_variables_for_each(pattern, &mut |name, _type| {
                        local_variables.push(name.value);
                    });
                }
                syntax_expression_connect_fns_in_graph_from(
                    project_fn_graph_node_by_name,
                    expressions,
                    expressions.item(result),
                    local_variables,
                    project_fn_graph,
                );
                local_variables.truncate(local_variables_count_without_destruct_pattern);
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
                    local_variables,
                    project_fn_graph,
                );
            }
            for case in cases {
                if let Some(result) = &case.result {
                    let local_variables_count_without_case_pattern = local_variables.len();
                    syntax_parameter_pattern_variables_for_each(
                        &case.pattern,
                        &mut |name, _type| {
                            local_variables.push(name.value);
                        },
                    );
                    syntax_expression_connect_fns_in_graph_from(
                        project_fn_graph_node_by_name,
                        expressions,
                        result,
                        local_variables,
                        project_fn_graph,
                    );
                    local_variables.truncate(local_variables_count_without_case_pattern);
                }
            }
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct SyntaxProjectFnInfo<'a> {
    pub range: lsp_types::Range,
    pub name: &'a WithStartPosition<Name>,
    pub type_parameters: &'a Vec<WithStartPosition<Name>>,
    pub parameters: &'a Vec<SyntaxPattern>,
    pub result_type: &'a Option<SyntaxType>,
    pub documentation: &'a Option<SyntaxComments>,
    pub result: &'a Option<SyntaxExpression>,
}
#[derive(Debug, Copy, Clone)]
pub struct SyntaxProjectTypeInfo<'a> {
    // consider introducing separate structs instead of separately referencing each field
    pub name: &'a WithStartPosition<Name>,
    pub documentation: &'a Option<SyntaxComments>,
    pub parameters: &'a Option<SyntaxTyParameters>,
    pub type_: &'a Option<SyntaxType>,
}

fn project_type_alias_check(
    project_type: SyntaxProjectTypeInfo,
    errors: &mut Vec<ErrorNode>,
    recursive_type_alias_parameters: &std::collections::HashMap<&Name, Vec<Name>>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
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
                documentation,
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
                recursive_type_alias_parameters,
                false,
                type_aliases,
                records_used,
                choices_used,
            ) {
                None => CheckedTypeAlias {
                    name_range: Some(name_range(with_start_position_as_ref(project_type.name))),
                    documentation,
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
                    let parameters = parameters_check_if_different_to_actual_type_parameters(
                        errors,
                        name_range(with_start_position_as_ref(project_type.name)),
                        project_type.parameters.iter().flat_map(|parameters| {
                            parameters.parameters.iter().map(with_start_position_as_ref)
                        }),
                        actually_used_type_variables,
                        "Type parameter variables follow after the name and are separated by commas, so for example ty buf(Origin _item)",
                    );
                    CheckedTypeAlias {
                        name_range: Some(name_range(with_start_position_as_ref(project_type.name))),
                        documentation,
                        parameters,
                        type_: Some(aliased_type),
                    }
                }
            }
        }
    }
}

fn syntax_project_fn_header_check<'a>(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fn: SyntaxProjectFnInfo<'a>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
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
                &std::collections::HashMap::new(),
                records_used,
                choices_used,
            )
        })
        .collect::<Vec<_>>();
    let result_type: Option<Type> = {
        match &project_fn.result_type {
            Some(syntax_result_type) => syntax_type_check_not_in_type_alias(
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
                        "missing function result type. An example of a full function is fn my-function() str  \":)\"str, where str is the result type",
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
            for parameter_type in parameter_types.iter().flatten() {
                type_variables_into(&mut parameter_type_variables, parameter_type);
            }
            type_variables_exclusively_used_in_result
                .retain(|var| !parameter_type_variables.contains(var));
            let actually_used_parameters = parameters_check_if_different_to_actual_type_parameters(
                errors,
                name_range(with_start_position_as_ref(project_fn.name)),
                project_fn
                    .type_parameters
                    .iter()
                    .map(|parameter| with_start_position_as_ref(parameter)),
                type_variables_exclusively_used_in_result,
                "Type parameter variables follow after the name and are enclosed in angle braces, so for example fn no-twice<A B>",
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
                .map(|name| name.value.clone())
                .collect(),
            parameter_types,
            result_type: result_type,
            result_expression_is_invalid: false,
        },
    }
}
fn syntax_project_fn_check<'a>(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    expressions: &Buf<Expressions, SyntaxExpression>,
    project_fn: SyntaxProjectFnInfo<'a>,
    checked_calls: &mut std::collections::HashMap<lsp_types::Position, CheckedCall>,
    checked_local_fns: &mut std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &mut std::collections::HashMap<lsp_types::Position, CheckedQuery>,
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
                project_fn,
                records_used,
                choices_used,
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
    let mut parameter_introduced_variables = std::collections::HashMap::with_capacity(2);
    for syntax_parameter in project_fn.parameters {
        syntax_parameter_pattern_variables_for_each(syntax_parameter, &mut |name, type_| {
            parameter_introduced_variables.insert(
                name.value,
                CheckedLocalVariable {
                    origin_start: name.start,
                    type_: type_.map(|type_| syntax_type_to_type(type_)),
                    used: false,
                },
            );
        });
    }
    let Some(checked_result_expression_type) = syntax_expression_check(
        errors,
        type_aliases,
        project_fns,
        expressions,
        &mut parameter_introduced_variables,
        syntax_result,
        checked_calls,
        checked_local_fns,
        checked_queries,
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
            .iter()
            .filter(|(_, var)| !var.used)
    {
        errors.push(error_introduced_pattern_variable_is_unused(
            parameter_introduced_variable_origin.origin_start,
            parameter_introduced_variable_name,
        ));
    }
    if let Some(result_type_diff) = type_diff(
        &header_result_type,
        &checked_result_expression_type,
        type_aliases,
    ) {
        place_expression_type_diff_errors(errors, syntax_result, &result_type_diff, expressions);
        return CheckedProjectFn {
            documentation: documentation,
            type_parameters: checked_header.type_parameters,
            parameter_types: checked_header.parameter_types,
            result_type: Some(header_result_type),
            result_expression_is_invalid: true,
        };
    }
    let mut reachable_bufs = function_signature_implicit_bufs(
        checked_header.parameter_types.iter().flatten(),
        Some(&checked_result_expression_type),
        type_aliases,
    );
    for parameter in project_fn.parameters {
        parameter_pattern_add_introduced_bufs_and_check_do_not_overlap(
            errors,
            parameter,
            &mut reachable_bufs,
            type_aliases,
        );
    }
    syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
        errors,
        syntax_result,
        &mut reachable_bufs,
        checked_calls,
        checked_queries,
        checked_local_fns,
        expressions,
        project_fns,
        type_aliases,
    );
    CheckedProjectFn {
        documentation: documentation,
        type_parameters: checked_header.type_parameters,
        parameter_types: checked_header.parameter_types,
        result_type: Some(header_result_type),
        result_expression_is_invalid: false,
    }
}
/// only use if you know `syntax_type_check` has already been called on it before
pub fn syntax_type_to_type(type_: &SyntaxType) -> Type {
    match type_ {
        SyntaxType::Variable(name) => Type::Variable(name.value.clone()),
        SyntaxType::Fn {
            fn_keyword_start: _,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => Type::Fn {
            parameters: parameters
                .iter()
                .map(|parameter| syntax_type_to_type(parameter))
                .collect(),
            result: Box::new(
                result
                    .as_ref()
                    .map(|result| syntax_type_to_type(result))
                    .unwrap_or_else(|| type_empty),
            ),
        },
        SyntaxType::ConstructWithArguments { name, arguments } => {
            let argument_types = arguments
                .iter()
                .flat_map(|arguments| &arguments.arguments)
                .map(|argument_type| syntax_type_to_type(argument_type))
                .collect();
            if let Some(CheckedTypeAlias { type_: None, .. }) = core_type_aliases.get(&name.value) {
                Type::CoreConstruct {
                    name: name.value.clone(),
                    arguments: argument_types,
                }
            } else {
                Type::Alias {
                    name: name.value.clone(),
                    arguments: argument_types,
                }
            }
        }
        SyntaxType::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => Type::Tuple(parts.iter().map(|part| syntax_type_to_type(part)).collect()),
        SyntaxType::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => Type::Record(
            fields
                .iter()
                .map(|field| TypeField {
                    name: field.name.value.clone(),
                    value: field
                        .value
                        .as_ref()
                        .map(|field_value| syntax_type_to_type(field_value))
                        .unwrap_or_else(|| type_empty),
                })
                .collect(),
        ),
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => Type::Choice(
            variants
                .iter()
                .map(|syntax_variant| TypeVariant {
                    name: syntax_variant.name.value.clone(),
                    values: syntax_variant
                        .values
                        .iter()
                        .flat_map(|values| &values.values)
                        .map(|syntax_variant_value| syntax_type_to_type(syntax_variant_value))
                        .collect(),
                })
                .collect(),
        ),
    }
}
pub fn syntax_type_check_not_in_type_alias(
    type_: &SyntaxType,
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> Option<Type> {
    syntax_type_check(
        type_,
        errors,
        &std::collections::HashMap::new(),
        false,
        type_aliases,
        records_used,
        choices_used,
    )
}
pub fn syntax_type_check(
    type_: &SyntaxType,
    errors: &mut Vec<ErrorNode>,
    recursive_type_alias_parameters: &std::collections::HashMap<&Name, Vec<Name>>,
    using_recursive_type_alias_allowed: bool,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    records_used: &mut std::collections::HashSet<Vec<Name>>,
    choices_used: &mut std::collections::HashSet<Vec<Name>>,
) -> Option<Type> {
    match type_ {
        SyntaxType::Variable(name) => Some(Type::Variable(name.value.clone())),
        SyntaxType::Fn {
            fn_keyword_start,
            open_paren_start,
            parameters: syntax_parameters,
            closed_paren_start,
            result,
        } => {
            let Some(parameters) = syntax_parameters
                .iter()
                .map(|parameter| {
                    syntax_type_check(
                        parameter,
                        errors,
                        recursive_type_alias_parameters,
                        using_recursive_type_alias_allowed,
                        type_aliases,
                        records_used,
                        choices_used,
                    )
                })
                .collect::<Option<Vec<_>>>()
            else {
                return None;
            };
            let Some(result) = result else {
                errors.push(ErrorNode {
                    message: Box::from("missing result type after this fn(..parameters..) here"),
                    range: closed_paren_start
                        .map(|closed_paren_start| symbol_range(closed_paren_start, ")"))
                        .or_else(|| {
                            syntax_parameters
                                .last()
                                .map(|parameter| type_range(parameter))
                        })
                        .or_else(|| {
                            open_paren_start
                                .map(|open_paren_start| symbol_range(open_paren_start, "("))
                        })
                        .unwrap_or_else(|| symbol_range(*fn_keyword_start, "fn")),
                });
                return None;
            };
            let Some(result) = syntax_type_check(
                result,
                errors,
                recursive_type_alias_parameters,
                using_recursive_type_alias_allowed,
                type_aliases,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            Some(Type::Fn {
                parameters,
                result: Box::new(result),
            })
        }
        SyntaxType::ConstructWithArguments { name, arguments } => {
            let using_recursive_type_alias_in_arguments_allowed = using_recursive_type_alias_allowed
                || match name.value.as_str() {
                    type_span_name | type_slot_name | type_buf_name => true,
                    _ => false,
                };
            let Some(argument_types) = arguments
                .iter()
                .flat_map(|arguments| &arguments.arguments)
                .map(|argument_type| {
                    syntax_type_check(
                        argument_type,
                        errors,
                        recursive_type_alias_parameters,
                        using_recursive_type_alias_in_arguments_allowed,
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
            let (origin_type_parameters, checked_type) = if let Some(origin_core_type) =
                core_type_aliases.get(&name.value)
            {
                (
                    &origin_core_type.parameters,
                    Type::CoreConstruct {
                        name: name.value.clone(),
                        arguments: argument_types,
                    },
                )
            } else if let Some(recursive_origin_type_alias_parameters) =
                recursive_type_alias_parameters.get(&name.value)
            {
                if using_recursive_type_alias_allowed {
                    (
                        recursive_origin_type_alias_parameters,
                        Type::Alias {
                            name: name.value.clone(),
                            arguments: argument_types,
                        },
                    )
                } else {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: Box::from("recursive type alias. A value of such a directly recursive type has no known maximum size on the stack.
Move nested parts of this data into the heap (using slot/span) to represent such nested data."),
                    });
                    return None;
                }
            } else if let Some(origin_type_alias) = type_aliases.get(&name.value) {
                (
                    &origin_type_alias.parameters,
                    Type::Alias {
                        name: name.value.clone(),
                        arguments: argument_types,
                    },
                )
            } else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from("no type alias or core type with this name exists"),
                });
                return None;
            };
            match origin_type_parameters.len().cmp(&argument_count) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Less => {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: format!(
                            "this type has {} less parameters than arguments are provided here.",
                            argument_count - origin_type_parameters.len(),
                        )
                        .into_boxed_str(),
                    });
                    return None;
                }
                std::cmp::Ordering::Greater => {
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: format!(
                            "this core type has {} more parameters than arguments are provided here. The additional {} called {}",
                            origin_type_parameters.len() - argument_count,
                            if origin_type_parameters.len() - argument_count == 1 {
                                "parameter is"
                            } else {
                                "parameters are"
                            },
                            origin_type_parameters.iter().map(|parameter| parameter.as_str()).skip(argument_count).collect::<Vec<_>>().join(", ")
                        ).into_boxed_str()
                    });
                    return None;
                }
            }
            Some(checked_type)
        }
        SyntaxType::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => parts
            .iter()
            .map(|part| {
                syntax_type_check(
                    part,
                    errors,
                    recursive_type_alias_parameters,
                    using_recursive_type_alias_allowed,
                    type_aliases,
                    records_used,
                    choices_used,
                )
            })
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
                    recursive_type_alias_parameters,
                    using_recursive_type_alias_allowed,
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
                match syntax_variant
                    .values
                    .iter()
                    .flat_map(|values| &values.values)
                    .map(|syntax_variant_value| {
                        syntax_type_check(
                            syntax_variant_value,
                            errors,
                            recursive_type_alias_parameters,
                            using_recursive_type_alias_allowed,
                            type_aliases,
                            records_used,
                            choices_used,
                        )
                    })
                    .collect::<Option<Vec<_>>>()
                {
                    None => {
                        any_variant_value_has_error = true;
                    }
                    Some(variant_value_types) => {
                        variant_types.push(TypeVariant {
                            name: syntax_variant.name.value.clone(),
                            values: variant_value_types,
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
fn type_alias_resolve_to_non_alias<'a>(
    type_: &'a Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) -> std::borrow::Cow<'a, Type> {
    type_alias_resolve_to_non_alias_or_error(type_, type_aliases)
        .unwrap_or_else(|| std::borrow::Cow::Borrowed(type_))
}
fn type_alias_resolve_to_non_alias_or_error<'a>(
    type_: &'a Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) -> Option<std::borrow::Cow<'a, Type>> {
    match type_ {
        Type::Alias {
            name: actual_name,
            arguments: actual_arguments,
        } => {
            if let Some(actual_origin_type_alias) = type_aliases.get(actual_name)
                && let Some(actual_resolved_alias) = type_construct_resolve_type_alias(
                    actual_origin_type_alias,
                    actual_arguments.iter().cloned(),
                )
            {
                type_alias_resolve_to_non_alias_or_error(&actual_resolved_alias, type_aliases)
                    .map(|resolved| std::borrow::Cow::Owned(resolved.into_owned()))
            } else {
                None
            }
        }
        _ => Some(std::borrow::Cow::Borrowed(type_)),
    }
}
fn type_fully_resolve_aliases(
    type_: &mut Type,
    // will have the same items after the call as they were before
    type_aliases_to_keep_unresolved: &mut Vec<Name>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match type_ {
        Type::Variable(_) => {}
        Type::Tuple(parts) => {
            for part in parts {
                type_fully_resolve_aliases(part, type_aliases_to_keep_unresolved, type_aliases);
            }
        }
        Type::Record(fields) => {
            for field in fields {
                type_fully_resolve_aliases(
                    &mut field.value,
                    type_aliases_to_keep_unresolved,
                    type_aliases,
                );
            }
        }
        Type::Choice(variants) => {
            for variant in variants {
                for variant_value in &mut variant.values {
                    type_fully_resolve_aliases(
                        variant_value,
                        type_aliases_to_keep_unresolved,
                        type_aliases,
                    );
                }
            }
        }
        Type::CoreConstruct { name, arguments } => {
            if (name == type_span_name || name == type_slot_name)
                && let [origin, _item] = arguments.as_mut_slice()
            {
                type_fully_resolve_aliases(origin, type_aliases_to_keep_unresolved, type_aliases);
            } else {
                for argument in arguments {
                    type_fully_resolve_aliases(
                        argument,
                        type_aliases_to_keep_unresolved,
                        type_aliases,
                    );
                }
            }
        }
        Type::Fn { parameters, result } => {
            for parameter in parameters {
                type_fully_resolve_aliases(
                    parameter,
                    type_aliases_to_keep_unresolved,
                    type_aliases,
                );
            }
            type_fully_resolve_aliases(result, type_aliases_to_keep_unresolved, type_aliases);
        }
        Type::Alias { name, arguments } => {
            if !type_aliases_to_keep_unresolved.contains(name)
                && let Some(actual_origin_type_alias) = type_aliases.get(name)
                && let Some(mut actual_resolved_alias) = type_construct_resolve_type_alias(
                    actual_origin_type_alias,
                    arguments.iter().cloned(),
                )
            {
                type_aliases_to_keep_unresolved.push(name.clone());
                type_fully_resolve_aliases(
                    &mut actual_resolved_alias,
                    type_aliases_to_keep_unresolved,
                    type_aliases,
                );
                *type_ = actual_resolved_alias;
                type_aliases_to_keep_unresolved.pop();
            }
        }
    }
}
fn type_construct_resolve_type_alias(
    origin_type_alias: &CheckedTypeAlias,
    argument_types: impl Iterator<Item = Type>,
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
                *type_ = (*replacement_type_node).clone();
            }
        }
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
                for variant_value in &mut variant.values {
                    type_replace_variables(type_parameter_replacements, variant_value);
                }
            }
        }
        Type::Alias { name: _, arguments } => {
            for argument_type in arguments {
                type_replace_variables(type_parameter_replacements, argument_type);
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
                for variant_value in &variant.values {
                    type_variables_into(type_variables, variant_value);
                }
            }
        }
        Type::Fn { parameters, result } => {
            for parameter in parameters {
                type_variables_into(type_variables, parameter);
            }
            type_variables_into(type_variables, result);
        }
        Type::Alias { name: _, arguments } => {
            for argument in arguments {
                type_variables_into(type_variables, argument);
            }
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
enum SpecificPatternCatch {
    OnlyMatchableByVariable,
    Variant {
        name: Name,
        values: Vec<SpecificPatternCatch>,
    },
    Tuple(Vec<SpecificPatternCatch>),
    Record(Vec<(Name, SpecificPatternCatch)>),
}
/// careful. This easily goes exponential
fn type_to_possible_specific_pattern_catches(
    type_: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    possibilities: &mut Vec<SpecificPatternCatch>,
) {
    // possible optimizations:
    // - calculate the count of possibilities in advance and pre-allocate
    // - add SpecificPatternCatch::Unexpanded(&'a Type) which expands only on encounter.
    //   The more future-proof solution
    match type_ {
        Type::Variable(_) | Type::CoreConstruct { .. } | Type::Fn { .. } => {
            possibilities.push(SpecificPatternCatch::OnlyMatchableByVariable);
        }
        Type::Alias { name, arguments } => {
            if let Some(origin_type_alias) = type_aliases.get(name)
                && let Some(resolved_alias) =
                    type_construct_resolve_type_alias(origin_type_alias, arguments.iter().cloned())
            {
                type_to_possible_specific_pattern_catches(
                    &resolved_alias,
                    type_aliases,
                    possibilities,
                );
            }
        }
        Type::Tuple(type_parts) => {
            if type_parts.is_empty() {
                possibilities.push(SpecificPatternCatch::Tuple(vec![]));
            } else {
                // can probably be optimized
                let mut parts_possibilities = vec![Vec::new()];
                for type_part in type_parts {
                    let mut part_possibilities = Vec::new();
                    type_to_possible_specific_pattern_catches(
                        type_part,
                        type_aliases,
                        &mut part_possibilities,
                    );
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
                        type_aliases,
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
                                    fields_possibility
                                        .push((type_field.name.clone(), value_possibility));
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
                // can probably be optimized
                let mut values_possibilities = vec![Vec::new()];
                for type_value in &type_variant.values {
                    let mut value_possibilities = Vec::new();
                    type_to_possible_specific_pattern_catches(
                        type_value,
                        type_aliases,
                        &mut value_possibilities,
                    );
                    values_possibilities = values_possibilities
                        .into_iter()
                        .flat_map(|values_possibility| {
                            value_possibilities
                                .iter()
                                .cloned()
                                .zip(std::iter::repeat_n(
                                    values_possibility,
                                    value_possibilities.len(),
                                ))
                                .map(|(value_possibility, mut values_possibility)| {
                                    values_possibility.push(value_possibility);
                                    values_possibility
                                })
                        })
                        .collect();
                }
                possibilities.extend(values_possibilities.into_iter().map(|values_possibility| {
                    SpecificPatternCatch::Variant {
                        name: type_variant.name.clone(),
                        values: values_possibility,
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
            output.push('_');
        }
        SpecificPatternCatch::Variant { name, values } => {
            output.push_str(name);
            if let Some((value0, value1_up)) = values.split_first() {
                output.push('(');
                specific_pattern_catch_format(value0, output);
                for value in value1_up {
                    output.push(' ');
                    specific_pattern_catch_format(value, output);
                }
                output.push(')');
            }
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
fn specific_pattern_catches_remove_those_matched_by_pattern(
    possibilities: &mut Vec<SpecificPatternCatch>,
    pattern: &SyntaxPattern,
) {
    // we do not use ::retain because one pattern usually only covers one specific pattern catch,
    // and calling retain for every pattern would thus be quadratic.

    // dear rust, add a version of Vec::retain which swap_removes
    let mut possibility_index = 0;
    loop {
        let Some(possibility) = possibilities.get(possibility_index) else {
            return;
        };
        if pattern_matches_specific_pattern_catch(pattern, possibility) {
            possibilities.swap_remove(possibility_index);
        } else {
            possibility_index += 1;
        }
    }
}
fn pattern_matches_specific_pattern_catch(
    pattern: &SyntaxPattern,
    possibility: &SpecificPatternCatch,
) -> bool {
    match pattern {
        SyntaxPattern::Variable { .. } => true,
        SyntaxPattern::Ignored { .. } => true,
        SyntaxPattern::Variant { name, values } => match possibility {
            SpecificPatternCatch::Variant {
                name: possibility_name,
                values: possibility_values,
            } if name.value == possibility_name => values
                .iter()
                .flat_map(|values| &values.values)
                .zip(possibility_values)
                .all(|(value, possibility_value)| {
                    pattern_matches_specific_pattern_catch(value, possibility_value)
                }),
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
                    pattern_matches_specific_pattern_catch(part, possibility_part)
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
                            .find(|(possibility_name, _)| possibility_name == &field.name.value)
                        {
                            None => true,
                            Some((_, value_possibility)) => {
                                pattern_matches_specific_pattern_catch(value, value_possibility)
                            }
                        }
                    }
                })
            }
            _ => false,
        },
    }
}

fn syntax_query_pattern_check<'a>(
    pattern: &'a SyntaxPattern,
    expected_type: &Type,
    errors: &mut Vec<ErrorNode>,
    introduced_variables: &mut std::collections::HashMap<&'a Name, CheckedLocalVariable>,
    introduced_reachable_bufs: &mut std::collections::HashSet<TypeBuf>,
    // consider moving checking for existing local variables to the callsite
    existing_local_variables: &std::collections::HashMap<&Name, CheckedLocalVariable>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
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
                    used: false,
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
            type_reachable_bufs(
                expected_type,
                vec![],
                type_aliases,
                &mut |type_buf, _path| {
                    introduced_reachable_bufs.insert(type_buf);
                },
            );
        }
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_,
        } => {
            if let Some(actual_type) = type_.as_ref() {
                errors.push(ErrorNode {
                    range: type_range(actual_type),
                    message: Box::from(
                        "ignored query patterns cannot specify an explicit type. Remove it",
                    ),
                });
            }
            type_reachable_bufs(
                expected_type,
                vec![],
                type_aliases,
                &mut |type_buf, _path| {
                    introduced_reachable_bufs.insert(type_buf);
                },
            );
        }
        SyntaxPattern::Variant { name, values } => {
            let expected_type = type_alias_resolve_to_non_alias(expected_type, type_aliases);
            let Type::Choice(origin_choice_type_variants) = expected_type.as_ref() else {
                let mut error_message: String = String::from(
                    "A variant is part of a choice type (for example [a u32  b str]) but the expected type here is\n",
                );
                type_format(&mut error_message, 0, expected_type.as_ref());
                error_message.push_str("\nYou might have intended this pattern to belong to a different if. Use parens for the if case results of ifs with multiple cases");
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: error_message.into_boxed_str(),
                });
                return;
            };
            let Some(expected_value_types) =
                origin_choice_type_variants.iter().find_map(|variant| {
                    if variant.name == name.value {
                        Some(&variant.values)
                    } else {
                        None
                    }
                })
            else {
                let mut error_message: String = format!(
                    "this variant name {} is not included in it's expected type\n",
                    name.value
                );
                type_format(&mut error_message, 0, expected_type.as_ref());
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: error_message.into_boxed_str(),
                });
                return;
            };
            for (value, expected_value_type) in values
                .iter()
                .flat_map(|values| &values.values)
                .zip(expected_value_types)
            {
                syntax_query_pattern_check(
                    value,
                    expected_value_type,
                    errors,
                    introduced_variables,
                    introduced_reachable_bufs,
                    existing_local_variables,
                    type_aliases,
                );
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            let expected_type = type_alias_resolve_to_non_alias(expected_type, type_aliases);
            let Type::Tuple(expected_type_parts) = expected_type.as_ref() else {
                let mut error_message: String =
                    String::from("This pattern matches a tuple but the expected type here is\n");
                type_format(&mut error_message, 0, expected_type.as_ref());
                error_message.push_str(
                    "\nMaybe you intended this pattern to belong to a different if case?",
                );
                errors.push(ErrorNode {
                    range: pattern_range(pattern),
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
                type_format(&mut error_message, 0, expected_type.as_ref());
                error_message.push_str(
                    "\nMaybe you intended this pattern to belong to a different if case?",
                );
                errors.push(ErrorNode {
                    range: pattern_range(pattern),
                    message: error_message.into_boxed_str(),
                });
                return;
            }
            for (part, expected_type_part) in parts.iter().zip(expected_type_parts) {
                syntax_query_pattern_check(
                    part,
                    expected_type_part,
                    errors,
                    introduced_variables,
                    introduced_reachable_bufs,
                    existing_local_variables,
                    type_aliases,
                );
            }
        }
        SyntaxPattern::Record {
            open_curly_start,
            fields,
            closed_curly_start: _,
        } => {
            let expected_type = type_alias_resolve_to_non_alias(expected_type, type_aliases);
            let Type::Record(expected_type_record_fields) = expected_type.as_ref() else {
                let mut error_message: String =
                    String::from("This pattern matches a record but the expected type here is\n");
                type_format(&mut error_message, 0, expected_type.as_ref());
                error_message.push_str(
                    "\nMaybe you intended this pattern to belong to a different if case?",
                );
                errors.push(ErrorNode {
                    range: pattern_range(pattern),
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
                        range: pattern_range(pattern),
                        message: error_message.into_boxed_str(),
                    });
                    return;
                };
                syntax_query_pattern_check(
                    value,
                    &expected_type_field.value,
                    errors,
                    introduced_variables,
                    introduced_reachable_bufs,
                    existing_local_variables,
                    type_aliases,
                );
            }
            if !remaining_expected_type_fields.is_empty() {
                let mut error_message = String::from("missing fields:\n");
                type_format(
                    &mut error_message,
                    0,
                    &Type::Record(remaining_expected_type_fields),
                );
                error_message.push_str("\nAdd them to the remaining fields to the query pattern");
                errors.push(ErrorNode {
                    range: symbol_range(*open_curly_start, "{"),
                    message: error_message.into_boxed_str(),
                });
            }
        }
    }
}
fn syntax_parameter_pattern_check<'a>(
    pattern: &'a SyntaxPattern,
    errors: &mut Vec<ErrorNode>,
    introduced_variables: &mut std::collections::HashMap<&'a Name, CheckedLocalVariable>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    existing_local_variables: &std::collections::HashMap<&Name, CheckedLocalVariable>,
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
                    let Some(actual_type) = syntax_type_check_not_in_type_alias(
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
                    used: false,
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
                let Some(actual_type) = syntax_type_check_not_in_type_alias(
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
        SyntaxPattern::Variant { name, values } => {
            let Some(checked_value_types) = values
                .iter()
                .flat_map(|values| &values.values)
                .map(|value| {
                    syntax_parameter_pattern_check(
                        value,
                        errors,
                        introduced_variables,
                        type_aliases,
                        existing_local_variables,
                        records_used,
                        choices_used,
                    )
                })
                .collect()
            else {
                return None;
            };
            Some(Type::Choice(vec![TypeVariant {
                name: name.value.clone(),
                values: checked_value_types,
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
                    existing_local_variables,
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
                    existing_local_variables,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TypeBuf {
    origin: Type,
    item: Type,
}
#[derive(Clone, PartialEq, Eq)]
enum BufReachPathSegment {
    InVariantValue {
        variant_name: Name,
        value_index: usize,
        value_count: usize,
    },
    InTuplePart(usize),
    InRecordFieldValue(Name),
}
fn type_reachable_bufs(
    type_: &Type,
    path: Vec<BufReachPathSegment>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    insert_reachable_buf: &mut impl FnMut(TypeBuf, Vec<BufReachPathSegment>),
) {
    match type_ {
        Type::Variable(_) => {}
        Type::Tuple(parts) => {
            for (part_index, part) in parts.iter().enumerate() {
                type_reachable_bufs(
                    part,
                    path.iter()
                        .cloned()
                        .chain(std::iter::once(BufReachPathSegment::InTuplePart(
                            part_index,
                        )))
                        .collect(),
                    type_aliases,
                    insert_reachable_buf,
                );
            }
        }
        Type::Record(fields) => {
            for TypeField { name, value } in fields {
                type_reachable_bufs(
                    value,
                    path.iter()
                        .cloned()
                        .chain(std::iter::once(BufReachPathSegment::InRecordFieldValue(
                            name.clone(),
                        )))
                        .collect(),
                    type_aliases,
                    insert_reachable_buf,
                );
            }
        }
        Type::Choice(variants) => {
            if variants.len() == 1
                && let Some(TypeVariant { name, values }) = variants.first()
            {
                for (value_index, value) in values.iter().enumerate() {
                    type_reachable_bufs(
                        value,
                        path.iter()
                            .cloned()
                            .chain(std::iter::once(BufReachPathSegment::InVariantValue {
                                variant_name: name.clone(),
                                value_index,
                                value_count: values.len(),
                            }))
                            .collect(),
                        type_aliases,
                        insert_reachable_buf,
                    );
                }
            }
        }
        Type::CoreConstruct { name, arguments } => {
            if name == type_buf_name
                && let [origin, item] = arguments.as_slice()
            {
                let mut otigin_fully_dealiased = origin.clone();
                type_fully_resolve_aliases(
                    &mut otigin_fully_dealiased,
                    &mut Vec::new(),
                    type_aliases,
                );
                let mut item_fully_dealiased = item.clone();
                type_fully_resolve_aliases(
                    &mut item_fully_dealiased,
                    &mut Vec::new(),
                    type_aliases,
                );
                insert_reachable_buf(
                    TypeBuf {
                        origin: otigin_fully_dealiased,
                        item: item_fully_dealiased,
                    },
                    path,
                );
            }
        }
        Type::Fn { .. } => {}
        Type::Alias { name, arguments } => {
            if let Some(origin_type_alias) = type_aliases.get(name)
                && let Some(resolved_alias) =
                    type_construct_resolve_type_alias(origin_type_alias, arguments.iter().cloned())
            {
                type_reachable_bufs(&resolved_alias, path, type_aliases, insert_reachable_buf);
            }
        }
    }
}
fn function_signature_implicit_bufs<'a>(
    // not +Clone but + Copy in theory
    // but rust doesn't implement Copy "because footgun"
    parameters: impl Iterator<Item = &'a Type> + Clone,
    result: Option<&Type>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) -> std::collections::HashSet<TypeBuf> {
    let mut type_bufs = std::collections::HashSet::new();
    {
        let mut parameter_type_bufs_to_ignore = std::collections::HashSet::new();
        for parameter in parameters.clone() {
            type_reachable_bufs(
                parameter,
                vec![],
                type_aliases,
                &mut |newly_reachable, _| {
                    parameter_type_bufs_to_ignore.insert(newly_reachable);
                },
            );
        }
        for parameter in parameters {
            type_implicit_bufs(
                parameter,
                &parameter_type_bufs_to_ignore,
                type_aliases,
                &mut type_bufs,
            );
        }
    }
    if let Some(result) = result {
        type_insert_implicit_bufs_ignoring_reachable(result, type_aliases, &mut type_bufs);
    }
    type_bufs
}
fn type_insert_implicit_bufs_ignoring_reachable(
    type_: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    type_bufs: &mut std::collections::HashSet<TypeBuf>,
) {
    let mut type_bufs_to_ignore = std::collections::HashSet::new();
    type_reachable_bufs(type_, vec![], type_aliases, &mut |newly_reachable, _| {
        type_bufs_to_ignore.insert(newly_reachable);
    });
    type_implicit_bufs(type_, &type_bufs_to_ignore, type_aliases, type_bufs);
}
/// use in combination with `type_reachable_bufs` to exclude already present bufs
fn type_implicit_bufs(
    type_: &Type,
    type_bufs_to_ignore: &std::collections::HashSet<TypeBuf>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    type_bufs: &mut std::collections::HashSet<TypeBuf>,
) {
    match type_ {
        Type::Variable(_) => {}
        Type::Tuple(parts) => {
            for part in parts {
                type_implicit_bufs(part, type_bufs_to_ignore, type_aliases, type_bufs);
            }
        }
        Type::Record(fields) => {
            for TypeField { name: _, value } in fields {
                type_implicit_bufs(value, type_bufs_to_ignore, type_aliases, type_bufs);
            }
        }
        Type::Choice(variants) => {
            for TypeVariant {
                name: _,
                values: variant_values,
            } in variants
            {
                let mut type_bufs_to_ignore = std::borrow::Cow::Borrowed(type_bufs_to_ignore);
                if variants.len() >= 2 {
                    for variant_value in variant_values {
                        type_reachable_bufs(
                            variant_value,
                            vec![],
                            type_aliases,
                            &mut |newly_reachable, _| {
                                type_bufs_to_ignore.to_mut().insert(newly_reachable);
                            },
                        );
                    }
                }
                for variant_value in variant_values {
                    type_implicit_bufs(
                        variant_value,
                        &type_bufs_to_ignore,
                        type_aliases,
                        type_bufs,
                    );
                }
            }
        }
        Type::CoreConstruct { name, arguments } => {
            if (name == type_slot_name || name == type_span_name)
                && let [origin, item] = arguments.as_slice()
            {
                let mut item_fully_dealiased = item.clone();
                type_fully_resolve_aliases(
                    &mut item_fully_dealiased,
                    &mut Vec::new(),
                    type_aliases,
                );
                let origin_fully_dealiased = origin.clone();
                type_fully_resolve_aliases(
                    &mut item_fully_dealiased,
                    &mut Vec::new(),
                    type_aliases,
                );
                let new_type_buf = TypeBuf {
                    origin: origin_fully_dealiased,
                    item: item_fully_dealiased,
                };
                if type_bufs_to_ignore.contains(&new_type_buf) {
                    return;
                }
                let same_origin_and_item_was_not_already_added = type_bufs.insert(new_type_buf);
                if !same_origin_and_item_was_not_already_added {
                    return;
                }
                let mut item_type_bufs_to_ignore = std::borrow::Cow::Borrowed(type_bufs_to_ignore);
                type_reachable_bufs(item, vec![], type_aliases, &mut |newly_reachable, _| {
                    item_type_bufs_to_ignore.to_mut().insert(newly_reachable);
                });
                type_implicit_bufs(item, &item_type_bufs_to_ignore, type_aliases, type_bufs);
            } else if name == type_buf_name
                && let [_, item] = arguments.as_slice()
            {
                let mut item_type_bufs_to_ignore = std::borrow::Cow::Borrowed(type_bufs_to_ignore);
                type_reachable_bufs(item, vec![], type_aliases, &mut |newly_reachable, _| {
                    item_type_bufs_to_ignore.to_mut().insert(newly_reachable);
                });
                type_implicit_bufs(item, &item_type_bufs_to_ignore, type_aliases, type_bufs);
            } else {
                for argument in arguments {
                    let mut argument_type_bufs_to_ignore =
                        std::borrow::Cow::Borrowed(type_bufs_to_ignore);
                    type_reachable_bufs(
                        argument,
                        vec![],
                        type_aliases,
                        &mut |newly_reachable, _| {
                            argument_type_bufs_to_ignore
                                .to_mut()
                                .insert(newly_reachable);
                        },
                    );
                    type_implicit_bufs(
                        argument,
                        &argument_type_bufs_to_ignore,
                        type_aliases,
                        type_bufs,
                    );
                }
            }
        }
        Type::Fn { parameters, result } => {
            {
                let mut parameter_type_bufs_to_ignore =
                    std::borrow::Cow::Borrowed(type_bufs_to_ignore);
                for parameter in parameters {
                    type_reachable_bufs(
                        parameter,
                        vec![],
                        type_aliases,
                        &mut |newly_reachable, _| {
                            parameter_type_bufs_to_ignore
                                .to_mut()
                                .insert(newly_reachable);
                        },
                    );
                }
                for parameter in parameters {
                    type_implicit_bufs(
                        parameter,
                        &parameter_type_bufs_to_ignore,
                        type_aliases,
                        type_bufs,
                    );
                }
            }
            let mut result_type_bufs_to_ignore = std::borrow::Cow::Borrowed(type_bufs_to_ignore);
            type_reachable_bufs(result, vec![], type_aliases, &mut |newly_reachable, _| {
                result_type_bufs_to_ignore.to_mut().insert(newly_reachable);
            });
            type_implicit_bufs(result, &result_type_bufs_to_ignore, type_aliases, type_bufs);
        }
        Type::Alias { name, arguments } => {
            if let Some(origin_type_alias) = type_aliases.get(name)
                && let Some(resolved_alias) =
                    type_construct_resolve_type_alias(origin_type_alias, arguments.iter().cloned())
            {
                type_implicit_bufs(
                    &resolved_alias,
                    type_bufs_to_ignore,
                    type_aliases,
                    type_bufs,
                );
            }
        }
    }
}

pub fn checked_project_to_zig(
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
    }: &CheckedSyntaxProject,
    syntax_project: &SyntaxProject,
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
                checked_type_aliases,
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
                &syntax_project.expressions,
                project_fn_name,
                checked_project_fn.documentation.as_deref(),
                &checked_project_fn.parameter_types,
                result_type,
                checked_project_fn.result_expression_is_invalid,
                syntax_project_fn.name.start,
                syntax_project_fn.parameters,
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
    output.push_str(") type {\n    return union(enum) { ");
    for variant_name in variant_names {
        output.push_str(&name_to_lowercase_zig(variant_name));
        output.push_str(": ");
        name_to_uppercase_local_zig(output, variant_name);
        output.push_str(", ");
    }
    output.push_str("};\n}\n\n");
}
fn variant_names_to_zig_choice_type_name<'a>(
    output: &mut String,
    variant_names: impl IntoIterator<Item = &'a Name>,
) {
    output.push_str("@\"[");
    let mut variant_names = variant_names.into_iter();
    if let Some(variant_name0) = variant_names.next() {
        output.push_str(&variant_name0.replace("-", "_"));
        for variant_name in variant_names {
            output.push(' ');
            output.push_str(&variant_name.replace("-", "_"));
        }
    }
    output.push_str("]\"");
}
fn project_type_alias_to_zig(
    output: &mut String,
    maybe_documentation: Option<&str>,
    name: &Name,
    parameters: &[Name],
    aliased_type: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
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
        type_to_zig(output, aliased_type, type_aliases);
        output.push(';');
    } else {
        output.push_str("pub fn ");
        output.push_str(&name_to_uppercase_zig(name));
        output.push('(');
        for parameter in parameters {
            output.push_str("comptime ");
            name_to_uppercase_local_zig(output, parameter);
            output.push_str(": SlugType, ");
        }
        output.push_str(") type {\n");
        for parameter in parameters {
            // slots and spans omit their parameters, so we need to mark them as used
            zig_ignore(output, |output| {
                name_to_uppercase_local_zig(output, parameter)
            });
        }
        output.push_str("return ");
        type_to_zig(output, aliased_type, type_aliases);
        output.push_str("; }");
    }
    output.push_str("\n");
    {
        output.push_str("fn @\"clone ");
        output.push_str(&name_to_uppercase_zig(name));
        output.push_str("\"(");
        for parameter in parameters {
            output.push_str("comptime ");
            name_to_uppercase_local_zig(output, parameter);
            output.push_str(": SlugType, ");
        }
        let context_scope_start = lsp_types::Position::default();
        zig_context_variable(output, context_scope_start);
        output.push_str(": anytype, it: anytype) void {\n");
        zig_ignore(output, |output| {
            zig_context_variable(output, context_scope_start)
        });
        zig_ignore(output, |output| output.push_str("it"));
        zig_variable_clone(
            output,
            "it",
            aliased_type,
            context_scope_start,
            0,
            type_aliases,
        );
        output.push_str("}\n");
    }
    {
        output.push_str("fn @\"drop ");
        output.push_str(&name_to_uppercase_zig(name));
        output.push_str("\"(");
        for parameter in parameters {
            output.push_str("comptime ");
            name_to_uppercase_local_zig(output, parameter);
            output.push_str(": SlugType, ");
        }
        let context_scope_start = lsp_types::Position::default();
        zig_context_variable(output, context_scope_start);
        output.push_str(": anytype, it: anytype) void {\n");
        zig_ignore(output, |output| {
            zig_context_variable(output, context_scope_start)
        });
        zig_ignore(output, |output| output.push_str("it"));
        zig_variable_drop(
            output,
            "it",
            aliased_type,
            context_scope_start,
            0,
            type_aliases,
        );
        output.push_str("}\n\n");
    }
}
fn syntax_project_fn_to_zig(
    output: &mut String,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    checked_calls: &std::collections::HashMap<lsp_types::Position, CheckedCall>,
    checked_local_fns: &std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    expressions: &Buf<Expressions, SyntaxExpression>,
    name: &Name,
    maybe_documentation: Option<&str>,
    parameter_types: &[Option<Type>],
    result_type: &Type,
    result_expression_is_invalid: bool,
    name_start: lsp_types::Position,
    syntax_parameters: &[SyntaxPattern],
    syntax_result: &SyntaxExpression,
) {
    if let Some(documentation) = maybe_documentation {
        for documentation_line in documentation.lines() {
            output.push_str("///");
            output.push_str(documentation_line);
            output.push('\n');
        }
    }
    let function_scope_start = name_start;
    output.push_str("pub fn ");
    output.push_str(&name_to_lowercase_zig(name));
    output.push('(');
    let mut type_parameters = std::collections::BTreeSet::new();
    for parameter_type in parameter_types.iter().flatten() {
        type_variables_into(&mut type_parameters, parameter_type);
    }
    type_variables_into(&mut type_parameters, result_type);
    for type_parameter in type_parameters {
        output.push_str("comptime ");
        name_to_uppercase_local_zig(output, type_parameter);
        output.push_str(": SlugType, ");
    }
    zig_context_variable(output, function_scope_start);
    output.push_str(": anytype");
    for (parameter_index, parameter_type) in parameter_types.iter().flatten().enumerate() {
        output.push_str(", @\"%");
        use std::fmt::Write as _;
        _ = write!(output, "{}", parameter_index);
        output.push_str("\": ");
        type_to_zig(output, parameter_type, type_aliases);
    }
    output.push_str(") Error!");
    type_to_zig(output, result_type, type_aliases);
    output.push_str(" {\n");
    let mut parameter_introduced_variables = std::collections::HashMap::with_capacity(2);
    let mut parameter_introduced_variables_last_uses = std::collections::HashMap::with_capacity(2);
    let mut parameter_introduced_reachable_bufs = std::collections::HashMap::new();
    for (parameter_index, (syntax_parameter, _)) in syntax_parameters
        .iter()
        .zip(parameter_types)
        .filter(|(_, parameter_type)| parameter_type.is_some())
        .enumerate()
    {
        syntax_pattern_to_zig_destructuring(
            output,
            syntax_parameter,
            None,
            &format!("@\"%{}\"", parameter_index),
            function_scope_start,
            type_aliases,
            &mut |variable_name, variable_type| {
                parameter_introduced_variables.insert(variable_name, variable_type);
                parameter_introduced_variables_last_uses
                    .insert(variable_name, Vec::with_capacity(1));
            },
            &mut parameter_introduced_reachable_bufs,
        );
    }
    let with_parameters_conntext_scope_start = expression_start(syntax_result, expressions);
    zig_update_context_variable_with_new_reachable_bufs(
        output,
        function_scope_start,
        with_parameters_conntext_scope_start,
        &parameter_introduced_reachable_bufs,
    );
    output.push_str("return ");
    if result_expression_is_invalid {
        zig_incomplete_expression(output);
    } else {
        syntax_expression_local_variables_last_uses(
            syntax_result,
            expressions,
            &mut std::collections::HashSet::with_capacity(2),
            &mut parameter_introduced_variables_last_uses,
        );
        syntax_expression_to_zig(
            output,
            type_aliases,
            project_fns,
            expressions,
            checked_calls,
            checked_local_fns,
            checked_queries,
            &mut parameter_introduced_variables,
            &mut parameter_introduced_variables_last_uses,
            syntax_result,
            with_parameters_conntext_scope_start,
            ZigReturnContext::Expression,
        );
    }
    output.push_str(";\n}\n\n");
}
fn zig_update_context_variable_with_new_reachable_bufs(
    output: &mut String,
    previous_context_scope_start: lsp_types::Position,
    new_context_scope_start: lsp_types::Position,
    introduced_reachable_bufs: &std::collections::HashMap<
        TypeBuf,
        (/* zig path */ Box<str>, Vec<BufReachPathSegment>),
    >,
) {
    output.push_str("const ");
    zig_context_variable(output, new_context_scope_start);
    output.push_str(" = ");
    if introduced_reachable_bufs.is_empty() {
        zig_context_variable(output, previous_context_scope_start);
    } else {
        output.push_str("structMerge(");
        zig_context_variable(output, previous_context_scope_start);
        output.push_str(", structFromFields(&.{");
        for introduced_type_buf in introduced_reachable_bufs.keys() {
            output.push_str("ctxBufName(");
            sloe_type_name_zig(output, &introduced_type_buf.origin);
            output.push_str(", ");
            sloe_type_name_zig(output, &introduced_type_buf.item);
            output.push_str("), ");
        }
        output.push_str("}, .{");
        for (zig_path, buf_reach_path) in introduced_reachable_bufs.values() {
            output.push('&'); // TODO remove once zig internal representation of Buf changes to *Rc(BufCore)
            output.push_str(zig_path.as_ref());
            reachable_type_buf_path_js(output, buf_reach_path);
            output.push_str(", ");
        }
        output.push_str("}))");
    }
    output.push_str(";\n");
    zig_ignore(output, |output| {
        zig_context_variable(output, new_context_scope_start)
    });
}
fn zig_ctx_buf(
    output: &mut String,
    context_variable: impl FnOnce(&mut String),
    origin_dealiased: &Type,
    item_dealiased: &Type,
) {
    output.push_str("@constCast(@field(");
    context_variable(output);
    output.push_str(", ctxBufName(");
    sloe_type_name_zig(output, origin_dealiased);
    output.push_str(", ");
    sloe_type_name_zig(output, item_dealiased);
    output.push_str(")))");
}
fn zig_context_variable(output: &mut String, scope_start: lsp_types::Position) {
    use std::fmt::Write as _;
    output.push_str("@\"%ctx:");
    _ = write!(output, "{}", scope_start.line);
    output.push(':');
    _ = write!(output, "{}", scope_start.character);
    output.push('"');
}
fn zig_context_variable_at_depth(
    output: &mut String,
    scope_start: lsp_types::Position,
    depth: u32,
) {
    if depth == 0 {
        zig_context_variable(output, scope_start);
    } else {
        use std::fmt::Write as _;
        output.push_str("@\"%ctx");
        _ = write!(output, "{}", depth);
        output.push('"');
    }
}
fn zig_incomplete_expression(output: &mut String) {
    output.push_str("std.process.abort()");
}
fn syntax_pattern_to_zig_matches_condition(
    output: &mut String,
    pattern: &SyntaxPattern,
    to_match: &str,
) {
    match pattern {
        SyntaxPattern::Variable { .. } => {
            output.push_str("true");
        }
        SyntaxPattern::Ignored { .. } => {
            output.push_str("true");
        }
        SyntaxPattern::Variant { name, values } => {
            output.push_str("switch (");
            output.push_str(to_match);
            output.push_str(") { .");
            output.push_str(&name_to_lowercase_zig(&name.value));
            output.push_str(" => true, else => false }");
            for (value_index, value) in values.iter().flat_map(|values| &values.values).enumerate()
            {
                output.push_str(" and ");
                syntax_pattern_to_zig_matches_condition(
                    output,
                    value,
                    &if values
                        .as_ref()
                        .is_some_and(|values| values.values.len() >= 2)
                    {
                        format!(
                            "{to_match}.{}.@\"{value_index}\"",
                            name_to_lowercase_zig(&name.value)
                        )
                    } else {
                        format!("{to_match}.{}", name_to_lowercase_zig(&name.value))
                    },
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
                    );
                    output.push_str(" and ");
                }
            }
            output.push_str("true");
        }
    }
}
fn syntax_pattern_to_zig_destructuring<'a>(
    output: &mut String,
    pattern: &'a SyntaxPattern,
    expected_type: Option<&'a Type>,
    to_destructure: &str, // tiny possible optimization: pass Cow<'b, str> and use that as zig path
    context_scope_start: lsp_types::Position,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    insert_introduced_variable: &mut impl FnMut(&'a Name, ZigLocalVariable<'a>),
    introduced_reachable_bufs: &mut std::collections::HashMap<
        TypeBuf,
        (/* zig path */ Box<str>, Vec<BufReachPathSegment>),
    >,
) {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            output.push_str("const ");
            name_to_lowercase_local_zig_introduced_at(output, &name.value, name.start);
            output.push_str(" = ");
            output.push_str(to_destructure);
            output.push_str(";\n");
            zig_ignore(output, |output| {
                name_to_lowercase_local_zig_introduced_at(output, &name.value, name.start)
            });
            insert_introduced_variable(
                &name.value,
                ZigLocalVariable {
                    introduced_start: name.start,
                    type_: expected_type
                        .map(|expected_type| std::borrow::Cow::Borrowed(expected_type))
                        .or_else(|| {
                            type_
                                .as_ref()
                                .map(|type_| std::borrow::Cow::Owned(syntax_type_to_type(type_)))
                        }),
                },
            );
            if let Some(variable_type) = expected_type
                .map(|expected_type| std::borrow::Cow::Borrowed(expected_type))
                .or_else(|| {
                    type_
                        .as_ref()
                        .map(|type_| std::borrow::Cow::Owned(syntax_type_to_type(type_)))
                })
            {
                type_reachable_bufs(
                    &variable_type,
                    vec![],
                    type_aliases,
                    &mut |type_buf, path| {
                        introduced_reachable_bufs
                            .insert(type_buf, (Box::from(to_destructure), path));
                    },
                );
            }
        }
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_,
        } => {
            if let Some(ignored_type) = expected_type
                .map(|expected_type| std::borrow::Cow::Borrowed(expected_type))
                .or_else(|| {
                    type_
                        .as_ref()
                        .map(|type_| std::borrow::Cow::Owned(syntax_type_to_type(type_)))
                })
            {
                let mut ignored_contains_reachable_bufs = false;
                type_reachable_bufs(
                    &ignored_type,
                    vec![],
                    type_aliases,
                    &mut |type_buf, path| {
                        ignored_contains_reachable_bufs = true;
                        introduced_reachable_bufs
                            .insert(type_buf, (Box::from(to_destructure), path));
                    },
                );
                if ignored_contains_reachable_bufs {
                    output.push_str("defer {\n");
                    zig_variable_drop(
                        output,
                        to_destructure,
                        &ignored_type,
                        context_scope_start,
                        0,
                        type_aliases,
                    );
                    output.push_str("}\n");
                } else {
                    zig_variable_drop(
                        output,
                        to_destructure,
                        &ignored_type,
                        context_scope_start,
                        0,
                        type_aliases,
                    );
                }
            }
        }
        SyntaxPattern::Variant { name, values } => {
            let expected_variant_type = match expected_type {
                Some(Type::Choice(variants)) => {
                    variants.iter().find(|variant| variant.name == name.value)
                }
                _ => None,
            };
            for (value_index, value) in values.iter().flat_map(|values| &values.values).enumerate()
            {
                syntax_pattern_to_zig_destructuring(
                    output,
                    value,
                    expected_variant_type.and_then(|variant| variant.values.get(value_index)),
                    &if values
                        .as_ref()
                        .is_some_and(|values| values.values.len() >= 2)
                    {
                        format!(
                            "{to_destructure}.{}.@\"{value_index}\"",
                            name_to_lowercase_zig(&name.value)
                        )
                    } else {
                        format!("{to_destructure}.{}", name_to_lowercase_zig(&name.value))
                    },
                    context_scope_start,
                    type_aliases,
                    insert_introduced_variable,
                    introduced_reachable_bufs,
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
                    match expected_type {
                        Some(Type::Tuple(expected_type_parts)) => {
                            expected_type_parts.get(part_index)
                        }
                        _ => None,
                    },
                    &format!("{to_destructure}.@\"{part_index}\""),
                    context_scope_start,
                    type_aliases,
                    insert_introduced_variable,
                    introduced_reachable_bufs,
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
                        match expected_type {
                            Some(Type::Record(expected_type_fields)) => expected_type_fields
                                .iter()
                                .find(|expected_type_field| {
                                    expected_type_field.name == field.name.value
                                })
                                .map(|expected_type_field| &expected_type_field.value),
                            _ => None,
                        },
                        &format!(
                            "{to_destructure}.{}",
                            name_to_lowercase_zig(&field.name.value)
                        ),
                        context_scope_start,
                        type_aliases,
                        insert_introduced_variable,
                        introduced_reachable_bufs,
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
#[derive(Debug, Clone)]
struct ZigLocalVariable<'a> {
    introduced_start: lsp_types::Position,
    type_: Option<std::borrow::Cow<'a, Type>>,
}
fn syntax_expression_to_zig<'a>(
    output: &mut String,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    expressions: &'a Buf<Expressions, SyntaxExpression>,
    checked_calls: &std::collections::HashMap<lsp_types::Position, CheckedCall>,
    checked_local_fns: &std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &'a std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    local_variables: &mut std::collections::HashMap<&'a Name, ZigLocalVariable<'a>>,
    local_variables_last_uses: &mut std::collections::HashMap<&'a Name, Vec<lsp_types::Position>>,
    expression: &'a SyntaxExpression,
    context_scope_start: lsp_types::Position,
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
            let type_ = syntax_type_to_type(syntax_type);
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
                && let Type::CoreConstruct { name, arguments: _ } = syntax_type_to_type(type_)
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
            let Some(local_variable) = local_variables.get(&name.value) else {
                zig_incomplete_expression(output);
                return;
            };
            if let Some(last_uses) = local_variables_last_uses.get(&name.value)
                && !last_uses.contains(&name.start)
                && let Some(type_) = &local_variable.type_
            {
                let label = match return_context {
                    ZigReturnContext::StatementsFollowedByBreak(label) => label,
                    ZigReturnContext::Expression => {
                        let label = expression_start(expression, expressions);
                        zig_block_start(output, label);
                        label
                    }
                };
                let mut zig_variable_name = String::new();
                name_to_lowercase_local_zig_introduced_at(
                    &mut zig_variable_name,
                    &name.value,
                    local_variable.introduced_start,
                );
                js_variable_clone(output, &zig_variable_name, type_, type_aliases);
                zig_break_start(output, label);
                output.push_str(&zig_variable_name);
                zig_break_end(output);
                if let ZigReturnContext::Expression = return_context {
                    zig_block_end(output);
                }
            } else {
                if let ZigReturnContext::StatementsFollowedByBreak(label) = return_context {
                    zig_break_start(output, label);
                }
                name_to_lowercase_local_zig_introduced_at(
                    output,
                    &name.value,
                    local_variable.introduced_start,
                );
                if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                    zig_break_end(output);
                }
            }
        }
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments: _,
            open_paren_start: _,
            arguments_rest,
            closed_paren_start: _,
        } => {
            let maybe_first_argument = first_argument_dot
                .as_ref()
                .map(|first_argument_dot| expressions.item(&first_argument_dot.argument));
            if let Some(_project_fn) = project_fns.get(&name.value)
                && let Some(checked_call) = checked_calls.get(&name.start)
            {
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
                for type_variable_argument in checked_call.type_variable_replacements.values() {
                    slug_type_zig(output, type_variable_argument, 0, type_aliases);
                    output.push_str(", ");
                }
                specialize_implicits_zig_comma(
                    output,
                    context_scope_start,
                    &checked_call.type_variable_replacements,
                    is_core_fn_taking_allocator_in_zig(&name.value)
                        || !core_fns.contains_key(&name.value),
                );
                for argument in maybe_first_argument.into_iter().chain(arguments_rest) {
                    syntax_expression_to_zig(
                        output,
                        type_aliases,
                        project_fns,
                        expressions,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        local_variables,
                        local_variables_last_uses,
                        argument,
                        context_scope_start,
                        ZigReturnContext::Expression,
                    );
                    output.push_str(", ");
                }
                output.push(')');
                if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                    zig_break_end(output);
                }
            } else if let Some(local_variable) = local_variables.get(&name.value)
                && let Some(local_variable_type) = &local_variable.type_
                && let Type::Fn {
                    parameters: local_variable_type_parameters,
                    result: local_variable_type_result,
                } = type_alias_resolve_to_non_alias(local_variable_type, type_aliases).as_ref()
            {
                if let ZigReturnContext::StatementsFollowedByBreak(label) = return_context {
                    zig_break_start(output, label);
                }
                output.push_str("try ");
                name_to_lowercase_local_zig_introduced_at(
                    output,
                    &name.value,
                    local_variable.introduced_start,
                );
                output.push_str("(structSubset(");
                zig_context_variable(output, context_scope_start);
                output.push_str(", ");
                type_fn_context_zig_type(
                    output,
                    local_variable_type_parameters,
                    local_variable_type_result,
                    type_aliases,
                );
                output.push_str("), ");
                for argument in maybe_first_argument.into_iter().chain(arguments_rest) {
                    syntax_expression_to_zig(
                        output,
                        type_aliases,
                        project_fns,
                        expressions,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        local_variables,
                        local_variables_last_uses,
                        argument,
                        context_scope_start,
                        ZigReturnContext::Expression,
                    );
                    output.push_str(", ");
                }
                output.push(')');
                if let ZigReturnContext::StatementsFollowedByBreak(_) = return_context {
                    zig_break_end(output);
                }
            } else {
                zig_incomplete_expression(output);
            }
        }
        SyntaxExpression::Variant {
            name,
            type_: syntax_type,
            values,
        } => {
            let Some(syntax_type) = syntax_type.as_ref().and_then(|type_| type_.type_.as_ref())
            else {
                zig_incomplete_expression(output);
                return;
            };
            let type_ = syntax_type_to_type(syntax_type);
            let type_ = type_alias_resolve_to_non_alias(&type_, type_aliases);
            let Type::Choice(provided_type_variants) = type_.as_ref() else {
                zig_incomplete_expression(output);
                return;
            };
            let label = match return_context {
                ZigReturnContext::StatementsFollowedByBreak(label) => label,
                ZigReturnContext::Expression => {
                    let label = expression_start(expression, expressions);
                    zig_block_start(output, label);
                    label
                }
            };
            fn values_variable_name(output: &mut String, name_end: lsp_types::Position) {
                use std::fmt::Write as _;
                output.push_str("@\"%value:");
                _ = write!(output, "{}", name_end.line);
                output.push(':');
                _ = write!(output, "{}", name_end.character);
                output.push('"');
            }
            output.push_str("const ");
            let values_variable_label = name_end(with_start_position_as_ref(name));
            values_variable_name(output, values_variable_label);
            output.push_str(" = ");
            match values
                .as_ref()
                .map(|values| values.values.len())
                .unwrap_or(0)
            {
                0 => {
                    output.push_str("{}");
                }
                1 => {
                    if let Some(value) = values.as_ref().and_then(|values| values.values.first()) {
                        syntax_expression_to_zig(
                            output,
                            type_aliases,
                            project_fns,
                            expressions,
                            checked_calls,
                            checked_local_fns,
                            checked_queries,
                            local_variables,
                            local_variables_last_uses,
                            value,
                            context_scope_start,
                            ZigReturnContext::Expression,
                        );
                    }
                }
                2.. => {
                    output.push_str(".{");
                    for value in values.iter().flat_map(|values| &values.values) {
                        syntax_expression_to_zig(
                            output,
                            type_aliases,
                            project_fns,
                            expressions,
                            checked_calls,
                            checked_local_fns,
                            checked_queries,
                            local_variables,
                            local_variables_last_uses,
                            value,
                            context_scope_start,
                            ZigReturnContext::Expression,
                        );
                        output.push_str(", ");
                    }
                    output.push('}');
                }
            }
            output.push_str(";\n");
            zig_break_start(output, label);
            let mut type_variants_sorted = provided_type_variants
                .iter()
                .map(
                    |TypeVariant {
                         name: type_variant_name,
                         values: type_variant_value,
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
                        variant_values_type_to_zig(output, type_variant_value, type_aliases);
                    }
                    None => {
                        output.push_str("@TypeOf(");
                        values_variable_name(output, values_variable_label);
                        output.push(')');
                    }
                }
                output.push_str(", ");
            }
            output.push_str(") { .");
            output.push_str(&name_to_lowercase_zig(&name.value));
            output.push_str(" = ");
            values_variable_name(output, values_variable_label);
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
            output.push_str("@as(");
            type_fn_to_zig(
                output,
                &checked_local_fn.parameter_types,
                &checked_local_fn.result_type,
                type_aliases,
            );
            output.push_str(", struct {\npub fn f(");
            zig_context_variable(output, local_function_scope_start);
            output.push_str(": ");
            type_fn_context_zig_type(
                output,
                &checked_local_fn.parameter_types,
                &checked_local_fn.result_type,
                type_aliases,
            );
            output.push_str(", ");
            for (parameter, parameter_type) in
                parameters.iter().zip(&checked_local_fn.parameter_types)
            {
                function_parameter_name(output, pattern_start(parameter));
                output.push_str(": ");
                type_to_zig(output, parameter_type, type_aliases);
                output.push(',');
            }
            output.push_str(") Error!");
            type_to_zig(output, &checked_local_fn.result_type, type_aliases);
            output.push_str(" {\n");
            zig_ignore(output, |output| {
                zig_context_variable(output, local_function_scope_start)
            });
            let mut parameter_introduced_variables = std::collections::HashMap::with_capacity(2);
            let mut parameter_introduced_variables_last_uses =
                std::collections::HashMap::with_capacity(2);
            let mut parameter_introduced_reachable_bufs = std::collections::HashMap::new();
            for syntax_parameter in parameters {
                let mut parameter_name = String::new();
                function_parameter_name(&mut parameter_name, pattern_start(syntax_parameter));
                syntax_pattern_to_zig_destructuring(
                    output,
                    syntax_parameter,
                    None,
                    &parameter_name,
                    context_scope_start,
                    type_aliases,
                    &mut |name, type_| {
                        parameter_introduced_variables.insert(name, type_);
                        parameter_introduced_variables_last_uses
                            .insert(name, Vec::with_capacity(1));
                    },
                    &mut parameter_introduced_reachable_bufs,
                );
            }
            match result {
                None => {
                    output.push_str("return ");
                    zig_incomplete_expression(output);
                }
                Some(result) => {
                    let result = expressions.item(result);
                    let local_function_with_parameters_context_scope_start =
                        expression_start(result, expressions);
                    zig_update_context_variable_with_new_reachable_bufs(
                        output,
                        local_function_scope_start,
                        local_function_with_parameters_context_scope_start,
                        &parameter_introduced_reachable_bufs,
                    );
                    output.push_str("return ");
                    syntax_expression_to_zig(
                        output,
                        type_aliases,
                        project_fns,
                        expressions,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        &mut parameter_introduced_variables,
                        &mut parameter_introduced_variables_last_uses,
                        result,
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
                ZigReturnContext::Expression => expression_start(expression, expressions),
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
                    checked_calls,
                    checked_local_fns,
                    checked_queries,
                    local_variables,
                    local_variables_last_uses,
                    part,
                    context_scope_start,
                    ZigReturnContext::Expression,
                );
                output.push_str(", ");
            }
            output.push('}');
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
                ZigReturnContext::Expression => expression_start(expression, expressions),
            };
            let mut sorted_field_setters =
                std::collections::BTreeMap::<&str, Option<&'a SyntaxExpression>>::new();
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
                            checked_calls,
                            checked_local_fns,
                            checked_queries,
                            local_variables,
                            local_variables_last_uses,
                            value,
                            context_scope_start,
                            ZigReturnContext::Expression,
                        );
                    }
                }
                output.push_str(", ");
            }
            output.push_str("})");
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
                checked_calls,
                checked_local_fns,
                checked_queries,
                local_variables,
                local_variables_last_uses,
                expressions.item(expression),
                context_scope_start,
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
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        local_variables,
                        local_variables_last_uses,
                        expressions.item(queried),
                        context_scope_start,
                        ZigReturnContext::Expression,
                    );
                }
            }
            output.push_str(";\n");
            zig_ignore(output, |output| output.push_str(&queried_variable_name));
            match result {
                None => {
                    zig_incomplete_expression(output);
                }
                Some(result) => {
                    let result = expressions.item(result);
                    let mut destructuring_introduced_reachable_bufs =
                        std::collections::HashMap::new();
                    let mut destruct_local_variables_last_uses = std::collections::HashMap::new();
                    if let Some(pattern) = pattern {
                        syntax_pattern_to_zig_destructuring(
                            output,
                            pattern,
                            checked_queries
                                .get(equals_start)
                                .map(|query| &query.queried_type),
                            &queried_variable_name,
                            context_scope_start,
                            type_aliases,
                            &mut |name, type_| {
                                local_variables.insert(name, type_);
                                destruct_local_variables_last_uses
                                    .insert(name, Vec::with_capacity(1));
                            },
                            &mut destructuring_introduced_reachable_bufs,
                        );
                    }
                    let with_destructured_context_scope_start =
                        expression_start(result, expressions);
                    zig_update_context_variable_with_new_reachable_bufs(
                        output,
                        context_scope_start,
                        with_destructured_context_scope_start,
                        &destructuring_introduced_reachable_bufs,
                    );
                    syntax_expression_local_variables_last_uses(
                        result,
                        expressions,
                        &mut std::collections::HashSet::new(),
                        &mut destruct_local_variables_last_uses,
                    );
                    local_variables_last_uses.extend(destruct_local_variables_last_uses);
                    syntax_expression_to_zig(
                        output,
                        type_aliases,
                        project_fns,
                        expressions,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        local_variables,
                        local_variables_last_uses,
                        result,
                        with_destructured_context_scope_start,
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
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        local_variables,
                        local_variables_last_uses,
                        expressions.item(queried),
                        context_scope_start,
                        ZigReturnContext::Expression,
                    );
                }
            }
            output.push_str(";\n");
            zig_ignore(output, |output| output.push_str(&queried_variable_name));
            for (case_index, case) in cases.iter().enumerate() {
                if !checked_query.invalid_case_indexes.contains(&case_index) {
                    if cases.len() >= 2 {
                        output.push_str("if (");
                        syntax_pattern_to_zig_matches_condition(
                            output,
                            &case.pattern,
                            &queried_variable_name,
                        );
                        output.push_str(") {\n");
                    }
                    // can be optimized by not cloning if there is only one case
                    let mut case_local_variables = local_variables.clone();
                    let mut case_local_variables_last_uses = std::collections::HashMap::new();
                    let mut case_introduced_reachable_bufs = std::collections::HashMap::new();
                    syntax_pattern_to_zig_destructuring(
                        output,
                        &case.pattern,
                        Some(&checked_query.queried_type),
                        &queried_variable_name,
                        context_scope_start,
                        type_aliases,
                        &mut |name, type_| {
                            case_local_variables.insert(name, type_);
                            case_local_variables_last_uses.insert(name, Vec::with_capacity(1));
                        },
                        &mut case_introduced_reachable_bufs,
                    );
                    match &case.result {
                        None => {
                            zig_incomplete_expression(output);
                        }
                        Some(case_result) => {
                            let case_context_scope_start =
                                expression_start(case_result, expressions);
                            zig_update_context_variable_with_new_reachable_bufs(
                                output,
                                context_scope_start,
                                case_context_scope_start,
                                &case_introduced_reachable_bufs,
                            );
                            syntax_expression_local_variables_last_uses(
                                case_result,
                                expressions,
                                &mut std::collections::HashSet::new(),
                                &mut case_local_variables_last_uses,
                            );
                            case_local_variables_last_uses.extend(
                                local_variables_last_uses
                                    .iter()
                                    .map(|(name, uses)| (*name, uses.clone())),
                            );
                            syntax_expression_to_zig(
                                output,
                                type_aliases,
                                project_fns,
                                expressions,
                                checked_calls,
                                checked_local_fns,
                                checked_queries,
                                &mut case_local_variables,
                                &mut case_local_variables_last_uses,
                                case_result,
                                case_context_scope_start,
                                ZigReturnContext::StatementsFollowedByBreak(label),
                            );
                        }
                    }
                    if cases.len() >= 2 {
                        output.push_str("\n} else ");
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
fn specialize_implicits_zig_comma(
    output: &mut String,
    context_variable_scope_start: lsp_types::Position,
    type_variables_to_substitute: &std::collections::BTreeMap<Name, Type>,
    takes_allocator: bool,
) {
    if type_variables_to_substitute.is_empty() {
        if takes_allocator {
            zig_context_variable(output, context_variable_scope_start);
            output.push_str(", ");
        }
    } else {
        zig_context_variable(output, context_variable_scope_start);
        output.push_str(", ");
        /* above should work. Previously each field name was individually specialized.
        // we include all implicit bufs for each variable in type_variable_replacements
        // because local functions _require_ having those passed
        for (_, type_variable_replacement) in type_variables_to_substitute {
            type_insert_implicit_bufs_ignoring_reachable(
                type_variable_replacement,
                type_aliases,
                implicit_bufs,
            );
        }
        output.push_str(".{ .allocator = ");
        zig_context_variable(output, context_variable_scope_start);
        output.push_str(".allocator, ");
        for implicit_buf in implicit_bufs.iter() {
            output.push('.');
            type_buf_zig_field_name(output, &implicit_buf.origin, &implicit_buf.item);
            output.push_str(" = ");
            zig_context_variable(output, context_variable_scope_start);
            output.push('.');
            let mut specialized_implicit_buf_origin = implicit_buf.origin.clone();
            type_replace_variables(
                type_variables_to_substitute,
                &mut specialized_implicit_buf_origin,
            );
            type_fully_resolve_aliases(
                &mut specialized_implicit_buf_origin,
                &mut Vec::new(),
                type_aliases,
            );
            let mut specialized_implicit_buf_item = implicit_buf.item.clone();
            type_replace_variables(
                type_variables_to_substitute,
                &mut specialized_implicit_buf_item,
            );
            type_fully_resolve_aliases(
                &mut specialized_implicit_buf_item,
                &mut Vec::new(),
                type_aliases,
            );
            type_buf_zig_field_name(
                output,
                &specialized_implicit_buf_origin,
                &specialized_implicit_buf_item,
            );
            output.push_str(", ");
        }
        output.push_str("}, ");
        */
    }
}
fn slug_type_zig(
    output: &mut String,
    type_: &Type,
    depth: u32,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    if let Type::Variable(type_variable) = type_ {
        name_to_uppercase_local_zig(output, type_variable);
        return;
    }
    output.push_str(".{ .name = ");
    // can be optimized
    let mut type_dealiased = type_.clone();
    type_fully_resolve_aliases(&mut type_dealiased, &mut Vec::new(), type_aliases);
    sloe_type_name_zig(output, &type_dealiased);
    output.push_str(", .Ty = ");
    type_to_zig(output, type_, type_aliases);
    output.push_str(", .drop = ");
    zig_drop_function(
        output,
        type_,
        lsp_types::Position::default(),
        depth + 1,
        type_aliases,
    );
    output.push_str(", .clone = ");
    zig_clone_function(
        output,
        type_,
        lsp_types::Position::default(),
        depth + 1,
        type_aliases,
    );
    output.push_str(", }");
}
fn zig_drop_function(
    output: &mut String,
    type_: &Type,
    context_scope_start: lsp_types::Position,
    depth: u32,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    output.push_str("struct { fn f(");
    zig_context_variable_at_depth(output, context_scope_start, depth);
    output.push_str(": anytype, ");
    let to_drop_variable_name = format!("it{}", depth);
    output.push_str(&to_drop_variable_name);
    output.push_str(": anytype) void {\n");
    zig_ignore(output, |output| {
        zig_context_variable_at_depth(output, context_scope_start, depth);
    });
    zig_ignore(output, |output| output.push_str(&to_drop_variable_name));
    zig_variable_drop(
        output,
        &to_drop_variable_name,
        type_,
        context_scope_start,
        depth,
        type_aliases,
    );
    output.push_str("} }.f");
}
fn zig_clone_function(
    output: &mut String,
    type_: &Type,
    context_scope_start: lsp_types::Position,
    depth: u32,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    output.push_str("struct { fn f(");
    zig_context_variable_at_depth(output, context_scope_start, depth);
    output.push_str(": anytype, ");
    let to_clone_variable_name = format!("it{}", depth);
    output.push_str(&to_clone_variable_name);
    output.push_str(": anytype) void {\n");
    zig_ignore(output, |output| {
        zig_context_variable_at_depth(output, context_scope_start, depth);
    });
    zig_ignore(output, |output| output.push_str(&to_clone_variable_name));
    zig_variable_clone(
        output,
        &to_clone_variable_name,
        type_,
        context_scope_start,
        depth,
        type_aliases,
    );
    output.push_str("} }.f");
}

fn zig_variable_drop(
    output: &mut String,
    variable_name: &str,
    type_: &Type,
    context_scope_start: lsp_types::Position,
    depth: u32,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match type_ {
        Type::Variable(type_variable) => {
            name_to_uppercase_local_zig(output, type_variable);
            output.push_str(".drop(");
            zig_context_variable_at_depth(output, context_scope_start, depth);
            output.push_str(", ");
            output.push_str(variable_name);
            output.push_str(");\n");
        }
        Type::Tuple(parts) => {
            for (part_index, part) in parts.iter().enumerate() {
                zig_variable_drop(
                    output,
                    &format!("{variable_name}.@\"{part_index}\""),
                    part,
                    context_scope_start,
                    depth,
                    type_aliases,
                );
            }
        }
        Type::Record(fields) => {
            for TypeField { name, value } in fields {
                zig_variable_drop(
                    output,
                    &format!("{variable_name}.{}", name_to_lowercase_zig(name)),
                    value,
                    context_scope_start,
                    depth,
                    type_aliases,
                );
            }
        }
        Type::Choice(variants) => {
            for TypeVariant { name, values } in variants {
                output.push_str("if (switch (");
                output.push_str(variable_name);
                output.push_str(") { .");
                let variant_name_zig = name_to_lowercase_zig(name);
                output.push_str(&variant_name_zig);
                output.push_str(" => true, else => false }) {\n");
                for (value_index, value) in values.iter().enumerate() {
                    zig_variable_drop(
                        output,
                        &if values.len() >= 2 {
                            format!("{variable_name}.{variant_name_zig}.@\"{value_index}\"")
                        } else {
                            format!("{variable_name}.{variant_name_zig}")
                        },
                        value,
                        context_scope_start,
                        depth,
                        type_aliases,
                    );
                }
                output.push_str("} else ");
            }
            output.push_str("{}\n");
        }
        Type::CoreConstruct { name, arguments } => {
            if name == type_slot_name
                && let [origin, item] = arguments.as_slice()
            {
                zig_ctx_buf(
                    output,
                    |output| zig_context_variable_at_depth(output, context_scope_start, depth),
                    &{
                        let mut origin_dealiased = origin.clone();
                        type_fully_resolve_aliases(
                            &mut origin_dealiased,
                            &mut Vec::new(),
                            type_aliases,
                        );
                        origin_dealiased
                    },
                    &{
                        let mut item_dealiased = item.clone();
                        type_fully_resolve_aliases(
                            &mut item_dealiased,
                            &mut Vec::new(),
                            type_aliases,
                        );
                        item_dealiased
                    },
                );
                output.push_str(".dropSlot(");
                zig_context_variable_at_depth(output, context_scope_start, depth);
                output.push_str(", ");
                output.push_str(variable_name);
                output.push_str(", ");
                zig_drop_function(output, item, context_scope_start, depth + 1, type_aliases);
                output.push_str(");\n");
            } else if name == type_span_name
                && let [origin, item] = arguments.as_slice()
            {
                zig_ctx_buf(
                    output,
                    |output| zig_context_variable_at_depth(output, context_scope_start, depth),
                    &{
                        let mut origin_dealiased = origin.clone();
                        type_fully_resolve_aliases(
                            &mut origin_dealiased,
                            &mut Vec::new(),
                            type_aliases,
                        );
                        origin_dealiased
                    },
                    &{
                        let mut item_dealiased = item.clone();
                        type_fully_resolve_aliases(
                            &mut item_dealiased,
                            &mut Vec::new(),
                            type_aliases,
                        );
                        item_dealiased
                    },
                );
                output.push_str(".dropSpan(");
                zig_context_variable_at_depth(output, context_scope_start, depth);
                output.push_str(", ");
                output.push_str(variable_name);
                output.push_str(", ");
                zig_drop_function(output, item, context_scope_start, depth + 1, type_aliases);
                output.push_str(");\n");
            } else {
                output.push_str(variable_name);
                output.push_str(".dropAndDestroy(");
                zig_context_variable_at_depth(output, context_scope_start, depth);
                output.push_str(".allocator);\n");
            }
        }
        Type::Fn { .. } => {}
        Type::Alias { name, arguments } => {
            output.push_str("@\"drop ");
            output.push_str(&name_to_uppercase_zig(name));
            output.push_str("\"(");
            for argument in arguments {
                slug_type_zig(output, argument, depth, type_aliases); // already increases depth by 1
                output.push_str(", ");
            }
            zig_context_variable_at_depth(output, context_scope_start, depth);
            output.push_str(", ");
            output.push_str(variable_name);
            output.push_str(");\n");
        }
    }
}
fn zig_variable_clone(
    output: &mut String,
    variable_name: &str,
    type_: &Type,
    context_scope_start: lsp_types::Position,
    depth: u32,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match type_ {
        Type::Variable(type_variable) => {
            name_to_uppercase_local_zig(output, type_variable);
            output.push_str(".clone(");
            zig_context_variable_at_depth(output, context_scope_start, depth);
            output.push_str(", ");
            output.push_str(variable_name);
            output.push_str(");\n");
        }
        Type::Tuple(parts) => {
            for (part_index, part) in parts.iter().enumerate() {
                zig_variable_clone(
                    output,
                    &format!("{variable_name}.@\"{part_index}\""),
                    part,
                    context_scope_start,
                    depth,
                    type_aliases,
                );
            }
        }
        Type::Record(fields) => {
            for TypeField { name, value } in fields {
                zig_variable_clone(
                    output,
                    &format!("{variable_name}.{}", name_to_lowercase_zig(name)),
                    value,
                    context_scope_start,
                    depth,
                    type_aliases,
                );
            }
        }
        Type::Choice(variants) => {
            for TypeVariant { name, values } in variants {
                output.push_str("if (switch (");
                output.push_str(variable_name);
                output.push_str(") { .");
                let variant_name_zig = name_to_lowercase_zig(name);
                output.push_str(&variant_name_zig);
                output.push_str(" => true, else => false }) {\n");
                for (value_index, value) in values.iter().enumerate() {
                    zig_variable_clone(
                        output,
                        &if values.len() >= 2 {
                            format!("{variable_name}.{variant_name_zig}.@\"{value_index}\"")
                        } else {
                            format!("{variable_name}.{variant_name_zig}")
                        },
                        value,
                        context_scope_start,
                        depth,
                        type_aliases,
                    );
                }
                output.push_str("} else ");
            }
            output.push_str("{}\n");
        }
        Type::CoreConstruct { name, arguments } => {
            if name == type_slot_name
                && let [origin, item] = arguments.as_slice()
            {
                zig_ctx_buf(
                    output,
                    |output| zig_context_variable_at_depth(output, context_scope_start, depth),
                    &{
                        let mut origin_dealiased = origin.clone();
                        type_fully_resolve_aliases(
                            &mut origin_dealiased,
                            &mut Vec::new(),
                            type_aliases,
                        );
                        origin_dealiased
                    },
                    &{
                        let mut item_dealiased = item.clone();
                        type_fully_resolve_aliases(
                            &mut item_dealiased,
                            &mut Vec::new(),
                            type_aliases,
                        );
                        item_dealiased
                    },
                );
                output.push_str(".cloneSlot(");
                output.push_str(variable_name);
                output.push_str(");\n");
            } else if name == type_span_name
                && let [origin, item] = arguments.as_slice()
            {
                zig_ctx_buf(
                    output,
                    |output| zig_context_variable_at_depth(output, context_scope_start, depth),
                    &{
                        let mut origin_dealiased = origin.clone();
                        type_fully_resolve_aliases(
                            &mut origin_dealiased,
                            &mut Vec::new(),
                            type_aliases,
                        );
                        origin_dealiased
                    },
                    &{
                        let mut item_dealiased = item.clone();
                        type_fully_resolve_aliases(
                            &mut item_dealiased,
                            &mut Vec::new(),
                            type_aliases,
                        );
                        item_dealiased
                    },
                );
                output.push_str(".cloneSpan(");
                output.push_str(variable_name);
                output.push_str(");\n");
            } else {
                output.push_str(variable_name);
                output.push_str(".clone();\n");
            }
        }
        Type::Fn { .. } => {}
        Type::Alias { name, arguments } => {
            output.push_str("@\"clone ");
            output.push_str(&name_to_uppercase_zig(name));
            output.push_str("\"(");
            for argument in arguments {
                slug_type_zig(output, argument, depth, type_aliases); // already increases depth + 1
                output.push_str(", ");
            }
            zig_context_variable_at_depth(output, context_scope_start, depth);
            output.push_str(", ");
            output.push_str(variable_name);
            output.push_str(");\n");
        }
    }
}
pub fn type_format_single_line_with_zig_variables(formatted: &mut String, type_: &Type) {
    match type_ {
        Type::Variable(name) => {
            formatted.push_str("\" ++ ");
            name_to_uppercase_local_zig(formatted, name);
            formatted.push_str(".name ++ \"");
        }
        Type::CoreConstruct { name, arguments } | Type::Alias { name, arguments } => {
            formatted.push_str(name);
            if let Some((argument0, argument1_up)) = arguments.split_first() {
                formatted.push('(');
                type_format_single_line_with_zig_variables(formatted, argument0);
                for argument in argument1_up {
                    formatted.push(' ');
                    type_format_single_line_with_zig_variables(formatted, argument);
                }
                formatted.push(')');
            }
        }
        Type::Fn { parameters, result } => {
            formatted.push_str("fn(");
            if let Some((parameter0, parameter1_up)) = parameters.split_first() {
                type_format_single_line_with_zig_variables(formatted, parameter0);
                for parameter in parameter1_up {
                    formatted.push(' ');
                    type_format_single_line_with_zig_variables(formatted, parameter);
                }
            }
            formatted.push_str(") ");
            type_format_single_line_with_zig_variables(formatted, result);
        }
        Type::Tuple(parts) => {
            formatted.push('*');
            if let Some((part0, part1_up)) = parts.split_first() {
                formatted.push('(');
                type_format_single_line_with_zig_variables(formatted, part0);
                for part in part1_up {
                    formatted.push(' ');
                    type_format_single_line_with_zig_variables(formatted, part);
                }
                formatted.push(')');
            }
        }
        Type::Record(fields) => {
            fn type_field_format_single_line_with_zig_variables(
                formatted: &mut String,
                type_field: &TypeField,
            ) {
                formatted.push_str(&type_field.name);
                formatted.push(' ');
                type_format_single_line_with_zig_variables(formatted, &type_field.value);
            }
            formatted.push('{');
            if let Some((field0, field1_up)) = fields.split_first() {
                type_field_format_single_line_with_zig_variables(formatted, field0);
                for field in field1_up {
                    formatted.push(' ');
                    type_field_format_single_line_with_zig_variables(formatted, field);
                }
            }
            formatted.push('}');
        }
        Type::Choice(variants) => {
            fn type_variant_format_single_line_with_zig_variables(
                formatted: &mut String,
                type_variant: &TypeVariant,
            ) {
                formatted.push_str(&type_variant.name);
                if let Some((value0, value1_up)) = type_variant.values.split_first() {
                    formatted.push('(');
                    type_format_single_line_with_zig_variables(formatted, value0);
                    for variant_value in value1_up {
                        formatted.push(' ');
                        type_format_single_line_with_zig_variables(formatted, variant_value);
                    }
                    formatted.push(')');
                }
            }
            formatted.push('[');
            if let Some((variant0, variant1_up)) = variants.split_first() {
                type_variant_format_single_line_with_zig_variables(formatted, variant0);
                for variant in variant1_up {
                    formatted.push(' ');
                    type_variant_format_single_line_with_zig_variables(formatted, variant);
                }
            }
            formatted.push(']');
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
/// mark as not unused, regardless of whether the given variable name is used later or not
fn zig_ignore(output: &mut String, variable_name: impl FnOnce(&mut String)) {
    output.push_str("_ = @TypeOf(");
    variable_name(output);
    output.push_str(");\n");
}
fn type_to_zig(
    output: &mut String,
    type_: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match type_ {
        Type::Variable(name) => {
            name_to_uppercase_local_zig(output, name);
            output.push_str(".Ty");
        }
        Type::Tuple(parts) => {
            if parts.is_empty() {
                output.push_str(empty_zig_type_name);
            } else {
                output.push_str("struct { ");
                for part in parts {
                    type_to_zig(output, part, type_aliases);
                    output.push_str(", ");
                }
                output.push('}');
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
                    type_to_zig(output, value, type_aliases);
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
                for TypeVariant { name: _, values } in variants_sorted {
                    variant_values_type_to_zig(output, values, type_aliases);
                    output.push_str(", ");
                }
                output.push(')');
            }
        }
        Type::CoreConstruct { name, arguments } => {
            // zig otherwise gets in trouble trying unroll "infinite" type aliases :(
            if name == type_slot_name {
                output.push_str("SlotCore");
            } else if name == type_span_name {
                output.push_str("SpanCore");
            } else {
                output.push_str(&name_to_uppercase_zig(name));
                if !arguments.is_empty() {
                    output.push('(');
                    for argument in arguments {
                        type_to_zig(output, argument, type_aliases);
                        output.push_str(", ");
                    }
                    output.push(')');
                }
            }
        }
        Type::Alias { name, arguments } => {
            output.push_str(&name_to_uppercase_zig(name));
            if !arguments.is_empty() {
                output.push('(');
                for argument in arguments {
                    slug_type_zig(output, argument, 0, type_aliases);
                    output.push_str(", ");
                }
                output.push(')');
            }
        }
        Type::Fn { parameters, result } => {
            type_fn_to_zig(output, parameters, result, type_aliases);
        }
    }
}
fn type_fn_context_zig_type(
    output: &mut String,
    parameters: &[Type],
    result: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    output.push_str("Ctx(&.{ ");
    let mut type_variables = std::collections::BTreeSet::new();
    for parameter in parameters {
        type_variables_into(&mut type_variables, parameter);
    }
    type_variables_into(&mut type_variables, result);
    for type_variable in type_variables {
        name_to_uppercase_local_zig(output, type_variable);
        output.push_str(".Bufs, ");
    }
    for implicit_buf in
        function_signature_implicit_bufs(parameters.iter(), Some(result), type_aliases)
    {
        output.push_str("CtxBuf(");
        zig_type_name_and_ty(output, &implicit_buf.origin, type_aliases);
        output.push_str(", ");
        zig_type_name_and_ty(output, &implicit_buf.item, type_aliases);
        output.push_str("), ");
    }
    output.push_str("})");
}
fn type_fn_to_zig(
    output: &mut String,
    parameters: &[Type],
    result: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    output.push_str("*const fn(");
    type_fn_context_zig_type(output, parameters, result, type_aliases);
    output.push_str(", ");
    for parameter in parameters {
        type_to_zig(output, parameter, type_aliases);
        output.push_str(", ");
    }
    output.push_str(") Error!");
    type_to_zig(output, result, type_aliases);
}
fn variant_values_type_to_zig(
    output: &mut String,
    values: &[Type],
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match values.len() {
        0 => {
            output.push_str(empty_zig_type_name);
        }
        1 => {
            if let Some(value) = values.first() {
                type_to_zig(output, value, type_aliases);
            }
        }
        2.. => {
            output.push_str("struct { ");
            for value in values {
                type_to_zig(output, value, type_aliases);
                output.push_str(", ");
            }
            output.push('}');
        }
    }
}
fn zig_type_name_and_ty(
    output: &mut String,
    type_: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    output.push_str(".{ .Ty = ");
    type_to_zig(output, type_, type_aliases);
    output.push_str(", .name = ");
    sloe_type_name_zig(output, type_);
    output.push_str(" }");
}
fn sloe_type_name_zig(output: &mut String, type_: &Type) {
    // can be made prettier by omitting "" ++ ..type_.. ++ "" when type_ is a variable
    output.push('"');
    type_format_single_line_with_zig_variables(output, type_);
    output.push('"');
}
fn name_to_uppercase_zig(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_uppercase();
    }
    // Not sure if type variables in core code can actually collide with generated type names?
    match sanitized.as_str() {
        "Record"
        | "RecordWithFieldNames"
        | "StructMerge"
        | "StructMergeAll"
        | "StructFromFields"
        | "Ctx"
        | "CtxBuf"
        | "Error"
        | "SlugType" => sanitized + "ø",
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

const empty_zig_type_name: &str = "void";
const choice_empty_zig_type_name: &str = "@\"[]\"";
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
const zig_unrelated_top_level_lowercase_names: [&str; 15] = [
    local_unnamed_function_name,
    empty_zig_type_name,
    "std",
    "record",
    "structMerge",
    "structSubset",
    "structFromFields",
    "ctxBufName",
    "ctxBuf",
    "slugType",
    "u32AddOrOutOfMem",
    "usizeAddOrOutOfMem",
    "strideOf",
    "mathOrderToOrder",
    "structMerge",
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

pub fn checked_project_to_js(
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
        checked_calls,
        checked_local_fns: _,
        checked_queries,
    }: &CheckedSyntaxProject,
    syntax_project: &SyntaxProject,
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
                checked_type_aliases,
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
                checked_project_fns,
                checked_type_aliases,
                checked_queries,
                checked_calls,
                &syntax_project.expressions,
                project_fn_name,
                checked_project_fn.documentation.as_deref(),
                &checked_project_fn.parameter_types,
                syntax_project_fn.parameters,
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
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
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
    output.push_str(&name_to_uppercase_js_segment(name));
    output.push_str("\n*/\n");

    // can be improved: add @param and @type and @template comment
    output.push_str("function drop$");
    output.push_str(&name_to_uppercase_js_segment(name));
    output.push_str("(it, $) {\n");
    js_variable_drop(output, "it", aliased_type, type_aliases);
    output.push_str("}\n");

    // can be improved: add @param and @type and @template comment
    output.push_str("function clone$");
    output.push_str(&name_to_uppercase_js_segment(name));
    output.push_str("(it, $) {\n");
    js_variable_clone(output, "it", aliased_type, type_aliases);
    output.push_str("}\n\n");
}
fn syntax_project_fn_to_js(
    output: &mut String,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_calls: &std::collections::HashMap<lsp_types::Position, CheckedCall>,
    expressions: &Buf<Expressions, SyntaxExpression>,
    name: &Name,
    maybe_documentation: Option<&str>,
    parameter_types: &[Option<Type>],
    syntax_parameters: &[SyntaxPattern],
    result_type: &Type,
    result_expression_is_invalid: bool,
    syntax_result: &SyntaxExpression,
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
    for type_parameter in &type_parameters {
        output.push_str("@template ");
        name_to_uppercase_local_js(output, type_parameter);
        output.push(' ');
    }

    let implicit_bufs = function_signature_implicit_bufs(
        parameter_types.iter().flatten(),
        Some(result_type),
        type_aliases,
    );
    for (parameter_index, parameter_type) in parameter_types.iter().flatten().enumerate() {
        output.push_str("@param {");
        type_to_js(output, parameter_type);
        output.push_str("} a");
        use std::fmt::Write as _;
        _ = write!(output, "{parameter_index}");
        output.push('\n');
    }
    if !implicit_bufs.is_empty() {
        output.push_str("@param {{");
        for implicit_buf in &implicit_bufs {
            type_buf_js_variable_name(output, &implicit_buf.origin, &implicit_buf.item);
            output.push_str(": Buf<");
            type_to_js(output, &implicit_buf.origin);
            output.push_str(", ");
            type_to_js(output, &implicit_buf.item);
            output.push_str(">, ");
        }
        for type_parameter in type_parameters {
            output.push_str("drop$");
            output.push_str(&name_to_uppercase_js_segment(type_parameter));
            output.push_str(": (_: ");
            name_to_uppercase_local_js(output, type_parameter);
            output.push_str(") => void, clone$");
            output.push_str(&name_to_uppercase_js_segment(type_parameter));
            output.push_str(": (_: ");
            name_to_uppercase_local_js(output, type_parameter);
            output.push_str(") => void, ");
        }
        output.push_str("}} $\n");
    }

    output.push_str("@returns {");
    type_to_js(output, result_type);
    output.push_str("}\n*/\n");
    let function_scope_start = expression_start(syntax_result, expressions);
    output.push_str("export function ");
    output.push_str(&name_to_lowercase_js(name));
    output.push('(');
    for (parameter_index, _) in parameter_types.iter().flatten().enumerate() {
        output.push('a');
        use std::fmt::Write as _;
        _ = write!(output, "{}", parameter_index);
        output.push_str(", ");
    }
    if !implicit_bufs.is_empty() {
        output.push('$');
    }
    output.push_str(") {\n");
    if implicit_bufs.is_empty() {
        output.push_str("const $ = {};\n");
    }
    let mut parameter_introduced_variables = std::collections::HashMap::with_capacity(2);
    let mut parameter_introduced_variables_last_uses = std::collections::HashMap::with_capacity(2);
    for (syntax_parameter_index, (syntax_parameter, _)) in syntax_parameters
        .iter()
        .zip(parameter_types)
        .filter(|(_, type_)| type_.is_some())
        .enumerate()
    {
        syntax_pattern_to_js_destructuring(
            output,
            syntax_parameter,
            None,
            &format!("a{}", syntax_parameter_index),
            &mut |variable_name, variable_type| {
                parameter_introduced_variables.insert(variable_name, variable_type);
                parameter_introduced_variables_last_uses
                    .insert(variable_name, Vec::with_capacity(1));
            },
            type_aliases,
        );
    }
    output.push_str("let ");
    js_scope_result_variable(output, function_scope_start);
    output.push_str(";\n");
    if result_expression_is_invalid {
        js_incomplete_statement();
    } else {
        syntax_expression_local_variables_last_uses(
            syntax_result,
            expressions,
            &mut std::collections::HashSet::with_capacity(2),
            &mut parameter_introduced_variables_last_uses,
        );
        syntax_expression_to_js(
            output,
            project_fns,
            type_aliases,
            expressions,
            checked_queries,
            checked_calls,
            &mut parameter_introduced_variables,
            &mut parameter_introduced_variables_last_uses,
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
        Type::Tuple(parts) => {
            if parts.is_empty() {
                output.push_str(empty_js_type_name);
            } else {
                output.push('[');
                for part in parts {
                    type_to_js(output, part);
                    output.push_str(", ");
                }
                output.push(']');
            }
        }
        Type::Record(fields) => {
            if fields.is_empty() {
                output.push_str(empty_js_type_name);
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
                fn variant_type_to_js(output: &mut String, variant: &TypeVariant) {
                    output.push_str("{ ");
                    output.push_str(&name_to_lowercase_js(&variant.name));
                    output.push_str(": ");
                    match variant.values.len() {
                        0 => {
                            output.push_str(empty_js_type_name);
                        }
                        1 => {
                            if let Some(variant_value) = variant.values.first() {
                                type_to_js(output, variant_value);
                            }
                        }
                        2.. => {
                            output.push('[');
                            for variant_value in &variant.values {
                                type_to_js(output, variant_value);
                                output.push_str(", ");
                            }
                            output.push(']');
                        }
                    }
                    output.push_str(" }");
                }
                variant_type_to_js(output, variant0);
                for variant in variant1_up {
                    output.push_str(" | ");
                    variant_type_to_js(output, variant);
                }
            }
        },
        Type::CoreConstruct { name, arguments } | Type::Alias { name, arguments } => {
            output.push_str(&name_to_uppercase_js_segment(name));
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
fn syntax_pattern_to_js_matches_condition(
    output: &mut String,
    pattern: &SyntaxPattern,
    to_match: &str,
) {
    match pattern {
        SyntaxPattern::Variable { .. } => {
            output.push_str("true");
        }
        SyntaxPattern::Ignored { .. } => {
            output.push_str("true");
        }
        SyntaxPattern::Variant { name, values } => {
            output.push('"');
            output.push_str(&name_to_lowercase_js(&name.value));
            output.push_str("\" in ");
            output.push_str(to_match);
            for (value_index, value) in values.iter().flat_map(|values| &values.values).enumerate()
            {
                output.push_str(" && ");
                syntax_pattern_to_js_matches_condition(
                    output,
                    value,
                    &if values
                        .as_ref()
                        .is_some_and(|values| values.values.len() >= 2)
                    {
                        format!("{to_match}.{}", name_to_lowercase_js(&name.value))
                    } else {
                        format!(
                            "{to_match}.{}[{value_index}]",
                            name_to_lowercase_js(&name.value)
                        )
                    },
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
                    );
                    output.push_str(" && ");
                }
            }
            output.push_str("true");
        }
    }
}
fn syntax_pattern_to_js_destructuring<'a>(
    output: &mut String,
    pattern: &'a SyntaxPattern,
    expected_type: Option<&'a Type>,
    to_destructure: &str,
    insert_introduced_variable: &mut impl FnMut(&'a Name, JsLocalVariable<'a>),
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match pattern {
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_,
        } => {
            if let Some(ignored_type) = expected_type
                .map(|expected_type| std::borrow::Cow::Borrowed(expected_type))
                .or_else(|| {
                    type_
                        .as_ref()
                        .map(|type_| std::borrow::Cow::Owned(syntax_type_to_type(type_)))
                })
            {
                let mut reachable_bufs = std::collections::HashMap::new();
                type_reachable_bufs(
                    &ignored_type,
                    vec![],
                    type_aliases,
                    &mut |type_buf, path| {
                        reachable_bufs.insert(type_buf, path);
                    },
                );
                for (reachable_type_buf, reachable_type_buf_path) in reachable_bufs {
                    output.push_str("$.");
                    type_buf_js_variable_name(
                        output,
                        &reachable_type_buf.origin,
                        &reachable_type_buf.item,
                    );
                    output.push_str(" = ");
                    output.push_str(to_destructure);
                    reachable_type_buf_path_js(output, &reachable_type_buf_path);
                    output.push_str(";\n");
                }
                // okay because bufs are not dropped but garbage collected
                js_variable_drop(output, to_destructure, &ignored_type, type_aliases);
            }
        }
        SyntaxPattern::Variable { name, type_ } => {
            output.push_str("const ");
            name_to_lowercase_local_js(output, &name.value);
            output.push_str(" = ");
            output.push_str(to_destructure);
            output.push_str(";\n");
            let variable_type = expected_type
                .map(|expected_type| std::borrow::Cow::Borrowed(expected_type))
                .or_else(|| {
                    type_
                        .as_ref()
                        .map(|type_| std::borrow::Cow::Owned(syntax_type_to_type(type_)))
                });
            if let Some(variable_type) = &variable_type {
                let mut reachable_bufs = std::collections::HashMap::new();
                type_reachable_bufs(
                    variable_type.as_ref(),
                    vec![],
                    type_aliases,
                    &mut |type_buf, path| {
                        reachable_bufs.insert(type_buf, path);
                    },
                );
                for (reachable_type_buf, reachable_type_buf_path) in reachable_bufs {
                    output.push_str("$.");
                    type_buf_js_variable_name(
                        output,
                        &reachable_type_buf.origin,
                        &reachable_type_buf.item,
                    );
                    output.push_str(" = ");
                    name_to_lowercase_local_js(output, &name.value);
                    reachable_type_buf_path_js(output, &reachable_type_buf_path);
                    output.push_str(";\n");
                }
            }
            insert_introduced_variable(
                &name.value,
                JsLocalVariable {
                    type_: variable_type,
                },
            );
        }
        SyntaxPattern::Variant { name, values } => {
            let expected_variant_type = match expected_type {
                Some(Type::Choice(variants)) => {
                    variants.iter().find(|variant| variant.name == name.value)
                }
                _ => None,
            };
            for (value_index, value) in values.iter().flat_map(|values| &values.values).enumerate()
            {
                syntax_pattern_to_js_destructuring(
                    output,
                    value,
                    expected_variant_type.and_then(|variant| variant.values.get(value_index)),
                    &if values
                        .as_ref()
                        .is_some_and(|values| values.values.len() >= 2)
                    {
                        format!(
                            "{to_destructure}.{}[{value_index}]",
                            name_to_lowercase_js(&name.value)
                        )
                    } else {
                        format!("{to_destructure}.{}", name_to_lowercase_js(&name.value))
                    },
                    insert_introduced_variable,
                    type_aliases,
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
                    match expected_type {
                        Some(Type::Tuple(expected_type_parts)) => {
                            expected_type_parts.get(part_index)
                        }
                        _ => None,
                    },
                    &format!("{to_destructure}[{part_index}]"),
                    insert_introduced_variable,
                    type_aliases,
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
                        match expected_type {
                            Some(Type::Record(expected_type_fields)) => expected_type_fields
                                .iter()
                                .find(|expected_type_field| {
                                    expected_type_field.name == field.name.value
                                })
                                .map(|expected_type_field| &expected_type_field.value),
                            _ => None,
                        },
                        &format!(
                            "{to_destructure}.{}",
                            name_to_lowercase_js(&field.name.value)
                        ),
                        insert_introduced_variable,
                        type_aliases,
                    );
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
struct JsLocalVariable<'a> {
    type_: Option<std::borrow::Cow<'a, Type>>,
}
fn syntax_expression_to_js<'a>(
    output: &mut String,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    expressions: &'a Buf<Expressions, SyntaxExpression>,
    checked_queries: &'a std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_calls: &'a std::collections::HashMap<lsp_types::Position, CheckedCall>,
    // would be nice if those 2 could be combined
    local_variables: &mut std::collections::HashMap<&'a Name, JsLocalVariable<'a>>,
    local_variables_last_uses: &mut std::collections::HashMap<&'a Name, Vec<lsp_types::Position>>,
    expression: &'a SyntaxExpression,
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
            let type_ = syntax_type_to_type(syntax_type);
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
            let Some(local_variable) = local_variables.get(&name.value) else {
                js_scope_result_variable(output, scope_start);
                output.push_str(" = ");
                name_to_lowercase_local_js(output, &name.value);
                output.push_str(";\n");
                return;
            };
            let mut js_variable_name = String::new();
            name_to_lowercase_local_js(&mut js_variable_name, &name.value);
            if let Some(last_uses) = local_variables_last_uses.get(&name.value)
                && !last_uses.contains(&name.start)
                && let Some(type_) = &local_variable.type_
            {
                js_variable_clone(output, &js_variable_name, type_, type_aliases);
            }
            js_scope_result_variable(output, scope_start);
            output.push_str(" = ");
            output.push_str(&js_variable_name);
            output.push_str(";\n");
        }
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments: _,
            open_paren_start: _,
            arguments_rest,
            closed_paren_start: _,
        } => {
            let maybe_first_argument = first_argument_dot
                .as_ref()
                .map(|first_argument_dot| expressions.item(&first_argument_dot.argument));
            for argument in maybe_first_argument.into_iter().chain(arguments_rest) {
                // not start because the first argument start matches the full function start
                let argument_scope_start = expression_end(argument, expressions);
                output.push_str("let ");
                js_scope_result_variable(output, argument_scope_start);
                output.push_str(";\n");
                syntax_expression_to_js(
                    output,
                    project_fns,
                    type_aliases,
                    expressions,
                    checked_queries,
                    checked_calls,
                    local_variables,
                    local_variables_last_uses,
                    argument,
                    argument_scope_start,
                );
            }
            js_scope_result_variable(output, scope_start);
            output.push_str(" = ");
            if let Some(project_fn) = project_fns.get(&name.value)
                && let Some(checked_call) = checked_calls.get(&name.start)
            {
                output.push_str(&name_to_lowercase_js(&name.value));
                output.push('(');
                let implicit_bufs = function_signature_implicit_bufs(
                    project_fn.parameter_types.iter().flatten(),
                    project_fn.result_type.as_ref(),
                    type_aliases,
                );
                for argument in maybe_first_argument.into_iter().chain(arguments_rest) {
                    js_scope_result_variable(output, expression_end(argument, expressions));
                    output.push_str(", ");
                }
                specialize_implicits_js(
                    output,
                    &implicit_bufs,
                    &checked_call.type_variable_replacements,
                    type_aliases,
                );
                output.push_str(");\n");
            } else if let Some(_local_variable) = local_variables.get(&name.value) {
                // no need to check for clone since functions are always freely copy-able (closures would not be)
                name_to_lowercase_local_js(output, &name.value);
                output.push('(');
                for argument in maybe_first_argument.into_iter().chain(arguments_rest) {
                    js_scope_result_variable(output, expression_end(argument, expressions));
                    output.push_str(", ");
                }
                // no need to specialize $
                output.push_str("$);\n");
            }
        }
        SyntaxExpression::Variant {
            name,
            type_: _,
            values,
        } => {
            for value in values.iter().flat_map(|values| &values.values) {
                let value_scope_start = expression_start(value, expressions);
                output.push_str("let ");
                js_scope_result_variable(output, value_scope_start);
                output.push_str(";\n");
                syntax_expression_to_js(
                    output,
                    project_fns,
                    type_aliases,
                    expressions,
                    checked_queries,
                    checked_calls,
                    local_variables,
                    local_variables_last_uses,
                    value,
                    value_scope_start,
                );
            }
            js_scope_result_variable(output, scope_start);
            output.push_str(" = { ");
            output.push_str(&name_to_lowercase_js(&name.value));
            output.push_str(": ");
            match values
                .as_ref()
                .map(|values| values.values.len())
                .unwrap_or(0)
            {
                0 => {
                    output.push_str("undefined");
                }
                1 => {
                    if let Some(value) = values.as_ref().and_then(|values| values.values.first()) {
                        let value_scope_start = expression_start(value, expressions);
                        js_scope_result_variable(output, value_scope_start);
                    }
                }
                2.. => {
                    output.push('[');
                    for value in values.iter().flat_map(|values| &values.values) {
                        let value_scope_start = expression_start(value, expressions);
                        js_scope_result_variable(output, value_scope_start);
                        output.push_str(", ");
                    }
                    output.push(']');
                }
            }
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
            output.push_str(" = (");
            for (parameter_index, _) in parameters.iter().enumerate() {
                output.push('a');
                use std::fmt::Write as _;
                _ = write!(output, "{parameter_index}");
                output.push_str(", ");
            }
            output.push_str("$) => {\n");
            'in_function: {
                let mut parameter_introduced_variables =
                    std::collections::HashMap::with_capacity(2);
                let mut parameter_introduced_variables_last_uses =
                    std::collections::HashMap::with_capacity(2);
                for (parameter_index, syntax_parameter) in parameters.iter().enumerate() {
                    syntax_pattern_to_js_destructuring(
                        output,
                        syntax_parameter,
                        None,
                        &format!("a{parameter_index}"),
                        &mut |name, type_| {
                            parameter_introduced_variables.insert(name, type_);
                            parameter_introduced_variables_last_uses
                                .insert(name, Vec::with_capacity(1));
                        },
                        type_aliases,
                    );
                }
                let Some(result) = result else {
                    js_incomplete_statement();
                    break 'in_function;
                };
                let result = expressions.item(result);
                let result_scope_start = expression_start(result, expressions);
                output.push_str("let ");
                js_scope_result_variable(output, result_scope_start);
                output.push_str(";\n");
                syntax_expression_local_variables_last_uses(
                    result,
                    expressions,
                    &mut std::collections::HashSet::with_capacity(2),
                    &mut parameter_introduced_variables_last_uses,
                );
                syntax_expression_to_js(
                    output,
                    project_fns,
                    type_aliases,
                    expressions,
                    checked_queries,
                    checked_calls,
                    &mut parameter_introduced_variables,
                    &mut parameter_introduced_variables_last_uses,
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
                js_scope_result_variable(output, expression_start(part, expressions));
                output.push_str(";\n");
                syntax_expression_to_js(
                    output,
                    project_fns,
                    type_aliases,
                    expressions,
                    checked_queries,
                    checked_calls,
                    local_variables,
                    local_variables_last_uses,
                    part,
                    expression_start(part, expressions),
                );
                output.push('\n');
            }
            js_scope_result_variable(output, scope_start);
            output.push_str(" = [ ");
            for part in parts {
                js_scope_result_variable(output, expression_start(part, expressions));
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
                    js_scope_result_variable(output, expression_start(value, expressions));
                    output.push_str(";\n");
                    syntax_expression_to_js(
                        output,
                        project_fns,
                        type_aliases,
                        expressions,
                        checked_queries,
                        checked_calls,
                        local_variables,
                        local_variables_last_uses,
                        value,
                        expression_start(value, expressions),
                    );
                    output.push('\n');
                }
            }
            js_scope_result_variable(output, scope_start);
            output.push_str(" = { ");
            for field in fields {
                if let Some(value) = &field.value {
                    output.push_str(&name_to_lowercase_js(&field.name.value));
                    output.push_str(": ");
                    js_scope_result_variable(output, expression_start(value, expressions));
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
                        project_fns,
                        type_aliases,
                        expressions,
                        checked_queries,
                        checked_calls,
                        local_variables,
                        local_variables_last_uses,
                        expressions.item(expression),
                        scope_start,
                    );
                }
            }
        }
        SyntaxExpression::Destruct {
            equals_start,
            pattern,
            queried,
            result,
        } => {
            if let Some(queried) = queried {
                let queried = expressions.item(queried);
                let queried_scope_start = expression_start(queried, expressions);
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
                    project_fns,
                    type_aliases,
                    expressions,
                    checked_queries,
                    checked_calls,
                    local_variables,
                    local_variables_last_uses,
                    queried,
                    queried_scope_start,
                );
                output.push_str(";\n");
                if let Some(pattern) = pattern {
                    let mut destruct_local_variables_last_uses = std::collections::HashMap::new();
                    syntax_pattern_to_js_destructuring(
                        output,
                        pattern,
                        checked_queries
                            .get(equals_start)
                            .map(|checked_query| &checked_query.queried_type),
                        &queried_variable_name,
                        &mut |name, type_| {
                            local_variables.insert(name, type_);
                            destruct_local_variables_last_uses.insert(name, Vec::with_capacity(1));
                        },
                        type_aliases,
                    );
                    match result {
                        None => {
                            js_incomplete_statement();
                        }
                        Some(result) => {
                            let result = expressions.item(result);
                            syntax_expression_local_variables_last_uses(
                                result,
                                expressions,
                                &mut std::collections::HashSet::new(),
                                &mut destruct_local_variables_last_uses,
                            );
                            local_variables_last_uses.extend(destruct_local_variables_last_uses);
                            syntax_expression_to_js(
                                output,
                                project_fns,
                                type_aliases,
                                expressions,
                                checked_queries,
                                checked_calls,
                                local_variables,
                                local_variables_last_uses,
                                result,
                                scope_start,
                            );
                        }
                    }
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
            let queried_scope_start = expression_start(queried, expressions);
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
                project_fns,
                type_aliases,
                expressions,
                checked_queries,
                checked_calls,
                local_variables,
                local_variables_last_uses,
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
                        );
                        output.push_str(") {\n");
                    }
                    // can be optimized
                    let mut case_local_variables = local_variables.clone();
                    let mut case_local_variables_last_uses = std::collections::HashMap::new();
                    syntax_pattern_to_js_destructuring(
                        output,
                        &case.pattern,
                        Some(&checked_query.queried_type),
                        &queried_variable_name,
                        &mut |name, type_| {
                            case_local_variables.insert(name, type_);
                            case_local_variables_last_uses.insert(name, Vec::with_capacity(1));
                        },
                        type_aliases,
                    );
                    match &case.result {
                        None => {
                            js_incomplete_statement();
                        }
                        Some(case_result) => {
                            syntax_expression_local_variables_last_uses(
                                case_result,
                                expressions,
                                &mut std::collections::HashSet::new(),
                                &mut case_local_variables_last_uses,
                            );
                            case_local_variables_last_uses.extend(
                                local_variables_last_uses
                                    .iter()
                                    .map(|(name, uses)| (*name, uses.clone())),
                            );
                            syntax_expression_to_js(
                                output,
                                project_fns,
                                type_aliases,
                                expressions,
                                checked_queries,
                                checked_calls,
                                &mut case_local_variables,
                                &mut case_local_variables_last_uses,
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
fn reachable_type_buf_path_js(output: &mut String, path: &[BufReachPathSegment]) {
    for path_segment in path {
        match path_segment {
            BufReachPathSegment::InVariantValue {
                variant_name,
                value_index,
                value_count,
            } => {
                output.push('.');
                output.push_str(&name_to_lowercase_js(variant_name));
                if *value_count >= 2 {
                    use std::fmt::Write as _;
                    _ = write!(output, "[{value_index}]");
                }
            }
            BufReachPathSegment::InTuplePart(part_index) => {
                use std::fmt::Write as _;
                _ = write!(output, "[{part_index}]");
            }
            BufReachPathSegment::InRecordFieldValue(field_name) => {
                output.push('.');
                output.push_str(&name_to_lowercase_js(field_name));
            }
        }
    }
}
fn specialize_implicits_js(
    output: &mut String,
    implicit_bufs: &std::collections::HashSet<TypeBuf>,
    type_variables_to_substitute: &std::collections::BTreeMap<Name, Type>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    if type_variables_to_substitute.is_empty() {
        if !implicit_bufs.is_empty() {
            output.push('$');
        }
    } else {
        output.push('{');
        for implicit_buf in implicit_bufs {
            type_buf_js_variable_name(output, &implicit_buf.origin, &implicit_buf.item);
            output.push_str(": $.");
            let mut specialized_implicit_buf_origin = implicit_buf.origin.clone();
            type_replace_variables(
                type_variables_to_substitute,
                &mut specialized_implicit_buf_origin,
            );
            type_fully_resolve_aliases(
                &mut specialized_implicit_buf_origin,
                &mut Vec::new(),
                type_aliases,
            );
            let mut specialized_implicit_buf_item = implicit_buf.item.clone();
            type_replace_variables(
                type_variables_to_substitute,
                &mut specialized_implicit_buf_item,
            );
            type_fully_resolve_aliases(
                &mut specialized_implicit_buf_item,
                &mut Vec::new(),
                type_aliases,
            );
            type_buf_js_variable_name(
                output,
                &specialized_implicit_buf_origin,
                &specialized_implicit_buf_item,
            );
            output.push_str(", ");
        }
        for (type_variable, type_variable_replacement_type) in type_variables_to_substitute {
            output.push_str("drop$");
            output.push_str(type_variable);
            output.push_str(": ");
            js_drop_function(output, type_variable_replacement_type, type_aliases);
            output.push_str(", ");
            output.push_str("clone$");
            output.push_str(type_variable);
            output.push_str(": ");
            js_clone_function(output, type_variable_replacement_type, type_aliases);
            output.push_str(", ");
        }
        output.push('}');
    }
}
fn js_variable_drop(
    output: &mut String,
    variable_name: &str,
    type_: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match type_ {
        Type::Variable(type_variable) => {
            output.push_str("$.drop");
            name_to_uppercase_local_js(output, type_variable);
            output.push('(');
            output.push_str(variable_name);
            output.push_str(");\n");
        }
        Type::Tuple(parts) => {
            for (part_index, part) in parts.iter().enumerate() {
                js_variable_drop(
                    output,
                    &format!("{variable_name}[{part_index}]"),
                    part,
                    type_aliases,
                );
            }
        }
        Type::Record(fields) => {
            for TypeField { name, value } in fields {
                js_variable_drop(
                    output,
                    &format!("{variable_name}.{}", name_to_lowercase_js(name)),
                    value,
                    type_aliases,
                );
            }
        }
        Type::Choice(variants) => {
            for TypeVariant { name, values } in variants {
                output.push_str("if (\"");
                let variant_name_js = name_to_lowercase_js(name);
                output.push_str(&variant_name_js);
                output.push_str("\" in ");
                output.push_str(variable_name);
                output.push_str(") {\n");
                for (value_index, value) in values.iter().enumerate() {
                    js_variable_drop(
                        output,
                        &if values.len() >= 2 {
                            format!("{variable_name}.{variant_name_js}[{value_index}]")
                        } else {
                            format!("{variable_name}.{variant_name_js}")
                        },
                        value,
                        type_aliases,
                    );
                }
                output.push_str("} else ");
            }
            output.push_str("{}");
        }
        Type::CoreConstruct { name, arguments } => {
            if name == type_slot_name
                && let [origin, item] = arguments.as_slice()
            {
                let mut origin_dealiased = origin.clone();
                type_fully_resolve_aliases(&mut origin_dealiased, &mut Vec::new(), type_aliases);
                let mut item_dealiased = item.clone();
                type_fully_resolve_aliases(&mut item_dealiased, &mut Vec::new(), type_aliases);
                output.push_str("slot$drop($.");
                type_buf_js_variable_name(output, &origin_dealiased, &item_dealiased);
                output.push_str(", ");
                js_drop_function(output, item, type_aliases);
                output.push_str(", ");
                output.push_str(variable_name);
                output.push_str(");\n");
            } else if name == type_span_name
                && let [origin, item] = arguments.as_slice()
            {
                let mut origin_dealiased = origin.clone();
                type_fully_resolve_aliases(&mut origin_dealiased, &mut Vec::new(), type_aliases);
                let mut item_dealiased = item.clone();
                type_fully_resolve_aliases(&mut item_dealiased, &mut Vec::new(), type_aliases);
                output.push_str("span$drop($.");
                type_buf_js_variable_name(output, &origin_dealiased, &item_dealiased);
                output.push_str(", ");
                js_drop_function(output, item, type_aliases);
                output.push_str(", ");
                output.push_str(variable_name);
                output.push_str(");\n");
            } else {
                // gc
            }
        }
        Type::Fn { .. } => {}
        Type::Alias { name, arguments } => {
            let Some(type_alias_info) = type_aliases.get(name) else {
                return;
            };
            let Some(aliased_type) = &type_alias_info.type_ else {
                return;
            };
            output.push_str("drop$");
            output.push_str(&name_to_uppercase_js_segment(name));
            output.push('(');
            output.push_str(variable_name);
            output.push_str(", ");
            let mut implicit_bufs = std::collections::HashSet::new();
            {
                let mut aliased_type_bufs_to_ignore = std::collections::HashSet::new();
                type_reachable_bufs(
                    aliased_type,
                    vec![],
                    type_aliases,
                    &mut |newly_reachable, _| {
                        aliased_type_bufs_to_ignore.insert(newly_reachable);
                    },
                );
                type_implicit_bufs(
                    aliased_type,
                    &aliased_type_bufs_to_ignore,
                    type_aliases,
                    &mut implicit_bufs,
                );
            }
            specialize_implicits_js(
                output,
                &implicit_bufs,
                // can be optimized: avoid cloning
                &type_alias_info
                    .parameters
                    .iter()
                    .cloned()
                    .zip(arguments.iter().cloned())
                    .collect(),
                type_aliases,
            );
            output.push_str(");\n");
        }
    }
}
fn js_variable_clone(
    output: &mut String,
    variable_name: &str,
    type_: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match type_ {
        Type::Variable(_) => {
            output.push_str("$.clone");
            type_js_variable_name_segment(output, type_);
            output.push('(');
            output.push_str(variable_name);
            output.push_str(");\n");
        }
        Type::Tuple(parts) => {
            for (part_index, part) in parts.iter().enumerate() {
                js_variable_clone(
                    output,
                    &format!("{variable_name}[{part_index}]"),
                    part,
                    type_aliases,
                );
            }
        }
        Type::Record(fields) => {
            for TypeField { name, value } in fields {
                js_variable_clone(
                    output,
                    &format!("{variable_name}.{}", name_to_lowercase_js(name)),
                    value,
                    type_aliases,
                );
            }
        }
        Type::Choice(variants) => {
            for TypeVariant { name, values } in variants {
                output.push_str("if (\"");
                let variant_name_js = name_to_lowercase_js(name);
                output.push_str(&variant_name_js);
                output.push_str("\" in ");
                output.push_str(variable_name);
                output.push_str(") {\n");
                for (value_index, value) in values.iter().enumerate() {
                    js_variable_clone(
                        output,
                        &if values.len() >= 2 {
                            format!("{variable_name}.{variant_name_js}[{value_index}]")
                        } else {
                            format!("{variable_name}.{variant_name_js}")
                        },
                        value,
                        type_aliases,
                    );
                }
                output.push_str("} else ");
            }
            output.push_str("{}\n");
        }
        Type::CoreConstruct { name, arguments } => {
            if name == type_slot_name
                && let [origin, item] = arguments.as_slice()
            {
                let mut origin = origin.clone();
                type_fully_resolve_aliases(&mut origin, &mut Vec::new(), type_aliases);
                let mut item = item.clone();
                type_fully_resolve_aliases(&mut item, &mut Vec::new(), type_aliases);
                output.push_str("slot$clone($.");
                type_buf_js_variable_name(output, &origin, &item);
                output.push_str(", ");
                output.push_str(variable_name);
                output.push_str(");\n");
            } else if name == type_span_name
                && let [origin, item] = arguments.as_slice()
            {
                let mut origin = origin.clone();
                type_fully_resolve_aliases(&mut origin, &mut Vec::new(), type_aliases);
                let mut item = item.clone();
                type_fully_resolve_aliases(&mut item, &mut Vec::new(), type_aliases);
                output.push_str("span$clone($.");
                type_buf_js_variable_name(output, &origin, &item);
                output.push_str(", ");
                output.push_str(variable_name);
                output.push_str(");\n");
            } else {
                // gc
            }
        }
        Type::Fn { .. } => {}
        Type::Alias { name, arguments } => {
            let Some(type_alias_info) = type_aliases.get(name) else {
                return;
            };
            let Some(aliased_type) = &type_alias_info.type_ else {
                return;
            };
            output.push_str("clone$");
            output.push_str(&name_to_uppercase_js_segment(name));
            output.push('(');
            output.push_str(variable_name);
            output.push_str(", ");
            let mut implicit_bufs = std::collections::HashSet::new();
            {
                let mut aliased_type_bufs_to_ignore = std::collections::HashSet::new();
                type_reachable_bufs(
                    aliased_type,
                    vec![],
                    type_aliases,
                    &mut |newly_reachable, _| {
                        aliased_type_bufs_to_ignore.insert(newly_reachable);
                    },
                );
                type_implicit_bufs(
                    aliased_type,
                    &aliased_type_bufs_to_ignore,
                    type_aliases,
                    &mut implicit_bufs,
                );
            }
            specialize_implicits_js(
                output,
                &implicit_bufs,
                // can be optimized: avoid cloning
                &type_alias_info
                    .parameters
                    .iter()
                    .cloned()
                    .zip(arguments.iter().cloned())
                    .collect(),
                type_aliases,
            );
            output.push_str(");\n");
        }
    }
}
fn js_drop_function(
    output: &mut String,
    type_: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    output.push_str("function(it) {\n");
    js_variable_drop(output, "it", type_, type_aliases);
    output.push('}');
}
fn js_clone_function(
    output: &mut String,
    type_: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    output.push_str("function(it) {\n");
    js_variable_clone(output, "it", type_, type_aliases);
    output.push('}');
}
fn js_incomplete_statement() {
    // just don't set the result to anything
}
const empty_js_type_name: &str = "void";
const choice_empty_js_type_name: &str = "never";
fn name_to_uppercase_local_js(output: &mut String, name: &str) {
    output.push('$');
    output.push_str(&name_to_uppercase_js_segment(name));
}
pub fn name_to_uppercase_js_segment(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_uppercase();
    }
    sanitized
}
pub fn name_to_lowercase_js_variable_segment(name: &str) -> String {
    let mut sanitized: String = name.replace("-", "_");
    if let Some(first) = sanitized.get_mut(0..=0) {
        first.make_ascii_lowercase();
    }
    sanitized
}
pub fn name_to_lowercase_js(name: &str) -> String {
    let sanitized = name_to_lowercase_js_variable_segment(name);
    if js_lowercase_keywords.contains(&sanitized.as_str()) {
        sanitized + "ø"
    } else {
        sanitized
    }
}
fn name_to_lowercase_local_js(output: &mut String, name: &str) {
    let sanitized = name_to_lowercase_js_variable_segment(name);
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

fn type_buf_js_variable_name(output: &mut String, origin: &Type, item: &Type) {
    output.push_str("buf");
    type_js_variable_name_segment(output, origin);
    type_js_variable_name_segment(output, item);
}
fn type_js_variable_name_segment(output: &mut String, type_: &Type) {
    match type_ {
        Type::Variable(variable) => {
            output.push('$');
            output.push_str(&name_to_uppercase_js_segment(variable));
        }
        Type::Tuple(parts) => {
            output.push_str("$tuple");
            for part in parts {
                output.push_str("$_");
                type_js_variable_name_segment(output, part);
            }
            output.push_str("$end");
        }
        Type::Record(fields) => {
            output.push_str("$record");
            for field in fields {
                output.push_str("$_");
                output.push_str(&name_to_lowercase_js_variable_segment(&field.name));
                type_js_variable_name_segment(output, &field.value);
            }
            output.push_str("$end");
        }
        Type::Choice(variants) => {
            output.push_str("$choice");
            for variant in variants {
                output.push_str("$or");
                output.push_str(&name_to_lowercase_js_variable_segment(&variant.name));
                for variant_value in &variant.values {
                    output.push_str("$_");
                    type_js_variable_name_segment(output, variant_value);
                }
            }
            output.push_str("$end");
        }
        Type::CoreConstruct { name, arguments } => {
            output.push_str("$ty");
            output.push_str(&name_to_lowercase_js_variable_segment(name));
            for argument in arguments {
                output.push_str("$_");
                type_js_variable_name_segment(output, argument);
            }
            output.push_str("$end");
        }
        Type::Alias { name, arguments } => {
            // should only run for recursive type aliases at this point
            output.push_str("$ty");
            output.push_str(&name_to_lowercase_js_variable_segment(name));
            for argument in arguments {
                output.push_str("$_");
                type_js_variable_name_segment(output, argument);
            }
            output.push_str("$end");
        }
        Type::Fn { parameters, result } => {
            output.push_str("$fn");
            for parameter in parameters {
                output.push_str("$_");
                type_js_variable_name_segment(output, parameter);
            }
            type_js_variable_name_segment(output, result);
        }
    }
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
fn syntax_expression_local_variables_last_uses<'a>(
    expression: &'a SyntaxExpression,
    expressions: &'a Buf<Expressions, SyntaxExpression>,
    local_variables_used: &mut std::collections::HashSet<&'a Name>,
    local_variables_last_uses: &mut std::collections::HashMap<&Name, Vec<lsp_types::Position>>,
) {
    match expression {
        SyntaxExpression::Number { .. } => {}
        SyntaxExpression::Text { .. } => {}
        SyntaxExpression::Variable(name) => {
            if let Some(last_uses) = local_variables_last_uses.get_mut(&name.value) {
                let local_variable_was_unused = local_variables_used.insert(&name.value);
                if local_variable_was_unused {
                    last_uses.push(name.start);
                }
            }
        }
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments: _,
            open_paren_start: _,
            arguments_rest,
            closed_paren_start: _,
        } => {
            if let Some(last_uses) = local_variables_last_uses.get_mut(&name.value) {
                let local_variable_was_unused = local_variables_used.insert(&name.value);
                if local_variable_was_unused {
                    last_uses.push(name.start);
                }
            }
            for value in first_argument_dot
                .iter()
                .map(|first| expressions.item(&first.argument))
                .chain(arguments_rest)
                .rev()
            {
                syntax_expression_local_variables_last_uses(
                    value,
                    expressions,
                    local_variables_used,
                    local_variables_last_uses,
                );
            }
        }
        SyntaxExpression::Variant {
            name: _,
            type_: _,
            values,
        } => {
            for value in values.iter().flat_map(|values| &values.values).rev() {
                syntax_expression_local_variables_last_uses(
                    value,
                    expressions,
                    local_variables_used,
                    local_variables_last_uses,
                );
            }
        }
        SyntaxExpression::Fn { .. } => {}
        SyntaxExpression::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts.iter().rev() {
                syntax_expression_local_variables_last_uses(
                    part,
                    expressions,
                    local_variables_used,
                    local_variables_last_uses,
                );
            }
        }
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for SyntaxRecordField { name: _, value } in fields.iter().rev() {
                if let Some(value) = value {
                    syntax_expression_local_variables_last_uses(
                        value,
                        expressions,
                        local_variables_used,
                        local_variables_last_uses,
                    );
                }
            }
        }
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => {
            if let Some(expression) = expression {
                syntax_expression_local_variables_last_uses(
                    expressions.item(expression),
                    expressions,
                    local_variables_used,
                    local_variables_last_uses,
                );
            }
        }
        SyntaxExpression::Destruct {
            equals_start: _,
            pattern: _,
            queried,
            result,
        } => {
            if let Some(result) = result {
                syntax_expression_local_variables_last_uses(
                    expressions.item(result),
                    expressions,
                    local_variables_used,
                    local_variables_last_uses,
                );
            }
            if let Some(queried) = queried {
                syntax_expression_local_variables_last_uses(
                    expressions.item(queried),
                    expressions,
                    local_variables_used,
                    local_variables_last_uses,
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
            let mut local_variables_used_across_cases = std::collections::HashSet::new();
            for SyntaxExpressionIfCase { pattern: _, result } in cases {
                if let Some(result) = result {
                    let mut case_local_variables_used = std::collections::HashSet::new();
                    syntax_expression_local_variables_last_uses(
                        result,
                        expressions,
                        &mut case_local_variables_used,
                        local_variables_last_uses,
                    );
                    local_variables_used_across_cases.extend(case_local_variables_used);
                }
            }
            local_variables_used.retain(|local_variable_used| {
                !local_variables_used_across_cases.contains(local_variable_used)
            });
            if let Some(queried) = queried {
                syntax_expression_local_variables_last_uses(
                    expressions.item(queried),
                    expressions,
                    local_variables_used,
                    local_variables_last_uses,
                );
            }
        }
    }
}

#[derive(Clone, Debug)]
struct CheckedLocalVariable {
    origin_start: lsp_types::Position,
    type_: Option<Type>,
    used: bool,
}
#[derive(Debug)]
pub struct CheckedCall {
    // includes <explicit type arguments>
    // includes even type variables that get translated to a type variable with the same name
    type_variable_replacements: std::collections::BTreeMap<Name, Type>,
}
#[derive(Debug)]
pub struct CheckedLocalFn {
    pub parameter_types: Vec<Type>,
    pub result_type: Type,
}
#[derive(Debug)]
pub struct CheckedQuery {
    pub is_exhaustive: bool,
    pub queried_type: Type,
    pub invalid_case_indexes: Vec<usize>,
}
fn syntax_expression_check<'a>(
    errors: &mut Vec<ErrorNode>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    expressions: &'a Buf<Expressions, SyntaxExpression>,
    local_variables: &mut std::collections::HashMap<&'a Name, CheckedLocalVariable>,
    expression: &'a SyntaxExpression,
    checked_calls: &mut std::collections::HashMap<lsp_types::Position, CheckedCall>,
    checked_local_fns: &mut std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    checked_queries: &mut std::collections::HashMap<lsp_types::Position, CheckedQuery>,
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
                    message: Box::from("missing type after this number. Each number requires an explicit type to know its precision and range, like 0u32 or 0f32"),
                });
                None
            }
            Some(syntax_type) => {
                let Some(type_) = syntax_type_check_not_in_type_alias(
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
                    message: Box::from("missing type after this quoted text. To represent a single unicode scalar, add type char. To represent a string of characters, add type str. An example of a valid string is \"Diego Schissi\"str"),
                });
                return None;
            };
            let Some(type_) = syntax_type_check_not_in_type_alias(
                syntax_type,
                errors,
                type_aliases,
                records_used,
                choices_used,
            ) else {
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
            if let Some(variable_info) = local_variables.get_mut(&name.value) {
                variable_info.used = true;
                let Some(variable_type) = &variable_info.type_ else {
                    return None;
                };
                Some(variable_type.clone())
            } else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: format!(
                        "No local variable in scope has this name. Hints:
- a function name always starts uppercase
- a local function result can not refer to any variable from the outside
- a variable whose type contains type variables cannot be duplicated (because it might introduce multiple bufs with the same origin)
- check for typos

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
            let maybe_first_argument = first_argument_dot
                .as_ref()
                .map(|first_argument_dot| expressions.item(&first_argument_dot.argument));
            if let Some(project_fn_info) = project_fns.get(&name.value) {
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
                let Some(type_arguments) = syntax_type_arguments
                    .iter()
                    .flat_map(|args| &args.types)
                    .map(|syntax_type_argument| {
                        syntax_type_check_not_in_type_alias(
                            syntax_type_argument,
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
                let expected_argument_types = project_fn_info
                    .parameter_types
                    .iter()
                    .flatten()
                    .cloned()
                    .collect::<Vec<Type>>();
                let mut result_type = project_fn_result_type.clone();
                let mut type_variable_replacements = project_fn_info
                    .type_parameters
                    .iter()
                    .cloned()
                    .zip(type_arguments)
                    .collect();
                expression_call_check_for_mismathching_arguments(
                    errors,
                    with_start_position_as_ref(name),
                    maybe_first_argument.map(|_| 1).unwrap_or(0) + rest_syntax_arguments.len(),
                    &expected_argument_types,
                );
                for (mut expected_argument_type, syntax_argument) in
                    expected_argument_types.into_iter().zip(
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
                        local_variables,
                        syntax_argument,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
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
                        ); // is this really necessary?
                        type_collect_variables_that_are_concrete_into(
                            &mut type_variable_replacements,
                            &expected_argument_type,
                            &checked_argument_type,
                            type_aliases,
                        );
                    }
                    type_replace_variables(
                        &type_variable_replacements,
                        &mut expected_argument_type,
                    );
                    if let Some(argument_variable_input_type_diff) = type_diff(
                        &expected_argument_type,
                        &checked_argument_type,
                        type_aliases,
                    ) {
                        place_expression_type_diff_errors(
                            errors,
                            syntax_argument,
                            &argument_variable_input_type_diff,
                            expressions,
                        );
                        return None;
                    }
                }
                type_replace_variables(&type_variable_replacements, &mut result_type);
                checked_calls.insert(
                    name.start,
                    CheckedCall {
                        type_variable_replacements,
                    },
                );
                Some(result_type)
            } else if let Some(checked_local_variable) = local_variables.get_mut(&name.value) {
                checked_local_variable.used = true;
                let Some(called_variable_type) = &checked_local_variable.type_ else {
                    return None;
                };
                let Type::Fn {
                    parameters: expected_argument_types,
                    result: result_type,
                } = type_alias_resolve_to_non_alias(called_variable_type, type_aliases)
                    .into_owned()
                else {
                    let mut error_message =
                        "this variable is called like a function but its type is\n".to_string();
                    type_format(
                        &mut error_message,
                        0,
                        type_alias_resolve_to_non_alias(called_variable_type, type_aliases)
                            .as_ref(),
                    );
                    errors.push(ErrorNode {
                        range: name_range(with_start_position_as_ref(name)),
                        message: error_message.into_boxed_str(),
                    });
                    return None;
                };
                for (expected_argument_type, syntax_argument) in expected_argument_types.iter().zip(
                    maybe_first_argument
                        .into_iter()
                        .chain(rest_syntax_arguments),
                ) {
                    let Some(checked_argument_type) = syntax_expression_check(
                        errors,
                        type_aliases,
                        project_fns,
                        expressions,
                        local_variables,
                        syntax_argument,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        records_used,
                        choices_used,
                    ) else {
                        return None;
                    };
                    if let Some(argument_variable_input_type_diff) =
                        type_diff(expected_argument_type, &checked_argument_type, type_aliases)
                    {
                        place_expression_type_diff_errors(
                            errors,
                            syntax_argument,
                            &argument_variable_input_type_diff,
                            expressions,
                        );
                        return None;
                    }
                }
                expression_call_check_for_mismathching_arguments(
                    errors,
                    with_start_position_as_ref(name),
                    maybe_first_argument.map(|_| 1).unwrap_or(0) + rest_syntax_arguments.len(),
                    &expected_argument_types,
                );
                Some(*result_type)
            } else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from("unknown function name. No variable, fn in core or in this project exists with this name. Note that a local fn() expression can not refer to any variable from the outside. Otherwise check for typos.")
                });
                None
            }
        }
        SyntaxExpression::Variant {
            name,
            type_: other_variants_type,
            values,
        } => {
            let Some(syntax_type) = other_variants_type
                .as_ref()
                .and_then(|type_| type_.type_.as_ref())
            else {
                errors.push(ErrorNode {
                    range: name_range(with_start_position_as_ref(name)),
                    message: Box::from("missing type of the remaining variants after this variant name, in <angle brackets>.
Examples of valid variants are  Yes<[No]>(\"hi c:\"str)  and  Yes<opt(str)>(\"c:\"str).
If there should only ever by one variant, using a record with a single field is recommended over a single variant choice."),
                });
                return None;
            };
            let Some(checked_type) = syntax_type_check_not_in_type_alias(
                syntax_type,
                errors,
                type_aliases,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            let checked_type_dealiased =
                type_alias_resolve_to_non_alias(&checked_type, type_aliases);
            let Type::Choice(origin_choice_type) = checked_type_dealiased.as_ref() else {
                let mut error_message: String = String::from(
                    "this variant type should be a choice (for example [A(u32) B(str)]  or  opt(u32)) but it's\n",
                );
                type_format(&mut error_message, 0, checked_type_dealiased.as_ref());
                error_message.push_str("\nMaybe you forgot to wrap it in a type like opt()?");
                errors.push(ErrorNode {
                    range: type_range(syntax_type),
                    message: error_message.into_boxed_str(),
                });
                return None;
            };
            let Some(checked_value_types) = values
                .iter()
                .flat_map(|values| &values.values)
                .map(|value| {
                    syntax_expression_check(
                        errors,
                        type_aliases,
                        project_fns,
                        expressions,
                        local_variables,
                        value,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
                        records_used,
                        choices_used,
                    )
                })
                .collect()
            else {
                return None;
            };
            match origin_choice_type.iter().find_map(|variant| {
                if variant.name == name.value {
                    Some(&variant.values)
                } else {
                    None
                }
            }) {
                Some(expected_value_types) => {
                    for ((value, expected_value_type), checked_value_type) in values
                        .iter()
                        .flat_map(|values| &values.values)
                        .zip(expected_value_types)
                        .zip(&checked_value_types)
                    {
                        if let Some(variant_value_type_diff) =
                            type_diff(expected_value_type, checked_value_type, type_aliases)
                        {
                            place_expression_type_diff_errors(
                                errors,
                                value,
                                &variant_value_type_diff,
                                expressions,
                            );
                            return None;
                        }
                    }
                    Some(checked_type)
                }
                None => Some(Type::Choice(
                    origin_choice_type
                        .iter()
                        .cloned()
                        .chain(std::iter::once(TypeVariant {
                            name: name.value.clone(),
                            values: checked_value_types,
                        }))
                        .collect(),
                )),
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start,
            open_paren_start,
            parameters,
            closed_paren_start,
            result,
        } => {
            let mut parameter_introduced_variables: std::collections::HashMap<
                &Name,
                CheckedLocalVariable,
            > = std::collections::HashMap::new();
            let Some(parameter_types) = parameters
                .iter()
                .map(|parameter| {
                    syntax_parameter_pattern_check(
                        parameter,
                        errors,
                        &mut parameter_introduced_variables,
                        type_aliases,
                        &std::collections::HashMap::new(),
                        records_used,
                        choices_used,
                    )
                })
                .collect::<Option<Vec<Type>>>()
            else {
                return None;
            };
            let Some(result) = result else {
                errors.push(ErrorNode {
                    range: closed_paren_start
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
                &mut parameter_introduced_variables,
                expressions.item(result),
                checked_calls,
                checked_local_fns,
                checked_queries,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            for (parameter_introduced_variable_name, parameter_introduced_variable_origin) in
                parameter_introduced_variables
                    .iter()
                    .filter(|(_, var)| !var.used)
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
                        local_variables,
                        part,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
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
                        local_variables,
                        field_value,
                        checked_calls,
                        checked_local_fns,
                        checked_queries,
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
                local_variables,
                expressions.item(expression),
                checked_calls,
                checked_local_fns,
                checked_queries,
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
                local_variables,
                queried,
                checked_calls,
                checked_local_fns,
                checked_queries,
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
                    range: pattern_range(pattern),
                    message: Box::from("missing result expression after this destruct. An example of a destruct is = { some x field y } function(z)  result-expression")
                });
                return None;
            };
            let mut pattern_introduced_variables = std::collections::HashMap::new();
            let mut case_pattern_introduced_bufs = std::collections::HashSet::new();
            syntax_query_pattern_check(
                pattern,
                &checked_queried_type,
                errors,
                &mut pattern_introduced_variables,
                &mut case_pattern_introduced_bufs,
                local_variables,
                type_aliases,
            );
            let mut remaining_specific_pattern_catch_possibilities = Vec::new();
            type_to_possible_specific_pattern_catches(
                &checked_queried_type,
                type_aliases,
                &mut remaining_specific_pattern_catch_possibilities,
            );
            specific_pattern_catches_remove_those_matched_by_pattern(
                &mut remaining_specific_pattern_catch_possibilities,
                pattern,
            );
            local_variables.extend(
                pattern_introduced_variables
                    .iter()
                    .map(|(binding, info)| (*binding, info.clone())),
            );
            let result = expressions.item(result);
            let Some(checked_result_type) = syntax_expression_check(
                errors,
                type_aliases,
                project_fns,
                expressions,
                local_variables,
                result,
                checked_calls,
                checked_local_fns,
                checked_queries,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            for (pattern_introduced_variable, pattern_introduced_variable_origin) in
                pattern_introduced_variables
            {
                if let Some(CheckedLocalVariable { used: false, .. }) =
                    local_variables.remove(pattern_introduced_variable)
                {
                    errors.push(error_introduced_pattern_variable_is_unused(
                        pattern_introduced_variable_origin.origin_start,
                        pattern_introduced_variable,
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
If you want to handle cases in separate branches, use an if like  if option { Yes(n) n  No 0u32 }"
                        ).into_boxed_str(),
                });
            }
            let mut destruct_result_required_additional_implicit_bufs =
                std::collections::HashSet::new();
            type_implicit_bufs(
                &checked_result_type,
                &std::collections::HashSet::new(),
                type_aliases,
                &mut destruct_result_required_additional_implicit_bufs,
            );
            if case_pattern_introduced_bufs
                .iter()
                .any(|pattern_newly_reachable_buf| {
                    destruct_result_required_additional_implicit_bufs
                        .contains(pattern_newly_reachable_buf)
                })
            {
                let mut error_message =
                    "The result of this destruct needs to include these bufs:".to_string();
                for TypeBuf { origin, item } in
                    case_pattern_introduced_bufs
                        .into_iter()
                        .filter(|pattern_newly_reachable_buf| {
                            destruct_result_required_additional_implicit_bufs
                                .contains(pattern_newly_reachable_buf)
                        })
                {
                    error_message.push_str("\n  - ");
                    type_format(&mut error_message, 4, &type_buf(origin, item));
                }
                error_message.push_str("
Without these additional bufs, the resulting slots/spans would have no way to be passed to functions, dropped or cloned."
                );
                errors.push(ErrorNode {
                    range: expression_range(result, expressions),
                    message: error_message.into_boxed_str(),
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
                    message: Box::from("missing queried expression after this if keyword. An example of a if is if option { Yes(n) n  No 0u32 }")
                });
                return None;
            };
            let queried = expressions.item(queried);
            let Some((case0, case1_up)) = cases.split_first() else {
                errors.push(ErrorNode {
                    range: symbol_range(*if_keyword_start, "if"),
                    message: Box::from("missing case(s) after the queried expression. Cases look like  pattern result-expression. An example of a if is if option { Yes(n) n  No 0u32 }")
                });
                return None;
            };
            let Some(checked_queried_type) = syntax_expression_check(
                errors,
                type_aliases,
                project_fns,
                expressions,
                local_variables,
                queried,
                checked_calls,
                checked_local_fns,
                checked_queries,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            let Some(case0_result) = &case0.result else {
                errors.push(ErrorNode {
                    range: pattern_range(&case0.pattern),
                    message: Box::from("missing result expression after this if case pattern. Cases look like  pattern result-expression. An example of a if is if option { Yes(n) n  No 0u32 }")
                });
                return None;
            };
            let mut case0_pattern_introduced_variables = std::collections::HashMap::new();
            let mut case0_pattern_introduced_bufs = std::collections::HashSet::new();
            syntax_query_pattern_check(
                &case0.pattern,
                &checked_queried_type,
                errors,
                &mut case0_pattern_introduced_variables,
                &mut case0_pattern_introduced_bufs,
                local_variables,
                type_aliases,
            );
            let mut remaining_specific_pattern_catch_possibilities = Vec::new();
            type_to_possible_specific_pattern_catches(
                &checked_queried_type,
                type_aliases,
                &mut remaining_specific_pattern_catch_possibilities,
            );
            specific_pattern_catches_remove_those_matched_by_pattern(
                &mut remaining_specific_pattern_catch_possibilities,
                &case0.pattern,
            );

            local_variables.extend(
                case0_pattern_introduced_variables
                    .iter()
                    .map(|(binding, info)| (*binding, info.clone())),
            );
            let Some(checked_query_result_type) = syntax_expression_check(
                errors,
                type_aliases,
                project_fns,
                expressions,
                local_variables,
                case0_result,
                checked_calls,
                checked_local_fns,
                checked_queries,
                records_used,
                choices_used,
            ) else {
                return None;
            };
            for (case0_pattern_introduced_variable, case0_pattern_introduced_variable_origin) in
                case0_pattern_introduced_variables
            {
                if let Some(CheckedLocalVariable { used: false, .. }) =
                    local_variables.remove(case0_pattern_introduced_variable)
                {
                    errors.push(error_introduced_pattern_variable_is_unused(
                        case0_pattern_introduced_variable_origin.origin_start,
                        case0_pattern_introduced_variable,
                    ));
                }
            }
            expression_query_check_for_missing_result_bufs(
                errors,
                case0_pattern_introduced_bufs,
                case0_result,
                &checked_query_result_type,
                expressions,
                type_aliases,
            );
            let mut invalid_case_indexes = Vec::new();
            'checking_case1_up: for (case_index, case) in case1_up
                .iter()
                .enumerate()
                .map(|(i_in_1up, case)| (i_in_1up + 1, case))
            {
                let mut case_pattern_introduced_variables = std::collections::HashMap::new();
                let mut case_pattern_introduced_bufs = std::collections::HashSet::new();
                syntax_query_pattern_check(
                    &case.pattern,
                    &checked_queried_type,
                    errors,
                    &mut case_pattern_introduced_variables,
                    &mut case_pattern_introduced_bufs,
                    local_variables,
                    type_aliases,
                );
                let remaining_specific_pattern_catch_possibilities_count_before_case =
                    remaining_specific_pattern_catch_possibilities.len();
                specific_pattern_catches_remove_those_matched_by_pattern(
                    &mut remaining_specific_pattern_catch_possibilities,
                    &case.pattern,
                );
                if remaining_specific_pattern_catch_possibilities.len()
                    == remaining_specific_pattern_catch_possibilities_count_before_case
                {
                    errors.push(ErrorNode {
                        range: pattern_range(&case.pattern),
                        message: Box::from("this if case pattern only covers possibilities that earlier case patterns will already catch. This means that any code in this case result won't ever be called. Remove this case or correct the pattern.")
                    });
                    invalid_case_indexes.push(case_index);
                    continue 'checking_case1_up;
                }
                let Some(case_result) = &case.result else {
                    errors.push(ErrorNode {
                        range: pattern_range(&case.pattern),
                        message: Box::from("missing result expression after this if case pattern. Cases are written as  pattern result-expression. An example of a if is if option { Yes(n) n  No 0u32 }")
                    });
                    continue 'checking_case1_up;
                };
                local_variables.extend(
                    case_pattern_introduced_variables
                        .iter()
                        .map(|(binding, info)| (*binding, info.clone())),
                );
                let Some(checked_case_result_type) = syntax_expression_check(
                    errors,
                    type_aliases,
                    project_fns,
                    expressions,
                    local_variables,
                    case_result,
                    checked_calls,
                    checked_local_fns,
                    checked_queries,
                    records_used,
                    choices_used,
                ) else {
                    invalid_case_indexes.push(case_index);
                    continue 'checking_case1_up;
                };
                for (case_pattern_introduced_variable, case_pattern_introduced_variable_origin) in
                    case_pattern_introduced_variables
                {
                    if let Some(CheckedLocalVariable { used: false, .. }) =
                        local_variables.remove(case_pattern_introduced_variable)
                    {
                        errors.push(error_introduced_pattern_variable_is_unused(
                            case_pattern_introduced_variable_origin.origin_start,
                            case_pattern_introduced_variable,
                        ));
                    }
                }
                if let Some(match_result_case_result_type_diff) = type_diff(
                    &checked_query_result_type,
                    &checked_case_result_type,
                    type_aliases,
                ) {
                    place_expression_type_diff_errors(
                        errors,
                        case_result,
                        &match_result_case_result_type_diff,
                        expressions,
                    );
                    invalid_case_indexes.push(case_index);
                }
                expression_query_check_for_missing_result_bufs(
                    errors,
                    case_pattern_introduced_bufs,
                    case_result,
                    &checked_query_result_type,
                    expressions,
                    type_aliases,
                );
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
fn expression_call_check_for_mismathching_arguments(
    errors: &mut Vec<ErrorNode>,
    function_name: WithStartPosition<&Name>,
    argument_count: usize,
    parameter_types: &[Type],
) {
    match argument_count.cmp(&parameter_types.len()) {
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Greater => {
            errors.push(ErrorNode {
                range: name_range(function_name),
                message: format!(
                    "{} arguments too many (expected {}, found {argument_count}). Remove the extra ones",
                    argument_count - parameter_types.len(),
                    parameter_types.len()
                ).into_boxed_str()
            });
        }
        std::cmp::Ordering::Less => {
            let mut error_message = format!(
                "{} arguments missing (expected {}, found {argument_count}). Their expected parameter types are\n",
                parameter_types.len() - argument_count,
                parameter_types.len()
            );
            for parameter_type in parameter_types.iter().skip(argument_count) {
                error_message.push_str("\n  - ");
                type_format(&mut error_message, 4, parameter_type);
            }
            errors.push(ErrorNode {
                range: name_range(function_name),
                message: error_message.into_boxed_str(),
            });
        }
    }
}
fn expression_query_check_for_missing_result_bufs(
    errors: &mut Vec<ErrorNode>,
    pattern_introduced_bufs: std::collections::HashSet<TypeBuf>,
    result: &SyntaxExpression,
    result_type: &Type,
    expressions: &Buf<Expressions, SyntaxExpression>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    let mut result_required_additional_implicit_bufs = std::collections::HashSet::new();
    type_implicit_bufs(
        result_type,
        &std::collections::HashSet::new(),
        type_aliases,
        &mut result_required_additional_implicit_bufs,
    );
    if pattern_introduced_bufs
        .iter()
        .any(|pattern_newly_reachable_buf| {
            result_required_additional_implicit_bufs.contains(pattern_newly_reachable_buf)
        })
    {
        let mut error_message =
            "The result of this destruct needs to include these bufs:".to_string();
        for TypeBuf { origin, item } in
            pattern_introduced_bufs
                .into_iter()
                .filter(|pattern_newly_reachable_buf| {
                    result_required_additional_implicit_bufs.contains(pattern_newly_reachable_buf)
                })
        {
            error_message.push_str("\n  - ");
            type_format(&mut error_message, 4, &type_buf(origin, item));
        }
        error_message.push_str("
Without these additional bufs, the resulting slots/spans would have no way to be passed to functions, dropped or cloned."
        );
        errors.push(ErrorNode {
            // is there a nicer place to put it?
            range: expression_range(result, expressions),
            message: error_message.into_boxed_str(),
        });
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
fn syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
    errors: &mut Vec<ErrorNode>,
    expression: &SyntaxExpression,
    reachable_bufs: &mut std::collections::HashSet<TypeBuf>,
    checked_calls: &std::collections::HashMap<lsp_types::Position, CheckedCall>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    checked_local_fns: &std::collections::HashMap<lsp_types::Position, CheckedLocalFn>,
    expressions: &Buf<Expressions, SyntaxExpression>,
    project_fns: &std::collections::HashMap<Name, CheckedProjectFn>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match expression {
        SyntaxExpression::Number { .. } => {}
        SyntaxExpression::Text { .. } => {}
        SyntaxExpression::Variable(_) => {}
        SyntaxExpression::Call {
            first_argument_dot,
            name,
            type_arguments: _,
            open_paren_start: _,
            arguments_rest,
            closed_paren_start: _,
        } => {
            if let Some(project_fn) = project_fns.get(&name.value)
                && let Some(result_type) = &project_fn.result_type
                && let Some(checked_call) = checked_calls.get(&name.start)
            {
                let mut result_type = result_type.clone();
                type_replace_variables(&checked_call.type_variable_replacements, &mut result_type);
                type_check_introduced_bufs_do_not_overlap(
                    errors,
                    || expression_range(expression, expressions),
                    &result_type,
                    reachable_bufs,
                    type_aliases,
                );
            }
            for argument in first_argument_dot
                .iter()
                .map(|first| expressions.item(&first.argument))
                .chain(arguments_rest)
            {
                syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
                    errors,
                    argument,
                    reachable_bufs,
                    checked_calls,
                    checked_queries,
                    checked_local_fns,
                    expressions,
                    project_fns,
                    type_aliases,
                );
            }
        }
        SyntaxExpression::Variant {
            name: _,
            type_: _,
            values,
        } => {
            for value in values.iter().flat_map(|values| &values.values) {
                syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
                    errors,
                    value,
                    reachable_bufs,
                    checked_calls,
                    checked_queries,
                    checked_local_fns,
                    expressions,
                    project_fns,
                    type_aliases,
                );
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start,
            open_paren_start: _,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            if let Some(result) = result
                && let Some(checked_fn) = checked_local_fns.get(fn_keyword_start)
            {
                let mut fn_reachable_bufs = function_signature_implicit_bufs(
                    checked_fn.parameter_types.iter(),
                    Some(&checked_fn.result_type),
                    type_aliases,
                );
                for parameter in parameters {
                    parameter_pattern_add_introduced_bufs_and_check_do_not_overlap(
                        errors,
                        parameter,
                        &mut fn_reachable_bufs,
                        type_aliases,
                    );
                }
                syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
                    errors,
                    expressions.item(result),
                    &mut fn_reachable_bufs,
                    checked_calls,
                    checked_queries,
                    checked_local_fns,
                    expressions,
                    project_fns,
                    type_aliases,
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
                syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
                    errors,
                    part,
                    reachable_bufs,
                    checked_calls,
                    checked_queries,
                    checked_local_fns,
                    expressions,
                    project_fns,
                    type_aliases,
                );
            }
        }
        SyntaxExpression::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field_value in fields.iter().filter_map(|field| field.value.as_ref()) {
                syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
                    errors,
                    field_value,
                    reachable_bufs,
                    checked_calls,
                    checked_queries,
                    checked_local_fns,
                    expressions,
                    project_fns,
                    type_aliases,
                );
            }
        }
        SyntaxExpression::Commented {
            comments: _,
            expression,
        } => {
            if let Some(expression) = expression {
                syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
                    errors,
                    expressions.item(expression),
                    reachable_bufs,
                    checked_calls,
                    checked_queries,
                    checked_local_fns,
                    expressions,
                    project_fns,
                    type_aliases,
                );
            }
        }
        SyntaxExpression::Destruct {
            equals_start,
            pattern,
            queried,
            result,
        } => {
            if let Some(queried) = queried {
                syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
                    errors,
                    expressions.item(queried),
                    reachable_bufs,
                    checked_calls,
                    checked_queries,
                    checked_local_fns,
                    expressions,
                    project_fns,
                    type_aliases,
                );
            }
            if let Some(result) = result {
                if let Some(pattern) = pattern
                    && let Some(checked_query) = checked_queries.get(equals_start)
                {
                    query_pattern_add_introduced_bufs_and_check_do_not_overlap(
                        errors,
                        pattern,
                        &checked_query.queried_type,
                        reachable_bufs,
                        type_aliases,
                    );
                }
                syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
                    errors,
                    expressions.item(result),
                    reachable_bufs,
                    checked_calls,
                    checked_queries,
                    checked_local_fns,
                    expressions,
                    project_fns,
                    type_aliases,
                );
            }
        }
        SyntaxExpression::If {
            if_keyword_start,
            queried,
            open_curly_start: _,
            cases,
            closed_curly_start: _,
        } => {
            if let Some(queried) = queried {
                syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
                    errors,
                    expressions.item(queried),
                    reachable_bufs,
                    checked_calls,
                    checked_queries,
                    checked_local_fns,
                    expressions,
                    project_fns,
                    type_aliases,
                );
            }
            let checked_query = checked_queries.get(if_keyword_start);
            for case in cases {
                if let Some(case_result) = &case.result {
                    if let Some(checked_query) = checked_query {
                        query_pattern_add_introduced_bufs_and_check_do_not_overlap(
                            errors,
                            &case.pattern,
                            &checked_query.queried_type,
                            reachable_bufs,
                            type_aliases,
                        );
                    }
                    syntax_expression_check_introduced_bufs_do_not_overlap_with_reachable_bufs(
                        errors,
                        case_result,
                        reachable_bufs,
                        checked_calls,
                        checked_queries,
                        checked_local_fns,
                        expressions,
                        project_fns,
                        type_aliases,
                    );
                }
            }
        }
    }
}
fn parameter_pattern_add_introduced_bufs_and_check_do_not_overlap(
    errors: &mut Vec<ErrorNode>,
    pattern: &SyntaxPattern,
    reachable_bufs: &mut std::collections::HashSet<TypeBuf>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match pattern {
        SyntaxPattern::Ignored {
            underscore_start,
            type_,
        } => {
            if let Some(type_) = type_ {
                pattern_type_add_introduced_bufs_and_check_do_not_overlap(
                    errors,
                    || symbol_range(*underscore_start, "_"),
                    &syntax_type_to_type(type_),
                    reachable_bufs,
                    type_aliases,
                );
            }
        }
        SyntaxPattern::Variable { name, type_ } => {
            if let Some(type_) = type_ {
                pattern_type_add_introduced_bufs_and_check_do_not_overlap(
                    errors,
                    || name_range(with_start_position_as_ref(name)),
                    &syntax_type_to_type(type_),
                    reachable_bufs,
                    type_aliases,
                );
            }
        }
        SyntaxPattern::Variant { name: _, values } => {
            for value in values.iter().flat_map(|values| &values.values) {
                parameter_pattern_add_introduced_bufs_and_check_do_not_overlap(
                    errors,
                    value,
                    reachable_bufs,
                    type_aliases,
                );
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                parameter_pattern_add_introduced_bufs_and_check_do_not_overlap(
                    errors,
                    part,
                    reachable_bufs,
                    type_aliases,
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
                    parameter_pattern_add_introduced_bufs_and_check_do_not_overlap(
                        errors,
                        value,
                        reachable_bufs,
                        type_aliases,
                    );
                }
            }
        }
    }
}
fn query_pattern_add_introduced_bufs_and_check_do_not_overlap(
    errors: &mut Vec<ErrorNode>,
    pattern: &SyntaxPattern,
    expected_type: &Type,
    reachable_bufs: &mut std::collections::HashSet<TypeBuf>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match pattern {
        SyntaxPattern::Ignored {
            underscore_start,
            type_: _,
        } => {
            pattern_type_add_introduced_bufs_and_check_do_not_overlap(
                errors,
                || symbol_range(*underscore_start, "_"),
                expected_type,
                reachable_bufs,
                type_aliases,
            );
        }
        SyntaxPattern::Variable { name, type_: _ } => {
            pattern_type_add_introduced_bufs_and_check_do_not_overlap(
                errors,
                || name_range(with_start_position_as_ref(name)),
                expected_type,
                reachable_bufs,
                type_aliases,
            );
        }
        SyntaxPattern::Variant { name, values } => {
            let expected_type = type_alias_resolve_to_non_alias(expected_type, type_aliases);
            let Some(expected_value_types) = (match expected_type.as_ref() {
                Type::Choice(expected_variants) => expected_variants
                    .iter()
                    .find(|expected_variant| name.value == expected_variant.name)
                    .map(|variant| &variant.values),
                _ => None,
            }) else {
                return;
            };
            for (value_index, value) in values.iter().flat_map(|values| &values.values).enumerate()
            {
                if let Some(value_expected_type) = expected_value_types.get(value_index) {
                    query_pattern_add_introduced_bufs_and_check_do_not_overlap(
                        errors,
                        value,
                        value_expected_type,
                        reachable_bufs,
                        type_aliases,
                    );
                }
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            let expected_type = type_alias_resolve_to_non_alias(expected_type, type_aliases);
            let Type::Tuple(expected_part_types) = expected_type.as_ref() else {
                return;
            };
            for (part_index, part) in parts.iter().enumerate() {
                if let Some(part_expected_type) = expected_part_types.get(part_index) {
                    query_pattern_add_introduced_bufs_and_check_do_not_overlap(
                        errors,
                        part,
                        part_expected_type,
                        reachable_bufs,
                        type_aliases,
                    );
                }
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            let expected_type = type_alias_resolve_to_non_alias(expected_type, type_aliases);
            let Type::Record(expected_field_types) = expected_type.as_ref() else {
                return;
            };
            for field in fields {
                if let Some(value) = &field.value
                    && let Some(expected_field_type) = expected_field_types
                        .iter()
                        .find(|expected_field| field.name.value == expected_field.name)
                {
                    query_pattern_add_introduced_bufs_and_check_do_not_overlap(
                        errors,
                        value,
                        &expected_field_type.value,
                        reachable_bufs,
                        type_aliases,
                    );
                }
            }
        }
    }
}
fn pattern_type_add_introduced_bufs_and_check_do_not_overlap(
    errors: &mut Vec<ErrorNode>,
    error_range: impl Fn() -> lsp_types::Range,
    type_: &Type,
    reachable_bufs: &mut std::collections::HashSet<TypeBuf>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    type_reachable_bufs(type_, vec![], type_aliases, &mut |reachable_buf, _path| {
        let introduced_buf_was_not_reachable_before = reachable_bufs.insert(reachable_buf);
        if !introduced_buf_was_not_reachable_before {
            errors.push(ErrorNode {
                range: error_range(),
                message: introduced_reachable_buf_overlaps_error_message(),
            });
        }
    });
}
fn type_check_introduced_bufs_do_not_overlap(
    errors: &mut Vec<ErrorNode>,
    error_range: impl Fn() -> lsp_types::Range,
    type_: &Type,
    reachable_bufs: &std::collections::HashSet<TypeBuf>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    let mut type_inner_reachable_bufs = std::collections::HashSet::new();
    type_reachable_bufs(type_, vec![], type_aliases, &mut |reachable_buf, _path| {
        if reachable_bufs.contains(&reachable_buf) {
            errors.push(ErrorNode {
                range: error_range(),
                message: introduced_reachable_buf_overlaps_error_message(),
            });
        } else {
            let introduced_buf_was_not_reachable_before_inner =
                type_inner_reachable_bufs.insert(reachable_buf);
            if !introduced_buf_was_not_reachable_before_inner {
                errors.push(ErrorNode {
                    range: error_range(),
                    message: introduced_reachable_buf_overlaps_error_message(),
                });
            }
        }
    });
}
fn introduced_reachable_buf_overlaps_error_message() -> Box<str> {
    Box::from("A buf with the same origin and is already reachable here:
Having 2 possibly different bufs of the same type could allow accessing slots/spans from one buf at a different buf")
}

pub fn syntax_type_variables_into<'a>(
    type_variables: &mut std::collections::HashSet<&'a Name>,
    type_: &'a SyntaxType,
) {
    match type_ {
        SyntaxType::Variable(name) => {
            type_variables.insert(&name.value);
        }
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
                for value in variant.values.iter().flat_map(|values| &values.values) {
                    syntax_type_variables_into(type_variables, value);
                }
            }
        }
    }
}
pub fn syntax_pattern_type_variables_into<'a>(
    type_variables: &mut std::collections::HashSet<&'a Name>,
    pattern: &'a SyntaxPattern,
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
        SyntaxPattern::Variant { name: _, values } => {
            for value in values.iter().flat_map(|values| &values.values) {
                syntax_pattern_type_variables_into(type_variables, value);
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_pattern_type_variables_into(type_variables, part);
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_pattern_type_variables_into(type_variables, value);
                }
            }
        }
    }
}

/// Fully validated type
#[derive(Clone, Debug)]
enum TypeDiff {
    /// None means no difference
    Conflict {
        expected: Type,
        actual: Type,
    },
    // should maybe be spit up
    CoreConstructOrAlias {
        name: Name,
        arguments: Vec<Option<TypeDiff>>,
    },
    Tuple(Vec<Option<TypeDiff>>),
    Record {
        fields: Vec<TypeDiffField>,
        actual_missing_fields: Vec<Name>,
        actual_extraneous_fields: Vec<Name>,
    },
    Choice(Vec<TypeDiffVariant>),
    Fn {
        parameters: Vec<Option<TypeDiff>>,
        result: Option<Box<TypeDiff>>,
    },
}
#[derive(Clone, Debug)]
struct TypeDiffField {
    name: Name,
    value: Option<TypeDiff>,
}
#[derive(Clone, Debug)]
struct TypeDiffVariant {
    name: Name,
    values: Vec<Option<TypeDiff>>,
}

fn type_collect_variables_that_are_concrete_into(
    type_parameter_replacements: &mut std::collections::BTreeMap<Name, Type>,
    type_with_variables: &Type,
    concrete_type: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match type_with_variables {
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
            } = type_alias_resolve_to_non_alias(concrete_type, type_aliases).as_ref()
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
                        type_aliases,
                    );
                }
            }
        }
        Type::Fn { parameters, result } => {
            if let Type::Fn {
                parameters: concrete_parameters,
                result: concrete_result,
            } = type_alias_resolve_to_non_alias(concrete_type, type_aliases).as_ref()
            {
                for (argument_type, concrete_argument_type) in
                    parameters.iter().zip(concrete_parameters)
                {
                    type_collect_variables_that_are_concrete_into(
                        type_parameter_replacements,
                        argument_type,
                        concrete_argument_type,
                        type_aliases,
                    );
                }
                type_collect_variables_that_are_concrete_into(
                    type_parameter_replacements,
                    result,
                    concrete_result,
                    type_aliases,
                );
            }
        }
        Type::Tuple(parts) => {
            if let Type::Tuple(concrete_parts) =
                type_alias_resolve_to_non_alias(concrete_type, type_aliases).as_ref()
            {
                for (part, matching_concrete_part) in parts.iter().zip(concrete_parts) {
                    type_collect_variables_that_are_concrete_into(
                        type_parameter_replacements,
                        part,
                        matching_concrete_part,
                        type_aliases,
                    );
                }
            }
        }
        Type::Record(fields) => {
            if let Type::Record(concrete_fields) =
                type_alias_resolve_to_non_alias(concrete_type, type_aliases).as_ref()
            {
                for field in fields {
                    if let Some(matching_concrete_field) = concrete_fields
                        .iter()
                        .find(|concrete_field| concrete_field.name == field.name)
                    {
                        type_collect_variables_that_are_concrete_into(
                            type_parameter_replacements,
                            &field.value,
                            &matching_concrete_field.value,
                            type_aliases,
                        );
                    }
                }
            }
        }
        Type::Choice(variants) => {
            if let Type::Choice(concrete_variants) =
                type_alias_resolve_to_non_alias(concrete_type, type_aliases).as_ref()
            {
                for variant in variants {
                    if let Some(matching_concrete_variant) = concrete_variants
                        .iter()
                        .find(|concrete_variant| concrete_variant.name == variant.name)
                    {
                        for (variant_value, concrete_variant_value) in
                            variant.values.iter().zip(&matching_concrete_variant.values)
                        {
                            type_collect_variables_that_are_concrete_into(
                                type_parameter_replacements,
                                variant_value,
                                concrete_variant_value,
                                type_aliases,
                            );
                        }
                    }
                }
            }
        }
        Type::Alias { name, arguments } => match concrete_type {
            Type::Alias {
                name: concrete_name,
                arguments: concrete_arguments,
            } if name == concrete_name => {
                for (argument, concrete_argument) in arguments.iter().zip(concrete_arguments.iter())
                {
                    type_collect_variables_that_are_concrete_into(
                        type_parameter_replacements,
                        argument,
                        concrete_argument,
                        type_aliases,
                    );
                }
            }
            _ => {
                if let Some(origin_type_alias) = type_aliases.get(name)
                    && let Some(resolved_alias) = type_construct_resolve_type_alias(
                        origin_type_alias,
                        arguments.iter().cloned(),
                    )
                {
                    type_collect_variables_that_are_concrete_into(
                        type_parameter_replacements,
                        &resolved_alias,
                        concrete_type,
                        type_aliases,
                    );
                }
            }
        },
    }
}

/// None means the types are equal. Some(_) means there is a difference somewhere
fn type_diff(
    expected_type: &Type,
    actual_type: &Type,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) -> Option<TypeDiff> {
    match expected_type {
        Type::Variable(expected_variable) => {
            match type_alias_resolve_to_non_alias(actual_type, type_aliases).as_ref() {
                Type::Variable(actual_variable) if expected_variable == actual_variable => None,
                _ => Some(TypeDiff::Conflict {
                    expected: expected_type.clone(),
                    actual: actual_type.clone(),
                }),
            }
        }
        Type::CoreConstruct {
            name: expected_name,
            arguments: expected_arguments,
        } => match type_alias_resolve_to_non_alias(actual_type, type_aliases).as_ref() {
            Type::CoreConstruct {
                name: actual_choice_type_construct_name,
                arguments: actual_choice_type_construct_arguments,
            } if expected_name == actual_choice_type_construct_name => {
                if expected_arguments
                    .iter()
                    .zip(actual_choice_type_construct_arguments.iter())
                    .all(|(expected_argument, actual_argument)| {
                        type_diff(expected_argument, actual_argument, type_aliases).is_none()
                    })
                {
                    return None;
                }
                Some(TypeDiff::CoreConstructOrAlias {
                    name: expected_name.clone(),
                    arguments: expected_arguments
                        .iter()
                        .zip(actual_choice_type_construct_arguments.iter())
                        .map(|(expected_argument, actual_argument)| {
                            type_diff(expected_argument, actual_argument, type_aliases)
                        })
                        .collect(),
                })
            }
            _ => Some(TypeDiff::Conflict {
                expected: expected_type.clone(),
                actual: actual_type.clone(),
            }),
        },
        Type::Fn {
            parameters: expected_parameters,
            result: expected_result,
        } => match type_alias_resolve_to_non_alias(actual_type, type_aliases).as_ref() {
            Type::Fn {
                parameters: actual_parameters,
                result: actual_result,
            } => {
                let result_type_diff = type_diff(expected_result, actual_result, type_aliases);
                if result_type_diff.is_none()
                    && expected_parameters.iter().zip(actual_parameters).all(
                        |(expected, actual)| type_diff(expected, actual, type_aliases).is_none(),
                    )
                {
                    return None;
                }
                Some(TypeDiff::Fn {
                    parameters: expected_parameters
                        .iter()
                        .zip(actual_parameters.iter())
                        .map(|(expected, actual)| type_diff(expected, actual, type_aliases))
                        .collect(),
                    result: type_diff(expected_result, actual_result, type_aliases).map(Box::new),
                })
            }
            _ => Some(TypeDiff::Conflict {
                expected: expected_type.clone(),
                actual: actual_type.clone(),
            }),
        },
        Type::Tuple(expected_parts) => {
            match type_alias_resolve_to_non_alias(actual_type, type_aliases).as_ref() {
                Type::Tuple(actual_parts) if expected_parts.len() == actual_parts.len() => {
                    if expected_parts
                        .iter()
                        .zip(actual_parts)
                        .all(|(expected, actual)| {
                            type_diff(expected, actual, type_aliases).is_none()
                        })
                    {
                        None
                    } else {
                        Some(TypeDiff::Tuple(
                            expected_parts
                                .iter()
                                .zip(actual_parts)
                                .map(|(expected, actual)| type_diff(expected, actual, type_aliases))
                                .collect(),
                        ))
                    }
                }
                _ => Some(TypeDiff::Conflict {
                    expected: expected_type.clone(),
                    actual: actual_type.clone(),
                }),
            }
        }
        Type::Record(expected_fields) => {
            match type_alias_resolve_to_non_alias(actual_type, type_aliases).as_ref() {
                Type::Record(actual_fields) => {
                    // all these filter(find) operations are quadratic.
                    // should be fine since records usually do not contain many fields
                    if expected_fields.len() == actual_fields.len()
                        && expected_fields.iter().all(|expected_field| {
                            actual_fields
                                .iter()
                                .find(|actual_field| actual_field.name == expected_field.name)
                                .is_some_and(|actual_field| {
                                    type_diff(
                                        &expected_field.value,
                                        &actual_field.value,
                                        type_aliases,
                                    )
                                    .is_none()
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
                            .map(|expected_field| expected_field.name.clone())
                            .collect(),
                        actual_extraneous_fields: actual_fields
                            .iter()
                            .filter(|expected_field| {
                                !expected_fields
                                    .iter()
                                    .any(|actual_field| actual_field.name == expected_field.name)
                            })
                            .map(|actual_field| actual_field.name.clone())
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
                                name: expected_field.name.clone(),
                                value: type_diff(
                                    &expected_field.value,
                                    actual_field_value,
                                    type_aliases,
                                ),
                            })
                            .collect(),
                    })
                }
                _ => Some(TypeDiff::Conflict {
                    expected: expected_type.clone(),
                    actual: actual_type.clone(),
                }),
            }
        }
        Type::Choice(expected_variants) => {
            match type_alias_resolve_to_non_alias(actual_type, type_aliases).as_ref() {
                Type::Choice(actual_variants)
                    if expected_variants.len() == actual_variants.len()
                        && expected_variants.iter().all(|expected_variant| {
                            actual_variants
                                .iter()
                                .any(|actual_variant| actual_variant.name == expected_variant.name)
                        }) =>
                {
                    if expected_variants
                        .iter()
                        .filter_map(|expected_variant| {
                            actual_variants
                                .iter()
                                .find(|actual_variant| actual_variant.name == expected_variant.name)
                                .map(|actual_variant| {
                                    (&expected_variant.values, &actual_variant.values)
                                })
                        })
                        .all(|(expected_variant_values, actual_variant_values)| {
                            expected_variant_values
                                .iter()
                                .zip(actual_variant_values)
                                .all(|(expected_variant_value, actual_variant_value)| {
                                    type_diff(
                                        expected_variant_value,
                                        actual_variant_value,
                                        type_aliases,
                                    )
                                    .is_none()
                                })
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
                                    .find(|actual_variant| {
                                        actual_variant.name == expected_variant.name
                                    })
                                    .map(|actual_variant| {
                                        (expected_variant, &actual_variant.values)
                                    })
                            })
                            .map(
                                |(expected_variant, actual_variant_values)| TypeDiffVariant {
                                    name: expected_variant.name.clone(),
                                    values: expected_variant
                                        .values
                                        .iter()
                                        .zip(actual_variant_values)
                                        .map(|(expected_variant_value, actual_variant_value)| {
                                            type_diff(
                                                expected_variant_value,
                                                actual_variant_value,
                                                type_aliases,
                                            )
                                        })
                                        .collect(),
                                },
                            )
                            .collect(),
                    ))
                }
                _ => Some(TypeDiff::Conflict {
                    expected: expected_type.clone(),
                    actual: actual_type.clone(),
                }),
            }
        }
        Type::Alias {
            name: expected_name,
            arguments: expected_arguments,
        } => match actual_type {
            Type::Alias {
                name: actual_name,
                arguments: actual_arguments,
            } if expected_name == actual_name => {
                if expected_arguments.iter().zip(actual_arguments.iter()).all(
                    |(expected_argument, actual_argument)| {
                        type_diff(expected_argument, actual_argument, type_aliases).is_none()
                    },
                ) {
                    return None;
                }
                Some(TypeDiff::CoreConstructOrAlias {
                    name: expected_name.clone(),
                    arguments: expected_arguments
                        .iter()
                        .zip(actual_arguments.iter())
                        .map(|(expected_argument, actual_argument)| {
                            type_diff(expected_argument, actual_argument, type_aliases)
                        })
                        .collect(),
                })
            }
            _ => {
                if let Some(expected_origin_type_alias) = type_aliases.get(expected_name)
                    && let Some(expected_resolved_alias) = type_construct_resolve_type_alias(
                        expected_origin_type_alias,
                        expected_arguments.iter().cloned(),
                    )
                {
                    type_diff(&expected_resolved_alias, actual_type, type_aliases)
                } else {
                    Some(TypeDiff::Conflict {
                        expected: expected_type.clone(),
                        actual: actual_type.clone(),
                    })
                }
            }
        },
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
        TypeDiff::CoreConstructOrAlias { name, arguments } => {
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
    formatted.push_str(&type_diff_field.name);
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
    formatted.push_str(&type_diff_variant.name);
    if let Some((value0, value1_up)) = type_diff_variant.values.split_first() {
        formatted.push('(');
        let line_span = type_diff_length_estimate_line_span(type_diff_variant_length_estimate(
            type_diff_variant,
        ));
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, next_indent(indent));
        }
        option_type_diff_format(formatted, next_indent(indent), value0.as_ref());
        for value in value1_up {
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            option_type_diff_format(formatted, next_indent(indent), value.as_ref());
        }
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, indent);
        }
        formatted.push(')');
    }
}
/// this is set to a low-seeming number because hover windows and similar
/// are often small and sometimes unresizable
const type_info_line_length_estimate_maximum: usize = 50;
fn type_diff_line_span(type_diff: &TypeDiff) -> LineSpan {
    type_diff_length_estimate_line_span(type_diff_length_estimate(type_diff))
}
fn type_diff_length_estimate_line_span(type_diff_length_estimate: usize) -> LineSpan {
    if type_diff_length_estimate <= type_info_line_length_estimate_maximum {
        LineSpan::Single
    } else {
        LineSpan::Multiple
    }
}
fn option_type_diff_line_span(type_diff: Option<&TypeDiff>) -> LineSpan {
    type_diff_length_estimate_line_span(option_type_diff_length_estimate(type_diff))
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
        TypeDiff::CoreConstructOrAlias { name, arguments } => {
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
            .map(|variant| type_diff_variant_length_estimate(variant))
            .sum(),
    }
}
fn type_diff_variant_length_estimate(type_diff_variant: &TypeDiffVariant) -> usize {
    type_diff_variant.name.len()
        + type_diff_variant
            .values
            .iter()
            .map(|value| 1 + option_type_diff_length_estimate(value.as_ref()))
            .sum::<usize>()
}

fn place_expression_type_diff_errors(
    errors: &mut Vec<ErrorNode>,
    expression: &SyntaxExpression,
    type_diff: &TypeDiff,
    expressions: &Buf<Expressions, SyntaxExpression>,
) {
    match expression {
        SyntaxExpression::Number { .. } => {
            errors.push(ErrorNode {
                range: expression_range(expression, expressions),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxExpression::Text { .. } => {
            errors.push(ErrorNode {
                range: expression_range(expression, expressions),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxExpression::Variable(_) => {
            errors.push(ErrorNode {
                range: expression_range(expression, expressions),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxExpression::Call { .. } => {
            errors.push(ErrorNode {
                range: expression_range(expression, expressions),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxExpression::Variant {
            name,
            type_: _,
            values,
        } => {
            let TypeDiff::Choice(variant_type_diffs) = type_diff else {
                errors.push(ErrorNode {
                    range: expression_range(expression, expressions),
                    message: type_diff_error_message(type_diff).into_boxed_str(),
                });
                return;
            };
            match variant_type_diffs
                .iter()
                .find(|variant_type_diff| variant_type_diff.name == name.value)
            {
                Some(variant_type_diff) => {
                    for (value, value_type_diff) in values
                        .iter()
                        .flat_map(|values| &values.values)
                        .zip(&variant_type_diff.values)
                    {
                        if let Some(variant_value_type_diff) = value_type_diff {
                            place_expression_type_diff_errors(
                                errors,
                                value,
                                variant_value_type_diff,
                                expressions,
                            );
                        }
                    }
                }
                None => {
                    errors.push(ErrorNode {
                        range: expression_range(expression, expressions),
                        message: type_diff_error_message(type_diff).into_boxed_str(),
                    });
                }
            }
        }
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
                        place_pattern_type_diff_errors(errors, parameter, type_diff_parameter);
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
                    );
                }
            }
            _ => {
                errors.push(ErrorNode {
                    range: expression_range(expression, expressions),
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
                    range: expression_range(expression, expressions),
                    message: type_diff_error_message(type_diff).into_boxed_str(),
                });
                return;
            };
            for (part, type_diff_part) in parts.iter().zip(type_diff_parts) {
                if let Some(type_diff_part) = type_diff_part {
                    place_expression_type_diff_errors(errors, part, type_diff_part, expressions);
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
                    range: expression_range(expression, expressions),
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
                place_expression_type_diff_errors(errors, case_result, type_diff, expressions);
            }
        }
    }
}
fn place_pattern_type_diff_errors(
    errors: &mut Vec<ErrorNode>,
    pattern: &SyntaxPattern,
    type_diff: &TypeDiff,
) {
    match pattern {
        SyntaxPattern::Variable { .. } => {
            errors.push(ErrorNode {
                range: pattern_range(pattern),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxPattern::Ignored { .. } => {
            errors.push(ErrorNode {
                range: pattern_range(pattern),
                message: type_diff_error_message(type_diff).into_boxed_str(),
            });
        }
        SyntaxPattern::Variant { name, values } => match type_diff {
            TypeDiff::Choice(variant_type_diffs) => {
                match variant_type_diffs
                    .iter()
                    .find(|variant_type_diff| variant_type_diff.name == name.value)
                {
                    Some(variant_type_diff) => {
                        for (value, value_type_diff) in values
                            .iter()
                            .flat_map(|values| &values.values)
                            .zip(&variant_type_diff.values)
                        {
                            if let Some(variant_value_type_diff) = value_type_diff {
                                place_pattern_type_diff_errors(
                                    errors,
                                    value,
                                    variant_value_type_diff,
                                );
                            }
                        }
                    }
                    None => {
                        errors.push(ErrorNode {
                            range: pattern_range(pattern),
                            message: type_diff_error_message(type_diff).into_boxed_str(),
                        });
                    }
                }
            }
            _ => {
                errors.push(ErrorNode {
                    range: pattern_range(pattern),
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
                    range: pattern_range(pattern),
                    message: type_diff_error_message(type_diff).into_boxed_str(),
                });
                return;
            };
            for (part, type_diff_part) in parts.iter().zip(type_diff_parts) {
                if let Some(type_diff_part) = type_diff_part {
                    place_pattern_type_diff_errors(errors, part, type_diff_part);
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
                            place_pattern_type_diff_errors(errors, value, field_value_type_diff);
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
                    range: pattern_range(pattern),
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
    Type::Variable(Name::from_static(name))
}
fn type_tuple(fields: impl IntoIterator<Item = Type>) -> Type {
    Type::Tuple(fields.into_iter().collect())
}
/// preferred type to hold no information
const type_empty: Type = type_tuple_empty;
const type_tuple_empty: Type = Type::Tuple(vec![]);
const type_choice_empty: Type = Type::Choice(vec![]);
fn type_choice(variants: impl IntoIterator<Item = (&'static str, Vec<Type>)>) -> Type {
    Type::Choice(
        variants
            .into_iter()
            .map(|(variant_name, variant_value)| TypeVariant {
                name: Name::from_static(variant_name),
                values: variant_value,
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
    Type::Alias {
        name: Name::from_static("opt"),
        arguments: vec![yes],
    }
}
const type_order: Type = Type::Alias {
    name: Name::from_static("order"),
    arguments: vec![],
};
fn type_fn(in_: Vec<Type>, out: Type) -> Type {
    Type::Fn {
        parameters: in_,
        result: Box::new(out),
    }
}
const type_buf_name: &str = "buf";
fn type_buf(origin: Type, item: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static(type_buf_name),
        arguments: vec![origin, item],
    }
}
const type_slot_name: &str = "slot";
fn type_slot(origin: Type, item: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static(type_slot_name),
        arguments: vec![origin, item],
    }
}
const type_span_name: &str = "span";
fn type_span(origin: Type, item: Type) -> Type {
    Type::CoreConstruct {
        name: Name::from_static(type_span_name),
        arguments: vec![origin, item],
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
                result: type_order,
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
                result: type_order,
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
                result: type_order,
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
However, it's slower than `f32-round-nearest-else-even` on most architectures.
```slug
fn age() f32
    f32-round-nearest-else-away-from-0(68.8f32)
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
An argument could be made that this is more "fair" than `f32-round-nearest-else-away-from-0`
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
                documentation: "`f32-round-up`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-down-to-i32-clamp",
                documentation: "`f32-round-down`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-toward-0-to-i32-clamp",
                documentation: "`f32-round-toward-0`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-away-from-0-to-i32-clamp",
                documentation: "`f32-round-away-from-0`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-nearest-else-away-from-0-to-i32-clamp",
                documentation: "`f32-round-nearest-else-away-from-0`, then clamp to within 32 bits.
In effect, `f32-round-toward-0-to-i32-clamp` truncates off at the decimal point:
```slug
fn age() f32
    f32-round-toward-0-to-i32-clamp(68.8f32)
```",
                type_parameters: vec![],
                parameters: vec![type_f32] ,
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-round-nearest-else-even-to-i32-clamp",
                documentation: "`f32-round-nearest-else-even`, then clamp to within 32 bits",
                type_parameters: vec![],
                parameters: vec![type_f32],
                result: type_i32,
            },
            CoreFnInfo {
                name: "f32-order",
                documentation: "Compare left to right. For example 'less means left < right",
                type_parameters: vec![],
                parameters: vec![type_f32, type_f32],
                result: type_order,
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
You usually want to use the slower strr-char-count.",
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
                result: type_tuple([type_char, type_opt(type_str)]),
            },
            CoreFnInfo {
                name: "str-end",
                documentation: "Split off its last char",
                type_parameters: vec![],
                parameters: vec![type_str],
                result: type_tuple([type_opt(type_str), type_char]),
            },
            CoreFnInfo {
                name: "choice-empty-to",
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
fn tried-unwrap(tried [Success(Ok) Failure([])]) Ok
    if tried {
    Success(ok) ok
    Failure(impossible) choice-empty-to<Ok>(impossible)
    }
```
Another nice use case: Creating a value whose only purpose is transporting type info:
```slug
ty type-only(Type)
# enables what is often called "phantom types"
    [A Unconstructible(Type [])]

# usage example

ty weak-slot(Origin)
    # Alternative to using a plain u32 which is tiny bit harder to abuse.
    # Can still point out of bounds,
    # alias another weak-slot or be used for the wrong span.
    # As a result, access will always return opt and should be checked with care
    {origin type-only(Origin) index u32}

fn index-to-weak-slot<Origin>(index u32) weak-slot(Origin)
    {index index origin A<type-only(Origin)>}

fn span-start-weak(span span(Origin)) weak-slot(Origin)
    = index span-start-index(span)
    index-to-weak-slot<Origin>(index)
```
This `type-only` pattern is very easy to overuse,
please limit your creativity for the rest of us mortals."#,
                type_parameters: vec![Name::from_static("Result")],
                parameters: vec![type_choice_empty],
                result: type_variable("Result")
            },
            CoreFnInfo {
                name: "slot-index",
                documentation: "At what offset the referenced item is located.
For example, the second item in a buf is at index 1",
                type_parameters: vec![],
                parameters: vec![type_slot(type_variable("Origin"), type_variable("Item"))],
                result: type_u32
            },
            CoreFnInfo {
                name: "slot-to-span",
                documentation: "Create a span covering just the one given slot",
                type_parameters: vec![],
                parameters: vec![type_slot(type_variable("Origin"), type_variable("Item"))],
                result: type_span(type_variable("Origin"), type_variable("Item")),
            },
            CoreFnInfo {
                name: "span-start-index",
                documentation: "At what offset the first contained slot is located.
For example, the second item in a buf is at index 1",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"), type_variable("Item"))],
                result: type_u32,
            },
            CoreFnInfo {
                name: "span-length",
                documentation: "How many slots it spans",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"), type_variable("Item"))],
                result: type_p32
            },
            CoreFnInfo {
                name: "opt-span-length",
                documentation: "How many slots it spans",
                type_parameters: vec![],
                parameters: vec![type_opt(type_span(type_variable("Origin"), type_variable("Item")))],
                result: type_u32
            },
            CoreFnInfo {
                name: "span-start",
                documentation: "Split into the first slot and span after.
To join disconnected slots and spans back together, use helpers like `span-add-own-opt-span`.
Note that to only read the start index you can directly call `span-start-index`",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"), type_variable("Item"))],
                result: type_tuple([
                    type_slot(type_variable("Origin"), type_variable("Item")),
                    type_opt(type_span(type_variable("Origin"), type_variable("Item")))
                ]),
            },
            CoreFnInfo {
                name: "span-end",
                documentation: "Split into the last slot and span before",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"), type_variable("Item"))],
                result: type_tuple([
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                    type_slot(type_variable("Origin"), type_variable("Item")),
                ]),
            },
            CoreFnInfo {
                name: "span-start-of-length-positive",
                documentation: "Split after a given length with the start half known to have positive length.
If the length is greater than the given span's length, .start will be the existing span and .after will be empty.
```slug
fn span-slot-at(span span(Origin) index u32)
    {
    before opt(span(Origin))
    at slot(Origin)
    after opt(span(Origin))
    }
    = *(until after)
        span-start-of-length-positive(span 1p32.p32-add(index))
    = *(before at) span-end(until)
    {before before at at after after}
```
See also `span-end-of-length-positive`, `span-start`.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_variable("Item")),
                    type_p32,
                ],
                result: type_tuple([
                    type_span(type_variable("Origin"), type_variable("Item")),
                    type_opt(type_span(type_variable("Origin"), type_variable("Item")))
                ]),
            },
            CoreFnInfo {
                name: "span-end-of-length-positive",
                documentation: "Split before a given length from the end with the end half known to have positive length.
If the length is greater than the given span's length, .end will be the existing span and .before will be empty.
See also `span-start-of-length-positive`, `span-end`.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_variable("Item")),
                    type_p32,
                ],
                result: type_tuple([
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                    type_span(type_variable("Origin"), type_variable("Item")),
                ]),
            },
            CoreFnInfo {
                name: "span-step",
                documentation: "Step through all slots, updating the given initial state for each taken slot in line.
A less powerful version that preserves the span is `span-step`",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_variable("Item")),
                    type_choice([("Up", vec![]), ("Down", vec![])]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_slot(type_variable("Origin"), type_variable("Item")),
                        ],
                        type_variable("State")
                    )
                ],
                result: type_variable("State"),
            },
            CoreFnInfo {
                name: "opt-span-step",
                documentation: "Step through all slots, updating the given initial state for each taken slot in line.
A less powerful version that preserves the span is `opt-span-step`",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                    type_choice([("Up", vec![]), ("Down", vec![])]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_slot(type_variable("Origin"), type_variable("Item")),
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
                    type_span(type_variable("Origin"), type_variable("Item")),
                    type_choice([("Up", vec![]), ("Down", vec![])]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_slot(type_variable("Origin"), type_variable("Item")),
                        ],
                        type_choice([("Done", vec![type_variable("Done")]), ("Going", vec![type_variable("State")])])
                    )
                ],
                result: type_choice([
                    (
                        "Done",
                        vec![type_tuple([
                            type_variable("Done"),
                            type_opt(type_span(type_variable("Origin"), type_variable("Item")))
                        ])]
                    ),
                    ("Going", vec![type_variable("State")])
                ]),
            },
            CoreFnInfo {
                name: "opt-span-step-while",
                documentation: "Step through all slots, updating the given initial state for each taken slot in line
by returning `'going` or exiting early with `'done` (like calling `break` in other languages).
```slug
fn next-non-space(span opt(span(Origin)))
    {
    non-space opt(char)
    after opt(span(Origin))
    }
    if
        opt-span-step-while(
            span
            Up<[Down]>
            *
            fn(slot slot(Origin) *)
            = char item(chars slot)
            if u32-order(char-to-u32(char) char-to-u32(\" \"char)) {
            Equal Going<[Done char]>(*)
            Less Done<[Going(*)]>(char)
            Greater Done<[Going(*)]>(char)
            }
        )
    {
    Going(chars)
        {non-space No<opt(char)> after No<opt(span(Origin))>}
    Done(non-space span-after)
        {non-space Yes<[No]>(non-space) after span-after}
    }
```
Note that `span-after` does not include any slot given to the step function, even the slot that resulted in `[done]`.
(This example looks convoluted. If you introduce helpers like char-equal it gets more resonable)",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                    type_choice([("Up", vec![]), ("Down", vec![])]),
                    type_variable("State"),
                    type_fn(
                        vec![
                            type_variable("State"),
                            type_slot(type_variable("Origin"), type_variable("Item")),
                        ],
                        type_choice([("Done", vec![type_variable("Done")]), ("Going", vec![type_variable("State")])])
                    )
                ],
                result: type_choice([
                    (
                        "Done",
                        vec![type_tuple([
                            type_variable("Done"),
                            type_opt(type_span(type_variable("Origin"), type_variable("Item")))
                        ])]
                    ),
                    ("Going", vec![type_variable("State")])
                ]),
            },
            CoreFnInfo {
                name: "buf-empty",
                documentation: "Create an empty buf.
To modify it, use `buf-pre-allocate-at-least`, `add`, `insert` etc.",
                type_parameters: vec![Name::from_static("Origin"), Name::from_static("Item")],
                parameters: vec![],
                result: type_buf(type_variable("Origin"), type_variable("Item")),
            },
            CoreFnInfo {
                name: "buf-reuse",
                documentation: "Initialize a `buf` with 0 items and spare allocated memory from an `Unset-slice`.
This can be used to recycle `buf` memory from one `buf` with one origin into another `buf` with a different origin.
```slug
fn buf-recycle<New-origin>(old buf(Old-origin Item))
    buf(New-origin Item)
    buf-reuse<New-origin>(buf-to-unset(old))
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
                parameters: vec![type_buf(type_variable("Origin"), type_variable("Item")), type_u32],
                result: type_empty,
            },
            CoreFnInfo {
                name: "buf-try-pre-allocate-at-least",
                documentation: "Reserves spare capacity for at least a given length more items to be added
and if not possible returns `Out-of-memory` (if the output language target can detect this).
A version which only checks for errors behind the scenes is `buf-pre-allocate-at-least`",
                type_parameters: vec![],
                parameters: vec![type_buf(type_variable("Origin"), type_variable("Item")), type_u32],
                result: type_choice([
                    ("Out-of-memory", vec![]),
                    ("Success", vec![])
                ]),
            },
            CoreFnInfo {
                name: "buf-pre-allocation-rid",
                documentation: "Shrinks down spare capacity as much as possible.
Some allocators may scrap and re-allocate the whole `buf` as a result.
It's rarely useful but can reuce idle memory usage for `buf`s that are very unlikely to be added to in the future.
You may also use it to adjust memory usage after `buf-reuse` when the given `Unset-slice` was large",
                type_parameters: vec![],
                parameters: vec![type_buf(type_variable("Origin"), type_variable("Item"))],
                result: type_empty,
            },
            CoreFnInfo {
                name: "insert",
                documentation: "Add a new item into the `buf` and keep a slot to it,
reusing unset space closest to the start of the `buf` when available.
Use `add` if you don't care about reuse.
Use `set-or-insert` if you already know of an index that's likely unset",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![type_variable("Item")],
                result: type_slot(type_variable("Origin"), type_variable("Item")),
            },
            CoreFnInfo {
                name: "set-or-insert",
                documentation: "Add a new item into the `buf`
a a specific index (starting at 0 for the first item) and keep a slot to it.
If the given index is already occupied, chooses an unset index closest to the start of the `buf` when available.

You can use this to more cheaply return temporarily removed items to their original spots.
`span-start-index` is also often useful here.
Note that `item-step`, `swap` etc. will be enough to fulfill most of the same purposes without indexes flying about.

Use `add` if you don't care about reuse.",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![
                    type_u32,
                    type_variable("Item"),
                ],
                result: type_slot(type_variable("Origin"), type_variable("Item")),
            },
            CoreFnInfo {
                name: "add",
                documentation: "Add a new item to the end of the `buf` and keep a slot to it without trying to reuse unset slots.
Can be faster than `insert` when you expect no unset space or when all the storage gets scrapped soon anyway.",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![type_variable("Item")],
                result: type_slot(type_variable("Origin"), type_variable("Item"))
            },
            CoreFnInfo {
                name: "add-str-chars",
                documentation: "Attach the chars in a given `str` to the end of the `buf` and keep a span to it without trying to reuse unset slots.
Convenient equivalent to `opt-span-add-str-chars` with an empty span.",
                type_parameters: vec![Name::from_static("Origin")],
                parameters: vec![type_str],
                result: type_span(type_variable("Origin"), type_char),
            },
            CoreFnInfo {
                name: "item",
                documentation: "Retrieve an item from the `buf` at a given slot (the inverse of `insert`/`add`)",
                type_parameters: vec![],
                parameters: vec![type_slot(type_variable("Origin"), type_variable("Item"))],
                result: type_variable("Item"),
            },
            CoreFnInfo {
                name: "swap",
                documentation: "Replace the item at the first given slot with the item at the second given slot and the other way around.
So after this operation, the first slot will point to the item originally behind the second slot and the other way around.

This may not seem very useful considering that you could also just switch out only the slots themselves.
An example use case is removing a specific index in a span
where you don't care about order among the items:
You can take the last slot and swap its item with the item at the specific index,
then remove the last slot with the item from the original certain index.
(Directly removing the last item and replacing the item at the specific index with the last item also works)",
                type_parameters: vec![],
                parameters: vec![
                    type_slot(type_variable("Origin"), type_variable("Item")),
                    type_slot(type_variable("Origin"), type_variable("Item")),
                ],
                result: type_empty
            },
            CoreFnInfo {
                name: "replace",
                documentation: "Switch out the item from the `buf` at a given slot and return the original item.
A more versatile version is `item-step`.",
                type_parameters: vec![],
                parameters: vec![
                    type_slot(type_variable("Origin"), type_variable("Item")),
                    type_variable("Item")
                ],
                result: type_variable("Item"),
            },
            CoreFnInfo {
                name: "item-step",
                documentation: "Look at an item from the `buf` at a given slot, change it and return something out.
It behaves like `item` followed by `insert`/`replace` but is more performant because it always operates on the same slot.
It's common to create simple helpers using `item-step`, for example
To just switch out an item, use `replace`",
                type_parameters: vec![],
                parameters: vec![
                    type_slot(type_variable("Origin"), type_variable("Item")),
                    type_variable("In"),
                    type_fn(
                        vec![type_variable("In"), type_variable("Item")],
                        type_tuple([type_variable("Out"), type_variable("Item")])
                    )
                ],
                result: type_variable("Out")
            },
            CoreFnInfo {
                name: "span-reverse",
                documentation: "Order the referenced items such that the previously last is now first, second last is second etc.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_variable("Item")),
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "opt-span-reverse",
                documentation: "Order the referenced items such that the previously last is now first, second last is second etc.",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "span-sort",
                documentation: r#"Order the referenced items, less before greater according to a given function.
Items whose comparison returns `'equal .` are ordered according to their original position in the span ("stable sort").
While this function allows the compared items to be altered, I strongly recommend against doing so
as for example the new altered items may violate the expected ordering at that point.
Do no rely on a specific order of when items are compared."#,
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_variable("Item")),
                    type_fn(
                        vec![type_variable("Item"), type_variable("Item")],
                        type_order
                    )
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "opt-span-sort",
                documentation: r#"Order the referenced items, less before greater according to a given function.
Items whose comparison returns `'equal .` are ordered according to their original position in the span ("stable sort").
While this function allows the compared items to be altered, I strongly recommend against doing so
as for example the new altered items may violate the expected ordering at that point.
Do no rely on a specific order of when items are compared."#,
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                    type_fn(
                        vec![type_variable("Item"), type_variable("Item")],
                        type_order
                    )
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "span-alter",
                documentation: "Change every item referenced by the given span with a given function to a new item of the same type.
If your change function needs extra context from the outside, use `span-step` instead",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_variable("Item")),
                    type_fn(vec![type_variable("Item")], type_variable("Item")),
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "opt-span-alter",
                documentation: "Change every item referenced by the given span with a given function to a new item of the same type.
If your change function needs extra context from the outside, use `opt-span-step` instead",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                    type_fn(vec![type_variable("Item")], type_variable("Item")),
                ],
                result: type_empty,
            },
            CoreFnInfo {
                name: "opt-span-add",
                documentation: "Attach a given item at the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                    type_variable("Item"),
                ],
                result: type_span(type_variable("Origin"), type_variable("Item"))
            },
            CoreFnInfo {
                name: "span-add",
                documentation: "Attach a given item at the end of the span",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_variable("Item")),
                    type_variable("Item"),
                ],
                result: type_span(type_variable("Origin"), type_variable("Item"))
            },
            CoreFnInfo {
                name: "opt-span-add-str-chars",
                documentation: "Attach a given `str` at the end of the span",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_char)),
                    type_str,
                ],
                result: type_span(type_variable("Origin"), type_char)
            },
            CoreFnInfo {
                name: "span-add-str-chars",
                documentation: "Attach a given `str` to the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_char),
                    type_str,
                ],
                result: type_span(type_variable("Origin"), type_char)
            },
            CoreFnInfo {
                name: "span-add-u32-chars",
                documentation: "Print a given `u32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_char),
                    type_u32,
                ],
                result: type_span(type_variable("Origin"), type_char)
            },
            CoreFnInfo {
                name: "opt-span-add-u32-chars",
                documentation: "Print a given `u32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_char)),
                    type_u32,
                ],
                result: type_span(type_variable("Origin"), type_char)
            },
            CoreFnInfo {
                name: "span-add-i32-chars",
                documentation: "Print a given `i32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_char),
                    type_i32,
                ],
                result: type_span(type_variable("Origin"), type_char)
            },
            CoreFnInfo {
                name: "opt-span-add-i32-chars",
                documentation: "Print a given `i32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_char)),
                    type_i32,
                ],
                result: type_span(type_variable("Origin"), type_char)
            },
            CoreFnInfo {
                name: "span-add-f32-chars",
                documentation: "Print a given `f32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_char),
                    type_f32,
                ],
                result: type_span(type_variable("Origin"), type_char)
            },
            CoreFnInfo {
                name: "opt-span-add-f32-chars",
                documentation: "Print a given `f32` after the end of the span.",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_char)),
                    type_f32,
                ],
                result: type_span(type_variable("Origin"), type_char)
            },
            CoreFnInfo {
                name: "opt-span-move-to-unset",
                documentation: "Move the given span to a unset range if there is unset space available where moving the given span to would reduce the amount of unset space.
Try to use sparingly",
                type_parameters: vec![],
                parameters: vec![type_opt(type_span(type_variable("Origin"), type_variable("Item")))],
                result: type_opt(type_span(type_variable("Origin"), type_variable("Item")))
            },
            CoreFnInfo {
                name: "span-move-to-unset",
                documentation: "Move the given span to a unset range if there is unset space available where moving the given span to would reduce the amount of unset space.",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"), type_variable("Item"))],
                result: type_span(type_variable("Origin"), type_variable("Item"))
            },
            CoreFnInfo {
                name: "span-move-to-end",
                documentation: "Move the given span to after all existing items if necessary.",
                type_parameters: vec![],
                parameters: vec![type_span(type_variable("Origin"), type_variable("Item"))],
                result: type_span(type_variable("Origin"), type_variable("Item"))
            },
            CoreFnInfo {
                name: "opt-span-move-to-end",
                documentation: "Move the given span to after all existing items if necessary.",
                type_parameters: vec![],
                parameters: vec![type_opt(type_span(type_variable("Origin"), type_variable("Item")))],
                result: type_opt(type_span(type_variable("Origin"), type_variable("Item")))
            },
            CoreFnInfo {
                name: "span-add-own-span",
                documentation: "Append the items of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are unset.
As an example, you could implement `span-add` in slug itself as
```slug
fn span-add(
    span span(Origin Item)
    new Item
)
    span(Origin Item)
    # the first line is optional: it ensures that the new slot will actually be connected,
    # meaning the new item can stay at its position
    = span buf-span-move-to-end(span)
    = new-slot buf-add(new)
    buf-span-add-own-span(span slot-to-span(new-slot))
```",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_variable("Item")),
                    type_span(type_variable("Origin"), type_variable("Item")),
                ],
                result: type_span(type_variable("Origin"), type_variable("Item"))
            },
            CoreFnInfo {
                name: "span-add-own-opt-span",
                documentation: "Append the items of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are unset.
The most common use case is re-combining spans that have been split up with e.g. `span-start` (see also `slot-to-span`)",
                type_parameters: vec![],
                parameters: vec![
                    type_span(type_variable("Origin"), type_variable("Item")),
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                ],
                result: type_span(type_variable("Origin"), type_variable("Item"))
            },
            CoreFnInfo {
                name: "opt-span-add-own-span",
                documentation: "Append the items of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are unset.
The most common use case is re-combining spans that have been split up with e.g. `span-end` (see also `slot-to-span`)",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                    type_span(type_variable("Origin"), type_variable("Item")),
                ],
                result: type_span(type_variable("Origin"), type_variable("Item"))
            },
            CoreFnInfo {
                name: "opt-span-add-own-opt-span",
                documentation: "Append the items of a given end span directly after the start span, returning the combined span.
If start and end spans are not already connected, both are appended at the end and their original spans are unset",
                type_parameters: vec![],
                parameters: vec![
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                    type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
                ],
                result: type_opt(type_span(type_variable("Origin"), type_variable("Item"))),
            },
            CoreFnInfo {
                name: "buf-to-unset",
                documentation: "Extract the underlying slice that has been used to store items in including spare capacity.
Note that this will return an empty unset-slice if there are still other instances of the same buf around.

The resulting `Unset-slice` can if necessary be casted to a new item type with `unset-slice-cast-or-rid-and-allocate`.
Finally, the allocation can be the base of a new `buf` with `buf-reuse`",
                type_parameters: vec![],
                parameters: vec![type_buf(type_variable("Origin"), type_variable("Item"))],
                result: type_unset_slice(type_variable("Item")),
            },
            CoreFnInfo {
                name: "unset-slice-allocate-length",
                documentation: "Create a new `Unset-slice` with a given length.
There rarely is a need to use this except when a function expects an `Unset-slice` as an argument",
                type_parameters: vec![Name::from_static("Item")],
                parameters: vec![type_u32],
                result: type_unset_slice(type_variable("Item")),
            },
            CoreFnInfo {
                name: "unset-slice-cast-or-rid-and-allocate",
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
                type_parameters: vec![Name::from_static("New-item")],
                parameters: vec![type_unset_slice(type_variable("Item"))],
                result: type_unset_slice(type_variable("New-item"))
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
fn answer() p32
    p32-add-clamp(2p32 40u32)
```",
                )),
                parameters: vec![],
                type_: None,
            },
        ),
        (
            Name::from_static("u32"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"A natural number >= 0 (unsigned integer) with 32 bits.
```slug
fn answer() u32
    u32-add-clamp(2u32 40u32)
```",
                )),
                parameters: vec![],
                type_: None,
            },
        ),
        (
            Name::from_static("i32"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r"A signed whole number (integer) with 32 bits.
```slug
fn answer() i32
    i32-add-clamp(-8i32 50i32)
```",
                )),
                parameters: vec![],
                type_: None,
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
fn answer() f32 =
    f32-add-clamp(-8.5f32 50.5f32)
```",
                )),
                parameters: vec![],
                type_: None,
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
str-start("🇳🇿"str)
# = {
# start "\(1F1FA)"char
#        Indicator N
# after Yes "\(1F1F8)"str
#        Indicator Z
# }
```
Read if interested: [swift's grapheme cluster docs](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/stringsandcharacters/#Extended-Grapheme-Clusters)"#,
                )),
                parameters: vec![],
                type_: None,
            },
        ),
        (
            Name::from_static("str"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    r#"A positive-length slice of known text which is valid for the entire program,
like `"abc"str` or `"\"hello 👀 \\\r\n world \(2665)\""str`
- `\(2665)` represents the hex code for special characters, here ♥
- `\"` represents a double quote "
- `\\` represents one backslash \
- `\n` represents line break
- `\r` represents carriage return

Internally, a string is compactly represented as UTF-8 bytes and can be accessed as such.
When building new strings at runtime, use functions like `opt-span-add-str-chars`."#,
                )),
                parameters: vec![],
                type_: None,
            },
        ),
        (
            Name::from_static("order"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "Result of a binary comparison.
```slug
u32-order(12u32 20u32)
# = Less<order>

fn u32-max(a u32 b u32) u32
    if u32-order(a b) {
    Less b
    Equal b
    Greater a
    }
```",
                )),
                parameters: vec![],
                type_: Some(type_choice([
                    ("Less", vec![]),
                    ("Equal", vec![]),
                    ("Greater", vec![]),
                ])),
            },
        ),
        (
            Name::from_static("opt"),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "Either you have some value (Yes) or you have nothing (No).",
                )),
                parameters: vec![Name::from_static("Yes")],
                type_: Some(type_choice([
                    ("No", vec![]),
                    ("Yes", vec![type_variable("Yes")])
                ])),
            },
        ),
        (
            Name::from_static(type_buf_name),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A grow- and shrinkable array buffer of items.
Has constant time access, modify, remove and add.
Note that like all other slug values a buf can be used multiple times at the same time
and modifying one instance will modify all other instances as well
(like with slots where modifying one item modifies the item pointed to by all other same slots).
This can get a little confusing, so it is recommended to only keep one instance of s specific buf around."
                )),
                parameters: vec![Name::from_static("Origin"), Name::from_static("Item")],
                type_: None,
            },
        ),
        (
            Name::from_static(type_slot_name),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A valid position of an item in a collection.
For consecutive `slot`s, check out `span`."
                )),
                parameters: vec![Name::from_static("Origin"), Name::from_static("Item")],
                type_: None,
            },
        ),
        (
            Name::from_static(type_span_name),
            CheckedTypeAlias {
                name_range: None,
                documentation: Some(Box::from(
                    "A range of ≥1 consecutive valid slots in a collection.
For potentially 0-length spans, use `opt(span(...))`"
                )),
                parameters: vec![Name::from_static("Origin"), Name::from_static("Item")],
                type_: None,
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
                type_: None,
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
                Type::Choice(variants) => {
                    for variant in variants {
                        for variant_value in &variant.values {
                            type_records(variant_value, records);
                        }
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
                Type::Alias { name: _, arguments } => {
                    for argument in arguments {
                        type_records(argument, records);
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
                Type::Choice(variants) => {
                    choices.insert(
                        sorted_variant_names(variants.iter().map(|field| &field.name))
                            // static variables will only be created once and won't be dropped anyway
                            .leak(),
                    );
                    for variant in variants {
                        for variant_value in &variant.values {
                            type_choices(variant_value, choices);
                        }
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
                Type::Alias { name: _, arguments } => {
                    for argument in arguments {
                        type_choices(argument, choices);
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
        | "item-step"
        | "opt-span-sort"
        | "span-sort"
        | "opt-span-alter"
        | "span-alter"
        | "opt-span-move-to-end"
        | "span-move-to-end"
        | "opt-span-add-own-opt-span"
        | "span-add-own-opt-span"
        | "opt-span-add-own-span"
        | "span-add-own-span"
        | "opt-span-add-span"
        | "opt-span-add-opt-span"
        | "span-add-span"
        | "span-add-opt-span"
        | "opt-span-add-f32-chars"
        | "span-add-f32-chars"
        | "opt-span-add-i32-chars"
        | "span-add-i32-chars"
        | "opt-span-add-u32-chars"
        | "span-add-u32-chars"
        | "span-add-str-chars"
        | "opt-span-add-str-chars"
        | "span-add"
        | "opt-span-add"
        | "add-array"
        | "add-str-chars"
        | "add"
        | "insert"
        | "set-or-insert"
        | "buf-pre-allocation-rid"
        | "buf-pre-allocate-at-least"
        | "buf-empty" => true,
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
pub fn syntax_project_format(project: &SyntaxProject, source: &str) -> String {
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
                            project_item_end(item_after_unrecognized, &project.expressions)
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
                if let Some(parameters) = parameters
                    && let Some((parameter0, parameter1_up)) = parameters.parameters.split_first()
                {
                    formatted.push('(');
                    formatted.push_str(&parameter0.value);
                    for parameter in parameter1_up {
                        formatted.push(' ');
                        formatted.push_str(&parameter.value);
                    }
                    formatted.push(')');
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
                syntax_type_parameters_format(&mut formatted, type_parameters);
                let header_line_span = range_line_span(lsp_types::Range {
                    start: *fn_keyword_start,
                    end: result_type
                        .as_ref()
                        .map(|result_type| type_end(result_type))
                        .or_else(|| {
                            closed_paren_start
                                .map(|closed_paren_start| symbol_end(closed_paren_start, ")"))
                        })
                        .or_else(|| parameters.last().map(|parameter| pattern_end(parameter)))
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
                formatted.push('(');
                if let Some((parameter0, parameter1_up)) = parameters.split_first() {
                    if header_line_span == LineSpan::Multiple {
                        linebreak_indented_into(&mut formatted, next_indent(0));
                    }
                    syntax_pattern_format(&mut formatted, next_indent(0), parameter0);
                    for parameter in parameter1_up {
                        space_or_linebreak_indented_into(
                            &mut formatted,
                            header_line_span,
                            next_indent(0),
                        );
                        syntax_pattern_format(&mut formatted, next_indent(0), parameter);
                    }
                    if header_line_span == LineSpan::Multiple {
                        linebreak_indented_into(&mut formatted, 0);
                    }
                }
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
                    }
                    None => {
                        space_or_linebreak_indented_into(&mut formatted, header_line_span, 0);
                    }
                }
                linebreak_indented_into(&mut formatted, next_indent(0));
                if let Some(documentation) = documentation {
                    syntax_comments_format(&mut formatted, 0, documentation);
                    linebreak_indented_into(&mut formatted, next_indent(0));
                }
                if let Some(result) = result {
                    syntax_expression_format(
                        &mut formatted,
                        next_indent(0),
                        &project.expressions,
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
fn syntax_type_parameters_format(
    formatted: &mut String,
    type_parameters: &Vec<WithStartPosition<Name>>,
) {
    if let Some((type_parameter0, type_parameter1_up)) = type_parameters.split_first() {
        formatted.push('<');
        formatted.push_str(&type_parameter0.value);
        for angled_type_parameter in type_parameter1_up {
            formatted.push(' ');
            formatted.push_str(&angled_type_parameter.value);
        }
        formatted.push('>');
    }
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
fn syntax_expression_format(
    formatted: &mut String,
    indent: usize,
    expressions: &Buf<Expressions, SyntaxExpression>,
    expression: &SyntaxExpression,
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
            arguments_rest,
            closed_paren_start,
        } => {
            if let Some(first_argument_dot) = first_argument_dot {
                syntax_expression_format(
                    formatted,
                    indent,
                    expressions,
                    expressions.item(&first_argument_dot.argument),
                );
                space_or_linebreak_indented_into(
                    formatted,
                    range_line_span(expression_range(expression, expressions)),
                    indent,
                );
                formatted.push('.');
            }
            formatted.push_str(&name.value);
            if let Some(type_arguments) = type_arguments
                && let Some((type_argument0, type_argument1_up)) =
                    type_arguments.types.split_first()
            {
                formatted.push('<');
                let type_arguments_line_span = range_line_span(lsp_types::Range {
                    start: type_arguments.open_angle_start,
                    end: type_arguments
                        .closed_angle_start
                        .map(|closed_angle_start| symbol_end(closed_angle_start, ">"))
                        .unwrap_or_else(|| {
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
                formatted.push('>');
            }
            formatted.push('(');
            if let Some((rest_argument0, rest_argument1_up)) = arguments_rest.split_first() {
                let arguments_line_span = range_line_span(lsp_types::Range {
                    start: open_paren_start
                        .or_else(|| {
                            type_arguments.as_ref().map(|type_arguments| {
                                expression_call_type_arguments_end(type_arguments)
                            })
                        })
                        .unwrap_or_else(|| name_end(with_start_position_as_ref(name))),
                    end: closed_paren_start.unwrap_or_else(|| {
                        expression_end(arguments_rest.last().unwrap_or(rest_argument0), expressions)
                    }),
                });
                if arguments_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_expression_format(formatted, indent, expressions, rest_argument0);
                for argument in rest_argument1_up {
                    space_or_linebreak_indented_into(
                        formatted,
                        arguments_line_span,
                        next_indent(indent),
                    );
                    syntax_expression_format(formatted, indent, expressions, argument);
                }
                if arguments_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
            }
            formatted.push(')');
        }
        SyntaxExpression::Variant {
            name,
            type_,
            values,
        } => {
            formatted.push_str(&name.value);
            if let Some(SyntaxExpressionVariantType {
                open_angle_start,
                type_: Some(type_),
                closed_angle_start,
            }) = type_
            {
                formatted.push('<');
                let type_line_span = range_line_span(lsp_types::Range {
                    start: *open_angle_start,
                    end: closed_angle_start
                        .map(|closed_angle_start| symbol_end(closed_angle_start, ">"))
                        .unwrap_or_else(|| type_end(type_)),
                });
                if type_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_type_format(formatted, next_indent(indent), type_);
                if type_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                formatted.push('>');
            }
            if let Some((value0, value1_up)) = values
                .as_ref()
                .and_then(|values| values.values.split_first())
            {
                formatted.push('(');
                let line_span = range_line_span(expression_range(expression, expressions));
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_expression_format(formatted, next_indent(indent), expressions, value0);
                for value in value1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_expression_format(formatted, next_indent(indent), expressions, value);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                formatted.push(')');
            }
        }
        SyntaxExpression::Fn {
            fn_keyword_start,
            open_paren_start,
            parameters,
            closed_paren_start: _,
            result,
        } => {
            formatted.push_str("fn(");
            if let Some((parameter0, parameter1_up)) = parameters.split_first() {
                let parameter_line_span = range_line_span(lsp_types::Range {
                    start: open_paren_start.unwrap_or(*fn_keyword_start),
                    end: pattern_end(parameter1_up.last().unwrap_or(parameter0)),
                });
                if parameter_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_pattern_format(formatted, next_indent(indent), parameter0);
                for parameter in parameter1_up {
                    space_or_linebreak_indented_into(
                        formatted,
                        parameter_line_span,
                        next_indent(indent),
                    );
                    syntax_pattern_format(formatted, next_indent(indent), parameter);
                }
                if parameter_line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
            }
            formatted.push(')');
            if let Some(result) = result {
                space_or_linebreak_indented_into(
                    formatted,
                    range_line_span(expression_range(expression, expressions)),
                    next_indent(indent),
                );
                syntax_expression_format(formatted, indent, expressions, expressions.item(result));
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
                let line_span = range_line_span(expression_range(expression, expressions));
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                syntax_expression_format(formatted, next_indent(indent), expressions, part0);
                for part in part1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_expression_format(formatted, next_indent(indent), expressions, part);
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
            let line_span = range_line_span(expression_range(expression, expressions));
            for field in fields {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                syntax_expression_record_field_format(formatted, indent, expressions, field);
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
                        range_line_span(pattern_range(pattern)),
                        next_indent(indent),
                    );
                    syntax_pattern_format(formatted, next_indent(indent), pattern);
                }
            }
            match queried {
                None => {}
                Some(queried) => {
                    let queried = expressions.item(queried);
                    let queried_line_span = range_line_span(expression_range(queried, expressions));
                    space_or_linebreak_indented_into(
                        formatted,
                        queried_line_span,
                        next_indent(indent),
                    );
                    syntax_expression_format(formatted, next_indent(indent), expressions, queried);
                    let line_span = range_line_span(expression_range(expression, expressions));
                    space_or_linebreak_indented_into(formatted, line_span, indent);
                }
            }
            if let Some(result) = result {
                syntax_expression_format(formatted, indent, expressions, expressions.item(result));
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
                    let line_span = range_line_span(expression_range(queried, expressions));
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_expression_format(formatted, next_indent(indent), expressions, queried);
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                }
            }
            formatted.push('{');
            for case in cases {
                linebreak_indented_into(formatted, indent);
                syntax_expression_if_case_format(formatted, indent, expressions, case);
            }
            linebreak_indented_into(formatted, indent);
            formatted.push('}');
        }
    }
}
fn syntax_expression_record_field_format(
    formatted: &mut String,
    indent: usize,
    expressions: &Buf<Expressions, SyntaxExpression>,

    field: &SyntaxRecordField<SyntaxExpression>,
) {
    formatted.push_str(&field.name.value);
    match &field.value {
        None => {
            formatted.push(' ');
        }
        Some(value) => {
            let line_span = range_line_span(lsp_types::Range {
                start: field.name.start,
                end: expression_range(value, expressions).end,
            });
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            syntax_expression_format(formatted, next_indent(indent), expressions, value);
        }
    }
}
fn syntax_expression_if_case_format(
    formatted: &mut String,
    indent: usize,
    expressions: &Buf<Expressions, SyntaxExpression>,

    case: &SyntaxExpressionIfCase,
) {
    syntax_pattern_format(formatted, next_indent(indent), &case.pattern);
    match &case.result {
        None => {
            formatted.push(' ');
        }
        Some(result) => {
            let line_span = range_line_span(lsp_types::Range {
                start: pattern_start(&case.pattern),
                end: expression_end(result, expressions),
            });
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            syntax_expression_format(formatted, next_indent(indent), expressions, result);
        }
    }
}
fn syntax_pattern_format(formatted: &mut String, indent: usize, pattern: &SyntaxPattern) {
    match pattern {
        SyntaxPattern::Variable { name, type_ } => {
            formatted.push_str(&name.value);
            if let Some(type_) = type_ {
                space_or_linebreak_indented_into(
                    formatted,
                    range_line_span(pattern_range(pattern)),
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
                    range_line_span(pattern_range(pattern)),
                    indent,
                );
                syntax_type_format(formatted, indent, type_);
            }
        }
        SyntaxPattern::Variant { name, values } => {
            formatted.push_str(&name.value.as_ref());
            if let Some((value0, value1_up)) = values
                .as_ref()
                .and_then(|values| values.values.split_first())
            {
                formatted.push('(');
                let line_span = range_line_span(pattern_range(pattern));
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_pattern_format(formatted, next_indent(indent), value0);
                for value in value1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_pattern_format(formatted, next_indent(indent), value);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                formatted.push(')');
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
                let line_span = range_line_span(pattern_range(pattern));
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_pattern_format(formatted, next_indent(indent), part0);
                for part in part1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_pattern_format(formatted, next_indent(indent), part);
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
            formatted.push('{');
            if let Some((field0, field1_up)) = fields.split_first() {
                let line_span = range_line_span(pattern_range(pattern));
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                syntax_pattern_record_field_format(formatted, indent, field0);
                for field in field1_up {
                    space_or_linebreak_indented_into(formatted, line_span, indent);
                    syntax_pattern_record_field_format(formatted, indent, field);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
            }
            formatted.push('}');
        }
    }
}
fn syntax_pattern_record_field_format(
    formatted: &mut String,
    indent: usize,

    field: &SyntaxRecordField<SyntaxPattern>,
) {
    formatted.push_str(&field.name.value);
    match &field.value {
        None => {
            formatted.push(' ');
        }
        Some(value) => {
            let line_span = range_line_span(lsp_types::Range {
                start: field.name.start,
                end: pattern_range(value).end,
            });
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            syntax_pattern_format(formatted, next_indent(indent), value);
        }
    }
}
fn syntax_type_format(formatted: &mut String, indent: usize, type_: &SyntaxType) {
    match type_ {
        SyntaxType::Variable(name) => {
            formatted.push_str(&name.value);
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
            space_or_linebreak_indented_into(formatted, line_span, indent);
            if let Some(result) = result {
                syntax_type_format(formatted, indent, result);
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
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, next_indent(indent));
                }
                syntax_type_format(formatted, next_indent(indent), argument0);
                for argument_type in argument1_up {
                    space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
                    syntax_type_format(formatted, next_indent(indent), argument_type);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
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
            formatted.push('{');
            if let Some((field0, field1_up)) = fields.split_first() {
                let line_span = range_line_span(type_range(type_));
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                syntax_type_field_format(formatted, indent, field0);
                for field in field1_up {
                    space_or_linebreak_indented_into(formatted, line_span, indent);
                    syntax_type_field_format(formatted, indent, field);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
            }
            formatted.push('}');
        }
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => {
            formatted.push('[');
            if let Some((variant0, variant1_up)) = variants.split_first() {
                let line_span = range_line_span(type_range(type_));
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
                syntax_type_variant_format(formatted, indent, variant0);
                for variant in variant1_up {
                    space_or_linebreak_indented_into(formatted, line_span, indent);
                    syntax_type_variant_format(formatted, indent, variant);
                }
                if line_span == LineSpan::Multiple {
                    linebreak_indented_into(formatted, indent);
                }
            }
            formatted.push(']');
        }
    }
}
fn syntax_type_field_format(
    formatted: &mut String,
    indent: usize,
    field: &SyntaxRecordField<SyntaxType>,
) {
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
fn syntax_type_variant_format(formatted: &mut String, indent: usize, variant: &SyntaxTypeVariant) {
    formatted.push_str(&variant.name.value);
    if let Some((value0, value1_up)) = variant
        .values
        .as_ref()
        .and_then(|values| values.values.split_first())
    {
        formatted.push('(');
        let line_span = range_line_span(lsp_types::Range {
            start: variant.name.start,
            end: type_end(value1_up.last().unwrap_or(value0)),
        });
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, next_indent(indent));
        }
        syntax_type_format(formatted, next_indent(indent), value0);
        for value in value1_up {
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            syntax_type_format(formatted, next_indent(indent), value);
        }
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, indent);
        }
        formatted.push(')');
    }
}

pub enum SyntaxSymbol<'a> {
    ProjectTypeOrUnknown {
        name: WithStartPosition<&'a Name>,
        construct_info: ConstructInfo,
    },
    TypeVariable {
        name: &'a Name,
        use_start: lsp_types::Position,
        scope: &'a SyntaxProjectItem,
    },
    VariantOrUnknown(WithStartPosition<&'a Name>),
    ProjectFnOrVariableUnknown {
        name: WithStartPosition<&'a Name>,
        construct_info: ConstructInfo,
        local_variables: std::collections::HashMap<&'a Name, LocalVariableSymbolOrigin<'a>>,
    },
    LocalVariable {
        name: &'a Name,
        use_start: lsp_types::Position,
        origin: LocalVariableSymbolOrigin<'a>,
    },
}
pub enum ConstructInfo {
    // these are a little weird. Maybe there should also be MaybeExpectingArgument or something?
    NotExpectingArgument,
    ArgumentMissing,
    ArgumentExists,
    Declaration,
}
pub struct LocalVariableSymbolOrigin<'a> {
    pub start: lsp_types::Position,
    pub scope: Option<&'a SyntaxExpression>,
    pub type_: Option<Type>,
}
impl<'a> Clone for LocalVariableSymbolOrigin<'a> {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            scope: self.scope,
            type_: self.type_.clone(),
        }
    }
}
pub fn project_symbol_at_position<'a>(
    project: &'a SyntaxProject,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    position: lsp_types::Position,
) -> Option<SyntaxSymbol<'a>> {
    let Ok(item_containing_position_index) =
        project
            .items
            .binary_search_by(|item| match project_item_start(item).cmp(&position) {
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    match project_item_end(item, &project.expressions).cmp(&position) {
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
                return Some(SyntaxSymbol::ProjectFnOrVariableUnknown {
                    name: with_start_position_as_ref(name),
                    construct_info: ConstructInfo::Declaration,
                    local_variables: std::collections::HashMap::new(),
                });
            }
            type_parameters
                .iter()
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
                        pattern_symbol_at_position(parameter, None, position, item, result.as_ref())
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
                            syntax_parameter_pattern_variables_for_each(
                                parameter,
                                &mut |parameter_variable_name, parameter_variable_type| {
                                    local_variables.insert(
                                        parameter_variable_name.value,
                                        LocalVariableSymbolOrigin {
                                            start: parameter_variable_name.start,
                                            scope: Some(result),
                                            type_: parameter_variable_type
                                                .map(|type_| syntax_type_to_type(type_)),
                                        },
                                    );
                                },
                            );
                        }
                        expression_symbol_at_position(
                            result,
                            position,
                            type_aliases,
                            checked_queries,
                            &project.expressions,
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
fn expression_symbol_at_position<'a>(
    expression: &'a SyntaxExpression,
    position: lsp_types::Position,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    expressions: &'a Buf<Expressions, SyntaxExpression>,

    scope: &'a SyntaxProjectItem,
    local_variables: &mut std::collections::HashMap<&'a Name, LocalVariableSymbolOrigin<'a>>,
) -> Option<SyntaxSymbol<'a>> {
    if !range_includes_position(expression_range(expression, expressions), position) {
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
                .unwrap_or_else(|| SyntaxSymbol::ProjectFnOrVariableUnknown {
                    name: with_start_position_as_ref(name),
                    construct_info: ConstructInfo::NotExpectingArgument,
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
                return Some(
                    local_variables
                        .remove(&name.value)
                        .map(|local_variable| SyntaxSymbol::LocalVariable {
                            name: &name.value,
                            use_start: name.start,
                            origin: local_variable,
                        })
                        .unwrap_or_else(|| SyntaxSymbol::ProjectFnOrVariableUnknown {
                            name: with_start_position_as_ref(name),
                            construct_info: if first_argument_dot.is_some() || !arguments.is_empty()
                            {
                                ConstructInfo::ArgumentExists
                            } else {
                                ConstructInfo::ArgumentMissing
                            },
                            local_variables: std::mem::take(local_variables),
                        }),
                );
            }
            type_arguments
                .iter()
                .flat_map(|type_argument| &type_argument.types)
                .find_map(|type_argument| type_symbol_at_position(type_argument, position, scope))
                .or_else(|| {
                    first_argument_dot.as_ref().and_then(
                        |SyntaxExpressionArgumentDot {
                             argument,
                             dot_start: _,
                         }| {
                            expression_symbol_at_position(
                                expressions.item(argument),
                                position,
                                type_aliases,
                                checked_queries,
                                expressions,
                                scope,
                                local_variables,
                            )
                        },
                    )
                })
                .or_else(|| {
                    arguments.iter().find_map(|argument| {
                        expression_symbol_at_position(
                            argument,
                            position,
                            type_aliases,
                            checked_queries,
                            expressions,
                            scope,
                            local_variables,
                        )
                    })
                })
        }
        SyntaxExpression::Variant {
            name,
            type_,
            values,
        } => {
            if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
                return Some(SyntaxSymbol::VariantOrUnknown(WithStartPosition {
                    start: name.start,
                    value: &name.value,
                }));
            }
            type_
                .as_ref()
                .and_then(
                    |SyntaxExpressionVariantType {
                         open_angle_start: _,
                         type_,
                         closed_angle_start: _,
                     }| {
                        type_
                            .as_ref()
                            .and_then(|type_| type_symbol_at_position(type_, position, scope))
                    },
                )
                .or_else(|| {
                    values
                        .iter()
                        .flat_map(|values| &values.values)
                        .find_map(|value| {
                            expression_symbol_at_position(
                                value,
                                position,
                                type_aliases,
                                checked_queries,
                                expressions,
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
                    pattern_symbol_at_position(parameter, None, position, scope, result)
                })
                .or_else(|| {
                    result.as_ref().and_then(|result| {
                        for parameter in parameters {
                            syntax_parameter_pattern_variables_for_each(
                                parameter,
                                &mut |name, type_| {
                                    local_variables.insert(
                                        name.value,
                                        LocalVariableSymbolOrigin {
                                            start: name.start,
                                            scope: Some(result),
                                            type_: type_.map(|type_| syntax_type_to_type(type_)),
                                        },
                                    );
                                },
                            );
                        }
                        expression_symbol_at_position(
                            result,
                            position,
                            type_aliases,
                            checked_queries,
                            expressions,
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
                expressions,
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
                    expressions,
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
                expressions,
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
                    expressions,
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
                            scope,
                            result,
                        )
                    })
                    .or_else(|| {
                        let Some(result) = result else {
                            return None;
                        };
                        if let Some(pattern) = pattern {
                            syntax_query_pattern_variables_for_each(
                                pattern,
                                expected_pattern_type,
                                &mut |name, type_| {
                                    local_variables.insert(
                                        name.value,
                                        LocalVariableSymbolOrigin {
                                            start: name.start,
                                            scope: Some(result),
                                            type_: type_,
                                        },
                                    );
                                },
                                type_aliases,
                            );
                        }
                        expression_symbol_at_position(
                            result,
                            position,
                            type_aliases,
                            checked_queries,
                            expressions,
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
                    expressions,
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
                        expressions,
                        scope,
                        local_variables,
                    )
                })
            }),
    }
}
fn expression_if_case_symbol_at_position<'a>(
    case: &'a SyntaxExpressionIfCase,
    expected_pattern_type: Option<&Type>,
    position: lsp_types::Position,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
    checked_queries: &std::collections::HashMap<lsp_types::Position, CheckedQuery>,
    expressions: &'a Buf<Expressions, SyntaxExpression>,
    scope: &'a SyntaxProjectItem,
    local_variables: &mut std::collections::HashMap<&'a Name, LocalVariableSymbolOrigin<'a>>,
) -> Option<SyntaxSymbol<'a>> {
    pattern_symbol_at_position(
        &case.pattern,
        expected_pattern_type,
        position,
        scope,
        case.result.as_ref(),
    )
    .or_else(|| {
        let Some(result) = &case.result else {
            return None;
        };
        // don't modify local_variables unless known to be in the right case
        if !range_includes_position(expression_range(result, expressions), position) {
            return None;
        }
        syntax_query_pattern_variables_for_each(
            &case.pattern,
            expected_pattern_type,
            &mut |name, type_| {
                local_variables.insert(
                    name.value,
                    LocalVariableSymbolOrigin {
                        start: name.start,
                        scope: Some(result),
                        type_: type_,
                    },
                );
            },
            type_aliases,
        );
        expression_symbol_at_position(
            result,
            position,
            type_aliases,
            checked_queries,
            expressions,
            scope,
            local_variables,
        )
    })
}
fn syntax_parameter_pattern_variables_for_each<'a>(
    pattern: &'a SyntaxPattern,
    step_variable: &mut impl FnMut(WithStartPosition<&'a Name>, Option<&'a SyntaxType>),
) {
    match pattern {
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_: _,
        } => {}
        SyntaxPattern::Variable { name, type_ } => {
            step_variable(with_start_position_as_ref(name), type_.as_ref());
        }
        SyntaxPattern::Variant { name: _, values } => {
            for value in values.iter().flat_map(|values| &values.values) {
                syntax_parameter_pattern_variables_for_each(value, step_variable);
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_parameter_pattern_variables_for_each(part, step_variable);
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_parameter_pattern_variables_for_each(value, step_variable);
                }
            }
        }
    }
}
fn syntax_query_pattern_variables_for_each<'a>(
    pattern: &'a SyntaxPattern,
    expected_type: Option<&Type>,
    step_variable: &mut impl FnMut(WithStartPosition<&'a Name>, Option<Type>),
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    match pattern {
        SyntaxPattern::Ignored {
            underscore_start: _,
            type_: _,
        } => {}
        SyntaxPattern::Variable { name, type_: _ } => {
            step_variable(with_start_position_as_ref(name), expected_type.cloned());
        }
        SyntaxPattern::Variant { name, values } => {
            let expected_type = expected_type
                .map(|expected_type| type_alias_resolve_to_non_alias(expected_type, type_aliases));
            let expected_value_types = match expected_type.as_deref() {
                Some(Type::Choice(expected_variants)) => expected_variants
                    .iter()
                    .find(|expected_variant| name.value == expected_variant.name)
                    .map(|variant| &variant.values),
                _ => None,
            };
            for (value_index, value) in values.iter().flat_map(|values| &values.values).enumerate()
            {
                syntax_query_pattern_variables_for_each(
                    value,
                    expected_value_types
                        .and_then(|expected_value_types| expected_value_types.get(value_index)),
                    step_variable,
                    type_aliases,
                );
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            let expected_type = expected_type
                .map(|expected_type| type_alias_resolve_to_non_alias(expected_type, type_aliases));
            let expected_part_types = match expected_type.as_deref() {
                Some(Type::Tuple(expected_part_types)) => Some(expected_part_types),
                _ => None,
            };
            for (part_index, part) in parts.iter().enumerate() {
                syntax_query_pattern_variables_for_each(
                    part,
                    expected_part_types
                        .and_then(|expected_part_types| expected_part_types.get(part_index)),
                    step_variable,
                    type_aliases,
                );
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            let expected_type = expected_type
                .map(|expected_type| type_alias_resolve_to_non_alias(expected_type, type_aliases));
            let expected_field_types = match expected_type.as_deref() {
                Some(Type::Record(expected_field_types)) => Some(expected_field_types),
                _ => None,
            };
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_query_pattern_variables_for_each(
                        value,
                        expected_field_types.and_then(|expected_field_types| {
                            expected_field_types
                                .iter()
                                .find(|expected_field| field.name.value == expected_field.name)
                                .map(|field| &field.value)
                        }),
                        step_variable,
                        type_aliases,
                    );
                }
            }
        }
    }
}
fn pattern_symbol_at_position<'a>(
    pattern: &'a SyntaxPattern,
    expected_type: Option<&Type>,
    position: lsp_types::Position,
    project_item_scope: &'a SyntaxProjectItem,
    expression_scope: Option<&'a SyntaxExpression>,
) -> Option<SyntaxSymbol<'a>> {
    if !range_includes_position(pattern_range(pattern), position) {
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
                        type_: expected_type
                            .cloned()
                            .or_else(|| type_.as_ref().map(|type_| syntax_type_to_type(type_))),
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
        SyntaxPattern::Variant { name, values } => {
            if range_includes_position(name_range(with_start_position_as_ref(name)), position) {
                return Some(SyntaxSymbol::VariantOrUnknown(with_start_position_as_ref(
                    name,
                )));
            }
            values
                .iter()
                .flat_map(|values| &values.values)
                .enumerate()
                .find_map(|(value_index, value)| {
                    pattern_symbol_at_position(
                        value,
                        expected_type.and_then(|expected_type| match expected_type {
                            Type::Choice(expected_variants) => expected_variants
                                .iter()
                                .find(|expected_variant| name.value == expected_variant.name)
                                .and_then(|expected_variant| {
                                    expected_variant.values.get(value_index)
                                }),
                            _ => None,
                        }),
                        position,
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
                                .find(|expected_field| field.name.value == expected_field.name),
                            _ => None,
                        })
                        .map(|field| &field.value),
                    position,
                    project_item_scope,
                    expression_scope,
                )
            })
        }),
    }
}
fn type_symbol_at_position<'a>(
    type_: &'a SyntaxType,
    position: lsp_types::Position,

    scope: &'a SyntaxProjectItem,
) -> Option<SyntaxSymbol<'a>> {
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
        } => fields.iter().find_map(|field| {
            field
                .value
                .as_ref()
                .and_then(|field_value| type_symbol_at_position(field_value, position, scope))
        }),
        SyntaxType::Choice {
            open_bracket_start: _,
            variants,
            closed_bracket_start: _,
        } => variants
            .iter()
            .filter_map(|variant| variant.values.as_ref())
            .flat_map(|values| &values.values)
            .find_map(|value| type_symbol_at_position(value, position, scope)),
    }
}

pub fn syntax_project_symbol_origin_range(
    project: &SyntaxProject,
    symbol: &SyntaxSymbol,
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
                    && type_alias_name.value == symbol_name.value
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
                    if name.value == symbol_name {
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
            } => type_parameters.iter().find_map(|name| {
                if name.value == symbol_name {
                    Some(name_range(with_start_position_as_ref(name)))
                } else {
                    None
                }
            }),
            SyntaxProjectItem::Comments(_) | SyntaxProjectItem::Unrecognized { .. } => None,
        },
        SyntaxSymbol::VariantOrUnknown(_) => None,
        SyntaxSymbol::ProjectFnOrVariableUnknown {
            name: symbol_name,
            construct_info: _,
            local_variables: _,
        } => project.items.iter().find_map(|item| match item {
            SyntaxProjectItem::Fn {
                name: Some(fn_name),
                ..
            } if fn_name.value == symbol_name.value => {
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
    }
}
/// resulting uses do not include the origin. For that, use syntax_project_symbol_origin_range
pub fn syntax_project_symbol_uses(
    project: &SyntaxProject,
    symbol: &SyntaxSymbol,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
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
                parameters,
                closed_paren_start: _,
                result_type,
                documentation: _,
                result,
            } => {
                for parameter in parameters {
                    syntax_pattern_symbol_uses_into(&mut uses, parameter, symbol);
                }
                if let Some(result_type) = result_type {
                    syntax_type_symbol_uses_into(&mut uses, result_type, symbol);
                }
                if let Some(result) = result {
                    syntax_expression_symbol_uses_into(
                        &mut uses,
                        result,
                        symbol,
                        &std::collections::HashSet::new(),
                        &project.expressions,
                        type_aliases,
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
                    &std::collections::HashSet::new(),
                    &project.expressions,
                    type_aliases,
                );
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
                            syntax_pattern_symbol_uses_into(&mut uses, parameter, symbol);
                        }
                        if let Some(result_type) = result_type {
                            syntax_type_symbol_uses_into(&mut uses, result_type, symbol);
                        }
                        if let Some(result) = result {
                            syntax_expression_symbol_uses_into(
                                &mut uses,
                                result,
                                symbol,
                                &std::collections::HashSet::new(),
                                &project.expressions,
                                type_aliases,
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
        SyntaxSymbol::VariantOrUnknown(_) | SyntaxSymbol::ProjectFnOrVariableUnknown { .. } => {
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
                            syntax_pattern_symbol_uses_into(&mut uses, parameter, symbol);
                        }
                        if let Some(result_type) = result_type {
                            syntax_type_symbol_uses_into(&mut uses, result_type, symbol);
                        }
                        if let Some(result) = result {
                            let mut parameter_introduced_variables =
                                std::collections::HashSet::with_capacity(2);
                            for parameter in parameters {
                                syntax_parameter_pattern_variables_for_each(
                                    parameter,
                                    &mut |parameter_introduced_variable_name, _type_| {
                                        parameter_introduced_variables
                                            .insert(parameter_introduced_variable_name.value);
                                    },
                                );
                            }
                            syntax_expression_symbol_uses_into(
                                &mut uses,
                                result,
                                symbol,
                                &parameter_introduced_variables,
                                &project.expressions,
                                type_aliases,
                            );
                        }
                    }
                    SyntaxProjectItem::TypeAlias { .. } => {}
                    SyntaxProjectItem::Comments(_) => {}
                    SyntaxProjectItem::Unrecognized { .. } => {}
                }
            }
        }
    }
    uses
}
fn syntax_type_symbol_uses_into(
    uses: &mut Vec<lsp_types::Range>,
    type_: &SyntaxType,
    symbol: &SyntaxSymbol,
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
            for variant in variants {
                for variant_value in variant.values.iter().flat_map(|values| &values.values) {
                    syntax_type_symbol_uses_into(uses, variant_value, symbol);
                }
            }
        }
        SyntaxType::ConstructWithArguments { name, arguments } => {
            if let SyntaxSymbol::ProjectTypeOrUnknown {
                name: symbol_name,
                construct_info: _,
            } = symbol
                && name.value == symbol_name.value
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
fn syntax_pattern_symbol_uses_into(
    uses: &mut Vec<lsp_types::Range>,
    pattern: &SyntaxPattern,
    symbol: &SyntaxSymbol,
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
        SyntaxPattern::Variant { name, values } => {
            if let SyntaxSymbol::VariantOrUnknown(symbol_name) = symbol
                && name.value == symbol_name.value
            {
                uses.push(name_range(with_start_position_as_ref(name)));
            }
            for value in values.iter().flat_map(|values| &values.values) {
                syntax_pattern_symbol_uses_into(uses, value, symbol);
            }
        }
        SyntaxPattern::Tuple {
            star_start: _,
            open_paren_start: _,
            parts,
            closed_paren_start: _,
        } => {
            for part in parts {
                syntax_pattern_symbol_uses_into(uses, part, symbol);
            }
        }
        SyntaxPattern::Record {
            open_curly_start: _,
            fields,
            closed_curly_start: _,
        } => {
            for field in fields {
                if let Some(value) = &field.value {
                    syntax_pattern_symbol_uses_into(uses, value, symbol);
                }
            }
        }
    }
}
// it would most likely be better to separate out variables as e.g. there can only be one
fn syntax_expression_symbol_uses_into(
    uses: &mut Vec<lsp_types::Range>,
    expression: &SyntaxExpression,
    symbol: &SyntaxSymbol,
    local_variables: &std::collections::HashSet<&Name>,
    expressions: &Buf<Expressions, SyntaxExpression>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
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
            | SyntaxSymbol::ProjectFnOrVariableUnknown { .. } => {}
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
                | SyntaxSymbol::ProjectTypeOrUnknown { .. }
                | SyntaxSymbol::VariantOrUnknown(_) => {}
                SyntaxSymbol::LocalVariable {
                    name: symbol_name, ..
                } => {
                    if *symbol_name == &name.value {
                        uses.push(name_range(with_start_position_as_ref(name)));
                    }
                }
                SyntaxSymbol::ProjectFnOrVariableUnknown {
                    name:
                        WithStartPosition {
                            start: _,
                            value: symbol_name,
                        },
                    construct_info: _,
                    local_variables: _,
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
                    local_variables,
                    expressions,
                    type_aliases,
                );
            }
            for argument in arguments {
                syntax_expression_symbol_uses_into(
                    uses,
                    argument,
                    symbol,
                    local_variables,
                    expressions,
                    type_aliases,
                );
            }
        }
        SyntaxExpression::Variant {
            name,
            type_: type_argument,
            values,
        } => {
            if let SyntaxSymbol::VariantOrUnknown(symbol_name) = symbol
                && name.value == symbol_name.value
            {
                uses.push(name_range(with_start_position_as_ref(name)));
            }
            if let Some(SyntaxExpressionVariantType {
                open_angle_start: _,
                type_: Some(type_),
                closed_angle_start: _,
            }) = type_argument
            {
                syntax_type_symbol_uses_into(uses, type_, symbol);
            }
            for value in values.iter().flat_map(|values| &values.values) {
                syntax_expression_symbol_uses_into(
                    uses,
                    value,
                    symbol,
                    local_variables,
                    expressions,
                    type_aliases,
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
                | SyntaxSymbol::TypeVariable { .. }
                | SyntaxSymbol::VariantOrUnknown(_)
                | SyntaxSymbol::ProjectFnOrVariableUnknown { .. } => {}
            }
            for parameter in parameters {
                syntax_pattern_symbol_uses_into(uses, parameter, symbol);
            }
            if let Some(result) = result {
                let mut parameter_pattern_variables = std::collections::HashSet::new();
                for parameter in parameters {
                    syntax_pattern_symbol_uses_into(uses, parameter, symbol);
                    syntax_parameter_pattern_variables_for_each(
                        parameter,
                        &mut |pattern_variable_name, _type_| {
                            parameter_pattern_variables.insert(pattern_variable_name.value);
                        },
                    );
                }
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.item(result),
                    symbol,
                    &parameter_pattern_variables,
                    expressions,
                    type_aliases,
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
                    local_variables,
                    expressions,
                    type_aliases,
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
                    local_variables,
                    expressions,
                    type_aliases,
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
                    local_variables,
                    expressions,
                    type_aliases,
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
                    local_variables,
                    expressions,
                    type_aliases,
                );
            }
            if let Some(pattern) = pattern {
                syntax_pattern_symbol_uses_into(uses, pattern, symbol);
            }
            if let Some(result) = result {
                let mut local_variables = std::borrow::Cow::Borrowed(local_variables);
                if let Some(pattern) = pattern {
                    syntax_parameter_pattern_variables_for_each(
                        pattern,
                        &mut |pattern_variable_name, _type_| {
                            local_variables.to_mut().insert(pattern_variable_name.value);
                        },
                    );
                }
                syntax_expression_symbol_uses_into(
                    uses,
                    expressions.item(result),
                    symbol,
                    &local_variables,
                    expressions,
                    type_aliases,
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
                    local_variables,
                    expressions,
                    type_aliases,
                );
            }
            for case in cases {
                if let Some(result) = &case.result {
                    let mut local_variables = std::borrow::Cow::Borrowed(local_variables);
                    syntax_pattern_symbol_uses_into(uses, &case.pattern, symbol);
                    syntax_query_pattern_variables_for_each(
                        &case.pattern,
                        None,
                        &mut |pattern_variable_name, _type_| {
                            local_variables.to_mut().insert(pattern_variable_name.value);
                        },
                        type_aliases,
                    );
                    syntax_expression_symbol_uses_into(
                        uses,
                        result,
                        symbol,
                        &local_variables,
                        expressions,
                        type_aliases,
                    );
                }
            }
        }
    }
}
fn syntax_expression_record_field_symbol_uses_into(
    uses: &mut Vec<lsp_types::Range>,
    field: &SyntaxRecordField<SyntaxExpression>,
    symbol: &SyntaxSymbol,
    local_variables: &std::collections::HashSet<&Name>,
    expressions: &Buf<Expressions, SyntaxExpression>,
    type_aliases: &std::collections::HashMap<Name, CheckedTypeAlias>,
) {
    if let Some(field_value) = &field.value {
        syntax_expression_symbol_uses_into(
            uses,
            field_value,
            symbol,
            local_variables,
            expressions,
            type_aliases,
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
pub fn project_highlight(state: &mut HighlightState, project: &SyntaxProject) {
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
                parameters,
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
                for type_parameter_name in type_parameters {
                    syntax_name_highlight(
                        state,
                        type_parameter_name,
                        lsp_types::SemanticTokenTypes::TypeParameter,
                    );
                }
                for parameter in parameters {
                    syntax_pattern_highlight(state, parameter);
                }
                if let Some(result_type) = result_type {
                    syntax_type_highlight(state, result_type);
                }
                if let Some(documentation) = documentation {
                    syntax_comments_highlight(state, documentation);
                }
                if let Some(result) = result {
                    syntax_expression_highlight(state, &project.expressions, result);
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
    for type_parameter_name in type_parameters {
        syntax_name_highlight(
            state,
            type_parameter_name,
            lsp_types::SemanticTokenTypes::TypeParameter,
        );
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
fn syntax_pattern_highlight(state: &mut HighlightState, pattern: &SyntaxPattern) {
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
        SyntaxPattern::Variant { name, values } => {
            syntax_variant_name_highlight(state, name);
            for value in values.iter().flat_map(|values| &values.values) {
                syntax_pattern_highlight(state, value);
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
                syntax_pattern_highlight(state, part);
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
                    syntax_pattern_highlight(state, value);
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
                for value in variant.values.iter().flat_map(|values| &values.values) {
                    syntax_type_highlight(state, value);
                }
            }
        }
    }
}
fn syntax_expression_highlight(
    state: &mut HighlightState,
    expressions: &Buf<Expressions, SyntaxExpression>,
    expression: &SyntaxExpression,
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
                syntax_expression_highlight(state, expressions, argument);
            }
        }
        SyntaxExpression::Variant {
            name,
            type_: type_argument,
            values,
        } => {
            syntax_name_highlight(state, name, lsp_types::SemanticTokenTypes::EnumMember);
            if let Some(SyntaxExpressionVariantType {
                open_angle_start: _,
                type_: Some(type_),
                closed_angle_start: _,
            }) = type_argument
            {
                syntax_type_highlight(state, type_);
            }
            for value in values.iter().flat_map(|values| &values.values) {
                syntax_expression_highlight(state, expressions, value);
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
                syntax_pattern_highlight(state, parameter);
            }
            if let Some(result) = result {
                syntax_expression_highlight(state, expressions, expressions.item(result));
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
                syntax_expression_highlight(state, expressions, part);
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
                    syntax_expression_highlight(state, expressions, value);
                }
            }
        }
        SyntaxExpression::Commented {
            comments,
            expression,
        } => {
            syntax_comments_highlight(state, comments);
            if let Some(expression) = expression {
                syntax_expression_highlight(state, expressions, expressions.item(expression));
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
                syntax_pattern_highlight(state, pattern);
            }
            if let Some(queried) = queried {
                syntax_expression_highlight(state, expressions, expressions.item(queried));
            }
            if let Some(result) = result {
                syntax_expression_highlight(state, expressions, expressions.item(result));
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
                syntax_expression_highlight(state, expressions, expressions.item(queried));
            }
            for case in cases {
                syntax_pattern_highlight(state, &case.pattern);
                if let Some(result) = &case.result {
                    syntax_expression_highlight(state, expressions, result);
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
    if let Some((type_parameter0, type_parameter1_up)) = project_fn.type_parameters.split_first() {
        formatted.push('<');
        formatted.push_str(type_parameter0);
        for angled_type_parameter in type_parameter1_up {
            formatted.push(' ');
            formatted.push_str(angled_type_parameter);
        }
        formatted.push('>');
    }
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
pub fn checked_type_alias_format(
    formatted: &mut String,
    name: &Name,
    type_alias: &CheckedTypeAlias,
) {
    formatted.push_str("ty ");
    formatted.push_str(name);
    if let Some((parameter0, parameter1_up)) = type_alias.parameters.split_first() {
        formatted.push_str(parameter0);
        for parameter in parameter1_up {
            formatted.push(' ');
            formatted.push_str(parameter);
        }
    }
    formatted.push_str("\n    ");
    if let Some(type_) = &type_alias.type_ {
        type_format(formatted, 4, type_);
    }
}
pub fn type_format_single_line(formatted: &mut String, type_: &Type) {
    match type_ {
        Type::Variable(name) => {
            formatted.push_str(name);
        }
        Type::CoreConstruct { name, arguments } | Type::Alias { name, arguments } => {
            formatted.push_str(name);
            if let Some((argument0, argument1_up)) = arguments.split_first() {
                formatted.push('(');
                type_format_single_line(formatted, argument0);
                for argument in argument1_up {
                    formatted.push(' ');
                    type_format_single_line(formatted, argument);
                }
                formatted.push(')');
            }
        }
        Type::Fn { parameters, result } => {
            formatted.push_str("fn(");
            if let Some((parameter0, parameter1_up)) = parameters.split_first() {
                type_format_single_line(formatted, parameter0);
                for parameter in parameter1_up {
                    formatted.push(' ');
                    type_format_single_line(formatted, parameter);
                }
            }
            formatted.push_str(") ");
            type_format_single_line(formatted, result);
        }
        Type::Tuple(parts) => {
            formatted.push('*');
            if let Some((part0, part1_up)) = parts.split_first() {
                formatted.push('(');
                type_format_single_line(formatted, part0);
                for part in part1_up {
                    formatted.push(' ');
                    type_format_single_line(formatted, part);
                }
                formatted.push(')');
            }
        }
        Type::Record(fields) => {
            formatted.push('{');
            if let Some((field0, field1_up)) = fields.split_first() {
                type_field_format_single_line(formatted, field0);
                for field in field1_up {
                    formatted.push(' ');
                    type_field_format_single_line(formatted, field);
                }
            }
            formatted.push('}');
        }
        Type::Choice(variants) => {
            formatted.push('[');
            if let Some((variant0, variant1_up)) = variants.split_first() {
                type_variant_format_single_line(formatted, variant0);
                for variant in variant1_up {
                    formatted.push(' ');
                    type_variant_format_single_line(formatted, variant);
                }
            }
            formatted.push(']');
        }
    }
}
fn type_field_format_single_line(formatted: &mut String, type_field: &TypeField) {
    formatted.push_str(&type_field.name);
    formatted.push(' ');
    type_format_single_line(formatted, &type_field.value);
}
fn type_variant_format_single_line(formatted: &mut String, type_variant: &TypeVariant) {
    formatted.push_str(&type_variant.name);
    if let Some((value0, value1_up)) = type_variant.values.split_first() {
        formatted.push('(');
        type_format_single_line(formatted, value0);
        for variant_value in value1_up {
            formatted.push(' ');
            type_format_single_line(formatted, variant_value);
        }
        formatted.push(')');
    }
}
pub fn type_format(formatted: &mut String, indent: usize, type_: &Type) {
    match type_ {
        Type::Variable(name) => {
            formatted.push_str(name);
        }
        Type::CoreConstruct { name, arguments } | Type::Alias { name, arguments } => {
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
                        linebreak_indented_into(formatted, indent);
                    }
                    type_variant_format(formatted, indent, variant0);
                    for variant in variant1_up {
                        space_or_linebreak_indented_into(formatted, line_span, indent);
                        type_variant_format(formatted, indent, variant);
                    }
                    if line_span == LineSpan::Multiple {
                        linebreak_indented_into(formatted, indent);
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
            if line_span == LineSpan::Multiple {
                linebreak_indented_into(formatted, indent);
            }
            type_field_format(formatted, indent, field0);
            for field in field1_up {
                space_or_linebreak_indented_into(formatted, line_span, indent);
                type_field_format(formatted, indent, field);
            }
            if line_span == LineSpan::Multiple {
                linebreak_indented_into(formatted, indent);
            }
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
    if let Some((value0, value1_up)) = type_variant.values.split_first() {
        formatted.push('(');
        let line_span = type_info_length_estimate_line_span(
            type_variant.name.len()
                + type_variant
                    .values
                    .iter()
                    .map(|type_variant_value| 1 + type_length_estimate(type_variant_value))
                    .sum::<usize>(),
        );
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, next_indent(indent));
        }
        type_format(formatted, next_indent(indent), value0);
        for variant_value in value1_up {
            space_or_linebreak_indented_into(formatted, line_span, next_indent(indent));
            type_format(formatted, next_indent(indent), variant_value);
        }
        if line_span == LineSpan::Multiple {
            linebreak_indented_into(formatted, indent);
        }
        formatted.push(')');
    }
}
fn type_line_span(type_: &Type) -> LineSpan {
    type_info_length_estimate_line_span(type_length_estimate(type_))
}
fn type_info_length_estimate_line_span(type_info_length_estimate: usize) -> LineSpan {
    if type_info_length_estimate <= type_info_line_length_estimate_maximum {
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
        Type::CoreConstruct { name, arguments } | Type::Alias { name, arguments } => {
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
            .map(|variant| {
                variant.name.len()
                    + variant
                        .values
                        .iter()
                        .map(|variant_value| 2 + type_length_estimate(variant_value))
                        .sum::<usize>()
            })
            .sum(),
    }
}
fn type_record_length_estimate(fields: &[TypeField]) -> usize {
    4 + fields
        .iter()
        .map(|field| 2 + field.name.len() + type_length_estimate(&field.value))
        .sum::<usize>()
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
            {
                let vertex_mut = &mut state.vertex_state[vertex];
                vertex_mut.index = state.index;
                vertex_mut.low_link = state.index;
                state.index += 1;
                state.stack.push(vertex);
                vertex_mut.is_on_stack = true;
            }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slot<Origin> {
    pub index: u32,
    pub origin: std::marker::PhantomData<Origin>,
}
#[derive(Debug)]
pub struct Buf<Origin, Item> {
    pub vec: Vec<Item>,
    pub origin: std::marker::PhantomData<Origin>,
}
impl<Origin, Item> Buf<Origin, Item> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            origin: std::marker::PhantomData::<Origin>,
        }
    }
    pub fn pre_allocate_at_least_usize(&mut self, spare: usize) {
        self.vec.reserve(spare);
    }
    pub fn insert(&mut self, new_item: Item) -> Slot<Origin> {
        let new_slot = Slot {
            index: self.vec.len() as u32,
            origin: std::marker::PhantomData::<Origin>,
        };
        self.vec.push(new_item);
        new_slot
    }
    pub fn item<'a>(&'a self, slot: &'a Slot<Origin>) -> &'a Item {
        &self.vec[slot.index as usize]
    }
    /// careful, this invalidates all slots
    pub fn clear(&mut self) {
        self.vec.clear();
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
            let core_ty_name = name_to_uppercase_js_segment(core_fn_name);
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
