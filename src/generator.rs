use image::imageops::{resize, FilterType};
use image::png::PngEncoder;
use image::{ImageBuffer, RgbaImage};
use indexmap::indexmap;
use indexmap::IndexMap;
use rand::prelude::StdRng;
use rand::prelude::*;
use rand::SeedableRng;

use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::sheets::{Biome, GenKind, GenSheet, Sheets};
use crate::spelunkicon::Spelunkicon;

const OUTPUT_DIMENSION: u32 = 250;

pub struct Generator {
    pub sheets: Sheets,
    pub sheet_gens: IndexMap<&'static str, GenSheet>,
}

impl Generator {
    pub fn new() -> Self {
        let sheet_gens = indexmap![
            "FLOOR_CAVE"     => GenSheet::new(GenKind::Floor, Biome::Cave),
            "FLOOR_JUNGLE"   => GenSheet::new(GenKind::Floor, Biome::Jungle),
            "FLOOR_BABYLON"  => GenSheet::new(GenKind::Floor, Biome::Babylon),
            "FLOOR_EGGPLANT" => GenSheet::new(GenKind::Floor, Biome::Eggplant),
            "FLOOR_ICE"      => GenSheet::new(GenKind::Floor, Biome::Ice),
            "FLOOR_SUNKEN"   => GenSheet::new(GenKind::Floor, Biome::Sunken),
            "FLOOR_SURFACE"  => GenSheet::new(GenKind::Floor, Biome::Surface),
            "FLOOR_TEMPLE"   => GenSheet::new(GenKind::Floor, Biome::Temple),
            "FLOOR_TIDEPOOL" => GenSheet::new(GenKind::Floor, Biome::TidePool),
            "FLOOR_VOLCANO"  => GenSheet::new(GenKind::Floor, Biome::Volcana),
            // "FLOORSTYLED_BABYLON"    => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_BABYLON),
            // "FLOORSTYLED_BEEHIVE"    => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_BEEHIVE),
            // "FLOORSTYLED_DUAT"       => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_DUAT),
            // "FLOORSTYLED_GOLD"       => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_GOLD),
            // "FLOORSTYLED_GUTS"       => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_GUTS),
            // "FLOORSTYLED_MOTHERSHIP" => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_MOTHERSHIP),
            // "FLOORSTYLED_PAGODA"     => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_PAGODA),
            // "FLOORSTYLED_PALACE"     => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_PALACE),
            // "FLOORSTYLED_STONE"      => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_STONE),
            // "FLOORSTYLED_SUNKEN"     => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_SUNKEN),
            // "FLOORSTYLED_TEMPLE"     => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_TEMPLE),
            // "FLOORSTYLED_VLAD"       => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_VLAD),
            // "FLOORSTYLED_WOOD"       => Sheet::new(SheetKind::FloorStyled, pngs::FLOORSTYLED_WOOD),
        ];
        Generator {
            sheets: Sheets::new(),
            sheet_gens,
        }
    }

    pub fn make_png(&self, config: Spelunkicon) -> Option<Vec<u8>> {
        // Generate Image

        let image_height = config.height as u32 * TILE_HEIGHT;
        let image_width = config.height as u32 * TILE_WIDTH;

        let mut image: RgbaImage = ImageBuffer::new(image_width, image_height);
        let mut rng = StdRng::seed_from_u64(config.hash as u64);
        let sheet_idx = rng.gen::<usize>() % self.sheet_gens.len();

        let sheet = &self.sheet_gens[sheet_idx];
        sheet.generate_image(&mut image, &self.sheets, &config, &mut rng);

        let image = resize(
            &image,
            OUTPUT_DIMENSION,
            OUTPUT_DIMENSION,
            FilterType::Nearest,
        );
        // Convert to PNG
        let mut out = Vec::new();
        let png = PngEncoder::new(&mut out).encode(
            image.as_raw(),
            OUTPUT_DIMENSION,
            OUTPUT_DIMENSION,
            image::ColorType::Rgba8,
        );

        if png.is_err() {
            dbg!(&png);
        }

        png.map(|_| out).ok()
    }
}
