#[allow(dead_code)]
#[derive(Debug)]
enum Token {
    Int(i64),
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Node {
    Int(i64),
    BinaryOp {
        operator: Token,
        left: Box<Node>,
        right: Box<Node>,
    },
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        return self.tokens.get(self.position);
    }

    fn consume(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    // Grammar:
    //      expression ::= term (('+' | '-') term)*
    //      term ::= factor (('*' | '/') factor)*
    //      factor ::= Int

    fn parse_expression(&mut self) -> Node {
        // Call parse_term for left leaf node.
        let mut left = self.parse_term();

        // If the next token is either '+' or '-',
        // then call parse_term for right leaf node.
        match self.peek() {
            Some(Token::Add) | Some(Token::Subtract) => {
                let operator = match self.consume().unwrap() {
                    Token::Add => Token::Add,
                    Token::Subtract => Token::Subtract,
                    _ => unreachable!(),
                };
                let right = self.parse_term();
                left = Node::BinaryOp {
                    operator: operator,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            _ => {}
        }

        left
    }

    fn parse_term(&mut self) -> Node {
        // Call parse_factor on left leaf node.
        let mut left = self.parse_factor();

        // If the next token is either '*' or '/',
        // then call parse_factor for right leaf node.
        match self.peek() {
            Some(Token::Multiply) | Some(Token::Divide) => {
                let operator = match self.consume().unwrap() {
                    Token::Multiply => Token::Multiply,
                    Token::Divide => Token::Divide,
                    _ => unreachable!(),
                };
                let right = self.parse_factor();
                left = Node::BinaryOp {
                    operator: operator,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            _ => {}
        }

        left
    }

    fn parse_factor(&mut self) -> Node {
        // Expect and return Int.
        match self.consume() {
            Some(Token::Int(n)) => Node::Int(*n),
            other => panic!("Expected Integer, got {:?}", other),
        }
    }
}

fn main() {
    let tokens = vec![
        Token::Int(10),
        Token::Add,
        Token::Int(5),
        Token::Multiply,
        Token::Int(20),
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expression();
    println!("{:?}", ast);
}
