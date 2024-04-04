# data manipulation documentation

the data manipulation file is composed of two enums

## registers enum

```rust
// Eq and Hash serves to add the ability to use Registers as a key to a hashmap
// PartialEq and PartialOrd serves in the tests
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Copy, Clone)]
pub enum Registers {
    R1, R2, R3, R4, R5, R6, R7, R8
}
```

### methods

`get_val(&self, c: &Context)`
returns a Result<i32, String> depending on if the register has an associated value
in the register table, the String is `"no value associated to register : {} "`

`set_val(&self, value: i32, c: &mut Context)`
changes or set the value of the register passed as argument in the context

### traits implementation

the register enum also implements the Display trait


## value enum

```rust
// PartialEq and PartialOrd serves in the tests
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    Reg(Registers), Num(i32)
}
```

the value register represents the choice we can make for the middle value in
an opcode like `ADD value register`

### methods

`get_val(&self, c: &Context)`
returns a Result<i32, String> depending on if the register has an associated value
in the register table, the String is `"no value associated to register : {} "`