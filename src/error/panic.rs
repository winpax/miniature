use super::{exit_immediately, log_error, set_exit_code};

const PANIC_EXIT_CODE: u32 = u32::from_le_bytes(*b"pnkd");

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    info.message()
        .as_str()
        .map(|msg| {
            log_error(msg).unwrap_or_default();
        })
        .unwrap_or_default();

    set_exit_code(PANIC_EXIT_CODE);
    exit_immediately();
}
