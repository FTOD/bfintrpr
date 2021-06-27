# bfintrpr
This is my first projet in Rust. The goal is to write an interpreter of brainfuck, with intuitive syntax error.
Concretely, the only two possible syntax error is 
    1.  loop start-end not matched.
    2.  unknown character. i.e. not in {'+','-','<','>','\[','\]','.',',',}.

The project is separated into 2 modules, a parser written with 'nom', that generates a very simple AST (since there are not expression/statement etc. in brainfuck); and a machine which executes the AST generated. 

The machine has a fixed size data memory and a so-called 'sp' pointer (but nothing related to the call stack...) that indicates the current pointer as described in the semantic of brainfuck.

## Current Status

There is not clear API as for now, a code example can be found in 'main.rs'. 

The error management is shitty, i will improve it.

## Future Works
Compile brainfuck programs into a \'assembly language\' that can be executed by my machine. The compiled program is \'charges\' into the instruction memory of the machine.
The machine start at a given adreese and execute like a real processor (of course, a very simple instruction set without a lot of things...). 


