use lexer::lexer;
use parser::Parser;

fn main() {
    let input = "a + 6 * 1";
    let tokens = lexer(input);
    
    let mut parser = Parser::new(&tokens);
    
    match parser.parse_e() {
        Ok(()) => println!("Parse success!"),
        Err(e) => println!("Parse error: {:?}", e),
    }
}