use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use widestring::{WideCStr, WideCString};

use crate::{error, shim::ShimArgs};

#[derive(Debug, Clone)]
pub struct StringTable([&'static WideCStr; 16]);

impl StringTable {
    pub fn new(target: impl AsRef<Path>, args: &[String]) -> error::Result<Self> {
        let mut this = Self::default();
        this.set_target(target)?;
        this.set_args(args)?;

        Ok(this)
    }

    pub fn set_target(&mut self, path: impl AsRef<Path>) -> error::Result<()> {
        let c_target = WideCString::from_os_str(path.as_ref().as_os_str())?.into_boxed_ucstr();

        self.0[1] = Box::leak(c_target);

        Ok(())
    }

    pub fn set_args(&mut self, args: &[String]) -> error::Result<()> {
        let c_args = WideCString::from_str(args.join(" "))?.into_boxed_ucstr();

        self.0[2] = Box::leak(c_args);

        Ok(())
    }

    pub fn get_buffer(&self) -> Vec<u16> {
        let mut table_buffer = Vec::<u16>::new();

        #[allow(clippy::cast_possible_truncation)]
        for entry in &self.0 {
            let entry = *entry;
            table_buffer.push((entry.len() + 1) as u16);

            let entry_buffer = entry.as_slice_with_nul();
            table_buffer.extend(entry_buffer);
        }

        table_buffer
    }
}

impl TryFrom<ShimArgs> for StringTable {
    type Error = error::Error;

    fn try_from(value: ShimArgs) -> error::Result<Self> {
        Self::new(value.target, &value.args)
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
