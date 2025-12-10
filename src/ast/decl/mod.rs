use data::DataAst;
use function::FunctionAst;
use gibberish_core::node::Group;
use qbe_gibberish_parser::{Qbe, QbeSyntax, QbeToken};
use ty::TypeDefAst;

use super::CheckState;

pub mod arg;
pub mod block;
pub mod data;
pub mod function;
pub mod ty;

pub enum DeclAst<'a> {
    Function(FunctionAst<'a>),
    Data(DataAst<'a>),
    Type(TypeDefAst<'a>),
}

pub enum ParseDeclError {
    Unmatched,
    MissingName,
    Unexpected(QbeSyntax),
}

impl<'a> TryFrom<&'a Group<Qbe>> for DeclAst<'a> {
    type Error = ParseDeclError;

    fn try_from(value: &'a Group<Qbe>) -> Result<Self, Self::Error> {
        match value.kind {
            QbeSyntax::FunctionDef => {
                if value.lexeme_by_kind(QbeToken::Global).is_some() {
                    Ok(Self::Function(FunctionAst(value)))
                } else {
                    Err(ParseDeclError::MissingName)
                }
            }
            QbeSyntax::DataDef => {
                if value.lexeme_by_kind(QbeToken::Global).is_some() {
                    Ok(Self::Data(DataAst(value)))
                } else {
                    Err(ParseDeclError::MissingName)
                }
            }
            QbeSyntax::TypeDef => Ok(Self::Type(TypeDefAst(value))),
            QbeSyntax::Unmatched => Err(ParseDeclError::Unmatched),
            other => Err(ParseDeclError::Unexpected(other)),
        }
    }
}

impl<'a> DeclAst<'a> {
    pub fn check(&self, state: &mut CheckState) {
        match self {
            DeclAst::Function(f) => f.check(state),
            DeclAst::Data(d) => todo!(),
            DeclAst::Type(t) => todo!(),
        }
    }
}
