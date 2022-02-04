use image::RgbaImage;
use rand::prelude::*;

use crate::grid_generator::*;
use crate::grid_renderer::{GridRenderer, Sheets};
use crate::spelunkicon::Spelunkicon;

pub enum Biome {
    Cave,
    Jungle,
    Beehive,
    Babylon,
    PalaceOfPleasure,
    Eggplant,
    Ice,
    Mothership,
    Sunken,
    Guts,
    Surface,
    Temple,
    CityOfGold,
    Duat,
    TidePool,
    Volcana,
    Vlad,
    Olmec,
}

pub enum GenKind {
    Floor,
    FloorStyled,
    FloorAndFloorStyled,
}

pub enum GenSheet {
    Floor(Biome),
    FloorStyled(Biome),
    FloorAndFloorStyled(Biome),
}

impl GenSheet {
    pub fn new(kind: GenKind, biome: Biome) -> Self {
        match kind {
            GenKind::Floor => GenSheet::Floor(biome),
            GenKind::FloorStyled => GenSheet::FloorStyled(biome),
            GenKind::FloorAndFloorStyled => GenSheet::FloorAndFloorStyled(biome),
        }
    }

    pub fn generate_image(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
    ) {
        match self {
            GenSheet::Floor(biome) => {
                let generator = GridGenerator {};
                let floor = generator.place_floor_tiles(biome, config, rng);
                let floor = generator.place_floormisc_tiles(biome, config, rng, floor);

                let renderer = GridRenderer {};
                renderer.render_floor_tiles(base_image, sheets, biome, config, rng, &floor);
                renderer.render_floormisc_tiles(base_image, sheets, biome, config, rng, &floor);
                renderer.render_floor_decorations(base_image, sheets, biome, config, rng, &floor);
                renderer.render_floor_embeds(base_image, sheets, config, rng, &floor);
            }
            GenSheet::FloorStyled(biome) => {
                let generator = GridGenerator {};
                let floor = generator.place_floorstyled_tiles(biome, config, rng, None);
                let floor = generator.place_floormisc_tiles(biome, config, rng, floor);

                let renderer = GridRenderer {};
                renderer.render_floorstyled_tiles(base_image, sheets, biome, config, rng, &floor);
                renderer.render_floormisc_tiles(base_image, sheets, biome, config, rng, &floor);
            }
            GenSheet::FloorAndFloorStyled(biome) => {
                let generator = GridGenerator {};
                let floor = generator.place_floor_tiles(biome, config, rng);
                let floor = generator.place_floorstyled_tiles(biome, config, rng, Some(floor));
                let floor = generator.place_floormisc_tiles(biome, config, rng, floor);

                let renderer = GridRenderer {};
                renderer.render_floor_tiles(base_image, sheets, biome, config, rng, &floor);
                renderer.render_floorstyled_tiles(base_image, sheets, biome, config, rng, &floor);
                renderer.render_floormisc_tiles(base_image, sheets, biome, config, rng, &floor);
                renderer.render_floor_decorations(base_image, sheets, biome, config, rng, &floor);
                renderer.render_floor_embeds(base_image, sheets, config, rng, &floor);
            }
        }
    }
}
