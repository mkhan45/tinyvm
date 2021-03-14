use std::io::Read;

type Pointer = usize;
type Program<'a> = &'a [Inst];

struct Stack(Vec<isize>);

impl Stack {
    fn push(&mut self, v: isize) {
        self.0.push(v);
    }

    fn pop(&mut self) -> isize {
        self.0.pop().expect("popped an empty stack")
    }

    fn peek(&mut self) -> isize {
        *self.0.last().expect("popped an empty stack")
    }
}

#[derive(Debug)]
enum Inst {
    Push(isize),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Jump(Pointer),
    JE(Pointer),
    JNE(Pointer),
    Get(Pointer),
    Set(Pointer),
    Noop,
    Print,
    PrintC,
    PrintStack,
}

fn interpret<'a>(program: Program<'a>) -> isize {
    use Inst::*;

    let mut stack: Stack = Stack(Vec::new());
    let mut pointer: Pointer = 0;

    while let Some(instruction) = program.get(pointer) {
        pointer += 1;

        match instruction {
            Push(d) => stack.push(*d),
            Pop => {
                stack.pop();
            }
            Add => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a + b)
            }
            Sub => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b - a)
            }
            Mul => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a * b)
            }
            Div => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b / a)
            }
            Jump(p) => pointer = *p,
            JE(p) => {
                if stack.peek() == 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JNE(p) => {
                if stack.peek() != 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            Get(i) => stack.push(*stack.0.get(*i).expect(&format!(
                "Tried to access index {} with stack of length {}",
                i,
                stack.0.len(),
            ))),
            Set(i) => {
                let t = stack.peek();
                *stack.0.get_mut(*i).unwrap() = t;
            }
            Print => print!("{}", stack.peek()),
            PrintC => print!("{}", stack.peek() as u8 as char),
            PrintStack => println!("{:?}", stack.0),
            Noop => {}
        }
    }

    stack.pop()
}

fn parse_instruction(s: &str) -> Inst {
    use Inst::*;

    match s.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["Push", x] => Push(x.parse::<isize>().unwrap()),
        ["Pop"] => Pop,
        ["Add"] => Add,
        ["Sub"] => Sub,
        ["Mul"] => Mul,
        ["Div"] => Div,
        ["Jump", x] => Jump(x.parse::<Pointer>().unwrap()),
        ["JE", x] => JE(x.parse::<Pointer>().unwrap()),
        ["JNE", x] => JNE(x.parse::<Pointer>().unwrap()),
        ["Get", x] => Get(x.parse::<Pointer>().unwrap()),
        ["Set", x] => Set(x.parse::<Pointer>().unwrap()),
        ["Print"] => Print,
        ["PrintC"] => PrintC,
        ["PrintStack"] => PrintStack,
        [] | ["--", ..] => Noop,
        l => panic!("Invalid instruction: {:?}", l),
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut f = std::fs::File::open(&args[1])?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let instructions: Vec<Inst> = buffer.split('\n').map(parse_instruction).collect();

    interpret(&instructions[..]);

    Ok(())
}
