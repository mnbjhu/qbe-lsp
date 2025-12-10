use gibberish_core::node::{Group, Node};
use qbe_gibberish_parser::Qbe;

use crate::{
    ast::{expr::ExprAst, CheckState},
    semantic_analyze::Type,
};

pub struct AddInstrAst<'a>(pub &'a Group<Qbe>);

impl<'a> AddInstrAst<'a> {
    fn args(&self) -> impl Iterator<Item = ExprAst<'a>> {
        self.0.children.iter().filter_map(|it| {
            if let Node::Lexeme(l) = it {
                // l.try_into().ok()
                todo!()
            } else {
                None
            }
        })
    }
    pub fn first(&self) -> Option<ExprAst<'a>> {
        self.args().next()
    }

    pub fn second(&self) -> Option<ExprAst<'a>> {
        self.args().nth(1)
    }

    pub fn check(&self, expected: &Type, state: &mut CheckState) {
        if let Some(first) = self.first() {
            let ty = first.get_type(state);
            if !ty.is_sub_type_of(expected) {
                state.error(format!("Expected {expected} but found {ty}"), first.span());
            }
        }
        if let Some(second) = self.second() {
            let ty = second.get_type(state);
            if !ty.is_sub_type_of(expected) {
                state.error(format!("Expected {expected} but found {ty}"), second.span());
            }
        }
    }
}
