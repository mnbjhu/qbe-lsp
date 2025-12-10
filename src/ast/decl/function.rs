use gibberish_core::node::{Group, Lexeme};
use qbe_gibberish_parser::{Qbe, QbeSyntax, QbeToken};

use crate::ast::CheckState;

use super::{arg::ArgAst, block::BlockAst};

pub struct FunctionAst<'a>(pub &'a Group<Qbe>);

impl<'a> FunctionAst<'a> {
    pub fn name(&self) -> Option<&'a Lexeme<Qbe>> {
        self.0.lexeme_by_kind(QbeToken::Global)
    }

    pub fn args(&self) -> impl Iterator<Item = ArgAst<'a>> {
        let res: Box<dyn Iterator<Item = ArgAst<'_>>> =
            if let Some(args) = self.0.green_node_by_name(QbeSyntax::FuncArgs) {
                Box::new(
                    args.green_children()
                        .filter_map(|it| ArgAst::try_from(it).ok()),
                )
            } else {
                Box::new(std::iter::empty())
            };
        res
    }

    pub fn body(&self) -> impl Iterator<Item = BlockAst<'a>> {
        let res: Box<dyn Iterator<Item = BlockAst<'_>>> =
            if let Some(args) = self.0.green_node_by_name(QbeSyntax::FuncBody) {
                Box::new(args.green_children().map(BlockAst))
            } else {
                Box::new(std::iter::empty())
            };
        res
    }

    pub fn check(&self, state: &mut CheckState) {
        self.body().for_each(|it| it.check(state));
    }
}
