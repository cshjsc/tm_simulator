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

// fn tm_ident_<Input>() -> impl Parser< Input, Output = String >
//     where Input: Stream<Token = char>,
//           Input::Error: ParseError<Input::Token, Input::Range, Input::Position> 
// {
//     static REGEX: Lazy<Regex> = 
//         Lazy::new(|| Regex::new("[:alpha:]+").unwrap());

//     let ident = find(&*REGEX);

//     ident.skip(skip_spaces()).map(String::from)
// }

fn tm_block_<Input>() -> impl Parser< Input, Output = ast::TmBlock >
    where
        Input: RangeStream<Token = char>,
        Input::Range: Range,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
        Regex: combine::parser::regex::Regex<Input::Range>
{
    static ALPHA_SYM: Lazy<Regex> 
        = Lazy::new(|| Regex::new("[[:alpha:]0-9]+").unwrap());

    let alpha_sym = find(&*ALPHA_SYM);

    alpha_sym.map(|_| ast::TmBlock::new())
}

parser!{
    fn tm_block[Input]()(Input) -> ast::TmBlock
    where 
    [
        Input: RangeStream<Token = char>,
        Input::Range: Range,
        Regex: combine::parser::regex::Regex<Input::Range>
    ]
    {
        tm_block_()
    }
}