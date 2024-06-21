use super::*;
use crate::hardware::Hardware;

impl FetchDecodeExecuteLoop {
    pub fn execute(hardware: &mut Hardware, instruction: Instruction) {
        match instruction.opcode {
            0x0 => match instruction.nn {
                0xE0 => Self::clear_screen(hardware),
                0xEE => hardware.pc = hardware.stack.pop(),
                _ => {}
            },
            0x1 => hardware.pc = instruction.nnn,
            0x2 => Self::call_subroutine(hardware, instruction),
            0x3 => {
                if hardware.registors[instruction.x as usize] == instruction.nn {
                    hardware.pc += 2;
                }
            }
            0x4 => {
                if hardware.registors[instruction.x as usize] != instruction.nn {
                    hardware.pc += 2;
                }
            }
            0x5 => {
                if hardware.registors[instruction.x as usize]
                    == hardware.registors[instruction.y as usize]
                {
                    hardware.pc += 2;
                }
            }
            0x6 => Self::set_register(hardware, instruction),
            0x7 => Self::add_to_register(hardware, instruction),
            0x9 => {
                if hardware.registors[instruction.x as usize]
                    != hardware.registors[instruction.y as usize]
                {
                    hardware.pc += 2;
                }
            }
            0xA => Self::set_index(hardware, instruction),
            0xD => Self::display(hardware, instruction),
            0xF => match instruction.nn {
                0x07 => hardware.registors[instruction.x as usize] = hardware.delay_timer,
                0x15 => hardware.delay_timer = hardware.registors[instruction.x as usize],
                0x18 => hardware.sound_timer = hardware.registors[instruction.x as usize],
                0x1E => hardware.i += hardware.registors[instruction.x as usize] as u16,
                0x29 => hardware.i = hardware.registors[instruction.x as usize] as u16 * 5,
                0x33 => {
                    let value = hardware.registors[instruction.x as usize];
                    hardware.memory[hardware.i as usize] = value / 100;
                    hardware.memory[hardware.i as usize + 1] = (value / 10) % 10;
                    hardware.memory[hardware.i as usize + 2] = value % 10;
                }
                0x55 => {
                    for i in 0..=instruction.x {
                        hardware.memory[hardware.i as usize + i as usize] =
                            hardware.registors[i as usize];
                    }
                }
                0x65 => {
                    for i in 0..=instruction.x {
                        hardware.registors[i as usize] =
                            hardware.memory[hardware.i as usize + i as usize];
                    }
                }
                _ => {}
            },
            _ => panic!("Unknown instruction: {:#X}", instruction.raw),
        }
    }

    fn call_subroutine(hardware: &mut Hardware, instruction: Instruction) {
        hardware.stack.push(hardware.pc);
        hardware.pc = instruction.nnn;
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

    #[test]
    fn test_call_subroutine() {
        let mut hardware = Hardware::new();

        hardware.pc = 0x200;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x2123));

        assert_eq!(hardware.pc, 0x123);
        assert_eq!(hardware.stack.pop(), 0x200);
    }

    #[test]
    fn test_return_subroutine() {
        let mut hardware = Hardware::new();

        hardware.pc = 0x200;
        hardware.stack.push(0x300);
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x00EE));

        assert_eq!(hardware.pc, 0x300);
    }

    #[test]
    fn test_if_equal() {
        let mut hardware = Hardware::new();

        hardware.registors[0] = 0x01;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x3010));
        assert_eq!(hardware.pc, 0x200);

        hardware.registors[0] = 0x10;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x3010));
        assert_eq!(hardware.pc, 0x202);
    }

    #[test]
    fn test_if_not_equal() {
        let mut hardware = Hardware::new();

        hardware.registors[0] = 0x01;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x4010));
        assert_eq!(hardware.pc, 0x202);

        hardware.registors[0] = 0x10;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x4010));
        assert_eq!(hardware.pc, 0x202);
    }

    #[test]
    fn test_if_equal_registers() {
        let mut hardware = Hardware::new();

        hardware.registors[0] = 1;
        hardware.registors[1] = 1;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x5010));
        assert_eq!(hardware.pc, 0x202);

        hardware.registors[0] = 1;
        hardware.registors[1] = 2;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x5010));
        assert_eq!(hardware.pc, 0x202);

        hardware.registors[0] = 2;
        hardware.registors[1] = 1;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x5010));
        assert_eq!(hardware.pc, 0x202);

        hardware.registors[0] = 2;
        hardware.registors[1] = 2;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x5010));
        assert_eq!(hardware.pc, 0x204);
    }

    #[test]
    fn test_if_not_equal_registers() {
        let mut hardware = Hardware::new();

        hardware.registors[0] = 1;
        hardware.registors[1] = 1;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x9010));
        assert_eq!(hardware.pc, 0x200);

        hardware.registors[0] = 1;
        hardware.registors[1] = 2;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x9010));
        assert_eq!(hardware.pc, 0x202);

        hardware.registors[0] = 2;
        hardware.registors[1] = 1;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x9010));
        assert_eq!(hardware.pc, 0x204);

        hardware.registors[0] = 2;
        hardware.registors[1] = 2;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0x9010));
        assert_eq!(hardware.pc, 0x204);
    }

    #[test]
    fn test_set_delay_timer() {
        let mut hardware = Hardware::new();

        hardware.registors[0] = 0x10;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0xF015));
        assert_eq!(hardware.delay_timer, 0x10);
    }

    #[test]
    fn test_set_sound_timer() {
        let mut hardware = Hardware::new();

        hardware.registors[0] = 0x10;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0xF018));
        assert_eq!(hardware.sound_timer, 0x10);
    }

    #[test]
    fn test_add_to_index() {
        let mut hardware = Hardware::new();

        hardware.i = 0x10;
        hardware.registors[0] = 0x10;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0xF01E));
        assert_eq!(hardware.i, 0x20);
    }

    #[test]
    fn test_set_bcd() {
        let mut hardware = Hardware::new();

        hardware.i = 0x10;
        hardware.registors[0] = 123;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0xF033));
        assert_eq!(hardware.memory[0x10], 1);
        assert_eq!(hardware.memory[0x11], 2);
        assert_eq!(hardware.memory[0x12], 3);
    }

    #[test]
    fn test_store_registers() {
        let mut hardware = Hardware::new();

        hardware.i = 0x10;
        hardware.registors[0] = 1;
        hardware.registors[1] = 2;
        hardware.registors[2] = 3;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0xF255));
        assert_eq!(hardware.memory[0x10], 1);
        assert_eq!(hardware.memory[0x11], 2);
        assert_eq!(hardware.memory[0x12], 3);
    }

    #[test]
    fn test_load_registers() {
        let mut hardware = Hardware::new();

        hardware.i = 0x10;
        hardware.memory[0x10] = 1;
        hardware.memory[0x11] = 2;
        hardware.memory[0x12] = 3;
        FetchDecodeExecuteLoop::execute(&mut hardware, Instruction::from(0xF265));
        assert_eq!(hardware.registors[0], 1);
        assert_eq!(hardware.registors[1], 2);
        assert_eq!(hardware.registors[2], 3);
    }
}
