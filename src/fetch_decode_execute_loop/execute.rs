use super::*;
use crate::hardware::Hardware;

impl FetchDecodeExecuteLoop {
    pub fn execute(hardware: &mut Hardware, instruction: Instruction) {
        match instruction.opcode {
            0x0 => match instruction.nn {
                0xE0 => Self::clear_screen(hardware),
                _ => {}
            },
            0x1 => hardware.pc = instruction.nnn,
            0x6 => Self::set_register(hardware, instruction),
            0x7 => Self::add_to_register(hardware, instruction),
            0xA => Self::set_index(hardware, instruction),
            0xD => Self::display(hardware, instruction),
            _ => panic!("Unknown instruction: {:#X}", instruction.raw),
        }
    }

    fn clear_screen(hardware: &mut Hardware) {
        for i in 0..hardware.display.len() {
            hardware.display[i] = 0;
        }
    }

    fn set_register(hardware: &mut Hardware, instruction: Instruction) {
        hardware.registors[instruction.x as usize] = instruction.nn;
    }

    fn add_to_register(hardware: &mut Hardware, instruction: Instruction) {
        hardware.registors[instruction.x as usize] += instruction.nn;
    }

    fn set_index(hardware: &mut Hardware, instruction: Instruction) {
        hardware.i = instruction.nnn;
    }

    fn display(hardware: &mut Hardware, instruction: Instruction) {
        let x = hardware.registors[instruction.x as usize];
        let y = hardware.registors[instruction.y as usize];
        let height = instruction.n as usize;

        hardware.registors[0xF] = 0;
        let index = hardware.i as usize;
        for row in 0..height {
            let x = (x % 64) as usize;
            let y = (y % 32) as usize;
            let sprite = hardware.memory[index + row];
            for col in 0..8 {
                if (sprite & (0x80 >> col)) != 0 {
                    let display_index = (x + col + ((y + row) * 64)) as usize;
                    if hardware.display[display_index] == 1 {
                        hardware.registors[0xF] = 1;
                    }
                    hardware.display[display_index] ^= 1;
                }
            }
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

    #[test]
    fn test_jump() {
        let mut hardware = Hardware::new();

        hardware.pc = 0x200;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x1234));

        assert_eq!(hardware.pc, 0x234);
    }

    #[test]
    fn test_set_register() {
        let mut hardware = Hardware::new();

        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x6001));
        assert_eq!(hardware.registors[0], 1);

        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x6102));
        assert_eq!(hardware.registors[1], 2);
    }

    #[test]
    fn test_add_to_register() {
        let mut hardware = Hardware::new();

        hardware.registors[0] = 1;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x7001));
        assert_eq!(hardware.registors[0], 2);

        hardware.registors[1] = 2;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x7102));
        assert_eq!(hardware.registors[1], 4);
    }

    #[test]
    fn test_set_index() {
        let mut hardware = Hardware::new();

        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0xA123));
        assert_eq!(hardware.i, 0x123);
    }

    #[test]
    fn test_display() {
        let mut hardware = Hardware::new();

        hardware.i = 0;
        hardware.registors[0] = 0;
        hardware.registors[1] = 0;
        hardware.memory[0] = 0b11111111;

        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0xD011));

        assert_eq!(hardware.display[0], 1);
        assert_eq!(hardware.display[1], 1);
        assert_eq!(hardware.display[2], 1);
        assert_eq!(hardware.display[3], 1);
        assert_eq!(hardware.display[4], 1);
        assert_eq!(hardware.display[5], 1);
        assert_eq!(hardware.display[6], 1);
        assert_eq!(hardware.display[7], 1);
    }
}
