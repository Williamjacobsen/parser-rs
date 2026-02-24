#[path = "./Top-Down/Recursive-descent-arithmetic-parser.rs"]
mod recursive_descent_arithmetic_parser;

#[path = "./Top-Down/Recursive-descent-parser.rs"]
mod recursive_descent_parser;

fn main() {
    println!("Top-Down Recursive-descent parser (arithmetic):");
    let tokens = vec![
        recursive_descent_arithmetic_parser::Token::Int(10),
        recursive_descent_arithmetic_parser::Token::Add,
        recursive_descent_arithmetic_parser::Token::Int(5),
        recursive_descent_arithmetic_parser::Token::Multiply,
        recursive_descent_arithmetic_parser::Token::Int(20),
    ];

    let mut parser = recursive_descent_arithmetic_parser::Parser::new(tokens);
    let ast = parser.parse_expression();
    println!("{:?}", ast);

    println!("Top-Down Recursive-descent parser:");
    let tokens = vec![];

    let mut parser = recursive_descent_parser::Parser::new(tokens);
    let ast = parser.parse();
    println!("{:?}", ast);
}
