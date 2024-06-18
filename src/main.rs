extern crate sdl2;

mod emulator;
mod fetch_decode_execute_loop;
mod font;
mod hardware;
mod input_output;
mod sprite;
mod timers;

fn main() -> Result<(), String> {
    let context = sdl2::init()?;

    let rom = std::env::args().nth(1).expect("No rom file provided");
    emulator::Emulator::load(&rom)
        .register_component(font::Font::new())
        .register_component(input_output::display_output::DisplayOutput::new(
            800,
            400,
            context.clone(),
        )?)
        .register_component(input_output::event_listener::EventListener::new(context)?)
        .register_component(timers::TimerComponent::new())
        .register_component(fetch_decode_execute_loop::FetchDecodeExecuteLoop::new())
        .init()
        .run();
    Ok(())
}
