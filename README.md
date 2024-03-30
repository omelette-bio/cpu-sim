# cpu-sim

this is a cpu simulator, that means it is a 2-adresses micro-assembly interpreter
there are 8 registers and for now, 5 opcodes are present :

```as
// reg2 = reg2 + reg1
ADD reg1 reg2

// reg = reg + value
ADD $value reg

// reg2 = reg2 - reg1
SUB reg1 reg2

// reg = reg - value
SUB $value reg

// reg2 = reg2 * reg1
MUL reg1 reg2

// reg = reg * value
MUL $value reg

// reg2 = reg2 / reg1
DIV reg1 reg2

// reg = reg / value
DIV $value reg
```

when its launches you can do something like this :
```bash
Context { register_table: {} }
assembly # MOVE $1 R1
Context { register_table: {R1: 1} }
assembly # ADD $42 R1
Context { register_table: {R1: 43} }
assembly # 
```

## Future
plans:
- give more clear messages
- add a stack
- add commands like "showStack" or "showRegisterTable"
- make more tests to make sure is robust
