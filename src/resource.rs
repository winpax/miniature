mod ids;

use widestring::{U16CString, WideString};
use windows::{
    core::PWSTR,
    Win32::{System::Environment::GetCommandLineW, UI::WindowsAndMessaging::LoadStringW},
};

use crate::MAX_PATH;

pub struct ChildResource {
    pub path: U16CString,
    pub args: U16CString,
}

impl ChildResource {
    pub unsafe fn load() -> Self {
        let path = load_resource_string(ids::IDS_PATH);
        let args = load_resource_string(ids::IDS_ARGS);

        Self { path, args }
    }

    pub fn calculate_command(&self) -> WideString {
        let mut command_length: usize = 256;
        let path = self.path.as_ustr();
        let args = self.args.as_ustr();

        command_length += path.len();
        command_length += args.len() + 1;

        let commandline = unsafe { GetCommandLineW() };

        let program_length =
            unsafe { crate::interop::rcompute_program_length(commandline.as_wide()) };

        let given_command = &unsafe { commandline.as_wide() }[program_length..];

        command_length += given_command.len();

        let mut command = WideString::with_capacity(command_length);
        command.push(path);
        command.push_char(' ');
        command.push(args);
        command.push_char(' ');
        command.push_slice(given_command);
        command.push_char(' ');

        command
    }
}

unsafe fn load_resource_string(id: u32) -> widestring::U16CString {
    // This will allocate a buffer of size MAX_PATH and fill it with zeros.
    // The buffer is then passed to LoadStringW, which will fill it with the string
    // corresponding to the given resource ID.
    // This buffer is freed at the end of the scope, and the resulting string is only as long
    // as the resource string is. While this is not optimal, the large(ish) allocation only exists in this scope
    let mut buffer = alloc::vec![0; MAX_PATH];
    let string_pointer: PWSTR = PWSTR::from_raw(buffer.as_mut_ptr());

    #[allow(clippy::cast_possible_wrap)]
    LoadStringW(None, id, string_pointer, MAX_PATH as i32);

    widestring::U16CString::from_vec_truncate(buffer)
}
