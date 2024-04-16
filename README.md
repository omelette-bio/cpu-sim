# cpu-sim

this is a cpu simulator, that means it is a 2-addresses micro-assembly interpreter
there are 8 registers and for now, 5 opcodes are present :

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
AND value regitser    // register = register AND value
OR value register     // register = register OR value
NOT register          // register = NOT register

// stack manipulation
POP register          // register = stack_summit (stack_summit is removed)
PUSH register         // puts the register value on the stack (register value is not reset)

printf register       // explicit enough
```

the program has two options of launching:

- first without file passed as argument every command will be printed like:
```bash
cargo run
assembly # MOVE $4 R1
R1 := 4
assembly # 
```

- second with a file passed as argument nothing will be printed if there is no print statement
```bash
cargo run test/test1.a
(nothing prints)

cargo run test/test2.a
R1 := 3
R1 := 6
```

## Future
plans:
- add conditional and unconditional branches
- give more clear messages
- ~~add a stack~~
- add commands like "showStack" or "showRegisterTable"
- make more tests to make sure is robust
- complete the documentation

check the [documentation](docs/doc.md) if you want to know more about how the project works