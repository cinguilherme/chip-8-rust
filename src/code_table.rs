use crate::system_components;

pub struct Op {
    pub code: u16,
    pub decoded: u16,
    pub _type: String,
    pub address: u16,
    pub logic: String,
    pub execution: Box<dyn Fn(&mut system_components::System, &Op)>,
}

struct OpTable {
    op_table: [Op; 35],
}

pub fn init_table() {}

pub fn decode_op_code(op_code: u16) -> Op {

    match op_code {
        0x00E0 => Op {
            code: op_code,
            decoded: op_code & 0x0FFF,
            _type: "Display".to_string(),
            address: 0,
            logic: "Clear the display".to_string(),
            execution: Box::new(move | system, decoded | {
                system.pc += 2;
            }),
        },
        0x00EE => Op {
            code: op_code,
            decoded: op_code & 0x0FFF,
            _type: "Flow".to_string(),
            address: 0,
            logic: "Return from a subroutine".to_string(),
            execution: Box::new(move | system, decoded | {
                
            }),
        },
        x if (x >= 0xA000 && x < 0xB000) => Op {
            code: op_code,
            decoded: op_code & 0x0FFF,
            _type: "MEM".to_string(),
            address: op_code & 0x0FFF,
            logic: "Sets I to the address NNN".to_string(),
            execution: Box::new(move | system, decoded | {
                system.register_i = decoded.address;
                system.pc += 2;
            }),
        },
        _ => Op {
            code: op_code,
            decoded: op_code & 0x0FFF,
            _type: "Unknown".to_string(),
            address: 0,
            logic: "Unknown".to_string(),
            execution: Box::new(move | system, decoded | {
                println!("No OP, code not recognized found");
            }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_display() {
        let op_code = 0x00E0;

        let decoded = decode_op_code(op_code);

        assert_eq!(decoded.code, 0x00E0);
        assert_eq!(decoded.decoded, 0x0E0);
        assert_eq!(decoded._type, "Display");
    }

    #[test]
    fn it_works_flow() {
        let op_code = 0x00EE;

        let decoded = decode_op_code(op_code);

        assert_eq!(decoded.code, 0x00EE);
        assert_eq!(decoded.decoded, 0x0EE);
        assert_eq!(decoded._type, "Flow");
    }

    #[test]
    fn it_works_set_i_MEM() {
        let op_code = 0xA2F0;

        let decoded = decode_op_code(op_code);

        assert_eq!(decoded.code, 0xA2F0);
        assert_eq!(decoded.decoded, 0x2F0);
        assert_eq!(decoded._type, "MEM");
    }
}
