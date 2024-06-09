use crate::emulator::component::Component;

extern crate sdl2;
use sdl2::pixels::Color;

pub struct DisplayOutput {
    width: u32,
    height: u32,

    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl DisplayOutput {
    pub fn new(width: u32, height: u32, context: sdl2::Sdl) -> Result<Box<Self>, String> {
        let video_subsystem = context.video()?;

        let window = video_subsystem
            .window("CHIP - 8 Emulator", width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Box::new(Self {
            width,
            height,
            canvas,
        }))
    }

    fn color_pixel(&mut self, x: u8, y: u8) {
        let x = x as u32;
        let y = y as u32;
        let cell_height = self.height / 32;
        let cell_width = self.width / 64;
        self.canvas.set_draw_color(Color::RGB(240, 240, 240));
        let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(
            (x * cell_height) as i32,
            (y * cell_width) as i32,
            cell_height,
            cell_width,
        ));
    }
}

impl Component for DisplayOutput {
    fn clock(&mut self, hardware: &mut crate::hardware::Hardware) {
        self.canvas.set_draw_color(Color::RGB(15, 15, 15));
        self.canvas.clear();
        for y in 0..32 {
            let row = hardware.display[y as usize];
            for x in 0..64 {
                if (row >> (63 - x)) & 1 == 1 {
                    self.color_pixel(x as u8, y);
                }
            }
        }
        self.canvas.present();
    }
}
