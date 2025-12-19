use gibberish_core::node::{Group, Lexeme};
use qbe_gibberish_parser::{Qbe, QbeSyntax, QbeToken};

use crate::{
    ast::{common::ty::TypeAst, instr::FunctionType, CheckState, LspItem, LspNode},
    semantic_analyze::Type,
};

use super::{arg::ArgAst, block::BlockAst};

#[derive(Clone)]
pub struct FunctionAst<'a>(pub &'a Group<Qbe>);

impl<'a> FunctionAst<'a> {
    pub fn name(&self) -> Option<&'a Lexeme<Qbe>> {
        self.0.token_by_kind(QbeToken::Global)
    }

    pub fn return_ty(&self) -> Option<TypeAst<'a>> {
        self.0
            .group_by_kind(QbeSyntax::Ty)
            .map(|it| it.try_into().unwrap())
    }

    pub fn args(&self) -> impl Iterator<Item = ArgAst<'a>> {
        let res: Box<dyn Iterator<Item = ArgAst<'_>>> =
            if let Some(args) = self.0.group_by_kind(QbeSyntax::FuncArgs) {
                Box::new(args.groups().filter_map(|it| ArgAst::try_from(it).ok()))
            } else {
                Box::new(std::iter::empty())
            };
        res
    }

    pub fn body(&self) -> impl Iterator<Item = BlockAst<'a>> {
        let res: Box<dyn Iterator<Item = BlockAst<'_>>> =
            if let Some(args) = self.0.group_by_kind(QbeSyntax::FuncBody) {
                Box::new(args.groups().map(BlockAst))
            } else {
                Box::new(std::iter::empty())
            };
        res
    }

    pub fn check(&self, state: &mut CheckState<'a>) {
        let ret = self.return_ty().map(Type::from).unwrap_or(Type::Unit);
        state.return_ty = Some(ret.clone());
        let mut def = FunctionType {
            return_ty: ret,
            args: vec![],
        };
        for arg in self.args() {
            let ty = Type::from(arg.ty());
            def.args.push(ty.clone());
            let name = arg.name();
            state
                .temps
                .insert(name.text.clone(), (ty, name.span.clone()));
        }
        self.body().for_each(|it| it.check(state));
        if let Some(name) = self.name().cloned() {
            if state.data_defs.contains_key(&name.text)
                || state.function_defs.contains_key(&name.text)
            {
                state.error(
                    format!("Duplicate definition of {}", name.text),
                    name.span.clone(),
                );
            } else {
                state
                    .function_defs
                    .insert(name.text, (def, Some(name.span)));
            }
        }
    }
}

impl<'a> LspItem<'a> for FunctionAst<'a> {
    fn at(&self, offset: usize) -> Option<LspNode<'a>> {
        if self.0.span().contains(&offset) {
            for arg in self.args() {
                let res = arg.at(offset);
                if res.is_some() {
                    return res;
                }
            }
            for block in self.body() {
                let res = block.at(offset);
                if res.is_some() {
                    return res;
                }
            }
            Some(LspNode::Function(self.clone()))
        } else {
            None
        }
    }
}
