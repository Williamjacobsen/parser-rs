#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    IntType,
    IntValue(i64),
    String(String),
    Identifier(String),
    Equal,
    EqualEqual,
    And,
    Or,
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
    Illegal,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Node {
    Int(i64),
    String(String),
    Identifier(String),

    VarDeclaration {
        name: String,
        value: Box<Node>,
    },

    Equal(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),

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
// statement ::= print_statement, if_statement, var_declaration
//
// print_statement ::= "print" "(" expression ")" ";"
//
// if_statement ::= "if" "(" expression ")" "{" statement* "}"
//                      ( "else" "if" "(" expression ")" "{" statement* "}" )*
//                      ( "else" "{" statement* "}" )?
//
// var_declaration ::= "int" IDENTIFIER "=" expression ";"
//
// expression ::= logical
//
// logical ::= comparison ( ("&&" | "||") comparison )*
//
// comparison ::= additive ( ("==" | "!=" | "<" | "<=" | ">" | ">=") additive )?
//
// additive ::= multiplicative ( ("+" | "-") multiplicative )*
//
// multiplicative ::= factor ( ("*" | "/") factor )*
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
        let result = self.parse_program();
        println!("{:?}", result)
    }

    fn parse_program(&mut self) -> Node {
        let mut statements = Vec::new();

        while self.peek().is_some() {
            statements.push(self.parse_statement());
        }

        Node::Program(statements)
    }

    fn parse_statement(&mut self) -> Node {
        let token = self.consume().unwrap_or(&Token::Illegal);

        let statement = match token {
            Token::IfStatement => self.parse_if_statement(),
            Token::Print => self.parse_print(),
            t => panic!("PANIC: Not a valid statement: {:?}", t),
        };

        statement
    }

    fn parse_if_statement(&mut self) -> Node {
        let _left_paren = match self.consume().unwrap() {
            Token::LeftParen => Token::LeftParen,
            t => panic!("Expected '(', but got {:?}", t),
        };

        let expression = self.parse_expression();

        let _right_paren = match self.consume().unwrap() {
            Token::RightParen => Token::RightParen,
            t => panic!("Expected ')', but got {:?}", t),
        };

        let _left_curly_bracket = match self.consume().unwrap() {
            Token::LeftCurlyBracket => Token::LeftCurlyBracket,
            t => panic!("Expected '{{', but got {:?}", t),
        };

        let mut then_block: Vec<Node> = Vec::new();

        while *self.peek().unwrap() != Token::RightCurlyBracket {
            then_block.push(self.parse_statement());
            if self.position > self.tokens.len() {
                panic!("Never found '}}'")
            }
        }

        let _right_curly_bracket = match self.consume().unwrap() {
            Token::RightCurlyBracket => Token::RightCurlyBracket,
            t => panic!("Expected '}}', but got {:?}", t),
        };

        Node::If {
            condition: Box::new(expression),
            then_block: then_block,
            else_block: None,
        }
    }

    fn parse_print(&mut self) -> Node {
        let _left_paren = match self.consume().unwrap() {
            Token::LeftParen => Token::LeftParen,
            t => panic!("Expected '(', but got {:?}", t),
        };

        let expression = self.parse_expression();

        let _right_paren = match self.consume().unwrap() {
            Token::RightParen => Token::RightParen,
            t => panic!("Expected ')', but got {:?}", t),
        };

        let _semi_colon = match self.consume().unwrap() {
            Token::SemiColon => Token::SemiColon,
            t => panic!("Expected ';', but got {:?}", t),
        };

        Node::Print(Box::new(expression))
    }

    fn parse_expression(&mut self) -> Node {
        self.parse_logical()
    }

    fn parse_logical(&mut self) -> Node {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Node {
        let left = self.parse_additive();

        match self.peek() {
            Some(Token::EqualEqual) => {
                self.consume();

                let right = self.parse_additive();
                Node::Equal(Box::new(left), Box::new(right))
            }
            _ => left,
        }
    }

    fn parse_additive(&mut self) -> Node {
        self.parse_multiplicative()
    }

    fn parse_multiplicative(&mut self) -> Node {
        self.parse_factor()
    }

    fn parse_factor(&mut self) -> Node {
        let factor = self.consume().unwrap();

        match factor {
            Token::String(s) => Node::String(s.clone()),
            Token::IntValue(n) => Node::Int(*n),
            Token::Identifier(name) => Node::Identifier(name.clone()),
            t => panic!("Failed to parse factor for: {:?}", t),
        }
    }
}
