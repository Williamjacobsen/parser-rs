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

    println!("\nTop-Down Recursive-descent parser:");
    let tokens = vec![
        recursive_descent_parser::Token::IfStatement,
        recursive_descent_parser::Token::LeftParen,
        recursive_descent_parser::Token::Identifier("a".to_string()),
        recursive_descent_parser::Token::EqualEqual,
        recursive_descent_parser::Token::Identifier("b".to_string()),
        recursive_descent_parser::Token::RightParen,
        recursive_descent_parser::Token::LeftCurlyBracket,
        recursive_descent_parser::Token::Print,
        recursive_descent_parser::Token::LeftParen,
        recursive_descent_parser::Token::String("abc".to_string()),
        recursive_descent_parser::Token::RightParen,
        recursive_descent_parser::Token::RightCurlyBracket,
    ];

    let mut parser = recursive_descent_parser::Parser::new(tokens);
    let ast = parser.parse();
    println!("{:?}", ast);
}
