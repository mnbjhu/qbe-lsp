use gibberish_core::node::{Group, Lexeme};
use qbe_gibberish_parser::{Qbe, QbeToken};

use crate::ast::{stmt::StmtAst, CheckState, LspItem, LspNode};

#[derive(Clone)]
pub struct BlockAst<'a>(pub &'a Group<Qbe>);

impl<'a> BlockAst<'a> {
    pub fn label(&self) -> &'a Lexeme<Qbe> {
        self.0.token_by_kind(QbeToken::Label).unwrap()
    }

    pub fn stmts(&self) -> impl Iterator<Item = StmtAst<'a>> {
        self.0.groups().map(StmtAst::from)
    }
    pub fn check(&self, state: &mut CheckState<'a>) {
        let name = self.label();
        if state.labels.contains_key(&name.text) {
            state.error(
                "Duplicate label name".to_string(),
                self.label().span.clone(),
            );
        } else {
            state
                .labels
                .insert(name.text.to_string(), name.span.clone());
        }
        self.stmts().for_each(|it| it.check(state));
    }
}

impl<'a> LspItem<'a> for BlockAst<'a> {
    fn at(&self, offset: usize) -> Option<LspNode<'a>> {
        if self.0.span().contains(&offset) {
            if self.label().span.contains(&offset) {
                return Some(LspNode::Label(self.label()));
            }
            for stmt in self.stmts() {
                if let Some(stmt) = stmt.at(offset) {
                    return Some(stmt);
                }
            }
            Some(LspNode::Block(self.clone()))
        } else {
            None
        }
    }
}
