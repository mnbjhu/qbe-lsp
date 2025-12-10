use gibberish_core::node::{Group, Lexeme, Node, Span};
use qbe_gibberish_parser::{Qbe, QbeSyntax, QbeToken};

use crate::semantic_analyze::Type;

use super::CheckState;

pub enum ExprAst<'a> {
    Immediate(&'a Lexeme<Qbe>),
    Temp(&'a Lexeme<Qbe>),
    Global(&'a Lexeme<Qbe>),
    Label(&'a Lexeme<Qbe>),
}

impl<'a> TryFrom<&'a Group<Qbe>> for ExprAst<'a> {
    type Error = ();

    fn try_from(value: &'a Group<Qbe>) -> Result<Self, Self::Error> {
        if value.kind != QbeSyntax::Expr {
            return Err(());
        }
        let Node::Lexeme(l) = value.children.first().unwrap() else {
            panic!()
        };
        match l.kind {
            QbeToken::Int => Ok(ExprAst::Immediate(l)),
            QbeToken::Temp => Ok(ExprAst::Temp(l)),
            QbeToken::Global => Ok(ExprAst::Global(l)),
            QbeToken::Label => Ok(ExprAst::Label(l)),
            kind => {
                dbg!("Found error expr", kind);
                Err(())
            }
        }
    }
}

impl<'a> ExprAst<'a> {
    pub fn get_type(&self, state: &mut CheckState) -> Type {
        match self {
            ExprAst::Immediate(_) => Type::AnyInt, // TODO: Check this
            ExprAst::Temp(lexeme) => state
                .temps
                .get(&lexeme.text)
                .map(|(ty, _)| ty.clone())
                .unwrap_or_else(|| {
                    dbg!("Temp not defined", &lexeme.text);
                    state.error(
                        format!("Temp not defined '{}'", lexeme.text),
                        lexeme.span.clone(),
                    );
                    Type::Unknown
                }),
            ExprAst::Global(_) => Type::Long,
            ExprAst::Label(lexeme) => {
                state.label_refs.push(Lexeme::clone(lexeme));
                Type::Label
            }
        }
    }
    pub fn lexeme(&self) -> &'a Lexeme<Qbe> {
        match self {
            ExprAst::Immediate(lexeme) => lexeme,
            ExprAst::Temp(lexeme) => lexeme,
            ExprAst::Global(lexeme) => lexeme,
            ExprAst::Label(lexeme) => lexeme,
        }
    }

    pub fn span(&self) -> Span {
        let (ExprAst::Immediate(g) | ExprAst::Temp(g) | ExprAst::Global(g) | ExprAst::Label(g)) =
            self;
        g.span.clone()
    }
}
