
use nom::sequence::delimited;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::combinator::recognize;
use nom::sequence::preceded;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::character::complete::space0;
use nom::sequence::terminated;
use nom::error::ParseError;
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
use nom::error::{Error, ErrorKind};

use either::Either;
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
            recognize(alt((alpha1, is_a("_")))
                .and(take_while(|c| is_alphanumeric(c as u8) || c == '_')))
        };

        ident()
            .parse(input)
            .map_err(|e| e.map(IdentError::UnexpectedCharacter))
            .and_then(|(xs, res)| {
                if KEYWORDS.contains(res) {
                    Err(Err::Error(IdentError::UnexpectedKeyword))
                }
                else {
                    Ok((xs, String::from(res)))
                }
            })
    }

    fn tm_alpha<'a>(&self, input: &'a str) -> IResult<&'a str, String> {
        let alpha = || {
            take_while(|c| is_alphanumeric(c as u8) 
                || c == '_' || c == '\'' || c == '\"'
                || c == '#' || c == '$' || c == '!'
            )
        };

        map_opt(alpha(), |res| {
            if KEYWORDS.contains(res) || self.0.contains(res) {
                None
            }
            else {
                Some(String::from(res))
            }
        })(input)
    }

    fn tm_block<'a>(&self, input: &'a str) -> IResult<&'a str, ast::TmBlock> {
        let alpha_closure = |||input| self.tm_alpha(input);

        let simple_step = || {
            tuple((
                terminated(alpha_closure(), space0).map(|x| vec![x]),
                opt(preceded(tuple((tag("->"), space0)), alpha_closure())),
                opt(terminated(alt((
                    tag(">>").map(|_| ast::TmDir::Right), 
                    tag("<<").map(|_| ast::TmDir::Left))), 
                    space0))
                    .map(|dir| dir.unwrap_or(ast::TmDir::Stay))
            ))
                .map(|(lhs, rhs, dir)| ast::AtomicTmStep::new(lhs, rhs, dir))
        };

        let step_sep = || terminated(tag("or"), space0);

        let default_step = || {
            terminated(alt((
                tag("break").map(|_| ast::TmOperation::Break),
                tag("halt").map(|_| ast::TmOperation::Halt),
                tag("<<").map(|_| ast::TmOperation::Move { 
                    replace: None, 
                    direction: ast::TmDir::Left
                }),
                tag(">>").map(|_| ast::TmOperation::Move {
                    replace: None,
                    direction: ast::TmDir::Right
                })
            )), space0)
        };

        let step = || {
            tuple((
                separated_list0(step_sep(), simple_step()),
                opt(preceded(step_sep(), default_step()))
            ))
                .map(|(act, def)| ast::TmStep::new(act, def))
        };

        let stmt = || {
            terminated(step().map(ast::TmStmt::Step), char(';'))
        };

        delimited(terminated(char('{'), space0), many0(stmt()), char('}'))
            .map(ast::TmBlock::new)
            .parse(input)
    }
}