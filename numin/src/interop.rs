macro_rules! MAKEINTRESOURCE {
    ($x:expr) => {
        PCWSTR::from_raw($x as u16 as _)
    };
}

pub(crate) use MAKEINTRESOURCE;

#[allow(non_snake_case)]
pub const fn MAKELANGID(primary: u32, sub: u32) -> u16 {
    (sub as u16) << 10 | (primary as u16)
}
