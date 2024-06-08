extern crate sdl2;

mod emulator;
mod font;
mod hardware;
mod input_output;
mod sprite;

fn main() -> Result<(), String> {
    let context = sdl2::init()?;
    emulator::Emulator::new()
        .register_component(font::Font::new())
        .register_component(input_output::display::Display::new(
            800,
            400,
            context.clone(),
        )?)
        .register_component(input_output::event_listener::EventListener::new(context)?)
        .init()
        .run();
    Ok(())
}
