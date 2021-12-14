use std::io::Read;

//Enums - Op
#[derive(Debug)] //Temp for printing
enum Operand {
    Int(i32),
}

impl Operand {
    fn type_match(&self, other: &Operand) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Print,
}

enum Op {
    Operand(Operand),
    Operator(Operator),
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

        self.stack.push(Operand::Int(i + i2));
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

        self.stack.push(Operand::Int(i2 - i));
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

        self.stack.push(Operand::Int(i * i2));
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

        self.stack.push(Operand::Int(i2 / i));
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

    //i know this is stupid but the match wasnt letting me return
    //on only some of the branches, so this is my workaround...
    fn next(&mut self) -> Option<Op> {
        let mut out = None;

        'a: while self.current < self.chars.len() {
            let c = self.chars[self.current];

            match c {
                c if c.is_ascii_digit() => {
                    out = Some(Op::Operand(Operand::Int(self.read_num())));
                    break 'a;
                }
                '+' => {
                    self.current += 1;
                    out = Some(Op::Operator(Operator::Add));
                    break 'a;
                }
                '-' => {
                    self.current += 1;
                    out = Some(Op::Operator(Operator::Sub));
                    break 'a;
                }
                '.' => {
                    self.current += 1;
                    out = Some(Op::Operator(Operator::Print));
                    break 'a;
                }
                '*' => {
                    self.current += 1;
                    out = Some(Op::Operator(Operator::Mul));
                    break 'a;
                }
                '/' => {
                    self.current += 1;
                    out = Some(Op::Operator(Operator::Div));
                    break 'a;
                }
                c if c.is_ascii_whitespace() => self.current += 1,
                _ => panic!("Unrecognised token!"),
            };
        }

        out
    }

    fn read_num(&mut self) -> i32 {
        let mut num = String::new();
        while self.current < self.chars.len() && self.chars[self.current].is_ascii_digit() {
            num.push(self.chars[self.current]);
            self.current += 1;
        }

        num.parse().unwrap()
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
            Op::Operand(o) => stack.push(o),
            Op::Operator(o) => match o {
                Operator::Add => stack.add(),
                Operator::Sub => stack.sub(),
                Operator::Print => stack.print(),
                Operator::Mul => stack.mul(),
                Operator::Div => stack.div(),
                _ => panic!("WIP"),
            },
        }
    }
}
