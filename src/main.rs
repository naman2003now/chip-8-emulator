mod emulator;
mod font;
mod hardware;
mod sprite;

fn main() {
    emulator::Emulator::new()
        .register_component(Box::new(font::Font::new()))
        .init();
}
