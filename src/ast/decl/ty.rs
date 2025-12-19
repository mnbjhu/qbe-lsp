use gibberish_core::node::Group;
use qbe_gibberish_parser::{Qbe, QbeToken};

pub struct TypeDefAst<'a>(pub &'a Group<Qbe>);

impl<'a> TypeDefAst<'a> {
    pub fn name(&self) {
        self.0.token_by_kind(QbeToken::TypeName);
    }
}
