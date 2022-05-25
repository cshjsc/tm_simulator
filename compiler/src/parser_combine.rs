use combine::attempt;
use combine::sep_by1;
use combine::choice;
use combine::optional;
use combine::value;
use combine::parser::char::string;
use combine::many;
use combine::between;
use combine::sep_by;
use combine::unexpected_any;
use combine::parser::char::char;
use combine::parser::char::spaces;
use combine::stream::Range;
use combine::RangeStream;
use combine::parser::char::letter;
use combine::many1;
use combine::ParseError;
use combine::Stream;
use combine::RangeStreamOnce;
use super::ast;

use std::collections::HashSet;
use either::Either;
use once_cell::sync::Lazy;
use regex::Regex;
use combine::Parser;
use combine::parser::regex::find;

fn tm_def_<Input>() -> impl Parser< Input, Output = ast::TmDef >
    where 
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
        Regex: combine::parser::regex::Regex<Input::Range>
{
    let skip_spaces = || spaces().silent();

    string("fn").skip(skip_spaces())
        .with(tm_ident()).skip(skip_spaces())
        .and(tm_alpha_set()).skip(skip_spaces())
        .then(|(id, set)| {
            tm_block(&set)
                .and(value((id, set)))
                .map(|(block, (id, set))| ast::TmDef::new(id, set, block))
        })
}

parser!{
    pub fn tm_def[Input]()(Input) -> ast::TmDef
    where
    [
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Regex: combine::parser::regex::Regex<Input::Range>
    ]
    {
        tm_def_()
    }
}

static KEYWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    vec!["cycle", "branch", "break", "halt"].into_iter().collect()
});

fn tm_ident_<Input>() -> impl Parser< Input, Output = String >
    where 
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
        Regex: combine::parser::regex::Regex<Input::Range>
{
    static REGEX: Lazy<Regex> = 
        Lazy::new(|| Regex::new("^[[:alpha:]_][[:word:]]*").unwrap());

    find(&*REGEX)
        .map(Input::Range::into)
        .expected("identifier")
        .then(|res: String| {
            if let Some(kw) = KEYWORDS.get(res.as_str()) {
                unexpected_any(kw)
                    .message("You cannot use a keyword as identifier")
                    .left()
            }
            else {
                value(res).right()
            }
        })
}

parser!{
    fn tm_ident[Input]()(Input) -> String
    where
    [
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Regex: combine::parser::regex::Regex<Input::Range>
    ]
    {
        tm_ident_()
    }
}

fn tm_alpha_<Input>() -> impl Parser< Input, Output = String >
    where
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
        Regex: combine::parser::regex::Regex<Input::Range>
{
    static ALPHA_SYM: Lazy<Regex> 
        = Lazy::new(|| Regex::new("^[[:alpha:]0-9_'\"!@#$%^&]+").unwrap());

    find(&*ALPHA_SYM)
        .map(Input::Range::into)
        .expected("alphabet symbol")
        .then(|res: String| {
            if let Some(kw) = KEYWORDS.get(res.as_str()) {
                unexpected_any(kw)
                    .message("You cannot use a keyword as alphabet symbol")
                    .left()
            }
            else {
                value(res).right()
            }
        })
}

parser!{
    fn tm_alpha[Input]()(Input) -> String
    where
    [
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Regex: combine::parser::regex::Regex<Input::Range>
    ]
    {
        tm_alpha_()
    }
}

fn tm_alpha_set_<Input>() -> impl Parser< Input, Output = HashSet<String> >
    where
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
        Regex: combine::parser::regex::Regex<Input::Range>
{
    let skip_spaces = || spaces().silent();
    let lex_char = |c| char(c).skip(skip_spaces());

    between(
        lex_char('['), lex_char(']'),
        sep_by(tm_alpha().skip(skip_spaces()), lex_char(','))
    )
}

parser!{
    fn tm_alpha_set[Input]()(Input) -> HashSet<String>
    where
    [
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Regex: combine::parser::regex::Regex<Input::Range>
    ]
    {
        tm_alpha_set_()
    }
}

fn tm_block_<Input>(alphabet: &HashSet<String>) -> impl Parser< Input, Output = ast::TmBlock >
    where
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
        Regex: combine::parser::regex::Regex<Input::Range>
{
    let skip_spaces = || spaces().silent();
    let lex_char = |c| char(c).skip(skip_spaces());

    let simple_step = || {
        (
            tm_alpha().skip(skip_spaces()).map(|x| vec![x]),
            optional(string("->").skip(skip_spaces()).with(tm_alpha_())),
            optional(choice((
                string(">>").skip(skip_spaces())
                    .with(value(ast::TmDir::Right)),
                string("<<").skip(skip_spaces())
                    .with(value(ast::TmDir::Left))
            ))).map(|x| x.unwrap_or(ast::TmDir::Stay))
        )
            .map(|(lhs, rhs, dir)| ast::AtomicTmStep::new(lhs, rhs, dir))
    };

    let step_sep = || string("or").skip(skip_spaces());

    let default_step = || {
        choice([
            string("break").with(value(ast::TmOperation::Break)),
            string("halt").with(value(ast::TmOperation::Halt)),
            string("<<").with(value(ast::TmOperation::Move { 
                replace: None, 
                direction: ast::TmDir::Left 
            })),
            string(">>").with(value(ast::TmOperation::Move {
                replace: None,
                direction: ast::TmDir::Right
            }))
        ]).skip(skip_spaces())
    };

    let final_step = || {
        attempt(simple_step().map(Either::Left))
            .or(default_step().map(Either::Right))
    };

    let step = || {
        (
            many(attempt(simple_step().skip(step_sep()))),
            final_step()
        )
            .map(|res: (Vec<_>, _)| {
                let mut atomic_steps = res.0;
                match res.1 {
                    Either::Left(atomic_final) => {
                        atomic_steps.push(atomic_final);
                        ast::TmStep::new(atomic_steps, None)
                    },
                    Either::Right(default) => {
                        ast::TmStep::new(atomic_steps, Some(default))
                    }
                }
            })
    };
    
    let stmt = || {
        step().map(ast::TmStmt::Step).skip(lex_char(';'))
    };

    between(lex_char('{'), lex_char('}'), many(stmt()))
        .map(ast::TmBlock::new)
}

parser!{
    fn tm_block['a, Input](alphabet: &'a HashSet<String>)(Input) -> ast::TmBlock
    where 
    [
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Regex: combine::parser::regex::Regex<Input::Range>
    ]
    {
        tm_block_(*alphabet)
    }

}