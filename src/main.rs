
extern crate error_reporter;
extern crate array_pattern;

mod parsing;

fn main() {

    let input = "";
    let token_result = parsing::tokenizer::tokenize(input);
    let tokens = match token_result {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    println!("Hello, world!");
}
