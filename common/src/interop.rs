//! Both functions in this file are ported almost directly from the original <https://github.com/71/scoop-better-shimexe/blob/master/shim.c>

#[cfg(feature = "crt_functions")]
mod provided;

unsafe extern "C" {
    pub fn loword(dword: usize) -> u16;
    pub fn hiword(dword: usize) -> u16;
}

#[must_use]
pub fn compute_program_length(commandline: &[u16]) -> usize {
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
