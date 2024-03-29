register = { "R" ~ ('0'..'8') }
digit    = { "$" ~ ('0'..'9') }
opcode   = { "ADD" | "SUB" }
command  = { opcode ~ " " ~ (digit | register) ~ " " ~ register }

if the grammar is made, it should be like this
