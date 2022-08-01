#[derive(Copy, Clone)]
pub enum OpCodeType {
    Call,
    Display,
    Flow,
    Cond,
    BitOp,
    Assig,
    Math,
    MEM,
    Rand,
    KeyOp,
    Timer,
    Sound,
    BCD,
}

pub struct OpCode {
    pub op_code: u16,
    pub op_code_type: OpCodeType,
}

pub struct OpCodeTable {
    pub op_code_table: [OpCode; 15],
}

impl OpCodeTable {
    pub fn new() -> Self {
        todo!()
    }

    pub fn decode_op_code(&self, op_code: u16) -> OpCode {
        todo!()
    }
}
