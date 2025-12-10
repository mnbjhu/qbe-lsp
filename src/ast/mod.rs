use decl::DeclAst;
use gibberish_core::node::{Group, Lexeme};
use im_rc::HashMap;
use instr::FunctionType;
use qbe_gibberish_parser::Qbe;
use tower_lsp::lsp_types::DiagnosticSeverity;

use crate::{semantic_analyze::Type, span::Span};

pub mod common;
pub mod decl;
pub mod expr;
pub mod instr;
pub mod stmt;

pub struct QbeAst<'a>(pub &'a Group<Qbe>);

impl<'a> QbeAst<'a> {
    pub fn decls(&self) -> impl Iterator<Item = DeclAst<'a>> {
        self.0
            .green_children()
            .filter_map(|it| DeclAst::try_from(it).ok())
    }

    pub fn check(&self, state: &mut CheckState) {
        dbg!("Checking root");
        self.decls().for_each(|it| {
            it.check(state);
            state.clear_locals();
        });
    }
}

pub struct CheckError {
    pub message: String,
    pub span: Span,
    pub severity: DiagnosticSeverity,
}

#[derive(Default)]
pub struct CheckState {
    pub temps: HashMap<String, (Type, Span)>,
    pub labels: HashMap<String, Span>,
    pub label_refs: Vec<Lexeme<Qbe>>,
    pub function_defs: HashMap<String, (FunctionType, Span)>,
    pub errors: Vec<CheckError>,
}

impl CheckState {
    pub fn clear_locals(&mut self) {
        for label_ref in self.label_refs.clone() {
            if !self.labels.contains_key(&label_ref.text) {
                self.error(
                    format!("Label not defined '{}'", label_ref.text),
                    label_ref.span,
                );
            }
        }
        self.labels.clear();
        self.temps.clear();
    }

    pub fn error(&mut self, message: String, span: Span) {
        self.errors.push(CheckError {
            message,
            span,
            severity: DiagnosticSeverity::ERROR,
        });
    }

    pub fn warn(&mut self, message: String, span: Span) {
        self.errors.push(CheckError {
            message,
            span,
            severity: DiagnosticSeverity::WARNING,
        });
    }
}
