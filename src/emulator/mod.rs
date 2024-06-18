pub mod component;

use crate::hardware::Hardware;
use std::time::Instant;

use std::fs::File;
use std::io::Read;

pub struct Emulator {
    hardware: Hardware,
    components: Vec<Box<dyn component::Component>>,
    time_of_previous_cycle: Instant,
}

impl Emulator {
    pub fn load(filename: &str) -> Self {
        let mut emulator = Emulator::new();

        let mut f = File::open(&filename).expect("no file found");
        let rom = &mut emulator.hardware.memory[0x200..];
        f.read(rom).expect("buffer overflow");
        emulator
    }

    pub fn new() -> Self {
        Self {
            hardware: Hardware::new(),
            components: Vec::new(),
            time_of_previous_cycle: Instant::now(),
        }
    }

    pub fn register_component(&mut self, component: Box<dyn component::Component>) -> &mut Self {
        self.components.push(component);
        self
    }

    pub fn init(&mut self) -> &mut Self {
        for component in &mut self.components {
            component.init(&mut self.hardware);
        }
        self
    }

    pub fn run(&mut self) {
        loop {
            if self.hardware.power_on == false {
                break;
            }
            for component in &mut self.components {
                component.clock(&mut self.hardware);
            }
            if (self.time_of_previous_cycle.elapsed().as_millis() as u64) < (1000 / 60) {
                continue;
            }
            self.cycle();
            self.time_of_previous_cycle = Instant::now();
        }
    }

    pub fn cycle(&mut self) {
        for component in &mut self.components {
            component.cycle(&mut self.hardware);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent;
    impl component::Component for TestComponent {
        fn init(&mut self, hardware: &mut Hardware) {
            hardware.pc = 0x100;
        }

        fn clock(&mut self, hardware: &mut Hardware) {
            hardware.pc += 1;
            hardware.power_on = false;
        }
    }

    #[test]
    fn test_emulator_new() {
        let emulator = Emulator::new();
        assert_eq!(emulator.hardware.pc, 0x200);
    }

    #[test]
    fn test_emulator_register_component() {
        let mut emulator = Emulator::new();
        emulator.register_component(Box::new(TestComponent));
        assert_eq!(emulator.components.len(), 1);
    }

    #[test]
    fn test_emulator_init() {
        let mut emulator = Emulator::new();
        emulator.register_component(Box::new(TestComponent));
        emulator.init();
        assert_eq!(emulator.hardware.pc, 0x100);
    }

    #[test]
    fn test_emulator_run() {
        let mut emulator = Emulator::new();
        emulator.register_component(Box::new(TestComponent));
        emulator.init();
        emulator.run();
        assert_eq!(emulator.hardware.pc, 0x101);
    }

    struct TimedComponenet;
    impl component::Component for TimedComponenet {
        fn init(&mut self, hardware: &mut Hardware) {
            hardware.pc = 0;
        }

        fn cycle(&mut self, hardware: &mut Hardware) {
            if hardware.pc > 60 {
                hardware.power_on = false;
                return;
            }
            hardware.pc += 1;
        }
    }

    #[test]
    fn test_cycle_time() {
        let mut emulator = Emulator::new();
        let start = Instant::now();
        emulator.register_component(Box::new(TimedComponenet));
        emulator.init();
        emulator.run();
        assert_eq!(emulator.hardware.pc, 61);
        println!("Elapsed: {}", start.elapsed().as_millis());
        assert!((start.elapsed().as_millis() as i64 - 1000).abs() < 10);
    }
}
