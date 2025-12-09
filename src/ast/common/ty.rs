use gibberish_core::node::Group;
use qbe_gibberish_parser::Qbe;

pub struct TypeAst<'a>(pub &'a Group<Qbe>);
