# cpu-sim

## what is this ?

cpu-sim is a personal project where I try to make a micro-assembly interpreter in rust.
The language contains less opcode than a regular assembly and the memory management is way easier, plus there are 8
registers to develop.

## commands

notes : 
- value keyword means either a digit or a register
- PC stands for program counter, which is an index over lines of a program

<table>

<tr>
<td colspan="2">Arithmetical Operations</td>
</tr>
<tr>
<td>ADD value register</td>
<td>register = register + value</td>
</tr>
<tr>
<td>SUB value register</td>
<td>register = register - value</td>
</tr>
<tr>
<td>MUL value register</td>
<td>register = register * value</td>
</tr>
<tr>
<td>DIV value register</td>
<td>register = register / value</td>
</tr>

<tr>
<td colspan="2">Bitwise Operations</td>
</tr>


<tr>
<td>AND value register</td>
<td>register = register & value</td>
</tr>
<tr>
<td>OR value register</td>
<td>register = register | value</td>
</tr>
<tr>
<td>NOT value register</td>
<td>invert bits of the register's value</td>
</tr>

<tr>
<td colspan="2">Data Manipulation and Heap</td>
</tr>
<tr>
<td>MOVE value register</td>
<td>register = value</td>
</tr>
<tr>
<td> POP register </td>
<td> removes top of the stack and put the value in register </td>
</tr>
<tr>
<td> PUSH register </td>
<td> puts register value to the stack summit </td>
</tr>

<tr>
<td colspan="2">Branching and Program Counter Manipulation</td>
</tr>
<tr>
<td>JUMP value</td>
<td>PC = PC + value</td>
</tr>
<tr>
<td>BNEZ value register</td>
<td>PC = PC + value if register != 0</td>
</tr>
<tr>
<td>BEZ value register</td>
<td>PC = PC + value if register = 0</td>
</tr>

<tr>
<td colspan="2">Other</td>
</tr>
<tr>
<td>printf register</td>
<td>prints register's value to the terminal</td>
</tr>
</table>

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