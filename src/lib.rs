mod op_code_mod;
mod system_components;
mod code_table;

use system_components::*;
use code_table::{decode_op_code, Op};

impl System {

    pub fn emulate_cycle(&self) {
        let op_code = self.fetch_op_code();
        let decoded: Op = decode_op_code(op_code);

        println!("{:?}", decoded);

        self.update_timer();
    }

    fn update_timer(&self) {
        todo!()
    }

    fn fetch_op_code(&self) -> u16 {
        let pc_position = self.pc;
        let first_half = self.memory[pc_position] as u16;
        let second_half = self.memory[pc_position + 1] as u16; 
        let op_code = first_half << 8 | second_half;
        op_code
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_fetch_op_code() {
        let mut system = super::System::new();
        system.memory[0] = 0xA2;
        system.memory[1] = 0xF0;
        let op_code = system.fetch_op_code();
        assert_eq!(op_code, 0xA2F0);
    }
}
