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
| Jump (usize)        | Jumps to line (argument - 1) in the code                                                   |
| JNE  (usize)        | Jumps if the top of the stack is not zero                                                  |
| JE  (usize)         | Jumps if the top of the stack is zero                                                      |
| Get  (usize)        | Gets index of the stack and copies it to the top                                           |
| Set  (usize)        | Copies value at the top of the stack to the index                                          |
| Noop                | Doesn't do anything, used by comments to keep instruction pointers correspondent to lines  |
| Print               | Prints value at the top of the stack as an integer                                         |
| PrintC              | Prints value at the top of the stack as an ASCII character                                 |
| PrintStack          | Prints the whole stack, used mostly for debugging                                          |

## Examples

In `test_files/`:

`hello_world.bytecode` prints "Hello World"

`sum.bytecode` prints the sum of all integers from 0 to 100

`fib.bytecode` prints the first 40 fibonacci numbers
