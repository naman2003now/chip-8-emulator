use crate::emulator::component::Component;
use crate::hardware::Hardware;

pub struct TimerComponent;

impl TimerComponent {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Component for TimerComponent {
    fn cycle(&mut self, hardware: &mut Hardware) {
        if hardware.delay_timer > 0 {
            hardware.delay_timer -= 1;
        }

        if hardware.sound_timer > 0 {
            hardware.sound_timer -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::Hardware;

    #[test]
    fn test_timer_component_cycle() {
        let mut hardware = Hardware::new();
        hardware.delay_timer = 10;
        hardware.sound_timer = 20;

        let mut timer_component = TimerComponent::new();
        timer_component.cycle(&mut hardware);

        assert_eq!(hardware.delay_timer, 9);

        timer_component.cycle(&mut hardware);
        assert_eq!(hardware.sound_timer, 18);
    }
}
