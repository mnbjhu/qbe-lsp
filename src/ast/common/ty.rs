use gibberish_core::node::Group;
use qbe_gibberish_parser::Qbe;

pub struct TypeAst<'a>(pub &'a Group<Qbe>);

impl<'a> From<&'a Group<Qbe>> for TypeAst<'a> {
    fn from(value: &'a Group<Qbe>) -> Self {
        TypeAst(value)
    }
}
