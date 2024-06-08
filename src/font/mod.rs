use crate::emulator::component::Component;
use crate::sprite::Sprite;

pub struct Font {
    sprites: Vec<Sprite>,
}

impl Font {
    pub fn new() -> Self {
        let mut sprites = Vec::new();

        sprites.push(Sprite::new(&[0xF0, 0x90, 0x90, 0x90, 0xF0])); // 0
        sprites.push(Sprite::new(&[0x20, 0x60, 0x20, 0x20, 0x70])); // 1
        sprites.push(Sprite::new(&[0xF0, 0x10, 0xF0, 0x80, 0xF0])); // 2
        sprites.push(Sprite::new(&[0xF0, 0x10, 0xF0, 0x10, 0xF0])); // 3
        sprites.push(Sprite::new(&[0x90, 0x90, 0xF0, 0x10, 0x10])); // 4
        sprites.push(Sprite::new(&[0xF0, 0x80, 0xF0, 0x10, 0xF0])); // 5
        sprites.push(Sprite::new(&[0xF0, 0x80, 0xF0, 0x90, 0xF0])); // 6
        sprites.push(Sprite::new(&[0xF0, 0x10, 0x20, 0x40, 0x40])); // 7
        sprites.push(Sprite::new(&[0xF0, 0x90, 0xF0, 0x90, 0xF0])); // 8
        sprites.push(Sprite::new(&[0xF0, 0x90, 0xF0, 0x10, 0xF0])); // 9
        sprites.push(Sprite::new(&[0xF0, 0x90, 0xF0, 0x90, 0x90])); // A
        sprites.push(Sprite::new(&[0xE0, 0x90, 0xE0, 0x90, 0xE0])); // B
        sprites.push(Sprite::new(&[0xF0, 0x80, 0x80, 0x80, 0xF0])); // C
        sprites.push(Sprite::new(&[0xE0, 0x90, 0x90, 0x90, 0xE0])); // D
        sprites.push(Sprite::new(&[0xF0, 0x80, 0xF0, 0x80, 0xF0])); // E
        sprites.push(Sprite::new(&[0xF0, 0x80, 0xF0, 0x80, 0x80])); // F

        Self { sprites }
    }
}

impl Component for Font {
    fn init(&self, hardware: &mut crate::hardware::Hardware) {
        let start_memory = 0x50;
        for (i, sprite) in self.sprites.iter().enumerate() {
            for row in 0..sprite.get_height() {
                hardware.memory[start_memory + i * 5 + row as usize] = sprite.get_row(row);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_new() {
        let font = Font::new();
        assert_eq!(font.sprites.len(), 16);
    }

    #[test]
    fn font_init() {
        let font = Font::new();
        let mut hardware = crate::hardware::Hardware::new();
        font.init(&mut hardware);
        assert_eq!(hardware.memory[0x50], 0xF0);
        assert_eq!(hardware.memory[0x51], 0x90);
        assert_eq!(hardware.memory[0x52], 0x90);
        assert_eq!(hardware.memory[0x53], 0x90);
        assert_eq!(hardware.memory[0x54], 0xF0);

        assert_eq!(hardware.memory[0x9B], 0xF0);
        assert_eq!(hardware.memory[0x9C], 0x80);
        assert_eq!(hardware.memory[0x9D], 0xF0);
        assert_eq!(hardware.memory[0x9E], 0x80);
        assert_eq!(hardware.memory[0x9F], 0x80);
    }
}
