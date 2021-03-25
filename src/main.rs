use std::collections::BTreeMap;
use std::io::Read;

// Pointers are just indices into a Vec
type Pointer = usize;

// The Program is a list of instructions
type Program<'a> = &'a [Instruction];

// A Label is a name and an instruction pointer
type Label<'a> = (&'a str, Pointer);
type Labels<'a> = BTreeMap<&'a str, Pointer>;

// A procedure has a name, a start instruction pointer,
// and an end instruction pointer.
// The ending instruction pointer is just used to skip over
// the procedure.
type Procedures<'a> = BTreeMap<&'a str, (Pointer, Pointer)>;

// A StackFrame has an offset and an instruction pointer
// to return to.
// The offset is used for the Get/Set and GetArg/SetArg instructions.
//
// Example:
// With stack [1, 3, 2] on instruction 24, call a procedure
// at location 96.
//
// The stack frame to be pushed will look like:
// StackFrame {
//      stack_offset: 3, // the length of the stack before calling the procedure
//      ip: i,
// }
//
// The current instruction pointer will be set to 96.
//
// Inside the procedure, let's say a few values are pushed,
// resulting in a stack [1, 3, 2, 5, 7, 4].
//
// Stack values are now accessed by Get/Set and GetArg/SetArg
// relatively to the stack offset (denoted by the pipe '|'):
//
//       GetArg 2    GetArg 1    GetArg 0     Get 0       Get 1       Get 2
// [        1,          3,          2,    |     5,          7,          4       ]
struct StackFrame {
    pub stack_offset: Pointer,
    pub ip: Pointer,
}

// The CallStack is just a Vec of StackFrames.
type CallStack = Vec<StackFrame>;

// Since the only values allowed by this VM are isizes,
// the Stack is just a Vec of isizes.
//
// I made a wrapper type just to panic and crash the program
// on any errors. In a real VM you'd want to add proper error
// handling.
struct Stack(Vec<isize>);

impl Stack {
    fn push(&mut self, v: isize) {
        self.0.push(v);
    }

    fn pop(&mut self) -> isize {
        self.0.pop().expect("popped an empty stack")
    }

    fn peek(&mut self) -> isize {
        *self.0.last().expect("peeked an empty stack")
    }

    fn peek_mut(&mut self) -> &mut isize {
        self.0.last_mut().expect("peeked an empty stack")
    }

    fn get(&self, i: usize) -> &isize {
        self.0.get(i).expect("accessed a nonexistent stack index")
    }

    fn get_mut(&mut self, i: usize) -> &mut isize {
        self.0
            .get_mut(i)
            .expect("mutably accessed a nonexistent stack index")
    }
}

// For simplicity, this VM runs off of tagged union
// Instructions which carry data with them. For that reason,
// this isn't strictly a *bytecode* interpreter, since instructions
// take 16 bytes.
//
// Usually, you would want to store each Instruction
// as just a discriminant (e.g. Push or Jump) so that
// they fit in one byte each.
//
// The instruction arguments would then be read byte by byte from
// the code.
//
// An explanation of the individual instructions is in the
// `interpret()` function.
#[derive(Debug)]
enum Instruction {
    Push(isize),
    Pop,
    Add,
    Sub,
    Incr,
    Decr,
    Mul,
    Div,
    Jump(Pointer),
    JE(Pointer),
    JNE(Pointer),
    JGT(Pointer),
    JLT(Pointer),
    JGE(Pointer),
    JLE(Pointer),
    Get(Pointer),
    Set(Pointer),
    GetArg(Pointer),
    SetArg(Pointer),
    Noop,
    Print,
    PrintC,
    PrintStack,
    Call(Pointer),
    Ret,
}

fn interpret<'a>(program: Program<'a>) {
    use Instruction::*;

    let mut stack: Stack = Stack(Vec::new());
    let mut pointer: Pointer = 0;
    let mut call_stack = CallStack::new();

    while let Some(instruction) = program.get(pointer) {
        pointer += 1;

        match instruction {
            // Noop doesn't do anything. However, it's used as a placeholder
            // for labels and procedures in the code.
            Noop => {}

            // Push pushes a value to the top of the stack.
            Push(d) => stack.push(*d),

            // Pop removes a value from the top of the stack.
            Pop => {
                stack.pop();
            }

            // Add pops the two top values, adds them, and pushes
            // the result.
            //
            // Before:
            // [.., a, b]
            //
            // After:
            // [.., a + b]
            Add => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a + b)
            }

            // Sub pops the two top values, and pushes the difference.
            // Importantly, the order of operations is switched.
            //
            // This is a bit more intuitive because the stack is
            // usually reasoned about from left to right.
            //
            // Before:
            // [.., a, b]
            //
            // After:
            // [.., b - a]
            Sub => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b - a)
            }

            // I think you can figure out Mul and Div
            Mul => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a * b)
            }
            Div => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b / a)
            }

            // Incr and Decr increment or decrement the value
            // at the top of the stack.
            //
            // These instructions are redundant because of Add and Sub,
            // but they improve performance significantly because they
            // remove an unecessary Push.
            Incr => *stack.peek_mut() += 1,
            Decr => *stack.peek_mut() -= 1,

            // Jump unconditionally changes the stack pointer
            Jump(p) => pointer = *p,

            // JE changes the stack pointer if the value
            // on top of the stack is zero. This is generally
            // used after Sub for equality testing, hence the
            // name of the instruction, Jump (if) Equal.
            //
            // Example:
            //
            // PrintStack -- [.., a, b]
            // Sub
            // PrintStack -- [.., b - a] // this will be zero if a and b are equal
            // JE i // jumps to Instruction i if a and b were equal.
            JE(p) => {
                if stack.peek() == 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // JNE (Jump Not Equal) changes the stack pointer
            // if the value on top of the stack is *not* zero.
            JNE(p) => {
                if stack.peek() != 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // JGT (Jump Greater Than) changes the stack pointer
            // if the value on top of the stack is greater than zero.
            JGT(p) => {
                if stack.peek() > 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // JLT (Jump Less Than) changes the stack pointer
            // if the value on top of the stack is less than zero.
            JLT(p) => {
                if stack.peek() < 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // JGE (Jump Greater Equal) changes the stack pointer
            // if the value on top of the stack is greater than
            // or equal to zero.
            JGE(p) => {
                if stack.peek() >= 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // JLE (Jump Less Equal) changes the stack pointer
            // if the value on top of the stack is greater than
            // or equal to zero.
            JLE(p) => {
                if stack.peek() <= 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // The above instructions can be confusing because they
            // don't quite match the naming semantics of JE and JNE.
            // For example, when used after a Sub, JGT will jump if
            // a was *less* than b, not greater than it, because
            // if a is *less* than b, b - a will be *greater* than zero.

            // Get pushes the value at index i to the top of the stack.
            //
            // Example:
            //
            // PrintStack -- [0, 1, 3, 2, 5]
            // Get 2
            // PrintStack -- [0, 1, 3, 2, 5, 3]
            //
            // Remember that values are indexed relatively to the stackframe
            // as explained near the start of the file.
            Get(i) => stack.push(*stack.get(*i + call_stack.last().map_or(0, |s| s.stack_offset))),

            // Set sets the value at index i to be equal to the value
            // at the top of the stack. It does *not* pop the top value.
            //
            // Example:
            //
            // PrintStack -- [0, 1, 3, 2, 5]
            // Set 2
            // PrintStack -- [0, 1, 5, 2, 5]
            //
            // Remember that values are indexed relatively to the stackframe
            // as explained near the start of the file.
            Set(i) => {
                *stack
                    .0
                    .get_mut(*i + call_stack.last().map_or(0, |s| s.stack_offset))
                    .unwrap() = stack.peek()
            }

            // GetArg and SetArg mirror Get and Set.
            GetArg(i) => stack.push(
                *stack
                    .0
                    .get(call_stack.last().unwrap().stack_offset - 1 - *i)
                    .unwrap(),
            ),
            SetArg(i) => {
                let offset_i = call_stack.last().unwrap().stack_offset - 1 - *i;
                let new_val = stack.peek();
                *stack.get_mut(offset_i) = new_val;
            }

            // Print prints the value at the top of the stack.
            Print => print!("{}", stack.peek()),

            // PrintC prints the value at the top of the stack
            // as an ASCII character.
            PrintC => print!("{}", stack.peek() as u8 as char),

            // PrintStack prints the whole stack. It's meant to be
            // used for debugging.
            PrintStack => println!("{:?}", stack.0),

            // Call calls a procedure, pushing a new StackFrame.
            // Details about the StackFrame can be found near the
            // start of the file.
            Call(p) => {
                call_stack.push(StackFrame {
                    stack_offset: stack.0.len(),
                    ip: pointer,
                });
                pointer = *p;
            }

            // Ret returns from the current procedure, popping the
            // stack frame from the top of the call stack and returning
            // to the instruction list at the index right after it was called at.
            Ret => pointer = call_stack.pop().unwrap().ip,
        }
    }
}

// The next instructions aren't really part of the VM, they are essentially
// an extremely simplistic compiler. That's because the VM doesn't quite
// support labels. It doesn't support named procedures either; it specifies
// for procedures that contain an index into the Instruction list. Additionally,
// this simplistic compiler allows for some basic comments.

// `parse_instruction` takes a line split by spaces and returns a singular
// instruction that the line represents.
//
// Labels must be obtained by preprocessing the string, it's just a map of
// names to their index in the instruction set.
//
// Procedures is similar, but it contains the instruction to jump to when called
// along with the instruction to jump to in order to skip the Procedure declaration.
// Procedures are encoded directly into the list of instructions, so the actual
// Procedure declaration is replaced by a Jump instruction to skip over it.
//
// Example:
//
// Procedure proc_name -- Line n
// ...
// End -- line e
//
// Gets turned into
//
// Jump e -- Line n
// ...
// Noop -- line e
fn parse_instruction(s: &[&str], labels: &Labels, procedures: &Procedures) -> Instruction {
    use Instruction::*;

    match s {
        ["Push", x] => Push(x.parse::<isize>().unwrap()),
        ["Pop"] => Pop,
        ["Add"] => Add,
        ["Sub"] => Sub,
        ["Mul"] => Mul,
        ["Div"] => Div,
        ["Incr"] => Incr,
        ["Decr"] => Decr,
        ["Jump", l] => Jump(*labels.get(l).unwrap()),
        ["JE", l] => JE(*labels.get(l).unwrap()),
        ["JNE", l] => JNE(*labels.get(l).unwrap()),
        ["JGE", l] => JGE(*labels.get(l).unwrap()),
        ["JLE", l] => JLE(*labels.get(l).unwrap()),
        ["JGT", l] => JGT(*labels.get(l).unwrap()),
        ["JLT", l] => JLT(*labels.get(l).unwrap()),
        ["Get", p] => Get(p.parse::<Pointer>().unwrap()),
        ["Set", p] => Set(p.parse::<Pointer>().unwrap()),
        ["GetArg", p] => GetArg(p.parse::<Pointer>().unwrap()),
        ["SetArg", p] => SetArg(p.parse::<Pointer>().unwrap()),
        ["Print"] => Print,
        ["PrintC"] => PrintC,
        ["PrintStack"] => PrintStack,
        ["Proc", proc] => Jump(procedures.get(proc).unwrap().1),
        ["Call", proc] => Call(procedures.get(proc).unwrap().0 + 1),
        ["Ret"] => Ret,
        ["label", ..] | ["End"] => Noop,
        l => panic!("Invalid instruction: {:?}", l),
    }
}

// find_label takes a line split by spaces and the label it represents,
// or None if it does not represent a label.
fn find_label<'a>(i: Pointer, s: &'a [&'a str]) -> Option<Label> {
    if let ["label", l] = s {
        Some((l, i))
    } else {
        None
    }
}

// find_procedures takes a list of lines split on space and
// returns the procedures declared.
fn find_procedures<'a>(lines: &'a [Vec<&str>]) -> Procedures<'a> {
    let mut ip = 0;
    let mut res = Procedures::new();

    while ip < lines.len() {
        if let ["Proc", proc_name] = lines[ip].as_slice() {
            let start_ip = ip;
            while lines[ip] != &["End"] {
                ip += 1;
            }
            res.insert(proc_name, (start_ip, ip + 1));
        } else {
            ip += 1;
        }
    }

    res
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut f = std::fs::File::open(&args[1])?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let line_splits = buffer
        .split('\n')
        .map(|s| s.split_whitespace().collect::<Vec<_>>())
        .filter(|s| !matches!(s.as_slice(), [] | ["--", ..]))
        .collect::<Vec<_>>();

    let labels: Labels = line_splits
        .iter()
        .enumerate()
        .filter_map(|(i, s)| find_label(i, s.as_slice()))
        .collect();

    let procedures: Procedures = find_procedures(line_splits.as_slice());

    let instructions: Vec<Instruction> = line_splits
        .iter()
        .map(|s| parse_instruction(s.as_slice(), &labels, &procedures))
        .collect();

    interpret(&instructions[..]);

    Ok(())
}
