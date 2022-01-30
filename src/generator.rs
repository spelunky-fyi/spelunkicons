use image::imageops::overlay;
use image::png::PngEncoder;
use image::GenericImageView;
use image::ImageFormat::Png;
use image::{load_from_memory_with_format, DynamicImage, ImageBuffer, RgbaImage};
use indexmap::{indexmap, IndexMap};
use rand::prelude::StdRng;
use rand::prelude::*;
use rand::SeedableRng;

use crate::pngs;
use crate::spelunkicon::Spelunkicon;

const NUM_ROWS: usize = 6;
const NUM_COLS: usize = 6;

const TILE_WIDTH: u32 = 128;
const TILE_HEIGHT: u32 = 128;
const IMAGE_WIDTH: u32 = TILE_WIDTH * NUM_ROWS as u32;
const IMAGE_HEIGHT: u32 = TILE_HEIGHT * NUM_COLS as u32;

pub struct Generator {
    pub sheets: IndexMap<String, DynamicImage>,
}

impl Generator {
    pub fn new() -> Self {
        let sheets = indexmap! {
            String::from("CAVE_FLOOR") => load_from_memory_with_format(pngs::FLOOR_CAVE, Png).unwrap(),
            String::from("CAVE_JUNGLE") => load_from_memory_with_format(pngs::FLOOR_JUNGLE, Png).unwrap(),
            String::from("FLOOR_BABYLON") => load_from_memory_with_format(pngs::FLOOR_BABYLON, Png).unwrap(),
            String::from("FLOOR_EGGPLANT") => load_from_memory_with_format(pngs::FLOOR_EGGPLANT, Png).unwrap(),
            String::from("FLOOR_ICE") => load_from_memory_with_format(pngs::FLOOR_ICE, Png).unwrap(),
            String::from("FLOOR_SUNKEN") => load_from_memory_with_format(pngs::FLOOR_SUNKEN, Png).unwrap(),
            String::from("FLOOR_SURFACE") => load_from_memory_with_format(pngs::FLOOR_SURFACE, Png).unwrap(),
            String::from("FLOOR_TEMPLE") => load_from_memory_with_format(pngs::FLOOR_TEMPLE, Png).unwrap(),
            String::from("FLOOR_TIDEPOOL") => load_from_memory_with_format(pngs::FLOOR_TIDEPOOL, Png).unwrap(),
            String::from("FLOOR_VOLCANO") => load_from_memory_with_format(pngs::FLOOR_VOLCANO, Png).unwrap(),
            // String::from("FLOORSTYLED_BABYLON") => load_from_memory_with_format(pngs::FLOORSTYLED_BABYLON, Png).unwrap(),
            // String::from("FLOORSTYLED_BEEHIVE") => load_from_memory_with_format(pngs::FLOORSTYLED_BEEHIVE, Png).unwrap(),
            // String::from("FLOORSTYLED_DUAT") => load_from_memory_with_format(pngs::FLOORSTYLED_DUAT, Png).unwrap(),
            // String::from("FLOORSTYLED_GOLD") => load_from_memory_with_format(pngs::FLOORSTYLED_GOLD, Png).unwrap(),
            // String::from("FLOORSTYLED_GUTS") => load_from_memory_with_format(pngs::FLOORSTYLED_GUTS, Png).unwrap(),
            // String::from("FLOORSTYLED_MOTHERSHIP") => load_from_memory_with_format(pngs::FLOORSTYLED_MOTHERSHIP, Png).unwrap(),
            // String::from("FLOORSTYLED_PAGODA") => load_from_memory_with_format(pngs::FLOORSTYLED_PAGODA, Png).unwrap(),
            // String::from("FLOORSTYLED_PALACE") => load_from_memory_with_format(pngs::FLOORSTYLED_PALACE, Png).unwrap(),
            // String::from("FLOORSTYLED_STONE") => load_from_memory_with_format(pngs::FLOORSTYLED_STONE, Png).unwrap(),
            // String::from("FLOORSTYLED_SUNKEN") => load_from_memory_with_format(pngs::FLOORSTYLED_SUNKEN, Png).unwrap(),
            // String::from("FLOORSTYLED_TEMPLE") => load_from_memory_with_format(pngs::FLOORSTYLED_TEMPLE, Png).unwrap(),
            // String::from("FLOORSTYLED_VLAD") => load_from_memory_with_format(pngs::FLOORSTYLED_VLAD, Png).unwrap(),
            // String::from("FLOORSTYLED_WOOD") => load_from_memory_with_format(pngs::FLOORSTYLED_WOOD, Png).unwrap(),
        };
        Generator { sheets }
    }

    pub fn make_png(&self, config: Spelunkicon) -> Option<Vec<u8>> {
        // Generate Image
        let mut image: RgbaImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
        let mut rng = StdRng::seed_from_u64(config.hash as u64);
        let sheet_idx = rng.gen::<usize>() % self.sheets.len();

        let (_, sheet) = self.sheets.get_index(sheet_idx).unwrap();
        let tile = sheet.view(0, 0, TILE_WIDTH, TILE_HEIGHT);

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }
                let x = col_idx as u32 * TILE_HEIGHT as u32;
                let y = row_idx as u32 * TILE_WIDTH as u32;

                overlay(&mut image, &tile, x, y);
            }
        }

        // Convert to PNG
        let mut out = Vec::new();
        let png = PngEncoder::new(&mut out).encode(
            image.as_raw(),
            IMAGE_WIDTH,
            IMAGE_HEIGHT,
            image::ColorType::Rgba8,
        );

        if png.is_err() {
            dbg!(&png);
        }

        png.map(|_| out).ok()
    }
}
