use crate::emulator::component::Component;

extern crate sdl2;
use sdl2::pixels::Color;

pub struct Display {
    width: u32,
    height: u32,

    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn new(width: u32, height: u32, context: sdl2::Sdl) -> Result<Self, String> {
        let video_subsystem = context.video()?;

        let window = video_subsystem
            .window("CHIP - 8 Emulator", width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::RGB(255, 0, 0));

        Ok(Self {
            width,
            height,
            canvas,
        })
    }
}

impl Component for Display {
    fn clock(&mut self, _hardware: &mut crate::hardware::Hardware) {
        self.canvas.clear();
        self.canvas.present();
    }
}
