mod ids;

use core::ffi::c_int;

use alloc::{borrow::ToOwned, vec::Vec};
use widestring::{U16CStr, U16CString, WideString};
use windows::{
    core::PWSTR,
    Win32::{
        Foundation::{LocalFree, HLOCAL},
        System::Environment::GetCommandLineW,
        UI::{Shell::CommandLineToArgvW, WindowsAndMessaging::LoadStringW},
    },
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
        let mut command_length: usize = 0;
        let path = self.path.as_ustr();
        let pretty_path = path.to_string_lossy();
        let args = self.args.as_ustr();
        let pretty_args = args.to_string_lossy();

        command_length += path.len() + 1;
        command_length += args.len() + 1;

        // let commandline = unsafe { GetCommandLineW() };
        // let (cl_arguments_raw, arg_ptr) = {
        //     let mut arg_count: c_int = 0;

        //     assert!((arg_count >= 0), "CommandLineToArgvW failed");

        //     let arg_ptr = unsafe { CommandLineToArgvW(commandline, &mut arg_count) };

        //     (
        //         #[allow(clippy::cast_sign_loss)]
        //         unsafe {
        //             core::slice::from_raw_parts(arg_ptr.cast_const(), arg_count as usize)
        //         },
        //         arg_ptr,
        //     )
        // };

        // let given_command = {
        //     let mut arguments = cl_arguments_raw.iter().map(|raw_arg| {
        //         unsafe { U16CStr::from_slice_truncate(raw_arg.as_wide()) }.expect("null terminated")
        //     });

        //     let arguments_length = arguments.by_ref().fold(0, |acc, arg| acc + arg.len() + 1);

        //     let given_command = arguments.fold(
        //         WideString::with_capacity(arguments_length),
        //         |mut acc, arg| {
        //             acc.push_slice(arg);
        //             acc.push_char(' ');
        //             acc
        //         },
        //     );

        //     unsafe { LocalFree(Some(arg_ptr as HLOCAL)) };

        //     given_command
        // };

        // let program_length =
        //     unsafe { crate::interop::rcompute_program_length(commandline.as_wide()) };

        // let given_command = &unsafe { commandline.as_wide() }[program_length..];

        #[allow(static_mut_refs)]
        let cli_args = unsafe { crate::ARGS.args() };
        let given_command = cli_args.iter().fold(WideString::new(), |mut acc, arg| {
            acc.push_slice(arg);
            acc.push_char(' ');
            acc
        });

        let pretty_given_command = given_command.to_string_lossy();

        command_length += given_command.len() + 1;

        let mut command = WideString::with_capacity(command_length);
        command.push(path);
        command.push_char(' ');
        command.push(args);
        command.push_char(' ');
        command.push_slice(given_command);
        command.push_char(' ');

        let pretty_command = command.to_string_lossy();

        command
    }
}

unsafe fn load_resource_string(id: u32) -> widestring::U16CString {
    let mut buffer = [PWSTR::null(); 1];
    let buffer_pointer: PWSTR = PWSTR::from_raw(buffer.as_mut_ptr().cast());

    #[allow(clippy::cast_possible_wrap)]
    let characters = LoadStringW(None, id, buffer_pointer, 0) as usize;

    let [actual_string] = buffer;

    let output_string = widestring::U16CStr::from_ptr(actual_string.as_ptr(), characters as usize)
        .unwrap()
        .to_owned();

    let pretty_string = output_string.to_string_lossy();

    output_string
}
