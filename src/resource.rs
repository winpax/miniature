mod ids;

use widestring::U16CString;
use windows::{core::PWSTR, Win32::UI::WindowsAndMessaging::LoadStringW};

use crate::MAX_PATH;

pub struct ChildResource {
    path: U16CString,
    args: U16CString,
}

impl ChildResource {
    pub unsafe fn load() -> Self {
        let path = load_resource_string(ids::IDS_PATH);
        let args = load_resource_string(ids::IDS_ARGS);

        Self { path, args }
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
