pub struct Registers {
    pub v_0: u8,
    pub v_1: u8,
    pub v_2: u8,
    pub v_3: u8,
    pub v_4: u8,
    pub v_5: u8,
    pub v_6: u8,
    pub v_7: u8,
    pub v_8: u8,
    pub v_9: u8,
    pub v_a: u8,
    pub v_b: u8,
    pub v_c: u8,
    pub v_d: u8,
    pub v_e: u8,
    pub v_f: u8, //carry flag, no borrow flag and pixel collision, is not to be used
}

impl Registers {
    fn new() -> Self {
        Registers {
            v_0: 0u8,
            v_1: 0u8,
            v_2: 0u8,
            v_3: 0u8,
            v_4: 0u8,
            v_5: 0u8,
            v_6: 0u8,
            v_7: 0u8,
            v_8: 0u8,
            v_9: 0u8,
            v_a: 0u8,
            v_b: 0u8,
            v_c: 0u8,
            v_d: 0u8,
            v_e: 0u8,
            v_f: 0u8,
        }
    }
}

pub struct System {
    pub registers: Registers,
    pub register_i: u8,
    pub pc: usize,
    pub memory: [u8; 4096],
    pub graphics: [u8; 64 * 32],
    pub stack: [u8; 16],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub input: [u8; 16],
}

impl System {
    pub fn new() -> Self {
        System {
            registers: Registers::new(),
            register_i: 0u8,
            pc: 0,
            memory: [0u8; 4096],
            graphics: [0u8; 64 * 32],
            stack: [0u8; 16],
            sp: 0,
            delay_timer: 0u8,
            sound_timer: 0u8,
            input: [0u8; 16],
        }
    }
}
