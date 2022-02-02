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
            "FLOOR_SURFACE"          => GenSheet::new(GenKind::Floor, Biome::Surface),
            "FLOOR_EGGPLANT"         => GenSheet::new(GenKind::Floor, Biome::Eggplant),
            "FLOOR_ICE"              => GenSheet::new(GenKind::Floor, Biome::Ice),
            "FLOOR_VOLCANO"          => GenSheet::new(GenKind::Floor, Biome::Volcana),

            "FLOOR_CAVE"             => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::Cave),
            "FLOOR_JUNGLE"           => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::Jungle),
            "FLOOR_BABYLON"          => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::Babylon),
            "FLOOR_SUNKEN"           => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::Sunken),
            "FLOOR_TEMPLE"           => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::Temple),
            "FLOOR_TIDEPOOL"         => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::TidePool),

            "FLOORSTYLED_BEEHIVE"    => GenSheet::new(GenKind::FloorStyled, Biome::Beehive),
            "FLOORSTYLED_VLAD"       => GenSheet::new(GenKind::FloorStyled, Biome::Vlad),
            "FLOORSTYLED_GOLD"       => GenSheet::new(GenKind::FloorStyled, Biome::CityOfGold),
            "FLOORSTYLED_DUAT"       => GenSheet::new(GenKind::FloorStyled, Biome::Duat),
            "FLOORSTYLED_MOTHERSHIP" => GenSheet::new(GenKind::FloorStyled, Biome::Mothership),
            "FLOORSTYLED_PALACE"     => GenSheet::new(GenKind::FloorStyled, Biome::PalaceOfPleasure),
            "FLOORSTYLED_GUTS"       => GenSheet::new(GenKind::FloorStyled, Biome::Guts),
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
