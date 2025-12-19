use std::fmt::Display;

use common::ty::TypeAst;
use decl::{arg::ArgAst, block::BlockAst, function::FunctionAst, DeclAst};
use expr::ExprAst;
use gibberish_core::node::{Group, Lexeme};
use im_rc::HashMap;
use instr::{call::CallAst, FunctionType, InstrAst};
use qbe_gibberish_parser::Qbe;
use stmt::assign::AssignAst;
use tower_lsp::lsp_types::{DiagnosticSeverity, Hover, HoverContents, MarkedString};

use crate::{semantic_analyze::Type, span::Span};

pub mod common;
pub mod decl;
pub mod expr;
pub mod instr;
pub mod stmt;

#[derive(Clone)]
pub struct QbeAst<'a>(pub &'a Group<Qbe>);

impl<'a> QbeAst<'a> {
    pub fn decls(&self) -> impl Iterator<Item = DeclAst<'a>> {
        self.0.groups().filter_map(|it| DeclAst::try_from(it).ok())
    }

    pub fn check(&self, state: &mut CheckState<'a>) {
        self.decls().for_each(|it| {
            it.check(state);
            state.clear_locals();
        });
        for (ast, expected) in state.function_refs.clone() {
            ast.check(&expected, state)
        }
    }
}

pub struct CheckError {
    pub message: String,
    pub span: Span,
    pub severity: DiagnosticSeverity,
}

#[derive(Default)]
pub struct CheckState<'a> {
    pub temps: HashMap<String, (Type, Span)>,
    pub temp_refs: Vec<Lexeme<Qbe>>,
    pub labels: HashMap<String, Span>,
    pub label_refs: Vec<Lexeme<Qbe>>,
    pub function_defs: HashMap<String, (FunctionType, Option<Span>)>,
    pub function_refs: Vec<(CallAst<'a>, Type)>,
    pub data_defs: HashMap<String, Definition>,
    pub errors: Vec<CheckError>,
    pub return_ty: Option<Type>,
}

#[derive(Clone)]
pub struct Definition {
    pub definition: Span,
    pub references: Vec<Span>,
}

impl Definition {
    pub fn new(span: Span) -> Self {
        Definition {
            definition: span,
            references: vec![],
        }
    }
}

impl<'a> CheckState<'a> {
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
        self.label_refs.clear();
        self.temps.clear();
        self.return_ty = None;
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

pub trait LspItem<'a> {
    fn at(&self, offset: usize) -> Option<LspNode<'a>>;
}

impl<'a> LspNode<'a> {
    pub fn hover(&self, _: &CheckState) -> Option<HoverContents> {
        Some(HoverContents::Scalar(MarkedString::String(
            self.to_string(),
        )))
    }

    pub fn definition(&self, state: &CheckState) -> Option<Span> {
        match self {
            LspNode::Expr(ExprAst::Temp(temp)) => {
                state.temps.get(&temp.text).map(|it| it.1.clone())
            }
            LspNode::Expr(ExprAst::Label(label)) | LspNode::Label(label) => {
                state.labels.get(&label.text).map(|it| it.clone())
            }
            _ => None,
        }
    }

    pub fn references(&self, state: &CheckState) -> Vec<Span> {
        match self {
            LspNode::Expr(ExprAst::Temp(temp)) => state
                .temp_refs
                .iter()
                .filter(|it| it.text == temp.text)
                .map(|it| it.span.clone())
                .collect(),
            LspNode::Expr(ExprAst::Label(label)) | LspNode::Label(label) => state
                .label_refs
                .iter()
                .filter(|it| it.text == label.text)
                .map(|it| it.span.clone())
                .collect(),
            _ => vec![],
        }
    }
}

pub enum LspNode<'a> {
    Root(QbeAst<'a>),
    Function(FunctionAst<'a>),
    FunctionParam(&'a Lexeme<Qbe>),
    Instr(InstrAst<'a>),
    Assign(AssignAst<'a>),
    Arg(ArgAst<'a>),
    Expr(ExprAst<'a>),
    Type(TypeAst<'a>),
    Block(BlockAst<'a>),
    Label(&'a Lexeme<Qbe>),
}

impl Display for LspNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LspNode::Root(_) => write!(f, "Root"),
            LspNode::Function(_) => write!(f, "Function"),
            LspNode::FunctionParam(_) => write!(f, "FunctionParam"),
            LspNode::Instr(_) => write!(f, "Instr"),
            LspNode::Assign(_) => write!(f, "Assign"),
            LspNode::Arg(_) => write!(f, "Arg"),
            LspNode::Expr(_) => write!(f, "Expr"),
            LspNode::Type(_) => write!(f, "Type"),
            LspNode::Block(_) => write!(f, "Block"),
            LspNode::Label(_) => write!(f, "Label"),
        }
    }
}

impl<'a> LspItem<'a> for QbeAst<'a> {
    fn at(&self, offset: usize) -> Option<LspNode<'a>> {
        for decl in self.decls() {
            if let Some(res) = decl.at(offset) {
                return Some(res);
            }
        }
        Some(LspNode::Root(self.clone()))
    }
}
