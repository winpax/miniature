use std::ops::{Deref, DerefMut};

use widestring::{WideCStr, WideCString};

#[derive(Debug, Clone)]
pub struct StringTable([&'static WideCStr; 16]);

impl StringTable {
    pub fn set_path(&mut self, path: &'static WideCStr) {
        self.0[1] = path;
    }

    pub fn set_args(&mut self, args: &'static WideCStr) {
        self.0[2] = args;
    }
}

impl Drop for StringTable {
    fn drop(&mut self) {
        for s in &mut self.0 {
            _ = unsafe { Box::from_raw((*s).as_ptr().cast_mut()) };
        }
    }
}

impl Default for StringTable {
    fn default() -> Self {
        let data =
            [0; 16].map(|_| Box::leak(WideCString::new().into_boxed_ucstr()) as &'static WideCStr);
        Self(data)
    }
}

impl Deref for StringTable {
    type Target = [&'static WideCStr; 16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StringTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
