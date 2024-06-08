pub trait Component {
    fn init(&self, hardware: &mut crate::hardware::Hardware);
    fn clock(&self, hardware: &mut crate::hardware::Hardware);
}
