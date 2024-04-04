# context documentation
the context struct is a struct composed (for now) of two variables and three methods

## structure
the struct is build like this 

```rust
#[derive(Debug)]
pub struct Context {
    register_table: HashMap<Registers, i32>,
    in_file: bool,
}
```

## attributes
the two variables that compose the struct are:

`register_table: HashMap<Registers, i32>` 
this represents the symbol table where a register is associated with a signed integer

`in_file: bool`
this variable is useful to tell the program to print or not depending on if we've given a file or not

## methods
the type has three simple methods

`new()` 
returns a new context element with an empty hashmap and in_file set to false

`change_file_context(&mut self)`
changes the variable in_file to the opposite

`is_in_file(&self)`
return in_file