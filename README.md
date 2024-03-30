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
