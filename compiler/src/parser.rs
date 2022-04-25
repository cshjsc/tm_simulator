use combine::sep_by1;
use combine::choice;
use combine::optional;
use combine::value;
use combine::parser::char::string;
use combine::many;
use combine::between;
use combine::sep_by;
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

use once_cell::sync::Lazy;
use regex::Regex;
use combine::Parser;
use combine::parser::regex::find;

fn tm_ident_<Input>() -> impl Parser< Input, Output = String >
    where 
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
        Regex: combine::parser::regex::Regex<Input::Range>
{
    static REGEX: Lazy<Regex> = 
        Lazy::new(|| Regex::new("[:alpha:]+").unwrap());

    let ident = find(&*REGEX);

    ident.map(Input::Range::into)
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
        = Lazy::new(|| Regex::new("[[:alpha:]0-9_'\"!@#$%^&]+").unwrap());

    let alpha_sym = find(&*ALPHA_SYM);

    alpha_sym.map(Input::Range::into)
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

fn tm_block_<Input>() -> impl Parser< Input, Output = ast::TmBlock >
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

    let step = || {
        sep_by1(simple_step(), string("or").skip(skip_spaces()))
            .map(|res: Vec<_>| res.into_iter().collect::<ast::TmStep>())
    };
    
    let stmt = || {
        step().with(value(String::from(""))).skip(lex_char(';'))
    };

    between(lex_char('{'), lex_char('}'), many(stmt()))
        .map(|_: Vec<String>| ast::TmBlock::new())
}

parser!{
    fn tm_block[Input]()(Input) -> ast::TmBlock
    where 
    [
        Input: RangeStream<Token = char>,
        Input::Range: Range + Into<String>,
        Regex: combine::parser::regex::Regex<Input::Range>
    ]
    {
        tm_block_()
    }

}