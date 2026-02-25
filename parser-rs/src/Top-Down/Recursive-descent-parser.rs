#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Int(i64),
    String(String),
    Identifier(String),
    Equal,
    Add,
    Subtract,
    Multiply,
    Divide,
    Print,
    LeftParen,
    RightParen,
    IfStatement,
    ElseIfStatement,
    ElseStatement,
    LeftCurlyBracket,
    RightCurlyBracket,
    SemiColon,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Node {
    Int(i64),
    String(String),

    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),

    Print(Box<Node>),
    If {
        condition: Box<Node>,
        then_block: Vec<Node>,
        else_block: Option<ElseBlock>,
    },

    Program(Vec<Node>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ElseBlock {
    If(Box<Node>),

    Block(Vec<Node>),
}

// Grammar:
//
// program ::= statement*
//
// statement ::= print_statement, if_statement, expression_statement
//
// print_statement ::= "print" "(" expression ")" ";"
//
// if_statement ::= "if" "(" expression ")" "{" statement* "}"
//                      ( "else" "if" "(" expression ")" "{" statement* "}" )*
//                      ( "else" "{" statement* "}" )?
//
// expression_statement ::= expression ";"
//
// expression ::= term ( ("+" | "-") term )*
//
// term ::= factor ( ("*" | "/") factor )*
//
// factor ::= INT_LITERAL | STRING_LITERAL | IDENTIFIER | "(" expression ")"

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
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

    pub fn parse(&mut self) {
        self.parse_program();
    }

    fn parse_program(&mut self) -> Node {
        let mut statements = Vec::new();

        while self.peek().is_some() {
            statements.push(self.parse_statement());
        }

        Node::Program(statements)
    }

    fn parse_statement(&mut self) -> Node {
        Node::Print(Box::new(Node::String("Test".to_string())))
    }
}
