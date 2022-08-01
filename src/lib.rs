mod system_components;
mod code_table;

use system_components::*;
use code_table::{decode_op_code, Op};

use std::thread;
use std::time;

impl System {

    pub fn emulate_cycle(&mut self) {
        let op_code = self.fetch_op_code();
        let decoded_op: Op = decode_op_code(op_code);

        decoded_op.execution.as_ref()(self, &decoded_op);

        //do the thing with the op code and inc PC

        self.update_timer();

        self.slow_down_cycle_time();
    }

    fn slow_down_cycle_time(&self) {
        thread::sleep(time::Duration::from_millis(10));
    }

    fn update_timer(&mut self) {
        self.delay_timer -= 1;
        self.sound_timer -= 1;
    }

    fn fetch_op_code(&self) -> u16 {
        let pc_position = self.pc as usize;
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
