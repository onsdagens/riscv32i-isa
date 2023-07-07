use std::fmt::Debug;
use std::fmt;
pub enum Opcode{
    SYSTEM,
    MISCMEM,
    OP,
    OPIMM,
    STORE,
    LOAD,
    BRANCH,
    JAL,
    JALR,
    LUI,
    AUIPC
}
pub struct Instruction{
    opcode:Opcode,
}
impl Instruction{
    pub fn new(opcode:Opcode)->Instruction{
        Instruction{opcode:opcode}
    }
}
impl Debug for Instruction{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}",self.opcode)
    }
}
impl Debug for Opcode{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Opcode::AUIPC=>write!(f, "AUIPC"),
            Opcode::LUI=>write!(f, "LUI"),
            Opcode::JALR=>write!(f, "JALR"),
            Opcode::JAL=>write!(f, "JAL"),
            Opcode::BRANCH=>write!(f, "BRANCH"),
            Opcode::LOAD=>write!(f, "LOAD"),
            Opcode::STORE=>write!(f, "STORE"),
            Opcode::OPIMM=>write!(f, "OPIMM"),
            Opcode::OP=>write!(f, "OP"),
            Opcode::MISCMEM=>write!(f, "MISCMEM"),
            Opcode::SYSTEM=>write!(f, "SYSTEM"),
        }
    }
}
pub fn bits_to_opcode(bits:u8) -> Opcode{
    match(bits){
        0b1110011=>Opcode::SYSTEM,
        0b0001111=>Opcode::MISCMEM,
        0b0110011=>Opcode::OP,
        0b0010011=>Opcode::OPIMM,
        0b0100011=>Opcode::STORE,
        0b0000011=>Opcode::LOAD,
        0b1100011=>Opcode::BRANCH,
        0b1100111=>Opcode::JALR,
        0b1101111=>Opcode::JAL,
        0b0110111=>Opcode::LUI,
        0b0010111=>Opcode::AUIPC,
        _=>{
            panic!("Unknown opcode:{}", bits);
        }
    }
}
