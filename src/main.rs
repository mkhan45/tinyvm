use std::collections::BTreeMap;
use std::io::Read;

type Pointer = usize;
type Program<'a> = &'a [Inst];
type Label<'a> = (&'a str, Pointer);
type Labels<'a> = BTreeMap<&'a str, Pointer>;

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

fn parse_instruction(s: &[&str], labels: &Labels) -> Inst {
    use Inst::*;

    match s {
        ["Push", x] => Push(x.parse::<isize>().unwrap()),
        ["Pop"] => Pop,
        ["Add"] => Add,
        ["Sub"] => Sub,
        ["Mul"] => Mul,
        ["Div"] => Div,
        ["Jump", l] => Jump(*labels.get(l).unwrap()),
        ["JE", l] => JE(*labels.get(l).unwrap()),
        ["JNE", l] => JNE(*labels.get(l).unwrap()),
        ["Get", p] => Get(p.parse::<Pointer>().unwrap()),
        ["Set", p] => Set(p.parse::<Pointer>().unwrap()),
        ["Print"] => Print,
        ["PrintC"] => PrintC,
        ["PrintStack"] => PrintStack,
        [] | ["--", ..] | ["label", ..] => Noop,
        l => panic!("Invalid instruction: {:?}", l),
    }
}

fn find_label<'a>(i: Pointer, s: &'a [&'a str]) -> Option<Label> {
    if let ["label", l] = s {
        Some((l, i))
    } else {
        None
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut f = std::fs::File::open(&args[1])?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let line_splits = buffer
        .split('\n')
        .map(|s| s.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let labels: Labels = line_splits
        .iter()
        .enumerate()
        .filter_map(|(i, s)| find_label(i, s.as_slice()))
        .collect();

    let instructions: Vec<Inst> = line_splits
        .iter()
        .map(|s| parse_instruction(s.as_slice(), &labels))
        .collect();

    interpret(&instructions[..]);

    Ok(())
}
