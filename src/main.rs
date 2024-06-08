mod emulator;
mod hardware;
mod sprite;

fn main() {
    emulator::Emulator::new().init();
}
