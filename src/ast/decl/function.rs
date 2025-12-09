use gibberish_core::node::Group;
use qbe_gibberish_parser::{Qbe, QbeToken};

use super::arg::ArgAst;

pub struct FunctionAst<'a>(pub &'a Group<Qbe>);

impl<'a> FunctionAst<'a> {
    pub fn name(&self) {
        self.0.lexeme_by_kind(QbeToken::Global);
    }

    // pub fn args(&self) -> impl Iterator<Item = ArgAst<'a>> {
    //     self.0
    //         .green_children()
    //         .filter_map(|it| ArgAst::try_from(it).ok())
    //     todo!()
    // }
}
