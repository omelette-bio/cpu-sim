```
register     = { "R" ~ ('0'..'8') }
digit        = { "$" ~ ('0'..'9') }
calc_opcode  = { "ADD" | "SUB" | "DIV" | "MUL" | "MOVE" }
stack_opcode = { "POP" | "PUSH" }
command      = {
    calc_opcode ~ " " ~ (digit | register) ~ " " ~ register
  | stack_opcode ~ " " ~ register
}
```

if the grammar is made, it should be like this
