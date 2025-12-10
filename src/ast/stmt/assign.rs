use gibberish_core::node::{Group, Lexeme};
use qbe_gibberish_parser::{Qbe, QbeSyntax, QbeToken};

use crate::{
    ast::{instr::InstrAst, CheckState},
    semantic_analyze::Type,
};

pub struct AssignAst<'a>(pub &'a Group<Qbe>);

impl<'a> AssignAst<'a> {
    pub fn name(&self) -> &'a Lexeme<Qbe> {
        self.0.lexeme_by_kind(QbeToken::Temp).unwrap()
    }

    pub fn ty(&self) -> Option<&'a Lexeme<Qbe>> {
        self.0
            .lexeme_by_kind(QbeToken::Eqb)
            .or(self.0.lexeme_by_kind(QbeToken::Eqw))
            .or(self.0.lexeme_by_kind(QbeToken::Eql))
            .or(self.0.lexeme_by_kind(QbeToken::TypeName))
    }

    pub fn instr(&self) -> Option<InstrAst<'a>> {
        self.0.green_node_by_name(QbeSyntax::Instr).map(InstrAst)
    }

    pub fn check(&self, state: &mut CheckState) {
        let ty = Type::from(self.ty());
        state
            .temps
            .insert(self.name().text.clone(), (ty.clone(), self.0.span()));
        if let Some(instr) = self.instr() {
            instr.check(&ty, state);
        }
    }
}

impl<'a> From<Option<&'a Lexeme<Qbe>>> for Type {
    fn from(value: Option<&'a Lexeme<Qbe>>) -> Self {
        match value {
            Some(l) => match l.kind {
                QbeToken::Eqb => Type::Byte,
                QbeToken::Eqw => Type::Word,
                QbeToken::Eql => Type::Long,
                QbeToken::TypeName => Type::Custom(l.text.clone()),
                _ => panic!(),
            },
            None => Type::Unknown,
        }
    }
}
