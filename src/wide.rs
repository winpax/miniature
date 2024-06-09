use alloc::vec::Vec;
use widestring::WideCString;

pub trait Widen {
    fn widen(&self) -> Vec<u16>;
}

impl Widen for str {
    fn widen(&self) -> Vec<u16> {
        self.encode_utf16().collect()
    }
}

impl Widen for [u8] {
    fn widen(&self) -> Vec<u16> {
        self.iter().copied().map(u16::from).collect()
    }
}

impl Widen for [u16] {
    fn widen(&self) -> Vec<u16> {
        self.to_vec()
    }
}
