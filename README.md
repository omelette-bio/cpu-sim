# cpu-sim

this is a cpu simulator, that means it is a 2-addresses micro-assembly interpreter
there are 8 registers and for now, 5 opcodes are present :

```as
(note: value can either be a digit or a register)
ADD value register    // register = register + value
SUB value register    // register = register - value
MUL value register    // register = register * value
DIV value register    // register = register / value
MOVE value register   // register = value
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
```

## Future
plans:
- give more clear messages
- add a stack
- add commands like "showStack" or "showRegisterTable"
- make more tests to make sure is robust

check the [documentation](docs/doc.md) if you want to know more about how the project works