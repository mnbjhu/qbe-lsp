use decl::DeclAst;
use gibberish_core::node::Group;
use qbe_gibberish_parser::Qbe;

pub mod common;
pub mod decl;

pub struct QbeAst<'a>(&'a Group<Qbe>);

impl<'a> QbeAst<'a> {
    pub fn decls(&self) -> impl Iterator<Item = DeclAst<'a>> {
        self.0
            .green_children()
            .filter_map(|it| DeclAst::try_from(it).ok())
    }
}
