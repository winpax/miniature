#![warn(clippy::all, clippy::pedantic)]
#![no_main]
#![feature(let_chains)]

extern crate alloc;

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

fn ctrl_handler(ctrl_type: u32) -> bool {
    use windows::Win32::System::Console::{
        CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT, CTRL_C_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT,
    };

    matches!(
        ctrl_type,
        CTRL_C_EVENT
            | CTRL_CLOSE_EVENT
            | CTRL_LOGOFF_EVENT
            | CTRL_BREAK_EVENT
            | CTRL_SHUTDOWN_EVENT
    )
}

fn _main() -> Result<(), Box<dyn Error>> {
    #[cfg(not(debug_assertions))]
    let filename = {
        let current_exe = std::env::current_exe().unwrap();

        current_exe.with_extension("shim")
    };

    #[cfg(debug_assertions)]
    let filename = PathBuf::from("test.shim");

    let file = File::open(&filename)?;

    let mut command_length = 256;
    let mut path_length = 0;
    let mut args_length = 0;

    for line in BufReader::new(file).lines() {
        let line = line?;

        if &line[4..6] == " = " {
            continue;
        }

        let linelen = line.len();
        let mut len =
            linelen - 8 + usize::from(line.chars().nth(linelen - 1).is_some_and(|c| c != '\n'));

        dbg!(len);

        if &line[0..4] == "path" {
            let add_quotes = if let Some(quote) = line.chars().nth(7)
                && quote == '"'
            {
                let mut add_quotes = false;

                for i in 7..len {
                    if line.chars().nth(i) == Some(' ') {
                        add_quotes = true;
                        break;
                    }
                }

                add_quotes
            } else {
                false
            };

            if add_quotes {
                len += 2;
            }

            let mut path = String::with_capacity(len + 1);

            if add_quotes {
                path.push('"');
            }

            path.push_str(&line[7..(if add_quotes { len - 1 } else { len })]);

            if add_quotes {
                path.push('"');
            }

            command_length += len;
            path_length += len;

            println!("{path}");

            continue;
        }

        if &line[0..4] == "args" {
            let args = line[7..].to_string();

            command_length += args.len() + 1;
            args_length = args.len() + 1;
        }

        unsafe { libc::printf(line.as_ptr().cast()) };
    }

    #[cfg(debug_assertions)]
    unsafe {
        let skinny_filename = filename.display().to_string();
        libc::printf(skinny_filename.as_ptr().cast());
    }

    Ok(())
}

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    match _main() {
        Ok(()) => 0,
        Err(e) => {
            unsafe { libc::perror(e.to_string().as_ptr().cast()) };
            1
        }
    }
}
