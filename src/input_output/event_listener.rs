extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::emulator::component::Component;

pub struct EventListener {
    event_pump: sdl2::EventPump,
}

impl EventListener {
    pub fn new(context: sdl2::Sdl) -> Result<Box<Self>, String> {
        let event_pump = context.event_pump()?;
        Ok(Box::new(Self { event_pump }))
    }
}

impl Component for EventListener {
    fn clock(&mut self, hardware: &mut crate::hardware::Hardware) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => hardware.power_on = false,
                _ => {}
            }
        }
    }
}
