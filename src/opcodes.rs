use crate::data_manipulation::{Registers, Value};

#[derive(Debug, PartialOrd, PartialEq , Clone)]
pub enum OpCode {
	ADD(Value, Registers), SUB(Value, Registers), MUL(Value, Registers), DIV(Value, Registers), MOVE(Value, Registers),
	AND(Value, Registers), OR(Value, Registers), NOT(Registers),
	POP(Registers), PUSH(Registers),
	TLT(Value, Registers), TEQ(Value, Registers), TLE(Value, Registers),
	JUMP(Value), BEZ(Value, Registers), BNEZ(Value, Registers), GOTO(String), LABEL(String),
	PRINTF(Registers)
}

/*
pub enum OpCode {
	ADD(Registers, Registers, Registers), ADDC(Registers, i16, Registers),
	SUB(Registers, Registers, Registers), SUBC(Registers, i16, Registers),
	MUL(Registers, Registers, Registers), MULC(Registers, i16, Registers),
	DIV(Registers, Registers, Registers), DIVC(Registers, i16, Registers),
	AND(Registers, Registers, Registers), ANDC(Registers, i16, Registers),
	OR(Registers, Registers, Registers), ORC(Registers, i16, Registers),

}
*/


// OK objectif refaire les opcodes
// en mode euuuuuh soit rester sur du deux adresses (jsp j'ai pas envie en v2v)
// ou alors rester sur un truc beaucoup plus fixe en mode calculer un prog 32 bits par exemple
// et check combien d'opcodes je peux faire, combien de registres je peux avoir etc etc tu captes

// 19 opcodes = 5 bits pour les stocker donc 27 bits restants pour les operations

// ALU R1, R2, Rd qui serait 5 + 4 + 4 + 4 = 17 bits et donc 17/32 qui sont utilises
// ALUC R1, const, Rd qui serait donc 5 + 4 + 4 = 13 et donc 19 bits pour avoir l'offset on peut se caler un petit 16 bits pour l'offset
// marche aussi pour les operations logiques

// pour les operations POP et PUSH faut voir