use std::ops::{Deref, DerefMut};

use widestring::{WideCStr, WideCString};

pub struct StringTable(Vec<&'static WideCStr>);

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
        let mut data: Vec<&'static WideCStr> = Vec::with_capacity(16);
        for _ in 0..16 {
            data.push(Box::leak(WideCString::new().into_boxed_ucstr()));
        }
        Self(data)
    }
}

impl Deref for StringTable {
    type Target = Vec<&'static WideCStr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StringTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
