// SPDX-License-Identifier: GPL-2.0

use kernel::{
    file::{File, IoctlCommand, IoctlHandler},
    miscdev::Registration,
    prelude::*,
    user_ptr::{UserSlicePtrReader, UserSlicePtrWriter},
};

module! {
    type: RustIoctlServer,
    name: "rust_ioctl_server",
    author: "Your Name",
    description: "Rust IOCTL server sample",
    license: "GPL",
}

struct RustIoctlServer {
    _dev: Pin<Box<Registration<FileState>>>,
}

impl kernel::Module for RustIoctlServer {
    fn init(name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust IOCTL server sample (init)\n");

        Ok(Self {
            _dev: Registration::new_pinned(fmt!("{name}"), Box::pin(FileState))?,
        })
    }
}

impl Drop for RustIoctlServer {
    fn drop(&mut self) {
        pr_info!("Rust IOCTL server sample (exit)\n");
    }
}

const IOCTL_CUSTOM_COMMAND: u32 = 0x80086301;

struct FileState;

#[vtable]
impl IoctlHandler for FileState {
    type Target<'a> = &'a Self;

    fn ioctl(this: &Self, _: &File, cmd: &mut IoctlCommand) -> Result<i32> {
        match cmd.cmd {
            IOCTL_CUSTOM_COMMAND => {
                let mut reader = UserSlicePtrReader::new(cmd.arg);
                let mut data = vec![0u8; reader.remaining()];
                reader.read_slice(&mut data)?;
                // Process the data received from userspace
                pr_info!("Received data from userspace: {:?}", data);
                Ok(0)
            }
            _ => Err(EINVAL),
        }
    }
}
