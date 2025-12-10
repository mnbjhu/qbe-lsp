use gibberish_core::node::{Group, Lexeme, Node};
use qbe_gibberish_parser::{Qbe, QbeToken};

use crate::semantic_analyze::Type;

use super::{expr::ExprAst, CheckState};

pub mod add;

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
        self.0.green_children().filter_map(|it| it.try_into().ok())
    }

    pub fn check(&self, expected: &Type, state: &mut CheckState) {
        dbg!("Checking instr");
        if let Some(def) = get_function_type(self.kind().kind) {
            let ty = def.map(expected);
            let args = self.args().map(|it| it.get_type(state)).collect::<Vec<_>>();
            for (index, arg_ast) in self.args().enumerate() {
                let arg_ty = &args[index];
                let expected_ty = ty.args.get(index);
                if let Some(expected) = expected_ty {
                    if !arg_ty.is_sub_type_of(expected) {
                        state.error(
                            format!("Expected {expected} but found {arg_ty}"),
                            arg_ast.lexeme().span.clone(),
                        );
                    }
                } else {
                    state.error(
                        "This argument is unexpected".to_string(),
                        arg_ast.lexeme().span.clone(),
                    );
                }
            }
            if ty.args.len() > args.len() {
                state.error(
                    format!(
                        "Expected {} args but {} were found",
                        ty.args.len(),
                        args.len()
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
