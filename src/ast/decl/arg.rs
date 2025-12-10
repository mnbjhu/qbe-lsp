use gibberish_core::node::{Group, Lexeme};
use qbe_gibberish_parser::{Qbe, QbeSyntax, QbeToken};

use crate::ast::common::ty::TypeAst;

pub struct ArgAst<'a>(pub &'a Group<Qbe>);

impl<'a> TryFrom<&'a Group<Qbe>> for ArgAst<'a> {
    type Error = ();

    fn try_from(value: &'a Group<Qbe>) -> Result<Self, Self::Error> {
        assert_eq!(value.kind, QbeSyntax::FuncArg);
        if value.lexeme_by_kind(QbeToken::Temp).is_some() {
            Ok(ArgAst(value))
        } else {
            Err(())
        }
    }
}

impl<'a> ArgAst<'a> {
    pub fn ty(&self) -> TypeAst<'a> {
        TypeAst::try_from(self.0.green_node_by_name(QbeSyntax::Ty).unwrap()).unwrap()
    }

    pub fn name(&self) -> &'a Lexeme<Qbe> {
        self.0.lexeme_by_kind(QbeToken::Temp).unwrap()
    }
}
