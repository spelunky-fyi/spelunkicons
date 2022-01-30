// This module includes the embedded spritesheets. To add additional spritesheets
// update `build.rs`.
pub mod pngs {
    include!(concat!(env!("OUT_DIR"), "/pngs.rs"));
}

pub mod generator;
pub mod service;
pub mod spelunkicon;
