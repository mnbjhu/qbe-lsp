use gibberish_core::node::{Group, Lexeme, Node};
use qbe_gibberish_parser::{Qbe, QbeSyntax, QbeToken};

use crate::{
    ast::{LspItem, LspNode},
    semantic_analyze::Type,
};

#[derive(Clone)]
pub enum TypeAst<'a> {
    Long(&'a Lexeme<Qbe>),
    Word(&'a Lexeme<Qbe>),
    Byte(&'a Lexeme<Qbe>),
    Custom(&'a Lexeme<Qbe>),
}

impl<'a> TryFrom<&'a Group<Qbe>> for TypeAst<'a> {
    type Error = ();

    fn try_from(value: &'a Group<Qbe>) -> Result<Self, Self::Error> {
        if value.kind != QbeSyntax::Ty {
            return Err(());
        }
        let Node::Lexeme(l) = value.children.first().unwrap() else {
            panic!()
        };
        match l.kind {
            QbeToken::L => Ok(TypeAst::Long(l)),
            QbeToken::W => Ok(TypeAst::Word(l)),
            QbeToken::B => Ok(TypeAst::Byte(l)),
            QbeToken::TypeName => Ok(TypeAst::Custom(l)),
            kind => Err(()),
        }
    }
}

impl<'a> TypeAst<'a> {
    pub fn lexeme(&self) -> &'a Lexeme<Qbe> {
        let (TypeAst::Long(lexeme)
        | TypeAst::Word(lexeme)
        | TypeAst::Byte(lexeme)
        | TypeAst::Custom(lexeme)) = self;
        lexeme
    }
}

impl<'a> From<TypeAst<'a>> for Type {
    fn from(value: TypeAst) -> Self {
        match value {
            TypeAst::Long(_) => Type::Long,
            TypeAst::Word(_) => Type::Word,
            TypeAst::Byte(_) => Type::Byte,
            TypeAst::Custom(lexeme) => Type::Custom(lexeme.text.clone()),
        }
    }
}

impl<'a> LspItem<'a> for TypeAst<'a> {
    fn at(&self, offset: usize) -> Option<LspNode<'a>> {
        if self.lexeme().span.contains(&offset) {
            Some(LspNode::Type(self.clone()))
        } else {
            None
        }
    }
}
