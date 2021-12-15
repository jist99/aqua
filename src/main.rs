use std::io::Read;

//Enums - Op
#[derive(Debug)] //Temp for printing
enum Operand {
    Int(i32),
    Bool(bool),
}

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
}

#[derive(PartialEq)]
enum Glyph {
    OpenSquiggle,
    CloseSquiggle,
}

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

    fn add(&mut self) {
        //Int Int
        let i = match self.pop() {
            Operand::Int(v) => v,
            _ => panic!("Add only implemented for int"),
        };

        let i2 = match self.pop() {
            Operand::Int(v) => v,
            _ => panic!("Add only implemented for int"),
        };

        self.push(Operand::Int(i + i2));
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
                }
            }
            _ => panic!("Conditional requires Bool at top of stack!"),
        }
    }

    fn print(&mut self) {
        println!("{:?}", self.stack[self.stack.len() - 1]);
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

            match c {
                //Comment
                _c if self.is_str("//") => self.comment(),

                //Operands
                c if c.is_ascii_digit() => return Some(Op::Operand(Operand::Int(self.read_num()))),
                _c if self.is_str("true") => return Some(Op::Operand(Operand::Bool(true))),
                _c if self.is_str("false") => return Some(Op::Operand(Operand::Bool(false))),

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

                //Glyphs
                '{' => return Some(Op::Glyph(Glyph::OpenSquiggle)),
                '}' => return Some(Op::Glyph(Glyph::CloseSquiggle)),

                //Misc
                c if c.is_ascii_whitespace() => (),
                _ => panic!("Unrecognised token {}", c),
            };
        }
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
    let mut file = std::fs::File::open("J:\\_programming\\Rust\\teal\\test.tl").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut lex = Lexer::new(data);
    let mut stack = OpStack::new();

    'a: loop {
        let op = match lex.next() {
            Some(v) => v,
            None => break 'a,
        };

        match op {
            Op::Glyph(_) => (),
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
                _ => panic!("WIP"),
            },
        }

        //println!("{:?}", stack.stack);
    }
}
