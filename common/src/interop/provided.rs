trait Float:
    Copy
    + PartialOrd
    + core::ops::Mul<Output = Self>
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + Default
{
    fn is_nan(self) -> bool;
}

impl Float for f64 {
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}
impl Float for f32 {
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

#[unsafe(no_mangle)]
extern "C" fn wcslen(ptr: *const u16) -> usize {
    if ptr.is_null() {
        return 0;
    }
    let mut len = 0;
    while unsafe { *ptr.add(len) } != 0 {
        len += 1;
    }
    len
}

#[unsafe(no_mangle)]
extern "C" fn fma(x: f64, y: f64, z: f64) -> f64 {
    fma_generic(x, y, z)
}

#[unsafe(no_mangle)]
extern "C" fn fmaf(x: f32, y: f32, z: f32) -> f32 {
    fma_generic(x, y, z)
}

fn fma_generic<T: Float>(x: T, y: T, z: T) -> T {
    let result = x * y + z;
    if result.is_nan() {
        T::default()
    } else {
        result
    }
}

#[cfg(target_pointer_width = "32")]
mod x86;
