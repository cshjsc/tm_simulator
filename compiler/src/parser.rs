
use nom::Err;
use nom::bytes::complete::is_a;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map_opt;
use nom::character::is_alphanumeric;
use nom::bytes::complete::take_while;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric0;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom::combinator::map;

use once_cell::sync::Lazy;
use std::collections::HashSet;

use super::ast;

struct TmParser(HashSet<String>);

static KEYWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    vec!["cycle", "branch", "break", "halt"].into_iter().collect()
});

enum IdentError<I> {
    UnexpectedCharacter(nom::error::Error<I>),
    UnexpectedKeyword,
}

impl TmParser {
    fn parse_def(input: &str) -> IResult<&str, ast::TmDef> {
        todo!()
    }

    fn tm_ident(input: &str) -> IResult<&str, String, IdentError<&str>> {
        let ident = || {
            alt((alpha1, is_a("_")))
                .and(take_while(|c| is_alphanumeric(c as u8) || c == '_'))
                .map(|(x, xs)| format!("{}{}", x, xs))
        };

        ident()
            .parse(input)
            .map_err(|e| e.map(IdentError::UnexpectedCharacter))
            .and_then(|(xs, res)| {
                if KEYWORDS.contains(res.as_str()) {
                    Err(Err::Error(IdentError::UnexpectedKeyword))
                }
                else {
                    Ok((xs, res))
                }
            })
    }

    fn tm_alpha(&self, input: &str) -> IResult<&str, String> {
        todo!()
    }

    fn tm_block(&self, input: &str) -> IResult<&str, String> {
        todo!()
    }
}