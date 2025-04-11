use super::exit_immediately;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // Note that this function is never called, because panics immediately abort the process.
    // exit_immediately()
    loop {}
}
