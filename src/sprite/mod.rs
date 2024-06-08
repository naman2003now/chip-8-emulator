pub struct Sprite {
    height: u8,
    data: Vec<u8>,
}

impl Sprite {
    pub fn new(data: &[u8]) -> Self {
        Self {
            height: data.len() as u8,
            data: data.to_vec(),
        }
    }

    pub fn get_height(&self) -> u8 {
        self.height
    }

    pub fn get_row(&self, row: u8) -> u8 {
        self.data[row as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_new() {
        let sprite = Sprite::new(&[0xF0, 0x90, 0x90, 0x90, 0xF0]);
        assert_eq!(sprite.height, 5);
        assert_eq!(sprite.data, vec![0xF0, 0x90, 0x90, 0x90, 0xF0]);
    }

    #[test]
    fn sprite_get_height() {
        let sprite = Sprite::new(&[0xF0, 0x90, 0x90, 0x90, 0xF0]);
        assert_eq!(sprite.get_height(), 5);
    }

    #[test]
    fn sprite_get_row() {
        let sprite = Sprite::new(&[0xF0, 0x90, 0x90, 0x90, 0xF0]);
        assert_eq!(sprite.get_row(0), 0xF0);
        assert_eq!(sprite.get_row(1), 0x90);
        assert_eq!(sprite.get_row(2), 0x90);
        assert_eq!(sprite.get_row(3), 0x90);
        assert_eq!(sprite.get_row(4), 0xF0);
    }
}
