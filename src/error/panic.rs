use crate::error::{ExitCode, ExitCodeReason, exit_immediately, log_error};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    info.message()
        .as_str()
        .map(|msg| {
            log_error(msg).unwrap_or_default();
        })
        .unwrap_or_default();

    ExitCode::set_code(ExitCode::PANIC_EXIT_CODE);
    ExitCode::set_reason(ExitCodeReason::Panic);
    exit_immediately();
}
