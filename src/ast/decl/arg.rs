use gibberish_core::node::Group;
use qbe_gibberish_parser::Qbe;

use crate::ast::common::ty::TypeAst;

pub struct ArgAst<'a>(pub &'a Group<Qbe>);

impl<'a> ArgAst<'a> {
    pub fn ty(&self) -> TypeAst<'a> {
        todo!()
    }
}
