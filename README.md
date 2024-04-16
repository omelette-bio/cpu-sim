# cpu-sim

## what is this ?

cpu-sim is a personal project where I try to make a micro-assembly interpreter in rust.
The language contains less opcode than a regular assembly and the memory management is way easier, plus there are 8
registers to develop.

## commands

```as
(note: value can either be a digit or a register)

// arithmetical operations
ADD value register    // register = register + value
SUB value register    // register = register - value
MUL value register    // register = register * value
DIV value register    // register = register / value

// set a register value
MOVE value register   // register = value

// bitwise operations
AND value register    // register = register AND value
OR value register     // register = register OR value
NOT register          // register = NOT register

// stack manipulation
POP register          // register = stack_summit (stack_summit is removed)
PUSH register         // puts the register value on the stack (register value is not reset)

// branch operations
JUMP value            // add value to the program counter

printf register       // explicit enough
```

## how to use it

the program has two options of launching:

- first without file passed as argument every command will be printed like:
```
cargo run
μAssembly # MOVE $4 R1
R1 := 4
μAssembly # MOVE $0 R2
R2 := 0
μAssembly # DIV R2 R1
Cannot divide, R2 is evaluated to zero
μAssembly # MOVE $3 R3; ADD R3 R2
R3 := 3
R2 := 3
μAssembly # JUMP $3
Cannot branch in this context !
```

- second with a file passed as argument nothing will be printed if there is no print statement
```
cargo run test/test1.a
(nothing prints)

cargo run test/test2.a
R1 := 3
R1 := 6

cargo run test/test3.a
The register R1 is not initialized

cargo run test/test4.a
R2 := 48

cargo run test/test5.a
infinite loop
```

## Future
plans:
- add conditional and ~~unconditional branches~~
- give more clear messages
- ~~add a stack~~
- add commands like "showStack" or "showRegisterTable"
- make more tests to make sure is robust
- complete the documentation

check the [documentation](docs/doc.md) if you want to know more about how the project works