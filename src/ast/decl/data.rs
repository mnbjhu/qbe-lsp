use gibberish_core::node::Group;
use qbe_gibberish_parser::{Qbe, QbeToken};

pub struct DataAst<'a>(pub &'a Group<Qbe>);

impl<'a> DataAst<'a> {
    pub fn name(&self) {
        self.0.lexeme_by_kind(QbeToken::Global);
    }
}
