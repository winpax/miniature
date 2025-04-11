//! Both functions in this file are ported almost directly from the original <https://github.com/71/scoop-better-shimexe/blob/master/shim.c>

mod externfns {
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
}

unsafe extern "C" {
    pub fn loword(dword: usize) -> u16;
    pub fn hiword(dword: usize) -> u16;
}

pub unsafe fn compute_program_length(commandline: &[u16]) -> usize {
    let mut i = 0usize;

    if commandline[0] == ('"' as u16) {
        i += 1;
    }

    loop {
        i += 1;
        let char = commandline[i];

        // String already terminated
        if char == 0 {
            i -= 1;
            break;
        }
        // End of string
        if char == ('"' as u16) {
            // Skip the space after the closing quote
            i += 1;
            break;
        }
    }

    i
}
