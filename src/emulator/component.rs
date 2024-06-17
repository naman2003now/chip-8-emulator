pub trait Component {
    fn init(&mut self, _hardware: &mut crate::hardware::Hardware) {}
    fn clock(&mut self, _hardware: &mut crate::hardware::Hardware) {}
    fn cycle(&mut self, _hardware: &mut crate::hardware::Hardware) {}
}
