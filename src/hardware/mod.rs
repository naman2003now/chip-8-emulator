mod stack;
use stack::Stack;

pub struct Hardware {
    pub memory: [u8; 4096],
    pub display: [u64; 32],
    pub pc: u16,
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub registors: [u8; 16],
    pub power_on: bool,
    pub stack: Stack,
}

impl Hardware {
    pub fn new() -> Self {
        let memory = [0; 4096];
        let display = [0; 32];
        let pc = 0;
        let i = 0;
        let delay_timer = 0;
        let sound_timer = 0;
        let registors = [0; 16];
        let power_on = true;
        let stack = Stack::new();

        Self {
            memory,
            display,
            pc,
            i,
            delay_timer,
            sound_timer,
            registors,
            power_on,
            stack,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Hardware;
    use std::any::{type_name, type_name_of_val};
    use std::mem::size_of_val;

    #[test]
    fn memory_size() {
        let hardware = Hardware::new();
        assert_eq!(size_of_val(&hardware.memory), 4096);
    }

    #[test]
    fn display() {
        let hardware = Hardware::new();
        assert_eq!(hardware.display.len(), 32);
        assert_eq!(type_name_of_val(&hardware.display[0]), type_name::<u64>());
    }

    #[test]
    fn registers() {
        let hardware = Hardware::new();
        assert_eq!(hardware.registors.len(), 16);
        assert_eq!(type_name_of_val(&hardware.registors[0]), type_name::<u8>());
    }

    #[test]
    fn pc() {
        let hardware = Hardware::new();
        assert_eq!(type_name_of_val(&hardware.pc), type_name::<u16>());
    }

    #[test]
    fn i() {
        let hardware = Hardware::new();
        assert_eq!(type_name_of_val(&hardware.i), type_name::<u16>());
    }

    #[test]
    fn delay_timer() {
        let hardware = Hardware::new();
        assert_eq!(type_name_of_val(&hardware.delay_timer), type_name::<u8>());
    }

    #[test]
    fn sound_timer() {
        let hardware = Hardware::new();
        assert_eq!(type_name_of_val(&hardware.sound_timer), type_name::<u8>());
    }

    #[test]
    fn power_on() {
        let hardware = Hardware::new();
        assert_eq!(type_name_of_val(&hardware.power_on), type_name::<bool>());
    }
}
