use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Cursor, Write};
use std::path::Path;
use std::path::PathBuf;

use image::ImageFormat::Png;
use image::{load, DynamicImage, GenericImageView};

use num_rational::Ratio;
type Ru32 = Ratio<u32>;

static PNGS: &'static [(&str, &str, Option<(Ru32, Ru32, Ru32, Ru32)>)] = &[
    ("FLOOR_CAVE", "floor_cave.png", Option::None),
    ("FLOOR_JUNGLE", "floor_jungle.png", Option::None),
    ("FLOOR_BABYLON", "floor_babylon.png", Option::None),
    ("FLOOR_EGGPLANT", "floor_eggplant.png", Option::None),
    ("FLOOR_ICE", "floor_ice.png", Option::None),
    ("FLOOR_SUNKEN", "floor_sunken.png", Option::None),
    ("FLOOR_SURFACE", "floor_surface.png", Option::None),
    ("FLOOR_TEMPLE", "floor_temple.png", Option::None),
    ("FLOOR_TIDEPOOL", "floor_tidepool.png", Option::None),
    ("FLOOR_VOLCANO", "floor_volcano.png", Option::None),
    (
        "FLOORSTYLED_BABYLON",
        "floorstyled_babylon.png",
        Option::None,
    ),
    (
        "FLOORSTYLED_BEEHIVE",
        "floorstyled_beehive.png",
        Option::None,
    ),
    ("FLOORSTYLED_DUAT", "floorstyled_duat.png", Option::None),
    ("FLOORSTYLED_GOLD", "floorstyled_gold.png", Option::None),
    ("FLOORSTYLED_GUTS", "floorstyled_guts.png", Option::None),
    (
        "FLOORSTYLED_MOTHERSHIP",
        "floorstyled_mothership.png",
        Option::None,
    ),
    ("FLOORSTYLED_PAGODA", "floorstyled_pagoda.png", Option::None),
    ("FLOORSTYLED_PALACE", "floorstyled_palace.png", Option::None),
    ("FLOORSTYLED_STONE", "floorstyled_stone.png", Option::None),
    ("FLOORSTYLED_SUNKEN", "floorstyled_sunken.png", Option::None),
    ("FLOORSTYLED_TEMPLE", "floorstyled_temple.png", Option::None),
    ("FLOORSTYLED_VLAD", "floorstyled_vlad.png", Option::None),
    ("FLOORSTYLED_WOOD", "floorstyled_wood.png", Option::None),
    ("FLOORMISC", "floormisc.png", Option::None),
    ("BASECAMP_DECO", "deco_basecamp.png", Option::None),
    ("ITEMS", "items.png", Option::None),
    (
        "CHAR_PRECIOUS",
        "monstersbasic02.png",
        Option::Some((
            Ru32::new_raw(1, 16),
            Ru32::new_raw(7, 16),
            Ru32::new_raw(1, 16),
            Ru32::new_raw(1, 16),
        )),
    ),
    (
        "CHAR_BEG",
        "monstersbasic03.png",
        Option::Some((
            Ru32::new_raw(6, 16),
            Ru32::new_raw(4, 16),
            Ru32::new_raw(1, 16),
            Ru32::new_raw(1, 16),
        )),
    ),
];

fn main() {
    let texture_roots = [
        (
            "",
            fs::canonicalize(PathBuf::from("target/Textures")).unwrap(),
        ),
        (
            "CLASSIC_",
            fs::canonicalize(PathBuf::from("target/ClassicTextures")).unwrap(),
        ),
    ];

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("pngs.rs");

    let mut out_file = File::create(&dest_path).unwrap();

    let build_profile = env::var("PROFILE").unwrap();

    let mut tile_width: Option<u32> = Option::None;

    let load_image_from_file = |path: &PathBuf| {
        let file = File::open(&path).unwrap_or_else(|err| {
            panic!("Error opening file {}: {:?}", path.to_string_lossy(), err)
        });
        let reader = BufReader::new(file);
        load(reader, Png).unwrap_or_else(|err| {
            panic!("Error reading image {}: {:?}", path.to_string_lossy(), err)
        })
    };

    for (name, path, region) in PNGS {
        for (prefix, root) in &texture_roots {
            let root = match build_profile.as_str() {
                "debug" => {
                    let low_res_root = root.join("LowRes");
                    let new_root = if low_res_root.is_dir() {
                        low_res_root
                    } else {
                        Path::new(root).to_path_buf()
                    };
                    new_root
                }
                _ => root.to_path_buf(),
            };

            let path = root.join(path);

            let name = format!("{}{}", prefix, name);
            if name == "FLOOR_CAVE" {
                let image = load_image_from_file(&path);
                let (w, _h) = (image.width(), image.height());
                let floor_cave_num_tiles: u32 = 12;
                tile_width = Option::Some(w / floor_cave_num_tiles);
            }

            match region {
                Some(region) => {
                    let image = load_image_from_file(&path);
                    let (w, h) = (image.width(), image.height());
                    let (rx, ry, rw, rh) = &region;
                    let (x, y, w, h) = (
                        (rx * w).to_integer(),
                        (ry * h).to_integer(),
                        (rw * w).to_integer(),
                        (rh * h).to_integer(),
                    );
                    let subimage = DynamicImage::ImageRgba8(image.view(x, y, w, h).to_image());

                    let mut out_bytes: Vec<u8> = Vec::new();
                    subimage
                        .write_to(&mut Cursor::new(&mut out_bytes), Png)
                        .unwrap_or_else(|err| panic!("Error writing subimage: {:?}", err));
                    out_file
                        .write(
                            format!(
                                "pub static {}: &'static [u8; {}] = &{:?};\n",
                                name,
                                out_bytes.len(),
                                out_bytes
                            )
                            .as_bytes(),
                        )
                        .unwrap();
                }
                None => {
                    let metadata = fs::metadata(&path).unwrap_or_else(|err| {
                        panic!(
                            "Error loading metadata of file {}: {:?}",
                            path.to_string_lossy(),
                            err
                        )
                    });

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
            }
        }
    }

    out_file
        .write(
            format!(
                "pub const TILE_WIDTH: u32 = {};\npub const TILE_HEIGHT: u32 = TILE_WIDTH;",
                tile_width.unwrap_or(128)
            )
            .as_bytes(),
        )
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
