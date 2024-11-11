pub trait Interruptable {
    fn interrupt(&self) -> Option<u32>;
}
