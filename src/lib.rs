// This module includes the embedded spritesheets. To add additional spritesheets
// update `build.rs`.
pub mod sheets {
    include!(concat!(env!("OUT_DIR"), "/sheets.rs"));
}

pub mod service;
pub mod spelunkicon;
