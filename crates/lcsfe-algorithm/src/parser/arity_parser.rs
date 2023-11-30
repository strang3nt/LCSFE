use chumsky::prelude::*;

pub fn arity_parser() -> impl Parser<char, Vec<(String, usize)>, Error = Simple<char>> {
    let ident = text::ident().padded();

    let fun_arity = ident
        .then(text::int(10).padded_by(just(' ').repeated()))
        .map(|(name, arity)| (name, arity.parse().unwrap()))
        .separated_by(just('\n'))
        .allow_trailing();

    fun_arity.then_ignore(end())
}
