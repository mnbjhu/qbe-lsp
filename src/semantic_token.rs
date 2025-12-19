use std::collections::HashMap;

use log::info;
use qbe_gibberish_parser::QbeToken;
use tower_lsp::lsp_types::SemanticTokenType;

use crate::ast::QbeAst;

#[derive(Debug)]
pub struct ImCompleteSemanticToken {
    pub start: usize,
    pub length: usize,
    pub token_type: usize,
}

pub const LEGEND_TYPE: &[SemanticTokenType] = &[
    SemanticTokenType::FUNCTION,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::STRING,
    SemanticTokenType::COMMENT,
    SemanticTokenType::NUMBER,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::TYPE,
    SemanticTokenType::DECORATOR,
    SemanticTokenType::PROPERTY,
];

pub fn semantic_token_from_ast(ast: &QbeAst) -> Vec<ImCompleteSemanticToken> {
    let mut semantic_tokens = vec![];

    ast.0.all_tokens().for_each(|it| {
        let kind = match it.kind {
            QbeToken::Global => Some(SemanticTokenType::PROPERTY),
            QbeToken::TypeName | QbeToken::L | QbeToken::W | QbeToken::B => {
                Some(SemanticTokenType::TYPE)
            }
            QbeToken::String => Some(SemanticTokenType::STRING),
            QbeToken::Int => Some(SemanticTokenType::NUMBER),
            QbeToken::Comment => Some(SemanticTokenType::COMMENT),
            QbeToken::Label => Some(SemanticTokenType::DECORATOR),
            QbeToken::Newline | QbeToken::Ws | QbeToken::Temp => None,
            QbeToken::Comma
            | QbeToken::Eq
            | QbeToken::Eql
            | QbeToken::Eqw
            | QbeToken::Eqb
            | QbeToken::LBrace
            | QbeToken::RBrace
            | QbeToken::LParen
            | QbeToken::RParen => None,
            _ => Some(SemanticTokenType::KEYWORD),
        };
        if let Some(kind) = kind {
            semantic_tokens.push(ImCompleteSemanticToken {
                start: it.span.start,
                length: it.span.len(),
                token_type: LEGEND_TYPE.iter().position(|item| item == &kind).unwrap(),
            });
        }
    });

    semantic_tokens
}
