use std::collections::hash_map::HashMap;
use std::env;
use std::fmt;
use std::io::Read;

//Enums - misc
#[derive(Debug)]
enum Brace {
    Open,
    OpenFor(usize),
}

//Enums - Op
#[derive(Clone)]
#[derive(PartialEq)]
enum Operand {
    Int(i32),
    Bool(bool),
    String(String),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Int(i) => write!(f, "{}", i),
            Operand::Bool(b) => write!(f, "{}", b),
            Operand::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    Cond,
    Pop,
    Clear,
    Assign(String),
    Access(String),
}

#[derive(PartialEq)]
enum Glyph {
    OpenSquiggle,
    CloseSquiggle,
    Loop,
    Break,
    Else,
    Index(usize),
}

#[derive(PartialEq)]
enum Op {
    Operand(Operand),
    Operator(Operator),
    Glyph(Glyph),
}

//Operation stack
struct OpStack {
    stack: Vec<Operand>,
}

impl OpStack {
    fn new() -> OpStack {
        OpStack {
            stack: Vec::<Operand>::new(),
        }
    }

    fn push(&mut self, op: Operand) {
        self.stack.push(op);
    }

    fn pop(&mut self) -> Operand {
        match self.stack.pop() {
            Some(o) => o,
            None => panic!("Cannot pop from empty stack!"),
        }
    }

    //Operations
    fn silent_pop(&mut self) {
        self.pop();
    }

    fn clear(&mut self) {
        self.stack.clear();
    }

    //TODO: Some better way of checking types on operations, this is ugly
    fn add(&mut self) {
        //Int Int
        match self.pop() {
            Operand::Int(v) => match self.pop() {
                Operand::Int(v2) => self.push(Operand::Int(v + v2)),
                Operand::String(v2) => self.push(Operand::String(v2 + &v.to_string())),
                _ => panic!("Add only implemented for Int and String"),
            },
            Operand::String(v) => match self.pop() {
                Operand::Int(v2) => self.push(Operand::String(v2.to_string() + &v)),
                Operand::String(v2) => self.push(Operand::String(v2 + &v)),
                Operand::Bool(v2) => self.push(Operand::String(v2.to_string() + &v)),
                _ => panic!("Add only implemented for Int and String"),
            },
            Operand::Bool(v) => match self.pop() {
                Operand::String(v2) => self.push(Operand::String(v2 + &v.to_string())),
                _ => panic!("Can only append Bools to Strings!"),
            },
            _ => panic!("Add only implemented for Int and String"),
        };
    }

    fn sub(&mut self) {
        //Int Int
        let i = match self.pop() {
            Operand::Int(v) => v,
            _ => panic!("Sub only implemented for int"),
        };

        let i2 = match self.pop() {
            Operand::Int(v) => v,
            _ => panic!("Sub only implemented for int"),
        };

        self.push(Operand::Int(i2 - i));
    }

    fn mul(&mut self) {
        //Int Int
        let i = match self.pop() {
            Operand::Int(v) => v,
            _ => panic!("Mul only implemented for int"),
        };

        let i2 = match self.pop() {
            Operand::Int(v) => v,
            _ => panic!("Mul only implemented for int"),
        };

        self.push(Operand::Int(i * i2));
    }

    fn div(&mut self) {
        //Int Int
        let i = match self.pop() {
            Operand::Int(v) => v,
            _ => panic!("Mul only implemented for int"),
        };

        let i2 = match self.pop() {
            Operand::Int(v) => v,
            _ => panic!("Mul only implemented for int"),
        };

        self.push(Operand::Int(i2 / i));
    }

    fn equal(&mut self) {
        //Int Int || Bool Bool
        match self.pop() {
            Operand::Int(i) => match self.pop() {
                Operand::Int(i2) => self.push(Operand::Bool(i == i2)),
                _ => panic!("Can only compare Int with Int!"),
            },

            Operand::Bool(v) => match self.pop() {
                Operand::Bool(v2) => self.push(Operand::Bool(v == v2)),
                _ => panic!("Can only compare Bool with Bool!"),
            },

            Operand::String(v) => match self.pop() {
                Operand::String(v2) => self.push(Operand::Bool(v == v2)),
                _ => panic!("Can only compare String with String!"),
            },

            _ => panic!("Can only compare Ints and Bools!"),
        }
    }

    fn not_equal(&mut self) {
        //Int Int || Bool Bool
        match self.pop() {
            Operand::Int(i) => match self.pop() {
                Operand::Int(i2) => self.push(Operand::Bool(i != i2)),
                _ => panic!("Can only compare Int with Int!"),
            },

            Operand::Bool(v) => match self.pop() {
                Operand::Bool(v2) => self.push(Operand::Bool(v != v2)),
                _ => panic!("Can only compare Bool with Bool!"),
            },

            Operand::String(v) => match self.pop() {
                Operand::String(v2) => self.push(Operand::Bool(v != v2)),
                _ => panic!("Can only compare String with String!"),
            },

            _ => panic!("Can only compare Ints and Bools!"),
        }
    }

    fn less_than(&mut self) {
        //Int Int
        let i = match self.pop() {
            Operand::Int(v) => v,
            _ => panic!("Can only compare Ints!"),
        };

        match self.pop() {
            Operand::Int(v) => self.push(Operand::Bool(v < i)),
            _ => panic!("Can only compare Ints!"),
        };
    }

    fn greater_than(&mut self) {
        //Int Int
        let i = match self.pop() {
            Operand::Int(v) => v,
            _ => panic!("Can only compare Ints!"),
        };

        match self.pop() {
            Operand::Int(v) => self.push(Operand::Bool(v > i)),
            _ => panic!("Can only compare Ints!"),
        };
    }

    fn cond(&mut self, lex: &mut Lexer) {
        match self.pop() {
            Operand::Bool(v) => {
                //on false, skip to end
                if !v {
                    lex.exit_body();
                    //If the next token is an else, execute it
                    match lex.peek() {
                        Some(Op::Glyph(Glyph::Else)) => {
                            lex.next();
                        }
                        _ => (),
                    }
                }
            }
            _ => panic!("Conditional requires Bool at top of stack!"),
        }
    }

    fn print(&mut self) {
        if self.stack.is_empty() {
            println!("_");
        } else {
            println!("{}", self.stack[self.stack.len() - 1]);
        }
    }

    fn assign(&mut self, var_store: &mut VarStore, name: String) {
        if self.stack.is_empty() {
            panic!("Cannot assign from empty stack!");
        }

        let val = self.pop();
        var_store.vars.insert(name, val);
    }

    fn access(&mut self, var_store: &mut VarStore, lex: &mut Lexer, name: String) {
        match var_store.vars.get(&name) {
            Some(v) => {
                match lex.peek() {
                    Some(Op::Glyph(Glyph::Index(i))) => {
                        match v {
                            Operand::String(s) => { 
                                self.push(Operand::String(s.chars().nth(i).unwrap().to_string()));
                                lex.next();
                            },
                            _ => panic!("Can only index Strings!"),
                        }
                    }
                    _ => self.push((*v).clone()),
                }
            },
            None => panic!("Variable {} not found!", name),
        }
    }
}

//Variable storage
struct VarStore {
    vars: HashMap<String, Operand>,
}

impl VarStore {
    fn new() -> VarStore {
        VarStore {
            vars: HashMap::<String, Operand>::new(),
        }
    }
}

//Lexer
struct Lexer {
    chars: Vec<char>,
    current: usize,
}

impl Lexer {
    fn new(file: String) -> Lexer {
        let mut l = Lexer {
            chars: Vec::<char>::new(),
            current: 0,
        };
        l.chars = file.chars().collect();
        l
    }

    fn next(&mut self) -> Option<Op> {
        loop {
            self.current += 1; //TODO: fix so that a space before the program is not needed
            if self.current >= self.chars.len() {
                return None;
            }

            let c = self.chars[self.current];
            /*if !c.is_ascii_whitespace(){
                println!("{}", c);
            }*/

            match c {
                //Comment
                _c if self.is_str("//") => self.comment(),

                //Operands
                c if c.is_ascii_digit() => return Some(Op::Operand(Operand::Int(self.read_num()))),
                _c if self.is_str("true") => return Some(Op::Operand(Operand::Bool(true))),
                _c if self.is_str("false") => return Some(Op::Operand(Operand::Bool(false))),
                '"' | '\'' => return Some(Op::Operand(Operand::String(self.read_str()))),

                //Operators
                _c if self.is_str("==") => return Some(Op::Operator(Operator::Equal)),
                _c if self.is_str("!=") => return Some(Op::Operator(Operator::NotEqual)),
                '<' => return Some(Op::Operator(Operator::LessThan)),
                '>' => return Some(Op::Operator(Operator::GreaterThan)),
                '+' => return Some(Op::Operator(Operator::Add)),
                '-' => return Some(Op::Operator(Operator::Sub)),
                '.' => return Some(Op::Operator(Operator::Print)),
                '*' => return Some(Op::Operator(Operator::Mul)),
                '/' => return Some(Op::Operator(Operator::Div)),
                '?' => return Some(Op::Operator(Operator::Cond)),
                ',' => return Some(Op::Operator(Operator::Pop)),
                ';' => return Some(Op::Operator(Operator::Clear)),
                '=' => return Some(Op::Operator(Operator::Assign(self.read_until_space(1)))),

                //Glyphs
                '{' => return Some(Op::Glyph(Glyph::OpenSquiggle)),
                '}' => return Some(Op::Glyph(Glyph::CloseSquiggle)),
                '~' => return Some(Op::Glyph(Glyph::Loop)),
                '$' => return Some(Op::Glyph(Glyph::Break)),
                ':' => return Some(Op::Glyph(Glyph::Else)),
                '[' => return Some(Op::Glyph(Glyph::Index(self.read_index()))),

                //Variable access
                c if c.is_ascii_alphabetic() => {
                    return Some(Op::Operator(Operator::Access(self.read_until_space(0))))
                }

                //Misc
                c if c.is_ascii_whitespace() => (),
                _ => panic!("Unrecognised token {}", c),
            };
        }
    }

    fn peek(&mut self) -> Option<Op> {
        let old_pos = self.current;
        let op = self.next();
        self.current = old_pos;
        op
    }

    fn read_num(&mut self) -> i32 {
        let mut num = String::new();
        while self.current < self.chars.len() && self.chars[self.current].is_ascii_digit() {
            num.push(self.chars[self.current]);
            self.current += 1;
        }

        self.current -= 1;
        num.parse().unwrap()
    }

    fn read_index(&mut self) -> usize {
        let mut num = String::new();
        let mut c = self.chars[self.current];

        while self.current < self.chars.len() && c != ']' {
            num.push(self.chars[self.current]);
            self.current += 1;
            c = self.chars[self.current];
        }

        match num.parse() {
            Ok(n) => n,
            Err(_) => panic!("Invalid index!"),
        }
    }

    fn read_str(&mut self) -> String {
        let mut str = String::new();
        self.current += 1;
        let mut c = self.chars[self.current];

        while self.current < self.chars.len() && c != '"' && c != '\'' {
            str.push(c);
            self.current += 1;
            c = self.chars[self.current];
        }

        str
    }

    fn seek(&mut self, op: Op, err: String) {
        let mut old = self.current;
        loop {
            match self.next() {
                Some(o) => {
                    if o == op {
                        self.current = old;
                        return;
                    }
                }
                None => panic!("{}", err),
            }
            old = self.current;
        }
    }

    fn is_str(&mut self, s: &str) -> bool {
        let mut temp = 0usize;
        let s: Vec<char> = s.chars().collect();

        for i in self.current..self.chars.len() {
            if self.chars[i] != s[temp] {
                return false;
            }
            temp += 1;

            if temp >= s.len() {
                break;
            }
        }

        self.current += temp - 1;
        true
    }

    fn comment(&mut self) {
        loop {
            if self.current >= self.chars.len() {
                return;
            }

            let c = self.chars[self.current];
            if c == '\n' {
                return;
            }
            self.current += 1;
        }
    }

    fn read_until_space(&mut self, offset: usize) -> String {
        self.current += offset;
        let mut s = String::new();
        loop {
            if self.current >= self.chars.len() {
                return s;
            }

            let c = self.chars[self.current];
            if c.is_ascii_whitespace() {
                return s;
            }
            s.push(c);
            self.current += 1;
        }
    }

    fn exit_body(&mut self) {
        let mut skips = 0;
        loop {
            let next = self.next();
            let op = match next {
                None => panic!("Missing closing brace!"),
                Some(v) => v,
            };

            if let Op::Glyph(g) = op {
                if g == Glyph::OpenSquiggle {
                    skips += 1;
                } else if g == Glyph::CloseSquiggle {
                    if skips == 1 {
                        return;
                    } else {
                        skips -= 1;
                    }
                }
            }
        }
    }
}

fn main() {
    //accept filename from terminal
    let args: Vec<String> = env::args().collect();
    let mut file = std::fs::File::open(&args[1]).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    //append newline to start of data - to make lexing easier
    let data = "\n".to_string() + &data;

    //Main structures
    let mut lex = Lexer::new(data);
    let mut stack = OpStack::new();
    let mut var_store = VarStore::new();

    //Sub structures
    let mut to_open = false;
    let mut loop_stack = Vec::<Brace>::new();

    'a: loop {
        let op = match lex.next() {
            Some(v) => v,
            None => break 'a,
        };

        match op {
            //Dealing with loops, ensuring that when the latter bracket of a loop is reached, we actually loop
            //Requires a stack to keep track of which brackets are for what, etc
            Op::Glyph(g) => match g {
                Glyph::Loop => to_open = true,
                Glyph::OpenSquiggle => {
                    if to_open {
                        loop_stack.push(Brace::OpenFor(lex.current));
                        to_open = false;
                    } else {
                        loop_stack.push(Brace::Open);
                    }
                }
                Glyph::CloseSquiggle => {
                    if loop_stack.is_empty() {
                        panic!("Unmatched braces!");
                    }

                    match loop_stack[loop_stack.len() - 1] {
                        Brace::Open => {
                            loop_stack.pop();
                        }
                        //If we're closing a loop, loop
                        Brace::OpenFor(pos) => lex.current = pos,
                        _ => (),
                    };
                }
                Glyph::Break => {
                    //Can probably be improved, instead of jumping to the top of the scope and exiting, instead use the already tracked brackets to get out
                    let mut found = false;

                    for i in (0..loop_stack.len()).rev() {
                        let brace = &mut loop_stack[i];
                        match brace {
                            Brace::Open => {
                                loop_stack.pop();
                            }
                            Brace::OpenFor(pos) => {
                                //Current should be pos-1 because pos is the bracket,
                                //so we must go before that for exit_body() to work correctly
                                lex.current = *pos - 1;
                                lex.exit_body();

                                found = true;
                                break;
                            }
                        };
                    }

                    if !found {
                        panic!("Can only break from a loop!");
                    } else {
                        loop_stack.pop();
                    }
                }
                //Skip over else's in normal code
                Glyph::Else => {
                    lex.seek(Op::Glyph(Glyph::OpenSquiggle), "Missing braces after else!".to_string());
                    lex.exit_body();
                }
                Glyph::Index(_) => panic!("Can only index variables!"),
                _ => (),
            },
            Op::Operand(o) => stack.push(o),
            Op::Operator(o) => match o {
                Operator::Add => stack.add(),
                Operator::Sub => stack.sub(),
                Operator::Print => stack.print(),
                Operator::Mul => stack.mul(),
                Operator::Div => stack.div(),
                Operator::Equal => stack.equal(),
                Operator::NotEqual => stack.not_equal(),
                Operator::LessThan => stack.less_than(),
                Operator::GreaterThan => stack.greater_than(),
                Operator::Cond => stack.cond(&mut lex),
                Operator::Pop => stack.silent_pop(),
                Operator::Clear => stack.clear(),
                Operator::Access(s) => stack.access(&mut var_store, &mut lex, s),
                Operator::Assign(s) => stack.assign(&mut var_store, s),
                _ => panic!("WIP"),
            },
        }

        //println!("{:?}", stack.stack);
    }
}
