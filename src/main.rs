use std::env;
use std::fs::File;
use std::str;

#[macro_use]
mod simplelang;
use simplelang::lexer::Lexer;
use simplelang::parser::Parser;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("Arguments {:?}", args);

    let filename = &args[0];
    let file = File::open(filename).expect("file not found");
    let lexer = Lexer::new();

    let tokens = lexer.lex(file).expect("Error while lexing source");

    for token in &tokens {
        dump!(token);
    }

    let parser = Parser::new();
    let expression = parser
        .expression(tokens.as_slice())
        .expect("Error while parsing source")
        .1;

    dump!(expression);
}
