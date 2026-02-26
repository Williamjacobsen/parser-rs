#[allow(dead_code)]
#[derive(Debug)]
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
        let token = self.consume().unwrap_or(&Token::Illegal);
        println!("{:?}", token);

        let statement = match token {
            Token::IfStatement => self.parse_if_statement(),
            _ => panic!("Not supposed to happen."),
        };
        println!("{:?}", statement);

        Node::Print(Box::new(Node::String("Test".to_string())))
    }

    fn parse_if_statement(&mut self) -> Node {
        let left_paren = match self.consume().unwrap() {
            Token::LeftParen => Token::LeftParen,
            t => panic!("Expected '(', but got {:?}", t),
        };

        let expression = self.parse_expression();

        let right_paren = match self.consume().unwrap() {
            Token::RightParen => Token::RightParen,
            t => panic!("Expected ')', but got {:?}", t),
        };

        let left_curly_bracket = match self.consume().unwrap() {
            Token::LeftCurlyBracket => Token::LeftCurlyBracket,
            t => panic!("Expected '{{', but got {:?}", t),
        };

        // parse statements

        let right_curly_bracket = match self.consume().unwrap() {
            Token::RightCurlyBracket => Token::RightCurlyBracket,
            t => panic!("Expected '}}', but got {:?}", t),
        };

        Node::If {
            condition: Box::new(Node::Equal(Box::new(Node::Int(0)), Box::new(Node::Int(0)))),
            then_block: Vec::new(),
            else_block: None,
        }
    }

    fn parse_expression(&mut self) {
        self.parse_logical()
    }

    fn parse_logical(&mut self) {
        self.parse_comparison();
    }

    fn parse_comparison(&mut self) -> Node {
        let left = self.parse_additive();

        match self.peek() {
            Some(Token::EqualEqual) => {
                self.consume();
                println!("{:?}", Token::EqualEqual);

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
        println!("{:?}", factor);

        match factor {
            Token::String(s) => Node::String(s.clone()),
            Token::IntValue(n) => Node::Int(*n),
            Token::Identifier(name) => Node::Identifier(name.clone()),
            t => panic!("Failed to parse factor for: {:?}", t),
        }
    }
}
