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
    pub op_code: String,
    pub op_code_type: OpCodeType,
}

pub struct OpCodeTable {
    pub op_code_table: [OpCode; 15],
}

impl OpCodeTable {
    pub fn new() -> Self {
        OpCodeTable {
            op_code_table: [
                OpCode {
                    op_code: "00E0".to_string(),
                    op_code_type: OpCodeType::Display,
                },
                OpCode {
                    op_code: "00EE".to_string(),
                    op_code_type: OpCodeType::Flow,
                },
                OpCode {
                    op_code: "1NNN".to_string(),
                    op_code_type: OpCodeType::Flow,
                },
                OpCode {
                    op_code: "2NNN".to_string(),
                    op_code_type: OpCodeType::Flow,
                },
                OpCode {
                    op_code: "3XNN".to_string(),
                    op_code_type: OpCodeType::Cond,
                },
                OpCode {
                    op_code: "4XNN".to_string(),
                    op_code_type: OpCodeType::Cond,
                },
                OpCode {
                    op_code: "5XY0".to_string(),
                    op_code_type: OpCodeType::Cond,
                },
                OpCode {
                    op_code: "6XNN".to_string(),
                    op_code_type: OpCodeType::Assig,
                },
                OpCode {
                    op_code: "7XNN".to_string(),
                    op_code_type: OpCodeType::Assig,
                },
                OpCode {
                    op_code: "8XY0".to_string(),
                    op_code_type: OpCodeType::Math,
                },
                OpCode {
                    op_code: "8XY1".to_string(),
                    op_code_type: OpCodeType::Math,
                },
                OpCode {
                    op_code: "8XY2".to_string(),
                    op_code_type: OpCodeType::Math,
                },
                OpCode {
                    op_code: "8XY3".to_string(),
                    op_code_type: OpCodeType::Math,
                },
                OpCode {
                    op_code: "8XY4".to_string(),
                    op_code_type: OpCodeType::Math,
                },
                OpCode {
                    op_code: "8XY5".to_string(),
                    op_code_type: OpCodeType::Math,
                },
            ],
        }
    }

    pub fn decode_op_code(&self, op_code: u16) -> OpCode {
        let op_code_string = format!("{:04X}", op_code);
        let op_code_type = self.op_code_table
            .iter()
            .find(|op_code| op_code.op_code == op_code_string)
            .unwrap()
            .op_code_type;
        OpCode {
            op_code: op_code_string,
            op_code_type,
        }
    }
}
