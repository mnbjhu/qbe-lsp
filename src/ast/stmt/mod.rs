use assign::AssignAst;
use gibberish_core::node::Group;
use qbe_gibberish_parser::{Qbe, QbeSyntax};

use crate::semantic_analyze::Type;

use super::{instr::InstrAst, CheckState};

pub mod assign;

pub enum StmtAst<'a> {
    Assign(AssignAst<'a>),
    Instr(InstrAst<'a>),
}

impl<'a> StmtAst<'a> {
    pub fn check(&self, state: &mut CheckState) {
        dbg!("Checking stmt");
        match self {
            StmtAst::Assign(assign) => assign.check(state),
            StmtAst::Instr(instr) => instr.check(&Type::Unknown, state),
        }
    }
}

impl<'a> From<&'a Group<Qbe>> for StmtAst<'a> {
    fn from(value: &'a Group<Qbe>) -> Self {
        if value.kind == QbeSyntax::Assignment {
            StmtAst::Assign(AssignAst(value))
        } else {
            StmtAst::Instr(InstrAst(value))
        }
    }
}
