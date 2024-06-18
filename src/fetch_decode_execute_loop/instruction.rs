#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    raw: u16,
    nnn: u16,
    nn: u8,
    n: u8,
    x: u8,
    y: u8,
    opcode: u8,
}

impl Instruction {
    pub fn from(instruction: u16) -> Self {
        Self {
            raw: instruction,
            nnn: instruction & 0x0FFF,
            nn: (instruction & 0x00FF) as u8,
            n: (instruction & 0x000F) as u8,
            x: ((instruction & 0x0F00) >> 8) as u8,
            y: ((instruction & 0x00F0) >> 4) as u8,
            opcode: ((instruction & 0xF000) >> 12) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let instruction = Instruction::from(0x1234);
        assert_eq!(instruction.raw, 0x1234);
        assert_eq!(instruction.nnn, 0x234);
        assert_eq!(instruction.nn, 0x34);
        assert_eq!(instruction.n, 0x4);
        assert_eq!(instruction.x, 0x2);
        assert_eq!(instruction.y, 0x3);
        assert_eq!(instruction.opcode, 0x1);
    }

    #[test]
    fn test_from_2() {
        let instruction = Instruction::from(0x0ABC);
        assert_eq!(instruction.raw, 0x0ABC);
        assert_eq!(instruction.nnn, 0xABC);
        assert_eq!(instruction.nn, 0xBC);
        assert_eq!(instruction.n, 0xC);
        assert_eq!(instruction.x, 0xA);
        assert_eq!(instruction.y, 0xB);
        assert_eq!(instruction.opcode, 0x0);
    }

    #[test]
    fn test_from_3() {
        let instruction = Instruction::from(0x8ABC);
        assert_eq!(instruction.raw, 0x8ABC);
        assert_eq!(instruction.nnn, 0xABC);
        assert_eq!(instruction.nn, 0xBC);
        assert_eq!(instruction.n, 0xC);
        assert_eq!(instruction.x, 0xA);
        assert_eq!(instruction.y, 0xB);
        assert_eq!(instruction.opcode, 0x8);
    }

    #[test]
    fn test_from_4() {
        let instruction = Instruction::from(0x8ABC);
        assert_eq!(instruction.raw, 0x8ABC);
        assert_eq!(instruction.nnn, 0xABC);
        assert_eq!(instruction.nn, 0xBC);
        assert_eq!(instruction.n, 0xC);
        assert_eq!(instruction.x, 0xA);
        assert_eq!(instruction.y, 0xB);
        assert_eq!(instruction.opcode, 0x8);
    }

    #[test]
    fn test_from_5() {
        let instruction = Instruction::from(0x8ABC);
        assert_eq!(instruction.raw, 0x8ABC);
        assert_eq!(instruction.nnn, 0xABC);
        assert_eq!(instruction.nn, 0xBC);
        assert_eq!(instruction.n, 0xC);
        assert_eq!(instruction.x, 0xA);
        assert_eq!(instruction.y, 0xB);
        assert_eq!(instruction.opcode, 0x8);
    }
}
