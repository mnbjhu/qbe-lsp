use gibberish_core::node::{Group, Lexeme, Span};
use im_rc::HashMap;
use qbe_gibberish_parser::{Qbe, QbeSyntax, QbeToken};

use crate::{
    ast::{common::ty::TypeAst, expr::ExprAst, CheckState},
    semantic_analyze::Type,
};

use super::FunctionType;

#[derive(Clone)]
pub struct CallAst<'a>(pub &'a Group<Qbe>);

impl<'a> CallAst<'a> {
    pub fn name(&self) -> Option<&'a Lexeme<Qbe>> {
        self.0.token_by_kind(QbeToken::Global)
    }

    pub fn args(&self) -> Option<impl Iterator<Item = CallArgAst<'a>>> {
        self.0.group_by_kind(QbeSyntax::CallArgs).map(|args| {
            args.groups()
                .filter(|it| it.token_by_kind(QbeToken::Ellipsis).is_none())
                .map(CallArgAst)
        })
    }

    pub fn check(&self, expected: &Type, state: &mut CheckState<'a>) {
        if let Some((def, _)) = state.function_defs.get(&self.name().unwrap().text) {
            let ty = def.map(expected);
            if self.args().is_some() {
                for (index, arg_ast) in self.args().unwrap().enumerate() {
                    let arg_ty: Type = arg_ast.ty().into();
                    let expected_ty = ty.args.get(index);
                    if let Some(expected) = expected_ty {
                        if !arg_ty.is_sub_type_of(expected) {
                            state.error(
                                format!("Expected {expected} but found {arg_ty}"),
                                arg_ast.0.span().clone(),
                            );
                        }
                    } else {
                        state.error(
                            "This argument is unexpected".to_string(),
                            arg_ast.0.span().clone(),
                        );
                    }
                    let arg_count = self.args().map(|it| it.count()).unwrap_or(0);
                    if ty.args.len() > arg_count {
                        state.error(
                            format!(
                                "Expected {} args but {} were found",
                                ty.args.len(),
                                arg_count
                            ),
                            self.name().unwrap().span.clone(),
                        );
                    }
                }
            }
        } else {
            state.error(
                "Function not found".to_string(),
                self.name().unwrap().span.clone(),
            );
        }
    }
}

pub struct CallArgAst<'a>(pub &'a Group<Qbe>);

impl<'a> CallArgAst<'a> {
    pub fn ty(&self) -> TypeAst<'a> {
        self.0
            .group_by_kind(QbeSyntax::Ty)
            .map(|it| it.try_into().unwrap())
            .unwrap()
    }
    pub fn expr(&self) -> Option<ExprAst<'a>> {
        self.0
            .group_by_kind(QbeSyntax::Expr)
            .map(|it| it.try_into().unwrap())
    }

    pub fn check(&self, state: &mut CheckState<'a>) {
        let ty: Type = self.ty().into();
        if let Some(expr) = self.expr() {
            expr.check(&ty, state);
        }
    }
}

pub fn default_functions() -> HashMap<String, (FunctionType, Option<Span>)> {
    let mut res = HashMap::new();
    res.insert(
        "$memcpy".to_string(),
        (
            FunctionType {
                return_ty: Type::Long,
                args: vec![Type::Long, Type::Long, Type::Long],
            },
            None,
        ),
    );
    res.insert(
        "$free".to_string(),
        (
            FunctionType {
                return_ty: Type::Unit,
                args: vec![Type::Long],
            },
            None,
        ),
    );
    res.insert(
        "$malloc".to_string(),
        (
            FunctionType {
                return_ty: Type::Unit,
                args: vec![Type::Long],
            },
            None,
        ),
    );
    res
}
