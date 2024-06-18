mod instruction;

use crate::emulator::component::Component;
use instruction::*;

pub struct FetchDecodeExecuteLoop;

impl Component for FetchDecodeExecuteLoop {
    fn clock(&mut self, hardware: &mut crate::hardware::Hardware) {
        let raw_instruction = Self::fetch(hardware);
        let instruction = Self::decode(raw_instruction);
        // Self::execute(instruction, hardware);
    }
}

impl FetchDecodeExecuteLoop {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn fetch(hardware: &mut crate::hardware::Hardware) -> u16 {
        let pc = hardware.pc;
        let most_significant_bits = hardware.memory[pc as usize] as u16;
        let least_significant_bits = hardware.memory[(pc + 1) as usize] as u16;
        let instruction = (most_significant_bits << 8) | least_significant_bits;
        hardware.pc += 2;
        instruction
    }

    fn decode(instruction: u16) -> Instruction {
        Instruction::from(instruction)
    }
}
