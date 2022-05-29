use image::imageops::{resize, FilterType};
use image::png::PngEncoder;
use image::{ImageBuffer, RgbaImage};
use indexmap::indexmap;
use indexmap::IndexMap;
use rand::prelude::StdRng;
use rand::prelude::*;
use rand::SeedableRng;

use crate::constants::{OUTPUT_DIMENSION, TILE_HEIGHT, TILE_WIDTH};
use crate::grid_renderer::Sheets;
use crate::sheets::{Biome, GenKind, GenSheet};
use crate::spelunkicon::Spelunkicon;

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
            "FLOORSTYLED_BEEHIVE"    => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::Beehive),
            "FLOOR_BABYLON"          => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::Babylon),
            "FLOOR_SUNKEN"           => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::Sunken),
            "FLOOR_TEMPLE"           => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::Temple),
            "FLOOR_TIDEPOOL"         => GenSheet::new(GenKind::FloorAndFloorStyled, Biome::TidePool),

            "FLOORSTYLED_VLAD"       => GenSheet::new(GenKind::FloorStyled, Biome::Vlad),
            "FLOORSTYLED_STONE"      => GenSheet::new(GenKind::FloorStyled, Biome::Olmec),
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
        let image_height = config.grid_height as u32 * TILE_HEIGHT;
        let image_width = config.grid_width as u32 * TILE_WIDTH;

        let mut image: RgbaImage = ImageBuffer::new(image_width, image_height);
        let mut rng = StdRng::seed_from_u64(config.hash as u64);
        let sheet_idx = rng.gen::<usize>() % self.sheet_gens.len();

        let egg = config.egg.clone().unwrap_or_else(Vec::<String>::new);

        // Note intentionally doing rng first so that the manual classic mode has same rng as random classic mode
        let classic_mode = rng.gen_bool(0.01) || egg.contains(&String::from("classic"));

        // Generate Image
        if egg.contains(&String::from("pride")) {
            let sheet = GenSheet::new(GenKind::Pride, Biome::Cave);
            sheet.generate_image(&mut image, &self.sheets, &config, classic_mode, &mut rng);
        } else {
            let sheet = &self.sheet_gens[sheet_idx];
            sheet.generate_image(&mut image, &self.sheets, &config, classic_mode, &mut rng);
        }

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
