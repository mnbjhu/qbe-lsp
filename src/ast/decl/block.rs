use gibberish_core::node::{Group, Lexeme};
use qbe_gibberish_parser::{Qbe, QbeToken};

use crate::ast::{stmt::StmtAst, CheckState};

pub struct BlockAst<'a>(pub &'a Group<Qbe>);

impl<'a> BlockAst<'a> {
    pub fn label(&self) -> &'a Lexeme<Qbe> {
        self.0.lexeme_by_kind(QbeToken::Label).unwrap()
    }

    pub fn stmts(&self) -> impl Iterator<Item = StmtAst<'a>> {
        self.0.green_children().map(StmtAst::from)
    }
    pub fn check(&self, state: &mut CheckState) {
        let name = self.label();
        state
            .labels
            .insert(name.text.to_string(), name.span.clone());
        self.stmts().for_each(|it| it.check(state));
    }
}
