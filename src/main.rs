mod emulator;
mod hardware;

fn main() {
    emulator::Emulator::new().init();
}
