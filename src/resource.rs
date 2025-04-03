mod ids;

use widestring::WideString;
use windows::{
    core::PWSTR,
    Win32::{System::Environment::GetCommandLineW, UI::WindowsAndMessaging::LoadStringW},
};

pub struct ChildResource {
    pub path: WideString,
    pub args: WideString,
}

impl ChildResource {
    pub unsafe fn load() -> Self {
        let path = load_resource_string(ids::IDS_PATH);
        let args = load_resource_string(ids::IDS_ARGS);

        Self { path, args }
    }

    pub fn calculate_command(&self) -> WideString {
        let mut command_length: usize = 0;
        let path = self.path.as_ustr();
        let args = self.args.as_ustr();

        command_length += path.len() + 1;
        command_length += args.len() + 1;

        let commandline = unsafe { GetCommandLineW() };

        let program_length =
            unsafe { crate::interop::rcompute_program_length(commandline.as_wide()) };

        let given_command = &unsafe { commandline.as_wide() }[program_length..];

        command_length += given_command.len() + 1;

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

unsafe fn load_resource_string(id: u32) -> WideString {
    let mut buffer = [PWSTR::null(); 1];
    let buffer_pointer: PWSTR = PWSTR::from_raw(buffer.as_mut_ptr().cast());

    #[allow(clippy::cast_possible_wrap)]
    let characters = LoadStringW(None, id, buffer_pointer, 0) as usize;

    let [actual_string] = buffer;

    WideString::from_ptr(actual_string.as_ptr(), characters)
}
