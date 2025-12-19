use gibberish_core::node::{Group, Lexeme};
use qbe_gibberish_parser::{Qbe, QbeToken};

use crate::ast::{CheckState, Definition};

pub struct DataAst<'a>(pub &'a Group<Qbe>);

impl<'a> DataAst<'a> {
    pub fn name(&self) -> Option<&'a Lexeme<Qbe>> {
        self.0.token_by_kind(QbeToken::Global)
    }

    pub fn check(&self, state: &mut CheckState<'a>) {
        let Some(name) = self.name() else {
            return;
        };
        if state.data_defs.contains_key(&name.text) || state.function_defs.contains_key(&name.text)
        {
            state.error(
                format!("Duplicate definition of {}", name.text),
                name.span.clone(),
            );
        } else {
            state
                .data_defs
                .insert(name.text.clone(), Definition::new(name.span.clone()));
        }
    }
}
