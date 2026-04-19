// SPDX-License-Identifier: MIT OR Apache-2.0
// Traits for DIP abstraction
use crate::core::compiler::{BuildRequest, BuildResult};
use crate::core::flasher::{FlashRequest, FlashResult};

pub trait Builder {
    fn build(&self, req: &BuildRequest) -> BuildResult;
}

pub trait Flasher {
    fn flash(&self, req: &FlashRequest) -> FlashResult;
}

pub trait PortScanner {
    fn list_ports(&self) -> Vec<String>;
}
