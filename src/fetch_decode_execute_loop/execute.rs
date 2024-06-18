use super::*;
use crate::hardware::Hardware;

impl FetchDecodeExecuteLoop {
    pub fn execute(hardware: &mut Hardware, instruction: Instruction) {
        match instruction.opcode {
            0x00 => match instruction.nn {
                0xE0 => Self::clear_screen(hardware),
                _ => panic!("Unknown instruction: {:#X}", instruction.raw),
            },
            _ => panic!("Unknown instruction: {:#X}", instruction.raw),
        }
    }

    fn clear_screen(hardware: &mut Hardware) {
        for i in 0..hardware.display.len() {
            hardware.display[i] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_screen() {
        let mut hardware = Hardware::new();

        hardware.display[0] = 1;
        hardware.display[1] = 1;
        hardware.display[2] = 1;
        hardware.display[3] = 1;

        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x00E0));

        assert_eq!(hardware.display[0], 0);
        assert_eq!(hardware.display[1], 0);
        assert_eq!(hardware.display[2], 0);
        assert_eq!(hardware.display[3], 0);
    }
}
