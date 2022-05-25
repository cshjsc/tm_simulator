mod ast;
mod parser;
mod tm;

use std::io::{stdin, Read};

fn main() {
    let mut contents = String::new();
    stdin().read_to_string(&mut contents);

//    let parse_res = parser::tm_def().easy_parse(position::Stream::new(contents.as_str()));
//    if let Err(err) = parse_res {
//        eprint!("{}", err);
//        return;
//    }

//    let (parsed, _) = parse_res.unwrap();
//    let steps = ast::into_steps(parsed);

//    println!("{:?}", steps);
}
