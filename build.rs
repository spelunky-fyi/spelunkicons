use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

static PNGS: &'static [(&str, &str)] = &[
    ("FLOOR_CAVE", "floor_cave.png"),
    ("FLOOR_JUNGLE", "floor_jungle.png"),
    ("FLOOR_BABYLON", "floor_babylon.png"),
    ("FLOOR_EGGPLANT", "floor_eggplant.png"),
    ("FLOOR_ICE", "floor_ice.png"),
    ("FLOOR_SUNKEN", "floor_sunken.png"),
    ("FLOOR_SURFACE", "floor_surface.png"),
    ("FLOOR_TEMPLE", "floor_temple.png"),
    ("FLOOR_TIDEPOOL", "floor_tidepool.png"),
    ("FLOOR_VOLCANO", "floor_volcano.png"),
    // ("FLOORSTYLED_BABYLON", "floorstyled_babylon.png"),
    // ("FLOORSTYLED_BEEHIVE", "floorstyled_beehive.png"),
    // ("FLOORSTYLED_DUAT", "floorstyled_duat.png"),
    // ("FLOORSTYLED_GOLD", "floorstyled_gold.png"),
    // ("FLOORSTYLED_GUTS", "floorstyled_guts.png"),
    // ("FLOORSTYLED_MOTHERSHIP", "floorstyled_mothership.png"),
    // ("FLOORSTYLED_PAGODA", "floorstyled_pagoda.png"),
    // ("FLOORSTYLED_PALACE", "floorstyled_palace.png"),
    // ("FLOORSTYLED_STONE", "floorstyled_stone.png"),
    // ("FLOORSTYLED_SUNKEN", "floorstyled_sunken.png"),
    // ("FLOORSTYLED_TEMPLE", "floorstyled_temple.png"),
    // ("FLOORSTYLED_VLAD", "floorstyled_vlad.png"),
    // ("FLOORSTYLED_WOOD", "floorstyled_wood.png"),
    // ("FLOORMISC", "floormisc.png"),
    ("ITEMS", "items.png"),
];

fn main() {
    let textures_path = fs::canonicalize(PathBuf::from("target/Textures")).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("pngs.rs");

    let mut out_file = File::create(&dest_path).unwrap();

    for (name, path) in PNGS {
        let path = textures_path.join(path);
        let metadata = fs::metadata(&path).unwrap();

        out_file
            .write(
                format!(
                    "pub static {}: &'static [u8; {}] = include_bytes!({:?});\n",
                    name,
                    metadata.len(),
                    &path
                )
                .as_bytes(),
            )
            .unwrap();
    }
    println!("cargo:rerun-if-changed=build.rs");
}
