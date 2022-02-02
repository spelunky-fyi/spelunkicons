use image::imageops::{flip_horizontal, overlay};
use image::ImageFormat::Png;
use image::{load_from_memory_with_format, DynamicImage, SubImage};
use image::{GenericImageView, RgbaImage};
use rand::prelude::*;

use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::pngs;
use crate::spelunkicon::Spelunkicon;

pub struct Sheets {
    floor_cave: DynamicImage,
    floor_jungle: DynamicImage,
    floor_babylon: DynamicImage,
    floor_eggplant: DynamicImage,
    floor_ice: DynamicImage,
    floor_sunken: DynamicImage,
    floor_surface: DynamicImage,
    floor_temple: DynamicImage,
    floor_tidepool: DynamicImage,
    floor_volcano: DynamicImage,

    floorstyled_vlad: DynamicImage,
    floorstyled_wood: DynamicImage,
    floorstyled_babylon: DynamicImage,
    floorstyled_beehive: DynamicImage,
    floorstyled_duat: DynamicImage,
    floorstyled_gold: DynamicImage,
    floorstyled_guts: DynamicImage,
    floorstyled_mothership: DynamicImage,
    floorstyled_pagoda: DynamicImage,
    floorstyled_palace: DynamicImage,
    floorstyled_stone: DynamicImage,
    floorstyled_sunken: DynamicImage,
    floorstyled_temple: DynamicImage,

    items: DynamicImage,
}

impl Sheets {
    pub fn new() -> Self {
        Self {
            floor_cave: load_from_memory_with_format(pngs::FLOOR_CAVE, Png).unwrap(),
            floor_jungle: load_from_memory_with_format(pngs::FLOOR_JUNGLE, Png).unwrap(),
            floor_babylon: load_from_memory_with_format(pngs::FLOOR_BABYLON, Png).unwrap(),
            floor_eggplant: load_from_memory_with_format(pngs::FLOOR_EGGPLANT, Png).unwrap(),
            floor_ice: load_from_memory_with_format(pngs::FLOOR_ICE, Png).unwrap(),
            floor_sunken: load_from_memory_with_format(pngs::FLOOR_SUNKEN, Png).unwrap(),
            floor_surface: load_from_memory_with_format(pngs::FLOOR_SURFACE, Png).unwrap(),
            floor_temple: load_from_memory_with_format(pngs::FLOOR_TEMPLE, Png).unwrap(),
            floor_tidepool: load_from_memory_with_format(pngs::FLOOR_TIDEPOOL, Png).unwrap(),
            floor_volcano: load_from_memory_with_format(pngs::FLOOR_VOLCANO, Png).unwrap(),

            floorstyled_vlad: load_from_memory_with_format(pngs::FLOORSTYLED_VLAD, Png).unwrap(),
            floorstyled_wood: load_from_memory_with_format(pngs::FLOORSTYLED_WOOD, Png).unwrap(),
            floorstyled_babylon: load_from_memory_with_format(pngs::FLOORSTYLED_BABYLON, Png)
                .unwrap(),
            floorstyled_beehive: load_from_memory_with_format(pngs::FLOORSTYLED_BEEHIVE, Png)
                .unwrap(),
            floorstyled_duat: load_from_memory_with_format(pngs::FLOORSTYLED_DUAT, Png).unwrap(),
            floorstyled_gold: load_from_memory_with_format(pngs::FLOORSTYLED_GOLD, Png).unwrap(),
            floorstyled_guts: load_from_memory_with_format(pngs::FLOORSTYLED_GUTS, Png).unwrap(),
            floorstyled_mothership: load_from_memory_with_format(pngs::FLOORSTYLED_MOTHERSHIP, Png)
                .unwrap(),
            floorstyled_pagoda: load_from_memory_with_format(pngs::FLOORSTYLED_PAGODA, Png)
                .unwrap(),
            floorstyled_palace: load_from_memory_with_format(pngs::FLOORSTYLED_PALACE, Png)
                .unwrap(),
            floorstyled_stone: load_from_memory_with_format(pngs::FLOORSTYLED_STONE, Png).unwrap(),
            floorstyled_sunken: load_from_memory_with_format(pngs::FLOORSTYLED_SUNKEN, Png)
                .unwrap(),
            floorstyled_temple: load_from_memory_with_format(pngs::FLOORSTYLED_TEMPLE, Png)
                .unwrap(),

            items: load_from_memory_with_format(pngs::ITEMS, Png).unwrap(),
        }
    }

    fn sheet_floor_from_biome(&self, biome: &Biome) -> Option<&DynamicImage> {
        match biome {
            Biome::Cave => Some(&self.floor_cave),
            Biome::Jungle => Some(&self.floor_jungle),
            Biome::Babylon => Some(&self.floor_babylon),
            Biome::Eggplant => Some(&self.floor_eggplant),
            Biome::Ice => Some(&self.floor_ice),
            Biome::Sunken => Some(&self.floor_sunken),
            Biome::Surface => Some(&self.floor_surface),
            Biome::Temple => Some(&self.floor_temple),
            Biome::TidePool => Some(&self.floor_tidepool),
            Biome::Volcana => Some(&self.floor_volcano),

            _ => None,
        }
    }

    fn sheet_floorstyled_from_biome(&self, biome: &Biome) -> Option<&DynamicImage> {
        match biome {
            Biome::Cave => Some(&self.floorstyled_wood),
            Biome::Jungle => Some(&self.floorstyled_stone),
            Biome::Babylon => Some(&self.floorstyled_babylon),
            Biome::Sunken => Some(&self.floorstyled_sunken),
            Biome::Temple => Some(&self.floorstyled_temple),
            Biome::TidePool => Some(&self.floorstyled_pagoda),

            Biome::Beehive => Some(&self.floorstyled_beehive),
            Biome::Vlad => Some(&self.floorstyled_vlad),
            Biome::CityOfGold => Some(&self.floorstyled_gold),
            Biome::Duat => Some(&self.floorstyled_duat),
            Biome::Mothership => Some(&self.floorstyled_mothership),
            Biome::PalaceOfPleasure => Some(&self.floorstyled_palace),
            Biome::Guts => Some(&self.floorstyled_guts),

            _ => None,
        }
    }
}

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

#[derive(Debug, Clone, PartialEq)]
enum PlacedTile {
    None,
    Floor,
    FloorStyled,
}
type PlacedTileGrid = Vec<Vec<PlacedTile>>;

impl GenSheet {
    pub fn new(kind: GenKind, biome: Biome) -> Self {
        match kind {
            GenKind::Floor => GenSheet::Floor(biome),
            GenKind::FloorStyled => GenSheet::FloorStyled(biome),
            GenKind::FloorAndFloorStyled => GenSheet::FloorAndFloorStyled(biome),
        }
    }

    pub fn base_tile<'a>(&self, image: &'a DynamicImage) -> SubImage<&'a DynamicImage> {
        match self {
            GenSheet::Floor(_) | GenSheet::FloorAndFloorStyled(_) => {
                image.view(0, 0, TILE_WIDTH, TILE_HEIGHT)
            }
            GenSheet::FloorStyled(_) => {
                image.view(1 * TILE_WIDTH, 3 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT)
            }
        }
    }

    fn left_empty(&self, config: &Spelunkicon, curr_row: usize, curr_col: usize) -> bool {
        if curr_col == 0 {
            return false;
        }

        config.grid[curr_row][curr_col - 1]
    }

    fn right_empty(&self, config: &Spelunkicon, curr_row: usize, curr_col: usize) -> bool {
        if curr_col == config.height as usize - 1 {
            return false;
        }

        config.grid[curr_row][curr_col + 1]
    }

    fn up_empty(&self, config: &Spelunkicon, curr_row: usize, curr_col: usize) -> bool {
        if curr_row == 0 {
            return false;
        }

        config.grid[curr_row - 1][curr_col]
    }

    fn down_empty(&self, config: &Spelunkicon, curr_row: usize, curr_col: usize) -> bool {
        if curr_row >= config.height as usize - 1 {
            return false;
        }

        config.grid[curr_row + 1][curr_col]
    }

    fn place_floor_tiles(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        _rng: &mut StdRng,
    ) -> PlacedTileGrid {
        let sheet_image = sheets.sheet_floor_from_biome(biome).unwrap();

        let tile = self.base_tile(sheet_image);

        let mut placed_grid =
            vec![vec![PlacedTile::None; config.grid_height as usize]; config.grid_width as usize];

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }
                let x = col_idx as u32 * TILE_HEIGHT as u32;
                let y = row_idx as u32 * TILE_WIDTH as u32;

                // Place down base tile
                overlay(base_image, &tile, x, y);

                // Mark that we placed a tile here
                placed_grid[col_idx as usize][row_idx as usize] = PlacedTile::Floor;
            }
        }

        return placed_grid;
    }

    fn place_floor_decorations(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        rng: &mut StdRng,
        existing_grid: &PlacedTileGrid,
    ) {
        let sheet_image = sheets.sheet_floor_from_biome(biome).unwrap();

        let right_deco = vec![
            sheet_image.view(5 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];
        let right_up_deco =
            sheet_image.view(7 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);

        let left_deco = vec![
            flip_horizontal(&right_deco[0]),
            flip_horizontal(&right_deco[1]),
        ];
        let left_up_deco = flip_horizontal(&right_up_deco);

        let up_deco = vec![
            sheet_image.view(5 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(7 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];

        let spikes_deco = vec![
            sheet_image.view(5 * TILE_WIDTH, 8 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 8 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(7 * TILE_WIDTH, 8 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];

        let down_deco = vec![
            sheet_image.view(5 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(7 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }

                if existing_grid[col_idx as usize][row_idx as usize] != PlacedTile::None {
                    let x = col_idx as u32 * TILE_HEIGHT as u32;
                    let y = row_idx as u32 * TILE_WIDTH as u32;

                    // Place generic deco
                    if self.left_empty(config, row_idx, col_idx) {
                        let x = x - (TILE_WIDTH / 2) + 5;
                        if self.up_empty(config, row_idx, col_idx) {
                            overlay(base_image, &left_up_deco, x, y);
                        } else {
                            overlay(base_image, left_deco.choose(rng).unwrap(), x, y);
                        }
                    }

                    if self.right_empty(config, row_idx, col_idx) {
                        let x = x + (TILE_WIDTH / 2) - 5;
                        if self.up_empty(config, row_idx, col_idx) {
                            overlay(base_image, &right_up_deco, x, y);
                        } else {
                            overlay(base_image, right_deco.choose(rng).unwrap(), x, y);
                        }
                    }

                    if self.down_empty(config, row_idx, col_idx) {
                        let y = y + (TILE_HEIGHT / 2) - 12;
                        overlay(base_image, down_deco.choose(rng).unwrap(), x, y);
                    }

                    // Place generic top-deco or spikes top-deco
                    if self.up_empty(config, row_idx, col_idx) {
                        let y = y - (TILE_HEIGHT / 2) + 4;
                        if rng.gen::<u32>() % 12 == 0 {
                            overlay(base_image, spikes_deco.choose(rng).unwrap(), x, y);
                        } else {
                            overlay(base_image, up_deco.choose(rng).unwrap(), x, y);
                        }
                    }
                }
            }
        }
    }

    fn place_floorstyled_tiles(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        _rng: &mut StdRng,
        existing_grid: Option<PlacedTileGrid>,
    ) -> PlacedTileGrid {
        let sheet_image = sheets.sheet_floorstyled_from_biome(biome).unwrap();
        let tile = self.base_tile(sheet_image);

        let has_existing_grid = existing_grid.is_some();
        let mut placed_grid = existing_grid.unwrap_or_else(|| {
            vec![vec![PlacedTile::None; config.grid_height as usize]; config.grid_width as usize]
        });

        if has_existing_grid {
        } else {
            for (row_idx, row) in config.grid.iter().enumerate() {
                for (col_idx, col) in row.iter().enumerate() {
                    if *col {
                        continue;
                    }
                    let x = col_idx as u32 * TILE_HEIGHT as u32;
                    let y = row_idx as u32 * TILE_WIDTH as u32;

                    // Place down base tile
                    overlay(base_image, &tile, x, y);
                    placed_grid[col_idx as usize][row_idx as usize] = PlacedTile::FloorStyled;
                }
            }
        }
        return placed_grid;
    }

    fn place_floor_embeds(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
        existing_grid: &PlacedTileGrid,
    ) {
        let crust_gold = vec![
            sheets
                .items
                .view(TILE_WIDTH * 10, 0, TILE_WIDTH, TILE_HEIGHT),
            sheets
                .items
                .view(TILE_WIDTH * 11, 0, TILE_WIDTH, TILE_HEIGHT),
        ];

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }

                if existing_grid[col_idx as usize][row_idx as usize] != PlacedTile::None {
                    let x = col_idx as u32 * TILE_HEIGHT as u32;
                    let y = row_idx as u32 * TILE_WIDTH as u32;

                    // Place Gold
                    if rng.gen::<u32>() % 12 == 0 {
                        overlay(base_image, crust_gold.choose(rng).unwrap(), x, y);
                    }
                }
            }
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
                let floor = self.place_floor_tiles(base_image, sheets, biome, config, rng);
                self.place_floor_decorations(base_image, sheets, biome, config, rng, &floor);
                self.place_floor_embeds(base_image, sheets, config, rng, &floor);
            }
            GenSheet::FloorStyled(biome) => {
                self.place_floorstyled_tiles(base_image, sheets, biome, config, rng, None);
            }
            GenSheet::FloorAndFloorStyled(biome) => {
                let floor = self.place_floor_tiles(base_image, sheets, biome, config, rng);
                let floor = self.place_floorstyled_tiles(
                    base_image,
                    sheets,
                    biome,
                    config,
                    rng,
                    Some(floor),
                );
                self.place_floor_decorations(base_image, sheets, biome, config, rng, &floor);
                self.place_floor_embeds(base_image, sheets, config, rng, &floor);
            }
        }
    }
}
