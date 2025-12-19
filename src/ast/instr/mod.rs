use call::CallAst;
use gibberish_core::node::{Group, Lexeme, Node};
use qbe_gibberish_parser::{Qbe, QbeToken};

use crate::semantic_analyze::Type;

use super::{expr::ExprAst, CheckState, LspItem, LspNode};

pub mod call;

#[derive(Clone)]
pub struct InstrAst<'a>(pub &'a Group<Qbe>);

impl<'a> InstrAst<'a> {
    pub fn kind(&self) -> &'a Lexeme<Qbe> {
        self.0
            .children
            .iter()
            .find_map(|it| match it {
                Node::Lexeme(lexeme)
                    if lexeme.kind != QbeToken::Ws && lexeme.kind != QbeToken::Newline =>
                {
                    Some(lexeme)
                }
                _ => None,
            })
            .unwrap()
    }

    fn args(&self) -> impl Iterator<Item = ExprAst<'a>> {
        self.0.groups().filter_map(|it| it.try_into().ok())
    }

    pub fn check(&self, expected: &Type, state: &mut CheckState<'a>) {
        if self.kind().kind == QbeToken::Call {
            let ast = CallAst(self.0);
            if let Some(args) = ast.args() {
                for arg in args {
                    arg.check(state);
                }
            }
            if ast.name().is_some() {
                state.function_refs.push((ast, expected.clone()));
            }
            return;
        }
        if self.kind().kind == QbeToken::Ret {
            let mut args = self.args();
            if let Some(ret) = state.return_ty.clone() {
                if let Some(first) = args.next() {
                    first.check(&ret, state);
                    for arg in args {
                        state.error(
                            "This argument is unexpected".to_string(),
                            arg.span().clone(),
                        );
                    }
                } else {
                    state.error("Expected 1 argument".to_string(), self.kind().span.clone());
                }
            } else {
                state.error(
                    "This function doesn't return anything".to_string(),
                    self.kind().span.clone(),
                );
            }
            return;
        }
        if let Some(def) = get_function_type(self.kind().kind) {
            let ty = def.map(expected);
            for (index, arg_ast) in self.args().enumerate() {
                let expected_ty = ty.args.get(index);
                if let Some(expected) = expected_ty {
                    arg_ast.check(expected, state);
                } else {
                    state.error(
                        "This argument is unexpected".to_string(),
                        arg_ast.lexeme().span.clone(),
                    );
                }
            }
            let args_len = self.args().count();
            if ty.args.len() > args_len {
                state.error(
                    format!(
                        "Expected {} args but {} were found",
                        ty.args.len(),
                        args_len
                    ),
                    self.kind().span.clone(),
                );
            }
        } else {
            state.warn(
                "Instrution not implemented".to_string(),
                self.kind().span.clone(),
            );
        }
    }
}

#[derive(Clone)]
pub struct FunctionType {
    pub return_ty: Type,
    pub args: Vec<Type>,
}

impl FunctionType {
    pub fn map(&self, return_ty: &Type) -> Self {
        if let Type::T = self.return_ty {
            FunctionType {
                args: self
                    .args
                    .iter()
                    .map(|it| {
                        if let Type::T = &self.return_ty {
                            return_ty.clone()
                        } else {
                            it.clone()
                        }
                    })
                    .collect(),
                return_ty: return_ty.clone(),
            }
        } else {
            self.clone()
        }
    }

    pub fn parameterized(&self) -> bool {
        matches!(self.return_ty, Type::T) || self.args.iter().any(|it| matches!(it, Type::T))
    }
}

fn get_function_type(kind: QbeToken) -> Option<FunctionType> {
    match kind {
        QbeToken::Add | QbeToken::Sub | QbeToken::Mul => Some(FunctionType {
            return_ty: Type::T,
            args: vec![Type::T, Type::T],
        }),
        QbeToken::Ceql => Some(FunctionType {
            return_ty: Type::T,
            args: vec![Type::Long, Type::Long],
        }),
        QbeToken::Copy => Some(FunctionType {
            return_ty: Type::T,
            args: vec![Type::T],
        }),
        QbeToken::Loadl => Some(FunctionType {
            return_ty: Type::Long,
            args: vec![Type::Long],
        }),
        QbeToken::Loadw => Some(FunctionType {
            return_ty: Type::Word,
            args: vec![Type::Long],
        }),
        QbeToken::Storel => Some(FunctionType {
            return_ty: Type::Unit,
            args: vec![Type::Long, Type::Long],
        }),
        QbeToken::Storew => Some(FunctionType {
            return_ty: Type::Unit,
            args: vec![Type::Word, Type::Long],
        }),
        QbeToken::Jmp => Some(FunctionType {
            return_ty: Type::Unit,
            args: vec![Type::Label],
        }),
        QbeToken::Jnz => Some(FunctionType {
            return_ty: Type::Unit,
            args: vec![Type::T, Type::Label, Type::Label],
        }),
        _ => None,
    }
}

impl<'a> LspItem<'a> for InstrAst<'a> {
    fn at(&self, offset: usize) -> Option<LspNode<'a>> {
        if self.0.span().contains(&offset) {
            for arg in self.args() {
                if arg.span().contains(&offset) {
                    return Some(LspNode::Expr(arg));
                }
            }
            Some(LspNode::Instr(self.clone()))
        } else {
            None
        }
    }
}
