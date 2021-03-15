An MVP stack-based bytecode VM

Runs a very simple instruction set and is Turing Complete.

## Instructions

| Instruction         | Description                                                                                |
|---------------------|--------------------------------------------------------------------------------------------|
| Push (isize)        | Pushes the argument to the top of the stack                                                |
| Pop                 | Removes the value on top of the stack                                                      |
| Add                 | Pops the top two values and pushes their sum                                               |
| Sub                 | Pops the top two values and pushes their difference                                        |
| Mul                 | Pops the top two values and pushes their product                                           |
| Div                 | Pops the top two values and pushes their quotient                                          |
| Jump (label)        | Sets the instruction pointer to the label                                                  |
| JNE  (label)        | Jumps if the top of the stack is not zero                                                  |
| JE   (label)        | Jumps if the top of the stack is zero                                                      |
| JGT  (label)        | Jumps if the top of the stack is greater than zero                                         |
| JLT  (label)        | Jumps if the top of the stack is less than zero                                            |
| JGE  (label)        | Jumps if the top of the stack is greater than or equal to zero                             |
| JLE  (label)        | Jumps if the top of the stack is less than or equal to zero                                |
| Call (procedure)    | Calls a procedure, setting the stack offset to the current s stack length                  |
| Get  (usize)        | Gets index of the stack and copies it to the top                                           |
| Set  (usize)        | Copies value at the top of the stack to the index                                          |
| GetArg  (usize)     | Gets nth argument from top of callstack stack offset, used  for procedures                 |
| SetArg  (usize)     | Sets nth argument from top of callstack stack offset, used  for procedures                 |
| Noop                | Doesn't do anything, used by comments to keep instruction pointers correspondent to lines  |
| Print               | Prints value at the top of the stack as an integer                                         |
| PrintC              | Prints value at the top of the stack as an ASCII character                                 |
| PrintStack          | Prints the whole stack, used mostly for debugging                                          |

You can also set a label with the line `label $name`, and you can declare a procedure by using `Proc $name`, `Ret`, and `End`. See `test_files/procedure.bytecode` or `test_files/fib_recurse.bytecode` for more details.

## Examples

In `test_files/`:

`hello_world.bytecode` prints "Hello World"

`sum.bytecode` prints the sum of all integers from 0 to 100

`fib.bytecode` prints the first 40 fibonacci numbers

`fib_recurse.bytecode` recursively calculates 35th fibonacci number
