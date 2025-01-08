use std::fmt::Display;

struct F32 {
    exact_value: f32,
    significant_digits: u8,
}
impl F32 {
    pub fn value(&self) -> (bool, Vec<u8>, Vec<u8>) {
        todo!()
    }
}
impl PartialEq for F32 {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
