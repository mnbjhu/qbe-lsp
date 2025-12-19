use gibberish_core::node::{Group, Lexeme};
use qbe_gibberish_parser::{Qbe, QbeSyntax, QbeToken};

use crate::ast::{common::ty::TypeAst, LspItem, LspNode};

#[derive(Clone)]
pub struct ArgAst<'a>(pub &'a Group<Qbe>);

impl<'a> TryFrom<&'a Group<Qbe>> for ArgAst<'a> {
    type Error = ();

    fn try_from(value: &'a Group<Qbe>) -> Result<Self, Self::Error> {
        assert_eq!(value.kind, QbeSyntax::FuncArg);
        if value.token_by_kind(QbeToken::Temp).is_some() {
            Ok(ArgAst(value))
        } else {
            Err(())
        }
    }
}

impl<'a> ArgAst<'a> {
    pub fn ty(&self) -> TypeAst<'a> {
        TypeAst::try_from(self.0.group_by_kind(QbeSyntax::Ty).unwrap()).unwrap()
    }

    pub fn name(&self) -> &'a Lexeme<Qbe> {
        self.0.token_by_kind(QbeToken::Temp).unwrap()
    }
}

impl<'a> LspItem<'a> for ArgAst<'a> {
    fn at(&self, offset: usize) -> Option<LspNode<'a>> {
        if self.0.span().contains(&offset) {
            let res = if let Some(ty) = self.ty().at(offset) {
                ty
            } else if self.name().span.contains(&offset) {
                LspNode::FunctionParam(self.name())
            } else {
                LspNode::Arg(self.clone())
            };
            Some(res)
        } else {
            None
        }
    }
}
