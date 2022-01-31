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

            items: load_from_memory_with_format(pngs::ITEMS, Png).unwrap(),
        }
    }

    fn sheet_from_biome(&self, biome: &Biome) -> &DynamicImage {
        match biome {
            Biome::Cave => &self.floor_cave,
            Biome::Jungle => &self.floor_jungle,
            Biome::Babylon => &self.floor_babylon,
            Biome::Eggplant => &self.floor_eggplant,
            Biome::Ice => &self.floor_ice,
            Biome::Sunken => &self.floor_sunken,
            Biome::Surface => &self.floor_surface,
            Biome::Temple => &self.floor_temple,
            Biome::TidePool => &self.floor_tidepool,
            Biome::Volcana => &self.floor_volcano,
        }
    }
}

pub enum Biome {
    Cave,
    Jungle,
    Babylon,
    Eggplant,
    Ice,
    Sunken,
    Surface,
    Temple,
    TidePool,
    Volcana,
}

pub enum GenKind {
    Floor,
    FloorStyled,
}

pub enum GenSheet {
    Floor(Biome),
    FloorStyled(Biome),
}

impl GenSheet {
    pub fn new(kind: GenKind, biome: Biome) -> Self {
        match kind {
            GenKind::Floor => GenSheet::Floor(biome),
            GenKind::FloorStyled => GenSheet::FloorStyled(biome),
        }
    }

    pub fn base_tile<'a>(&self, image: &'a DynamicImage) -> SubImage<&'a DynamicImage> {
        match self {
            GenSheet::Floor(_) => image.view(0, 0, TILE_WIDTH, TILE_HEIGHT),
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

    fn decorate_floor(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        rng: &mut StdRng,
    ) {
        let sheet_image = sheets.sheet_from_biome(biome);

        let tile = self.base_tile(sheet_image);
        let crust_gold = vec![
            sheets
                .items
                .view(TILE_WIDTH * 10, 0, TILE_WIDTH, TILE_HEIGHT),
            sheets
                .items
                .view(TILE_WIDTH * 11, 0, TILE_WIDTH, TILE_HEIGHT),
        ];

        let right_deco = vec![
            sheet_image.view(5 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(7 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];
        let left_deco = vec![
            flip_horizontal(&right_deco[0]),
            flip_horizontal(&right_deco[1]),
            flip_horizontal(&right_deco[2]),
        ];

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
                let x = col_idx as u32 * TILE_HEIGHT as u32;
                let y = row_idx as u32 * TILE_WIDTH as u32;

                // Place down base tile
                overlay(base_image, &tile, x, y);

                // Place Gold
                if rng.gen::<u32>() % 12 == 0 {
                    overlay(base_image, crust_gold.choose(rng).unwrap(), x, y);
                }

                if self.left_empty(config, row_idx, col_idx) {
                    overlay(
                        base_image,
                        left_deco.choose(rng).unwrap(),
                        x - (TILE_WIDTH / 2) + 5,
                        y,
                    );
                }

                if self.right_empty(config, row_idx, col_idx) {
                    overlay(
                        base_image,
                        right_deco.choose(rng).unwrap(),
                        x + (TILE_WIDTH / 2) - 5,
                        y,
                    );
                }

                if self.up_empty(config, row_idx, col_idx) {
                    if rng.gen::<u32>() % 12 == 0 {
                        overlay(
                            base_image,
                            spikes_deco.choose(rng).unwrap(),
                            x,
                            y - (TILE_HEIGHT / 2) + 4,
                        );
                    } else {
                        overlay(
                            base_image,
                            up_deco.choose(rng).unwrap(),
                            x,
                            y - (TILE_HEIGHT / 2) + 4,
                        );
                    }
                }

                if self.down_empty(config, row_idx, col_idx) {
                    overlay(
                        base_image,
                        down_deco.choose(rng).unwrap(),
                        x,
                        y + (TILE_HEIGHT / 2) - 12,
                    );
                }
            }
        }
    }

    pub fn decorate_floorstyled(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        _rng: &mut StdRng,
    ) {
        let sheet_image = sheets.sheet_from_biome(biome);
        let tile = self.base_tile(sheet_image);

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }
                let x = col_idx as u32 * TILE_HEIGHT as u32;
                let y = row_idx as u32 * TILE_WIDTH as u32;

                // Place down base tile
                overlay(base_image, &tile, x, y);
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
            GenSheet::Floor(biome) => self.decorate_floor(base_image, sheets, biome, config, rng),
            GenSheet::FloorStyled(biome) => {
                self.decorate_floorstyled(base_image, sheets, biome, config, rng)
            }
        }
    }
}
